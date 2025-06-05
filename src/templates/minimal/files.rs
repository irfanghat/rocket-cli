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
    routes![index, hello]
}

#[get("/")]
fn index() -> &'static str {
    "Hello, {{project_name}}!"
}

/*-----------------------------------------------------------
Rocket automatically parses dynamic data in path segments into any desired type. 

This hello route has two dynamic parameters, identified with angle brackets, declared in the route URI: <name> and <age>. Rocket maps each parameter to an identically named function argument: name: &str and age: u8. The dynamic data in the incoming request is parsed automatically into a value of the argument's type. The route is called only when parsing succeeds.

Parsing is directed by the FromParam trait. Rocket implements FromParam for many standard types, including both &str and u8. You can implement it for your own types, too!
-----------------------------------------------------------*/
#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}
"#;
