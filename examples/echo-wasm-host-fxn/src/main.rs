use std::io::{self, Read, Write};

fn main() {
    let mut request = String::new();
    if let Err(err) = io::stdin().read_to_string(&mut request) {
        write_error(&format!("failed to read request: {err}"));
        return;
    }

    match extract_args_json(&request) {
        Ok(args) => print!("{{\"result\":{}}}", args.trim()),
        Err(err) => write_error(err),
    }
}

fn extract_args_json(request: &str) -> Result<&str, &'static str> {
    let bytes = request.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i].is_ascii_whitespace() || bytes[i] == b'{' || bytes[i] == b',' {
            i += 1;
            continue;
        }

        if bytes[i] != b'"' {
            return Err("expected JSON object key");
        }

        let (key, next) = parse_string(request, i)?;
        i = skip_ws(bytes, next);
        if bytes.get(i) != Some(&b':') {
            return Err("expected ':' after JSON object key");
        }
        i = skip_ws(bytes, i + 1);

        let value_end = json_value_end(request, i)?;
        if key == "args" {
            return Ok(&request[i..value_end]);
        }
        i = value_end;
    }

    Err("missing args field")
}

fn parse_string(input: &str, start: usize) -> Result<(String, usize), &'static str> {
    let bytes = input.as_bytes();
    if bytes.get(start) != Some(&b'"') {
        return Err("expected JSON string");
    }

    let mut out = String::new();
    let mut i = start + 1;
    while i < bytes.len() {
        match bytes[i] {
            b'"' => return Ok((out, i + 1)),
            b'\\' => {
                i += 1;
                if i >= bytes.len() {
                    return Err("unterminated escape sequence");
                }
                match bytes[i] {
                    b'"' => out.push('"'),
                    b'\\' => out.push('\\'),
                    b'/' => out.push('/'),
                    b'b' => out.push('\u{0008}'),
                    b'f' => out.push('\u{000c}'),
                    b'n' => out.push('\n'),
                    b'r' => out.push('\r'),
                    b't' => out.push('\t'),
                    b'u' => return Err("unicode escapes are not supported in object keys"),
                    _ => return Err("invalid escape sequence"),
                }
            }
            byte => out.push(byte as char),
        }
        i += 1;
    }

    Err("unterminated JSON string")
}

fn json_value_end(input: &str, start: usize) -> Result<usize, &'static str> {
    let bytes = input.as_bytes();
    if start >= bytes.len() {
        return Err("missing JSON value");
    }

    let mut i = start;
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escape = false;

    while i < bytes.len() {
        let byte = bytes[i];
        if in_string {
            if escape {
                escape = false;
            } else if byte == b'\\' {
                escape = true;
            } else if byte == b'"' {
                in_string = false;
            }
            i += 1;
            continue;
        }

        match byte {
            b'"' => in_string = true,
            b'{' | b'[' => depth += 1,
            b'}' | b']' => {
                if depth == 0 {
                    return Ok(i);
                }
                depth -= 1;
                if depth == 0 {
                    return Ok(i + 1);
                }
            }
            b',' if depth == 0 => return Ok(i),
            byte if byte.is_ascii_whitespace() && depth == 0 => return Ok(i),
            _ => {}
        }
        i += 1;
    }

    if in_string || depth != 0 {
        Err("unterminated JSON value")
    } else {
        Ok(i)
    }
}

fn skip_ws(bytes: &[u8], mut i: usize) -> usize {
    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }
    i
}

fn write_error(message: &str) {
    let escaped = message.replace('\\', "\\\\").replace('"', "\\\"");
    let _ = write!(io::stdout(), "{{\"error\":\"{escaped}\"}}");
}

#[cfg(test)]
mod tests {
    use super::extract_args_json;

    #[test]
    fn extracts_args_object() {
        let request = r#"{"name":"echo","args":{"message":"hello"}}"#;
        assert_eq!(
            extract_args_json(request).unwrap(),
            r#"{"message":"hello"}"#
        );
    }

    #[test]
    fn extracts_args_when_first() {
        let request = r#"{"args":["hello",{"nested":true}],"name":"echo"}"#;
        assert_eq!(
            extract_args_json(request).unwrap(),
            r#"["hello",{"nested":true}]"#
        );
    }
}
