use std::io::{self, BufRead, Write};
use message::HeaderMessage;
mod message;

fn main() {
    let input = io::stdin();
    let mut output = io::stdout().lock();
    let mut handle = input.lock();
    let mut buffer = String::new();
    handle.read_line( &mut buffer).expect("Should read fine");

    let init_request: HeaderMessage = serde_json::from_str(&buffer).expect("InitMessage");
    let init_response = init_request.respond();

    let json = serde_json::to_string(&init_response).expect("valid internal json");
    let json = format!("{json}\n");  

    output.write(json.as_bytes()).expect("write init ok");

    output.flush().unwrap();

    loop {

        buffer.clear();

        handle.read_line( &mut buffer).expect("Should read fine");

        let request: HeaderMessage = serde_json::from_str(&buffer).expect("Message");
        let response = request.respond();
        
        let json = serde_json::to_string(&response).expect("valid internal json");
        let json = format!("{json}\n");  

        output.write(json.as_bytes()).expect("write ok Response");

        output.flush().unwrap();
    }
}
