use serde_derive::{Serialize, Deserialize};

use super::{Payload, BodyResponseBase, BodyRequestBase};

#[derive(Serialize, Deserialize)]
pub struct EchoRequest<'a> {
    #[serde(flatten)]
    pub body: BodyRequestBase,
    pub echo: &'a str,
}

impl<'a> EchoRequest<'a> {
    #[allow(dead_code)]
    pub fn respond(self) -> Option<Payload<'a>> {
        Some(Payload::EchoOk { e: 
            EchoResponse { 
                echo: self.echo,
                body: BodyResponseBase { in_reply_to: self.body.msg_id }
                }
            }
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct EchoResponse<'a> {
    #[serde(flatten)]
    pub body: BodyResponseBase,
    pub echo: &'a str,
}
