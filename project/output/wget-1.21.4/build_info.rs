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
pub static mut compiled_features: [*const i8; 14] = [
    b"-cares\0" as *const u8 as *const i8,
    b"+digest\0" as *const u8 as *const i8,
    b"-gpgme\0" as *const u8 as *const i8,
    b"+https\0" as *const u8 as *const i8,
    b"+ipv6\0" as *const u8 as *const i8,
    b"+iri\0" as *const u8 as *const i8,
    b"+large-file\0" as *const u8 as *const i8,
    b"-metalink\0" as *const u8 as *const i8,
    b"+nls\0" as *const u8 as *const i8,
    b"+ntlm\0" as *const u8 as *const i8,
    b"+opie\0" as *const u8 as *const i8,
    b"-psl\0" as *const u8 as *const i8,
    b"+ssl/gnutls\0" as *const u8 as *const i8,
    0 as *const i8,
];