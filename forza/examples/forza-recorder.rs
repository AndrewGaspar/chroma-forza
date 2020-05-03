use tokio::prelude::*;

use std::{error::Error, mem::size_of_val, slice};

use clap::Arg;
use futures_util::pin_mut;
use tokio::{fs::File, signal::ctrl_c, stream::StreamExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap::App::new("forza-recorder")
        .arg(
            Arg::with_name("local_addr")
                .short("l")
                .long("local-addr")
                .default_value("0.0.0.0:18733"),
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .possible_values(&["json", "raw"])
                .default_value("json"),
        )
        .arg(
            Arg::with_name("separator")
                .short("s")
                .long("separator")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true),
        )
        .get_matches();

    // Box the inner-most file stream - performacne at this level is inconsequential.
    let output: Box<dyn AsyncWrite + Unpin> = if let Some(output) = matches.value_of("output") {
        Box::new(File::create(output).await?)
    } else {
        Box::new(io::stdout())
    };

    let mut output = io::BufWriter::new(Box::pin(output));

    let server_addr = matches.value_of("local_addr").unwrap();
    let format = matches.value_of("format").unwrap();

    let stream = forza::horizon4(server_addr).await?;
    pin_mut!(stream);
    match format {
        "json" => {
            let separator = matches.value_of("separator").unwrap_or("\n");

            while let Some(datagram) = tokio::select! {
                datagram = stream.next() => datagram,
                _ = ctrl_c() => None,
            } {
                let datagram = datagram?;
                output
                    .write(serde_json::to_string(&datagram)?.as_bytes())
                    .await?;
                output.write(separator.as_bytes()).await?;
            }
        }
        "raw" => {
            while let Some(datagram) = tokio::select! {
                datagram = stream.next() => datagram,
                _ = ctrl_c() => None,
            } {
                let datagram = datagram?;
                let datagram = unsafe {
                    slice::from_raw_parts(&datagram as *const _ as *const _, size_of_val(&datagram))
                };
                output.write(datagram).await?;
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}
