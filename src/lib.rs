use std::io::{self, BufRead, Write};

use error::Result;
use message::{HeaderMessage, Payload};

mod message;
pub mod error;

pub struct Rurtex {
    pub(crate) msg: Vec<isize>,
}

impl Rurtex {
    fn process<'a>(&mut self, msg: HeaderMessage<'a>)  -> Option<HeaderMessage<'a>> {

        let resp: Option<Payload<'_>> = match msg.body {
            Payload::Init{i, ..} => i.respond(),
            Payload::InitOk{ .. } => None,
            Payload::Echo{e, ..} => e.respond(),
            Payload::EchoOk{ .. } => None,
            Payload::Generate{g, ..} => g.respond(),
            Payload::GenerateOk{ ..} => None,
            Payload::Broadcast{b, ..} => {
                self.msg.push(b.message);
                b.respond()
            },
            Payload::BroadcastOk{ .. } => None,
            Payload::Read{r, ..} => {
                r.respond(&self.msg)
            },
            Payload::ReadOk{ .. } => None,
            Payload::Topology{t, ..} => t.respond(),
            Payload::TopologyOk{ .. } => None,

        };

        if let Some(payload) = resp {
            Some(HeaderMessage {
                src: msg.dst,
                dst: msg.src,
                body: payload,
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

            eprintln!("msg: {buffer}");

            let request: HeaderMessage = serde_json::from_str(&buffer).expect("Message");
            let response = self.process(request);
            
            let json = serde_json::to_string(&response).expect("valid internal json");
            let json = format!("{json}\n");  

            output.write(json.as_bytes()).expect("write ok Response");

            output.flush().unwrap();
        }
    }
} 
