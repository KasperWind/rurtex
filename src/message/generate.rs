use serde_derive::{Serialize, Deserialize};
use ulid::Ulid;

use super::{BodyRequestBase, Payload, BodyResponseBase};

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateRequest { 
    #[serde(flatten)]
    pub body: BodyRequestBase,
}

impl<'a> GenerateRequest {
    #[allow(dead_code)]
    pub fn respond(self) -> Option<Payload<'a>> {
        let Ulid(id) = Ulid::new();
        Some(Payload::GenerateOk {
            g: GenerateResponse {
                body : BodyResponseBase { in_reply_to: self.body.msg_id},
                id
            }
        }
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateResponse {
    #[serde(flatten)]
    pub body: BodyResponseBase,
    pub id: u128,
}
