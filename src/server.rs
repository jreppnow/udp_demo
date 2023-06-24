use std::convert::Infallible;
use std::error::Error;
use std::io;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use tokio::task::JoinHandle;
use udp_connections::Listener;

#[tokio::main]
async fn main() -> Result<Infallible, Box<dyn Error>> {
    let server = Listener::bind(SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::new(127, 0, 0, 1),
        55555,
    )))?;
    loop {
        let peer = server.listen().await?;
        #[allow(clippy::let_underscore_future)]
        let _: JoinHandle<io::Result<Infallible>> = tokio::task::spawn(async move {
            let mut buffer = [0u8; 1024];
            loop {
                let length = peer.recv(&mut buffer).await?;
                println!("Received {length} bytes: {:x?}", &buffer[..length]);
            }
        });
    }
}
