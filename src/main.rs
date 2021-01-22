use crate::stream::start_event_thread;
use crate::stream::start_event_thread_blocking;

mod stream;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let (handle, rx) = start_event_thread().await;

    for received in rx {
        println!("Got: {}", received);
    }

    handle.await;
    Ok(())
}

//io::stdout().write_all(&chunk).await?;
//let xd = serde_json::from_str("{}")?;
//let xd = serde_json::from_str(std::str::from_utf8(chunk.as_ref())?)?;


// let xd: Value = serde_json::from_str("{\"jebanie\":\"disa\"}")?;
// if let Some(x) = xd["jebanie"].as_str() {
//     println!("Please call {}", x);
// }