use std::io::Error;
use std::sync::Arc;
use std::sync::mpsc::{Receiver};

use hyper::{Body, Client, Request};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use tokio::io::ErrorKind;
use tokio::task::JoinHandle;

use crate::game;
use crate::platform_event::PlatformEvent;
use crate::stream_utils::{get_event_stream, URL};

const EVENT_ENDPOINT: &str = "stream/event";
const CHALLENGE_ENDPOINT: &str = "challenge";

pub type SecureClient = Client<HttpsConnector<HttpConnector>, hyper::Body>;

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
        get_event_stream(Arc::clone(&self.client), format!("{}", EVENT_ENDPOINT), self.token.clone()).await
    }

    pub async fn accept_challenge(&self, challenge: String) -> std::result::Result<game::Game, Box<dyn std::error::Error>> {
        let request = Request::post(format!("{}/{}/{}/{}", URL, CHALLENGE_ENDPOINT, challenge, "accept"))
            .header("Authorization", format!("Bearer {}", self.token))
            .body(Body::from(""))?;

        let resp = self.client.request(request).await?;

        if !resp.status().is_success() {
            return Err(Box::new(Error::new(ErrorKind::Other, resp.status().as_str())));
        }

        Ok(game::Game::new(self.client.clone(), self.token.clone(), challenge))
    }

    pub async fn open_game(&self, game_id: String) -> std::result::Result<game::Game, Box<dyn std::error::Error>> {
        Ok(game::Game::new(self.client.clone(), self.token.clone(), game_id))
    }
}