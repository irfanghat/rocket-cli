#[macro_use] extern crate rocket;

mod routes;

#[get("/")]
fn index() -> &'static str {
    "Hello, demo!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
