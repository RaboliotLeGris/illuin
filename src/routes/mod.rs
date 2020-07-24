mod index;
mod img;

pub fn register_routes(rocket: rocket::Rocket) -> rocket::Rocket {
    let routes = index::register_routes(rocket);
    img::register_routes(routes)
}
