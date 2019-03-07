// #![deny(warnings)]

extern crate chrono;
extern crate glob;
extern crate serde_derive;
extern crate tokio;
extern crate uuid;

mod lib;

use futures::future::lazy;
use lib::{conf, operations, state};
use std::env;
use std::io::BufReader;
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use tokio::io::{lines, write_all};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::timer::Interval;

fn main() -> Result<(), Box<std::error::Error>> {
    let mrkvconf = env::args()
        .nth(1)
        .unwrap_or("./example/mrkvconf.toml".to_string());
    let conf = conf::get_conf(mrkvconf);
    let addr = conf.get::<SocketAddr>("network.address").unwrap();
    let backup_interval = conf.get::<u64>("backup.interval").unwrap();

    let start_backup = Interval::new(
        Instant::now(),
        Duration::from_millis(&backup_interval * 1_000 + 1),
    )
    .for_each(move |_| {
        println!("running");
        let connection = TcpStream::connect(&addr);
        let _do_process = connection.and_then(|socket| {
            println!("process");
            let (_, mut tx) = socket.split();
            tx.poll_write(b"foo RECENT\n")
                .expect("Unable to send to TCP connection");
            return Ok(());
        });

        Ok(())
    })
    .map_err(|e| panic!("interval errored; err={:?}", e));

    let socket = TcpListener::bind(&addr)?;
    println!("Listening on: {}", addr);
    let db = state::create_db();
    let done = socket
        .incoming()
        .map_err(|e| println!("failed to accept socket; error = {:?}", e))
        .for_each(move |socket| {
            println!("accepted socket; addr={:?}", socket.peer_addr().unwrap());

            let (reader, writer) = socket.split();
            let lines = lines(BufReader::new(reader));

            let db = db.clone();

            let responses = lines.map(move |line| operations::handle_request(&db, line));
            let writes = responses.fold(writer, |writer, response| {
                let mut response = response.serialize();
                response.push('\n');
                write_all(writer, response.into_bytes()).map(|(w, _)| w)
            });

            let msg = writes.then(move |_| Ok(()));

            tokio::spawn(msg)
        });

    tokio::run(lazy(move || {
        if backup_interval > 0 {
            println!("starting backup");
            tokio::spawn(start_backup);
        }
        tokio::spawn(done);
        Ok(())
    }));
    Ok(())
}
