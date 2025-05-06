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
pub type __uint32_t = u32;
pub type uint32_t = __uint32_t;
#[no_mangle]
pub unsafe extern "C" fn nettle_memeql_sec(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
    mut n: size_t,
) -> i32 {
    let mut ap: *const u8 = a as *const u8;
    let mut bp: *const u8 = b as *const u8;
    let mut diff: u8 = 0;
    let mut i: size_t = 0;
    ::core::ptr::write_volatile(&mut diff as *mut u8, 0 as i32 as u8);
    i = ::core::ptr::read_volatile::<u8>(&diff as *const u8) as size_t;
    while i < n {
        ::core::ptr::write_volatile(
            &mut diff as *mut u8,
            (::core::ptr::read_volatile::<u8>(&diff as *const u8) as i32
                | *ap.offset(i as isize) as i32 ^ *bp.offset(i as isize) as i32) as u8
                as u8,
        );
        i = i.wrapping_add(1);
        i;
    }
    return ((diff as uint32_t).wrapping_sub(1 as u32) >> 31 as i32) as i32;
}