use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use std::slice;
use std::str;

const _ISalnum: u16 = 8;
const _ISpunct: u16 = 4;
const _IScntrl: u16 = 2;
const _ISblank: u16 = 1;
const _ISgraph: u16 = 32768;
const _ISprint: u16 = 16384;
const _ISspace: u16 = 8192;
const _ISxdigit: u16 = 4096;
const _ISdigit: u16 = 2048;
const _ISalpha: u16 = 1024;
const _ISlower: u16 = 512;
const _ISupper: u16 = 256;

static QUOTE_TRANSTAB: &[u8] = b"\\\\a\x07b\x08f\x0Cn\nr\rt\t\0";

fn toupper(c: c_int) -> c_int {
    if c >= -128 && c < 256 {
        c.to_ascii_uppercase() as c_int
    } else {
        c
    }
}

fn argcv_scan(
    len: c_int,
    command: &[u8],
    delim: &[u8],
    cmnt: Option<&[u8]>,
    start: &mut c_int,
    end: &mut c_int,
    save: &mut c_int,
) -> c_int {
    let mut i = *save;
    loop {
        if i >= len {
            return i + 1;
        }

        while i < len && [b' ', b'\t', b'\n'].contains(&command[i as usize]) {
            i += 1;
        }

        *start = i;
        if i >= len || !delim.contains(&command[i as usize]) {
            while i < len {
                if command[i as usize] == b'\\' {
                    i += 1;
                    if i == len {
                        break;
                    }
                    i += 1;
                } else if [b'\'', b'"'].contains(&command[i as usize]) {
                    let quote = command[i as usize];
                    let mut j = i + 1;
                    while j < len && command[j as usize] != quote {
                        if command[j as usize] == b'\\' {
                            j += 1;
                        }
                        j += 1;
                    }
                    if j < len {
                        i = j + 1;
                    } else {
                        i += 1;
                    }
                } else {
                    if [b' ', b'\t', b'\n'].contains(&command[i as usize])
                        || delim.contains(&command[i as usize])
                    {
                        break;
                    }
                    i += 1;
                }
            }
            i -= 1;
        }

        *end = i;
        *save = i + 1;
        if *save > len {
            break;
        }

        if let Some(cmnt) = cmnt {
            if !cmnt.contains(&command[*start as usize]) {
                break;
            }
        } else {
            break;
        }

        i = *save;
        while i < len && command[i as usize] != b'\n' {
            i += 1;
        }
        *save = i;
    }
    *save
}

fn argcv_unquote_char(c: c_int) -> c_int {
    let mut i = 0;
    while i < QUOTE_TRANSTAB.len() - 1 {
        if QUOTE_TRANSTAB[i] as c_int == c {
            return QUOTE_TRANSTAB[i + 1] as c_int;
        }
        i += 2;
    }
    c
}

fn argcv_quote_char(c: c_int) -> c_int {
    let mut i = QUOTE_TRANSTAB.len() - 2;
    while i > 0 {
        if QUOTE_TRANSTAB[i] as c_int == c {
            return QUOTE_TRANSTAB[i - 1] as c_int;
        }
        i -= 2;
    }
    -1
}

fn xtonum(src: &[u8], base: c_int, cnt: c_int) -> (c_int, c_int) {
    let mut i = 0;
    let mut val = 0;
    while i < cnt as usize && i < src.len() {
        let mut n = src[i] as c_int;
        if n > 127 {
            break;
        }

        n = if n.is_ascii_digit() {
            n - b'0' as c_int
        } else if n.is_ascii_hexdigit() {
            let upper = n.to_ascii_uppercase();
            upper - b'A' as c_int + 10
        } else {
            255
        };

        if n >= base {
            break;
        }

        val = val * base + n;
        i += 1;
    }
    (val, i as c_int)
}

fn argcv_quoted_length(s: &[u8], quote: &mut c_int) -> usize {
    let mut len = 0;
    *quote = 0;
    for &c in s {
        if c == b' ' {
            len += 1;
            *quote = 1;
        } else if c == b'"' || c == b'\'' {
            len += 2;
            *quote = 1;
        } else if c != b'\t' && c != b'\\' && c.is_ascii_graphic() {
            len += 1;
        } else if argcv_quote_char(c as c_int) != -1 {
            len += 2;
        } else {
            len += 4;
        }
    }
    len
}

