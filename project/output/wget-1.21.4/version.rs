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
pub static mut version_string: *const i8 = b"1.21.4\0" as *const u8 as *const i8;
#[no_mangle]
pub static mut compilation_string: *const i8 = b"gcc -DHAVE_CONFIG_H -DSYSTEM_WGETRC=\"/usr/local/etc/wgetrc\" -DLOCALEDIR=\"/usr/local/share/locale\" -I. -I../lib -I../lib -I/usr/include/p11-kit-1 -DHAVE_LIBGNUTLS -DNDEBUG -g -O2\0"
    as *const u8 as *const i8;
#[no_mangle]
pub static mut link_string: *const i8 = b"gcc -I/usr/include/p11-kit-1 -DHAVE_LIBGNUTLS -DNDEBUG -g -O2 -lpcre -lidn2 -lnettle -lgnutls -lz ../lib/libgnu.a \0"
    as *const u8 as *const i8;