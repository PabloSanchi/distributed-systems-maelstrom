use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: Body,
}

impl Message {
    pub fn new(message: &Message, payload: Payload) -> Self {
        Self {
            src: message.dest.clone(),
            dest: message.src.clone(),
            body: Body {
                msg_id: None,
                in_reply_to: message.body.msg_id,
                payload: payload.to_owned(),
            },
        }
    }
}

pub struct MessageBuilder {
    src: Option<String>,
    dest: Option<String>,
    msg_id: Option<usize>,
    in_reply_to: Option<usize>,
    payload: Option<Payload>,
}

impl Default for MessageBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl MessageBuilder {
    pub fn new() -> Self {
        Self {
            src: None,
            dest: None,
            msg_id: None,
            in_reply_to: None,
            payload: None,
        }
    }

    pub fn from(msg: &Message) -> Self {
        let Message {
            src: to,
            dest: from,
            body,
        } = msg.to_owned();
        let Body { msg_id, .. } = body;

        Self {
            src: Some(from),
            dest: Some(to),
            msg_id: None,
            in_reply_to: msg_id,
            payload: None,
        }
    }

    pub fn with_src(mut self, src: &str) -> Self {
        self.src = Some(src.to_owned());
        self
    }

    pub fn with_dest(mut self, dest: &str) -> Self {
        self.dest = Some(dest.to_owned());
        self
    }

    pub fn with_msg_id(mut self, msg_id: usize) -> Self {
        self.msg_id = Some(msg_id);
        self
    }

    pub fn with_in_reply_to(mut self, in_reply_to: usize) -> Self {
        self.in_reply_to = Some(in_reply_to);
        self
    }

    pub fn with_payload(mut self, payload: Payload) -> Self {
        self.payload = Some(payload);
        self
    }

    pub fn build(self) -> Message {
        if self.src.is_none() {
            panic!("Message src must be defined!")
        }

        if self.dest.is_none() {
            panic!("Message dest must be defined!")
        }

        if self.payload.is_none() {
            panic!("Message payload must be defined!")
        }

        Message {
            src: self.src.unwrap(),
            dest: self.dest.unwrap(),
            body: Body {
                msg_id: self.msg_id,
                in_reply_to: self.in_reply_to,
                payload: self.payload.unwrap(),
            },
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Body {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyPayload {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitPayload {
    pub node_id: String,
    pub node_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EchoPayload {
    pub echo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratePayload {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Payload {
    Init(InitPayload),
    InitOk(EmptyPayload),
    Echo(EchoPayload),
    EchoOk(EchoPayload),
    Generate(EmptyPayload),
    GenerateOk(GeneratePayload),
}
