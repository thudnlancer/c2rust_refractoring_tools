use std::fmt::Write;

pub type intmax_t = i64;

pub fn imaxtostr(i: intmax_t) -> String {
    let mut buf = String::new();
    if i < 0 {
        write!(&mut buf, "{}", i).unwrap();
    } else {
        write!(&mut buf, "{}", i).unwrap();
    }
    buf
}