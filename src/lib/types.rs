use crate::lib::state::Message;

pub enum Request {
    Push {
        channel_id: String,
        value: String,
    },
    Retrieve {
        channel_id: String,
        uid: String,
    },
    Update {
        channel_id: String,
        uid: String,
        value: String,
    },
    Recent {
        channel_id: String,
        count: usize,
        offset: usize,
    },
    Connect {
        channel_id: String,
    },
    Flush {
        channel_id: String,
    },
    Backup {
        channel_id: String,
    },
    Stats {
        channel_id: String,
    },
}

pub enum Response {
    Push { message: Message },
    Recent { messages: Vec<Message> },
    Retrieve { message: Message },
    Stats { message: String },
    Done {},
    Error { message: String },
}

impl Request {
    pub fn parse(input: &str) -> Result<Request, String> {
        // println!("Incoming: {:?}", &input);
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
            Some("UPDATE") => {
                let uid = match parts.next() {
                    Some(uid) => uid,
                    None => return Err(format!("UPDATE needs a uid")),
                };
                let temp = match parts.next() {
                    Some(temp) => temp,
                    None => return Err(format!("UPDATE needs a value")),
                };
                let value = match parts.next() {
                    Some(value) => format!("{} {}", temp, value),
                    None => format!("{}", temp),
                };
                Ok(Request::Update {
                    channel_id: channel_id.to_string(),
                    uid: uid.to_string(),
                    value: value.to_string(),
                })
            }
            Some("CONNECT") => Ok(Request::Connect {
                channel_id: channel_id.to_string(),
            }),
            Some("FLUSH") => Ok(Request::Flush {
                channel_id: channel_id.to_string(),
            }),
            Some("BACKUP") => Ok(Request::Backup {
                channel_id: channel_id.to_string(),
            }),
            Some("STATS") => Ok(Request::Stats {
                channel_id: channel_id.to_string(),
            }),
            Some(cmd) => Err(format!("ER unknown command: {}\n", cmd)),
            None => Err(format!("ER empty input\n")),
        }
    }
}

impl Response {
    pub fn serialize(&self) -> String {
        match *self {
            // Response::Foo { ref message } => {
            //     format!("foo {}", message)
            // },
            Response::Push { ref message } => format!("OK {}\n", message.uid),
            Response::Recent { ref messages } => {
                let serialized = serde_json::to_string(messages).unwrap();
                format!("OK {}\n", serialized)
            }
            Response::Retrieve { ref message } => {
                let serialized = serde_json::to_string(message).unwrap();
                format!("OK {}\n", serialized)
            }
            Response::Stats { ref message } => format!("OK {}\n", message),
            Response::Done {} => format!("OK Done.\n"),
            Response::Error { ref message } => format!("ER {}\n", message),
        }
    }
}
