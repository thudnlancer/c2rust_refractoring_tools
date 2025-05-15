pub fn c_tolower(c: i32) -> i32 {
    match c {
        b'A'..=b'Z' => c - b'A' as i32 + b'a' as i32,
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
    let mut result = 0;

    while remaining > 0 {
        let c1 = iter1.next().map(|&c| c_tolower(c as i32) as i32;
        let c2 = iter2.next().map(|&c| c_tolower(c as i32) as i32);

        remaining -= 1;

        match (c1, c2) {
            (0, _) => break,
            (_, None) => {
                result = c1;
                break;
            }
            (Some(c1), Some(c2)) if c1 != c2 => {
                result = c1 - c2;
                break;
            }
            _ => continue,
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