pub fn parse_command(msg: String, msg_len:usize) -> (String, Vec<String>) {
    let mut tokens = msg.split("\r\n");
    let mut filter = Vec::new();
    for token in tokens {
        if token[0] != '*'{
            filter.push(token.to_string());
        }
    }
    return (filter[0],filter[1..].to_vec());
}
