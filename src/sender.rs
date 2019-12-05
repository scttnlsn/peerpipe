use crate::crypto;
use cryptostream;
use failure::Error;
use openssl::symm::Cipher;
use sha2::{Sha256, Digest};
use std::io::{Read, Write};
use std::net;

const BUFSIZE: usize = 4096;

pub struct Sender<T: Read> {
    listener: net::TcpListener,
    addr: net::SocketAddr,
    source: T,
    secret: Option<String>,
}

impl<T> Sender<T> where T: Read {
    pub fn new(source: T) -> Result<Self, Error> {
        let listener = net::TcpListener::bind("0.0.0.0:0")?;
        let addr = listener.local_addr()?;

        Ok(Self {
            listener: listener,
            addr: addr,
            source: source,
            secret: None,
        })
    }

    pub fn port(&self) -> u16 {
        self.addr.port()
    }

    pub fn set_secret(&mut self, secret: Option<String>) {
        self.secret = secret;
    }

    pub fn serve(&mut self) -> Result<(), Error> {
        loop {
            match self.listener.incoming().next() {
                Some(stream) => {
                    let accepted = self.handle_stream(stream?)?;
                    if accepted {
                        break;
                    }
                },
                None => {
                    break;
                },
            }
        }

        Ok(())
    }

    fn handle_stream(&mut self, mut stream: net::TcpStream) -> Result<bool, Error> {
        let shared_secret = crypto::key_exchange(&mut stream)?;
        let shared_iv = crypto::key_exchange(&mut stream)?;

        let cipher = Cipher::aes_128_cbc();

        let mut key: Vec<u8> = shared_secret.as_bytes().to_vec();
        key.truncate(cipher.block_size());

        let mut iv: Vec<u8> = shared_iv.as_bytes().to_vec();
        iv.truncate(cipher.block_size());

        if let Some(secret) = &self.secret {
            let mut hasher = Sha256::new();
            hasher.input(secret.as_bytes());

            // read hashed secret from receiver
            let mut buf = [0u8; 32];
            let size = stream.read(&mut buf[..])?;
            if size != 32 || hasher.result()[..] != buf[..] {
                // receiver sent wrong secret
                return Ok(false);
            }
        }

        let mut buf = [0u8; BUFSIZE];
        let mut encryptor = cryptostream::write::Encryptor::new(&stream, cipher, &key, &iv)?;

        loop {
            let size = self.source.read(&mut buf)?;

            if size == 0 {
                break;
            }

            encryptor.write(&buf[0..size])?;
        }

        encryptor.flush()?;

        Ok(true)
    }
}
