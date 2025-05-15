pub fn c_strcasecmp(s1: &str, s2: &str) -> i32 {
    let mut p1 = s1.chars();
    let mut p2 = s2.chars();

    loop {
        let c1 = p1.next().map(|c| c.to_ascii_lowercase());
        let c2 = p2.next().map(|c| c.to_ascii_lowercase());

        match (c1, c2) {
            (None, None) => return 0,
            (None, _) => return -1,
            (_, None) => return 1,
            (Some(a), Some(b)) if a == b => continue,
            (Some(a), Some(b)) => return (a as i32) - (b as i32),
        }
    }
}

#[inline]
fn c_tolower(c: char) -> char {
    c.to_ascii_lowercase()
}