use failure::Error;
use std::io::{Read, Write};
use std::net;

const BUFSIZE: usize = 4096;

#[derive(Debug)]
pub struct Sender<T: Read> {
    listener: net::TcpListener,
    addr: net::SocketAddr,
    source: T,
}

impl<T> Sender<T> where T: Read {
    pub fn new(source: T) -> Result<Self, Error> {
        let listener = net::TcpListener::bind("0.0.0.0:0")?;
        let addr = listener.local_addr()?;

        Ok(Self {
            listener: listener,
            addr: addr,
            source: source,
        })
    }

    pub fn port(&self) -> u16 {
        self.addr.port()
    }

    pub fn serve(&mut self) -> Result<(), Error> {
        match self.listener.incoming().next() {
            Some(stream) => {
                self.handle_stream(stream?)?;
            },
            None => {
            },
        }

        Ok(())
    }

    fn handle_stream(&mut self, mut stream: net::TcpStream) -> Result<(), Error> {
        let mut buf = [0u8; BUFSIZE];

        loop {
            let size = self.source.read(&mut buf)?;

            if size == 0 {
                break;
            }

            stream.write(&buf[0..size])?;
        }

        stream.flush()?;

        Ok(())
    }
}
