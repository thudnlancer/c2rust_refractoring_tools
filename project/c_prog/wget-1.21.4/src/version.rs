use ::libc;
#[no_mangle]
pub static mut version_string: *const libc::c_char = b"1.21.4\0" as *const u8
    as *const libc::c_char;
#[no_mangle]
pub static mut compilation_string: *const libc::c_char = b"gcc -DHAVE_CONFIG_H -DSYSTEM_WGETRC=\"/usr/local/etc/wgetrc\" -DLOCALEDIR=\"/usr/local/share/locale\" -I. -I../lib -I../lib -I/usr/include/p11-kit-1 -DHAVE_LIBGNUTLS -DNDEBUG -g -O2\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut link_string: *const libc::c_char = b"gcc -I/usr/include/p11-kit-1 -DHAVE_LIBGNUTLS -DNDEBUG -g -O2 -lpcre -lidn2 -lnettle -lgnutls -lz ../lib/libgnu.a \0"
    as *const u8 as *const libc::c_char;
