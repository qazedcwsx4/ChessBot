use crate::stream::get_incoming_event_stream;

mod stream;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let (handle, rx) = get_incoming_event_stream().await;

    for received in rx {
        println!("Got: {}", received);
    }

    handle.await?;
    Ok(())
}