use std::collections::{HashMap, HashSet, VecDeque};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct Message {
    id: usize,
    src: String,
    dest: String,
    body: Body,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
struct Body {
    msg_id: usize,
    in_reply_to: Option<usize>,
    #[serde(flatten)]
    payload: MessageType,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum MessageType {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    #[default]
    InitOk,
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
    Generate,
    GenerateOk {
        id: usize,
    },
    Broadcast {
        message: usize,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: HashSet<usize>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
}

#[derive(Default)]
pub struct Node {
    id: String,
    nodes: Vec<String>,
    msg_id: usize,
    unique_id: usize,
    messages: HashSet<usize>,
    neighbors: Vec<String>,
}

impl Node {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn handle_message(&mut self, msg: Message) -> VecDeque<Message> {
        let mut responses = VecDeque::new();
        let mut response = Message::default();
        match msg.body.payload {
            MessageType::Init { node_id, node_ids } => {
                self.id = node_id;
                self.nodes = node_ids;
                self.unique_id = self.id[1..].parse().unwrap();
                response.body.payload = MessageType::InitOk;
            }
            MessageType::Echo { echo } => {
                response.body.payload = MessageType::EchoOk { echo };
            }
            MessageType::Generate => {
                response.body.payload = MessageType::GenerateOk { id: self.unique_id };
                self.unique_id += self.nodes.len();
            }
            MessageType::Broadcast { message } => {
                response.body.payload = MessageType::BroadcastOk;
                if !self.messages.contains(&message) {
                    self.messages.insert(message);
                    let mut broadcast = Message::default();
                    broadcast.src = self.id.clone();
                    broadcast.body.payload = MessageType::Broadcast { message };
                    for node in &self.neighbors {
                        if node == &msg.src || node == &self.id {
                            continue;
                        }
                        broadcast.dest = node.clone();
                        broadcast.body.msg_id = self.msg_id;
                        self.msg_id += 1;
                        responses.push_back(broadcast.clone());
                    }
                }
            }
            MessageType::Read => {
                response.body.payload = MessageType::ReadOk {
                    messages: self.messages.clone(),
                };
            }
            MessageType::Topology { topology } => {
                self.neighbors = topology.get(&self.id).cloned().unwrap_or_default();
                response.body.payload = MessageType::TopologyOk;
            }
            _ => return responses,
        }
        response.src = self.id.clone();
        response.dest = msg.src;
        response.body.in_reply_to = Some(msg.body.msg_id);
        response.body.msg_id = self.msg_id;
        self.msg_id += 1;
        responses.push_front(response);
        responses
    }
}
