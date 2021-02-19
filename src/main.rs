#![feature(proc_macro_hygiene, decl_macro)]

extern crate nanoid;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[cfg(test)]
mod api_tests;

mod routes;
mod cli;

fn construct_from_cli() -> rocket::Rocket {
    let app_config = cli::get_config();
    ensure_storage_path_exist(&app_config.storage_path);

    build_rocket(app_config)
}

fn build_rocket(app_config: cli::AppConfig) -> rocket::Rocket {
    // If the env var PORT is present, then it overwrites the one provided by the config
    let port = std::env::var("PORT")
        .unwrap_or(app_config.port.to_string())
        .parse()
        .unwrap_or(8080);

    let rocket_config = rocket::Config::build(rocket::config::Environment::Production)
        .address("0.0.0.0")
        .port(port)
        .finalize().unwrap();

    let router = rocket::custom(rocket_config);

    routes::register_routes(&app_config.base_route, router)
        .attach(Template::fairing())
        .manage(app_config)
}

fn main() {
    construct_from_cli().launch();
}

fn ensure_storage_path_exist(path: &str) {
    match std::fs::create_dir(path) {
        Ok(_t) => {}
        Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
        Err(e) => panic!("{}",e),
    };
}
