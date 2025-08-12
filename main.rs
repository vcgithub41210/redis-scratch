#![allow(unused_imports)]
use std::thread;
use std::net::TcpListener;
use std::io::{Read,Write};
mod token;

fn echo_contents(contents: String, stream: &mut std::net::TcpStream) {
    let response = format!("+{}\r\n", contents);
    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

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
                        //parse command
                        let tokens = token::parse_command(&buf[..bytes_count],bytes_count);
                        let arg = format!("+{}\r\n",tokens[0]);
                        stream.write_all(arg.as_bytes()).unwrap();

                        match tokens[0].as_str(){
                            "ECHO" => {
                                echo_contents(tokens[1].to_string(), &mut stream);
                            }
                            _ => {
                                stream.write_all(b"+PONG\r\n").unwrap();
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
