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

    let s1_bytes = s1.to_bytes();
    let s2_bytes = s2.to_bytes();

    let mut i = 0;
    loop {
        let c1 = if i < s1_bytes.len() { s1_bytes[i] } else { 0 };
        let c2 = if i < s2_bytes.len() { s2_bytes[i] } else { 0 };

        let c1_lower = c_tolower(c1 as i32);
        let c2_lower = c_tolower(c2 as i32);

        if c1_lower == 0 || c1_lower != c2_lower {
            return if c1_lower < c2_lower {
                -1
            } else if c1_lower > c2_lower {
                1
            } else {
                0
            };
        }
        i += 1;
    }
}