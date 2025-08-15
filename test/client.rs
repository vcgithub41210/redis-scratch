#![allow(unused_imports)]
use std::net::TcpStream;
use std::io::{self, BufRead, Write, Read};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:6379").unwrap();
    let stdin = io::stdin();

    println!("Connected to server at 127.0.0.1:6379");
    println!("Type commands like: PING, ECHO hello, SET key value, GET key");
    println!("Type EXIT to quit.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("EXIT") {
            break;
        }

        let msg = format!("{}\n", input);
        stream.write_all(msg.as_bytes()).unwrap();

        let mut buf = [0; 512];
        let bytes_read = stream.read(&mut buf).unwrap();
        if bytes_read == 0 {
            println!("Server closed connection");
            break;
        }
        let message = String::from_utf8_lossy(&buf[..bytes_read]);
        println!("Received: {}", message);
    }
}
