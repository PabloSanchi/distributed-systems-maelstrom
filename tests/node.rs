use challenge::{Body, Node, Message, Payload};
use std::string::String;

#[test]
fn init_use_case() {
    // given
    let mut node = Node::new();
    let init_msg = default_init_msg();

    // when
    let reply = node
        .handle_msg(init_msg)
        .expect("init_ok reply is expected");

    // then
    let Message { src, dest, body } = reply;
    let Body {
        in_reply_to,
        payload,
        ..
    } = body;

    assert_eq!(src, "n3");
    assert_eq!(dest, "master");
    assert_eq!(in_reply_to, Some(1));
    assert!(matches!(payload, Payload::InitOk {}));
}

#[test]
fn echo_use_case() {
    // given
    let mut node = Node::new();
    let init_msg = default_init_msg();

    node.handle_msg(init_msg)
        .expect("init_ok reply is expected");

    let echo_content = String::from("this is the echo msg");
    let echo_msg = Message {
        src: "master".into(),
        dest: "n3".into(),
        body: Body {
            msg_id: Some(1),
            in_reply_to: None,
            payload: Payload::Echo {
                echo: echo_content.clone(),
            },
        },
    };

    // when
    let reply = node
        .handle_msg(echo_msg)
        .expect("init_ok reply is expected");

    // then
    let Message { src, dest, body } = reply;
    let Body {
        in_reply_to,
        payload,
        ..
    } = body;

    assert_eq!(src, "n3");
    assert_eq!(dest, "master");
    assert_eq!(in_reply_to, Some(1));

    match payload {
        Payload::EchoOk { echo } => {
            assert_eq!(echo, echo_content);
        }
        other => panic!("expected EchoOk, got {:?}", other),
    }
}

fn default_init_msg() -> Message {
    Message {
        src: "master".into(),
        dest: "n3".into(),
        body: Body {
            msg_id: Some(1),
            in_reply_to: None,
            payload: Payload::Init {
                node_id: "n3".into(),
                node_ids: vec!["n1".into(), "n2".into()],
            },
        },
    }
}
