use maelstrom_echo::Node;


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
