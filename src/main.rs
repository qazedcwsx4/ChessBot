use crate::stream::{Lichess};
use crate::stream::platform_event::Event::Challenge;
use crate::stream::platform_event::Event;

mod stream;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let lichess = Lichess::new();
    let (handle, rx) = lichess.get_incoming_event_stream().await;

    for received in rx {
        match received {
            Challenge { challenge } => {
                println!("Got challenge");
                lichess.accept_challenge(challenge.id.as_str()).await?;
            }
            _ => {
                println!("Got: {:#?}", received);
            }
        }
    }

    handle.await?;
    Ok(())
}