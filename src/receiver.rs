use crate::crypto;
use cryptostream::read::Decryptor;
use failure::Error;
use openssl::symm::Cipher;
use std::io::{Read, Write};
use std::net;

const BUFSIZE: usize = 4096;

pub struct Receiver<T: Write> {
    stream: net::TcpStream,
    sink: T,
}

impl<T> Receiver<T> where T: Write {
    pub fn new(addr: net::SocketAddr, sink: T) -> Result<Self, Error> {
        let stream = net::TcpStream::connect(addr)?;

        Ok(Self {
            stream: stream,
            sink: sink,
        })
    }

    pub fn recv(&mut self) -> Result<(), Error> {
        let shared_secret = crypto::key_exchange(&mut self.stream)?;
        let shared_iv = crypto::key_exchange(&mut self.stream)?;

        let cipher = Cipher::aes_128_cbc();

        let mut key: Vec<u8> = shared_secret.as_bytes().to_vec();
        key.truncate(cipher.block_size());

        let mut iv: Vec<u8> = shared_iv.as_bytes().to_vec();
        iv.truncate(cipher.block_size());

        let mut decryptor = Decryptor::new(&self.stream, cipher, &key, &iv)?;

        let mut buf = [0u8; BUFSIZE];

        loop {
            let size = decryptor.read(&mut buf)?;

            if size == 0 {
                break;
            }

            self.sink.write(&buf[0..size])?;
        }

        Ok(())
    }
}
