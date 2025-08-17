pub fn parse_command(msg: &[u8], _len: usize) -> Vec<String> {
    let s = String::from_utf8_lossy(&msg[.._len]);
    s.split("\r\n")
        .filter(|line| !line.is_empty() && !line.starts_with("*") && !line.starts_with("$"))
        .map(|line| line.to_string())
        .collect()
}
