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
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn mbsinit(__ps: *const mbstate_t) -> i32;
    fn wcwidth(__c: wchar_t) -> i32;
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const i8,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
}
pub type C2RustUnnamed = u32;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type size_t = u64;
pub type wchar_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: i32,
    pub __value: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub __wch: u32,
    pub __wchb: [i8; 4],
}
pub type mbstate_t = __mbstate_t;
#[inline]
unsafe extern "C" fn to_uchar(mut ch: i8) -> u8 {
    return ch as u8;
}
unsafe extern "C" fn unibyte_qmark_chars(mut buf: *mut i8, mut len: size_t) -> size_t {
    let mut p: *mut i8 = buf;
    let mut plimit: *const i8 = buf.offset(len as isize);
    while p < plimit as *mut i8 {
        if !(1 as i32 != 0
            && *(*__ctype_b_loc()).offset(to_uchar(*p) as i32 as isize) as i32
                & _ISprint as i32 as libc::c_ushort as i32 != 0)
        {
            *p = '?' as i32 as i8;
        }
        p = p.offset(1);
        p;
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn qmark_chars(mut buf: *mut i8, mut len: size_t) -> size_t {
    if __ctype_get_mb_cur_max() <= 1 as i32 as u64 {
        return unibyte_qmark_chars(buf, len)
    } else {
        let mut p: *const i8 = buf;
        let mut plimit: *const i8 = buf.offset(len as isize);
        let mut q: *mut i8 = buf;
        while p < plimit {
            match *p as i32 {
                32 | 33 | 34 | 35 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47
                | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 | 60 | 61
                | 62 | 63 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76
                | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90
                | 91 | 92 | 93 | 94 | 95 | 97 | 98 | 99 | 100 | 101 | 102 | 103 | 104
                | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115 | 116
                | 117 | 118 | 119 | 120 | 121 | 122 | 123 | 124 | 125 | 126 => {
                    let fresh0 = p;
                    p = p.offset(1);
                    let fresh1 = q;
                    q = q.offset(1);
                    *fresh1 = *fresh0;
                }
                _ => {
                    let mut mbstate: mbstate_t = mbstate_t {
                        __count: 0,
                        __value: C2RustUnnamed_0 { __wch: 0 },
                    };
                    memset(
                        &mut mbstate as *mut mbstate_t as *mut libc::c_void,
                        0 as i32,
                        ::core::mem::size_of::<mbstate_t>() as u64,
                    );
                    loop {
                        let mut wc: wchar_t = 0;
                        let mut bytes: size_t = 0;
                        let mut w: i32 = 0;
                        bytes = rpl_mbrtowc(
                            &mut wc,
                            p,
                            plimit.offset_from(p) as i64 as size_t,
                            &mut mbstate,
                        );
                        if bytes == -(1 as i32) as size_t {
                            p = p.offset(1);
                            p;
                            let fresh2 = q;
                            q = q.offset(1);
                            *fresh2 = '?' as i32 as i8;
                            break;
                        } else if bytes == -(2 as i32) as size_t {
                            p = plimit;
                            let fresh3 = q;
                            q = q.offset(1);
                            *fresh3 = '?' as i32 as i8;
                            break;
                        } else {
                            if bytes == 0 as i32 as u64 {
                                bytes = 1 as i32 as size_t;
                            }
                            w = wcwidth(wc);
                            if w >= 0 as i32 {
                                while bytes > 0 as i32 as u64 {
                                    let fresh4 = p;
                                    p = p.offset(1);
                                    let fresh5 = q;
                                    q = q.offset(1);
                                    *fresh5 = *fresh4;
                                    bytes = bytes.wrapping_sub(1);
                                    bytes;
                                }
                            } else {
                                p = p.offset(bytes as isize);
                                let fresh6 = q;
                                q = q.offset(1);
                                *fresh6 = '?' as i32 as i8;
                            }
                            if !(mbsinit(&mut mbstate) == 0) {
                                break;
                            }
                        }
                    }
                }
            }
        }
        len = q.offset_from(buf) as i64 as size_t;
        return len;
    };
}