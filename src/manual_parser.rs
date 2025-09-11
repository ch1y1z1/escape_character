#[inline(never)]
pub fn parse(input: &str) -> Result<String, ()> {
    let mut last_is_escape = false;
    input
        .chars()
        .try_fold(String::with_capacity(input.len()), |mut out, ch| {
            if !last_is_escape {
                if ch == '\\' {
                    last_is_escape = true;
                } else {
                    out.push(ch);
                }
                return Ok(out);
            }
            last_is_escape = false;

            let decoded = match ch {
                '\\' => '\\',
                't' => '\t',
                'n' => '\n',
                'r' => '\r',
                '0' => '\0',
                '"' => '"',
                '\'' => '\'',
                'b' => '\x08',
                'f' => '\x0C',
                'v' => '\x0B',
                'a' => '\x07',
                _ => return Err(()),
            };
            out.push(decoded);
            Ok(out)
        })
}

#[test]
fn test_parse() {
    let s = parse(r#"hello\\\n\tniho\r\"\'\0\bxl\a\n123\b2"#).unwrap();
    println!("{}", s);
}
