use std::{io, mem::replace, net::SocketAddr};

use socket2::{Protocol, SockAddr, Type};
use tokio::net::UdpSocket;

pub struct Listener {
    socket: UdpSocket,
}

pub struct Connection {
    socket: UdpSocket,
}

impl Listener {
    pub fn bind<A: Into<SockAddr>>(addr: A) -> Result<Self, io::Error> {
        let addr = addr.into();
        let socket = socket2::Socket::new(addr.domain(), Type::DGRAM, Some(Protocol::UDP))?;

        socket.set_reuse_address(true)?;
        socket.bind(&addr)?;

        Ok(Self {
            socket: UdpSocket::from_std(socket.into())?,
        })
    }

    pub async fn listen(&mut self) -> Result<Connection, io::Error> {
        let peer = self.socket.peek_sender().await?;
        println!("New connection from {peer:?}!");
        // Note: The reason this swap trick is done, is that the notification does not get cleared
        // on the current socket after the peek - it will trigger again immediately.
        // Therefore, we give out our own socket and replace ourselves with a fresh one.
        let Listener { socket } = replace(self, Self::bind(self.socket.local_addr()?)?);

        socket.connect(peer).await?;

        Ok(Connection { socket })
    }
}

impl Connection {
    pub async fn recv(&self, buffer: &mut [u8]) -> Result<usize, io::Error> {
        self.socket.recv(buffer).await
    }

    pub fn peer(&self) -> Result<SocketAddr, io::Error> {
        self.socket.peer_addr()
    }
}
