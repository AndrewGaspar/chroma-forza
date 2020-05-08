use tokio::prelude::*;

use std::{
    mem::{size_of_val, MaybeUninit},
    pin::Pin,
    slice,
};

use tokio::time::{delay_until, Duration, Instant};

use forza::Horizon4Datagram;

pub async fn horizon4<'a>(
    local_addr: Option<&'a str>,
    recording: Option<&'a str>,
) -> Result<
    Pin<Box<dyn 'a + tokio::stream::Stream<Item = std::io::Result<Horizon4Datagram>>>>,
    io::Error,
> {
    Ok(if let Some(recording) = recording {
        let recording = tokio::fs::File::open(recording).await?;
        let mut recording = Box::pin(io::BufReader::new(recording));
        Box::pin(async_stream::try_stream! {
            let start_time = Instant::now();
            let mut start = None;
            loop {
                    let mut datagram = MaybeUninit::<Horizon4Datagram>::zeroed();
                    let slice = unsafe { slice::from_raw_parts_mut(&mut datagram as *mut _ as *mut _, size_of_val(&datagram))};
                    let read = recording.read_exact(slice).await?;
                    if read < size_of_val(&datagram) {
                        break;
                    }
                    let datagram = unsafe { datagram.assume_init() };
                    if let Some((start_time, first_recorded_time)) = start.as_ref() {
                        let since_first_time = Duration::from_millis(datagram.sled.timestamp_ms.wrapping_sub(*first_recorded_time) as u64);
                        delay_until(*start_time + since_first_time).await;
                    } else {
                        start = Some((Instant::now(), datagram.sled.timestamp_ms));
                    }
                    yield datagram;
            }
        })
    } else {
        let local_addr = local_addr.unwrap_or("0.0.0.0:18733");
        Box::pin(forza::horizon4(local_addr).await?)
    })
}
