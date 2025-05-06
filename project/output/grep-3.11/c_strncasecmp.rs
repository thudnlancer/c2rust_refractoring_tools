#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type size_t = u64;
#[inline]
unsafe extern "C" fn c_tolower(mut c: i32) -> i32 {
    match c {
        65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80
        | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => {
            return c - 'A' as i32 + 'a' as i32;
        }
        _ => return c,
    };
}
#[no_mangle]
pub unsafe extern "C" fn c_strncasecmp(
    mut s1: *const i8,
    mut s2: *const i8,
    mut n: size_t,
) -> i32 {
    let mut p1: *const u8 = s1 as *const u8;
    let mut p2: *const u8 = s2 as *const u8;
    let mut c1: u8 = 0;
    let mut c2: u8 = 0;
    if p1 == p2 || n == 0 as i32 as u64 {
        return 0 as i32;
    }
    loop {
        c1 = c_tolower(*p1 as i32) as u8;
        c2 = c_tolower(*p2 as i32) as u8;
        n = n.wrapping_sub(1);
        if n == 0 as i32 as u64 || c1 as i32 == '\0' as i32 {
            break;
        }
        p1 = p1.offset(1);
        p1;
        p2 = p2.offset(1);
        p2;
        if !(c1 as i32 == c2 as i32) {
            break;
        }
    }
    if 127 as i32 * 2 as i32 + 1 as i32 <= 2147483647 as i32 {
        return c1 as i32 - c2 as i32
    } else {
        return (c1 as i32 > c2 as i32) as i32 - ((c1 as i32) < c2 as i32) as i32
    };
}