#![recursion_limit = "256"]

use tokio::prelude::*;

use std::{
    mem::{size_of_val, MaybeUninit},
    pin::Pin,
    process,
};

use chroma::{Key, KeyboardCustomKeyEffectBuilder};
use clap::Arg;
use forza::Horizon4Datagram;
use futures_util::pin_mut;
use rgb::RGB8;
use tokio::{
    stream::StreamExt,
    time::{Duration, Instant},
};

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
    let matches = clap::App::new("forza-chroma")
        .arg(
            Arg::with_name("local_addr")
                .short("l")
                .long("local-addr")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("recording")
                .short("r")
                .long("recording")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .possible_values(&["json", "raw"])
                .default_value("raw"),
        )
        .get_matches();

    let local_addr = matches.value_of("local_addr");
    let recording = matches.value_of("recording");

    if local_addr.is_some() && recording.is_some() {
        eprintln!("Error: You cannot specify a recording file and local-addr at the same time");
        process::exit(1);
    }

    let stream: Pin<Box<dyn tokio::stream::Stream<Item = std::io::Result<Horizon4Datagram>>>> =
        if let Some(recording) = recording {
            let recording = tokio::fs::File::open(recording).await?;
            let mut recording = Box::pin(io::BufReader::new(recording));
            Box::pin(async_stream::try_stream! {
                let start_time = Instant::now();
                let mut start = None;
                loop {
                        let mut datagram = MaybeUninit::<Horizon4Datagram>::zeroed();
                        let slice = unsafe { std::slice::from_raw_parts_mut(&mut datagram as *mut _ as *mut _, size_of_val(&datagram))};
                        let read = recording.read_exact(slice).await?;
                        if read < size_of_val(&datagram) {
                            break;
                        }
                        let datagram = unsafe { datagram.assume_init() };
                        if let Some((start_time, first_recorded_time)) = start.as_ref() {
                            let since_first_time = Duration::from_millis(datagram.sled.timestamp_ms.wrapping_sub(*first_recorded_time) as u64);
                            tokio::time::delay_until(*start_time + since_first_time).await;
                        } else {
                            start = Some((Instant::now(), datagram.sled.timestamp_ms));
                        }
                        yield datagram;
                }
            })
        } else {
            let local_addr = local_addr.unwrap_or("0.0.0.0:18733");
            Box::pin(forza::horizon4(local_addr).await?)
        };

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

        let pct_rpm = (datagram.sled.current_engine_rpm - datagram.sled.engine_idle_rpm)
            / (datagram.sled.engine_max_rpm - datagram.sled.engine_idle_rpm);
        let column_height = 6.0;
        let shade = column_height * pct_rpm;
        // round down
        let num_filled = shade as u8;
        for row in 0..num_filled {
            builder.set_position(
                5 - row,
                14,
                RGB8 {
                    r: 0xff,
                    g: 0,
                    b: 0,
                },
            );
        }
        builder.set_position(
            5 - num_filled,
            14,
            RGB8 {
                r: (256f32 * (shade % 1.0)) as u8,
                g: 0,
                b: 0,
            },
        );

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
