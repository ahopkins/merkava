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
    if value.chars().count() == 0 {
        return types::Response::Error {
            message: "Cannot push empty message".to_string(),
        };
    }

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
    debug!("doing recent");
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
    let channel: &state::Channel = match _channel {
        Some(_) => _channel.unwrap(),
        None => {
            return types::Response::Error {
                message: "No messages found".to_string(),
            };
        }
    };
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

fn do_backup(db: &Arc<state::Database>, conf: &config::Config, channel_id: String) -> types::Response {
    let channels = db.channels.lock().unwrap();
    let _channel = channels.get(&channel_id);
    let channel = _channel.unwrap();
    let data = channel.data.lock().unwrap();
    let index = channel.index.lock().unwrap();
    let backup_path = conf.get::<String>("persistence.path").unwrap();
    let path = format!("{}/{}", backup_path, channel_id);
    info!("{}", format!("Backing up to {}", path));

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
    if _channel.is_none() {
        return types::Response::Stats {
            message: "Messages: -".to_string(),
        }
    }
    let channel = _channel.unwrap();
    let data = channel.data.lock().unwrap();
    types::Response::Stats {
        message: format!("Messages: {:?}", data.len()),
    }
}

pub fn handle_request(db: &Arc<state::Database>, conf: &config::Config, line: String) -> types::Response {
    debug!("incoming request: {:?}", line);
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
        types::Request::Backup { channel_id } => do_backup(&db, &conf, channel_id),
        types::Request::Stats { channel_id } => do_stats(&db, channel_id),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::state;
    use serde_json::Value;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    fn make_db() -> std::sync::Arc<state::Database> {
        let channels = HashMap::new();
        Arc::new(state::Database {
            channels: Arc::new(Mutex::new(channels)),
        })
    }

    fn make_pushes(db: &std::sync::Arc<state::Database>, channel_id: String, number: u16) {
        for x in 0..number {
            do_push(&db, channel_id.to_string(), format!("{:?}", x));
        }
    }

    ////////////////
    // PUSH TESTS //
    ////////////////

    #[test]
    fn do_push_receive_ok_response() {
        let db = make_db();
        let response = do_push(&db, String::from("foobar"), String::from("something"));
        let message = response.serialize();
        assert_eq!(&message[..2], "OK");
    }

    #[test]
    fn do_push_receive_er_response() {
        let db = make_db();
        let response = do_push(&db, String::from("foobar"), String::from(""));
        let message = response.serialize();
        assert_eq!(&message[..2], "ER");
    }

    #[test]
    fn do_push_message_stored() {
        let db = make_db();
        let text = String::from("something");
        let response = do_push(&db, String::from("foobar"), text.clone());
        let mut message = response.serialize();
        let uid = &mut message[3..].to_string();
        uid.pop();

        let channels = db.channels.lock().unwrap();
        let index = &channels
            .get("foobar")
            .unwrap()
            .index
            .lock()
            .unwrap()
            .clone();
        let data = &channels.get("foobar").unwrap().data.lock().unwrap().clone();
        let message_index = index.get(uid).unwrap();
        let message = &data[*message_index];

        assert!(index.contains_key(uid), "uid={}. index={:?}", uid, index);
        assert_eq!(message.value, text);
    }

    #[test]
    fn do_push_messages_stored_in_order() {
        let db = make_db();

        make_pushes(&db, String::from("foobar"), 200);
        let channels = db.channels.lock().unwrap();
        let data = &channels.get("foobar").unwrap().data.lock().unwrap().clone();

        for (i, message) in data.iter().enumerate() {
            let next_i = i + 1;
            if next_i < data.len() {
                let next = &data[next_i];
                assert!(message.created < next.created);
            }
        }
    }

    //////////////////
    // RECENT TESTS //
    //////////////////

    #[test]
    fn do_recent_receive_ok_response() {
        let db = make_db();
        do_push(&db, String::from("foobar"), String::from("hello"));
        let response = do_recent(&db, String::from("foobar"), 1, 0);
        let message = response.serialize();
        assert_eq!(&message[..2], "OK");
    }

    #[test]
    fn do_recent_proper_length() {
        let db = make_db();

        let response = do_recent(&db, String::from("foobar"), 10, 0);
        let mut message = response.serialize();
        let json_string = &mut message[3..].to_string();
        json_string.pop();
        let messages: Value = serde_json::json!(json_string);
        assert_eq!(messages, String::from("No messages found"));

        make_pushes(&db, String::from("foobar"), 1);
        let response = do_recent(&db, String::from("foobar"), 10, 0);
        let mut message = response.serialize();
        let json_string = &mut message[3..].to_string();
        json_string.pop();
        let messages: Value = serde_json::from_str(json_string).unwrap();
        assert_eq!(messages.as_array().unwrap().len(), 1);

        make_pushes(&db, String::from("foobar"), 1);
        let response = do_recent(&db, String::from("foobar"), 10, 0);
        let mut message = response.serialize();
        let json_string = &mut message[3..].to_string();
        json_string.pop();
        let messages: Value = serde_json::from_str(json_string).unwrap();
        assert_eq!(messages.as_array().unwrap().len(), 2);

        make_pushes(&db, String::from("somethingelse"), 9);
        let response = do_recent(&db, String::from("somethingelse"), 10, 0);
        let mut message = response.serialize();
        let json_string = &mut message[3..].to_string();
        json_string.pop();
        let messages: Value = serde_json::from_str(json_string).unwrap();
        assert_eq!(messages.as_array().unwrap().len(), 9);
        let response = do_recent(&db, String::from("foobar"), 10, 0);
        let mut message = response.serialize();
        let json_string = &mut message[3..].to_string();
        json_string.pop();
        let messages: Value = serde_json::from_str(json_string).unwrap();
        assert_eq!(messages.as_array().unwrap().len(), 2);

        make_pushes(&db, String::from("foobar"), 9);
        let response = do_recent(&db, String::from("foobar"), 10, 0);
        let mut message = response.serialize();
        let json_string = &mut message[3..].to_string();
        json_string.pop();
        let messages: Value = serde_json::from_str(json_string).unwrap();
        assert_eq!(messages.as_array().unwrap().len(), 10);
    }

    #[test]
    fn do_recent_receive_er_response() {
        let db = make_db();

        let response = do_recent(&db, String::from("foobar"), 1, 0);
        let message = response.serialize();
        assert_eq!(&message[..2], "ER");

        make_pushes(&db, String::from("foobar"), 2);

        let response = do_recent(&db, String::from("foobar"), 2, 1);
        let message = response.serialize();
        assert_eq!(&message[..2], "ER");
    }

    ////////////////////
    // RETRIEVE TESTS //
    ////////////////////

    #[test]
    fn do_retrieve_receive_ok_response() {
        let db = make_db();
        let response = do_push(&db, String::from("foobar"), String::from("something"));
        let mut message = response.serialize();
        let uid = &mut message[3..].to_string();
        uid.pop();

        let response = do_retrieve(&db, String::from("foobar"), uid.to_string());
        let message = response.serialize();
        assert_eq!(&message[..2], "OK");
    }

    #[test]
    fn do_retrieve_receive_er_response() {
        let db = make_db();
        let response = do_push(&db, String::from("foobar"), String::from("something"));
        let mut message = response.serialize();
        let uid = &mut message[3..].to_string();
        uid.pop();

        let response = do_retrieve(&db, String::from("oops"), uid.to_string());
        let mut message = response.serialize();
        assert_eq!(&message[..2], "ER");
        let json_string = &mut message[3..].to_string();
        json_string.pop();
        let messages: Value = serde_json::json!(json_string);
        assert_eq!(messages, String::from("No messages found"));
    }

    #[test]
    fn do_retrieve_found_correct_data() {
        let db = make_db();
        let text = String::from("something");
        let response = do_push(&db, String::from("foobar"), text.clone());
        let mut message = response.serialize();
        let uid = &mut message[3..].to_string();
        uid.pop();

        let response = do_retrieve(&db, String::from("foobar"), uid.to_string());
        let mut message = response.serialize();
        let json_string = &mut message[3..].to_string();
        let message_value: Value = serde_json::from_str(json_string).unwrap();
        let message = message_value.as_object().unwrap();
        assert_eq!(message["value"], text);
    }
}
