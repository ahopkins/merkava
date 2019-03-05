use crate::lib::state::Message;

pub enum Request {
    Push { channel_id: String, value: String },
    Retrieve { channel_id: String, uuid: String },
    // Update { channel_id: String, value: String },
    Recent { channel_id: String, count: usize},
}

pub enum Response {
    Push {
        message: Message,
    },
    Recent {
        messages: Vec<Message>,
    },
    Retrieve {
        message: Message,
    },
    Foo {
        message: String,
    },
    Error {
        msg: String,
    },
}

impl Request {
    pub fn parse(input: &str) -> Result<Request, String> {
        println!("Incoming: {:?}", &input);
        let mut parts = input.splitn(3, " ");
        let channel_id = match parts.next() {
            Some(channel_id) => channel_id,
            None => return Err(format!("PUSH needs a channel_id")),
        };
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
                Ok(Request::Push {
                    channel_id: channel_id.to_string(),
                    value: value.to_string(),
                })
            }
            Some("RECENT") => {
                let count = match parts.next() {
                    Some("") => "5",
                    Some(count) => count,
                    _ => "5",
                };
                println!("count {:?}", count);
                Ok(Request::Recent {
                    channel_id: channel_id.to_string(),
                    count: count.parse::<usize>().unwrap()
                })
            }
            Some("RETRIEVE") => {
                let uuid = match parts.next() {
                    Some(uuid) => uuid,
                    None => return Err(format!("RETRIEVE needs a uuid")),
                };
                Ok(Request::Retrieve {
                    channel_id: channel_id.to_string(),
                    uuid: uuid.to_string(),
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
            Response::Foo { ref message } => {
                format!("foo {}", message)
            },
            Response::Push { ref message } => {
                format!("OK {}", message.uuid)
            },
            Response::Recent { ref messages } => {
                let serialized = serde_json::to_string(messages).unwrap();
                format!("OK {}", serialized)
            },
            Response::Retrieve { ref message } => {
                let serialized = serde_json::to_string(message).unwrap();
                format!("OK {}", serialized)
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
