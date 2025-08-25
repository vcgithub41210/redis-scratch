use crate::ResponseContent;

pub fn encode_response(response: ResponseContent) -> String {
    match response {
        ResponseContent::Integer(value) => format!(":{}\r\n",value),
        ResponseContent::BulkString(value) => format!("${}\r\n{}",if value.len() == 0 {"-1".to_string()} else {value.len().to_string() }, if value.len() == 0 {"".to_string() } else {format!("{}\r\n",value)}),
        ResponseContent::Array(value) => {
            let mut result = format!("*{}\r\n",values.len());
            for item in values {
                result.push_str(&encode_response(item));
            }
            result
        }
        ResponseContent::SimpleString(value) => format!("+{}\r\n", value),
    }
}
