use std::io::{self, BufRead, Write};
use serde_json::{json, Value};

fn main() {
    let input = io::stdin();
    let mut output = io::stdout().lock();
    let mut handle = input.lock();
    // let mut buffer = [0; 4096];
    let mut buffer = String::new();
    let size = handle.read_line( &mut buffer).expect("Should read fine");
    
    let value: Value = serde_json::from_str(&buffer).expect("proper formattet json");
    let body = value.get("body").expect("body");

    eprintln!("{value}");
    let src = value.get("src").expect("src").as_str();
    let dst = value.get("dest").expect("dest").as_str();
    let msg_id = body.get("msg_id").expect("msg_id node").as_u64().expect("msg_id value");

    let json = json!({
        "src": dst,
        "dest": src,
        "body": 
        {
            "type" : "init_ok", 
            "in_reply_to" : msg_id }
        } );

    let json = serde_json::to_string(&json).expect("valid internal json");
    let json = format!("{json}\n");  

    output.write(json.as_bytes()).expect("write ok");

    output.flush().unwrap();

    loop {

        // let size = handle.read_line( &mut buffer).expect("Should read fine");
        // 
        // let value: Value = serde_json::from_str(&buffer).expect("proper formattet json");
        // let value = value.get("body").expect("body");
        //
        // eprintln!("{value}");
        // let msg_id = value.get("msg_id").expect("msg_id node").as_u64().expect("msg_id value");
        //
        // let json = format!("{{\"type\" : \"init_ok\", \"in_reply_to\" : {msg_id} }}\n" );
        //
        // output.write(buffer.as_bytes()).expect("write ok");
        //
        // output.flush().unwrap();
    }

    



    // output.write(buffer.as_bytes()).expect("write ok");
    //
    // let ex = json!({
    //     "kasper": "fornavn",
    //     "wind": "efternavn"
    // });
    //
    // let str = format!("{ex}");
    // output.write(str.as_bytes()).expect("write2 ok");
    // let deserializer:Value = serde_json::from_str(&str).unwrap();
    //
    // let str = format!("{deserializer}");
    // output.write(str.as_bytes()).expect("write3 ok");

}
