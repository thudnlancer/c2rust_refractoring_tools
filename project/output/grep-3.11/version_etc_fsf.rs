#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#[no_mangle]
pub static mut version_etc_copyright: [i8; 47] = unsafe {
    *::core::mem::transmute::<
        &[u8; 47],
        &[i8; 47],
    >(b"Copyright %s %d Free Software Foundation, Inc.\0")
};