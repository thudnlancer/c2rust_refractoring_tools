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
#[no_mangle]
pub unsafe extern "C" fn hash_pjw(
    mut x: *const libc::c_void,
    mut tablesize: size_t,
) -> size_t {
    let mut s: *const i8 = 0 as *const i8;
    let mut h: size_t = 0 as i32 as size_t;
    s = x as *const i8;
    while *s != 0 {
        h = (*s as u64)
            .wrapping_add(
                h << 9 as i32
                    | h
                        >> (::core::mem::size_of::<size_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(9 as i32 as u64),
            );
        s = s.offset(1);
        s;
    }
    return h.wrapping_rem(tablesize);
}