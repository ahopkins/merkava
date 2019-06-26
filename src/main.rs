// #![deny(warnings)]

#[macro_use]
extern crate log;

extern crate chrono;
extern crate fern;
extern crate glob;
extern crate serde_derive;
extern crate tokio;
extern crate uuid;

mod lib;

use futures::future::lazy;
use lib::{conf, logging, operations, state};
// use log::Level;
use std::env;
use std::io::BufReader;
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use tokio::io::{lines, write_all};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::timer::Interval;

fn main() -> Result<(), Box<std::error::Error>> {
    // info!(target: "overly-verbose-target", "completed operation.");

    let mrkvconf = env::args()
        .nth(1)
        .unwrap_or("./src/mrkvconf.toml".to_string());
    let conf = conf::get_conf(mrkvconf);
    let addr_raw = conf.get::<String>("network.address").unwrap();
    let addr: SocketAddr = addr_raw.parse().unwrap();
    let backup_interval = conf.get::<u64>("persistence.interval").unwrap();
    let backup_path = conf.get::<String>("persistence.path").unwrap();

    logging::setup_logging(conf.get::<u64>("logging.verbosity").unwrap())
        .expect("failed to initialize logging.");
    info!("MerkavaDB starting up");

    let start_backup = Interval::new(
        Instant::now(),
        Duration::from_millis(&backup_interval * 1_000 + 1),
    )
    .for_each(move |_| {
        debug!("running");
        let connection = TcpStream::connect(&addr);
        let _do_process = connection.and_then(|socket| {
            debug!("process");
            let (_, mut tx) = socket.split();
            tx.poll_write(b"foo PUSH something\n")
                .expect("Unable to send to TCP connection");
            return Ok(());
        });

        Ok(())
    })
    .map_err(|e| panic!("interval errored; err={:?}", e));

    let socket = TcpListener::bind(&addr)?;
    info!("Listening on: {}", addr);
    let db = state::create_db(backup_path);
    let done = socket
        .incoming()
        .map_err(|e| error!("failed to accept socket; error = {:?}", e))
        .for_each(move |socket| {
            debug!("accepted socket; addr={:?}", socket.peer_addr().unwrap());

            let (reader, writer) = socket.split();
            let lines = lines(BufReader::new(reader));

            let db = db.clone();
            let conf = conf.clone();

            let responses = lines.map(move |line| operations::handle_request(&db, &conf, line));
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
            info!("starting backup");
            tokio::spawn(start_backup);
        }

        info!("Ready to receive");
        tokio::spawn(done);
        Ok(())
    }));
    Ok(())
}
