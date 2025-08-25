
use std::net::TcpStream;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::{SystemTime,UNIX_EPOCH};
use crate::token;
use crate::Value;
use crate::ResponseContent;
use crate::encoder;

pub fn handle_client(mut stream: TcpStream, map: Arc<Mutex<HashMap<String, Value>>>) {
    let mut buf = [0; 512];
    loop {
        let bytes_count = stream.read(&mut buf).unwrap();
        if bytes_count == 0 {
            break;
        }
        let (command, args) = token::parse_command(&buf,bytes_count);
        match command.as_str() {
            "LPOP" => {
                let mut map_lock = map.lock().unwrap();
                let key = args[0].to_string();
                let mut content = ResponseContent::BulkString("".to_string());
                if let Some(value) = map_lock.get_mut(&key) {
                    let mut count = 1;
                    if let Some(arg) = args.get(1) {
                        count = arg.parse().unwrap_or(1);
                    }

                    match value {
                        Value::List {items, ..} => {

                            let mut vector = vec![];
                            while count > 0 && !items.is_empty() {
                                let item = items.remove(0);
                                vector.push(ResponseContent::BulkString(item));
                                count -= 1;
                            }
                            if !vector.is_empty() {
                                if vector.len() == 1{
                                    match &vector[0] {
                                        ResponseContent::BulkString(bulkstr) => content = ResponseContent::BulkString(bulkstr.to_string()),
                                        _ => {}
                                    }
                                } else {
                                    content = ResponseContent::Array(vector);
                                }
                            }
                        }
                        - => {
                            stream.write_all(b"-ERR wrong type\r\n").unwrap();
                        }
                    }
                }
                let formatted_response = encoder::encode_response(content);
                stream.write_all(formatted_response.as_bytes()).unwrap();
            }
            "LLEN" => {
                let map_lock = map.lock().unwrap();
                let key = args[0].to_string();
                let mut content = ResponseContent::Integer(0);
                if let Some(value) = map_lock.get(&key) {
                    match value {
                        Value::List {items, .. } => {
                            content = ResponseContent::Integer(items.len());
                        }
                        _ => {
                            stream.write_all(b"-ERR wrong type\r\n").unwrap();
                        }
                    }
                }
                let formatted_response = encoder::encoder_response(content);
                stream.write_all(formatted_response.as_bytes()).unwrap();
            }
            "LRANGE" => {
                let n = args.len();
                let map_lock = map.lock().unwrap();

                if n < 3 {
                    stream.write_all(b"-ERR wrong number of arguements for 'lrange' command\r\n").unwrap();
                    continue;
                }
                let search_key = args[0].to_string();
                let mut content = ResponseContent::Array(vec![]);

                if let Some(value) = map_lock.get(&search_key) {
                    let mut start: isize = args[1].parse().unwrap_or(0);
                    let mut end: isize = args[2].parse().unwrap_or(0);

                    match value {
                        Value::List {items, ..} => {
                            let len = items.len() as isize;
                            if start < 0 {
                                start += len;
                                if start < 0 {
                                    start = 0;
                                }
                            }
                            if end < 0 {
                                end += len;
                                if end < 0 {
                                    end = 0;
                                }
                            }
                            if end >= len {
                                end = len -1;
                            }
                            if start <= end && start < len && start >= 0 {
                                content = ResponseContent::Array(
                                    items.iter()
                                    .skip(start as usize)
                                    .take((end - start+1) as usize)
                                    .map(|item| ResponseContent::BulkString(item.clone()))
                                    .collect()
                                    );
                            }
                        }
                        _ => {
                        }
                    }
                }
                let formatted_response = encoder::encode_response(content);
                stream.write_all(formatted_response.as_bytes()).unwrap();
            }
            "LPUSH" => {
                let n = args.len();
                let mut map_lock = map.lock().unwrap();
                let search_key = args[0].to_string();
                if let Some(value) = map_lock.get_mut(&search_key) {
                    match value {
                        Value::List {items,..} => {
                            for i in 1..n {
                                items.insert(0,args[i].to_string());
                            }
                            let content = ResponseContent::Integer(items.len());
                            let formatted_response = encoder::encode_response(content);
                            stream.write_all(formatted_response.as_bytes()).unwrap();
                        }
                        _ => {
                            stream.write_all(b"-ERR wrong type\r\n").unwrap();
                        }
                    }
                } else {
                    let mut value = Value::new_list(Vec::new(),None);
                    match &mut value {
                        Value::List {items,..} => {
                            for i in 1..n {
                                items.insert(0,args[i].to_string());
                            }
                        }
                        _ => {
                            stream.write_all(b"-ERR wrong type\r\n").unwrap();
                        }
                    }
                    let content = ResponseContent::Integer(n-1);
                    map_lock.insert(search_key,value);
                    let formatted_response = encoder::encode_response(content);
                    stream.write_all(formatted_response.as_bytes()).unwrap();
                }
            }
            "RPUSH" => {
                let n = args.len();
                let mut map_lock = map.lock().unwrap();
                let search_key = args[0].to_string();
                if let Some(value) = map_lock.get_mut(&search_key) {
                    match value {
                        Value::List {items, .. } => {
                            for i in 1..n {
                                items.push(args[i].to_string());
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
                    match &mut value {
                        Value::List {items, .. } => {
                            for i in 1..n {
                                items.push(args[i].to_string());
                            }
                        }
                        _ => {
                            stream.write_all(b"-ERR wrong type\r\n").unwrap();
                        }
                    }
                    map_lock.insert(search_key, value);
                    stream.write_all(format!(":{}\r\n",n-1).as_bytes()).unwrap();
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

