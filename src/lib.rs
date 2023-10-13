use std::{io::{self, BufRead, Write}, collections::HashSet};

use error::Result;
use message::{HeaderMessage, Payload, BodyRequestBase, broadcast::ReadRequest};
// use tracing::debug;
// use ulid::Ulid;
//
// how is it going, ? is it okay still

mod message;
pub mod error;

pub struct Rurtex {
    pub(crate) id: String,
    pub(crate) ids: Vec<String>,
    pub(crate) msg: HashSet<isize>,
    pub(crate) topology: HashSet<String>,
    pub(crate) rcv_count: usize,
}

impl Rurtex {
    fn process<'a>(&mut self, msg: HeaderMessage<'a>)  -> Option<(HeaderMessage<'a>, Vec<HeaderMessage<'_>>)> {

        let mut v = Vec::new();
        let resp: Option<Payload<'_>> = match msg.body {
            Payload::Init{i, ..} => {
                self.id = i.node_id.to_owned();
                for &x in i.node_ids.iter() {
                    self.ids.push(x.to_owned());
                }
                i.respond()
            },
            Payload::InitOk{ .. } => None,
            Payload::Echo{e, ..} => e.respond(),
            Payload::EchoOk{ .. } => None,
            Payload::Generate{g, ..} => g.respond(),
            Payload::GenerateOk{ ..} => None,
            Payload::Broadcast{b, ..} => {
                // tracing::debug!("broadcast rcv, count {}", self.rcv_count);
                self.rcv_count = self.rcv_count + 1;
                if self.rcv_count > 1 {

                    for i in self.ids.iter() {
                        let msg_id = rand::random();
                        let body = Payload::Read {
                            r: ReadRequest { body: BodyRequestBase {
                                msg_id
                            } } };
                        let message = HeaderMessage {
                            src: &self.id,
                            dst: &i,
                            body
                        };
                        v.push(message);
                    }
                    self.rcv_count = 0;
                    tracing::info!("send extra msg:{:?} ", v);
                }
                self.msg.insert(b.message);
                b.respond()
            },
            Payload::BroadcastOk{ .. } => None,
            Payload::Read{r, ..} => {
                r.respond(&self.msg)
            },
            Payload::ReadOk{r} => {
                // r.messages
                // debug!("ReadOk: {:?}", r);
                for &m in r.messages.iter() {
                    self.msg.insert(m);
                }

                None
            },
            Payload::Topology{t, ..} => {
                tracing::info!("topology request,{:?} ", t.topology);

                for (&k, v) in t.topology.iter() {
                    if k == self.id {
                        for &v in v.iter() {
                            self.topology.insert(v.to_owned());
                        }
                    }
                }

                tracing::info!("new topology,{:?} ", self.topology);
                t.respond()
            },
            Payload::TopologyOk{ .. } => None,

        };

        if let Some(payload) = resp {
            Some((HeaderMessage {
                src: msg.dst,
                dst: msg.src,
                body: payload,
            }, v))
        } else {
            None
        }
    }

    pub fn new() -> Rurtex {
        Rurtex { msg: HashSet::new(), topology: HashSet::new(), rcv_count: 0, id: String::new(), ids: Vec::new() }
    }

    pub fn execute(mut self) -> Result<()> {


        let input = io::stdin();
        let mut output = io::stdout().lock();
        let mut handle = input.lock();
        let mut buffer = String::new();
        handle.read_line( &mut buffer).expect("Should read fine");

        let init_request: HeaderMessage = serde_json::from_str(&buffer).expect("InitMessage");

        let init_response = self.process(init_request);

        if let Some((init_response, _)) = init_response {
            let json = serde_json::to_string(&init_response).expect("valid internal json");
            let json = format!("{json}\n");  
            output.write(json.as_bytes()).expect("write init ok");
            output.flush().unwrap();
        }

        tracing::info!("init done, id: {}, ids:{:?} ", self.id, self.ids);

        let do = false;
        let something = "".to_string();

        let how = "it is okay so far";

        if do {
            let new_how = how;
        }



        loop {

            buffer.clear();

            handle.read_line( &mut buffer).expect("Should read fine");
            let request: HeaderMessage = serde_json::from_str(&buffer).expect("Message");
            let response = self.process(request);

            if let Some((response, msg)) = response {
            
                let json = serde_json::to_string(&response).expect("valid internal json");
                let json = format!("{json}\n");  

                output.write(json.as_bytes()).expect("write ok Response");
                output.flush().unwrap();
                for m in msg.iter() {
                    
                    let json = serde_json::to_string(&m).expect("valid internal json");
                    let json = format!("{json}\n");  

                    output.write(json.as_bytes()).expect("write ok Response");

                    output.flush().unwrap();
                }
            }

            output.flush().unwrap();
        }
    }
} 
