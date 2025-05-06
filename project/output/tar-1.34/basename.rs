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
    fn base_len(filename: *const i8) -> size_t;
    fn last_component(filename: *const i8) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrndup(string: *const i8, n: size_t) -> *mut i8;
}
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn base_name(mut name: *const i8) -> *mut i8 {
    let mut base: *const i8 = last_component(name);
    let mut length: size_t = 0;
    if *base == 0 {
        return xstrndup(name, base_len(name));
    }
    length = base_len(base);
    if *base.offset(length as isize) as i32 == '/' as i32 {
        length = length.wrapping_add(1);
        length;
    }
    if 0 as i32 != 0 {
        let mut p: *mut i8 = xmalloc(length.wrapping_add(3 as i32 as u64)) as *mut i8;
        *p.offset(0 as i32 as isize) = '.' as i32 as i8;
        *p.offset(1 as i32 as isize) = '/' as i32 as i8;
        memcpy(
            p.offset(2 as i32 as isize) as *mut libc::c_void,
            base as *const libc::c_void,
            length,
        );
        *p.offset(length.wrapping_add(2 as i32 as u64) as isize) = '\0' as i32 as i8;
        return p;
    }
    return xstrndup(base, length);
}