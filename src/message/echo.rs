use serde_derive::{Serialize, Deserialize};

use super::Respond;

#[derive(Serialize, Deserialize)]
pub struct EchoRequest<'a> {
    pub echo: &'a str,
}

impl<'a> Respond<'a> for EchoRequest<'a> {
    fn respond(self, msg_id: isize) -> Option<(super::Payload<'a>, &'a str)> {
        Some((super::Payload::EchoOk (
            EchoResponse { in_reply_to: msg_id, echo: self.echo }
            ),"echo_ok"))
    }
}

#[derive(Serialize, Deserialize)]
pub struct EchoResponse<'a> {
    pub in_reply_to: isize,
    pub echo: &'a str,
}
