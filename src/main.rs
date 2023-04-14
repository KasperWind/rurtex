use std::io::{self, BufRead, Write};

fn main() {
    let input = io::stdin();
    let mut output = io::stdout().lock();
    let mut handle = input.lock();
    // let mut buffer = [0; 4096];
    let mut buffer = String::new();
    let size = handle.read_line( &mut buffer).expect("Should read fine");
    
    output.write(buffer.as_bytes()).expect("write ok");

}
