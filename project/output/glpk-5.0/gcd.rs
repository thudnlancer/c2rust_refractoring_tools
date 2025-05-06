#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_gcd(mut x: i32, mut y: i32) -> i32 {
    let mut r: i32 = 0;
    (x > 0 as i32 && y > 0 as i32
        || {
            glp_assert_(
                b"x > 0 && y > 0\0" as *const u8 as *const i8,
                b"misc/gcd.c\0" as *const u8 as *const i8,
                52 as i32,
            );
            1 as i32 != 0
        }) as i32;
    while y > 0 as i32 {
        r = x % y;
        x = y;
        y = r;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_gcdn(mut n: i32, mut x: *mut i32) -> i32 {
    let mut d: i32 = 0;
    let mut j: i32 = 0;
    (n > 0 as i32
        || {
            glp_assert_(
                b"n > 0\0" as *const u8 as *const i8,
                b"misc/gcd.c\0" as *const u8 as *const i8,
                87 as i32,
            );
            1 as i32 != 0
        }) as i32;
    j = 1 as i32;
    while j <= n {
        (*x.offset(j as isize) > 0 as i32
            || {
                glp_assert_(
                    b"x[j] > 0\0" as *const u8 as *const i8,
                    b"misc/gcd.c\0" as *const u8 as *const i8,
                    89 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if j == 1 as i32 {
            d = *x.offset(1 as i32 as isize);
        } else {
            d = _glp_gcd(d, *x.offset(j as isize));
        }
        if d == 1 as i32 {
            break;
        }
        j += 1;
        j;
    }
    return d;
}