pub fn c_strcasecmp(s1: &str, s2: &str) -> i32 {
    let mut s1_chars = s1.chars().map(|c| c.to_ascii_lowercase());
    let mut s2_chars = s2.chars().map(|c| c.to_ascii_lowercase());

    loop {
        let c1 = s1_chars.next();
        let c2 = s2_chars.next();

        match (c1, c2) {
            (None, None) => return 0,
            (None, _) => return -1,
            (_, None) => return 1,
            (Some(a), Some(b)) => {
                if a != b {
                    return a as i32 - b as i32;
                }
            }
        }
    }
}

fn c_tolower(c: char) -> char {
    c.to_ascii_lowercase()
}