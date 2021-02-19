#[cfg(test)]
mod api_tests {
    use std::collections::hash_map::DefaultHasher;
    use std::fs;
    use std::hash::{Hash, Hasher};
    use std::path::Path;

    use nanoid::nanoid;
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;

    use crate::{build_rocket, construct_from_cli, ensure_storage_path_exist};
    use crate::cli::AppConfig;

    #[test]
    fn should_get_index_from_root() {
        let rocket = construct_from_cli();
        let client = Client::new(rocket).expect("valid rocket instance");

        let mut response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::HTML));
        assert!(!response.body_string().expect("contains html page").is_empty(), "Body should not be empty");
    }

    #[test]
    fn should_get_image_from_storage_folder() {
        let test_folder_id = nanoid!(10);
        let default_app_storage_folder = std::env::temp_dir().join("illuin").join(test_folder_id);

        let config = AppConfig {
            port: 8080,
            storage_path: String::from(default_app_storage_folder.to_str().unwrap()),
            tls: false,
            base_route: String::from("/"),
        };

        // GIVEN
        let id = nanoid!(10);
        let image_name = format!("{}.jpg", id);
        let image_path = Path::new("resources/marius.jpeg");
        let image_hash = hash_binary(fs::read(image_path).expect("read reference picture"));

        // copy an image in the app storage folder
        ensure_storage_path_exist(default_app_storage_folder.to_str().unwrap());
        std::fs::copy(image_path, default_app_storage_folder.join(&image_name)).expect("copy to app storage folder to succeed");

        // Creating rocket server
        let rocket = build_rocket(config);
        let client = Client::new(rocket).expect("valid rocket instance");

        // WHEN
        // Requesting
        let mut response = client.get(format!("/i/{}", image_name)).dispatch();

        // THEN
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JPEG));
        assert_eq!(image_hash, hash_binary(response.body_bytes().expect("body to contains something")));

        // CLEANING
        clean_test_folder(default_app_storage_folder.to_str().unwrap().to_string());
    }

    #[test]
    fn should_succeed_when_using_a_base_url() {
        let test_folder_id = nanoid!(10);
        let default_app_storage_folder = std::env::temp_dir().join("illuin").join(test_folder_id);

        let config = AppConfig {
            port: 8080,
            storage_path: String::from(default_app_storage_folder.to_str().unwrap()),
            tls: false,
            base_route: String::from("/base/"),
        };

        // GIVEN
        let id = nanoid!(10);
        let image_name = format!("{}.jpg", id);
        let image_path = Path::new("resources/marius.jpeg");
        let image_hash = hash_binary(fs::read(image_path).expect("read reference picture"));

        // copy an image in the app storage folder
        ensure_storage_path_exist(default_app_storage_folder.to_str().unwrap());
        std::fs::copy(image_path, default_app_storage_folder.join(&image_name)).expect("copy to app storage folder to succeed");

        // Creating rocket server
        let rocket = build_rocket(config);
        let client = Client::new(rocket).expect("valid rocket instance");

        // WHEN
        // Requesting
        let response = client.get(format!("/base/i/{}", image_name)).dispatch();

        // THEN
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JPEG));

        // CLEANING
        clean_test_folder(default_app_storage_folder.to_str().unwrap().to_string());
    }

    #[test]
    fn should_fail_when_requesting_a_route_that_doesnt_exists() {
        let rocket = construct_from_cli();
        let client = Client::new(rocket).expect("valid rocket instance");

        let response = client.get("/does_not_exist").dispatch();

        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn should_fail_when_requesting_a_picture_that_doesnt_exist() {
        let rocket = construct_from_cli();
        let client = Client::new(rocket).expect("valid rocket instance");

        let response = client.get("/i/UnknownHash.jpg").dispatch();

        assert_eq!(response.status(), Status::InternalServerError); // `Must be Status::NotFound`
    }

    // WIP: Needs to build multipost localrequest
    // #[test]
    // fn should_from_post_write_image_to_storage_folder() {
    //     let test_folder_id = nanoid!(10);
    //     let default_app_storage_folder = Path::new("/tmp/illuin").join(test_folder_id);
    //
    //     let config = AppConfig {
    //         port: 8080,
    //         storage_path: String::from(default_app_storage_folder.to_str().unwrap())
    //     };
    //
    //     // GIVEN
    //     let image_path = Path::new("resources/marius.jpeg");
    //     let image_hash = hash_binary(fs::read(image_path).expect("read reference picture"));
    //
    //     // Creating rocket server
    //     let rocket = build_rocket(config);
    //     let client = Client::new(rocket).expect("valid rocket instance");
    //
    //     // WHEN
    //     // WIP: Needs to build multipost localrequest
    //     let mut reponse = client.post("/i/upload")
    //         .body(); // TODO: to multipart post
    //
    //     // then
    //     assert_eq!(response.status(), Status::Ok);
    //     assert_eq!(response.content_type(), Some(ContentType::HTML));
    //     assert!(!response.body_string().expect("contains html page").is_empty(), "Body should not be empty"); // We might want to be sure that it contains an image URL
    //
    //     // CLEANING
    //     clean_test_folder(default_app_storage_folder.to_str().unwrap().to_string());
    // }

    // HELPERS
    fn hash_binary(binary: Vec<u8>) -> u64 {
        let mut hasher = DefaultHasher::new();
        binary.hash(&mut hasher);
        hasher.finish()
    }

    fn clean_test_folder(test_dir: String) {
        let default_app_storage_folder = Path::new(&test_dir);

        fs::remove_dir_all(default_app_storage_folder).expect("folder to be deleted");
    }
}
