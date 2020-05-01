use std::{
    mem::{size_of_val, MaybeUninit},
    slice,
};

use tokio::{
    io,
    net::{ToSocketAddrs, UdpSocket},
};

use futures_core::Stream;

mod datagram;

pub use datagram::*;

pub async fn horizon4<A: ToSocketAddrs>(
    addr: A,
) -> io::Result<impl Stream<Item = io::Result<datagram::Horizon4Datagram>>> {
    let mut socket = UdpSocket::bind(addr).await?;
    Ok(async_stream::try_stream! {
        loop {
            let datagram = unsafe {
                // The following is safe because 1) datagram has no padding, 2) all datagram members
                // can hold any bit pattern, and 3) we verify that we received the exact datagram
                // size - i.e. all unitialized data is over-written
                let mut datagram = MaybeUninit::uninit();

                let mut buf =
                    slice::from_raw_parts_mut(datagram.as_mut_ptr() as *mut _, size_of_val(&datagram));

                let (amt, _src) = socket.recv_from(&mut buf).await?;
                assert_eq!(amt, size_of_val(&datagram));

                datagram.assume_init()
            };

            yield datagram;
        }
    })
}

// struct ForzaHorizon4Stream {
//     socket: udp::RecvHalf,
// }

// impl ForzaHorizon4Stream {
//     async fn bind<A: ToSocketAddrs>(addr: A) -> tokio::io::Result<Self> {
//         Ok(Self {
//             socket: UdpSocket::bind(addr).await?.split().0,
//         })
//     }
// }
