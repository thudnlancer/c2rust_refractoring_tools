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
    fn strlen(_: *const i8) -> u64;
    fn rpl_free(ptr: *mut libc::c_void);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn locale_charset() -> *const i8;
    fn iconv_open(__tocode: *const i8, __fromcode: *const i8) -> iconv_t;
    fn iconv(
        __cd: iconv_t,
        __inbuf: *mut *mut i8,
        __inbytesleft: *mut size_t,
        __outbuf: *mut *mut i8,
        __outbytesleft: *mut size_t,
    ) -> size_t;
}
pub type size_t = u64;
pub type iconv_t = *mut libc::c_void;
static mut conv_desc: [iconv_t; 2] = unsafe {
    [-(1 as i32) as iconv_t, -(1 as i32) as iconv_t]
};
unsafe extern "C" fn utf8_init(mut to_utf: bool) -> iconv_t {
    if conv_desc[to_utf as i32 as usize] == -(1 as i32) as iconv_t {
        if to_utf {
            conv_desc[to_utf as i32 as usize] = iconv_open(
                b"UTF-8\0" as *const u8 as *const i8,
                locale_charset(),
            );
        } else {
            conv_desc[to_utf as i32 as usize] = iconv_open(
                locale_charset(),
                b"UTF-8\0" as *const u8 as *const i8,
            );
        }
    }
    return conv_desc[to_utf as i32 as usize];
}
#[no_mangle]
pub unsafe extern "C" fn utf8_convert(
    mut to_utf: bool,
    mut input: *const i8,
    mut output: *mut *mut i8,
) -> bool {
    let mut ib: *mut i8 = 0 as *mut i8;
    let mut ob: *mut i8 = 0 as *mut i8;
    let mut ret: *mut i8 = 0 as *mut i8;
    let mut inlen: size_t = 0;
    let mut outlen: size_t = 0;
    let mut cd: iconv_t = utf8_init(to_utf);
    if cd.is_null() {
        *output = xstrdup(input);
        return 1 as i32 != 0;
    } else if cd == -(1 as i32) as iconv_t {
        return 0 as i32 != 0
    }
    inlen = (strlen(input)).wrapping_add(1 as i32 as u64);
    outlen = inlen.wrapping_mul(16 as i32 as u64).wrapping_add(1 as i32 as u64);
    ret = xmalloc(outlen) as *mut i8;
    ob = ret;
    ib = input as *mut i8;
    if iconv(cd, &mut ib, &mut inlen, &mut ob, &mut outlen) != 0 as i32 as u64 {
        rpl_free(ret as *mut libc::c_void);
        return 0 as i32 != 0;
    }
    *ob = 0 as i32 as i8;
    *output = ret;
    return 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn string_ascii_p(mut p: *const i8) -> bool {
    while *p != 0 {
        if *p as i32 & !(0x7f as i32) != 0 {
            return 0 as i32 != 0;
        }
        p = p.offset(1);
        p;
    }
    return 1 as i32 != 0;
}