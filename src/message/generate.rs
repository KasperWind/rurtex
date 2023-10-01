use serde_derive::{Serialize, Deserialize};
use super::Respond;
use ulid::Ulid;

#[derive(Serialize, Deserialize)]
pub struct GenerateRequest { }

impl<'a> Respond<'a> for GenerateRequest {
    fn respond(self, msg_id: isize) -> Option<(super::Payload<'a>, &'a str)> {
        let Ulid(id) = Ulid::new();
        Some((super::Payload::GenerateOk (
            GenerateResponse {in_reply_to:msg_id, id}
            ),"generate_ok"))
    }
}

#[derive(Serialize, Deserialize)]
pub struct GenerateResponse {
    pub in_reply_to: isize,
    pub id: u128,
}
