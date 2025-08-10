#![allow(unused_imports)]
use std::thread;
use std::net::TcpListener;
use std::io::{Read,Write};


fn main() {

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let mut handles = Vec::new();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {

                println!("accepted new connection");
                let thread = thread::spawn(move ||{
                    let mut buf = [0;512];
                    loop {

                        let bytes_count = stream.read(&mut buf).unwrap();
                        if bytes_count == 0{
                            break;
                        }
                        stream.write_all(b"+PONG\r\n").unwrap();
                    }
                });
                handles.push(thread);

            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