fn argcv_unquote_copy(dst: &mut [u8], src: &[u8]) {
    let mut i = 0;
    let mut dst_pos = 0;
    let mut expect_delim = 0;

    while i < src.len() {
        match src[i] {
            b'\'' | b'"' => {
                if expect_delim == 0 {
                    let quote = src[i];
                    let mut j = i + 1;
                    while j < src.len() && src[j] != quote {
                        if src[j] == b'\\' {
                            j += 1;
                        }
                        j += 1;
                    }
                    if j < src.len() {
                        i += 1;
                        expect_delim = quote as c_int;
                    } else {
                        dst[dst_pos] = src[i];
                        dst_pos += 1;
                        i += 1;
                    }
                } else if expect_delim == src[i] as c_int {
                    i += 1;
                } else {
                    dst[dst_pos] = src[i];
                    dst_pos += 1;
                    i += 1;
                }
            }
            b'\\' => {
                i += 1;
                if i >= src.len() {
                    dst[dst_pos] = b'\\';
                    dst_pos += 1;
                    break;
                }

                if src[i] == b'x' || src[i] == b'X' {
                    if src.len() - i < 2 {
                        dst[dst_pos] = b'\\';
                        dst_pos += 1;
                        dst[dst_pos] = src[i];
                        dst_pos += 1;
                        i += 1;
                    } else {
                        let (c, off) = xtonum(&src[i + 1..], 16, 2);
                        if off == 0 {
                            dst[dst_pos] = b'\\';
                            dst_pos += 1;
                            dst[dst_pos] = src[i];
                            dst_pos += 1;
                            i += 1;
                        } else {
                            dst[dst_pos] = c as u8;
                            dst_pos += 1;
                            i += off as usize + 1;
                        }
                    }
                } else if src[i].is_ascii_digit() {
                    if src.len() - i < 1 {
                        dst[dst_pos] = b'\\';
                        dst_pos += 1;
                        dst[dst_pos] = src[i];
                        dst_pos += 1;
                        i += 1;
                    } else {
                        let (c, off) = xtonum(&src[i..], 8, 3);
                        if off == 0 {
                            dst[dst_pos] = b'\\';
                            dst_pos += 1;
                            dst[dst_pos] = src[i];
                            dst_pos += 1;
                            i += 1;
                        } else {
                            dst[dst_pos] = c as u8;
                            dst_pos += 1;
                            i += off as usize;
                        }
                    }
                } else {
                    dst[dst_pos] = argcv_unquote_char(src[i] as c_int) as u8;
                    dst_pos += 1;
                    i += 1;
                }
            }
            _ => {
                dst[dst_pos] = src[i];
                dst_pos += 1;
                i += 1;
            }
        }
    }
}

fn argcv_quote_copy(dst: &mut [u8], src: &[u8]) {
    let mut pos = 0;
    for &c in src {
        if c == b'"' || c == b'\'' {
            dst[pos] = b'\\';
            pos += 1;
            dst[pos] = c;
            pos += 1;
        } else if c != b'\t' && c != b'\\' && c.is_ascii_graphic() {
            dst[pos] = c;
            pos += 1;
        } else {
            let quote = argcv_quote_char(c as c_int);
            dst[pos] = b'\\';
            pos += 1;
            if quote != -1 {
                dst[pos] = quote as u8;
                pos += 1;
            } else {
                let s = format!("{:03o}", c);
                dst[pos..pos + 3].copy_from_slice(s.as_bytes());
                pos += 3;
            }
        }
    }
}

pub fn argcv_get(
    command: &str,
    delim: &str,
    cmnt: Option<&str>,
    argc: &mut c_int,
    argv: &mut Vec<*mut c_char>,
) -> c_int {
    let command_bytes = command.as_bytes();
    let delim_bytes = delim.as_bytes();
    let cmnt_bytes = cmnt.map(|s| s.as_bytes());

    let len = command_bytes.len() as c_int;
    let mut start = 0;
    let mut end = 0;
    let mut save = 0;

    *argc = 0;
    argv.clear();

    while argcv_scan(len, command_bytes, delim_bytes, cmnt_bytes, &mut start, &mut end, &mut save) <= len {
        *argc += 1;
    }

    argv.reserve(*argc as usize + 1);

    save = 0;
    for _ in 0..*argc {
        argcv_scan(len, command_bytes, delim_bytes, cmnt_bytes, &mut start, &mut end, &mut save);

        let mut n = end - start + 1;
        let mut quoted = false;
        if (command_bytes[start as usize] == b'"' || command_bytes[start as usize] == b'\'')
            && command_bytes[end as usize] == command_bytes[start as usize]
        {
            start += 1;
            end -= 1;
            n -= 2;
            quoted = true;
        }

        let mut buf = vec![0u8; n as usize + 1];
        argcv_unquote_copy(&mut buf, &command_bytes[start as usize..=end as usize]);
        buf[n as usize] = 0;

        let cstr = CString::new(buf).unwrap();
        argv.push(cstr.into_raw());
    }

    argv.push(ptr::null_mut());
    0
}

pub fn argcv_free(argc: c_int, argv: &mut Vec<*mut c_char>) -> c_int {
    for i in 0..argc {
        unsafe {
            if !argv[i as usize].is_null() {
                let _ = CString::from_raw(argv[i as usize]);
            }
        }
    }
    argv.clear();
    0
}

pub fn argcv_string(argc: c_int, argv: &[*mut c_char], pstring: &mut *mut c_char) -> c_int {
    if pstring.is_null() {
        return 22;
    }

    let mut len = 0;
    let mut buffer = Vec::new();

    for i in 0..argc as usize {
        let mut quote = 0;
        let s = unsafe { CStr::from_ptr(argv[i]) }.to_bytes();
        let toklen = argcv_quoted_length(s, &mut quote) as c_int;

        len += toklen + 2;
        if quote != 0 {
            len += 2;
        }

        buffer.reserve(len as usize);
        if i > 0 {
            buffer.push(b' ');
        }

        if quote != 0 {
            buffer.push(b'"');
        }

        argcv_quote_copy(&mut buffer, s);

        if quote != 0 {
            buffer.push(b'"');
        }
    }

    while !buffer.is_empty() && buffer.last().unwrap().is_ascii_whitespace() {
        buffer.pop();
    }

    buffer.push(0);

    let cstring = CString::new(buffer).unwrap();
    *pstring = cstring.into_raw();

    0
}