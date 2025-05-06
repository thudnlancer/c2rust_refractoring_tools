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
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn rpl_free(ptr: *mut libc::c_void);
}
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct allocator {
    pub allocate: Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub reallocate: Option<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
    pub free: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub die: Option<unsafe extern "C" fn(size_t) -> ()>,
}
#[no_mangle]
pub static mut stdlib_allocator: allocator = unsafe {
    {
        let mut init = allocator {
            allocate: Some(malloc as unsafe extern "C" fn(u64) -> *mut libc::c_void),
            reallocate: Some(
                realloc
                    as unsafe extern "C" fn(*mut libc::c_void, u64) -> *mut libc::c_void,
            ),
            free: Some(rpl_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            die: None,
        };
        init
    }
};