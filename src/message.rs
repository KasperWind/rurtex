use serde::{Serialize, Deserialize};

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
            Payload::Init(_) => Some((Payload::InitOk(InitResponse { in_reply_to: self.body.msg_id }),"init_ok")),
            Payload::InitOk(_) => None,
            Payload::Echo(e) => Some((Payload::EchoOk(
                EchoResponse { in_reply_to: self.body.msg_id, echo: e.echo }
            ),"echo_ok")),
            Payload::EchoOk(_) => None,
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
}

#[derive(Serialize, Deserialize)]
pub struct InitRequest<'a> {
    pub node_id: &'a str,
    pub node_ids: Vec<&'a str>, 
}

#[derive(Serialize, Deserialize)]
pub struct InitResponse {
    pub in_reply_to: isize,
}

// impl<'a> HeaderMessage<'a, InitRequest<'a>> {
//
//     pub fn repond(&self) -> HeaderMessage<'a, InitResponse<'a>> {
//
//         HeaderMessage { src: self.dst, dst: self.src, body: 
//             InitResponse { msg_id: self.body.msg_id, type_: "init_ok", in_reply_to: self.body.msg_id } }
//     }
// }

#[derive(Serialize, Deserialize)]
pub struct EchoRequest<'a> {
    pub echo: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct EchoResponse<'a> {
    pub in_reply_to: isize,
    pub echo: &'a str,
}

// impl<'a> HeaderMessage<'a, EchoRequest<'a>> {
//
//     pub fn repond(&self) -> HeaderMessage<'a, EchoResponse<'a>> {
//
//         HeaderMessage { src: self.dst, dst: self.src, body: 
//             EchoResponse { msg_id: self.body.msg_id, 
//                 type_: "echo_ok", 
//                 in_reply_to: self.body.msg_id,
//                 echo: self.body.echo,
//             } }
//     }
// }
