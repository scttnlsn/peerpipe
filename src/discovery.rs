use byteorder::{ByteOrder, LittleEndian};
use failure::Error;
use std::net;

const MULTICAST_IP: &str = "239.255.250.250";
const MULTICAST_PORT: u16 = 8888;

#[derive(Debug)]
pub struct Multicast {
    addr: net::Ipv4Addr,
    port: u16,
}

impl Multicast {
    pub fn new(ip: &str, port: u16) -> Result<Self, Error> {
        let addr = ip.parse::<net::Ipv4Addr>()?;

        Ok(Self {
            addr: addr,
            port: port,
        })
    }

    pub fn send(&self, buf: &[u8]) -> Result<(), Error> {
        let socket = net::UdpSocket::bind("0.0.0.0:0")?;
        socket.send_to(buf, (self.addr, self.port))?;
        Ok(())
    }

    pub fn recv(&self, mut buf: &mut [u8]) -> Result<(usize, net::SocketAddr), Error> {
        let any = net::Ipv4Addr::new(0, 0, 0, 0);
        let socket = net::UdpSocket::bind((any, self.port))?;
        socket.join_multicast_v4(&self.addr, &any)?;

        let res = socket.recv_from(&mut buf)?;
        Ok(res)
    }
}

pub fn multicast() -> Result<Multicast, Error> {
    Multicast::new(MULTICAST_IP, MULTICAST_PORT)
}

#[derive(Debug)]
pub struct Discovery {
    multicast: Multicast,
}

impl Discovery {
    pub fn new(multicast: Multicast) -> Self {
        Discovery {
            multicast: multicast,
        }
    }

    pub fn announce(&self, port: u16) -> Result<(), Error> {
        let mut buf = [0u8; 2];
        LittleEndian::write_u16(&mut buf, port);
        self.multicast.send(&buf)?;

        Ok(())
    }

    pub fn discover(&self) -> Result<net::SocketAddr, Error> {
        let mut buf = [0u8; 1024];
        let (_, src) = self.multicast.recv(&mut buf)?;

        let port = LittleEndian::read_u16(&buf);

        Ok(net::SocketAddr::new(src.ip(), port))
    }
}
