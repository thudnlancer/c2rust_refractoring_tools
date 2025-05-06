#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic)]
extern "C" {
    fn vsnprintf(_: *mut i8, _: u64, _: *const i8, _: ::core::ffi::VaList) -> i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    static mut hiredisAllocFns: hiredisAllocFuncs;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __ssize_t = i64;
pub type va_list = __builtin_va_list;
pub type ssize_t = __ssize_t;
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type sds = *mut i8;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr5 {
    pub flags: u8,
    pub buf: [i8; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr8 {
    pub len: uint8_t,
    pub alloc: uint8_t,
    pub flags: u8,
    pub buf: [i8; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr16 {
    pub len: uint16_t,
    pub alloc: uint16_t,
    pub flags: u8,
    pub buf: [i8; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr32 {
    pub len: uint32_t,
    pub alloc: uint32_t,
    pub flags: u8,
    pub buf: [i8; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr64 {
    pub len: uint64_t,
    pub alloc: uint64_t,
    pub flags: u8,
    pub buf: [i8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hiredisAllocFuncs {
    pub mallocFn: Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub callocFn: Option<unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void>,
    pub reallocFn: Option<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
    pub strdupFn: Option<unsafe extern "C" fn(*const i8) -> *mut i8>,
    pub freeFn: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[inline]
unsafe extern "C" fn tolower(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn sdslen(s: sds) -> size_t {
    let mut flags: u8 = *s.offset(-(1 as i32) as isize) as u8;
    match flags as i32 & 7 as i32 {
        0 => return (flags as i32 >> 3 as i32) as size_t,
        1 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr8>() as u64 as isize))
                as *mut sdshdr8))
                .len as size_t;
        }
        2 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr16>() as u64 as isize))
                as *mut sdshdr16))
                .len as size_t;
        }
        3 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr32>() as u64 as isize))
                as *mut sdshdr32))
                .len as size_t;
        }
        4 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr64>() as u64 as isize))
                as *mut sdshdr64))
                .len;
        }
        _ => {}
    }
    return 0 as i32 as size_t;
}
#[inline]
unsafe extern "C" fn sdsavail(s: sds) -> size_t {
    let mut flags: u8 = *s.offset(-(1 as i32) as isize) as u8;
    match flags as i32 & 7 as i32 {
        0 => return 0 as i32 as size_t,
        1 => {
            let mut sh: *mut sdshdr8 = s
                .offset(-(::core::mem::size_of::<sdshdr8>() as u64 as isize))
                as *mut sdshdr8;
            return ((*sh).alloc as i32 - (*sh).len as i32) as size_t;
        }
        2 => {
            let mut sh_0: *mut sdshdr16 = s
                .offset(-(::core::mem::size_of::<sdshdr16>() as u64 as isize))
                as *mut sdshdr16;
            return ((*sh_0).alloc as i32 - (*sh_0).len as i32) as size_t;
        }
        3 => {
            let mut sh_1: *mut sdshdr32 = s
                .offset(-(::core::mem::size_of::<sdshdr32>() as u64 as isize))
                as *mut sdshdr32;
            return ((*sh_1).alloc).wrapping_sub((*sh_1).len) as size_t;
        }
        4 => {
            let mut sh_2: *mut sdshdr64 = s
                .offset(-(::core::mem::size_of::<sdshdr64>() as u64 as isize))
                as *mut sdshdr64;
            return ((*sh_2).alloc).wrapping_sub((*sh_2).len);
        }
        _ => {}
    }
    return 0 as i32 as size_t;
}
#[inline]
unsafe extern "C" fn sdssetlen(mut s: sds, mut newlen: size_t) {
    let mut flags: u8 = *s.offset(-(1 as i32) as isize) as u8;
    match flags as i32 & 7 as i32 {
        0 => {
            let mut fp: *mut u8 = (s as *mut u8).offset(-(1 as i32 as isize));
            *fp = (0 as i32 as u64 | newlen << 3 as i32) as u8;
        }
        1 => {
            (*(s.offset(-(::core::mem::size_of::<sdshdr8>() as u64 as isize))
                as *mut sdshdr8))
                .len = newlen as uint8_t;
        }
        2 => {
            (*(s.offset(-(::core::mem::size_of::<sdshdr16>() as u64 as isize))
                as *mut sdshdr16))
                .len = newlen as uint16_t;
        }
        3 => {
            (*(s.offset(-(::core::mem::size_of::<sdshdr32>() as u64 as isize))
                as *mut sdshdr32))
                .len = newlen as uint32_t;
        }
        4 => {
            (*(s.offset(-(::core::mem::size_of::<sdshdr64>() as u64 as isize))
                as *mut sdshdr64))
                .len = newlen;
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn sdsinclen(mut s: sds, mut inc: size_t) {
    let mut flags: u8 = *s.offset(-(1 as i32) as isize) as u8;
    match flags as i32 & 7 as i32 {
        0 => {
            let mut fp: *mut u8 = (s as *mut u8).offset(-(1 as i32 as isize));
            let mut newlen: u8 = ((flags as i32 >> 3 as i32) + inc as u8 as i32) as u8;
            *fp = (0 as i32 | (newlen as i32) << 3 as i32) as u8;
        }
        1 => {
            let ref mut fresh0 = (*(s
                .offset(-(::core::mem::size_of::<sdshdr8>() as u64 as isize))
                as *mut sdshdr8))
                .len;
            *fresh0 = (*fresh0 as i32 + inc as uint8_t as i32) as uint8_t;
        }
        2 => {
            let ref mut fresh1 = (*(s
                .offset(-(::core::mem::size_of::<sdshdr16>() as u64 as isize))
                as *mut sdshdr16))
                .len;
            *fresh1 = (*fresh1 as i32 + inc as uint16_t as i32) as uint16_t;
        }
        3 => {
            let ref mut fresh2 = (*(s
                .offset(-(::core::mem::size_of::<sdshdr32>() as u64 as isize))
                as *mut sdshdr32))
                .len;
            *fresh2 = (*fresh2 as u32).wrapping_add(inc as uint32_t) as uint32_t
                as uint32_t;
        }
        4 => {
            let ref mut fresh3 = (*(s
                .offset(-(::core::mem::size_of::<sdshdr64>() as u64 as isize))
                as *mut sdshdr64))
                .len;
            *fresh3 = (*fresh3 as u64).wrapping_add(inc) as uint64_t as uint64_t;
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn sdsalloc(s: sds) -> size_t {
    let mut flags: u8 = *s.offset(-(1 as i32) as isize) as u8;
    match flags as i32 & 7 as i32 {
        0 => return (flags as i32 >> 3 as i32) as size_t,
        1 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr8>() as u64 as isize))
                as *mut sdshdr8))
                .alloc as size_t;
        }
        2 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr16>() as u64 as isize))
                as *mut sdshdr16))
                .alloc as size_t;
        }
        3 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr32>() as u64 as isize))
                as *mut sdshdr32))
                .alloc as size_t;
        }
        4 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr64>() as u64 as isize))
                as *mut sdshdr64))
                .alloc;
        }
        _ => {}
    }
    return 0 as i32 as size_t;
}
#[inline]
unsafe extern "C" fn sdssetalloc(mut s: sds, mut newlen: size_t) {
    let mut flags: u8 = *s.offset(-(1 as i32) as isize) as u8;
    match flags as i32 & 7 as i32 {
        1 => {
            (*(s.offset(-(::core::mem::size_of::<sdshdr8>() as u64 as isize))
                as *mut sdshdr8))
                .alloc = newlen as uint8_t;
        }
        2 => {
            (*(s.offset(-(::core::mem::size_of::<sdshdr16>() as u64 as isize))
                as *mut sdshdr16))
                .alloc = newlen as uint16_t;
        }
        3 => {
            (*(s.offset(-(::core::mem::size_of::<sdshdr32>() as u64 as isize))
                as *mut sdshdr32))
                .alloc = newlen as uint32_t;
        }
        4 => {
            (*(s.offset(-(::core::mem::size_of::<sdshdr64>() as u64 as isize))
                as *mut sdshdr64))
                .alloc = newlen;
        }
        0 | _ => {}
    };
}
#[inline]
unsafe extern "C" fn hi_malloc(mut size: size_t) -> *mut libc::c_void {
    return (hiredisAllocFns.mallocFn).expect("non-null function pointer")(size);
}
#[inline]
unsafe extern "C" fn hi_realloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    return (hiredisAllocFns.reallocFn).expect("non-null function pointer")(ptr, size);
}
#[inline]
unsafe extern "C" fn hi_free(mut ptr: *mut libc::c_void) {
    (hiredisAllocFns.freeFn).expect("non-null function pointer")(ptr);
}
#[inline]
unsafe extern "C" fn sdsHdrSize(mut type_0: i8) -> i32 {
    match type_0 as i32 & 7 as i32 {
        0 => return ::core::mem::size_of::<sdshdr5>() as u64 as i32,
        1 => return ::core::mem::size_of::<sdshdr8>() as u64 as i32,
        2 => return ::core::mem::size_of::<sdshdr16>() as u64 as i32,
        3 => return ::core::mem::size_of::<sdshdr32>() as u64 as i32,
        4 => return ::core::mem::size_of::<sdshdr64>() as u64 as i32,
        _ => {}
    }
    return 0 as i32;
}
#[inline]
unsafe extern "C" fn sdsReqType(mut string_size: size_t) -> i8 {
    if string_size < 32 as i32 as u64 {
        return 0 as i32 as i8;
    }
    if string_size < 0xff as i32 as u64 {
        return 1 as i32 as i8;
    }
    if string_size < 0xffff as i32 as u64 {
        return 2 as i32 as i8;
    }
    if string_size < 0xffffffff as u32 as u64 {
        return 3 as i32 as i8;
    }
    return 4 as i32 as i8;
}
#[no_mangle]
pub unsafe extern "C" fn sdsnewlen(
    mut init: *const libc::c_void,
    mut initlen: size_t,
) -> sds {
    let mut sh: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut s: sds = 0 as *mut i8;
    let mut type_0: i8 = sdsReqType(initlen);
    if type_0 as i32 == 0 as i32 && initlen == 0 as i32 as u64 {
        type_0 = 1 as i32 as i8;
    }
    let mut hdrlen: i32 = sdsHdrSize(type_0);
    let mut fp: *mut u8 = 0 as *mut u8;
    sh = hi_malloc((hdrlen as u64).wrapping_add(initlen).wrapping_add(1 as i32 as u64));
    if sh.is_null() {
        return 0 as sds;
    }
    if init.is_null() {
        memset(
            sh,
            0 as i32,
            (hdrlen as u64).wrapping_add(initlen).wrapping_add(1 as i32 as u64),
        );
    }
    s = (sh as *mut i8).offset(hdrlen as isize);
    fp = (s as *mut u8).offset(-(1 as i32 as isize));
    match type_0 as i32 {
        0 => {
            *fp = (type_0 as u64 | initlen << 3 as i32) as u8;
        }
        1 => {
            let mut sh_0: *mut sdshdr8 = s
                .offset(-(::core::mem::size_of::<sdshdr8>() as u64 as isize))
                as *mut sdshdr8;
            (*sh_0).len = initlen as uint8_t;
            (*sh_0).alloc = initlen as uint8_t;
            *fp = type_0 as u8;
        }
        2 => {
            let mut sh_1: *mut sdshdr16 = s
                .offset(-(::core::mem::size_of::<sdshdr16>() as u64 as isize))
                as *mut sdshdr16;
            (*sh_1).len = initlen as uint16_t;
            (*sh_1).alloc = initlen as uint16_t;
            *fp = type_0 as u8;
        }
        3 => {
            let mut sh_2: *mut sdshdr32 = s
                .offset(-(::core::mem::size_of::<sdshdr32>() as u64 as isize))
                as *mut sdshdr32;
            (*sh_2).len = initlen as uint32_t;
            (*sh_2).alloc = initlen as uint32_t;
            *fp = type_0 as u8;
        }
        4 => {
            let mut sh_3: *mut sdshdr64 = s
                .offset(-(::core::mem::size_of::<sdshdr64>() as u64 as isize))
                as *mut sdshdr64;
            (*sh_3).len = initlen;
            (*sh_3).alloc = initlen;
            *fp = type_0 as u8;
        }
        _ => {}
    }
    if initlen != 0 && !init.is_null() {
        memcpy(s as *mut libc::c_void, init, initlen);
    }
    *s.offset(initlen as isize) = '\0' as i32 as i8;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn sdsempty() -> sds {
    return sdsnewlen(
        b"\0" as *const u8 as *const i8 as *const libc::c_void,
        0 as i32 as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn sdsnew(mut init: *const i8) -> sds {
    let mut initlen: size_t = if init.is_null() {
        0 as i32 as u64
    } else {
        strlen(init)
    };
    return sdsnewlen(init as *const libc::c_void, initlen);
}
#[no_mangle]
pub unsafe extern "C" fn sdsdup(s: sds) -> sds {
    return sdsnewlen(s as *const libc::c_void, sdslen(s));
}
#[no_mangle]
pub unsafe extern "C" fn sdsfree(mut s: sds) {
    if s.is_null() {
        return;
    }
    hi_free(
        s.offset(-(sdsHdrSize(*s.offset(-(1 as i32) as isize)) as isize))
            as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn sdsupdatelen(mut s: sds) {
    let mut reallen: i32 = strlen(s as *const i8) as i32;
    sdssetlen(s, reallen as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn sdsclear(mut s: sds) {
    sdssetlen(s, 0 as i32 as size_t);
    *s.offset(0 as i32 as isize) = '\0' as i32 as i8;
}
#[no_mangle]
pub unsafe extern "C" fn sdsMakeRoomFor(mut s: sds, mut addlen: size_t) -> sds {
    let mut sh: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut newsh: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut avail: size_t = sdsavail(s);
    let mut len: size_t = 0;
    let mut newlen: size_t = 0;
    let mut type_0: i8 = 0;
    let mut oldtype: i8 = (*s.offset(-(1 as i32) as isize) as i32 & 7 as i32) as i8;
    let mut hdrlen: i32 = 0;
    if avail >= addlen {
        return s;
    }
    len = sdslen(s);
    sh = s.offset(-(sdsHdrSize(oldtype) as isize)) as *mut libc::c_void;
    newlen = len.wrapping_add(addlen);
    if newlen < (1024 as i32 * 1024 as i32) as u64 {
        newlen = (newlen as u64).wrapping_mul(2 as i32 as u64) as size_t as size_t;
    } else {
        newlen = (newlen as u64).wrapping_add((1024 as i32 * 1024 as i32) as u64)
            as size_t as size_t;
    }
    type_0 = sdsReqType(newlen);
    if type_0 as i32 == 0 as i32 {
        type_0 = 1 as i32 as i8;
    }
    hdrlen = sdsHdrSize(type_0);
    if oldtype as i32 == type_0 as i32 {
        newsh = hi_realloc(
            sh,
            (hdrlen as u64).wrapping_add(newlen).wrapping_add(1 as i32 as u64),
        );
        if newsh.is_null() {
            return 0 as sds;
        }
        s = (newsh as *mut i8).offset(hdrlen as isize);
    } else {
        newsh = hi_malloc(
            (hdrlen as u64).wrapping_add(newlen).wrapping_add(1 as i32 as u64),
        );
        if newsh.is_null() {
            return 0 as sds;
        }
        memcpy(
            (newsh as *mut i8).offset(hdrlen as isize) as *mut libc::c_void,
            s as *const libc::c_void,
            len.wrapping_add(1 as i32 as u64),
        );
        hi_free(sh);
        s = (newsh as *mut i8).offset(hdrlen as isize);
        *s.offset(-(1 as i32) as isize) = type_0;
        sdssetlen(s, len);
    }
    sdssetalloc(s, newlen);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn sdsRemoveFreeSpace(mut s: sds) -> sds {
    let mut sh: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut newsh: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut type_0: i8 = 0;
    let mut oldtype: i8 = (*s.offset(-(1 as i32) as isize) as i32 & 7 as i32) as i8;
    let mut hdrlen: i32 = 0;
    let mut len: size_t = sdslen(s);
    sh = s.offset(-(sdsHdrSize(oldtype) as isize)) as *mut libc::c_void;
    type_0 = sdsReqType(len);
    hdrlen = sdsHdrSize(type_0);
    if oldtype as i32 == type_0 as i32 {
        newsh = hi_realloc(
            sh,
            (hdrlen as u64).wrapping_add(len).wrapping_add(1 as i32 as u64),
        );
        if newsh.is_null() {
            return 0 as sds;
        }
        s = (newsh as *mut i8).offset(hdrlen as isize);
    } else {
        newsh = hi_malloc(
            (hdrlen as u64).wrapping_add(len).wrapping_add(1 as i32 as u64),
        );
        if newsh.is_null() {
            return 0 as sds;
        }
        memcpy(
            (newsh as *mut i8).offset(hdrlen as isize) as *mut libc::c_void,
            s as *const libc::c_void,
            len.wrapping_add(1 as i32 as u64),
        );
        hi_free(sh);
        s = (newsh as *mut i8).offset(hdrlen as isize);
        *s.offset(-(1 as i32) as isize) = type_0;
        sdssetlen(s, len);
    }
    sdssetalloc(s, len);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn sdsAllocSize(mut s: sds) -> size_t {
    let mut alloc: size_t = sdsalloc(s);
    return (sdsHdrSize(*s.offset(-(1 as i32) as isize)) as u64)
        .wrapping_add(alloc)
        .wrapping_add(1 as i32 as u64);
}
#[no_mangle]
pub unsafe extern "C" fn sdsAllocPtr(mut s: sds) -> *mut libc::c_void {
    return s.offset(-(sdsHdrSize(*s.offset(-(1 as i32) as isize)) as isize))
        as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn sdsIncrLen(mut s: sds, mut incr: i32) {
    let mut flags: u8 = *s.offset(-(1 as i32) as isize) as u8;
    let mut len: size_t = 0;
    match flags as i32 & 7 as i32 {
        0 => {
            let mut fp: *mut u8 = (s as *mut u8).offset(-(1 as i32 as isize));
            let mut oldlen: u8 = (flags as i32 >> 3 as i32) as u8;
            if incr > 0 as i32 && oldlen as i32 + incr < 32 as i32
                || incr < 0 as i32 && oldlen as u32 >= -incr as u32
            {} else {
                __assert_fail(
                    b"(incr > 0 && oldlen+incr < 32) || (incr < 0 && oldlen >= (unsigned int)(-incr))\0"
                        as *const u8 as *const i8,
                    b"src/hiredis/sds.c\0" as *const u8 as *const i8,
                    321 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 26],
                        &[i8; 26],
                    >(b"void sdsIncrLen(sds, int)\0"))
                        .as_ptr(),
                );
            };
            *fp = (0 as i32 | oldlen as i32 + incr << 3 as i32) as u8;
            len = (oldlen as i32 + incr) as size_t;
        }
        1 => {
            let mut sh: *mut sdshdr8 = s
                .offset(-(::core::mem::size_of::<sdshdr8>() as u64 as isize))
                as *mut sdshdr8;
            if incr >= 0 as i32 && (*sh).alloc as i32 - (*sh).len as i32 >= incr
                || incr < 0 as i32 && (*sh).len as u32 >= -incr as u32
            {} else {
                __assert_fail(
                    b"(incr >= 0 && sh->alloc-sh->len >= incr) || (incr < 0 && sh->len >= (unsigned int)(-incr))\0"
                        as *const u8 as *const i8,
                    b"src/hiredis/sds.c\0" as *const u8 as *const i8,
                    328 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 26],
                        &[i8; 26],
                    >(b"void sdsIncrLen(sds, int)\0"))
                        .as_ptr(),
                );
            };
            (*sh).len = ((*sh).len as i32 + incr) as uint8_t;
            len = (*sh).len as size_t;
        }
        2 => {
            let mut sh_0: *mut sdshdr16 = s
                .offset(-(::core::mem::size_of::<sdshdr16>() as u64 as isize))
                as *mut sdshdr16;
            if incr >= 0 as i32 && (*sh_0).alloc as i32 - (*sh_0).len as i32 >= incr
                || incr < 0 as i32 && (*sh_0).len as u32 >= -incr as u32
            {} else {
                __assert_fail(
                    b"(incr >= 0 && sh->alloc-sh->len >= incr) || (incr < 0 && sh->len >= (unsigned int)(-incr))\0"
                        as *const u8 as *const i8,
                    b"src/hiredis/sds.c\0" as *const u8 as *const i8,
                    334 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 26],
                        &[i8; 26],
                    >(b"void sdsIncrLen(sds, int)\0"))
                        .as_ptr(),
                );
            };
            (*sh_0).len = ((*sh_0).len as i32 + incr) as uint16_t;
            len = (*sh_0).len as size_t;
        }
        3 => {
            let mut sh_1: *mut sdshdr32 = s
                .offset(-(::core::mem::size_of::<sdshdr32>() as u64 as isize))
                as *mut sdshdr32;
            if incr >= 0 as i32
                && ((*sh_1).alloc).wrapping_sub((*sh_1).len) >= incr as u32
                || incr < 0 as i32 && (*sh_1).len >= -incr as u32
            {} else {
                __assert_fail(
                    b"(incr >= 0 && sh->alloc-sh->len >= (unsigned int)incr) || (incr < 0 && sh->len >= (unsigned int)(-incr))\0"
                        as *const u8 as *const i8,
                    b"src/hiredis/sds.c\0" as *const u8 as *const i8,
                    340 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 26],
                        &[i8; 26],
                    >(b"void sdsIncrLen(sds, int)\0"))
                        .as_ptr(),
                );
            };
            (*sh_1).len = ((*sh_1).len as u32).wrapping_add(incr as u32) as uint32_t
                as uint32_t;
            len = (*sh_1).len as size_t;
        }
        4 => {
            let mut sh_2: *mut sdshdr64 = s
                .offset(-(::core::mem::size_of::<sdshdr64>() as u64 as isize))
                as *mut sdshdr64;
            if incr >= 0 as i32
                && ((*sh_2).alloc).wrapping_sub((*sh_2).len) >= incr as uint64_t
                || incr < 0 as i32 && (*sh_2).len >= -incr as uint64_t
            {} else {
                __assert_fail(
                    b"(incr >= 0 && sh->alloc-sh->len >= (uint64_t)incr) || (incr < 0 && sh->len >= (uint64_t)(-incr))\0"
                        as *const u8 as *const i8,
                    b"src/hiredis/sds.c\0" as *const u8 as *const i8,
                    346 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 26],
                        &[i8; 26],
                    >(b"void sdsIncrLen(sds, int)\0"))
                        .as_ptr(),
                );
            };
            (*sh_2).len = ((*sh_2).len as u64).wrapping_add(incr as u64) as uint64_t
                as uint64_t;
            len = (*sh_2).len;
        }
        _ => {
            len = 0 as i32 as size_t;
        }
    }
    *s.offset(len as isize) = '\0' as i32 as i8;
}
#[no_mangle]
pub unsafe extern "C" fn sdsgrowzero(mut s: sds, mut len: size_t) -> sds {
    let mut curlen: size_t = sdslen(s);
    if len <= curlen {
        return s;
    }
    s = sdsMakeRoomFor(s, len.wrapping_sub(curlen));
    if s.is_null() {
        return 0 as sds;
    }
    memset(
        s.offset(curlen as isize) as *mut libc::c_void,
        0 as i32,
        len.wrapping_sub(curlen).wrapping_add(1 as i32 as u64),
    );
    sdssetlen(s, len);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn sdscatlen(
    mut s: sds,
    mut t: *const libc::c_void,
    mut len: size_t,
) -> sds {
    let mut curlen: size_t = sdslen(s);
    s = sdsMakeRoomFor(s, len);
    if s.is_null() {
        return 0 as sds;
    }
    memcpy(s.offset(curlen as isize) as *mut libc::c_void, t, len);
    sdssetlen(s, curlen.wrapping_add(len));
    *s.offset(curlen.wrapping_add(len) as isize) = '\0' as i32 as i8;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn sdscat(mut s: sds, mut t: *const i8) -> sds {
    return sdscatlen(s, t as *const libc::c_void, strlen(t));
}
#[no_mangle]
pub unsafe extern "C" fn sdscatsds(mut s: sds, t: sds) -> sds {
    return sdscatlen(s, t as *const libc::c_void, sdslen(t));
}
#[no_mangle]
pub unsafe extern "C" fn sdscpylen(
    mut s: sds,
    mut t: *const i8,
    mut len: size_t,
) -> sds {
    if sdsalloc(s) < len {
        s = sdsMakeRoomFor(s, len.wrapping_sub(sdslen(s)));
        if s.is_null() {
            return 0 as sds;
        }
    }
    memcpy(s as *mut libc::c_void, t as *const libc::c_void, len);
    *s.offset(len as isize) = '\0' as i32 as i8;
    sdssetlen(s, len);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn sdscpy(mut s: sds, mut t: *const i8) -> sds {
    return sdscpylen(s, t, strlen(t));
}
#[no_mangle]
pub unsafe extern "C" fn sdsll2str(mut s: *mut i8, mut value: libc::c_longlong) -> i32 {
    let mut p: *mut i8 = 0 as *mut i8;
    let mut aux: i8 = 0;
    let mut v: libc::c_ulonglong = 0;
    let mut l: size_t = 0;
    v = (if value < 0 as i32 as libc::c_longlong { -value } else { value })
        as libc::c_ulonglong;
    p = s;
    loop {
        let fresh4 = p;
        p = p.offset(1);
        *fresh4 = ('0' as i32 as libc::c_ulonglong)
            .wrapping_add(v.wrapping_rem(10 as i32 as libc::c_ulonglong)) as i8;
        v = v.wrapping_div(10 as i32 as libc::c_ulonglong);
        if !(v != 0) {
            break;
        }
    }
    if value < 0 as i32 as libc::c_longlong {
        let fresh5 = p;
        p = p.offset(1);
        *fresh5 = '-' as i32 as i8;
    }
    l = p.offset_from(s) as i64 as size_t;
    *p = '\0' as i32 as i8;
    p = p.offset(-1);
    p;
    while s < p {
        aux = *s;
        *s = *p;
        *p = aux;
        s = s.offset(1);
        s;
        p = p.offset(-1);
        p;
    }
    return l as i32;
}
#[no_mangle]
pub unsafe extern "C" fn sdsull2str(mut s: *mut i8, mut v: libc::c_ulonglong) -> i32 {
    let mut p: *mut i8 = 0 as *mut i8;
    let mut aux: i8 = 0;
    let mut l: size_t = 0;
    p = s;
    loop {
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = ('0' as i32 as libc::c_ulonglong)
            .wrapping_add(v.wrapping_rem(10 as i32 as libc::c_ulonglong)) as i8;
        v = v.wrapping_div(10 as i32 as libc::c_ulonglong);
        if !(v != 0) {
            break;
        }
    }
    l = p.offset_from(s) as i64 as size_t;
    *p = '\0' as i32 as i8;
    p = p.offset(-1);
    p;
    while s < p {
        aux = *s;
        *s = *p;
        *p = aux;
        s = s.offset(1);
        s;
        p = p.offset(-1);
        p;
    }
    return l as i32;
}
#[no_mangle]
pub unsafe extern "C" fn sdsfromlonglong(mut value: libc::c_longlong) -> sds {
    let mut buf: [i8; 21] = [0; 21];
    let mut len: i32 = sdsll2str(buf.as_mut_ptr(), value);
    return sdsnewlen(buf.as_mut_ptr() as *const libc::c_void, len as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn sdscatvprintf(
    mut s: sds,
    mut fmt: *const i8,
    mut ap: ::core::ffi::VaList,
) -> sds {
    let mut cpy: ::core::ffi::VaListImpl;
    let mut staticbuf: [i8; 1024] = [0; 1024];
    let mut buf: *mut i8 = staticbuf.as_mut_ptr();
    let mut t: *mut i8 = 0 as *mut i8;
    let mut buflen: size_t = (strlen(fmt)).wrapping_mul(2 as i32 as u64);
    if buflen > ::core::mem::size_of::<[i8; 1024]>() as u64 {
        buf = hi_malloc(buflen) as *mut i8;
        if buf.is_null() {
            return 0 as sds;
        }
    } else {
        buflen = ::core::mem::size_of::<[i8; 1024]>() as u64;
    }
    loop {
        *buf.offset(buflen.wrapping_sub(2 as i32 as u64) as isize) = '\0' as i32 as i8;
        cpy = ap.clone();
        vsnprintf(buf, buflen, fmt, cpy.as_va_list());
        if !(*buf.offset(buflen.wrapping_sub(2 as i32 as u64) as isize) as i32
            != '\0' as i32)
        {
            break;
        }
        if buf != staticbuf.as_mut_ptr() {
            hi_free(buf as *mut libc::c_void);
        }
        buflen = (buflen as u64).wrapping_mul(2 as i32 as u64) as size_t as size_t;
        buf = hi_malloc(buflen) as *mut i8;
        if buf.is_null() {
            return 0 as sds;
        }
    }
    t = sdscat(s, buf);
    if buf != staticbuf.as_mut_ptr() {
        hi_free(buf as *mut libc::c_void);
    }
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn sdscatprintf(
    mut s: sds,
    mut fmt: *const i8,
    mut args: ...
) -> sds {
    let mut ap: ::core::ffi::VaListImpl;
    let mut t: *mut i8 = 0 as *mut i8;
    ap = args.clone();
    t = sdscatvprintf(s, fmt, ap.as_va_list());
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn sdscatfmt(
    mut s: sds,
    mut fmt: *const i8,
    mut args: ...
) -> sds {
    let mut current_block: u64;
    let mut f: *const i8 = fmt;
    let mut i: i32 = 0;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    i = sdslen(s) as i32;
    loop {
        if !(*f != 0) {
            current_block = 15004371738079956865;
            break;
        }
        let mut next: i8 = 0;
        let mut str: *mut i8 = 0 as *mut i8;
        let mut l: size_t = 0;
        let mut num: libc::c_longlong = 0;
        let mut unum: libc::c_ulonglong = 0;
        if sdsavail(s) == 0 as i32 as u64 {
            s = sdsMakeRoomFor(s, 1 as i32 as size_t);
            if s.is_null() {
                current_block = 6174974146017752131;
                break;
            }
        }
        match *f as i32 {
            37 => {
                next = *f.offset(1 as i32 as isize);
                f = f.offset(1);
                f;
                match next as i32 {
                    115 | 83 => {
                        str = ap.arg::<*mut i8>();
                        l = if next as i32 == 's' as i32 {
                            strlen(str)
                        } else {
                            sdslen(str)
                        };
                        if sdsavail(s) < l {
                            s = sdsMakeRoomFor(s, l);
                            if s.is_null() {
                                current_block = 6174974146017752131;
                                break;
                            }
                        }
                        memcpy(
                            s.offset(i as isize) as *mut libc::c_void,
                            str as *const libc::c_void,
                            l,
                        );
                        sdsinclen(s, l);
                        i = (i as u64).wrapping_add(l) as i32 as i32;
                    }
                    105 | 73 => {
                        if next as i32 == 'i' as i32 {
                            num = ap.arg::<i32>() as libc::c_longlong;
                        } else {
                            num = ap.arg::<libc::c_longlong>();
                        }
                        let mut buf: [i8; 21] = [0; 21];
                        l = sdsll2str(buf.as_mut_ptr(), num) as size_t;
                        if sdsavail(s) < l {
                            s = sdsMakeRoomFor(s, l);
                            if s.is_null() {
                                current_block = 6174974146017752131;
                                break;
                            }
                        }
                        memcpy(
                            s.offset(i as isize) as *mut libc::c_void,
                            buf.as_mut_ptr() as *const libc::c_void,
                            l,
                        );
                        sdsinclen(s, l);
                        i = (i as u64).wrapping_add(l) as i32 as i32;
                    }
                    117 | 85 => {
                        if next as i32 == 'u' as i32 {
                            unum = ap.arg::<u32>() as libc::c_ulonglong;
                        } else {
                            unum = ap.arg::<libc::c_ulonglong>();
                        }
                        let mut buf_0: [i8; 21] = [0; 21];
                        l = sdsull2str(buf_0.as_mut_ptr(), unum) as size_t;
                        if sdsavail(s) < l {
                            s = sdsMakeRoomFor(s, l);
                            if s.is_null() {
                                current_block = 6174974146017752131;
                                break;
                            }
                        }
                        memcpy(
                            s.offset(i as isize) as *mut libc::c_void,
                            buf_0.as_mut_ptr() as *const libc::c_void,
                            l,
                        );
                        sdsinclen(s, l);
                        i = (i as u64).wrapping_add(l) as i32 as i32;
                    }
                    _ => {
                        let fresh7 = i;
                        i = i + 1;
                        *s.offset(fresh7 as isize) = next;
                        sdsinclen(s, 1 as i32 as size_t);
                    }
                }
            }
            _ => {
                let fresh8 = i;
                i = i + 1;
                *s.offset(fresh8 as isize) = *f;
                sdsinclen(s, 1 as i32 as size_t);
            }
        }
        f = f.offset(1);
        f;
    }
    match current_block {
        6174974146017752131 => return 0 as sds,
        _ => {
            *s.offset(i as isize) = '\0' as i32 as i8;
            return s;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn sdstrim(mut s: sds, mut cset: *const i8) -> sds {
    let mut start: *mut i8 = 0 as *mut i8;
    let mut end: *mut i8 = 0 as *mut i8;
    let mut sp: *mut i8 = 0 as *mut i8;
    let mut ep: *mut i8 = 0 as *mut i8;
    let mut len: size_t = 0;
    start = s;
    sp = start;
    end = s.offset(sdslen(s) as isize).offset(-(1 as i32 as isize));
    ep = end;
    while sp <= end && !(strchr(cset, *sp as i32)).is_null() {
        sp = sp.offset(1);
        sp;
    }
    while ep > sp && !(strchr(cset, *ep as i32)).is_null() {
        ep = ep.offset(-1);
        ep;
    }
    len = (if sp > ep {
        0 as i32 as i64
    } else {
        ep.offset_from(sp) as i64 + 1 as i32 as i64
    }) as size_t;
    if s != sp {
        memmove(s as *mut libc::c_void, sp as *const libc::c_void, len);
    }
    *s.offset(len as isize) = '\0' as i32 as i8;
    sdssetlen(s, len);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn sdsrange(
    mut s: sds,
    mut start: ssize_t,
    mut end: ssize_t,
) -> i32 {
    let mut newlen: size_t = 0;
    let mut len: size_t = sdslen(s);
    if len > 9223372036854775807 as i64 as u64 {
        return -(1 as i32);
    }
    if len == 0 as i32 as u64 {
        return 0 as i32;
    }
    if start < 0 as i32 as i64 {
        start = len.wrapping_add(start as u64) as ssize_t;
        if start < 0 as i32 as i64 {
            start = 0 as i32 as ssize_t;
        }
    }
    if end < 0 as i32 as i64 {
        end = len.wrapping_add(end as u64) as ssize_t;
        if end < 0 as i32 as i64 {
            end = 0 as i32 as ssize_t;
        }
    }
    newlen = (if start > end { 0 as i32 as i64 } else { end - start + 1 as i32 as i64 })
        as size_t;
    if newlen != 0 as i32 as u64 {
        if start >= len as ssize_t {
            newlen = 0 as i32 as size_t;
        } else if end >= len as ssize_t {
            end = len.wrapping_sub(1 as i32 as u64) as ssize_t;
            newlen = (if start > end {
                0 as i32 as i64
            } else {
                end - start + 1 as i32 as i64
            }) as size_t;
        }
    } else {
        start = 0 as i32 as ssize_t;
    }
    if start != 0 && newlen != 0 {
        memmove(
            s as *mut libc::c_void,
            s.offset(start as isize) as *const libc::c_void,
            newlen,
        );
    }
    *s.offset(newlen as isize) = 0 as i32 as i8;
    sdssetlen(s, newlen);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn sdstolower(mut s: sds) {
    let mut len: i32 = sdslen(s) as i32;
    let mut j: i32 = 0;
    j = 0 as i32;
    while j < len {
        *s.offset(j as isize) = ({
            let mut __res: i32 = 0;
            if ::core::mem::size_of::<i8>() as u64 > 1 as i32 as u64 {
                if 0 != 0 {
                    let mut __c: i32 = *s.offset(j as isize) as i32;
                    __res = if __c < -(128 as i32) || __c > 255 as i32 {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    };
                } else {
                    __res = tolower(*s.offset(j as isize) as i32);
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(*s.offset(j as isize) as i32 as isize);
            }
            __res
        }) as i8;
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sdstoupper(mut s: sds) {
    let mut len: i32 = sdslen(s) as i32;
    let mut j: i32 = 0;
    j = 0 as i32;
    while j < len {
        *s.offset(j as isize) = ({
            let mut __res: i32 = 0;
            if ::core::mem::size_of::<i8>() as u64 > 1 as i32 as u64 {
                if 0 != 0 {
                    let mut __c: i32 = *s.offset(j as isize) as i32;
                    __res = if __c < -(128 as i32) || __c > 255 as i32 {
                        __c
                    } else {
                        *(*__ctype_toupper_loc()).offset(__c as isize)
                    };
                } else {
                    __res = toupper(*s.offset(j as isize) as i32);
                }
            } else {
                __res = *(*__ctype_toupper_loc())
                    .offset(*s.offset(j as isize) as i32 as isize);
            }
            __res
        }) as i8;
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sdscmp(s1: sds, s2: sds) -> i32 {
    let mut l1: size_t = 0;
    let mut l2: size_t = 0;
    let mut minlen: size_t = 0;
    let mut cmp: i32 = 0;
    l1 = sdslen(s1);
    l2 = sdslen(s2);
    minlen = if l1 < l2 { l1 } else { l2 };
    cmp = memcmp(s1 as *const libc::c_void, s2 as *const libc::c_void, minlen);
    if cmp == 0 as i32 {
        return l1.wrapping_sub(l2) as i32;
    }
    return cmp;
}
#[no_mangle]
pub unsafe extern "C" fn sdssplitlen(
    mut s: *const i8,
    mut len: i32,
    mut sep: *const i8,
    mut seplen: i32,
    mut count: *mut i32,
) -> *mut sds {
    let mut current_block: u64;
    let mut elements: i32 = 0 as i32;
    let mut slots: i32 = 5 as i32;
    let mut start: i32 = 0 as i32;
    let mut j: i32 = 0;
    let mut tokens: *mut sds = 0 as *mut sds;
    if seplen < 1 as i32 || len < 0 as i32 {
        return 0 as *mut sds;
    }
    tokens = hi_malloc((::core::mem::size_of::<sds>() as u64).wrapping_mul(slots as u64))
        as *mut sds;
    if tokens.is_null() {
        return 0 as *mut sds;
    }
    if len == 0 as i32 {
        *count = 0 as i32;
        return tokens;
    }
    j = 0 as i32;
    loop {
        if !(j < len - (seplen - 1 as i32)) {
            current_block = 15904375183555213903;
            break;
        }
        if slots < elements + 2 as i32 {
            let mut newtokens: *mut sds = 0 as *mut sds;
            slots *= 2 as i32;
            newtokens = hi_realloc(
                tokens as *mut libc::c_void,
                (::core::mem::size_of::<sds>() as u64).wrapping_mul(slots as u64),
            ) as *mut sds;
            if newtokens.is_null() {
                current_block = 8233276170833294896;
                break;
            }
            tokens = newtokens;
        }
        if seplen == 1 as i32
            && *s.offset(j as isize) as i32 == *sep.offset(0 as i32 as isize) as i32
            || memcmp(
                s.offset(j as isize) as *const libc::c_void,
                sep as *const libc::c_void,
                seplen as u64,
            ) == 0 as i32
        {
            let ref mut fresh9 = *tokens.offset(elements as isize);
            *fresh9 = sdsnewlen(
                s.offset(start as isize) as *const libc::c_void,
                (j - start) as size_t,
            );
            if (*tokens.offset(elements as isize)).is_null() {
                current_block = 8233276170833294896;
                break;
            }
            elements += 1;
            elements;
            start = j + seplen;
            j = j + seplen - 1 as i32;
        }
        j += 1;
        j;
    }
    match current_block {
        15904375183555213903 => {
            let ref mut fresh10 = *tokens.offset(elements as isize);
            *fresh10 = sdsnewlen(
                s.offset(start as isize) as *const libc::c_void,
                (len - start) as size_t,
            );
            if !(*tokens.offset(elements as isize)).is_null() {
                elements += 1;
                elements;
                *count = elements;
                return tokens;
            }
        }
        _ => {}
    }
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < elements {
        sdsfree(*tokens.offset(i as isize));
        i += 1;
        i;
    }
    hi_free(tokens as *mut libc::c_void);
    *count = 0 as i32;
    return 0 as *mut sds;
}
#[no_mangle]
pub unsafe extern "C" fn sdsfreesplitres(mut tokens: *mut sds, mut count: i32) {
    if tokens.is_null() {
        return;
    }
    loop {
        let fresh11 = count;
        count = count - 1;
        if !(fresh11 != 0) {
            break;
        }
        sdsfree(*tokens.offset(count as isize));
    }
    hi_free(tokens as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn sdscatrepr(
    mut s: sds,
    mut p: *const i8,
    mut len: size_t,
) -> sds {
    s = sdscatlen(
        s,
        b"\"\0" as *const u8 as *const i8 as *const libc::c_void,
        1 as i32 as size_t,
    );
    loop {
        let fresh12 = len;
        len = len.wrapping_sub(1);
        if !(fresh12 != 0) {
            break;
        }
        match *p as i32 {
            92 | 34 => {
                s = sdscatprintf(s, b"\\%c\0" as *const u8 as *const i8, *p as i32);
            }
            10 => {
                s = sdscatlen(
                    s,
                    b"\\n\0" as *const u8 as *const i8 as *const libc::c_void,
                    2 as i32 as size_t,
                );
            }
            13 => {
                s = sdscatlen(
                    s,
                    b"\\r\0" as *const u8 as *const i8 as *const libc::c_void,
                    2 as i32 as size_t,
                );
            }
            9 => {
                s = sdscatlen(
                    s,
                    b"\\t\0" as *const u8 as *const i8 as *const libc::c_void,
                    2 as i32 as size_t,
                );
            }
            7 => {
                s = sdscatlen(
                    s,
                    b"\\a\0" as *const u8 as *const i8 as *const libc::c_void,
                    2 as i32 as size_t,
                );
            }
            8 => {
                s = sdscatlen(
                    s,
                    b"\\b\0" as *const u8 as *const i8 as *const libc::c_void,
                    2 as i32 as size_t,
                );
            }
            _ => {
                if *(*__ctype_b_loc()).offset(*p as i32 as isize) as i32
                    & _ISprint as i32 as libc::c_ushort as i32 != 0
                {
                    s = sdscatprintf(s, b"%c\0" as *const u8 as *const i8, *p as i32);
                } else {
                    s = sdscatprintf(
                        s,
                        b"\\x%02x\0" as *const u8 as *const i8,
                        *p as u8 as i32,
                    );
                }
            }
        }
        p = p.offset(1);
        p;
    }
    return sdscatlen(
        s,
        b"\"\0" as *const u8 as *const i8 as *const libc::c_void,
        1 as i32 as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn hex_digit_to_int(mut c: i8) -> i32 {
    match c as i32 {
        48 => return 0 as i32,
        49 => return 1 as i32,
        50 => return 2 as i32,
        51 => return 3 as i32,
        52 => return 4 as i32,
        53 => return 5 as i32,
        54 => return 6 as i32,
        55 => return 7 as i32,
        56 => return 8 as i32,
        57 => return 9 as i32,
        97 | 65 => return 10 as i32,
        98 | 66 => return 11 as i32,
        99 | 67 => return 12 as i32,
        100 | 68 => return 13 as i32,
        101 | 69 => return 14 as i32,
        102 | 70 => return 15 as i32,
        _ => return 0 as i32,
    };
}
#[no_mangle]
pub unsafe extern "C" fn sdssplitargs(
    mut line: *const i8,
    mut argc: *mut i32,
) -> *mut sds {
    let mut p: *const i8 = line;
    let mut current: *mut i8 = 0 as *mut i8;
    let mut vector: *mut *mut i8 = 0 as *mut *mut i8;
    *argc = 0 as i32;
    's_13: loop {
        while *p as i32 != 0
            && *(*__ctype_b_loc()).offset(*p as i32 as isize) as i32
                & _ISspace as i32 as libc::c_ushort as i32 != 0
        {
            p = p.offset(1);
            p;
        }
        if *p != 0 {
            let mut inq: i32 = 0 as i32;
            let mut insq: i32 = 0 as i32;
            let mut done: i32 = 0 as i32;
            if current.is_null() {
                current = sdsempty();
            }
            while done == 0 {
                if inq != 0 {
                    if *p as i32 == '\\' as i32
                        && *p.offset(1 as i32 as isize) as i32 == 'x' as i32
                        && *(*__ctype_b_loc())
                            .offset(*p.offset(2 as i32 as isize) as i32 as isize) as i32
                            & _ISxdigit as i32 as libc::c_ushort as i32 != 0
                        && *(*__ctype_b_loc())
                            .offset(*p.offset(3 as i32 as isize) as i32 as isize) as i32
                            & _ISxdigit as i32 as libc::c_ushort as i32 != 0
                    {
                        let mut byte: u8 = 0;
                        byte = (hex_digit_to_int(*p.offset(2 as i32 as isize))
                            * 16 as i32 + hex_digit_to_int(*p.offset(3 as i32 as isize)))
                            as u8;
                        current = sdscatlen(
                            current,
                            &mut byte as *mut u8 as *mut i8 as *const libc::c_void,
                            1 as i32 as size_t,
                        );
                        p = p.offset(3 as i32 as isize);
                    } else if *p as i32 == '\\' as i32
                        && *p.offset(1 as i32 as isize) as i32 != 0
                    {
                        let mut c: i8 = 0;
                        p = p.offset(1);
                        p;
                        match *p as i32 {
                            110 => {
                                c = '\n' as i32 as i8;
                            }
                            114 => {
                                c = '\r' as i32 as i8;
                            }
                            116 => {
                                c = '\t' as i32 as i8;
                            }
                            98 => {
                                c = '\u{8}' as i32 as i8;
                            }
                            97 => {
                                c = '\u{7}' as i32 as i8;
                            }
                            _ => {
                                c = *p;
                            }
                        }
                        current = sdscatlen(
                            current,
                            &mut c as *mut i8 as *const libc::c_void,
                            1 as i32 as size_t,
                        );
                    } else if *p as i32 == '"' as i32 {
                        if *p.offset(1 as i32 as isize) as i32 != 0
                            && *(*__ctype_b_loc())
                                .offset(*p.offset(1 as i32 as isize) as i32 as isize) as i32
                                & _ISspace as i32 as libc::c_ushort as i32 == 0
                        {
                            break 's_13;
                        }
                        done = 1 as i32;
                    } else if *p == 0 {
                        break 's_13;
                    } else {
                        current = sdscatlen(
                            current,
                            p as *const libc::c_void,
                            1 as i32 as size_t,
                        );
                    }
                } else if insq != 0 {
                    if *p as i32 == '\\' as i32
                        && *p.offset(1 as i32 as isize) as i32 == '\'' as i32
                    {
                        p = p.offset(1);
                        p;
                        current = sdscatlen(
                            current,
                            b"'\0" as *const u8 as *const i8 as *const libc::c_void,
                            1 as i32 as size_t,
                        );
                    } else if *p as i32 == '\'' as i32 {
                        if *p.offset(1 as i32 as isize) as i32 != 0
                            && *(*__ctype_b_loc())
                                .offset(*p.offset(1 as i32 as isize) as i32 as isize) as i32
                                & _ISspace as i32 as libc::c_ushort as i32 == 0
                        {
                            break 's_13;
                        }
                        done = 1 as i32;
                    } else {
                        if *p == 0 {
                            break 's_13;
                        }
                        current = sdscatlen(
                            current,
                            p as *const libc::c_void,
                            1 as i32 as size_t,
                        );
                    }
                } else {
                    match *p as i32 {
                        32 | 10 | 13 | 9 | 0 => {
                            done = 1 as i32;
                        }
                        34 => {
                            inq = 1 as i32;
                        }
                        39 => {
                            insq = 1 as i32;
                        }
                        _ => {
                            current = sdscatlen(
                                current,
                                p as *const libc::c_void,
                                1 as i32 as size_t,
                            );
                        }
                    }
                }
                if *p != 0 {
                    p = p.offset(1);
                    p;
                }
            }
            let mut new_vector: *mut *mut i8 = hi_realloc(
                vector as *mut libc::c_void,
                ((*argc + 1 as i32) as u64)
                    .wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
            ) as *mut *mut i8;
            if new_vector.is_null() {
                hi_free(vector as *mut libc::c_void);
                return 0 as *mut sds;
            }
            vector = new_vector;
            let ref mut fresh13 = *vector.offset(*argc as isize);
            *fresh13 = current;
            *argc += 1;
            *argc;
            current = 0 as *mut i8;
        } else {
            if vector.is_null() {
                vector = hi_malloc(::core::mem::size_of::<*mut libc::c_void>() as u64)
                    as *mut *mut i8;
            }
            return vector;
        }
    }
    loop {
        let fresh14 = *argc;
        *argc = *argc - 1;
        if !(fresh14 != 0) {
            break;
        }
        sdsfree(*vector.offset(*argc as isize));
    }
    hi_free(vector as *mut libc::c_void);
    if !current.is_null() {
        sdsfree(current);
    }
    *argc = 0 as i32;
    return 0 as *mut sds;
}
#[no_mangle]
pub unsafe extern "C" fn sdsmapchars(
    mut s: sds,
    mut from: *const i8,
    mut to: *const i8,
    mut setlen: size_t,
) -> sds {
    let mut j: size_t = 0;
    let mut i: size_t = 0;
    let mut l: size_t = sdslen(s);
    j = 0 as i32 as size_t;
    while j < l {
        i = 0 as i32 as size_t;
        while i < setlen {
            if *s.offset(j as isize) as i32 == *from.offset(i as isize) as i32 {
                *s.offset(j as isize) = *to.offset(i as isize);
                break;
            } else {
                i = i.wrapping_add(1);
                i;
            }
        }
        j = j.wrapping_add(1);
        j;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn sdsjoin(
    mut argv: *mut *mut i8,
    mut argc: i32,
    mut sep: *mut i8,
) -> sds {
    let mut join: sds = sdsempty();
    let mut j: i32 = 0;
    j = 0 as i32;
    while j < argc {
        join = sdscat(join, *argv.offset(j as isize));
        if j != argc - 1 as i32 {
            join = sdscat(join, sep);
        }
        j += 1;
        j;
    }
    return join;
}
#[no_mangle]
pub unsafe extern "C" fn sdsjoinsds(
    mut argv: *mut sds,
    mut argc: i32,
    mut sep: *const i8,
    mut seplen: size_t,
) -> sds {
    let mut join: sds = sdsempty();
    let mut j: i32 = 0;
    j = 0 as i32;
    while j < argc {
        join = sdscatsds(join, *argv.offset(j as isize));
        if j != argc - 1 as i32 {
            join = sdscatlen(join, sep as *const libc::c_void, seplen);
        }
        j += 1;
        j;
    }
    return join;
}
#[no_mangle]
pub unsafe extern "C" fn sds_malloc(mut size: size_t) -> *mut libc::c_void {
    return hi_malloc(size);
}
#[no_mangle]
pub unsafe extern "C" fn sds_realloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    return hi_realloc(ptr, size);
}
#[no_mangle]
pub unsafe extern "C" fn sds_free(mut ptr: *mut libc::c_void) {
    hi_free(ptr);
}