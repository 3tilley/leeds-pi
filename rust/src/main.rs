// #[macro_use] extern crate rocket;

use std::net::Ipv4Addr;
use std::{io, thread};
use std::time::Duration;
// use futures_util::stream::stream::StreamExt;
use futures_util::{SinkExt, StreamExt};
use lazy_static::lazy_static;
use poem::{EndpointExt, get, handler, IntoResponse, Route, Server};
use poem::listener::TcpListener;
use poem::middleware::Tracing;
use poem::web::{Data, Html, Path};
use poem::web::websocket::{Message, WebSocket};
// use crate::antenna::{JunkData, listen_and_record};
use crate::rpi::{play_tone, play_tune};
use crate::frequency::*;

mod rpi;
mod antenna;
mod frequency;
// use rocket::Config;

const PORT: u32 = 8080;
lazy_static! {

static ref STATIC_CHAT_HTML: String = {
        let html = r###"
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
            ws = new WebSocket("ws://127.0.0.1:{PORT}/ws/" + nameInput.value);
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
    "###.replace("{PORT}", PORT.to_string().as_str());
        html
    };
}

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


#[handler]
fn beep_brief() -> &'static str {
    rpi::beep_freq(1, 500);
    "Beep"
}

#[handler]
fn beep_freq_route(Path((times, millis)): Path<(u8, u64)>) -> String {
    rpi::beep_freq(times, millis);
    format!("Times: {}. Millis: {}", times, millis)
}

#[handler]
fn blink_route() -> &'static str {
    rpi::blink(1);

    "Blink"
}

#[handler]
fn blink_times(Path(times): Path<u8>) -> String {
    rpi::blink(times);
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

#[handler]
fn tone_route(Path((note, secs)): Path<(u64, u8)>) -> String {
    play_tone(440.0, 16 * secs, 60);
    "Tone".to_string()
}

#[handler]
fn toxic() -> String {
    let tune = vec![
        (C_5, 2),
        (C_5, 2),
        (EB_5, 1),
        (D_5, 1),
        (C_5, 1),
        (D_5, 1),
        (EB_5, 3),
        (C_5, 1),
        (EB_5, 1),
        (D_5, 1),
        (C_5, 2),
        // Bar 2
        (GB_8, 4),
        (D_8, 4),
        (EB_8, 2),
        (D_8, 2),
        (C_8, 4)
    ];
    play_tune(tune, 144);
    "Tone".to_string()
}

#[handler]
fn chat() -> Html<&'static str> {
    Html(STATIC_CHAT_HTML.as_str())
}

#[handler]
fn antenna_data() -> Vec<u8> {
    // let junk = JunkData::new(0, Some(100), 32);
    // let mut vec: Vec<u8> = Vec::new();
    // {
    //     let mut s : Box<dyn io::Write> = Box::new(&mut vec);
    //     listen_and_record(Box::new(junk), &mut s);
    // }
    // vec
    // format!("{}", s.)
    vec![1,2,3]
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
        .at("/antenna", get(antenna_data))
        .at("/blink", get(blink_route))
        .at("/blink/:times", get(blink_times))
        .at("/beep", get(beep_brief))
        .at("/beep/:times/:level/:millis", get(beep_freq_route))
        .at("/tone/:note/:secs", get(tone_route))
        .at("/toxic", get(toxic))
        .with(Tracing);
    Server::new(TcpListener::bind(format!("0.0.0.0:{}", PORT)))
        .name("hello-world")
        .run(app)
        .await
}
