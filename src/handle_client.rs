
use std::net::TcpStream;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::{SystemTime,UNIX_EPOCH};
use crate::token;
use crate::Value;

pub fn format_and_send_response(stream: &mut TcpStream, value: Option<&String>){
    if let Some(res) = value {
        let formatted_response = format!("+{}\r\n",res);
        stream.write_all(formatted_response.as_bytes()).unwrap();
    } else {
        stream.write_all(b"$-1\r\n").unwrap();
    }
}
pub fn handle_client(mut stream: TcpStream, map: Arc<Mutex<HashMap<String, Value>>>) {
    let mut buf = [0; 512];
    loop {
        let bytes_count = stream.read(&mut buf).unwrap();
        if bytes_count == 0 {
            break;
        }
        let (command, args) = token::parse_command(&buf,bytes_count);
        match command.as_str() {
            "ECHO" => {
                format_and_send_respones(&mut stream, Some(&args[0]));
            }
            "SET" => {
                let mut map_lock = map.lock().unwrap();
                let key = args[0].to_string();
                let mut value = Value::new(args[1].to_string(),None);
                if args.len() > 2 && args[2] == "px" {
                    let expires = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64) + args[3].parse::<u64>().unwrap();
                    value.set_expires(Some(expires));
                }
                map_lock.insert(tokens[1].to_string(), value);
                stream.write_all(b"+OK\r\n").unwrap();
            }
            "GET" => {
                let mut map_lock = map.lock().unwrap();
                let search_key = args[0].to_string();
                if let Some(value) = map_lock.get(&search_key) {
                    if let Some(expires) = value.get_expires(){
                        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
                        if current_time > expires {
                            stream.write_all(b"$-1\r\n").unwrap();
                            map_lock.remove(&search_key);
                        } else {
                            let response = format!("+{}\r\n",value.get_value());
                            stream.write_all(response.as_bytes()).unwrap();
                        }
                    }
                    else {
                        let response = format!("+{}\r\n",value.get_value());
                        stream.write_all(response.as_bytes()).unwrap();
                    }
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

