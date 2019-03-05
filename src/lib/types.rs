use crate::lib::state::Message;

pub enum Request {
    Push { channel_id: String, value: String },
    Retrieve { channel_id: String, uid: String },
    // Update { channel_id: String, value: String },
    Recent { channel_id: String, count: usize, offset: usize },
    Backup { channel_id: String },
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
    Done {
    },
    Error {
        message: String,
    },
}

impl Request {
    pub fn parse(input: &str) -> Result<Request, String> {
        println!("Incoming: {:?}", &input);
        let mut parts = input.splitn(4, " ");
        let channel_id = match parts.next() {
            Some(channel_id) => channel_id,
            None => return Err(format!("PUSH needs a channel_id")),
        };
        match parts.next() {
            Some("PUSH") => {
                let temp = match parts.next() {
                    Some(temp) => temp,
                    None => return Err(format!("PUSH needs a value")),
                };
                let value = match parts.next() {
                    Some(value) => format!("{} {}", temp, value),
                    None => format!("{}", temp),
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
                let offset = match parts.next() {
                    Some("") => "0",
                    Some(offset) => offset,
                    _ => "0",
                };
                Ok(Request::Recent {
                    channel_id: channel_id.to_string(),
                    count: count.parse::<usize>().unwrap(),
                    offset: offset.parse::<usize>().unwrap(),
                })
            }
            Some("RETRIEVE") => {
                let uid = match parts.next() {
                    Some(uid) => uid,
                    None => return Err(format!("RETRIEVE needs a uid")),
                };
                Ok(Request::Retrieve {
                    channel_id: channel_id.to_string(),
                    uid: uid.to_string(),
                })
            }
            Some("BACKUP") => {
                Ok(Request::Backup {
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
            // Response::Foo { ref message } => {
            //     format!("foo {}", message)
            // },
            Response::Push { ref message } => {
                format!("OK {}", message.uid)
            },
            Response::Recent { ref messages } => {
                let serialized = serde_json::to_string(messages).unwrap();
                format!("OK {}", serialized)
            },
            Response::Retrieve { ref message } => {
                let serialized = serde_json::to_string(message).unwrap();
                format!("OK {}", serialized)
            },
            Response::Done { } => format!("OK Done."),
            Response::Error { ref message } => format!("ER {}", message),
        }
    }
}
