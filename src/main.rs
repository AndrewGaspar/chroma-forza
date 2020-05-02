use std::env;
use std::mem::size_of;

use chroma::{Key, KeyboardCustomKeyEffectBuilder};
use futures_util::pin_mut;
use rgb::RGB8;
use tokio::stream::StreamExt;

const NUM_KEYS: [chroma::Key; 9] = [
    Key::Row1,
    Key::Row2,
    Key::Row3,
    Key::Row4,
    Key::Row5,
    Key::Row6,
    Key::Row7,
    Key::Row8,
    Key::Row9,
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(232, size_of::<forza::Sled>());
    assert_eq!(79, size_of::<forza::Dash>());
    assert_eq!(324, size_of::<forza::Horizon4Datagram>());

    let server_ip = env::args().skip(1).next().unwrap_or("0.0.0.0".to_string());
    let server_addr = server_ip + ":8000";

    let stream = forza::horizon4(server_addr).await?;
    pin_mut!(stream);
    while let Some(Ok(datagram)) = stream.next().await {
        let mut builder = KeyboardCustomKeyEffectBuilder::new();
        if datagram.sled.is_race_on > 0 {
            builder.set_key(
                Key::Space,
                RGB8 {
                    r: 0xff,
                    g: 0xff,
                    b: 0xff,
                },
            );
        }

        let position = datagram.dash.race_position;
        if position > 0 && position < 10 {
            let key = NUM_KEYS[position as usize - 1];
            let color = if position >= 7 {
                RGB8 {
                    r: 0xff,
                    g: 0,
                    b: 0,
                }
            } else if position >= 4 {
                RGB8 {
                    r: 0,
                    g: 0,
                    b: 0xff,
                }
            } else {
                RGB8 {
                    r: 0,
                    g: 0xff,
                    b: 0,
                }
            };

            builder.set_key(key, color);
        }

        builder.build()?.set()?;
    }

    Ok(())
}
