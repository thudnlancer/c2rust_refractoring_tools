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
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn gl_dynarray_resize(
        _: *mut dynarray_header,
        size: size_t,
        scratch: *mut libc::c_void,
        element_size: size_t,
    ) -> bool;
}
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dynarray_header {
    pub used: size_t,
    pub allocated: size_t,
    pub array: *mut libc::c_void,
}
#[no_mangle]
pub unsafe extern "C" fn gl_dynarray_resize_clear(
    mut list: *mut dynarray_header,
    mut size: size_t,
    mut scratch: *mut libc::c_void,
    mut element_size: size_t,
) -> bool {
    let mut old_size: size_t = (*list).used;
    if !gl_dynarray_resize(list, size, scratch, element_size) {
        return 0 as i32 != 0;
    }
    let mut array: *mut i8 = (*list).array as *mut i8;
    memset(
        array.offset(old_size.wrapping_mul(element_size) as isize) as *mut libc::c_void,
        0 as i32,
        size.wrapping_sub(old_size).wrapping_mul(element_size),
    );
    return 1 as i32 != 0;
}