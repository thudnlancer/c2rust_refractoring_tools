#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub static mut version_string: *const libc::c_char = b"GNU Awk 5.2.2\0" as *const u8
    as *const libc::c_char;
