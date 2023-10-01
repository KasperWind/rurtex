use std::io::{self, BufRead, Write};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize)]
struct HeaderMessage<'a, T> {
    pub src: &'a str,
    #[serde(rename = "dest")]
    pub dst: &'a str,
    pub body: T
}


#[derive(Deserialize)]
struct InitRequest<'a> {
    pub msg_id: isize,
    #[serde(rename = "type")]
    pub type_: &'a str,
    pub node_id: &'a str,
    pub node_ids: Vec<&'a str>, 
}

#[derive(Serialize)]
struct InitResponse<'a> {
    pub msg_id: isize,
    #[serde(rename = "type")]
    pub type_: &'a str,
    pub in_reply_to: isize,
}


fn main() {
    let input = io::stdin();
    let mut output = io::stdout().lock();
    let mut handle = input.lock();
    let mut buffer = String::new();
    handle.read_line( &mut buffer).expect("Should read fine");

    let init_request: HeaderMessage<InitRequest> = serde_json::from_str(&buffer).expect("InitMessage");

    let init_response = InitResponse {
        msg_id: init_request.body.msg_id,
        type_: "init_ok",
        in_reply_to: init_request.body.msg_id,
    };

    let init_response = HeaderMessage {
        src: init_request.dst,
        dst: init_request.src,
        body: init_response,
    };

    let json = serde_json::to_string(&init_response).expect("valid internal json");
    let json = format!("{json}\n");  

    output.write(json.as_bytes()).expect("write ok");

    output.flush().unwrap();

    loop {

        buffer.clear();

        handle.read_line( &mut buffer).expect("Should read fine");

        let value: Value = serde_json::from_str(&buffer).expect("proper formattet json echo string");
        let src = value.get("src").expect("src").as_str();
        let dst = value.get("dest").expect("dest").as_str();
        let value = value.get("body").expect("body");

        let msg_id = value.get("msg_id").expect("msg_id node").as_u64().expect("msg_id value");
        let echo = value.get("echo").expect("echo node").as_str().expect("echo value");

        let json = json!({
            "src": dst,
            "dest": src,
            "body": 
            {
                "type" : "echo_ok", 
                "msg_id": msg_id,
                "in_reply_to" : msg_id,
                "echo": echo,
            }
        } );

        let json = serde_json::to_string(&json).expect("valid internal json");
        let json = format!("{json}\n");  

        output.write(json.as_bytes()).expect("write ok");

        output.flush().unwrap();
    }
}
