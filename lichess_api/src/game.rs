use std::io::Error;
use std::sync::Arc;
use std::sync::mpsc::Receiver;

use hyper::{Body, Request};
use tokio::io::ErrorKind;
use tokio::task::JoinHandle;

use crate::game_event::GameEvent;
use crate::lichess::SecureClient;
use crate::stream_utils::{get_event_stream, URL};

const GAME_STREAM_ENDPOINT: &str = "bot/game/stream";

pub struct Game {
    client: Arc<SecureClient>,
    token: String,
    game_id: String,
}

impl Game {
    pub fn new(client: Arc<SecureClient>, token: String, game_id: String) -> Game {
        Game {
            client,
            token,
            game_id,
        }
    }


    pub async fn get_game_event_steam(&self) -> (JoinHandle<()>, Receiver<GameEvent>) {
        get_event_stream(Arc::clone(&self.client), format!("{}/{}", GAME_STREAM_ENDPOINT, self.game_id), self.token.clone()).await
    }

    pub async fn make_move(&self, game_move: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let request = Request::post(format!("{}/bot/game/{}/move/{}", URL, self.game_id, game_move))
            .header("Authorization", format!("Bearer {}", self.token))
            .body(Body::from(""))?;

        let resp = self.client.request(request).await?;

        if !resp.status().is_success() {
            return Err(Box::new(Error::new(ErrorKind::Other, resp.status().as_str())));
        }

        Ok(())
    }
}