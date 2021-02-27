use std::io::Error;
use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, Sender};

use hyper::{Body, body::HttpBody as _, Client, Request};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use tokio::io::ErrorKind;
use tokio::task::JoinHandle;

use crate::stream::game_event::GameEvent;
use crate::stream::platform_event::PlatformEvent;

pub mod platform_event;
pub mod game_event;
pub mod model;

const URL: &str = "HTTPS://lichess.org/api";
const EVENT_ENDPOINT: &str = "stream/event";
const GAME_STREAM_ENDPOINT: &str = "bot/game/stream";
const CHALLENGE_ENDPOINT: &str = "challenge";
const EMPTY_MESSAGE: &str = "\n";

type SecureClient = Client<HttpsConnector<HttpConnector>, hyper::Body>;

pub struct Lichess {
    client: Arc<SecureClient>,
    token: String,
}

impl Lichess {
    pub fn new(token: String) -> Lichess {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        Lichess {
            client: Arc::new(client),
            token,
        }
    }

    pub async fn get_incoming_event_stream(&self) -> (JoinHandle<()>, Receiver<PlatformEvent>) {
        Lichess::get_event_stream(Arc::clone(&self.client), format!("{}", EVENT_ENDPOINT), self.token.clone()).await
    }

    pub async fn get_game_event_steam(&self, game: &str) -> (JoinHandle<()>, Receiver<GameEvent>) {
        Lichess::get_event_stream(Arc::clone(&self.client), format!("{}/{}", GAME_STREAM_ENDPOINT, game), self.token.clone()).await
    }

    pub async fn accept_challenge(&self, challenge: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let request = Request::post(format!("{}/{}/{}/{}", URL, CHALLENGE_ENDPOINT, challenge, "accept"))
            .header("Authorization", format!("Bearer {}", self.token))
            .body(Body::from(""))?;

        let resp = self.client.request(request).await?;

        if !resp.status().is_success() {
            return Err(Box::new(Error::new(ErrorKind::Other, resp.status().as_str())));
        }

        Ok(())
    }

    pub async fn make_move(&self, game: &str, game_move: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        println!("{}/bot/game/{}/move/{}", URL, game, game_move);

        let request = Request::post(format!("{}/bot/game/{}/move/{}", URL, game, game_move))
            .header("Authorization", format!("Bearer {}", self.token))
            .body(Body::from(""))?;

        let resp = self.client.request(request).await?;

        if !resp.status().is_success() {
            return Err(Box::new(Error::new(ErrorKind::Other, resp.status().as_str())));
        }

        Ok(())
    }
}

impl Lichess {
    async fn get_event_stream<T>(client: Arc<SecureClient>, endpoint: String, token: String) -> (JoinHandle<()>, Receiver<T>)
        where T: 'static + Send + Sync + DeserializeOwned {
        let (tx, rx): (Sender<T>, Receiver<T>) = mpsc::channel();
        let handle = tokio::spawn(Lichess::start_event_thread(client, tx, endpoint, token));

        (handle, rx)
    }

    async fn start_event_thread<T>(client: Arc<SecureClient>, tx: Sender<T>, endpoint: String, token: String)
        where T: 'static + Send + Sync + DeserializeOwned {
        if let Err(e) = Lichess::events(client, tx, endpoint, token).await { panic!(e.to_string()) }
    }

    async fn events<T>(client: Arc<SecureClient>, tx: Sender<T>, endpoint: String, token: String) -> std::result::Result<(), Box<dyn std::error::Error>>
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
}