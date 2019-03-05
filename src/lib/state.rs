use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use serde::Serialize;

pub struct Channel {
    pub index: Mutex<HashMap<String, usize>>,
    pub data: Mutex<Vec<Message>>,
}

pub struct Database {
    pub channels: Arc<Mutex<HashMap<String, Channel>>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Message {
    pub uuid: String,
    pub created: DateTime<Utc>,
    pub value: String,
    // pub data: String,
}

pub fn create_db() -> Arc<Database>{
    let index = HashMap::new();
    let data = Vec::new();
    let foo = Channel {
        index: Mutex::new(index),
        data: Mutex::new(data),
    };
    let mut channels = HashMap::new();
    channels.insert("foo".to_string(), foo);
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
