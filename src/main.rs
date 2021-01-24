use crate::stream::{Lichess};

mod stream;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let lichess = Lichess::new();
    let (handle, rx) = lichess.get_incoming_event_stream().await;

    for received in rx {
        println!("Got: {}", received);
    }

    handle.await?;
    Ok(())
}