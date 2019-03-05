// #![deny(warnings)]

extern crate tokio;
extern crate uuid;
extern crate chrono;
extern crate serde_derive;
extern crate glob;

mod lib;

use bincode::serialize_into;
use futures::future::lazy;
use tokio::io::{lines, write_all};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::timer::Interval;
use std::time::{Duration, Instant};
use std::sync::{Mutex};
use std::fs::{File, create_dir_all};
use std::io::{BufReader};
use std::env;
use std::net::SocketAddr;
use std::collections::HashMap;
use chrono::{Utc};
use lib::{state, types};
use uuid::Uuid;
use std::cmp;
use blob_uuid;

const MAXIMUM: usize = 10;
const BACKUP_INTERVAL: u64 = 3; // 5 minutes in seconds

fn main() -> Result<(), Box<std::error::Error>> {
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:6363".to_string());
    let addr = addr.parse::<SocketAddr>()?;

    let socket = TcpListener::bind(&addr)?;
    println!("Listening on: {}", addr);

    let db = state::create_db();

    let start_backup = Interval::new(Instant::now(), Duration::from_millis(BACKUP_INTERVAL * 1_000))
        .for_each(|_| {
            let address = "127.0.0.1:6363".parse().expect("Unable to parse address");
            let connection = TcpStream::connect(&address);
            connection.and_then(|socket| {
                let (rx, mut tx) = socket.split();
                tx.poll_write(b"foo RECENT");
                return Ok(());
            });

            Ok(())
        })
        .map_err(|e| panic!("interval errored; err={:?}", e));
    
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
                    Err(e) => return types::Response::Error { message: e },
                };

                match request {
                    types::Request::Push { channel_id, value } => {
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
                        );
                        let uid = blob_uuid::to_blob(&uuid).to_string();
                        let message = state::Message {
                            uid: uid.clone(),
                            created: now,
                            value,
                        };
                        let length = data.len();
                        data.push(message.clone());
                        index.insert(uid.clone(), length);
                        types::Response::Push {
                            message,
                        }
                    }
                    types::Request::Recent { channel_id, count, offset } => {
                        let channels = db.channels.lock().unwrap();
                        let _channel = channels.get(&channel_id);
                        let channel = _channel.unwrap();
                        let data = channel.data.lock().unwrap();
                        let index: usize = {
                            if data.len() < cmp::min(count, MAXIMUM) {
                                0
                            } else {
                                data.len() - cmp::min(count, MAXIMUM)
                            }
                        };
                        let end: usize = {
                            if offset > 0 {
                                if offset + count > data.len() {
                                    return types::Response::Error {
                                        message: "invalid offset".to_string(),
                                    }
                                } else {
                                    data.len() - offset
                                }
                            } else {
                                0
                            }
                        };

                        let messages = match offset {
                            0 => &data[index..],
                            // _ => &data[index..],
                            _ => &data[(index - offset)..end],
                        };
                        types::Response::Recent {
                            messages: messages.to_vec(),
                        }
                    }
                    types::Request::Retrieve { channel_id, uid } => {
                        let channels = db.channels.lock().unwrap();
                        let _channel = channels.get(&channel_id);
                        let channel = _channel.unwrap();
                        let data = channel.data.lock().unwrap();
                        let index = channel.index.lock().unwrap();
                        let message = &index.get(&uid);
                        if let Some(_) = message {
                            let message_index = message.unwrap();
                            let message = &data[*message_index];
                            return types::Response::Retrieve {
                                message: message.clone(),
                            }
                        }
                        types::Response::Error {
                            message: "uid not found".to_string(),
                        }
                    }
                    types::Request::Backup { channel_id } => {
                        let channels = db.channels.lock().unwrap();
                        let _channel = channels.get(&channel_id);
                        let channel = _channel.unwrap();
                        let data = channel.data.lock().unwrap();
                        let index = channel.index.lock().unwrap();
                        let message = &data[0];

                        let path = format!("/mnt/c/Users/Adam/Projects/merkava/.data/{}", channel_id);
                        match create_dir_all(path.clone()) {
                            Err(e) => return types::Response::Error { message: e.to_string() },
                            _ => ()
                        }
                        
                        let data_file = format!("{}/data.mrkv", path);
                        let writer = File::create(data_file).unwrap();
                        serialize_into(writer, &data.clone());
                        
                        let index_file = format!("{}/index.mrkv", path);
                        let writer = File::create(index_file).unwrap();
                        serialize_into(writer, &index.clone());
                        
                        types::Response::Done {}
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

    tokio::run(lazy(|| {
        tokio::spawn(start_backup);
        tokio::spawn(done);
        Ok(())
    }));
    // tokio::run(done);
    Ok(())
}
