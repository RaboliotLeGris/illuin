#[cfg(test)]
mod api_tests {
    use std::collections::hash_map::DefaultHasher;
    use std::fs;
    use std::hash::{Hash, Hasher};
    use std::path::Path;

    use nanoid::nanoid;
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;

    use crate::{
        construct,
        ensure_storage_path_exist,
    };

    #[test]
    fn should_get_index_from_root() {
        let rocket = construct();
        let client = Client::new(rocket).expect("valid rocket instance");

        let mut response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::HTML));
        assert!(!response.body_string().expect("contains html page").is_empty(), "Body should not be empty");
    }

    #[test]
    fn should_get_image_from_storage_folder() {
        // GIVEN
        let id = nanoid!(10);
        let image_name = format!("{}.jpg", id);
        let image_path = Path::new("resources/marius.jpeg");
        let image_hash = hash_binary(fs::read(image_path).expect("read reference picture"));

        // copy an image in the app storage folder
        let default_app_storage_folder = Path::new("/tmp/illuin");
        ensure_storage_path_exist(default_app_storage_folder.to_str().unwrap());
        std::fs::copy(image_path, default_app_storage_folder.join(&image_name)).expect("copy to app storage folder to succeed");

        // Creating rocket server
        let rocket = construct();
        let client = Client::new(rocket).expect("valid rocket instance");

        // WHEN
        // Requesting
        let mut response = client.get(format!("/i/{}", image_name)).dispatch();

        // THEN
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JPEG));
        assert_eq!(image_hash, hash_binary(response.body_bytes().expect("body to contains something")));

        // CLEANING
        clean_image(image_name);
    }

    // WIP: Require a better test env setting
    // #[test]
    // fn should_from_post_write_image_to_storage_folder() {
    //     // GIVEN
    //     let image_path = Path::new("resources/marius.jpeg");
    //     let image_hash = hash_binary(fs::read(image_path).expect("read reference picture"));
    //
    //     let default_app_storage_folder = Path::new("/tmp/illuin");
    //
    //     // Creating rocket server
    //     let rocket = construct();
    //     let client = Client::new(rocket).expect("valid rocket instance");
    //
    //     // WHEN
    //     let mut reponse; // TODO: to multipart post
    //
    //     // then
    //     assert_eq!(response.status(), Status::Ok);
    //     assert_eq!(response.content_type(), Some(ContentType::HTML));
    //     assert!(!response.body_string().expect("contains html page").is_empty(), "Body should not be empty"); // We might want to be sure that it contains an image URL
    // }

    #[test]
    fn should_fail_when_requesting_a_route_that_doesnt_exists() {
        let rocket = construct();
        let client = Client::new(rocket).expect("valid rocket instance");

        let response = client.get("/does_not_exist").dispatch();

        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn should_fail_when_requesting_a_picture_that_doesnt_exist() {
        let rocket = construct();
        let client = Client::new(rocket).expect("valid rocket instance");

        let response = client.get("/i/UnknownHash.jpg").dispatch();

        assert_eq!(response.status(), Status::InternalServerError); // `Must be Status::NotFound`
    }

    fn hash_binary(binary: Vec<u8>) -> u64 {
        let mut hasher = DefaultHasher::new();
        binary.hash(&mut hasher);
        hasher.finish()
    }

    fn clean_image(filename: String) {
        let default_app_storage_folder = Path::new("/tmp/illuin");

        fs::remove_file(default_app_storage_folder.join(filename)).expect("file to be deleted");
    }
}