mod crypto;
mod discovery;
mod receiver;
mod sender;

use crate::discovery::Discovery;
use crate::sender::Sender;
use crate::receiver::Receiver;
use clap::{App, AppSettings, SubCommand};
use failure::Error;
use std::io;
use std::{thread, time};

fn run_sender() -> Result<(), Error> {
    let discovery = Discovery::new(discovery::multicast()?);
    let mut sender = Sender::new(io::stdin())?;
    let port = sender.port();

    thread::spawn(move || {
        loop {
            discovery.announce(port).unwrap();
            thread::sleep(time::Duration::from_secs(1));
        }
    });

    sender.serve()?;

    Ok(())
}

fn run_receiver() -> Result<(), Error> {
    let discovery = Discovery::new(discovery::multicast()?);

    let peer = discovery.discover()?;
    let mut receiver = Receiver::new(peer, io::stdout())?;
    receiver.recv()?;

    Ok(())
}

fn main() -> Result<(), Error> {
    let app = App::new("peerpipe")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("send")
                .about("read from stdin, send data to socket")
        )
        .subcommand(
            SubCommand::with_name("recv")
                .about("read data from socket, write data to stdout")
        );

    let matches = app.get_matches();

    if matches.subcommand_matches("send").is_some() {
        run_sender()?;
    } else if matches.subcommand_matches("recv").is_some() {
        run_receiver()?;
    } else {

    }

    Ok(())
}
