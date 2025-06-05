pub const CARGO_TOML: &str = r#"[package]
name = "{{project_name}}"
version = "0.1.0"
edition = "2021"

[dependencies]
rocket = { version = "0.5.1", features = ["json"] }
"#;

pub const MAIN_RS: &str = r#"#[macro_use] extern crate rocket;

mod routes;

#[get("/")]
fn index() -> &'static str {
    "Hello, {{project_name}}!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
"#;

pub const ROUTES_MOD: &str = r#"pub fn routes() -> Vec<rocket::Route> {
    routes![index]
}

#[get("/")]
fn index() -> &'static str {
    "Hello, {{project_name}}!"
}
"#;
