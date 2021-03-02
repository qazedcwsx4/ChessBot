extern crate lichess_api;

use std::env;
use std::thread::sleep;

use tokio::time::Duration;

use lichess_api::game::Game;
use lichess_api::game_event::GameEvent;
use lichess_api::lichess::Lichess;
use lichess_api::platform_event::PlatformEvent::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("LICHESS_TOKEN")?;

    let lichess = Lichess::new(token);
    let (handle, rx) = lichess.get_incoming_event_stream().await;

    for received in rx {
        match received {
            Challenge { challenge } => {
                println!("Got challenge");
                lichess.accept_challenge(challenge.id).await?;
            },
            GameStart{game} => {
                println!("Started game");
                let game = lichess.open_game(game.id).await?;
                enter_game(game).await?;
            }
            _ => {
                println!("Got: {:#?}", received);
            }
        }
    }

    handle.await?;
    Ok(())
}

async fn enter_game(game: Game) -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, rx) = game.get_game_event_steam().await;
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
}