use std::env;
use std::mem::size_of;

use futures_util::pin_mut;
use tokio::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(232, size_of::<forza::Sled>());
    assert_eq!(79, size_of::<forza::Dash>());
    assert_eq!(324, size_of::<forza::Horizon4Datagram>());

    let server_ip = env::args().skip(1).next().unwrap();
    let server_addr = server_ip + ":8000";

    let stream = forza::horizon4(server_addr).await?;
    pin_mut!(stream);
    while let Some(Ok(datagram)) = stream.next().await {
        println!("{}", serde_json::to_string(&datagram).unwrap());
    }

    Ok(())
}
