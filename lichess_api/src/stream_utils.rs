use std::sync::{Arc, mpsc};
use std::sync::mpsc::{Receiver, Sender};

use hyper::{Body, body::HttpBody as _, Request};
use serde::de::DeserializeOwned;
use tokio::task::JoinHandle;

use crate::lichess::SecureClient;

pub const URL: &str = "HTTPS://lichess.org/api";
const EMPTY_MESSAGE: &str = "\n";

pub async fn get_event_stream<T>(client: Arc<SecureClient>, endpoint: String, token: String) -> (JoinHandle<()>, Receiver<T>)
    where T: 'static + Send + Sync + DeserializeOwned {
    let (tx, rx): (Sender<T>, Receiver<T>) = mpsc::channel();
    let handle = tokio::spawn(start_event_thread(client, tx, endpoint, token));

    (handle, rx)
}

pub async fn start_event_thread<T>(client: Arc<SecureClient>, tx: Sender<T>, endpoint: String, token: String)
    where T: 'static + Send + Sync + DeserializeOwned {
    if let Err(e) = events(client, tx, endpoint, token).await { panic!(e.to_string()) }
}

pub async fn events<T>(client: Arc<SecureClient>, tx: Sender<T>, endpoint: String, token: String) -> std::result::Result<(), Box<dyn std::error::Error>>
    where T: 'static + Send + Sync + DeserializeOwned {
    println!("Starting the listener");
    let request = Request::get(format!("{}/{}", URL, endpoint))
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(""))?;

    let mut resp = client.request(request).await?;
    while let Some(next) = resp.data().await {
        let bytes = next?;
        let slice = std::str::from_utf8(bytes.as_ref())?;

        if slice != EMPTY_MESSAGE {
            println!("{:#?}", slice);
            let json: T = serde_json::from_str(slice)?;

            tx.send(json)?;
        }
    }
    Ok(())
}
