mod broadcast;
mod echo;
mod generate;
use serde::{Deserialize, Serialize};

use self::{
    broadcast::{BroadcastRequest, BroadcastResponse, ReadResponse, ReadRequest, TopologyRequest, TopologyResponse},
    echo::{EchoRequest, EchoResponse},
    generate::{GenerateRequest, GenerateResponse},
};

#[derive(Serialize, Deserialize)]
pub struct HeaderMessage<'a> {
    pub src: &'a str,
    #[serde(rename = "dest")]
    pub dst: &'a str,
    pub body: Body<'a>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Body<'a> {
    pub msg_id: isize,
    #[serde(rename = "type")]
    pub type_: &'a str,
    #[serde(borrow, flatten)]
    pub payload: Payload<'a>,
}


#[derive(Serialize, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "snake_case")]
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
    Broadcast(BroadcastRequest),
    BroadcastOk(BroadcastResponse),
    Read(ReadRequest),
    ReadOk(ReadResponse),
    #[serde(borrow)]
    Topology(TopologyRequest<'a>),
    TopologyOk(TopologyResponse),
}

#[derive(Serialize, Deserialize)]
pub struct InitRequest<'a> {
    pub node_id: &'a str,
    pub node_ids: Vec<&'a str>,
}

impl<'a> InitRequest<'a> {
    #[allow(dead_code)]
    pub fn respond(self, msg_id: isize) -> Option<(Payload<'a>, &'a str)> {
        Some((
            Payload::InitOk(InitResponse {
                in_reply_to: msg_id,
            }),
            "init_ok",
        ))
    }
}

#[derive(Serialize, Deserialize)]
pub struct InitResponse {
    pub in_reply_to: isize,
}
