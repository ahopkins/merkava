use crate::lib::state::Message;

pub enum Request {
    Push { channel_id: String, value: String },
    // Retrieve { channel_id: String, value: String },
    // Update { channel_id: String, value: String },
    Recent { channel_id: String, },
}

pub enum Response {
    Push {
        message: Message,
    },
    Recent {
        channel_id: String,
    },
    Error {
        msg: String,
    },
}

impl Request {
    pub fn parse(input: &str) -> Result<Request, String> {
        let mut parts = input.splitn(3, " ");
        let channel_id = match parts.next() {
            Some(channel_id) => channel_id,
            None => return Err(format!("PUSH needs a channel_id")),
        };
        println!("Incoming on {:?}", channel_id);
        match parts.next() {
            // Some("RETRIEVE") => {
            //     let key = match parts.next() {
            //         Some(key) => key,
            //         None => return Err(format!("GET must be followed by a key")),
            //     };
            //     if parts.next().is_some() {
            //         return Err(format!("GET's key must not be followed by anything"));
            //     }
            //     Ok(Request::Retrieve {
            //         key: key.to_string(),
            //     })
            // }
            Some("PUSH") => {
                let value = match parts.next() {
                    Some(value) => value,
                    None => return Err(format!("PUSH needs a value")),
                };
                println!("DOING PUSH on {} with {}", channel_id, value);
                Ok(Request::Push {
                    channel_id: channel_id.to_string(),
                    value: value.to_string(),
                })
            }
            Some("RECENT") => {
                println!("DOING RECENT on {}", channel_id);
                Ok(Request::Recent {
                    channel_id: channel_id.to_string(),
                })
            }
            Some(cmd) => Err(format!("unknown command: {}", cmd)),
            None => Err(format!("empty input")),
        }
    }
}

impl Response {
    pub fn serialize(&self) -> String {
        match *self {
            Response::Push { ref message } => {
                format!("Pushed Id {}", message.uuid)
            },
            Response::Recent { ref channel_id } => {
                println!("recent {:?}", channel_id);
                format!("Recent on {}", channel_id)
            },
            // Response::Value { ref key, ref value } => format!("{} = {}", key, value),
            // Response::Set {
            //     ref key,
            //     ref value,
            //     ref previous,
            // } => format!("set {} = `{}`, previous: {:?}", key, value, previous),
            Response::Error { ref msg } => format!("error: {}", msg),
        }
    }
}
