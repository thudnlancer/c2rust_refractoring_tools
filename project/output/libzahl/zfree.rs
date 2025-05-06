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
    fn free(__ptr: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    static mut libzahl_pool_n: [size_t; 64];
    static mut libzahl_pool: [*mut *mut zahl_char_t; 64];
    static mut libzahl_pool_alloc: [size_t; 64];
}
pub type size_t = u64;
pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type zahl_char_t = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zahl {
    pub sign: i32,
    pub padding__: i32,
    pub used: size_t,
    pub alloced: size_t,
    pub chars: *mut zahl_char_t,
}
#[no_mangle]
pub unsafe extern "C" fn zfree(mut a: *mut zahl) {
    let mut i: size_t = 0;
    let mut x: size_t = 0;
    let mut j: size_t = 0;
    let mut new: *mut *mut zahl_char_t = 0 as *mut *mut zahl_char_t;
    if ((*a).chars).is_null() as i32 as i64 != 0 {
        return;
    }
    i = (8 as i32 as u64)
        .wrapping_mul(::core::mem::size_of::<u64>() as u64)
        .wrapping_sub(1 as i32 as u64)
        .wrapping_sub(((*a).alloced).leading_zeros() as i32 as size_t);
    let fresh0 = libzahl_pool_n[i as usize];
    libzahl_pool_n[i as usize] = (libzahl_pool_n[i as usize]).wrapping_add(1);
    j = fresh0;
    if j == libzahl_pool_alloc[i as usize] {
        x = if j != 0 {
            j.wrapping_mul(3 as i32 as u64) >> 1 as i32
        } else {
            128 as i32 as u64
        };
        new = realloc(
            libzahl_pool[i as usize] as *mut libc::c_void,
            x.wrapping_mul(::core::mem::size_of::<*mut zahl_char_t>() as u64),
        ) as *mut *mut zahl_char_t;
        if new.is_null() as i32 as i64 != 0 {
            free((*a).chars as *mut libc::c_void);
            free(libzahl_pool[i as usize] as *mut libc::c_void);
            libzahl_pool_n[i as usize] = 0 as i32 as size_t;
            libzahl_pool[i as usize] = 0 as *mut *mut zahl_char_t;
            libzahl_pool_alloc[i as usize] = 0 as i32 as size_t;
            return;
        }
        libzahl_pool[i as usize] = new;
        libzahl_pool_alloc[i as usize] = x;
    }
    let ref mut fresh1 = *(libzahl_pool[i as usize]).offset(j as isize);
    *fresh1 = (*a).chars;
}