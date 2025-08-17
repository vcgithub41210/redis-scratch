pub fn parse_command(msg: &[u8], _len: usize) -> (String, Vec<String>) {
    let s = String::from_utf8_lossy(&msg[.._len]);
    let tokens = s.split("\r\n")
        .filter(|line| !line.is_empty() && !line.startes_with("*") && !line.starts_with("$"))
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    (tokens[0].clone(),tokens[1..].to_vec())
}
