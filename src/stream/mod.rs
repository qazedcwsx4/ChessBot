use hyper::{body::HttpBody as _, Client, Request, Body};
use hyper::header;
use tokio::io;
use serde_json::{Value};
use hyper::http::HeaderValue;
use hyper_tls::HttpsConnector;
use std::thread::{sleep};
use std::time;
use std::error::Error;
use tokio::task;
use tokio::task::JoinHandle;
use std::sync::mpsc::{self, Sender, Receiver};
use tokio::runtime::Runtime;
use tokio::time::Duration;

const URL: &str = "https://lichess.org/api";
const ENDPOINT: &str = "stream/event";
const BEARER_TOKEN: &str = "eOcwgviIZY7b9ZU3";
const EMPTY_MESSAGE: &str = "\n";

pub fn start_event_thread_blocking() -> (JoinHandle<()>, Receiver<String>) {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let rt = Runtime::new().unwrap();

    let handle = rt.spawn(start_events(tx));

    (handle, rx)
}

pub async fn start_event_thread() -> (JoinHandle<()>, Receiver<String>) {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    let handle = tokio::spawn(start_events(tx));

    (handle, rx)
}

async fn start_events(tx: Sender<String>) {
    if let Err(e) = events(tx).await { panic!(e.to_string()) }
}

async fn events(tx: Sender<String>) -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("Starting the listener");
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let request = Request::get(format!("{}/{}", URL, ENDPOINT))
        .header("Authorization", format!("Bearer {}", BEARER_TOKEN))
        .body(Body::from(""))?;

    let mut resp = client.request(request).await?;

    while let Some(next) = resp.data().await {
        let chunk = next?;
        let slice = std::str::from_utf8(chunk.as_ref())?;

        if slice != EMPTY_MESSAGE {
            tx.send(slice.to_string());
        }
    }
    println!("Stream ended");

    Ok(())
}