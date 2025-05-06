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
    fn memchr(_: *const libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn strtod(__nptr: *const i8, __endptr: *mut *mut i8) -> libc::c_double;
    fn strcasecmp(__s1: *const i8, __s2: *const i8) -> i32;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __finitef(_: libc::c_float) -> i32;
    fn __finitel(_: f128::f128) -> i32;
    fn __finite(_: libc::c_double) -> i32;
    static mut hiredisAllocFns: hiredisAllocFuncs;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn sdscatlen(s: sds, t: *const libc::c_void, len: size_t) -> sds;
    fn sdsrange(s: sds, start: ssize_t, end: ssize_t) -> i32;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __ssize_t = i64;
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
pub type uintptr_t = u64;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisReadTask {
    pub type_0: i32,
    pub elements: libc::c_longlong,
    pub idx: i32,
    pub obj: *mut libc::c_void,
    pub parent: *mut redisReadTask,
    pub privdata: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisReplyObjectFunctions {
    pub createString: Option<
        unsafe extern "C" fn(*const redisReadTask, *mut i8, size_t) -> *mut libc::c_void,
    >,
    pub createArray: Option<
        unsafe extern "C" fn(*const redisReadTask, size_t) -> *mut libc::c_void,
    >,
    pub createInteger: Option<
        unsafe extern "C" fn(*const redisReadTask, libc::c_longlong) -> *mut libc::c_void,
    >,
    pub createDouble: Option<
        unsafe extern "C" fn(
            *const redisReadTask,
            libc::c_double,
            *mut i8,
            size_t,
        ) -> *mut libc::c_void,
    >,
    pub createNil: Option<
        unsafe extern "C" fn(*const redisReadTask) -> *mut libc::c_void,
    >,
    pub createBool: Option<
        unsafe extern "C" fn(*const redisReadTask, i32) -> *mut libc::c_void,
    >,
    pub freeObject: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisReader {
    pub err: i32,
    pub errstr: [i8; 128],
    pub buf: *mut i8,
    pub pos: size_t,
    pub len: size_t,
    pub maxbuf: size_t,
    pub maxelements: libc::c_longlong,
    pub task: *mut *mut redisReadTask,
    pub tasks: i32,
    pub ridx: i32,
    pub reply: *mut libc::c_void,
    pub fn_0: *mut redisReplyObjectFunctions,
    pub privdata: *mut libc::c_void,
}
pub type sds = *mut i8;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr64 {
    pub len: uint64_t,
    pub alloc: uint64_t,
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
pub struct sdshdr16 {
    pub len: uint16_t,
    pub alloc: uint16_t,
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
#[inline]
unsafe extern "C" fn hi_free(mut ptr: *mut libc::c_void) {
    (hiredisAllocFns.freeFn).expect("non-null function pointer")(ptr);
}
#[inline]
unsafe extern "C" fn hi_calloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    if (18446744073709551615 as u64).wrapping_div(size) < nmemb {
        return 0 as *mut libc::c_void;
    }
    return (hiredisAllocFns.callocFn).expect("non-null function pointer")(nmemb, size);
}
#[inline]
unsafe extern "C" fn hi_realloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    return (hiredisAllocFns.reallocFn).expect("non-null function pointer")(ptr, size);
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
unsafe extern "C" fn __redisReaderSetError(
    mut r: *mut redisReader,
    mut type_0: i32,
    mut str: *const i8,
) {
    let mut len: size_t = 0;
    if !((*r).reply).is_null() && !((*r).fn_0).is_null()
        && ((*(*r).fn_0).freeObject).is_some()
    {
        ((*(*r).fn_0).freeObject).expect("non-null function pointer")((*r).reply);
        (*r).reply = 0 as *mut libc::c_void;
    }
    sdsfree((*r).buf);
    (*r).buf = 0 as *mut i8;
    (*r).len = 0 as i32 as size_t;
    (*r).pos = (*r).len;
    (*r).ridx = -(1 as i32);
    (*r).err = type_0;
    len = strlen(str);
    len = if len
        < (::core::mem::size_of::<[i8; 128]>() as u64).wrapping_sub(1 as i32 as u64)
    {
        len
    } else {
        (::core::mem::size_of::<[i8; 128]>() as u64).wrapping_sub(1 as i32 as u64)
    };
    memcpy(
        ((*r).errstr).as_mut_ptr() as *mut libc::c_void,
        str as *const libc::c_void,
        len,
    );
    (*r).errstr[len as usize] = '\0' as i32 as i8;
}
unsafe extern "C" fn chrtos(mut buf: *mut i8, mut size: size_t, mut byte: i8) -> size_t {
    let mut len: size_t = 0 as i32 as size_t;
    match byte as i32 {
        92 | 34 => {
            len = snprintf(
                buf,
                size,
                b"\"\\%c\"\0" as *const u8 as *const i8,
                byte as i32,
            ) as size_t;
        }
        10 => {
            len = snprintf(buf, size, b"\"\\n\"\0" as *const u8 as *const i8) as size_t;
        }
        13 => {
            len = snprintf(buf, size, b"\"\\r\"\0" as *const u8 as *const i8) as size_t;
        }
        9 => {
            len = snprintf(buf, size, b"\"\\t\"\0" as *const u8 as *const i8) as size_t;
        }
        7 => {
            len = snprintf(buf, size, b"\"\\a\"\0" as *const u8 as *const i8) as size_t;
        }
        8 => {
            len = snprintf(buf, size, b"\"\\b\"\0" as *const u8 as *const i8) as size_t;
        }
        _ => {
            if *(*__ctype_b_loc()).offset(byte as i32 as isize) as i32
                & _ISprint as i32 as libc::c_ushort as i32 != 0
            {
                len = snprintf(
                    buf,
                    size,
                    b"\"%c\"\0" as *const u8 as *const i8,
                    byte as i32,
                ) as size_t;
            } else {
                len = snprintf(
                    buf,
                    size,
                    b"\"\\x%02x\"\0" as *const u8 as *const i8,
                    byte as u8 as i32,
                ) as size_t;
            }
        }
    }
    return len;
}
unsafe extern "C" fn __redisReaderSetErrorProtocolByte(
    mut r: *mut redisReader,
    mut byte: i8,
) {
    let mut cbuf: [i8; 8] = [0; 8];
    let mut sbuf: [i8; 128] = [0; 128];
    chrtos(cbuf.as_mut_ptr(), ::core::mem::size_of::<[i8; 8]>() as u64, byte);
    snprintf(
        sbuf.as_mut_ptr(),
        ::core::mem::size_of::<[i8; 128]>() as u64,
        b"Protocol error, got %s as reply type byte\0" as *const u8 as *const i8,
        cbuf.as_mut_ptr(),
    );
    __redisReaderSetError(r, 4 as i32, sbuf.as_mut_ptr());
}
unsafe extern "C" fn __redisReaderSetErrorOOM(mut r: *mut redisReader) {
    __redisReaderSetError(r, 5 as i32, b"Out of memory\0" as *const u8 as *const i8);
}
unsafe extern "C" fn readBytes(mut r: *mut redisReader, mut bytes: u32) -> *mut i8 {
    let mut p: *mut i8 = 0 as *mut i8;
    if ((*r).len).wrapping_sub((*r).pos) >= bytes as u64 {
        p = ((*r).buf).offset((*r).pos as isize);
        (*r).pos = ((*r).pos as u64).wrapping_add(bytes as u64) as size_t as size_t;
        return p;
    }
    return 0 as *mut i8;
}
unsafe extern "C" fn seekNewline(mut s: *mut i8, mut len: size_t) -> *mut i8 {
    let mut ret: *mut i8 = 0 as *mut i8;
    if len < 2 as i32 as u64 {
        return 0 as *mut i8;
    }
    len = len.wrapping_sub(1);
    len;
    loop {
        ret = memchr(s as *const libc::c_void, '\r' as i32, len) as *mut i8;
        if ret.is_null() {
            break;
        }
        if *ret.offset(1 as i32 as isize) as i32 == '\n' as i32 {
            break;
        }
        ret = ret.offset(1);
        ret;
        len = (len as u64).wrapping_sub(ret.offset_from(s) as i64 as u64) as size_t
            as size_t;
        s = ret;
    }
    return ret;
}
unsafe extern "C" fn string2ll(
    mut s: *const i8,
    mut slen: size_t,
    mut value: *mut libc::c_longlong,
) -> i32 {
    let mut p: *const i8 = s;
    let mut plen: size_t = 0 as i32 as size_t;
    let mut negative: i32 = 0 as i32;
    let mut v: libc::c_ulonglong = 0;
    if plen == slen {
        return -(1 as i32);
    }
    if slen == 1 as i32 as u64 && *p.offset(0 as i32 as isize) as i32 == '0' as i32 {
        if !value.is_null() {
            *value = 0 as i32 as libc::c_longlong;
        }
        return 0 as i32;
    }
    if *p.offset(0 as i32 as isize) as i32 == '-' as i32 {
        negative = 1 as i32;
        p = p.offset(1);
        p;
        plen = plen.wrapping_add(1);
        plen;
        if plen == slen {
            return -(1 as i32);
        }
    }
    if *p.offset(0 as i32 as isize) as i32 >= '1' as i32
        && *p.offset(0 as i32 as isize) as i32 <= '9' as i32
    {
        v = (*p.offset(0 as i32 as isize) as i32 - '0' as i32) as libc::c_ulonglong;
        p = p.offset(1);
        p;
        plen = plen.wrapping_add(1);
        plen;
    } else if *p.offset(0 as i32 as isize) as i32 == '0' as i32
        && slen == 1 as i32 as u64
    {
        *value = 0 as i32 as libc::c_longlong;
        return 0 as i32;
    } else {
        return -(1 as i32)
    }
    while plen < slen && *p.offset(0 as i32 as isize) as i32 >= '0' as i32
        && *p.offset(0 as i32 as isize) as i32 <= '9' as i32
    {
        if v
            > (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                .wrapping_mul(2 as libc::c_ulonglong)
                .wrapping_add(1 as libc::c_ulonglong)
                .wrapping_div(10 as i32 as libc::c_ulonglong)
        {
            return -(1 as i32);
        }
        v = v.wrapping_mul(10 as i32 as libc::c_ulonglong);
        if v
            > (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                .wrapping_mul(2 as libc::c_ulonglong)
                .wrapping_add(1 as libc::c_ulonglong)
                .wrapping_sub(
                    (*p.offset(0 as i32 as isize) as i32 - '0' as i32)
                        as libc::c_ulonglong,
                )
        {
            return -(1 as i32);
        }
        v = v
            .wrapping_add(
                (*p.offset(0 as i32 as isize) as i32 - '0' as i32) as libc::c_ulonglong,
            );
        p = p.offset(1);
        p;
        plen = plen.wrapping_add(1);
        plen;
    }
    if plen < slen {
        return -(1 as i32);
    }
    if negative != 0 {
        if v
            > (-(-(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong
                + 1 as i32 as libc::c_longlong) as libc::c_ulonglong)
                .wrapping_add(1 as i32 as libc::c_ulonglong)
        {
            return -(1 as i32);
        }
        if !value.is_null() {
            *value = v.wrapping_neg() as libc::c_longlong;
        }
    } else {
        if v > 9223372036854775807 as libc::c_longlong as libc::c_ulonglong {
            return -(1 as i32);
        }
        if !value.is_null() {
            *value = v as libc::c_longlong;
        }
    }
    return 0 as i32;
}
unsafe extern "C" fn readLine(mut r: *mut redisReader, mut _len: *mut i32) -> *mut i8 {
    let mut p: *mut i8 = 0 as *mut i8;
    let mut s: *mut i8 = 0 as *mut i8;
    let mut len: i32 = 0;
    p = ((*r).buf).offset((*r).pos as isize);
    s = seekNewline(p, ((*r).len).wrapping_sub((*r).pos));
    if !s.is_null() {
        len = s.offset_from(((*r).buf).offset((*r).pos as isize)) as i64 as i32;
        (*r).pos = ((*r).pos as u64).wrapping_add((len + 2 as i32) as u64) as size_t
            as size_t;
        if !_len.is_null() {
            *_len = len;
        }
        return p;
    }
    return 0 as *mut i8;
}
unsafe extern "C" fn moveToNextTask(mut r: *mut redisReader) {
    let mut cur: *mut redisReadTask = 0 as *mut redisReadTask;
    let mut prv: *mut redisReadTask = 0 as *mut redisReadTask;
    while (*r).ridx >= 0 as i32 {
        if (*r).ridx == 0 as i32 {
            (*r).ridx -= 1;
            (*r).ridx;
            return;
        }
        cur = *((*r).task).offset((*r).ridx as isize);
        prv = *((*r).task).offset(((*r).ridx - 1 as i32) as isize);
        if (*prv).type_0 == 2 as i32 || (*prv).type_0 == 9 as i32
            || (*prv).type_0 == 11 as i32 || (*prv).type_0 == 10 as i32
            || (*prv).type_0 == 12 as i32
        {} else {
            __assert_fail(
                b"prv->type == REDIS_REPLY_ARRAY || prv->type == REDIS_REPLY_MAP || prv->type == REDIS_REPLY_ATTR || prv->type == REDIS_REPLY_SET || prv->type == REDIS_REPLY_PUSH\0"
                    as *const u8 as *const i8,
                b"read.c\0" as *const u8 as *const i8,
                255 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[i8; 35],
                >(b"void moveToNextTask(redisReader *)\0"))
                    .as_ptr(),
            );
        };
        if (*cur).idx as libc::c_longlong
            == (*prv).elements - 1 as i32 as libc::c_longlong
        {
            (*r).ridx -= 1;
            (*r).ridx;
        } else {
            if ((*cur).idx as libc::c_longlong) < (*prv).elements {} else {
                __assert_fail(
                    b"cur->idx < prv->elements\0" as *const u8 as *const i8,
                    b"read.c\0" as *const u8 as *const i8,
                    260 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 35],
                        &[i8; 35],
                    >(b"void moveToNextTask(redisReader *)\0"))
                        .as_ptr(),
                );
            };
            (*cur).type_0 = -(1 as i32);
            (*cur).elements = -(1 as i32) as libc::c_longlong;
            (*cur).idx += 1;
            (*cur).idx;
            return;
        }
    }
}
unsafe extern "C" fn processLineItem(mut r: *mut redisReader) -> i32 {
    let mut cur: *mut redisReadTask = *((*r).task).offset((*r).ridx as isize);
    let mut obj: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut len: i32 = 0;
    p = readLine(r, &mut len);
    if !p.is_null() {
        if (*cur).type_0 == 3 as i32 {
            let mut v: libc::c_longlong = 0;
            if string2ll(p, len as size_t, &mut v) == -(1 as i32) {
                __redisReaderSetError(
                    r,
                    4 as i32,
                    b"Bad integer value\0" as *const u8 as *const i8,
                );
                return -(1 as i32);
            }
            if !((*r).fn_0).is_null() && ((*(*r).fn_0).createInteger).is_some() {
                obj = ((*(*r).fn_0).createInteger)
                    .expect("non-null function pointer")(cur, v);
            } else {
                obj = 3 as i32 as *mut libc::c_void;
            }
        } else if (*cur).type_0 == 7 as i32 {
            let mut buf: [i8; 326] = [0; 326];
            let mut eptr: *mut i8 = 0 as *mut i8;
            let mut d: libc::c_double = 0.;
            if len as size_t >= ::core::mem::size_of::<[i8; 326]>() as u64 {
                __redisReaderSetError(
                    r,
                    4 as i32,
                    b"Double value is too large\0" as *const u8 as *const i8,
                );
                return -(1 as i32);
            }
            memcpy(
                buf.as_mut_ptr() as *mut libc::c_void,
                p as *const libc::c_void,
                len as u64,
            );
            buf[len as usize] = '\0' as i32 as i8;
            if len == 3 as i32
                && strcasecmp(buf.as_mut_ptr(), b"inf\0" as *const u8 as *const i8)
                    == 0 as i32
            {
                d = ::core::f32::INFINITY as libc::c_double;
            } else if len == 4 as i32
                && strcasecmp(buf.as_mut_ptr(), b"-inf\0" as *const u8 as *const i8)
                    == 0 as i32
            {
                d = -::core::f32::INFINITY as libc::c_double;
            } else if len == 3 as i32
                && strcasecmp(buf.as_mut_ptr(), b"nan\0" as *const u8 as *const i8)
                    == 0 as i32
                || len == 4 as i32
                    && strcasecmp(buf.as_mut_ptr(), b"-nan\0" as *const u8 as *const i8)
                        == 0 as i32
            {
                d = ::core::f32::NAN as libc::c_double;
            } else {
                d = strtod(buf.as_mut_ptr(), &mut eptr);
                if buf[0 as i32 as usize] as i32 == '\0' as i32
                    || eptr != &mut *buf.as_mut_ptr().offset(len as isize) as *mut i8
                    || (if ::core::mem::size_of::<libc::c_double>() as u64
                        == ::core::mem::size_of::<libc::c_float>() as u64
                    {
                        __finitef(d as libc::c_float)
                    } else {
                        (if ::core::mem::size_of::<libc::c_double>() as u64
                            == ::core::mem::size_of::<libc::c_double>() as u64
                        {
                            __finite(d)
                        } else {
                            __finitel(f128::f128::new(d))
                        })
                    }) == 0
                {
                    __redisReaderSetError(
                        r,
                        4 as i32,
                        b"Bad double value\0" as *const u8 as *const i8,
                    );
                    return -(1 as i32);
                }
            }
            if !((*r).fn_0).is_null() && ((*(*r).fn_0).createDouble).is_some() {
                obj = ((*(*r).fn_0).createDouble)
                    .expect(
                        "non-null function pointer",
                    )(cur, d, buf.as_mut_ptr(), len as size_t);
            } else {
                obj = 7 as i32 as *mut libc::c_void;
            }
        } else if (*cur).type_0 == 4 as i32 {
            if len != 0 as i32 {
                __redisReaderSetError(
                    r,
                    4 as i32,
                    b"Bad nil value\0" as *const u8 as *const i8,
                );
                return -(1 as i32);
            }
            if !((*r).fn_0).is_null() && ((*(*r).fn_0).createNil).is_some() {
                obj = ((*(*r).fn_0).createNil).expect("non-null function pointer")(cur);
            } else {
                obj = 4 as i32 as *mut libc::c_void;
            }
        } else if (*cur).type_0 == 8 as i32 {
            let mut bval: i32 = 0;
            if len != 1 as i32
                || (strchr(
                    b"tTfF\0" as *const u8 as *const i8,
                    *p.offset(0 as i32 as isize) as i32,
                ))
                    .is_null()
            {
                __redisReaderSetError(
                    r,
                    4 as i32,
                    b"Bad bool value\0" as *const u8 as *const i8,
                );
                return -(1 as i32);
            }
            bval = (*p.offset(0 as i32 as isize) as i32 == 't' as i32
                || *p.offset(0 as i32 as isize) as i32 == 'T' as i32) as i32;
            if !((*r).fn_0).is_null() && ((*(*r).fn_0).createBool).is_some() {
                obj = ((*(*r).fn_0).createBool)
                    .expect("non-null function pointer")(cur, bval);
            } else {
                obj = 8 as i32 as *mut libc::c_void;
            }
        } else if (*cur).type_0 == 13 as i32 {
            let mut i: i32 = 0 as i32;
            while i < len {
                if !(i == 0 as i32 && *p.offset(0 as i32 as isize) as i32 == '-' as i32)
                {
                    if (*p.offset(i as isize) as i32) < '0' as i32
                        || *p.offset(i as isize) as i32 > '9' as i32
                    {
                        __redisReaderSetError(
                            r,
                            4 as i32,
                            b"Bad bignum value\0" as *const u8 as *const i8,
                        );
                        return -(1 as i32);
                    }
                }
                i += 1;
                i;
            }
            if !((*r).fn_0).is_null() && ((*(*r).fn_0).createString).is_some() {
                obj = ((*(*r).fn_0).createString)
                    .expect("non-null function pointer")(cur, p, len as size_t);
            } else {
                obj = 13 as i32 as *mut libc::c_void;
            }
        } else {
            let mut i_0: i32 = 0 as i32;
            while i_0 < len {
                if *p.offset(i_0 as isize) as i32 == '\r' as i32
                    || *p.offset(i_0 as isize) as i32 == '\n' as i32
                {
                    __redisReaderSetError(
                        r,
                        4 as i32,
                        b"Bad simple string value\0" as *const u8 as *const i8,
                    );
                    return -(1 as i32);
                }
                i_0 += 1;
                i_0;
            }
            if !((*r).fn_0).is_null() && ((*(*r).fn_0).createString).is_some() {
                obj = ((*(*r).fn_0).createString)
                    .expect("non-null function pointer")(cur, p, len as size_t);
            } else {
                obj = (*cur).type_0 as uintptr_t as *mut libc::c_void;
            }
        }
        if obj.is_null() {
            __redisReaderSetErrorOOM(r);
            return -(1 as i32);
        }
        if (*r).ridx == 0 as i32 {
            (*r).reply = obj;
        }
        moveToNextTask(r);
        return 0 as i32;
    }
    return -(1 as i32);
}
unsafe extern "C" fn processBulkItem(mut r: *mut redisReader) -> i32 {
    let mut cur: *mut redisReadTask = *((*r).task).offset((*r).ridx as isize);
    let mut obj: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut s: *mut i8 = 0 as *mut i8;
    let mut len: libc::c_longlong = 0;
    let mut bytelen: u64 = 0;
    let mut success: i32 = 0 as i32;
    p = ((*r).buf).offset((*r).pos as isize);
    s = seekNewline(p, ((*r).len).wrapping_sub((*r).pos));
    if !s.is_null() {
        p = ((*r).buf).offset((*r).pos as isize);
        bytelen = (s.offset_from(((*r).buf).offset((*r).pos as isize)) as i64
            + 2 as i32 as i64) as u64;
        if string2ll(p, bytelen.wrapping_sub(2 as i32 as u64), &mut len) == -(1 as i32) {
            __redisReaderSetError(
                r,
                4 as i32,
                b"Bad bulk string length\0" as *const u8 as *const i8,
            );
            return -(1 as i32);
        }
        if len < -(1 as i32) as libc::c_longlong
            || 9223372036854775807 as libc::c_longlong as libc::c_ulonglong
                > 18446744073709551615 as u64 as libc::c_ulonglong
                && len > 18446744073709551615 as u64 as libc::c_longlong
        {
            __redisReaderSetError(
                r,
                4 as i32,
                b"Bulk string length out of range\0" as *const u8 as *const i8,
            );
            return -(1 as i32);
        }
        if len == -(1 as i32) as libc::c_longlong {
            if !((*r).fn_0).is_null() && ((*(*r).fn_0).createNil).is_some() {
                obj = ((*(*r).fn_0).createNil).expect("non-null function pointer")(cur);
            } else {
                obj = 4 as i32 as *mut libc::c_void;
            }
            success = 1 as i32;
        } else {
            bytelen = (bytelen as libc::c_ulonglong)
                .wrapping_add((len + 2 as i32 as libc::c_longlong) as libc::c_ulonglong)
                as u64 as u64;
            if ((*r).pos).wrapping_add(bytelen) <= (*r).len {
                if (*cur).type_0 == 14 as i32 && len < 4 as i32 as libc::c_longlong
                    || (*cur).type_0 == 14 as i32
                        && *s.offset(5 as i32 as isize) as i32 != ':' as i32
                {
                    __redisReaderSetError(
                        r,
                        4 as i32,
                        b"Verbatim string 4 bytes of content type are missing or incorrectly encoded.\0"
                            as *const u8 as *const i8,
                    );
                    return -(1 as i32);
                }
                if !((*r).fn_0).is_null() && ((*(*r).fn_0).createString).is_some() {
                    obj = ((*(*r).fn_0).createString)
                        .expect(
                            "non-null function pointer",
                        )(cur, s.offset(2 as i32 as isize), len as size_t);
                } else {
                    obj = (*cur).type_0 as uintptr_t as *mut libc::c_void;
                }
                success = 1 as i32;
            }
        }
        if success != 0 {
            if obj.is_null() {
                __redisReaderSetErrorOOM(r);
                return -(1 as i32);
            }
            (*r).pos = ((*r).pos as u64).wrapping_add(bytelen) as size_t as size_t;
            if (*r).ridx == 0 as i32 {
                (*r).reply = obj;
            }
            moveToNextTask(r);
            return 0 as i32;
        }
    }
    return -(1 as i32);
}
unsafe extern "C" fn redisReaderGrow(mut r: *mut redisReader) -> i32 {
    let mut current_block: u64;
    let mut aux: *mut *mut redisReadTask = 0 as *mut *mut redisReadTask;
    let mut newlen: i32 = 0;
    newlen = (*r).tasks + 9 as i32;
    aux = hi_realloc(
        (*r).task as *mut libc::c_void,
        (::core::mem::size_of::<*mut redisReadTask>() as u64).wrapping_mul(newlen as u64),
    ) as *mut *mut redisReadTask;
    if !aux.is_null() {
        (*r).task = aux;
        loop {
            if !((*r).tasks < newlen) {
                current_block = 10879442775620481940;
                break;
            }
            let ref mut fresh0 = *((*r).task).offset((*r).tasks as isize);
            *fresh0 = hi_calloc(
                1 as i32 as size_t,
                ::core::mem::size_of::<redisReadTask>() as u64,
            ) as *mut redisReadTask;
            if (*((*r).task).offset((*r).tasks as isize)).is_null() {
                current_block = 2283036175170833903;
                break;
            }
            (*r).tasks += 1;
            (*r).tasks;
        }
        match current_block {
            2283036175170833903 => {}
            _ => return 0 as i32,
        }
    }
    __redisReaderSetErrorOOM(r);
    return -(1 as i32);
}
unsafe extern "C" fn processAggregateItem(mut r: *mut redisReader) -> i32 {
    let mut cur: *mut redisReadTask = *((*r).task).offset((*r).ridx as isize);
    let mut obj: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut elements: libc::c_longlong = 0;
    let mut root: i32 = 0 as i32;
    let mut len: i32 = 0;
    if (*r).ridx == (*r).tasks - 1 as i32 {
        if redisReaderGrow(r) == -(1 as i32) {
            return -(1 as i32);
        }
    }
    p = readLine(r, &mut len);
    if !p.is_null() {
        if string2ll(p, len as size_t, &mut elements) == -(1 as i32) {
            __redisReaderSetError(
                r,
                4 as i32,
                b"Bad multi-bulk length\0" as *const u8 as *const i8,
            );
            return -(1 as i32);
        }
        root = ((*r).ridx == 0 as i32) as i32;
        if elements < -(1 as i32) as libc::c_longlong
            || 9223372036854775807 as libc::c_longlong as libc::c_ulonglong
                > 18446744073709551615 as u64 as libc::c_ulonglong
                && elements as libc::c_ulonglong
                    > 18446744073709551615 as u64 as libc::c_ulonglong
            || (*r).maxelements > 0 as i32 as libc::c_longlong
                && elements > (*r).maxelements
        {
            __redisReaderSetError(
                r,
                4 as i32,
                b"Multi-bulk length out of range\0" as *const u8 as *const i8,
            );
            return -(1 as i32);
        }
        if elements == -(1 as i32) as libc::c_longlong {
            if !((*r).fn_0).is_null() && ((*(*r).fn_0).createNil).is_some() {
                obj = ((*(*r).fn_0).createNil).expect("non-null function pointer")(cur);
            } else {
                obj = 4 as i32 as *mut libc::c_void;
            }
            if obj.is_null() {
                __redisReaderSetErrorOOM(r);
                return -(1 as i32);
            }
            moveToNextTask(r);
        } else {
            if (*cur).type_0 == 9 as i32 || (*cur).type_0 == 11 as i32 {
                elements *= 2 as i32 as libc::c_longlong;
            }
            if !((*r).fn_0).is_null() && ((*(*r).fn_0).createArray).is_some() {
                obj = ((*(*r).fn_0).createArray)
                    .expect("non-null function pointer")(cur, elements as size_t);
            } else {
                obj = (*cur).type_0 as uintptr_t as *mut libc::c_void;
            }
            if obj.is_null() {
                __redisReaderSetErrorOOM(r);
                return -(1 as i32);
            }
            if elements > 0 as i32 as libc::c_longlong {
                (*cur).elements = elements;
                (*cur).obj = obj;
                (*r).ridx += 1;
                (*r).ridx;
                (**((*r).task).offset((*r).ridx as isize)).type_0 = -(1 as i32);
                (**((*r).task).offset((*r).ridx as isize)).elements = -(1 as i32)
                    as libc::c_longlong;
                (**((*r).task).offset((*r).ridx as isize)).idx = 0 as i32;
                let ref mut fresh1 = (**((*r).task).offset((*r).ridx as isize)).obj;
                *fresh1 = 0 as *mut libc::c_void;
                let ref mut fresh2 = (**((*r).task).offset((*r).ridx as isize)).parent;
                *fresh2 = cur;
                let ref mut fresh3 = (**((*r).task).offset((*r).ridx as isize)).privdata;
                *fresh3 = (*r).privdata;
            } else {
                moveToNextTask(r);
            }
        }
        if root != 0 {
            (*r).reply = obj;
        }
        return 0 as i32;
    }
    return -(1 as i32);
}
unsafe extern "C" fn processItem(mut r: *mut redisReader) -> i32 {
    let mut cur: *mut redisReadTask = *((*r).task).offset((*r).ridx as isize);
    let mut p: *mut i8 = 0 as *mut i8;
    if (*cur).type_0 < 0 as i32 {
        p = readBytes(r, 1 as i32 as u32);
        if !p.is_null() {
            match *p.offset(0 as i32 as isize) as i32 {
                45 => {
                    (*cur).type_0 = 6 as i32;
                }
                43 => {
                    (*cur).type_0 = 5 as i32;
                }
                58 => {
                    (*cur).type_0 = 3 as i32;
                }
                44 => {
                    (*cur).type_0 = 7 as i32;
                }
                95 => {
                    (*cur).type_0 = 4 as i32;
                }
                36 => {
                    (*cur).type_0 = 1 as i32;
                }
                42 => {
                    (*cur).type_0 = 2 as i32;
                }
                37 => {
                    (*cur).type_0 = 9 as i32;
                }
                124 => {
                    (*cur).type_0 = 11 as i32;
                }
                126 => {
                    (*cur).type_0 = 10 as i32;
                }
                35 => {
                    (*cur).type_0 = 8 as i32;
                }
                61 => {
                    (*cur).type_0 = 14 as i32;
                }
                62 => {
                    (*cur).type_0 = 12 as i32;
                }
                40 => {
                    (*cur).type_0 = 13 as i32;
                }
                _ => {
                    __redisReaderSetErrorProtocolByte(r, *p);
                    return -(1 as i32);
                }
            }
        } else {
            return -(1 as i32)
        }
    }
    match (*cur).type_0 {
        6 | 5 | 3 | 7 | 4 | 8 | 13 => return processLineItem(r),
        1 | 14 => return processBulkItem(r),
        2 | 9 | 11 | 10 | 12 => return processAggregateItem(r),
        _ => {
            if !(0 as *mut libc::c_void).is_null() {} else {
                __assert_fail(
                    b"NULL\0" as *const u8 as *const i8,
                    b"read.c\0" as *const u8 as *const i8,
                    654 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 31],
                        &[i8; 31],
                    >(b"int processItem(redisReader *)\0"))
                        .as_ptr(),
                );
            };
            return -(1 as i32);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn redisReaderCreateWithFunctions(
    mut fn_0: *mut redisReplyObjectFunctions,
) -> *mut redisReader {
    let mut current_block: u64;
    let mut r: *mut redisReader = 0 as *mut redisReader;
    r = hi_calloc(1 as i32 as size_t, ::core::mem::size_of::<redisReader>() as u64)
        as *mut redisReader;
    if r.is_null() {
        return 0 as *mut redisReader;
    }
    (*r).buf = sdsempty();
    if !((*r).buf).is_null() {
        (*r).task = hi_calloc(
            9 as i32 as size_t,
            ::core::mem::size_of::<*mut redisReadTask>() as u64,
        ) as *mut *mut redisReadTask;
        if !((*r).task).is_null() {
            loop {
                if !((*r).tasks < 9 as i32) {
                    current_block = 5720623009719927633;
                    break;
                }
                let ref mut fresh4 = *((*r).task).offset((*r).tasks as isize);
                *fresh4 = hi_calloc(
                    1 as i32 as size_t,
                    ::core::mem::size_of::<redisReadTask>() as u64,
                ) as *mut redisReadTask;
                if (*((*r).task).offset((*r).tasks as isize)).is_null() {
                    current_block = 11487261594761666878;
                    break;
                }
                (*r).tasks += 1;
                (*r).tasks;
            }
            match current_block {
                11487261594761666878 => {}
                _ => {
                    (*r).fn_0 = fn_0;
                    (*r).maxbuf = (1024 as i32 * 16 as i32) as size_t;
                    (*r).maxelements = ((1 as libc::c_longlong) << 32 as i32)
                        - 1 as i32 as libc::c_longlong;
                    (*r).ridx = -(1 as i32);
                    return r;
                }
            }
        }
    }
    redisReaderFree(r);
    return 0 as *mut redisReader;
}
#[no_mangle]
pub unsafe extern "C" fn redisReaderFree(mut r: *mut redisReader) {
    if r.is_null() {
        return;
    }
    if !((*r).reply).is_null() && !((*r).fn_0).is_null()
        && ((*(*r).fn_0).freeObject).is_some()
    {
        ((*(*r).fn_0).freeObject).expect("non-null function pointer")((*r).reply);
    }
    if !((*r).task).is_null() {
        let mut i: i32 = 0 as i32;
        while i < (*r).tasks {
            hi_free(*((*r).task).offset(i as isize) as *mut libc::c_void);
            i += 1;
            i;
        }
        hi_free((*r).task as *mut libc::c_void);
    }
    sdsfree((*r).buf);
    hi_free(r as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn redisReaderFeed(
    mut r: *mut redisReader,
    mut buf: *const i8,
    mut len: size_t,
) -> i32 {
    let mut current_block: u64;
    let mut newbuf: sds = 0 as *mut i8;
    if (*r).err != 0 {
        return -(1 as i32);
    }
    if !buf.is_null() && len >= 1 as i32 as u64 {
        if (*r).len == 0 as i32 as u64 && (*r).maxbuf != 0 as i32 as u64
            && sdsavail((*r).buf) > (*r).maxbuf
        {
            sdsfree((*r).buf);
            (*r).buf = sdsempty();
            if ((*r).buf).is_null() {
                current_block = 4293578962055144187;
            } else {
                (*r).pos = 0 as i32 as size_t;
                current_block = 6483416627284290920;
            }
        } else {
            current_block = 6483416627284290920;
        }
        match current_block {
            6483416627284290920 => {
                newbuf = sdscatlen((*r).buf, buf as *const libc::c_void, len);
                if newbuf.is_null() {
                    current_block = 4293578962055144187;
                } else {
                    (*r).buf = newbuf;
                    (*r).len = sdslen((*r).buf);
                    current_block = 11812396948646013369;
                }
            }
            _ => {}
        }
        match current_block {
            11812396948646013369 => {}
            _ => {
                __redisReaderSetErrorOOM(r);
                return -(1 as i32);
            }
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn redisReaderGetReply(
    mut r: *mut redisReader,
    mut reply: *mut *mut libc::c_void,
) -> i32 {
    if !reply.is_null() {
        *reply = 0 as *mut libc::c_void;
    }
    if (*r).err != 0 {
        return -(1 as i32);
    }
    if (*r).len == 0 as i32 as u64 {
        return 0 as i32;
    }
    if (*r).ridx == -(1 as i32) {
        (**((*r).task).offset(0 as i32 as isize)).type_0 = -(1 as i32);
        (**((*r).task).offset(0 as i32 as isize)).elements = -(1 as i32)
            as libc::c_longlong;
        (**((*r).task).offset(0 as i32 as isize)).idx = -(1 as i32);
        let ref mut fresh5 = (**((*r).task).offset(0 as i32 as isize)).obj;
        *fresh5 = 0 as *mut libc::c_void;
        let ref mut fresh6 = (**((*r).task).offset(0 as i32 as isize)).parent;
        *fresh6 = 0 as *mut redisReadTask;
        let ref mut fresh7 = (**((*r).task).offset(0 as i32 as isize)).privdata;
        *fresh7 = (*r).privdata;
        (*r).ridx = 0 as i32;
    }
    while (*r).ridx >= 0 as i32 {
        if processItem(r) != 0 as i32 {
            break;
        }
    }
    if (*r).err != 0 {
        return -(1 as i32);
    }
    if (*r).pos >= 1024 as i32 as u64 {
        if sdsrange((*r).buf, (*r).pos as ssize_t, -(1 as i32) as ssize_t) < 0 as i32 {
            return -(1 as i32);
        }
        (*r).pos = 0 as i32 as size_t;
        (*r).len = sdslen((*r).buf);
    }
    if (*r).ridx == -(1 as i32) {
        if !reply.is_null() {
            *reply = (*r).reply;
        } else if !((*r).reply).is_null() && !((*r).fn_0).is_null()
            && ((*(*r).fn_0).freeObject).is_some()
        {
            ((*(*r).fn_0).freeObject).expect("non-null function pointer")((*r).reply);
        }
        (*r).reply = 0 as *mut libc::c_void;
    }
    return 0 as i32;
}