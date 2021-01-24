use hyper::{body::HttpBody as _, Client, Request, Body};
use hyper_tls::HttpsConnector;
use tokio::task::JoinHandle;
use std::sync::mpsc::{self, Sender, Receiver};
use crate::stream::platform_event::Event;
use hyper::client::HttpConnector;
use hyper::client::connect::Connect;
use std::sync::Arc;

mod platform_event;

const URL: &str = "HTTPS://lichess.org/api";
const EVENT_ENDPOINT: &str = "stream/event";
const BEARER_TOKEN: &str = "eOcwgviIZY7b9ZU3";
const EMPTY_MESSAGE: &str = "\n";

type SecureClient = Client<HttpsConnector<HttpConnector>, hyper::Body>;

pub struct Lichess {
    client: Arc<SecureClient>
}

impl Lichess {
    pub fn new() -> Lichess {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        Lichess { client: Arc::new(client) }
    }

    pub async fn get_incoming_event_stream(&self) -> (JoinHandle<()>, Receiver<String>) {
        Lichess::get_event_stream(Arc::clone(&self.client), EVENT_ENDPOINT).await
    }
}

impl Lichess {
    async fn get_event_stream(client: Arc<SecureClient>, endpoint: &'static str) -> (JoinHandle<()>, Receiver<String>) {
        let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
        let handle = tokio::spawn(Lichess::start_event_thread(client, tx, endpoint));

        (handle, rx)
    }

    async fn start_event_thread(client: Arc<SecureClient>, tx: Sender<String>, endpoint: &str) {
        if let Err(e) = Lichess::events(client, tx, endpoint).await { panic!(e.to_string()) }
    }

    async fn events(client: Arc<SecureClient>, tx: Sender<String>, endpoint: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        println!("Starting the listener");
        let request = Request::get(format!("{}/{}", URL, endpoint))
            .header("Authorization", format!("Bearer {}", BEARER_TOKEN))
            .body(Body::from(""))?;

        let mut resp = client.request(request).await?;

        while let Some(next) = resp.data().await {
            let chunk = next?;
            let slice = std::str::from_utf8(chunk.as_ref())?;

            if slice != EMPTY_MESSAGE {
                tx.send(slice.to_string())?;

                let xd: Event = serde_json::from_str(slice)?;

                println!("{:#?}", xd);
            }
        }
        println!("Stream ended");

        Ok(())
    }
}