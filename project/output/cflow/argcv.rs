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
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
pub type __int32_t = i32;
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
#[inline]
unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
unsafe extern "C" fn argcv_scan(
    mut len: i32,
    mut command: *const i8,
    mut delim: *const i8,
    mut cmnt: *const i8,
    mut start: *mut i32,
    mut end: *mut i32,
    mut save: *mut i32,
) -> i32 {
    let mut i: i32 = 0 as i32;
    loop {
        i = *save;
        if i >= len {
            return i + 1 as i32;
        }
        while i < len
            && (*command.offset(i as isize) as i32 == ' ' as i32
                || *command.offset(i as isize) as i32 == '\t' as i32
                || *command.offset(i as isize) as i32 == '\n' as i32)
        {
            i += 1;
            i;
        }
        *start = i;
        if (strchr(delim, *command.offset(i as isize) as i32)).is_null() {
            while i < len {
                if *command.offset(i as isize) as i32 == '\\' as i32 {
                    i += 1;
                    if i == len {
                        break;
                    }
                    i += 1;
                    i;
                } else if *command.offset(i as isize) as i32 == '\'' as i32
                    || *command.offset(i as isize) as i32 == '"' as i32
                {
                    let mut j: i32 = 0;
                    j = i + 1 as i32;
                    while j < len
                        && *command.offset(j as isize) as i32
                            != *command.offset(i as isize) as i32
                    {
                        if *command.offset(j as isize) as i32 == '\\' as i32 {
                            j += 1;
                            j;
                        }
                        j += 1;
                        j;
                    }
                    if j < len {
                        i = j + 1 as i32;
                    } else {
                        i += 1;
                        i;
                    }
                } else {
                    if *command.offset(i as isize) as i32 == ' ' as i32
                        || *command.offset(i as isize) as i32 == '\t' as i32
                        || *command.offset(i as isize) as i32 == '\n' as i32
                        || !(strchr(delim, *command.offset(i as isize) as i32)).is_null()
                    {
                        break;
                    }
                    i += 1;
                    i;
                }
            }
            i -= 1;
            i;
        }
        *end = i;
        *save = i + 1 as i32;
        if !(*save <= len) {
            break;
        }
        if !(!cmnt.is_null()
            && !(strchr(cmnt, *command.offset(*start as isize) as i32)).is_null())
        {
            break;
        }
        i = *save;
        while i < len && *command.offset(i as isize) as i32 != '\n' as i32 {
            i += 1;
            i;
        }
        *save = i;
    }
    return *save;
}
static mut quote_transtab: [i8; 15] = unsafe {
    *::core::mem::transmute::<
        &[u8; 15],
        &mut [i8; 15],
    >(b"\\\\a\x07b\x08f\x0Cn\nr\rt\t\0")
};
#[no_mangle]
pub unsafe extern "C" fn argcv_unquote_char(mut c: i32) -> i32 {
    let mut p: *mut i8 = 0 as *mut i8;
    p = quote_transtab.as_mut_ptr();
    while *p != 0 {
        if *p as i32 == c {
            return *p.offset(1 as i32 as isize) as i32;
        }
        p = p.offset(2 as i32 as isize);
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn argcv_quote_char(mut c: i32) -> i32 {
    let mut p: *mut i8 = 0 as *mut i8;
    p = quote_transtab
        .as_mut_ptr()
        .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
        .offset(-(2 as i32 as isize));
    while p > quote_transtab.as_mut_ptr() {
        if *p as i32 == c {
            return *p.offset(-(1 as i32) as isize) as i32;
        }
        p = p.offset(-(2 as i32 as isize));
    }
    return -(1 as i32);
}
unsafe extern "C" fn xtonum(
    mut pval: *mut i32,
    mut src: *const i8,
    mut base: i32,
    mut cnt: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut val: i32 = 0;
    i = 0 as i32;
    val = 0 as i32;
    while i < cnt {
        let mut n: i32 = *(src as *mut u8) as i32;
        if n > 127 as i32
            || {
                n = (if *(*__ctype_b_loc()).offset(n as isize) as i32
                    & _ISdigit as i32 as libc::c_ushort as i32 != 0
                {
                    n - '0' as i32
                } else {
                    (if *(*__ctype_b_loc()).offset(n as isize) as i32
                        & _ISxdigit as i32 as libc::c_ushort as i32 != 0
                    {
                        ({
                            let mut __res: i32 = 0;
                            if ::core::mem::size_of::<i32>() as u64 > 1 as i32 as u64 {
                                if 0 != 0 {
                                    let mut __c: i32 = n;
                                    __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                                        __c
                                    } else {
                                        *(*__ctype_toupper_loc()).offset(__c as isize)
                                    });
                                } else {
                                    __res = toupper(n);
                                }
                            } else {
                                __res = *(*__ctype_toupper_loc()).offset(n as isize);
                            }
                            __res
                        }) - 'A' as i32 + 10 as i32
                    } else {
                        255 as i32
                    })
                });
                n >= base
            }
        {
            break;
        }
        val = val * base + n;
        i += 1;
        i;
        src = src.offset(1);
        src;
    }
    *pval = val;
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn argcv_quoted_length(
    mut str: *const i8,
    mut quote: *mut i32,
) -> size_t {
    let mut len: size_t = 0 as i32 as size_t;
    *quote = 0 as i32;
    while *str != 0 {
        if *str as i32 == ' ' as i32 {
            len = len.wrapping_add(1);
            len;
            *quote = 1 as i32;
        } else if *str as i32 == '"' as i32 || *str as i32 == '\'' as i32 {
            len = (len as u64).wrapping_add(2 as i32 as u64) as size_t as size_t;
            *quote = 1 as i32;
        } else if *str as i32 != '\t' as i32 && *str as i32 != '\\' as i32
            && *(*__ctype_b_loc()).offset(*str as i32 as isize) as i32
                & _ISprint as i32 as libc::c_ushort as i32 != 0
        {
            len = len.wrapping_add(1);
            len;
        } else if argcv_quote_char(*str as i32) != -(1 as i32) {
            len = (len as u64).wrapping_add(2 as i32 as u64) as size_t as size_t;
        } else {
            len = (len as u64).wrapping_add(4 as i32 as u64) as size_t as size_t;
        }
        str = str.offset(1);
        str;
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn argcv_unquote_copy(
    mut dst: *mut i8,
    mut src: *const i8,
    mut n: size_t,
) {
    let mut i: i32 = 0 as i32;
    let mut c: i32 = 0;
    let mut expect_delim: i32 = 0 as i32;
    while (i as u64) < n {
        match *src.offset(i as isize) as i32 {
            39 | 34 => {
                if expect_delim == 0 {
                    let mut p: *const i8 = 0 as *const i8;
                    p = src.offset(i as isize).offset(1 as i32 as isize);
                    while *p as i32 != 0 && *p as i32 != *src.offset(i as isize) as i32 {
                        if *p as i32 == '\\' as i32 {
                            p = p.offset(1);
                            p;
                        }
                        p = p.offset(1);
                        p;
                    }
                    if *p != 0 {
                        let fresh0 = i;
                        i = i + 1;
                        expect_delim = *src.offset(fresh0 as isize) as i32;
                    } else {
                        let fresh1 = i;
                        i = i + 1;
                        let fresh2 = dst;
                        dst = dst.offset(1);
                        *fresh2 = *src.offset(fresh1 as isize);
                    }
                } else if expect_delim == *src.offset(i as isize) as i32 {
                    i += 1;
                    i;
                } else {
                    let fresh3 = i;
                    i = i + 1;
                    let fresh4 = dst;
                    dst = dst.offset(1);
                    *fresh4 = *src.offset(fresh3 as isize);
                }
            }
            92 => {
                i += 1;
                i;
                if *src.offset(i as isize) as i32 == 'x' as i32
                    || *src.offset(i as isize) as i32 == 'X' as i32
                {
                    if n.wrapping_sub(i as u64) < 2 as i32 as u64 {
                        let fresh5 = dst;
                        dst = dst.offset(1);
                        *fresh5 = '\\' as i32 as i8;
                        let fresh6 = i;
                        i = i + 1;
                        let fresh7 = dst;
                        dst = dst.offset(1);
                        *fresh7 = *src.offset(fresh6 as isize);
                    } else {
                        let mut off: i32 = xtonum(
                            &mut c,
                            src.offset(i as isize).offset(1 as i32 as isize),
                            16 as i32,
                            2 as i32,
                        );
                        if off == 0 as i32 {
                            let fresh8 = dst;
                            dst = dst.offset(1);
                            *fresh8 = '\\' as i32 as i8;
                            let fresh9 = i;
                            i = i + 1;
                            let fresh10 = dst;
                            dst = dst.offset(1);
                            *fresh10 = *src.offset(fresh9 as isize);
                        } else {
                            let fresh11 = dst;
                            dst = dst.offset(1);
                            *fresh11 = c as i8;
                            i += off + 1 as i32;
                        }
                    }
                } else if (*src.offset(i as isize) as u8 as i32) < 128 as i32
                    && *(*__ctype_b_loc())
                        .offset(*src.offset(i as isize) as i32 as isize) as i32
                        & _ISdigit as i32 as libc::c_ushort as i32 != 0
                {
                    if n.wrapping_sub(i as u64) < 1 as i32 as u64 {
                        let fresh12 = dst;
                        dst = dst.offset(1);
                        *fresh12 = '\\' as i32 as i8;
                        let fresh13 = i;
                        i = i + 1;
                        let fresh14 = dst;
                        dst = dst.offset(1);
                        *fresh14 = *src.offset(fresh13 as isize);
                    } else {
                        let mut off_0: i32 = xtonum(
                            &mut c,
                            src.offset(i as isize),
                            8 as i32,
                            3 as i32,
                        );
                        if off_0 == 0 as i32 {
                            let fresh15 = dst;
                            dst = dst.offset(1);
                            *fresh15 = '\\' as i32 as i8;
                            let fresh16 = i;
                            i = i + 1;
                            let fresh17 = dst;
                            dst = dst.offset(1);
                            *fresh17 = *src.offset(fresh16 as isize);
                        } else {
                            let fresh18 = dst;
                            dst = dst.offset(1);
                            *fresh18 = c as i8;
                            i += off_0;
                        }
                    }
                } else {
                    let fresh19 = i;
                    i = i + 1;
                    let fresh20 = dst;
                    dst = dst.offset(1);
                    *fresh20 = argcv_unquote_char(*src.offset(fresh19 as isize) as i32)
                        as i8;
                }
            }
            _ => {
                let fresh21 = i;
                i = i + 1;
                let fresh22 = dst;
                dst = dst.offset(1);
                *fresh22 = *src.offset(fresh21 as isize);
            }
        }
    }
    *dst = 0 as i32 as i8;
}
#[no_mangle]
pub unsafe extern "C" fn argcv_quote_copy(mut dst: *mut i8, mut src: *const i8) {
    while *src != 0 {
        if *src as i32 == '"' as i32 || *src as i32 == '\'' as i32 {
            let fresh23 = dst;
            dst = dst.offset(1);
            *fresh23 = '\\' as i32 as i8;
            let fresh24 = dst;
            dst = dst.offset(1);
            *fresh24 = *src;
        } else if *src as i32 != '\t' as i32 && *src as i32 != '\\' as i32
            && *(*__ctype_b_loc()).offset(*src as i32 as isize) as i32
                & _ISprint as i32 as libc::c_ushort as i32 != 0
        {
            let fresh25 = dst;
            dst = dst.offset(1);
            *fresh25 = *src;
        } else {
            let mut c: i32 = argcv_quote_char(*src as i32);
            let fresh26 = dst;
            dst = dst.offset(1);
            *fresh26 = '\\' as i32 as i8;
            if c != -(1 as i32) {
                let fresh27 = dst;
                dst = dst.offset(1);
                *fresh27 = c as i8;
            } else {
                let mut tmp: [i8; 4] = [0; 4];
                snprintf(
                    tmp.as_mut_ptr(),
                    ::core::mem::size_of::<[i8; 4]>() as u64,
                    b"%03o\0" as *const u8 as *const i8,
                    *(src as *mut u8) as i32,
                );
                memcpy(
                    dst as *mut libc::c_void,
                    tmp.as_mut_ptr() as *const libc::c_void,
                    3 as i32 as u64,
                );
                dst = dst.offset(3 as i32 as isize);
            }
        }
        src = src.offset(1);
        src;
    }
}
#[no_mangle]
pub unsafe extern "C" fn argcv_get(
    mut command: *const i8,
    mut delim: *const i8,
    mut cmnt: *const i8,
    mut argc: *mut i32,
    mut argv: *mut *mut *mut i8,
) -> i32 {
    let mut len: i32 = strlen(command) as i32;
    let mut i: i32 = 0 as i32;
    let mut start: i32 = 0;
    let mut end: i32 = 0;
    let mut save: i32 = 0;
    *argv = 0 as *mut *mut i8;
    *argc = 0 as i32;
    save = 0 as i32;
    while argcv_scan(len, command, delim, cmnt, &mut start, &mut end, &mut save) <= len {
        *argc += 1;
        *argc;
    }
    *argv = calloc((*argc + 1 as i32) as u64, ::core::mem::size_of::<*mut i8>() as u64)
        as *mut *mut i8;
    if (*argv).is_null() {
        return 12 as i32;
    }
    i = 0 as i32;
    save = 0 as i32;
    i = 0 as i32;
    while i < *argc {
        let mut n: i32 = 0;
        argcv_scan(len, command, delim, cmnt, &mut start, &mut end, &mut save);
        if (*command.offset(start as isize) as i32 == '"' as i32
            || *command.offset(end as isize) as i32 == '\'' as i32)
            && *command.offset(end as isize) as i32
                == *command.offset(start as isize) as i32
        {
            start += 1;
            start;
            end -= 1;
            end;
        }
        n = end - start + 1 as i32;
        let ref mut fresh28 = *(*argv).offset(i as isize);
        *fresh28 = calloc((n + 1 as i32) as u64, ::core::mem::size_of::<i8>() as u64)
            as *mut i8;
        if (*(*argv).offset(i as isize)).is_null() {
            return 12 as i32;
        }
        argcv_unquote_copy(
            *(*argv).offset(i as isize),
            &*command.offset(start as isize),
            n as size_t,
        );
        *(*(*argv).offset(i as isize)).offset(n as isize) = 0 as i32 as i8;
        i += 1;
        i;
    }
    let ref mut fresh29 = *(*argv).offset(i as isize);
    *fresh29 = 0 as *mut i8;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn argcv_free(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    loop {
        argc -= 1;
        if !(argc >= 0 as i32) {
            break;
        }
        if !(*argv.offset(argc as isize)).is_null() {
            free(*argv.offset(argc as isize) as *mut libc::c_void);
        }
    }
    free(argv as *mut libc::c_void);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn argcv_string(
    mut argc: i32,
    mut argv: *mut *mut i8,
    mut pstring: *mut *mut i8,
) -> i32 {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut len: size_t = 0;
    let mut buffer: *mut i8 = 0 as *mut i8;
    if pstring.is_null() {
        return 22 as i32;
    }
    buffer = malloc(1 as i32 as u64) as *mut i8;
    if buffer.is_null() {
        return 12 as i32;
    }
    *buffer = '\0' as i32 as i8;
    j = 0 as i32 as size_t;
    i = j;
    len = i;
    while i < argc as u64 {
        let mut quote: i32 = 0;
        let mut toklen: i32 = 0;
        toklen = argcv_quoted_length(*argv.offset(i as isize), &mut quote) as i32;
        len = (len as u64).wrapping_add((toklen + 2 as i32) as u64) as size_t as size_t;
        if quote != 0 {
            len = (len as u64).wrapping_add(2 as i32 as u64) as size_t as size_t;
        }
        buffer = realloc(buffer as *mut libc::c_void, len) as *mut i8;
        if buffer.is_null() {
            return 12 as i32;
        }
        if i != 0 as i32 as u64 {
            let fresh30 = j;
            j = j.wrapping_add(1);
            *buffer.offset(fresh30 as isize) = ' ' as i32 as i8;
        }
        if quote != 0 {
            let fresh31 = j;
            j = j.wrapping_add(1);
            *buffer.offset(fresh31 as isize) = '"' as i32 as i8;
        }
        argcv_quote_copy(buffer.offset(j as isize), *argv.offset(i as isize));
        j = (j as u64).wrapping_add(toklen as u64) as size_t as size_t;
        if quote != 0 {
            let fresh32 = j;
            j = j.wrapping_add(1);
            *buffer.offset(fresh32 as isize) = '"' as i32 as i8;
        }
        i = i.wrapping_add(1);
        i;
    }
    while j > 0 as i32 as u64
        && *(*__ctype_b_loc())
            .offset(
                *buffer.offset(j.wrapping_sub(1 as i32 as u64) as isize) as i32 as isize,
            ) as i32 & _ISspace as i32 as libc::c_ushort as i32 != 0
    {
        j = j.wrapping_sub(1);
        j;
    }
    *buffer.offset(j as isize) = 0 as i32 as i8;
    if !pstring.is_null() {
        *pstring = buffer;
    }
    return 0 as i32;
}