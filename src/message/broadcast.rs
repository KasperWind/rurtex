use serde_derive::{Serialize, Deserialize};

use super::Payload;

#[derive(Serialize, Deserialize)]
pub struct BroadcastRequest {
    pub message: isize,
}

impl<'a> BroadcastRequest {
    pub fn respond(self, msg_id: isize) -> Option<(Payload<'a>, &'a str)> {
        Some((Payload::BroadcastOk(BroadcastResponse { in_reply_to: msg_id }),"init_ok"))
    }
}

#[derive(Serialize, Deserialize)]
pub struct BroadcastResponse {
    pub in_reply_to: isize,
}

