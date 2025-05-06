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
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    static _nettle_hashes: [*const nettle_hash; 0];
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
pub type nettle_hash_init_func = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type nettle_hash_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *const uint8_t,
) -> ();
pub type nettle_hash_digest_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *mut uint8_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_hash {
    pub name: *const i8,
    pub context_size: u32,
    pub digest_size: u32,
    pub block_size: u32,
    pub init: Option<nettle_hash_init_func>,
    pub update: Option<nettle_hash_update_func>,
    pub digest: Option<nettle_hash_digest_func>,
}
#[no_mangle]
pub unsafe extern "C" fn nettle_lookup_hash(mut name: *const i8) -> *const nettle_hash {
    let mut i: u32 = 0;
    i = 0 as i32 as u32;
    while !(*_nettle_hashes.as_ptr().offset(i as isize)).is_null() {
        if strcmp(name, (**_nettle_hashes.as_ptr().offset(i as isize)).name) == 0 {
            return *_nettle_hashes.as_ptr().offset(i as isize);
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *const nettle_hash;
}