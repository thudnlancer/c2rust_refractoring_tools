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
    fn strdup(__s: *const i8) -> *mut i8;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hiredisAllocFuncs {
    pub mallocFn: Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub callocFn: Option<unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void>,
    pub reallocFn: Option<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
    pub strdupFn: Option<unsafe extern "C" fn(*const i8) -> *mut i8>,
    pub freeFn: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[no_mangle]
pub static mut hiredisAllocFns: hiredisAllocFuncs = unsafe {
    {
        let mut init = hiredisAllocFuncs {
            mallocFn: Some(malloc as unsafe extern "C" fn(u64) -> *mut libc::c_void),
            callocFn: Some(
                calloc as unsafe extern "C" fn(u64, u64) -> *mut libc::c_void,
            ),
            reallocFn: Some(
                realloc
                    as unsafe extern "C" fn(*mut libc::c_void, u64) -> *mut libc::c_void,
            ),
            strdupFn: Some(strdup as unsafe extern "C" fn(*const i8) -> *mut i8),
            freeFn: Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn hiredisSetAllocators(
    mut override_0: *mut hiredisAllocFuncs,
) -> hiredisAllocFuncs {
    let mut orig: hiredisAllocFuncs = hiredisAllocFns;
    hiredisAllocFns = *override_0;
    return orig;
}
#[no_mangle]
pub unsafe extern "C" fn hiredisResetAllocators() {
    hiredisAllocFns = {
        let mut init = hiredisAllocFuncs {
            mallocFn: Some(malloc as unsafe extern "C" fn(u64) -> *mut libc::c_void),
            callocFn: Some(
                calloc as unsafe extern "C" fn(u64, u64) -> *mut libc::c_void,
            ),
            reallocFn: Some(
                realloc
                    as unsafe extern "C" fn(*mut libc::c_void, u64) -> *mut libc::c_void,
            ),
            strdupFn: Some(strdup as unsafe extern "C" fn(*const i8) -> *mut i8),
            freeFn: Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        };
        init
    };
}