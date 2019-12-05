use crate::crypto;
use cryptostream::read::Decryptor;
use failure::Error;
use openssl::symm::Cipher;
use sha2::{Sha256, Digest};
use std::io::{Read, Write};
use std::net;

const BUFSIZE: usize = 4096;

pub struct Receiver<T: Write> {
    stream: net::TcpStream,
    sink: T,
    secret: Option<String>,
}

impl<T> Receiver<T> where T: Write {
    pub fn new(addr: net::SocketAddr, sink: T) -> Result<Self, Error> {
        let stream = net::TcpStream::connect(addr)?;

        Ok(Self {
            stream: stream,
            sink: sink,
            secret: None,
        })
    }

    pub fn set_secret(&mut self, secret: Option<String>) {
        self.secret = secret;
    }

    pub fn recv(&mut self) -> Result<bool, Error> {
        let shared_secret = crypto::key_exchange(&mut self.stream)?;
        let shared_iv = crypto::key_exchange(&mut self.stream)?;

        let cipher = Cipher::aes_128_cbc();

        let mut key: Vec<u8> = shared_secret.as_bytes().to_vec();
        key.truncate(cipher.block_size());

        let mut iv: Vec<u8> = shared_iv.as_bytes().to_vec();
        iv.truncate(cipher.block_size());

        if let Some(secret) = &self.secret {
            let mut hasher = Sha256::new();
            hasher.input(secret.as_bytes());

            self.stream.write_all(&hasher.result())?;
            self.stream.flush()?;
        }

        let mut buf = [0u8; BUFSIZE];
        let mut decryptor = Decryptor::new(&self.stream, cipher, &key, &iv)?;
        let mut bytes_read = 0;

        loop {
            let size = decryptor.read(&mut buf)?;

            if size == 0 {
                break;
            }

            bytes_read += size;

            self.sink.write(&buf[0..size])?;
        }

        Ok(bytes_read > 0)
    }
}
