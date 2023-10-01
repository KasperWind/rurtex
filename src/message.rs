use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
#[serde(rename_all="snake_case")]
pub enum Body<'a> {
    #[serde(borrow)]
    Init(InitRequest<'a>),
    #[serde(borrow)]
    InitOk(InitResponse<'a>),
    #[serde(borrow)]
    Echo(EchoRequest<'a>),
    #[serde(borrow)]
    EchoOk(EchoResponse<'a>),
}

#[derive(Serialize, Deserialize)]
pub struct HeaderMessage<'a> {
    pub src: &'a str,
    #[serde(rename = "dest")]
    pub dst: &'a str,
    pub body: Body<'a>,
}

#[derive(Serialize, Deserialize)]
pub struct InitRequest<'a> {
    pub msg_id: isize,
    #[serde(rename = "type")]
    pub type_: &'a str,
    pub node_id: &'a str,
    pub node_ids: Vec<&'a str>, 
}

#[derive(Serialize, Deserialize)]
pub struct InitResponse<'a> {
    pub msg_id: isize,
    #[serde(rename = "type")]
    pub type_: &'a str,
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
    pub msg_id: isize,
    #[serde(rename = "type")]
    pub type_: &'a str,
    pub echo: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct EchoResponse<'a> {
    pub msg_id: isize,
    #[serde(rename = "type")]
    pub type_: &'a str,
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
