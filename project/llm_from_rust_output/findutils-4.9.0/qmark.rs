use std::char;
use std::mem;
use unicode_width::UnicodeWidthChar;

const PRINTABLE_ASCII: &[u8] = b" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

fn unibyte_qmark_chars(buf: &mut [u8]) -> usize {
    for byte in buf.iter_mut() {
        if !byte.is_ascii_graphic() && !byte.is_ascii_whitespace() {
            *byte = b'?';
        }
    }
    buf.len()
}

pub fn qmark_chars(buf: &mut [u8]) -> usize {
    if std::mem::size_of::<char>() == 1 {
        return unibyte_qmark_chars(buf);
    }

    let mut result = Vec::with_capacity(buf.len());
    let mut input = buf;
    
    while !input.is_empty() {
        let first = input[0];
        if PRINTABLE_ASCII.contains(&first) {
            result.push(first);
            input = &mut input[1..];
            continue;
        }

        match std::str::from_utf8(input) {
            Ok(s) => {
                let mut chars = s.chars();
                if let Some(c) = chars.next() {
                    if c.width() > 0 {
                        let bytes = c.len_utf8();
                        result.extend_from_slice(&input[..bytes]);
                        input = &mut input[bytes..];
                    } else {
                        result.push(b'?');
                        input = &mut input[c.len_utf8()..];
                    }
                }
            }
            Err(e) => {
                let valid_up_to = e.valid_up_to();
                if valid_up_to > 0 {
                    result.extend_from_slice(&input[..valid_up_to]);
                    input = &mut input[valid_up_to..];
                } else {
                    result.push(b'?');
                    input = &mut input[1..];
                }
            }
        }
    }

    buf[..result.len()].copy_from_slice(&result);
    result.len()
}