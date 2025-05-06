#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#[no_mangle]
pub unsafe extern "C" fn uinttostr(mut i: u32, mut buf: *mut i8) -> *mut i8 {
    let mut p: *mut i8 = buf
        .offset(
            (::core::mem::size_of::<u32>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub(!((0 as i32 as u32) < -(1 as i32) as u32) as i32 as u64)
                .wrapping_mul(146 as i32 as u64)
                .wrapping_add(484 as i32 as u64)
                .wrapping_div(485 as i32 as u64)
                .wrapping_add(!((0 as i32 as u32) < -(1 as i32) as u32) as i32 as u64)
                as isize,
        );
    *p = 0 as i32 as i8;
    if i < 0 as i32 as u32 {
        loop {
            p = p.offset(-1);
            *p = ('0' as i32 as u32).wrapping_sub(i.wrapping_rem(10 as i32 as u32))
                as i8;
            i = i.wrapping_div(10 as i32 as u32);
            if !(i != 0 as i32 as u32) {
                break;
            }
        }
        p = p.offset(-1);
        *p = '-' as i32 as i8;
    } else {
        loop {
            p = p.offset(-1);
            *p = ('0' as i32 as u32).wrapping_add(i.wrapping_rem(10 as i32 as u32))
                as i8;
            i = i.wrapping_div(10 as i32 as u32);
            if !(i != 0 as i32 as u32) {
                break;
            }
        }
    }
    return p;
}