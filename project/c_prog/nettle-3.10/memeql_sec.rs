use ::libc;
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
#[no_mangle]
pub unsafe extern "C" fn nettle_memeql_sec(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
    mut n: size_t,
) -> libc::c_int {
    let mut ap: *const libc::c_uchar = a as *const libc::c_uchar;
    let mut bp: *const libc::c_uchar = b as *const libc::c_uchar;
    let mut diff: libc::c_uchar = 0;
    let mut i: size_t = 0;
    ::core::ptr::write_volatile(
        &mut diff as *mut libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    );
    i = ::core::ptr::read_volatile::<libc::c_uchar>(&diff as *const libc::c_uchar)
        as size_t;
    while i < n {
        ::core::ptr::write_volatile(
            &mut diff as *mut libc::c_uchar,
            (::core::ptr::read_volatile::<libc::c_uchar>(&diff as *const libc::c_uchar)
                as libc::c_int
                | *ap.offset(i as isize) as libc::c_int
                    ^ *bp.offset(i as isize) as libc::c_int) as libc::c_uchar
                as libc::c_uchar,
        );
        i = i.wrapping_add(1);
        i;
    }
    return ((diff as uint32_t).wrapping_sub(1 as libc::c_uint) >> 31 as libc::c_int)
        as libc::c_int;
}
