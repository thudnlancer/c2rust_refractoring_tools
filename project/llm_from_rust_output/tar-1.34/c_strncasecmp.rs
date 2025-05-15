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

    let min_len = s1.len().min(s2.len()).min(n);
    let s1 = &s1[..min_len];
    let s2 = &s2[..min_len];

    for (i, (&c1, &c2)) in s1.iter().zip(s2).enumerate() {
        let lower_c1 = c_tolower(c1);
        let lower_c2 = c_tolower(c2);

        if lower_c1 != lower_c2 {
            return (lower_c1 as i32) - (lower_c2 as i32);
        }

        if lower_c1 == 0 {
            break;
        }
    }

    if s1.len() < s2.len() && s1.len() < n {
        -(s2[s1.len()] as i32)
    } else if s2.len() < s1.len() && s2.len() < n {
        s1[s2.len()] as i32
    } else {
        0
    }
}