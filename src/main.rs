use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
struct Message {
    id: usize,
    src: String,
    dest: String,
    body: Body,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct Body {
    msg_id: usize,
    in_reply_to: Option<usize>,
    #[serde(flatten)]
    payload: MessageType,
}

#[derive(Debug, Deserialize, Serialize, Default)]
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
}

#[derive(Default)]
struct Node {
    id: String,
    nodes: Vec<String>,
    msg_id: usize,
    unique_id: usize,
}

impl Node {
    fn new() -> Self {
        Default::default()
    }

    fn handle_message(&mut self, msg: Message) -> Message {
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
            _ => unreachable!(),
        }
        response.src = self.id.clone();
        response.dest = msg.src;
        response.body.in_reply_to = Some(msg.body.msg_id);
        response.body.msg_id = self.msg_id;
        self.msg_id += 1;
        response
    }
}

fn main() -> anyhow::Result<()> {
    let mut node = Node::new();
    let stdin = &std::io::stdin();
    let messages = serde_json::Deserializer::from_reader(stdin.lock()).into_iter();
    for message in messages {
        let response = node.handle_message(message?);
        let response = serde_json::to_string(&response).unwrap();
        println!("{response}");
    }
    Ok(())
}
