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
    fn base_len(file: *const i8) -> size_t;
    fn last_component(file: *const i8) -> *mut i8;
}
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn strip_trailing_slashes(mut file: *mut i8) -> bool {
    let mut base: *mut i8 = last_component(file);
    let mut base_lim: *mut i8 = 0 as *mut i8;
    let mut had_slash: bool = false;
    if *base == 0 {
        base = file;
    }
    base_lim = base.offset(base_len(base) as isize);
    had_slash = *base_lim as i32 != '\0' as i32;
    *base_lim = '\0' as i32 as i8;
    return had_slash;
}