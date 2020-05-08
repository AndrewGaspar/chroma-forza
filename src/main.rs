#![recursion_limit = "256"]

use futures::prelude::*;
use tokio::prelude::*;

use std::process;

use clap::Arg;
use futures_util::pin_mut;
use tokio::{fs::File, signal::ctrl_c};

mod config;
mod driver;
mod effects;
mod property;
mod state;
mod stream;

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
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .default_value("configs/default.toml"),
        )
        .get_matches();

    let mut config = String::new();
    File::open(matches.value_of("config").unwrap())
        .await?
        .read_to_string(&mut config)
        .await?;
    let config: config::Config = toml::from_str(&config)?;

    let local_addr = matches.value_of("local_addr");
    let recording = matches.value_of("recording");

    if local_addr.is_some() && recording.is_some() {
        eprintln!("Error: You cannot specify a recording file and local-addr at the same time");
        process::exit(1);
    }

    let stream = stream::horizon4(local_addr, recording).await?;
    pin_mut!(stream);

    let cancellation = ctrl_c().map(|_| ());
    pin_mut!(cancellation);

    let driver = driver::Driver::from_config(&config);

    driver.run(stream, cancellation).await?;

    // while let Some(Ok(datagram)) = stream.next().await {
    //     let mut builder = KeyboardCustomKeyEffectBuilder::new();
    //     if datagram.sled.is_race_on > 0 {
    //         builder.set_key(
    //             Key::Space,
    //             RGB8 {
    //                 r: 0xff,
    //                 g: 0xff,
    //                 b: 0xff,
    //             },
    //         );
    //     }

    //     let pct_rpm = (datagram.sled.current_engine_rpm - datagram.sled.engine_idle_rpm)
    //         / (datagram.sled.engine_max_rpm - datagram.sled.engine_idle_rpm);
    //     let column_height = 6.0;
    //     let shade = column_height * pct_rpm;
    //     // round down
    //     let num_filled = shade as u8;
    //     for row in 0..num_filled {
    //         builder.set_position(
    //             5 - row,
    //             14,
    //             RGB8 {
    //                 r: 0xff,
    //                 g: 0,
    //                 b: 0,
    //             },
    //         );
    //     }
    //     builder.set_position(
    //         5 - num_filled,
    //         14,
    //         RGB8 {
    //             r: (256f32 * (shade % 1.0)) as u8,
    //             g: 0,
    //             b: 0,
    //         },
    //     );

    //     let position = datagram.dash.race_position;
    //     if position > 0 && position < 10 {
    //         let key = NUM_KEYS[position as usize - 1];
    //         let color = if position >= 7 {
    //             RGB8 {
    //                 r: 0xff,
    //                 g: 0,
    //                 b: 0,
    //             }
    //         } else if position >= 4 {
    //             RGB8 {
    //                 r: 0,
    //                 g: 0,
    //                 b: 0xff,
    //             }
    //         } else {
    //             RGB8 {
    //                 r: 0,
    //                 g: 0xff,
    //                 b: 0,
    //             }
    //         };

    //         builder.set_key(key, color);
    //     }

    //     builder.build()?.set()?;
    // }

    Ok(())
}
