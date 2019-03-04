// #![deny(warnings)]

extern crate tokio;
extern crate uuid;
extern crate chrono;

mod lib;

use tokio::io::{lines, write_all};
use tokio::net::TcpListener;
use tokio::prelude::*;
use std::sync::{Mutex};
use std::io::BufReader;
use std::env;
use std::net::SocketAddr;
use std::collections::HashMap;
use chrono::{Utc};
use lib::{state, types};

use uuid::Uuid;

fn main() -> Result<(), Box<std::error::Error>> {
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:6363".to_string());
    let addr = addr.parse::<SocketAddr>()?;

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

            let responses = lines.map(move |line| {
                let request = match types::Request::parse(&line) {
                    Ok(req) => req,
                    Err(e) => return types::Response::Error { msg: e },
                };

                match request {
                    types::Request::Push { channel_id, value: _ } => {
                        let mut channels = db.channels.lock().unwrap();
                        if !channels.contains_key(&channel_id) {
                            channels.insert(channel_id.clone(), state::Channel {
                                index: Mutex::new(HashMap::new()),
                                data: Mutex::new(Vec::new()),
                            });
                        }
                        let _channel = channels.get(&channel_id);
                        let channel = _channel.unwrap();
                        let mut data = channel.data.lock().unwrap();
                        let mut index = channel.index.lock().unwrap();
                        let now = Utc::now();
                        let uuid = Uuid::new_v5(
                            &Uuid::NAMESPACE_DNS,
                            format!("{:?}-{:?}", channel_id, now.to_rfc3339()).as_bytes()
                        ).to_string();
                        let message = state::Message {
                            uuid: uuid.clone(),
                            created: now,
                            value: String::new(),
                        };
                        let length = data.len();
                        data.push(message.clone());
                        index.insert(uuid.clone(), length);
                        types::Response::Push {
                            message,
                        }
                    }
                    types::Request::Recent { channel_id } => {
                        let channels = db.channels.lock().unwrap();
                        let _channel = channels.get(&channel_id);
                        let channel = _channel.unwrap();
                        let data = channel.data.lock().unwrap();
                        let index: usize = data.len() - 1;
                        let recent = &data[index..];
                        println!("{:?}", recent);
                        types::Response::Recent {
                            channel_id,
                        }
                    }
                }
            });
            let writes = responses.fold(writer, |writer, response| {
                let mut response = response.serialize();
                response.push('\n');
                write_all(writer, response.into_bytes()).map(|(w, _)| w)
            });

            let msg = writes.then(move |_| Ok(()));

            tokio::spawn(msg)
        });

    tokio::run(done);
    Ok(())
}
