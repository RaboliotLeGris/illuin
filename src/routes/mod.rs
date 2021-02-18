mod img;

pub fn register_routes(base_route: &String, rocket: rocket::Rocket) -> rocket::Rocket {
    img::register_routes(base_route, rocket)
}
