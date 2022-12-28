#[macro_use] extern crate rocket;

use std::net::Ipv4Addr;
use rocket::Config;

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

#[get("/beep")]
fn beep_brief() -> &'static str {
    "Beep"
}

#[get("/blink")]
fn blink() -> &'static str {
    "Blink"
}

#[get("/blink/<times>")]
fn blink_times(times: u8) -> String {
    format!("Blink {} times", times)
}

#[launch]
fn rocket() -> _ {
    let config = Config {
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        ..Config::debug_default()
    };

    rocket::custom(config).mount("/", routes![index, hello, hello_name])
}
