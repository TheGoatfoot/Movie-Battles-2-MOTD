mod rcon;

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::thread::sleep;
use std::time::Duration;

use clap::{Arg, App, crate_version};

use rcon::Rcon;

fn main() {
    let matches = App::new("MOTD")
    .version(crate_version!())
    .author("Goatfoot")
    .about("Message of the day plugin for Movie Battles 2")
    .arg(Arg::with_name("motd")
        .short("m")
        .long("motd")
        .value_name("MOTD")
        .help("Sets the file that contains the message of the day")
        .default_value("./motd.txt"))
    .arg(Arg::with_name("rcon")
        .short("r")
        .long("rcon")
        .value_name("RCON")
        .help("Sets the rcon password")
        .default_value("password"))
    .arg(Arg::with_name("hostip")
        .short("i")
        .long("host-ip")
        .value_name("HOST IP")
        .help("Sets the host IP")
        .default_value("127.0.0.1"))
    .arg(Arg::with_name("hostport")
        .short("p")
        .long("host-port")
        .value_name("HOST PORT")
        .help("Sets the host port")
        .default_value("29070"))
    .arg(Arg::with_name("clientport")
        .short("c")
        .long("client-port")
        .value_name("CLIENT PORT")
        .help("Sets the client port")
        .default_value("3400"))
    .arg(Arg::with_name("interval")
        .short("t")
        .long("interval")
        .value_name("INTERVAL")
        .help("Sets the message interval")
        .default_value("30"))
    .get_matches();

    let motd = matches.value_of("motd").unwrap_or_default();
    let rcon = matches.value_of("rcon").unwrap_or_default();
    let hostip = matches.value_of("hostip").unwrap_or_default();
    let hostport: u16 = matches.value_of("hostport").unwrap_or_default().parse().expect("cannot read host port");
    let clientport: u16 = matches.value_of("clientport").unwrap_or_default().parse().expect("cannot read client port");
    let interval: u64 = matches.value_of("interval").unwrap_or_default().parse().expect("cannot read client port");

    let file = File::open(motd).expect("cannot open MOTD file");
    let file = BufReader::new(file);
    let messages: Vec<String> = file.lines()
        .map(|e| e.expect("cannot read MOTD message"))
        .filter(|e| !e.is_empty() && e.chars().next().unwrap() != '"').collect();
    let messages_count = messages.len();
    if messages_count == 0 {
        println!("no message found");
        return
    }
    let mut current_message: usize = 0;

    let mut rcon = Rcon::new(rcon.to_owned(), hostip.to_owned(), hostport, clientport);
    let mut error_count: u8 = 0;

    loop {
        println!("sending message...");
        let message = match messages.get(current_message) {
            Some(value)=>value,
            None=>break
        };
        if !rcon.svsay(message.clone()) {
            println!("error sending message.");
            error_count += 1;
        } else {
            println!("success sending message.");
            error_count = 0;
        }
        if error_count > 3 {
            println!("too many error");
            break
        }
        current_message += 1; 
        if current_message >= messages_count {
            current_message = 0;
        }
        sleep(Duration::from_secs(interval));
    }
}
