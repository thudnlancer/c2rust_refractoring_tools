#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type __int32_t = libc::c_int;
pub type C2RustUnnamed = libc::c_uint;
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
pub type size_t = libc::c_ulong;
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
unsafe extern "C" fn argcv_scan(
    mut len: libc::c_int,
    mut command: *const libc::c_char,
    mut delim: *const libc::c_char,
    mut cmnt: *const libc::c_char,
    mut start: *mut libc::c_int,
    mut end: *mut libc::c_int,
    mut save: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    loop {
        i = *save;
        if i >= len {
            return i + 1 as libc::c_int;
        }
        while i < len
            && (*command.offset(i as isize) as libc::c_int == ' ' as i32
                || *command.offset(i as isize) as libc::c_int == '\t' as i32
                || *command.offset(i as isize) as libc::c_int == '\n' as i32)
        {
            i += 1;
            i;
        }
        *start = i;
        if (strchr(delim, *command.offset(i as isize) as libc::c_int)).is_null() {
            while i < len {
                if *command.offset(i as isize) as libc::c_int == '\\' as i32 {
                    i += 1;
                    if i == len {
                        break;
                    }
                    i += 1;
                    i;
                } else if *command.offset(i as isize) as libc::c_int == '\'' as i32
                    || *command.offset(i as isize) as libc::c_int == '"' as i32
                {
                    let mut j: libc::c_int = 0;
                    j = i + 1 as libc::c_int;
                    while j < len
                        && *command.offset(j as isize) as libc::c_int
                            != *command.offset(i as isize) as libc::c_int
                    {
                        if *command.offset(j as isize) as libc::c_int == '\\' as i32 {
                            j += 1;
                            j;
                        }
                        j += 1;
                        j;
                    }
                    if j < len {
                        i = j + 1 as libc::c_int;
                    } else {
                        i += 1;
                        i;
                    }
                } else {
                    if *command.offset(i as isize) as libc::c_int == ' ' as i32
                        || *command.offset(i as isize) as libc::c_int == '\t' as i32
                        || *command.offset(i as isize) as libc::c_int == '\n' as i32
                        || !(strchr(delim, *command.offset(i as isize) as libc::c_int))
                            .is_null()
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
        *save = i + 1 as libc::c_int;
        if !(*save <= len) {
            break;
        }
        if !(!cmnt.is_null()
            && !(strchr(cmnt, *command.offset(*start as isize) as libc::c_int))
                .is_null())
        {
            break;
        }
        i = *save;
        while i < len && *command.offset(i as isize) as libc::c_int != '\n' as i32 {
            i += 1;
            i;
        }
        *save = i;
    }
    return *save;
}
static mut quote_transtab: [libc::c_char; 15] = unsafe {
    *::core::mem::transmute::<
        &[u8; 15],
        &mut [libc::c_char; 15],
    >(b"\\\\a\x07b\x08f\x0Cn\nr\rt\t\0")
};
#[no_mangle]
pub unsafe extern "C" fn argcv_unquote_char(mut c: libc::c_int) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = quote_transtab.as_mut_ptr();
    while *p != 0 {
        if *p as libc::c_int == c {
            return *p.offset(1 as libc::c_int as isize) as libc::c_int;
        }
        p = p.offset(2 as libc::c_int as isize);
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn argcv_quote_char(mut c: libc::c_int) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = quote_transtab
        .as_mut_ptr()
        .offset(::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as isize)
        .offset(-(2 as libc::c_int as isize));
    while p > quote_transtab.as_mut_ptr() {
        if *p as libc::c_int == c {
            return *p.offset(-(1 as libc::c_int) as isize) as libc::c_int;
        }
        p = p.offset(-(2 as libc::c_int as isize));
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn xtonum(
    mut pval: *mut libc::c_int,
    mut src: *const libc::c_char,
    mut base: libc::c_int,
    mut cnt: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    i = 0 as libc::c_int;
    val = 0 as libc::c_int;
    while i < cnt {
        let mut n: libc::c_int = *(src as *mut libc::c_uchar) as libc::c_int;
        if n > 127 as libc::c_int
            || {
                n = (if *(*__ctype_b_loc()).offset(n as isize) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    n - '0' as i32
                } else {
                    (if *(*__ctype_b_loc()).offset(n as isize) as libc::c_int
                        & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                    {
                        ({
                            let mut __res: libc::c_int = 0;
                            if ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
                                > 1 as libc::c_int as libc::c_ulong
                            {
                                if 0 != 0 {
                                    let mut __c: libc::c_int = n;
                                    __res = (if __c < -(128 as libc::c_int)
                                        || __c > 255 as libc::c_int
                                    {
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
                        }) - 'A' as i32 + 10 as libc::c_int
                    } else {
                        255 as libc::c_int
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
    mut str: *const libc::c_char,
    mut quote: *mut libc::c_int,
) -> size_t {
    let mut len: size_t = 0 as libc::c_int as size_t;
    *quote = 0 as libc::c_int;
    while *str != 0 {
        if *str as libc::c_int == ' ' as i32 {
            len = len.wrapping_add(1);
            len;
            *quote = 1 as libc::c_int;
        } else if *str as libc::c_int == '"' as i32 || *str as libc::c_int == '\'' as i32
        {
            len = (len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
            *quote = 1 as libc::c_int;
        } else if *str as libc::c_int != '\t' as i32
            && *str as libc::c_int != '\\' as i32
            && *(*__ctype_b_loc()).offset(*str as libc::c_int as isize) as libc::c_int
                & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            len = len.wrapping_add(1);
            len;
        } else if argcv_quote_char(*str as libc::c_int) != -(1 as libc::c_int) {
            len = (len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        } else {
            len = (len as libc::c_ulong).wrapping_add(4 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        }
        str = str.offset(1);
        str;
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn argcv_unquote_copy(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut n: size_t,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    let mut expect_delim: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_ulong) < n {
        match *src.offset(i as isize) as libc::c_int {
            39 | 34 => {
                if expect_delim == 0 {
                    let mut p: *const libc::c_char = 0 as *const libc::c_char;
                    p = src.offset(i as isize).offset(1 as libc::c_int as isize);
                    while *p as libc::c_int != 0
                        && *p as libc::c_int != *src.offset(i as isize) as libc::c_int
                    {
                        if *p as libc::c_int == '\\' as i32 {
                            p = p.offset(1);
                            p;
                        }
                        p = p.offset(1);
                        p;
                    }
                    if *p != 0 {
                        let fresh0 = i;
                        i = i + 1;
                        expect_delim = *src.offset(fresh0 as isize) as libc::c_int;
                    } else {
                        let fresh1 = i;
                        i = i + 1;
                        let fresh2 = dst;
                        dst = dst.offset(1);
                        *fresh2 = *src.offset(fresh1 as isize);
                    }
                } else if expect_delim == *src.offset(i as isize) as libc::c_int {
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
                if *src.offset(i as isize) as libc::c_int == 'x' as i32
                    || *src.offset(i as isize) as libc::c_int == 'X' as i32
                {
                    if n.wrapping_sub(i as libc::c_ulong)
                        < 2 as libc::c_int as libc::c_ulong
                    {
                        let fresh5 = dst;
                        dst = dst.offset(1);
                        *fresh5 = '\\' as i32 as libc::c_char;
                        let fresh6 = i;
                        i = i + 1;
                        let fresh7 = dst;
                        dst = dst.offset(1);
                        *fresh7 = *src.offset(fresh6 as isize);
                    } else {
                        let mut off: libc::c_int = xtonum(
                            &mut c,
                            src.offset(i as isize).offset(1 as libc::c_int as isize),
                            16 as libc::c_int,
                            2 as libc::c_int,
                        );
                        if off == 0 as libc::c_int {
                            let fresh8 = dst;
                            dst = dst.offset(1);
                            *fresh8 = '\\' as i32 as libc::c_char;
                            let fresh9 = i;
                            i = i + 1;
                            let fresh10 = dst;
                            dst = dst.offset(1);
                            *fresh10 = *src.offset(fresh9 as isize);
                        } else {
                            let fresh11 = dst;
                            dst = dst.offset(1);
                            *fresh11 = c as libc::c_char;
                            i += off + 1 as libc::c_int;
                        }
                    }
                } else if (*src.offset(i as isize) as libc::c_uchar as libc::c_int)
                    < 128 as libc::c_int
                    && *(*__ctype_b_loc())
                        .offset(*src.offset(i as isize) as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    if n.wrapping_sub(i as libc::c_ulong)
                        < 1 as libc::c_int as libc::c_ulong
                    {
                        let fresh12 = dst;
                        dst = dst.offset(1);
                        *fresh12 = '\\' as i32 as libc::c_char;
                        let fresh13 = i;
                        i = i + 1;
                        let fresh14 = dst;
                        dst = dst.offset(1);
                        *fresh14 = *src.offset(fresh13 as isize);
                    } else {
                        let mut off_0: libc::c_int = xtonum(
                            &mut c,
                            src.offset(i as isize),
                            8 as libc::c_int,
                            3 as libc::c_int,
                        );
                        if off_0 == 0 as libc::c_int {
                            let fresh15 = dst;
                            dst = dst.offset(1);
                            *fresh15 = '\\' as i32 as libc::c_char;
                            let fresh16 = i;
                            i = i + 1;
                            let fresh17 = dst;
                            dst = dst.offset(1);
                            *fresh17 = *src.offset(fresh16 as isize);
                        } else {
                            let fresh18 = dst;
                            dst = dst.offset(1);
                            *fresh18 = c as libc::c_char;
                            i += off_0;
                        }
                    }
                } else {
                    let fresh19 = i;
                    i = i + 1;
                    let fresh20 = dst;
                    dst = dst.offset(1);
                    *fresh20 = argcv_unquote_char(
                        *src.offset(fresh19 as isize) as libc::c_int,
                    ) as libc::c_char;
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
    *dst = 0 as libc::c_int as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn argcv_quote_copy(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
) {
    while *src != 0 {
        if *src as libc::c_int == '"' as i32 || *src as libc::c_int == '\'' as i32 {
            let fresh23 = dst;
            dst = dst.offset(1);
            *fresh23 = '\\' as i32 as libc::c_char;
            let fresh24 = dst;
            dst = dst.offset(1);
            *fresh24 = *src;
        } else if *src as libc::c_int != '\t' as i32
            && *src as libc::c_int != '\\' as i32
            && *(*__ctype_b_loc()).offset(*src as libc::c_int as isize) as libc::c_int
                & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            let fresh25 = dst;
            dst = dst.offset(1);
            *fresh25 = *src;
        } else {
            let mut c: libc::c_int = argcv_quote_char(*src as libc::c_int);
            let fresh26 = dst;
            dst = dst.offset(1);
            *fresh26 = '\\' as i32 as libc::c_char;
            if c != -(1 as libc::c_int) {
                let fresh27 = dst;
                dst = dst.offset(1);
                *fresh27 = c as libc::c_char;
            } else {
                let mut tmp: [libc::c_char; 4] = [0; 4];
                snprintf(
                    tmp.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong,
                    b"%03o\0" as *const u8 as *const libc::c_char,
                    *(src as *mut libc::c_uchar) as libc::c_int,
                );
                memcpy(
                    dst as *mut libc::c_void,
                    tmp.as_mut_ptr() as *const libc::c_void,
                    3 as libc::c_int as libc::c_ulong,
                );
                dst = dst.offset(3 as libc::c_int as isize);
            }
        }
        src = src.offset(1);
        src;
    }
}
#[no_mangle]
pub unsafe extern "C" fn argcv_get(
    mut command: *const libc::c_char,
    mut delim: *const libc::c_char,
    mut cmnt: *const libc::c_char,
    mut argc: *mut libc::c_int,
    mut argv: *mut *mut *mut libc::c_char,
) -> libc::c_int {
    let mut len: libc::c_int = strlen(command) as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut start: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut save: libc::c_int = 0;
    *argv = 0 as *mut *mut libc::c_char;
    *argc = 0 as libc::c_int;
    save = 0 as libc::c_int;
    while argcv_scan(len, command, delim, cmnt, &mut start, &mut end, &mut save) <= len {
        *argc += 1;
        *argc;
    }
    *argv = calloc(
        (*argc + 1 as libc::c_int) as libc::c_ulong,
        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    if (*argv).is_null() {
        return 12 as libc::c_int;
    }
    i = 0 as libc::c_int;
    save = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < *argc {
        let mut n: libc::c_int = 0;
        argcv_scan(len, command, delim, cmnt, &mut start, &mut end, &mut save);
        if (*command.offset(start as isize) as libc::c_int == '"' as i32
            || *command.offset(end as isize) as libc::c_int == '\'' as i32)
            && *command.offset(end as isize) as libc::c_int
                == *command.offset(start as isize) as libc::c_int
        {
            start += 1;
            start;
            end -= 1;
            end;
        }
        n = end - start + 1 as libc::c_int;
        let ref mut fresh28 = *(*argv).offset(i as isize);
        *fresh28 = calloc(
            (n + 1 as libc::c_int) as libc::c_ulong,
            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
        if (*(*argv).offset(i as isize)).is_null() {
            return 12 as libc::c_int;
        }
        argcv_unquote_copy(
            *(*argv).offset(i as isize),
            &*command.offset(start as isize),
            n as size_t,
        );
        *(*(*argv).offset(i as isize))
            .offset(n as isize) = 0 as libc::c_int as libc::c_char;
        i += 1;
        i;
    }
    let ref mut fresh29 = *(*argv).offset(i as isize);
    *fresh29 = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn argcv_free(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    loop {
        argc -= 1;
        if !(argc >= 0 as libc::c_int) {
            break;
        }
        if !(*argv.offset(argc as isize)).is_null() {
            free(*argv.offset(argc as isize) as *mut libc::c_void);
        }
    }
    free(argv as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn argcv_string(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pstring: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut len: size_t = 0;
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    if pstring.is_null() {
        return 22 as libc::c_int;
    }
    buffer = malloc(1 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    if buffer.is_null() {
        return 12 as libc::c_int;
    }
    *buffer = '\0' as i32 as libc::c_char;
    j = 0 as libc::c_int as size_t;
    i = j;
    len = i;
    while i < argc as libc::c_ulong {
        let mut quote: libc::c_int = 0;
        let mut toklen: libc::c_int = 0;
        toklen = argcv_quoted_length(*argv.offset(i as isize), &mut quote)
            as libc::c_int;
        len = (len as libc::c_ulong)
            .wrapping_add((toklen + 2 as libc::c_int) as libc::c_ulong) as size_t
            as size_t;
        if quote != 0 {
            len = (len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        }
        buffer = realloc(buffer as *mut libc::c_void, len) as *mut libc::c_char;
        if buffer.is_null() {
            return 12 as libc::c_int;
        }
        if i != 0 as libc::c_int as libc::c_ulong {
            let fresh30 = j;
            j = j.wrapping_add(1);
            *buffer.offset(fresh30 as isize) = ' ' as i32 as libc::c_char;
        }
        if quote != 0 {
            let fresh31 = j;
            j = j.wrapping_add(1);
            *buffer.offset(fresh31 as isize) = '"' as i32 as libc::c_char;
        }
        argcv_quote_copy(buffer.offset(j as isize), *argv.offset(i as isize));
        j = (j as libc::c_ulong).wrapping_add(toklen as libc::c_ulong) as size_t
            as size_t;
        if quote != 0 {
            let fresh32 = j;
            j = j.wrapping_add(1);
            *buffer.offset(fresh32 as isize) = '"' as i32 as libc::c_char;
        }
        i = i.wrapping_add(1);
        i;
    }
    while j > 0 as libc::c_int as libc::c_ulong
        && *(*__ctype_b_loc())
            .offset(
                *buffer
                    .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int as isize,
            ) as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        j = j.wrapping_sub(1);
        j;
    }
    *buffer.offset(j as isize) = 0 as libc::c_int as libc::c_char;
    if !pstring.is_null() {
        *pstring = buffer;
    }
    return 0 as libc::c_int;
}
