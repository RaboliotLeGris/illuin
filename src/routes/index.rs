use std::collections::HashMap;
use std::io::Error;

use rocket::response::Debug;
use rocket_contrib::templates::Template;

pub fn register_routes(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/", routes![get_index])
}

#[get("/")]
fn get_index() -> Result<Template, Debug<Error>> {
    let ctx: HashMap<&str, &str> = HashMap::new();
    Ok(Template::render("index", &ctx))
}