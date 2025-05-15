use std::cmp::Ordering;

fn to_lower(c: u8) -> u8 {
    match c {
        b'A'..=b'Z' => c - b'A' + b'a',
        _ => c,
    }
}

pub fn strncasecmp(s1: &[u8], s2: &[u8], n: usize) -> i32 {
    if s1.as_ptr() == s2.as_ptr() || n == 0 {
        return 0;
    }

    let mut iter1 = s1.iter();
    let mut iter2 = s2.iter();
    let mut remaining = n;
    let mut result = 0i32;

    while remaining > 0 {
        let c1 = iter1.next().map(|&c| to_lower(c)).unwrap_or(0);
        let c2 = iter2.next().map(|&c| to_lower(c)).unwrap_or(0);
        
        remaining -= 1;
        
        if c1 == 0 || c2 == 0 || c1 != c2 {
            result = match c1.cmp(&c2) {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            };
            break;
        }
    }

    result
}