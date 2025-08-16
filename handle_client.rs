
use std::net::TcpStream;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::token;


pub fn handle_client(mut stream: TcpStream, map: Arc<Mutex<HashMap<String, String>>>) {
    let mut buf = [0; 512];
    loop {
        let bytes_count = stream.read(&mut buf).unwrap();
        if bytes_count == 0 {
            break;
        }
        let tokens = token::parse_command(&buf, bytes_count);
        match tokens[0].as_str() {
            "ECHO" => {
                let response = format!("+{}\r\n",tokens[1].to_string());
                stream.write_all(response.as_bytes()).unwrap();
            }
            "SET" => {
                let mut map_lock = map.lock().unwrap();
                map_lock.insert(tokens[1].to_string(), tokens[2].to_string());
                stream.write_all(b"+OK\r\n").unwrap();
            }
            "GET" => {
                let map_lock = map.lock().unwrap();
                if let Some(value) = map_lock.get(&tokens[1]) {
                    let response = format!("+{}\r\n", value);
                    stream.write_all(response.as_bytes()).unwrap();
                } else {
                    stream.write_all(b"$-1\r\n").unwrap();
                }
            }
            "PING" => {
                stream.write_all(b"+PONG\r\n").unwrap();
            }
            _ => {
                stream.write_all(b"-ERR unknown command\r\n").unwrap();
            }
        }
    }
}

