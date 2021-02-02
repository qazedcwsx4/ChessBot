use crate::stream::Lichess;
use crate::stream::platform_event::PlatformEvent;
use crate::stream::platform_event::PlatformEvent::{Challenge, GameStart};
use std::borrow::Borrow;

mod stream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lichess = Lichess::new();
    let (handle, rx) = lichess.get_incoming_event_stream().await;

    for received in rx {
        match received {
            Challenge { challenge } => {
                println!("Got challenge");
                lichess.accept_challenge(challenge.id.as_str()).await?;
            },
            GameStart{game} => {
                println!("Started game");
                enter_game(lichess.borrow(), game.id.borrow()).await?;
            }
            _ => {
                println!("Got: {:#?}", received);
            }
        }
    }

    handle.await?;
    Ok(())
}

async fn enter_game(lichess : &Lichess, game: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (handle, rx) = lichess.get_game_event_steam(game).await;

    for received in rx {
        println!("Got: {:#?}", received);
    }

    handle.await?;
    Ok(())
}