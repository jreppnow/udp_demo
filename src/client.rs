use std::{
    convert::Infallible,
    error::Error,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    time::Duration,
};

use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<Infallible, Box<dyn Error>> {
    let socket = UdpSocket::bind(SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::new(127, 0, 0, 5),
        0,
    )))
    .await?;

    let server = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 55555));

    let data = b"Hello there!";
    let mut length = 0usize;
    loop {
        socket.send_to(&data[..length], &server).await?;

        length = (length + 1) % data.len();

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
