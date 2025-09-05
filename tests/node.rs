use challenge::{
    Body, Message, Node, Payload,
    message::{EchoPayload, InitPayload, MessageBuilder},
    node::MessageGenerator,
};
use std::string::String;

#[test]
fn init_use_case() {
    // given
    let mut node = Node::new();
    let init_msg = default_init_msg();

    // when
    let reply = node
        .generate_msg(&init_msg)
        .expect("init_ok reply is expected");

    // then
    let Message { src, dest, body } = reply;
    let Body { payload, .. } = body;

    assert_eq!(src, "n3");
    assert_eq!(dest, "master");
    assert!(matches!(payload, Payload::InitOk(..)));
}

#[test]
fn echo_use_case() {
    // given
    let mut node = Node::new();
    let init_msg = default_init_msg();

    node.generate_msg(&init_msg)
        .expect("init_ok reply is expected");

    let echo_content = String::from("this is the echo msg");
    let echo_payload = EchoPayload {
        echo: echo_content.clone(),
    };
    let echo_msg = MessageBuilder::new()
        .with_src("master".into())
        .with_dest("n3".into())
        .with_msg_id(1)
        .with_payload(Payload::Echo(echo_payload))
        .build();

    // when
    let reply = node
        .generate_msg(&echo_msg)
        .expect("echo_ok reply is expected");

    // then
    let Message { src, dest, body } = reply;
    let Body { payload, .. } = body;

    assert_eq!(src, "n3");
    assert_eq!(dest, "master");

    match payload {
        Payload::EchoOk(echo_payload) => {
            let EchoPayload { echo } = echo_payload;
            assert_eq!(echo, echo_content);
        }
        other => panic!("expected EchoOk, got {:?}", other),
    }
}

fn default_init_msg() -> Message {
    let init_payload = InitPayload {
        node_id: "n3".into(),
        node_ids: vec!["n1".into(), "n2".into()],
    };

    MessageBuilder::new()
        .with_src("master".into())
        .with_dest("n3".into())
        .with_msg_id(1)
        .with_payload(Payload::Init(init_payload))
        .build()
}
