// #[macro_use] extern crate rocket;

use std::net::Ipv4Addr;
use std::thread;
use std::time::Duration;
// use futures_util::stream::stream::StreamExt;
use futures_util::{SinkExt, StreamExt};
use poem::{EndpointExt, get, Route, Server, handler, IntoResponse};
use poem::listener::TcpListener;
use poem::middleware::Tracing;
use poem::web::{Data, Html, Path};
use poem::web::websocket::{Message, WebSocket};
// use rocket::Config;

// These are Broadcom pins (BCM), they correspond to physical pins 15 and 16 respectively
// https://electronicsmith.com/raspberry-pi-pinout-for-all-models/
//const GPIO_BUZZER: u8 = 22;
const GPIO_BUZZER: u8 = 27;
const GPIO_LED: u8 = 23;

#[handler]
fn index() -> String {
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        let info = rppal::system::DeviceInfo::new().unwrap().model();
        format!("This is the Rust service running on {}", info)
    }
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    {
        let info = "Windows";
        format!("This is the Rust service running on {}", info)
    }
}

#[handler]
fn hello() -> &'static str {
    "Hello, world!"
}

#[handler]
fn hello_name(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

fn beep_freq(times: u8, voltage: u8, millis: u64) {
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
    let mut pin = rppal::gpio::Gpio::new().unwrap().get(GPIO_BUZZER).unwrap().into_output();
    }

    for _ in 0..times {
        // Blink the LED by setting the pin's logic level high for 500 ms.
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        {
            println!("beep");
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            pin.write(voltage.into());
        }
        thread::sleep(Duration::from_millis(millis));
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        {
            println!("beep");
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            pin.set_low();
        }
        thread::sleep(Duration::from_millis(millis));
    }
}


#[handler]
fn beep_brief() -> &'static str {
    beep_freq(1, 255, 500);
    "Beep"
}

#[handler]
fn beep_freq_route(Path((times, level, millis)): Path<(u8, u8, u64)>) -> String {
    beep_freq(times, level, millis);
    format!("Times: {}. Level: {}. Millis: {}", times, level, millis)
}

fn blink(times: u8) {
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        let mut pin = rppal::gpio::Gpio::new().unwrap().get(GPIO_LED).unwrap().into_output();
    }

    for _ in 0..times {
        // Blink the LED by setting the pin's logic level high for 500 ms.
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        {
            println!("blink");
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            pin.set_high();
        }
        thread::sleep(Duration::from_millis(500));
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        {
            println!("blink");
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
                pin.set_low();
        }
        thread::sleep(Duration::from_millis(500));
    }
}

#[handler]
fn blink_route() -> &'static str {
    blink(1);

    "Blink"
}

#[handler]
fn blink_times(Path(times): Path<u8>) -> String {
    blink(times);
    format!("Blink {} times", times)
}

#[handler]
fn ws(
    Path(name): Path<String>,
    ws: WebSocket,
    sender: Data<&tokio::sync::broadcast::Sender<String>>,
) -> impl IntoResponse {
    let sender = sender.clone();
    let mut receiver = sender.subscribe();
    ws.on_upgrade(move |socket| async move {
        let (mut sink, mut stream) = socket.split();

        tokio::spawn(async move {
            while let Some(Ok(msg)) = stream.next().await {
                if let Message::Text(text) = msg {
                    println!("{:?}", &text);
                    if sender.send(format!("{}: {}", name, text)).is_err() {
                        break;
                    }
                }
            }
        });

        tokio::spawn(async move {
            while let Ok(msg) = receiver.recv().await {
                if sink.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }
        });
    })
}

// #[launch]
// fn rocket() -> _ {
//     let config = Config {
//         address: Ipv4Addr::new(0, 0, 0, 0).into(),
//         port: 80,
//         ..Config::debug_default()
//     };
//
//     rocket::custom(config).mount("/", routes![index, hello, hello_name])
// }

#[handler]
fn chat() -> Html<&'static str> {
    Html(
        r###"
    <body>
        <form id="loginForm">
            Name: <input id="nameInput" type="text" />
            <button type="submit">Login</button>
        </form>

        <form id="sendForm" hidden>
            Text: <input id="msgInput" type="text" />
            <button type="submit">Send</button>
        </form>

        <textarea id="msgsArea" cols="50" rows="30" hidden></textarea>
    </body>
    <script>
        let ws;
        const loginForm = document.querySelector("#loginForm");
        const sendForm = document.querySelector("#sendForm");
        const nameInput = document.querySelector("#nameInput");
        const msgInput = document.querySelector("#msgInput");
        const msgsArea = document.querySelector("#msgsArea");

        nameInput.focus();
        loginForm.addEventListener("submit", function(event) {
            event.preventDefault();
            loginForm.hidden = true;
            sendForm.hidden = false;
            msgsArea.hidden = false;
            msgInput.focus();
            ws = new WebSocket("ws://127.0.0.1:3000/ws/" + nameInput.value);
            ws.onmessage = function(event) {
                msgsArea.value += event.data + "\r\n";
            }
        });

        sendForm.addEventListener("submit", function(event) {
            event.preventDefault();
            ws.send(msgInput.value);
            msgInput.value = "";
        });
    </script>
    "###,
    )
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let app = Route::new()
        .at("/chat", get(chat))
        .at("/ws/:name", get(ws.data(tokio::sync::broadcast::channel::<String>(32).0)))
        .at("/", get(index))
        .at("/hello", get(hello))
        .at("/hello/:name", get(hello_name))
        .at("/blink", get(blink_route))
        .at("/blink/:times", get(blink_times))
        .at("/beep", get(beep_brief))
        .at("/beep/:times/:level/:millis", get(beep_freq_route)).with(Tracing);
    Server::new(TcpListener::bind("0.0.0.0:8080"))
        .name("hello-world")
        .run(app)
        .await
}
