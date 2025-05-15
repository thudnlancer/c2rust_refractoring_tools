pub fn to_lower(c: u8) -> u8 {
    match c {
        b'A'..=b'Z' => c - b'A' + b'a',
        _ => c,
    }
}

pub fn strncasecmp(s1: &[u8], s2: &[u8], n: usize) -> i32 {
    if s1.as_ptr() == s2.as_ptr() || n == 0 {
        return 0;
    }

    let mut p1 = s1.iter();
    let mut p2 = s2.iter();
    let mut remaining = n;
    let mut c1 = 0;
    let mut c2 = 0;

    loop {
        c1 = to_lower(*p1.next().unwrap_or(&0));
        c2 = to_lower(*p2.next().unwrap_or(&0));
        remaining -= 1;

        if remaining == 0 || c1 == 0 {
            break;
        }

        if c1 != c2 {
            break;
        }
    }

    if c1 > c2 {
        1
    } else if c1 < c2 {
        -1
    } else {
        0
    }
}