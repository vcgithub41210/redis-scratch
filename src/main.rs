#![allow(unused_imports)]
use std::thread;
use std::net::TcpListener;
use std::io::{Read,Write};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
mod token;
mod handle_client;

enum Value {

    String {
        value: String,
        expires: Option<u64>,
    },
    List {
        items: Vec<String>,
        expires: Option<u64>,
    }
}
impl Value {
    pub fn new_string(value: String, expires: Option<u64>) -> Self {
        Value::String {value, expires }
    }
    pub fn new_list(items: String, expires: Option<u64>) -> Self {
        Value::List {items,expires}
    }
    pub fn set_expires(&mut self, expires:Option<u64>) {
        match self {
            Value::String {expires: ref mut e, .. } => *e = expires,
            Value::List {expires: ref mut e, .. } => *e = expires,
        }
    }
    pub fn get_expires(&self) -> Option<u64> {
        match self {
            Value::String {expires: ref mut e, .. } => *e,
            Value::List {expires: ref mut e, ..} => *e,
        }
    }
    pub fn get_value(&self) -> &String {
        match self {
            Value::String {value, .. } => value,
            _ => None,
        }
    }
}

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
