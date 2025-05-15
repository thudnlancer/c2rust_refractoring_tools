pub fn c_tolower(c: u8) -> u8 {
    match c {
        b'A'..=b'Z' => c - b'A' + b'a',
        _ => c,
    }
}

pub fn c_strncasecmp(s1: &[u8], s2: &[u8], n: usize) -> i32 {
    if n == 0 {
        return 0;
    }

    let min_len = s1.len().min(s2.len()).min(n);
    for i in 0..min_len {
        let c1 = c_tolower(s1[i]);
        let c2 = c_tolower(s2[i]);
        if c1 != c2 {
            return c1 as i32 - c2 as i32;
        }
        if c1 == 0 {
            return 0;
        }
    }

    if min_len < n && s1.len() != s2.len() {
        if min_len == s1.len() {
            -(s2[min_len] as i32)
        } else {
            s1[min_len] as i32
        }
    } else {
        0
    }
}