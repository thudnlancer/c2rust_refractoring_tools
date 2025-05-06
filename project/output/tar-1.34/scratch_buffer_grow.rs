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
    fn rpl_free(ptr: *mut libc::c_void);
    fn malloc(_: u64) -> *mut libc::c_void;
}
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: libc::c_longlong,
    pub __clang_max_align_nonce2: f128::f128,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scratch_buffer {
    pub data: *mut libc::c_void,
    pub length: size_t,
    pub __space: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __align: max_align_t,
    pub __c: [i8; 1024],
}
#[inline]
unsafe extern "C" fn scratch_buffer_free(mut buffer: *mut scratch_buffer) {
    if (*buffer).data != ((*buffer).__space.__c).as_mut_ptr() as *mut libc::c_void {
        rpl_free((*buffer).data);
    }
}
#[inline]
unsafe extern "C" fn scratch_buffer_init(mut buffer: *mut scratch_buffer) {
    (*buffer).data = ((*buffer).__space.__c).as_mut_ptr() as *mut libc::c_void;
    (*buffer).length = ::core::mem::size_of::<C2RustUnnamed>() as u64;
}
#[no_mangle]
pub unsafe extern "C" fn gl_scratch_buffer_grow(
    mut buffer: *mut scratch_buffer,
) -> bool {
    let mut new_ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut new_length: size_t = ((*buffer).length).wrapping_mul(2 as i32 as u64);
    scratch_buffer_free(buffer);
    if (new_length >= (*buffer).length) as i32 as i64 != 0 {
        new_ptr = malloc(new_length);
    } else {
        *__errno_location() = 12 as i32;
        new_ptr = 0 as *mut libc::c_void;
    }
    if (new_ptr == 0 as *mut libc::c_void) as i32 as i64 != 0 {
        scratch_buffer_init(buffer);
        return 0 as i32 != 0;
    }
    (*buffer).data = new_ptr;
    (*buffer).length = new_length;
    return 1 as i32 != 0;
}