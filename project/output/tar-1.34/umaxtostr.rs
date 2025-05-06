#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __uintmax_t = u64;
pub type uintmax_t = __uintmax_t;
#[no_mangle]
pub unsafe extern "C" fn umaxtostr(mut i: uintmax_t, mut buf: *mut i8) -> *mut i8 {
    let mut p: *mut i8 = buf
        .offset(
            (::core::mem::size_of::<uintmax_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub(
                    !((0 as i32 as uintmax_t) < -(1 as i32) as uintmax_t) as i32 as u64,
                )
                .wrapping_mul(146 as i32 as u64)
                .wrapping_add(484 as i32 as u64)
                .wrapping_div(485 as i32 as u64)
                .wrapping_add(
                    !((0 as i32 as uintmax_t) < -(1 as i32) as uintmax_t) as i32 as u64,
                ) as isize,
        );
    *p = 0 as i32 as i8;
    if i < 0 as i32 as u64 {
        loop {
            p = p.offset(-1);
            *p = ('0' as i32 as u64).wrapping_sub(i.wrapping_rem(10 as i32 as u64))
                as i8;
            i = (i as u64).wrapping_div(10 as i32 as u64) as uintmax_t as uintmax_t;
            if !(i != 0 as i32 as u64) {
                break;
            }
        }
        p = p.offset(-1);
        *p = '-' as i32 as i8;
    } else {
        loop {
            p = p.offset(-1);
            *p = ('0' as i32 as u64).wrapping_add(i.wrapping_rem(10 as i32 as u64))
                as i8;
            i = (i as u64).wrapping_div(10 as i32 as u64) as uintmax_t as uintmax_t;
            if !(i != 0 as i32 as u64) {
                break;
            }
        }
    }
    return p;
}