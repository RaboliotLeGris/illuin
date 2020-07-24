#![feature(proc_macro_hygiene, decl_macro)]

extern crate nanoid;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use rocket_contrib::templates::Template;

mod routes;
mod cli;

fn main() {
    let app_config = cli::get_config();
    ensure_storage_path_exist(&app_config.storage_path);

    let rocket_config = rocket::Config::build(rocket::config::Environment::Development)
        .port(app_config.port)
        .finalize().unwrap();

    let router = rocket::custom(rocket_config);

    routes::register_routes(router)
        .attach(Template::fairing())
        .manage(app_config)
        .launch();
}

fn ensure_storage_path_exist(path: &str) {
    match std::fs::create_dir(path) {
        Ok(_t) => {}
        Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
        Err(e) => panic!(e),
    };
}