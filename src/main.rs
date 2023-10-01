use std::io::{self, BufRead, Write};
use message::{HeaderMessage, Body};


mod message;

fn main() {
    let input = io::stdin();
    let mut output = io::stdout().lock();
    let mut handle = input.lock();
    let mut buffer = String::new();
    handle.read_line( &mut buffer).expect("Should read fine");

    let init_request: HeaderMessage = serde_json::from_str(&buffer).expect("InitMessage");
    let b = match init_request.body {
        Body::Init(b) => b,
        _ => panic!("not allowed"),
    };
    let init_response = HeaderMessage {src: init_request.dst, dst: init_request.src,
        body: Body::InitOk(message::InitResponse { msg_id: b.msg_id, type_: "init_ok", in_reply_to: b.msg_id })
    };

    let json = serde_json::to_string(&init_response).expect("valid internal json");
    let json = format!("{json}\n");  

    output.write(json.as_bytes()).expect("write init ok");

    output.flush().unwrap();

    loop {

        buffer.clear();

        handle.read_line( &mut buffer).expect("Should read fine");

        let echo_request: HeaderMessage = serde_json::from_str(&buffer).expect("EchoMessage");

        let b = match echo_request.body {
            Body::Echo(e) => e,
            _ => panic!("not allowed"),
        };
        let echo_response = HeaderMessage {
            src: echo_request.dst,
            dst: echo_request.src,
            body: Body::EchoOk(message::EchoResponse { msg_id: b.msg_id, type_: "echo_ok", in_reply_to: b.msg_id, echo: b.echo })
        };
        
        let json = serde_json::to_string(&echo_response).expect("valid internal json");
        let json = format!("{json}\n");  

        output.write(json.as_bytes()).expect("write ok Echo Response");

        output.flush().unwrap();
    }
}
