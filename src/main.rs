use std::io::{self, BufRead, Write};
use message::{HeaderMessage, InitRequest, EchoRequest};
use serde::{Deserialize, Serialize};

mod message;

fn main() {
    let input = io::stdin();
    let mut output = io::stdout().lock();
    let mut handle = input.lock();
    let mut buffer = String::new();
    handle.read_line( &mut buffer).expect("Should read fine");

    let init_request: HeaderMessage<InitRequest> = serde_json::from_str(&buffer).expect("InitMessage");
    let init_response = init_request.repond();

    let json = serde_json::to_string(&init_response).expect("valid internal json");
    let json = format!("{json}\n");  

    output.write(json.as_bytes()).expect("write init ok");

    output.flush().unwrap();

    loop {

        buffer.clear();

        handle.read_line( &mut buffer).expect("Should read fine");

        let echo_request: HeaderMessage<EchoRequest> = serde_json::from_str(&buffer).expect("EchoMessage");
        let echo_response = echo_request.repond();

        let json = serde_json::to_string(&echo_response).expect("valid internal json");
        let json = format!("{json}\n");  

        output.write(json.as_bytes()).expect("write ok Echo Response");

        output.flush().unwrap();
    }
}
