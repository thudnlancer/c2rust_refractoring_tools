#[no_mangle]
pub static version_string: &'static str = "1.21.4";
#[no_mangle]
pub static compilation_string: &'static str = "gcc -DHAVE_CONFIG_H -DSYSTEM_WGETRC=\"/usr/local/etc/wgetrc\" -DLOCALEDIR=\"/usr/local/share/locale\" -I. -I../lib -I../lib -I/usr/include/p11-kit-1 -DHAVE_LIBGNUTLS -DNDEBUG -g -O2";
#[no_mangle]
pub static link_string: &'static str = "gcc -I/usr/include/p11-kit-1 -DHAVE_LIBGNUTLS -DNDEBUG -g -O2 -lpcre -lidn2 -lnettle -lgnutls -lz ../lib/libgnu.a";