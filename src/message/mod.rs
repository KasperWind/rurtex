mod echo;
mod generate;
use serde::{Serialize, Deserialize};

use self::{echo::{EchoResponse, EchoRequest}, generate::{GenerateRequest, GenerateResponse}};

#[derive(Serialize, Deserialize)]
pub struct HeaderMessage<'a> {
    pub src: &'a str,
    #[serde(rename = "dest")]
    pub dst: &'a str,
    pub body: Body<'a>,
}

impl<'a> HeaderMessage<'a> {
    pub fn respond(self) -> Option<HeaderMessage<'a>> {

        let resp:Option<(Payload<'_>, &'a str)> = match self.body.payload {
            Payload::Init(i) => i.respond(self.body.msg_id),
            Payload::InitOk(_) => None,
            Payload::Echo(e) => e.respond(self.body.msg_id),
            Payload::EchoOk(_) => None,
            Payload::Generate(g) => g.respond(self.body.msg_id),
            Payload::GenerateOk(_) => None,
        };

        if let Some((payload, type_)) = resp {

            Some(HeaderMessage { src: self.dst, dst: self.src, body: 
                Body { msg_id: self.body.msg_id, type_, payload }
            })

        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub struct Body<'a> {
    pub msg_id: isize,
    #[serde(rename = "type")]
    pub type_: &'a str,
    #[serde(borrow, flatten)]
    pub payload: Payload<'a>
}

trait Respond<'a> {
   fn respond(self, msg_id: isize) -> Option<(Payload<'a>, &'a str)>;
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
#[serde(rename_all="snake_case")]
pub enum Payload<'a> {
    #[serde(borrow)]
    Init(InitRequest<'a>),
    InitOk(InitResponse),
    #[serde(borrow)]
    Echo(EchoRequest<'a>),
    #[serde(borrow)]
    EchoOk(EchoResponse<'a>),
    Generate(GenerateRequest),
    GenerateOk(GenerateResponse),
}

#[derive(Serialize, Deserialize)]
pub struct InitRequest<'a> {
    pub node_id: &'a str,
    pub node_ids: Vec<&'a str>, 
}

impl<'a> Respond<'a> for InitRequest<'a> {
    fn respond(self, msg_id: isize) -> Option<(Payload<'a>, &'a str)> {
        Some((Payload::InitOk(InitResponse { in_reply_to: msg_id }),"init_ok"))
    }
}

#[derive(Serialize, Deserialize)]
pub struct InitResponse {
    pub in_reply_to: isize,
}


