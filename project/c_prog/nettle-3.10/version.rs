use ::libc;
#[no_mangle]
pub unsafe extern "C" fn nettle_version_major() -> libc::c_int {
    return 3 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_version_minor() -> libc::c_int {
    return 10 as libc::c_int;
}
