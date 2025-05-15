use std::cmp::Ordering;
use std::mem;
use std::ptr;
use std::slice;
use std::str;

#[derive(Debug, Clone, Copy)]
struct MbChar {
    ptr: *const u8,
    bytes: usize,
    wc_valid: bool,
    wc: i32,
    buf: [u8; 24],
}

#[derive(Debug, Clone, Copy)]
struct MbuIterMulti {
    in_shift: bool,
    state: MbState,
    next_done: bool,
    cur: MbChar,
}

#[derive(Debug, Clone, Copy)]
struct MbState {
    count: i32,
    value: MbStateValue,
}

#[derive(Debug, Clone, Copy)]
union MbStateValue {
    wch: u32,
    wchb: [u8; 4],
}

fn mbscasestr(haystack: &str, needle: &str) -> Option<&str> {
    if needle.is_empty() {
        return Some(haystack);
    }

    let mut haystack_chars = haystack.chars().flat_map(|c| c.to_lowercase());
    let needle_first = needle.chars().next().unwrap().to_lowercase().next().unwrap();

    for (i, c) in haystack.chars().enumerate() {
        if c.to_lowercase().next().unwrap() == needle_first {
            let mut haystack_iter = haystack[i..].chars().flat_map(|c| c.to_lowercase());
            let mut needle_iter = needle.chars().flat_map(|c| c.to_lowercase());

            let mut matched = true;
            for (h, n) in haystack_iter.by_ref().zip(needle_iter.by_ref()) {
                if h != n {
                    matched = false;
                    break;
                }
            }

            if matched && needle_iter.next().is_none() {
                return Some(&haystack[i..]);
            }
        }
    }

    None
}

fn knuth_morris_pratt(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let m = needle.len();
    if m == 0 {
        return Some(0);
    }

    let mut table = vec![0; m + 1];
    let mut i = 2;
    let mut j = 0;
    table[1] = 1;

    while i < m {
        let b = needle[i - 1];
        loop {
            if b == needle[j] {
                j += 1;
                table[i] = i - j;
                break;
            } else if j == 0 {
                table[i] = i;
                break;
            } else {
                j -= table[j];
            }
        }
        i += 1;
    }

    let mut j = 0;
    let mut rhaystack = haystack;
    let mut phaystack = haystack;

    while !phaystack.is_empty() {
        if needle[j] == phaystack[0] {
            j += 1;
            phaystack = &phaystack[1..];
            if j == m {
                return Some(rhaystack.len() - haystack.len());
            }
        } else if j > 0 {
            rhaystack = &rhaystack[table[j]..];
            j -= table[j];
        } else {
            rhaystack = &rhaystack[1..];
            phaystack = &phaystack[1..];
        }
    }

    None
}

fn knuth_morris_pratt_multibyte(haystack: &str, needle: &str) -> Option<usize> {
    let needle_chars: Vec<char> = needle.chars().collect();
    let m = needle_chars.len();
    if m == 0 {
        return Some(0);
    }

    let mut table = vec![0; m + 1];
    let mut i = 2;
    let mut j = 0;
    table[1] = 1;

    while i < m {
        let b = needle_chars[i - 1];
        loop {
            if b == needle_chars[j] {
                j += 1;
                table[i] = i - j;
                break;
            } else if j == 0 {
                table[i] = i;
                break;
            } else {
                j -= table[j];
            }
        }
        i += 1;
    }

    let mut j = 0;
    let mut haystack_chars = haystack.chars();
    let mut rhaystack_pos = 0;
    let mut current_pos = 0;

    while let Some(c) = haystack_chars.next() {
        if needle_chars[j].to_lowercase().eq(c.to_lowercase()) {
            j += 1;
            current_pos += c.len_utf8();
            if j == m {
                return Some(rhaystack_pos);
            }
        } else if j > 0 {
            let skip = table[j];
            for _ in 0..skip {
                if let Some(c) = haystack[rhaystack_pos..].chars().next() {
                    rhaystack_pos += c.len_utf8();
                }
            }
            j -= skip;
        } else {
            rhaystack_pos += c.len_utf8();
            current_pos = rhaystack_pos;
        }
    }

    None
}