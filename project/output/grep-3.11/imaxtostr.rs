#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __intmax_t = i64;
pub type intmax_t = __intmax_t;
#[no_mangle]
pub unsafe extern "C" fn imaxtostr(mut i: intmax_t, mut buf: *mut i8) -> *mut i8 {
    let mut p: *mut i8 = buf
        .offset(
            (::core::mem::size_of::<intmax_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub(
                    !((0 as i32 as intmax_t) < -(1 as i32) as intmax_t) as i32 as u64,
                )
                .wrapping_mul(146 as i32 as u64)
                .wrapping_add(484 as i32 as u64)
                .wrapping_div(485 as i32 as u64)
                .wrapping_add(
                    !((0 as i32 as intmax_t) < -(1 as i32) as intmax_t) as i32 as u64,
                ) as isize,
        );
    *p = 0 as i32 as i8;
    if i < 0 as i32 as i64 {
        loop {
            p = p.offset(-1);
            *p = ('0' as i32 as i64 - i % 10 as i32 as i64) as i8;
            i /= 10 as i32 as i64;
            if !(i != 0 as i32 as i64) {
                break;
            }
        }
        p = p.offset(-1);
        *p = '-' as i32 as i8;
    } else {
        loop {
            p = p.offset(-1);
            *p = ('0' as i32 as i64 + i % 10 as i32 as i64) as i8;
            i /= 10 as i32 as i64;
            if !(i != 0 as i32 as i64) {
                break;
            }
        }
    }
    return p;
}