use crate::lib::{state, types};
use bincode::serialize_into;
use blob_uuid;
use chrono::Utc;
use std::cmp;
use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

const MAXIMUM: usize = 10;

fn do_push(db: &Arc<state::Database>, channel_id: String, value: String) -> types::Response {
    let mut channels = db.channels.lock().unwrap();
    if !channels.contains_key(&channel_id) {
        channels.insert(
            channel_id.clone(),
            state::Channel {
                index: Mutex::new(HashMap::new()),
                data: Mutex::new(Vec::new()),
            },
        );
    }
    let _channel = channels.get(&channel_id);
    let channel = _channel.unwrap();
    let mut data = channel.data.lock().unwrap();
    let mut index = channel.index.lock().unwrap();
    let now = Utc::now();
    let uuid = Uuid::new_v5(
        &Uuid::NAMESPACE_DNS,
        format!("{:?}-{:?}", channel_id, now.to_rfc3339()).as_bytes(),
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
    types::Response::Push { message }
}

fn do_recent(
    db: &Arc<state::Database>,
    channel_id: String,
    count: usize,
    offset: usize,
) -> types::Response {
    println!("doing recent");
    let channels = db.channels.lock().unwrap();
    let _channel = channels.get(&channel_id);
    let channel: &state::Channel = match _channel {
        Some(_) => _channel.unwrap(),
        None => {
            return types::Response::Error {
                message: "No messages found".to_string(),
            };
        }
    };
    // let channel = _channel.unwrap();
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
                };
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
    if messages.len() == 0 {
        return types::Response::Error {
            message: "No messages found".to_string(),
        };
    }
    types::Response::Recent {
        messages: messages.to_vec(),
    }
}

fn do_update(
    db: &Arc<state::Database>,
    channel_id: String,
    uid: String,
    value: String,
) -> types::Response {
    let channels = db.channels.lock().unwrap();
    let _channel = channels.get(&channel_id);
    let channel = _channel.unwrap();
    let mut data = channel.data.lock().unwrap();
    let index = channel.index.lock().unwrap();
    let message = &index.get(&uid);
    if let Some(_) = message {
        let message_index = message.unwrap();
        let mut message = &mut data[*message_index];
        message.value = value;
        return types::Response::Done {};
    }
    types::Response::Error {
        message: "uid not found".to_string(),
    }
}

fn do_retrieve(db: &Arc<state::Database>, channel_id: String, uid: String) -> types::Response {
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
        };
    }
    types::Response::Error {
        message: "uid not found".to_string(),
    }
}

fn do_connect(_db: &Arc<state::Database>, _channel_id: String) -> types::Response {
    types::Response::Done {}
}

fn do_flush(db: &Arc<state::Database>, channel_id: String) -> types::Response {
    let mut channels = db.channels.lock().unwrap();
    channels.remove(&channel_id);
    types::Response::Done {}
}

fn do_backup(db: &Arc<state::Database>, channel_id: String) -> types::Response {
    let channels = db.channels.lock().unwrap();
    let _channel = channels.get(&channel_id);
    let channel = _channel.unwrap();
    let data = channel.data.lock().unwrap();
    let index = channel.index.lock().unwrap();

    let path = format!("/home/adam/Projects/merkava/.data/{}", channel_id);
    match create_dir_all(path.clone()) {
        Err(e) => {
            return types::Response::Error {
                message: e.to_string(),
            };
        }
        _ => (),
    }

    let data_file = format!("{}/data.mrkv", path);
    let writer = File::create(data_file).unwrap();
    serialize_into(writer, &data.clone()).expect("Unable to write to file");

    let index_file = format!("{}/index.mrkv", path);
    let writer = File::create(index_file).unwrap();
    serialize_into(writer, &index.clone()).expect("Unable to write to file");

    types::Response::Done {}
}

fn do_stats(db: &Arc<state::Database>, channel_id: String) -> types::Response {
    let channels = db.channels.lock().unwrap();
    let _channel = channels.get(&channel_id);
    let channel = _channel.unwrap();
    let data = channel.data.lock().unwrap();
    types::Response::Stats {
        message: format!("Messages: {:?}", data.len()),
    }
}

pub fn handle_request(db: &Arc<state::Database>, line: String) -> types::Response {
    println!("incoming request{:?}", line);
    let request = match types::Request::parse(&line) {
        Ok(req) => req,
        Err(e) => return types::Response::Error { message: e },
    };

    match request {
        types::Request::Push { channel_id, value } => do_push(&db, channel_id, value),
        types::Request::Recent {
            channel_id,
            count,
            offset,
        } => do_recent(&db, channel_id, count, offset),
        types::Request::Retrieve { channel_id, uid } => do_retrieve(&db, channel_id, uid),
        types::Request::Update {
            channel_id,
            uid,
            value,
        } => do_update(&db, channel_id, uid, value),
        types::Request::Connect { channel_id } => do_connect(&db, channel_id),
        types::Request::Flush { channel_id } => do_flush(&db, channel_id),
        types::Request::Backup { channel_id } => do_backup(&db, channel_id),
        types::Request::Stats { channel_id } => do_stats(&db, channel_id),
    }
}
