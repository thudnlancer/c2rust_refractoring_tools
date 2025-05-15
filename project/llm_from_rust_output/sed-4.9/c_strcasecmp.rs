fn c_tolower(c: u8) -> u8 {
    match c {
        b'A'..=b'Z' => c - b'A' + b'a',
        _ => c,
    }
}

#[no_mangle]
pub extern "C" fn c_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int {
    if s1 == s2 {
        return 0;
    }

    let mut p1 = s1;
    let mut p2 = s2;

    loop {
        let c1 = unsafe { *p1 };
        let c2 = unsafe { *p2 };

        let c1_lower = c_tolower(c1 as u8);
        let c2_lower = c_tolower(c2 as u8);

        if c1_lower == 0 {
            break;
        }

        if c1_lower != c2_lower {
            break;
        }

        unsafe {
            p1 = p1.add(1);
            p2 = p2.add(1);
        }
    }

    let c1 = unsafe { *p1 } as u8;
    let c2 = unsafe { *p2 } as u8;
    let c1_lower = c_tolower(c1);
    let c2_lower = c_tolower(c2);

    if 127 * 2 + 1 <= i32::MAX {
        (c1_lower as i32) - (c2_lower as i32)
    } else {
        (c1_lower > c2_lower) as i32 - (c1_lower < c2_lower) as i32
    }
}