use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct EchoRequest<'a> {
    pub echo: &'a str,
}

impl<'a> EchoRequest<'a> {
    #[allow(dead_code)]
    pub fn respond(self, msg_id: isize) -> Option<(super::Payload<'a>, &'a str)> {
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
