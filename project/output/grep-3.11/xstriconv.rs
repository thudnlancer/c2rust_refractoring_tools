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
    fn __errno_location() -> *mut i32;
    fn mem_cd_iconv(
        src: *const i8,
        srclen: size_t,
        cd: iconv_t,
        resultp: *mut *mut i8,
        lengthp: *mut size_t,
    ) -> i32;
    fn str_cd_iconv(src: *const i8, cd: iconv_t) -> *mut i8;
    fn str_iconv(
        src: *const i8,
        from_codeset: *const i8,
        to_codeset: *const i8,
    ) -> *mut i8;
    fn xalloc_die();
}
pub type size_t = u64;
pub type iconv_t = *mut libc::c_void;
#[no_mangle]
pub unsafe extern "C" fn xmem_cd_iconv(
    mut src: *const i8,
    mut srclen: size_t,
    mut cd: iconv_t,
    mut resultp: *mut *mut i8,
    mut lengthp: *mut size_t,
) -> i32 {
    let mut retval: i32 = mem_cd_iconv(src, srclen, cd, resultp, lengthp);
    if retval < 0 as i32 && *__errno_location() == 12 as i32 {
        xalloc_die();
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn xstr_cd_iconv(mut src: *const i8, mut cd: iconv_t) -> *mut i8 {
    let mut result: *mut i8 = str_cd_iconv(src, cd);
    if result.is_null() && *__errno_location() == 12 as i32 {
        xalloc_die();
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn xstr_iconv(
    mut src: *const i8,
    mut from_codeset: *const i8,
    mut to_codeset: *const i8,
) -> *mut i8 {
    let mut result: *mut i8 = str_iconv(src, from_codeset, to_codeset);
    if result.is_null() && *__errno_location() == 12 as i32 {
        xalloc_die();
    }
    return result;
}