
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
        let (command, args) = token::parse_command(&buf);
        match command.as_str() {
            "RPUSH" => {
                let n = args.len();
                let mut map_lock = map.lock().unwrap();
                let search_key = args[0].to_string();
                if let Some(value) = map_lock.get_mut(&search_key) {
                    match value {
                        Value::List {items, expires} => {
                            for i in 1..n {
                                items.push(args[1].to_string());
                            }
                            let len = items.len();
                            stream.write_all(format!(":{}\r\n",len).as_bytes()).unwrap();
                        }
                        _ => {
                            stream.write_all(b"-ERR wrong type\r\n").unwrap();
                        }
                    }
                } else {
                    let mut value = Value::new_list(Vec::new(),None);
                    match value {
                        Value::List {items,expires} => {
                            for i in 1..n {
                                items.push(args[i].to_string());
                            }
                        }
                    }
                    map_lock.insert(search_key, value);
                    stream.write_all(b":{}\r\n",n-1).unwrap();
                }
            }
            "ECHO" => {
                format_and_send_respones(&mut stream, Some(&args[0]));
            }
            "SET" => {
                let mut map_lock = map.lock().unwrap();
                let key = args[0].to_string();
                let mut value = Value::new_string(args[1].to_string(),None);
                if args.len() > 2 && args[2] == "px" {
                    let expires = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64) + args[3].parse::<u64>().unwrap();
                    value.set_expires(Some(expires));
                }
                map_lock.insert(key,value);
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
                            format_and_send_response(&mut stream, value.get_value());
                        }
                    }
                    else {
                        format_and_send_response(&mut stream, value.get_value());
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

