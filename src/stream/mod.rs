use hyper::{body::HttpBody as _, Client, Request, Body};
use hyper_tls::HttpsConnector;
use tokio::task::JoinHandle;
use std::sync::mpsc::{self, Sender, Receiver};
use crate::stream::platform_event::Event;

mod platform_event;

const URL: &str = "https://lichess.org/api";
const EVENT_ENDPOINT: &str = "stream/event";
const BEARER_TOKEN: &str = "eOcwgviIZY7b9ZU3";
const EMPTY_MESSAGE: &str = "\n";

pub async fn get_incoming_event_stream() -> (JoinHandle<()>, Receiver<String>){
    get_event_stream(EVENT_ENDPOINT).await
}

async fn get_event_stream(endpoint: &'static str) -> (JoinHandle<()>, Receiver<String>) {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    let handle = tokio::spawn(start_event_thread(tx, endpoint));

    (handle, rx)
}

async fn start_event_thread(tx: Sender<String>, endpoint: &str) {
    if let Err(e) = events(tx, endpoint).await { panic!(e.to_string()) }
}

async fn events(tx: Sender<String>, endpoint: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("Starting the listener");
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let request = Request::get(format!("{}/{}", URL, endpoint))
        .header("Authorization", format!("Bearer {}", BEARER_TOKEN))
        .body(Body::from(""))?;

    let mut resp = client.request(request).await?;

    while let Some(next) = resp.data().await {
        let chunk = next?;
        let slice = std::str::from_utf8(chunk.as_ref())?;

        if slice != EMPTY_MESSAGE {
            tx.send(slice.to_string())?;

            let xd: Event = serde_json::from_str(slice).unwrap();

            println!("{:#?}", xd);
        }
    }
    println!("Stream ended");

    Ok(())
}