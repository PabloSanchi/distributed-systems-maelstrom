use crate::message::{
    EchoPayload, EmptyPayload, GeneratePayload, InitPayload, Message, MessageBuilder, Payload,
};
use crate::writer::NewLineWriter;
use std::io::StdoutLock;
use std::sync::OnceLock;

pub struct Node {
    node_id: OnceLock<String>,
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
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
        if let Some(reply_msg) = self.generate_msg(&input_msg) {
            output.write(&reply_msg)?;
        }

        Ok(())
    }

    #[inline]
    fn assert_initialized(&self) {
        if self.node_id.get().is_none() {
            panic!("Node was not initalized!")
        }
    }
}

pub trait MessageGenerator {
    fn generate_msg(&mut self, message: &Message) -> Option<Message>;

    fn handle_init(&mut self, payload: InitPayload) -> Payload;
    fn handle_echo(&self, payload: EchoPayload) -> Payload;
    fn handle_generate(&self) -> Payload;
}

impl MessageGenerator for Node {
    fn generate_msg(&mut self, message: &Message) -> Option<Message> {
        use Payload::*;
        let generated_payload = match message.body.payload.clone() {
            Init(init_payload) => self.handle_init(init_payload),
            Echo(echo_payload) => self.handle_echo(echo_payload),
            Generate(_) => self.handle_generate(),
            _ => panic!("Node received invalid message!"),
        };

        let msg = MessageBuilder::from(&message)
            .with_msg_id(1) // todo: use a distributed id generator
            .with_payload(generated_payload)
            .build();

        Some(msg)
    }

    fn handle_init(&mut self, payload: InitPayload) -> Payload {
        let InitPayload { node_id, .. } = payload;
        let _ = self.node_id.set(node_id);
        Payload::InitOk(EmptyPayload {})
    }

    fn handle_echo(&self, payload: EchoPayload) -> Payload {
        self.assert_initialized();
        Payload::EchoOk(payload)
    }

    fn handle_generate(&self) -> Payload {
        self.assert_initialized();
        Payload::GenerateOk(GeneratePayload {
            id: String::from("some id"),
        })
    }
}
