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
    fn strlen(_: *const i8) -> u64;
}
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn last_component(mut name: *const i8) -> *mut i8 {
    let mut base: *const i8 = name.offset(0 as i32 as isize);
    let mut p: *const i8 = 0 as *const i8;
    let mut saw_slash: bool = 0 as i32 != 0;
    while *base as i32 == '/' as i32 {
        base = base.offset(1);
        base;
    }
    p = base;
    while *p != 0 {
        if *p as i32 == '/' as i32 {
            saw_slash = 1 as i32 != 0;
        } else if saw_slash {
            base = p;
            saw_slash = 0 as i32 != 0;
        }
        p = p.offset(1);
        p;
    }
    return base as *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn base_len(mut name: *const i8) -> size_t {
    let mut len: size_t = 0;
    let mut prefix_len: size_t = 0 as i32 as size_t;
    len = strlen(name);
    while (1 as i32 as u64) < len
        && *name.offset(len.wrapping_sub(1 as i32 as u64) as isize) as i32 == '/' as i32
    {
        len = len.wrapping_sub(1);
        len;
    }
    if 0 as i32 != 0 && len == 1 as i32 as u64
        && *name.offset(0 as i32 as isize) as i32 == '/' as i32
        && *name.offset(1 as i32 as isize) as i32 == '/' as i32
        && *name.offset(2 as i32 as isize) == 0
    {
        return 2 as i32 as size_t;
    }
    if 0 as i32 != 0 && prefix_len != 0 && len == prefix_len
        && *name.offset(prefix_len as isize) as i32 == '/' as i32
    {
        return prefix_len.wrapping_add(1 as i32 as u64);
    }
    return len;
}