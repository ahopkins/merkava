use bincode::deserialize_from;
use chrono::{DateTime, Utc};
use glob::glob;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::result::Result;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Channel {
    pub index: Mutex<HashMap<String, usize>>,
    pub data: Mutex<Vec<Message>>,
}

#[derive(Debug)]
pub struct Database {
    pub channels: Arc<Mutex<HashMap<String, Channel>>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Message {
    pub uid: String,
    pub created: DateTime<Utc>,
    pub value: String,
    // pub data: String,
}

pub fn create_db(data_directory: String) -> Arc<Database> {
    debug!("Creating database");
    let mut channels = HashMap::new();

    create_dir_all(&data_directory).expect("unable to create data directory");

    debug!("{}", format!("Loading records from {}", data_directory));

    let glob_path = format!("{}/*", data_directory);

    for entry in glob(&glob_path).unwrap().filter_map(Result::ok) {
        let path = entry.as_path();
        let split_path = path.components();
        let channel_id = match split_path.last() {
            Some(item) => item.as_os_str().to_os_string().into_string().unwrap(),
            _ => break,
        };

        let data_file = format!("{}/data.mrkv", path.display());
        let reader = File::open(data_file).unwrap();
        let data: Vec<Message> = deserialize_from(reader).unwrap();

        let index_file = format!("{}/index.mrkv", path.display());
        let reader = File::open(index_file).unwrap();
        let index: HashMap<String, usize> = deserialize_from(reader).unwrap();

        let channel = Channel {
            index: Mutex::new(index),
            data: Mutex::new(data),
        };
        channels.insert(channel_id, channel);
    }

    let db = Arc::new(Database {
        channels: Arc::new(Mutex::new(channels)),
    });
    db
}

// pub fn get_channel<'a>(db: &'a Database, channel_id: String) -> &'a Channel {
//     let channels = db.channels.lock().unwrap();
//     let _channel = channels.get(&channel_id);
//     let channel = _channel.unwrap();
//     channel
// }
