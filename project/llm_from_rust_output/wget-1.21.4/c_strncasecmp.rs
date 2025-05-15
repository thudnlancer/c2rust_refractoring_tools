pub fn c_tolower(c: u8) -> u8 {
    match c {
        b'A'..=b'Z' => c - b'A' + b'a',
        _ => c,
    }
}

pub fn c_strncasecmp(s1: &[u8], s2: &[u8], n: usize) -> i32 {
    if s1.as_ptr() == s2.as_ptr() || n == 0 {
        return 0;
    }

    let mut iter1 = s1.iter();
    let mut iter2 = s2.iter();
    let mut remaining = n;
    let mut result = 0i32;

    while remaining > 0 {
        let c1 = iter1.next().map(|&c| c_tolower(c)).unwrap_or(0);
        let c2 = iter2.next().map(|&c| c_tolower(c)).unwrap_or(0);

        remaining -= 1;
        if remaining == 0 || c1 == 0 {
            result = c1 as i32 - c2 as i32;
            break;
        }

        if c1 != c2 {
            result = c1 as i32 - c2 as i32;
            break;
        }
    }

    if i32::MAX >= 255 {
        result
    } else {
        match result {
            x if x > 0 => 1,
            x if x < 0 => -1,
            _ => 0,
        }
    }
}