use socket2::{Protocol, SockAddr, Type};
use std::io;
use tokio::net::UdpSocket;

pub struct Listener {
    socket: UdpSocket,
}

pub struct Connection {
    socket: UdpSocket,
}

impl Listener {
    pub fn new<A: Into<SockAddr>>(addr: A) -> Result<Self, io::Error> {
        let addr = addr.into();
        let socket = socket2::Socket::new(addr.domain(), Type::DGRAM, Some(Protocol::UDP))?;

        socket.set_reuse_address(true)?;
        socket.bind(&addr)?;

        Ok(Self {
            socket: UdpSocket::from_std(socket.into())?,
        })
    }

    pub async fn listen(&self) -> Result<Connection, io::Error> {
        let mut buffer = [0u8; 1024];
        // ideally, you would want to use peek.. but alas, not supported atm
        let (_, peer) = self.socket.recv_from(&mut buffer).await?;
        println!("New connection from {peer:?}!");

        let addr: SockAddr = self.socket.local_addr()?.into();

        let socket = socket2::Socket::new(addr.domain(), Type::DGRAM, Some(Protocol::UDP))?;

        socket.set_reuse_address(true)?;
        socket.bind(&addr)?;
        socket.connect(&peer.into())?;

        Ok(Connection {
            socket: UdpSocket::from_std(socket.into())?,
        })
    }
}

impl Connection {
    pub async fn recv(&self, buffer: &mut [u8]) -> Result<usize, io::Error> {
        self.socket.recv(buffer).await
    }
}
