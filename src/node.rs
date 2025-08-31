use crate::message::{Body, Message, Payload};
use crate::writer::NewLineWriter;
use log::info;
use rand::Rng;
use std::io::StdoutLock;
use std::sync::OnceLock;

pub struct Node {
    node_id: OnceLock<String>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            node_id: OnceLock::new(),
        }
    }

    pub fn handle(
        &mut self,
        input_msg: Message,
        output: &mut NewLineWriter<StdoutLock>,
    ) -> std::io::Result<()> {
        if let Some(reply_msg) = self.handle_msg(input_msg) {
            output.write(&reply_msg)?;
        }

        Ok(())
    }

    // visible for testing
    pub fn handle_msg(&mut self, input: Message) -> Option<Message> {
        let Message {
            src: dest, body, ..
        } = input;
        let Body {
            msg_id, payload, ..
        } = body;

        match payload {
            Payload::Echo { echo } => {
                let src = self.src().to_owned();
                let msg = Message {
                    src: src.clone(),
                    dest: dest,
                    body: Body {
                        msg_id: Some(generate_id()),
                        in_reply_to: msg_id,
                        payload: Payload::EchoOk { echo },
                    },
                };

                Some(msg)
            }
            Payload::Init { node_id, .. } => {
                let _ = self.node_id.set(node_id);

                let src = self.src().to_owned();
                let msg = Message {
                    src: src.clone(),
                    dest: dest,
                    body: Body {
                        msg_id: Some(generate_id()),
                        in_reply_to: msg_id,
                        payload: Payload::InitOk {},
                    },
                };

                info!("EchoNode {} was initialized", src.clone());

                Some(msg)
            }
            Payload::InitOk {} => panic!("EchoNode should never receive an InitOk message"),
            Payload::EchoOk { .. } => None,
        }
    }

    fn src(&self) -> &str {
        self.node_id.get().expect("EchoNode was not initialized!")
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}

fn generate_id() -> usize {
    let mut rng = rand::rng();
    rng.random::<u64>() as usize
}
