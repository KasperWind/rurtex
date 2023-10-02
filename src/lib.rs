use std::io::{self, BufRead, Write};

use error::Result;
use message::{HeaderMessage, Payload, Body};

mod message;
pub mod error;

pub struct Rurtex {
    pub(crate) msg: Vec<isize>,
}

impl Rurtex {
    fn process<'a>(&mut self, msg: HeaderMessage<'a>)  -> Option<HeaderMessage<'a>> {

        let resp: Option<(Payload<'_>, &'a str)> = match msg.body.payload {
            Payload::Init(i) => i.respond(msg.body.msg_id),
            Payload::InitOk(_) => None,
            Payload::Echo(e) => e.respond(msg.body.msg_id),
            Payload::EchoOk(_) => None,
            Payload::Generate(g) => g.respond(msg.body.msg_id),
            Payload::GenerateOk(_) => None,
            Payload::Broadcast(b) => {
                self.msg.push(b.message);
                b.respond(msg.body.msg_id)
            },
            Payload::BroadcastOk(_) => None,
            Payload::Read(r) => {
                r.respond(&self.msg, msg.body.msg_id)
            },
            Payload::ReadOk(_) => None,
            Payload::Topology(t) => t.respond(msg.body.msg_id),
            Payload::TopologyOk(_) => None,

        };

        if let Some((payload, type_)) = resp {
            Some(HeaderMessage {
                src: msg.dst,
                dst: msg.src,
                body: Body {
                    msg_id: msg.body.msg_id,
                    type_,
                    payload,
                },
            })
        } else {
            None
        }
    }

    pub fn new() -> Rurtex {
        Rurtex { msg: Vec::new() }
    }

    pub fn execute(mut self) -> Result<()> {


        let input = io::stdin();
        let mut output = io::stdout().lock();
        let mut handle = input.lock();
        let mut buffer = String::new();
        handle.read_line( &mut buffer).expect("Should read fine");

        let init_request: HeaderMessage = serde_json::from_str(&buffer).expect("InitMessage");
        let init_response = self.process(init_request);

        let json = serde_json::to_string(&init_response).expect("valid internal json");
        let json = format!("{json}\n");  

        output.write(json.as_bytes()).expect("write init ok");

        output.flush().unwrap();

        loop {

            buffer.clear();

            handle.read_line( &mut buffer).expect("Should read fine");

            let request: HeaderMessage = serde_json::from_str(&buffer).expect("Message");
            let response = self.process(request);
            
            let json = serde_json::to_string(&response).expect("valid internal json");
            let json = format!("{json}\n");  

            output.write(json.as_bytes()).expect("write ok Response");

            output.flush().unwrap();
        }
    }
} 
