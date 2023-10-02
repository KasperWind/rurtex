use std::collections::BTreeMap;

use serde_derive::{Serialize, Deserialize};
use serde_json::Map;

use super::{Payload, BodyRequestBase, BodyResponseBase};

#[derive(Serialize, Deserialize)]
pub struct BroadcastRequest {
    #[serde(flatten)]
    pub body: BodyRequestBase,
    pub message: isize,
}

impl<'a> BroadcastRequest {
    #[allow(dead_code)]
    pub fn respond(self) -> Option<Payload<'a>> {

        Some(Payload::BroadcastOk{b: BroadcastResponse {
            body: BodyResponseBase { in_reply_to: self.body.msg_id }
        }})
    }
}

#[derive(Serialize, Deserialize)]
pub struct BroadcastResponse {
    #[serde(flatten)]
    pub body: BodyResponseBase,
}

#[derive(Serialize, Deserialize)]
pub struct ReadRequest { 
    #[serde(flatten)]
    pub body: BodyRequestBase,
}

impl<'a> ReadRequest {
    #[allow(dead_code)]
    pub fn respond(self, messages: &Vec<isize>) -> Option<Payload<'a>> {
        Some(Payload::ReadOk{r: ReadResponse { 
            body: BodyResponseBase { in_reply_to: self.body.msg_id},
            messages: messages.clone() 
        }})
    }
}

#[derive(Serialize, Deserialize)]
pub struct ReadResponse {
    #[serde(flatten)]
    pub body: BodyResponseBase,
    pub messages: Vec<isize>,
}

#[derive(Serialize, Deserialize)]
pub struct TopologyRequest<'a> {
    #[serde(flatten)]
    pub body: BodyRequestBase,
    #[serde(borrow)]
    pub topology: BTreeMap<&'a str, Vec<&'a str>>,

}

impl<'a> TopologyRequest<'a> {
    #[allow(dead_code)]
    pub fn respond(self) -> Option<Payload<'a>> {

        Some(Payload::TopologyOk{t: TopologyResponse { body: BodyResponseBase { in_reply_to:  self.body.msg_id }}})
    }
}

#[derive(Serialize, Deserialize)]
pub struct TopologyResponse {
    #[serde(flatten)]
    pub body: BodyResponseBase,
}
