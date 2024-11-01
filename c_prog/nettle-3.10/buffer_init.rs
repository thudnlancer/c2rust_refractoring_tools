#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn nettle_buffer_init_realloc(
        buffer: *mut nettle_buffer,
        realloc_ctx: *mut libc::c_void,
        realloc: Option::<nettle_realloc_func>,
    );
    fn nettle_realloc(
        _: *mut libc::c_void,
        _: *mut libc::c_void,
        _: size_t,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type nettle_realloc_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
    size_t,
) -> *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_buffer {
    pub contents: *mut uint8_t,
    pub alloc: size_t,
    pub realloc_ctx: *mut libc::c_void,
    pub realloc: Option::<nettle_realloc_func>,
    pub size: size_t,
}
#[no_mangle]
pub unsafe extern "C" fn nettle_buffer_init(mut buffer: *mut nettle_buffer) {
    nettle_buffer_init_realloc(
        buffer,
        0 as *mut libc::c_void,
        Some(nettle_realloc as nettle_realloc_func),
    );
}
