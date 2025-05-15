use std::char;
use std::str;

/// Convert a possibly-signed character to an unsigned character.
/// This is a bit safer than casting to unsigned char, since it catches some type
/// errors that the cast doesn't.
fn to_uchar(ch: u8) -> u8 {
    ch
}

/// Scan a unibyte buffer, replacing any non-printable characters with question marks.
fn unibyte_qmark_chars(buf: &mut [u8]) -> usize {
    for p in buf.iter_mut() {
        if !(*p as char).is_ascii_graphic() && *p != b' ' {
            *p = b'?';
        }
    }
    buf.len()
}

/// Scan a buffer, replacing any dangerous-looking characters with question marks.
/// This function may shrink the buffer. Either way, the new length is returned.
pub fn qmark_chars(buf: &mut [u8]) -> usize {
    if std::env::consts::OS == "windows" || unsafe { libc::MB_CUR_MAX } <= 1 {
        return unibyte_qmark_chars(buf);
    }

    let mut result = Vec::with_capacity(buf.len());
    let mut input = buf;
    
    while !input.is_empty() {
        let printable_ascii = input.iter().position(|&c| {
            !matches!(c, 
                b' ' | b'!' | b'"' | b'#' | b'%' |
                b'&' | b'\'' | b'(' | b')' | b'*' |
                b'+' | b',' | b'-' | b'.' | b'/' |
                b'0'..=b'9' |
                b':' | b';' | b'<' | b'=' | b'>' |
                b'?' |
                b'A'..=b'Z' |
                b'[' | b'\\' | b']' | b'^' | b'_' |
                b'a'..=b'z' | b'{' | b'|' | b'}' | b'~'
            )
        }).unwrap_or(input.len());

        result.extend_from_slice(&input[..printable_ascii]);
        input = &input[printable_ascii..];

        if input.is_empty() {
            break;
        }

        match std::str::from_utf8(input) {
            Ok(s) => {
                match s.chars().next() {
                    Some(c) => {
                        let char_len = c.len_utf8();
                        if c.is_control() || c == '\u{FFFD}' {
                            result.push(b'?');
                        } else {
                            result.extend_from_slice(&input[..char_len]);
                        }
                        input = &input[char_len..];
                    }
                    None => {
                        result.push(b'?');
                        input = &input[1..];
                    }
                }
            }
            Err(e) => {
                let valid_up_to = e.valid_up_to();
                if valid_up_to > 0 {
                    result.extend_from_slice(&input[..valid_up_to]);
                    input = &input[valid_up_to..];
                } else {
                    result.push(b'?');
                    input = &input[1..];
                }
            }
        }
    }

    buf[..result.len()].copy_from_slice(&result);
    result.len()
}