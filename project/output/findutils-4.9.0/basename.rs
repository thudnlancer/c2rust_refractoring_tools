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
    fn ximalloc(s: idx_t) -> *mut libc::c_void;
}
pub type size_t = u64;
pub type ptrdiff_t = i64;
pub type idx_t = ptrdiff_t;
#[no_mangle]
pub unsafe extern "C" fn base_name(mut name: *const i8) -> *mut i8 {
    let mut base: *const i8 = last_component(name);
    let mut length: idx_t = 0;
    let mut dotslash_len: i32 = 0;
    if *base != 0 {
        length = base_len(base) as idx_t;
        length += (*base.offset(length as isize) as i32 == '/' as i32) as i32 as i64;
        dotslash_len = if 0 as i32 != 0 as i32 { 2 as i32 } else { 0 as i32 };
    } else {
        base = name;
        length = base_len(base) as idx_t;
        dotslash_len = 0 as i32;
    }
    let mut p: *mut i8 = ximalloc(dotslash_len as i64 + length + 1 as i32 as i64)
        as *mut i8;
    if dotslash_len != 0 {
        *p.offset(0 as i32 as isize) = '.' as i32 as i8;
        *p.offset(1 as i32 as isize) = '/' as i32 as i8;
    }
    memcpy(
        p.offset(dotslash_len as isize) as *mut libc::c_void,
        base as *const libc::c_void,
        length as u64,
    );
    *p.offset((dotslash_len as i64 + length) as isize) = '\0' as i32 as i8;
    return p;
}