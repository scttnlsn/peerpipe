use failure::Error;
use rand_os::OsRng;
use std::io::{Read, Write};
use x25519_dalek::{EphemeralSecret, PublicKey, SharedSecret};

pub fn key_exchange<T: Read + Write>(mut stream: T) -> Result<SharedSecret, Error> {
    let mut rng = OsRng::new().unwrap();
    let private_secret = EphemeralSecret::new(&mut rng);

    // send public key to peer
    let public_key = PublicKey::from(&private_secret);
    stream.write(&public_key.as_bytes()[0..32])?;

    // read public key from peer
    let mut peer_public_key = [0u8; 32];
    let n = stream.read(&mut peer_public_key)?;
    if n != 32 {
        // FIXME: return error
    }

    let peer_public_key = PublicKey::from(peer_public_key);
    let shared_secret = private_secret.diffie_hellman(&peer_public_key);

    Ok(shared_secret)
}
