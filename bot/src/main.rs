use std::{env};
use std::borrow::Borrow;
use std::thread::sleep;
use tokio::time::Duration;

use lichess_api::game_event::GameEvent;
use lichess_api::lichess::Lichess;
use lichess_api::platform_event::PlatformEvent::*;

extern crate lichess_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("LICHESS_TOKEN")?;

    let lichess = Lichess::new(token);
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
    let (_handle, rx) = lichess.get_game_event_steam(game).await;
    sleep(Duration::from_millis(100));
    //let result = lichess.make_move(game, "h2h3").await?;
    loop {
        for received in rx.try_iter() {
            println!("Got: {:#?}", received);
            match received {
                GameEvent::GameFull { .. } => {}
                GameEvent::GameState { .. } => {}
                GameEvent::ChatLine { .. } => {}
            }
        }

        sleep(Duration::from_millis(100))
    }
    //
    // println!("Got");
    // _handle.await?;
    // Ok(())
}