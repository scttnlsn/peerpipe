mod crypto;
mod discovery;
mod receiver;
mod sender;

use crate::discovery::Discovery;
use crate::sender::Sender;
use crate::receiver::Receiver;
use clap::{App, AppSettings, Arg, SubCommand};
use failure::Error;
use std::io;
use std::io::prelude::*;
use std::{thread, time};

fn run_sender(secret: Option<String>) -> Result<(), Error> {
    let discovery = Discovery::new(discovery::multicast()?);
    let mut sender = Sender::new(io::stdin())?;
    sender.set_secret(secret);
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

fn run_receiver(secret: Option<String>) -> Result<(), Error> {
    let discovery = Discovery::new(discovery::multicast()?);

    let peer = discovery.discover()?;
    let mut receiver = Receiver::new(peer, io::stdout())?;
    receiver.set_secret(secret);

    if !receiver.recv()? {
        let mut stderr = io::stderr();
        writeln!(&mut stderr, "error: nothing to receive").unwrap();
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    let secret = Arg::with_name("secret")
        .short("s")
        .long("secret")
        .value_name("SECRET")
        .takes_value(true);

    let app = App::new("peerpipe")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("send")
                .about("read from stdin, send data to socket")
                .arg(secret.clone())
        )
        .subcommand(
            SubCommand::with_name("recv")
                .about("read data from socket, write data to stdout")
                .arg(secret.clone())
        );

    let matches = app.get_matches();

    if let Some(cmd) = matches.subcommand_matches("send") {
        let secret = cmd
            .value_of("secret")
            .and_then(|s| Some(s.to_owned()));

        run_sender(secret)?;
    } else if let Some(cmd) = matches.subcommand_matches("recv") {
        let secret = cmd
            .value_of("secret")
            .and_then(|s| Some(s.to_owned()));

        run_receiver(secret)?;
    }

    Ok(())
}
