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
pub unsafe extern "C" fn nettle_cnd_memcpy(
    mut cnd: i32,
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
    mut n: size_t,
) {
    let mut sp: *const u8 = src as *const u8;
    let mut dp: *mut u8 = dst as *mut u8;
    let mut c: u8 = 0;
    let mut m: u8 = 0;
    let mut i: size_t = 0;
    ::core::ptr::write_volatile(&mut m as *mut u8, -(cnd as u8 as i32) as u8);
    i = 0 as i32 as size_t;
    while i < n {
        ::core::ptr::write_volatile(
            &mut c as *mut u8,
            (*sp.offset(i as isize) as i32 & m as i32) as u8,
        );
        ::core::ptr::write_volatile(
            &mut c as *mut u8,
            (::core::ptr::read_volatile::<u8>(&c as *const u8) as i32
                | *dp.offset(i as isize) as i32 & !(m as i32)) as u8 as u8,
        );
        ::core::ptr::write_volatile(dp.offset(i as isize), c);
        i = i.wrapping_add(1);
        i;
    }
}