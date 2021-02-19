use rocket::State;
use crate::cli;
use rocket_contrib::templates::Template;
use rocket::response::Debug;
use std::io;

mod img;

pub fn register_routes(base_route: &String, rocket: rocket::Rocket) -> rocket::Rocket {
    let rocket = rocket.mount(base_route, routes![get_index]);
    img::register_routes(base_route, rocket)
}

#[derive(Serialize)]
struct IndexTemplateContext {
    base: String,
}

#[get("/")]
fn get_index(config: State<cli::AppConfig>) -> Result<Template, Debug<io::Error>> {
    let ctx = IndexTemplateContext { base: config.base_route.clone() };
    Ok(Template::render("index", &ctx))
}
