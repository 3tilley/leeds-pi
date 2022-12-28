#[macro_use] extern crate rocket;

use std::net::Ipv4Addr;
use std::thread;
use std::time::Duration;
use rocket::Config;
use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

// These are Broadcom pins (BCM), they correspond to physical pins 15 and 16 respectively
// https://electronicsmith.com/raspberry-pi-pinout-for-all-models/
const GPIO_BUZZER: u8 = 22;
const GPIO_LED: u8 = 23;

#[get("/")]
fn index() -> String {
    format!("This is the Rust service running on {}", DeviceInfo::new().unwrap()?.model())
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
    let mut pin = Gpio::new().unwrap().get(GPIO_BUZZER).unwrap().into_output();

    // Blink the LED by setting the pin's logic level high for 500 ms.
    pin.set_high();
    thread::sleep(Duration::from_millis(500));
    pin.set_low();
    "Beep"
}

#[get("/blink")]
fn blink() -> &'static str {
    let mut pin = Gpio::new().unwrap().get(GPIO_LED).unwrap().into_output();

    // Blink the LED by setting the pin's logic level high for 500 ms.
    pin.set_high();
    thread::sleep(Duration::from_millis(500));
    pin.set_low();
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
        port: 80,
        ..Config::debug_default()
    };

    rocket::custom(config).mount("/", routes![index, hello, hello_name])
}
