
pub fn parse_command(msg: &[u8],len: usize) -> (String,Vec<String>) {
    let res = String::from_utf8_lossy(&buf[..len]);
    let tokens = res.split("\r\n")
        .filter(|line| !line.is_empty() && !line.starts_with("*") && !line.starts_with("$"))
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    (tokens[0].clone(),tokens[1..].to_vec())
}
