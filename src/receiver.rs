use failure::Error;
use std::io::{Read, Write};
use std::net;

const BUFSIZE: usize = 4096;

#[derive(Debug)]
pub struct Receiver<T: Write> {
    addr: net::SocketAddr,
    stream: net::TcpStream,
    sink: T,
}

impl<T> Receiver<T> where T: Write {
    pub fn new(addr: net::SocketAddr, sink: T) -> Result<Self, Error> {
        let stream = net::TcpStream::connect(addr)?;

        Ok(Self {
            addr: addr,
            stream: stream,
            sink: sink,
        })
    }

    pub fn recv(&mut self) -> Result<(), Error> {
        let mut buf = [0u8; BUFSIZE];

        loop {
            let size = self.stream.read(&mut buf)?;

            if size == 0 {
                break;
            }

            self.sink.write(&buf[0..size])?;
        }

        Ok(())
    }
}
