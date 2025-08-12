pub fn parse_command(buf: &[u8], bytes_count: usize) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_quotes = false;

    for &byte in &buf[..bytes_count] {
        match byte {
            b'"' => {
                in_quotes = !in_quotes;
                if !in_quotes && !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            b' ' if !in_quotes => {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            _ => {
                current_token.push(byte as char);
            }
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}
