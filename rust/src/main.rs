#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "This is the Rust service"
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello_name(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello, hello_name])
}
