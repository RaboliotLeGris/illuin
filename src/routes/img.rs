use crate::cli;

use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::io::{Error, Write};
use std::path::Path;

use nanoid::nanoid;
use rocket::{Data, State};
use rocket::http::ContentType;
use rocket::response::{Debug, NamedFile};
use rocket_contrib::templates::Template;
use rocket_multipart_form_data::{mime, MultipartFormData, MultipartFormDataError, MultipartFormDataField, MultipartFormDataOptions, RawField};

pub fn register_routes(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/i", routes![get_img, post_img])
}

#[get("/<filename>")]
fn get_img(config: State<cli::Config>, filename: String) -> Result<NamedFile, io::Error> {
    NamedFile::open(Path::new(config.storage_path.as_str()).join(filename))
}

#[derive(Serialize)]
struct UploadTemplateContext {
    url: String,
}

#[post("/upload", data = "<data>")]
fn post_img(config: State<cli::Config>, content_type: &ContentType, data: Data) -> Result<Template, Debug<io::Error>> {
    let img_field_name = "img";
    let image = get_multipart_field(content_type, data, img_field_name)?;

    let image_name: String;
    match image {
        RawField::Single(raw) => {
            let id = nanoid!(10);
            image_name = format!("{}.{}", id, get_extension(&raw.file_name));

            let mut file = File::create(Path::new(config.storage_path.as_str()).join(&image_name))?;
            file.write_all(&raw.raw)?;
            let ctx = UploadTemplateContext { url: format!("/i/{}", &image_name) };
            Ok(Template::render("uploaded", &ctx))
        }
        RawField::Multiple(_) => unreachable!(),
    }
}

fn get_extension(filename: &Option<String>) -> String {
    match filename {
        Some(s) => {
            if let Some(os_filename) = Path::new(&s).extension().and_then(OsStr::to_str) {
                String::from(os_filename)
            } else {
                String::from("bin")
            }
        }
        None => String::from("bin")
    }
}

fn get_multipart_field(content_type: &ContentType, data: Data, field_name: &str) -> Result<RawField, Debug<io::Error>> {
    let mut options = MultipartFormDataOptions::new();
    options.allowed_fields.push(
        MultipartFormDataField::raw(field_name).content_type_by_string(Some(mime::IMAGE_STAR)).unwrap(),
    );

    let mut multipart_form_data = match MultipartFormData::parse(content_type, data, options) {
        Ok(multipart_form_data) => multipart_form_data,
        Err(err) => {
            match err {
                MultipartFormDataError::DataTooLargeError(_) => {
                    return Err(Debug::from(Error::new(std::io::ErrorKind::InvalidInput, "Data too large")));
                }
                MultipartFormDataError::DataTypeError(_) => {
                    return Err(Debug::from(Error::new(std::io::ErrorKind::InvalidInput, "Data not an image")));
                }
                _ => panic!("{:?}", err),
            }
        }
    };
    if let Some(field) = multipart_form_data.raw.remove(field_name) {
        return Ok(field);
    };
    Err(Debug::from(Error::new(std::io::ErrorKind::NotFound, "Missing field")))
}