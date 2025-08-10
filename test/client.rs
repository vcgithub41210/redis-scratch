#![allow(unused_imports)]
use std::net::TcpStream;
use std::io::{Read,Write};

fn main(){
    let mut stream = TcpStream::connect("127.0.0.1:6379").unwrap();

    let msg = String::from("PING\n");

    stream.write_all(msg.as_bytes()).unwrap();

    let mut buf = [0;512];

    loop{
        let bytes_read = stream.read(&mut buf).unwrap();

        if bytes_read == 0{
            println!("Server closed connection");
            break;
        }
        let message = String::from_utf8_lossy(&buf[..bytes_read]);
        println!("Received: {}",message);
    }
}
