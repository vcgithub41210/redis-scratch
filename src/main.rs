#![allow(unused_imports)]
use std::thread;
use std::net::TcpListener;
use std::io::{Read,Write};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
mod token;
mod handle_client;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let mut handles = Vec::new();
    let map = Arc::new(Mutex::new(HashMap::new()));
    println!("did it work");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let map_clone = Arc::clone(&map);

                println!("accepted new connection");
                let thread = thread::spawn(move ||{
                    handle_client::handle_client(stream, map_clone);
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
