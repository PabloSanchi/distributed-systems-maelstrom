use challenge::{Message, NewLineWriter, Node};

fn main() -> std::io::Result<()> {
    env_logger::init();

    let stdin = std::io::stdin().lock();
    let stdout = std::io::stdout().lock();
    let mut output = NewLineWriter::new(stdout);
    let mut echo_node = Node::new();

    let input_stream = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();
    for input in input_stream {
        let msg = input.expect("Maelstrom input from STDIN cannot be deserialized");
        echo_node
            .handle(msg, &mut output)
            .expect("Failed to handle message")
    }

    Ok(())
}
