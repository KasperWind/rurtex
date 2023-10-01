use serde_derive::{Serialize, Deserialize};

use super::Respond;

#[derive(Serialize, Deserialize)]
pub struct GenerateRequest { }

impl<'a> Respond<'a> for GenerateRequest {
    fn respond(self, msg_id: isize) -> Option<(super::Payload<'a>, &'a str)> {
        Some((super::Payload::GenerateOk (
            GenerateResponse {in_reply_to:msg_id, id: rand::random::<usize>()}
            ),"generate_ok"))
    }
}

#[derive(Serialize, Deserialize)]
pub struct GenerateResponse {
    pub in_reply_to: isize,
    pub id: usize,
}
