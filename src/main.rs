// #![deny(warnings)]

extern crate chrono;
extern crate glob;
extern crate serde_derive;
extern crate tokio;
extern crate uuid;

mod lib;

use futures::future::lazy;
use lib::{operations, state};
use std::env;
use std::io::BufReader;
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use tokio::io::{lines, write_all};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::timer::Interval;

const BACKUP_INTERVAL: u64 = 0; // seconds

fn main() -> Result<(), Box<std::error::Error>> {
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:6363".to_string());
    let addr = addr.parse::<SocketAddr>()?;

    let socket = TcpListener::bind(&addr)?;
    println!("Listening on: {}", addr);

    let start_backup = Interval::new(
        Instant::now(),
        Duration::from_millis(BACKUP_INTERVAL * 1_000 + 1),
    )
    .for_each(|_| {
        let address = "127.0.0.1:6363".parse().expect("Unable to parse address");
        let connection = TcpStream::connect(&address);
        let _do_process = connection.and_then(|socket| {
            let (_, mut tx) = socket.split();
            tx.poll_write(b"foo RECENT")
                .expect("Unable to send to TCP connection");
            return Ok(());
        });

        Ok(())
    })
    .map_err(|e| panic!("interval errored; err={:?}", e));

    let db = state::create_db();
    let done = socket
        .incoming()
        .map_err(|e| println!("failed to accept socket; error = {:?}", e))
        .for_each(move |socket| {
            // println!("accepted socket; addr={:?}", socket.peer_addr().unwrap());

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

    tokio::run(lazy(|| {
        if BACKUP_INTERVAL > 0 {
            tokio::spawn(start_backup);
        }
        tokio::spawn(done);
        Ok(())
    }));
    Ok(())
}
