pub fn c_tolower(c: i32) -> i32 {
    match c {
        b'A'..=b'Z' => c - (b'A' as i32) + (b'a' as i32),
        _ => c,
    }
}

#[no_mangle]
pub extern "C" fn c_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int {
    if s1 == s2 {
        return 0;
    }

    let s1 = unsafe { std::ffi::CStr::from_ptr(s1) };
    let s2 = unsafe { std::ffi::CStr::from_ptr(s2) };

    let mut iter1 = s1.to_bytes().iter();
    let mut iter2 = s2.to_bytes().iter();

    loop {
        let c1 = iter1.next().map(|&c| c_tolower(c as i32) as i32;
        let c2 = iter2.next().map(|&c| c_tolower(c as i32) as i32);

        match (c1, c2) {
            (0, _) => break,
            (_, None) => return c1,
            (Some(c1), Some(c2)) if c1 != c2 => return c1 - c2,
            _ => continue,
        }
    }

    // Handle remaining characters in s2
    if let Some(c2) = iter2.next() {
        return -(c_tolower(*c2 as i32));
    }

    0
}