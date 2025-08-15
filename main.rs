#![allow(unused_imports)]
use std::thread;
use std::net::TcpListener;
use std::io::{Read,Write};
use std::sync::{Arc, Mutex};
mod token;

fn echo_contents(contents: String, stream: &mut std::net::TcpStream) {
    let response = format!("+{}\r\n", contents);
    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let mut handles = Vec::new();
    let map = Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let map_clone = Arc::clone(&map);

                println!("accepted new connection");
                let thread = thread::spawn(move ||{
                    let mut buf = [0;512];
                    loop {

                        let bytes_count = stream.read(&mut buf).unwrap();
                        if bytes_count == 0{
                            break;
                        }
                        let tokens = token::parse_command(msg,bytes_count);

                        match tokens[0].as_str(){
                            "ECHO" => {
                                echo_contents(tokens[1].to_string(), &mut stream);
                            }
                            "PING" => {
                                stream.write_all(b"+PONG\r\n").unwrap();
                            }
                            _ => {
                                stream.write_all(b"-ERR unknown command\r\n").unwrap();
                            }
                        }



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
