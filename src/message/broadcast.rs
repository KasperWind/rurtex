use serde_derive::{Serialize, Deserialize};
use serde_json::Map;

use super::Payload;

#[derive(Serialize, Deserialize)]
pub struct BroadcastRequest {
    pub message: isize,
}

impl<'a> BroadcastRequest {
    #[allow(dead_code)]
    pub fn respond(self, msg_id: isize) -> Option<(Payload<'a>, &'a str)> {

        Some((Payload::BroadcastOk(BroadcastResponse { in_reply_to: msg_id }),"broadcast_ok"))
    }
}

#[derive(Serialize, Deserialize)]
pub struct BroadcastResponse {
    pub in_reply_to: isize,
}

#[derive(Serialize, Deserialize)]
pub struct ReadRequest { }

impl<'a> ReadRequest {
    #[allow(dead_code)]
    pub fn respond(self, messages: &Vec<isize>, msg_id: isize) -> Option<(Payload<'a>, &'a str)> {

        Some((Payload::ReadOk(ReadResponse { in_reply_to: msg_id, messages: messages.clone() }),"read_ok "))
    }
}

#[derive(Serialize, Deserialize)]
pub struct ReadResponse {
    pub in_reply_to: isize,
    pub messages: Vec<isize>,
}

#[derive(Serialize, Deserialize)]
pub struct TopologyRequest<'a> {
    #[serde(borrow)]
    pub topology: Vec<(&'a str, Vec<&'a str>)>,
}

impl<'a> TopologyRequest<'a> {
    #[allow(dead_code)]
    pub fn respond(self, msg_id: isize) -> Option<(Payload<'a>, &'a str)> {

        Some((Payload::TopologyOk(TopologyResponse { in_reply_to: msg_id }),"topology_ok"))
    }
}

#[derive(Serialize, Deserialize)]
pub struct TopologyResponse {
    pub in_reply_to: isize,
}
