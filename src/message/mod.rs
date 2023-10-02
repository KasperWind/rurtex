mod echo;
mod generate;
mod broadcast;
use serde::{Deserialize, Serialize};

use self::{
    echo::{EchoRequest, EchoResponse},
    generate::{GenerateRequest, GenerateResponse},
    broadcast::{BroadcastRequest, BroadcastResponse, ReadResponse, ReadRequest, TopologyRequest, TopologyResponse},
};

#[derive(Serialize, Deserialize)]
pub struct HeaderMessage<'a> {
    pub src: &'a str,
    #[serde(rename = "dest")]
    pub dst: &'a str,
    pub body: Payload<'a>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BodyRequestBase {
    pub msg_id: isize,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BodyResponseBase {
    pub in_reply_to: isize,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload<'a> {
    Init{
        #[serde(borrow, flatten)]
        i: InitRequest<'a>
    },
    InitOk{
        #[serde(flatten)]
        i: InitResponse
    },
    Echo{ 
        #[serde(borrow, flatten)]
        e: EchoRequest<'a>
    },
    EchoOk{
        #[serde(borrow, flatten)]
        e: EchoResponse<'a>
    },
    Generate{
        #[serde( flatten)]
        g: GenerateRequest
    },
    GenerateOk{
        #[serde( flatten)]
        g: GenerateResponse
    },
    Broadcast{
		#[serde(flatten)]
		b :BroadcastRequest
	},
    BroadcastOk{
		#[serde(flatten)]
		b :BroadcastResponse
	},
    Read{
		#[serde(flatten)]
		r :ReadRequest
	},
    ReadOk{
		#[serde(flatten)]
		r :ReadResponse
	},
    Topology{
		#[serde(flatten, borrow)]
		t :TopologyRequest<'a>
	},
    TopologyOk{
		#[serde(flatten)]
		t :TopologyResponse
    },
}

#[derive(Serialize, Deserialize)]
pub struct InitRequest<'a> {
    #[serde(flatten)]
    pub body: BodyRequestBase,
    pub node_id: &'a str,
    pub node_ids: Vec<&'a str>,
}

impl<'a> InitRequest<'a> {
    #[allow(dead_code)]
    pub fn respond(self) -> Option<Payload<'a>> {
        Some (Payload::InitOk{ 
            i : InitResponse {
                body: BodyResponseBase {
                    in_reply_to: self.body.msg_id,
                }
            }
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct InitResponse {
    #[serde(flatten)]
    pub body: BodyResponseBase,
}
