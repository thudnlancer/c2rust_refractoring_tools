#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn time(__timer: *mut time_t) -> time_t;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
pub type uint32_t = __uint32_t;
unsafe extern "C" fn seed_from_timestamp_and_pid(
    mut seed: *mut uint32_t,
) -> libc::c_int {
    *seed = time(0 as *mut time_t) as uint32_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn generate_seed() -> uint32_t {
    let mut seed: uint32_t = 0 as libc::c_int as uint32_t;
    let mut done: libc::c_int = 0 as libc::c_int;
    if done == 0 {
        seed_from_timestamp_and_pid(&mut seed);
    }
    if seed == 0 as libc::c_int as libc::c_uint {
        seed = 1 as libc::c_int as uint32_t;
    }
    return seed;
}
#[no_mangle]
pub static mut hashtable_seed: uint32_t = 0 as libc::c_int as uint32_t;
#[no_mangle]
pub unsafe extern "C" fn json_object_seed(mut seed: size_t) {
    let mut new_seed: uint32_t = seed as uint32_t;
    if hashtable_seed == 0 as libc::c_int as libc::c_uint {
        if new_seed == 0 as libc::c_int as libc::c_uint {
            new_seed = generate_seed();
        }
        ::core::ptr::write_volatile(&mut hashtable_seed as *mut uint32_t, new_seed);
    }
}
