mod img;

pub fn register_routes(rocket: rocket::Rocket) -> rocket::Rocket {
    img::register_routes(rocket)
}
