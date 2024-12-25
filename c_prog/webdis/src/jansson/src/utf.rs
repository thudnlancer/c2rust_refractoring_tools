#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
#[no_mangle]
pub unsafe extern "C" fn utf8_encode(
    mut codepoint: int32_t,
    mut buffer: *mut libc::c_char,
    mut size: *mut size_t,
) -> libc::c_int {
    if codepoint < 0 as libc::c_int {
        return -(1 as libc::c_int)
    } else if codepoint < 0x80 as libc::c_int {
        *buffer.offset(0 as libc::c_int as isize) = codepoint as libc::c_char;
        *size = 1 as libc::c_int as size_t;
    } else if codepoint < 0x800 as libc::c_int {
        *buffer
            .offset(
                0 as libc::c_int as isize,
            ) = (0xc0 as libc::c_int
            + ((codepoint & 0x7c0 as libc::c_int) >> 6 as libc::c_int)) as libc::c_char;
        *buffer
            .offset(
                1 as libc::c_int as isize,
            ) = (0x80 as libc::c_int + (codepoint & 0x3f as libc::c_int))
            as libc::c_char;
        *size = 2 as libc::c_int as size_t;
    } else if codepoint < 0x10000 as libc::c_int {
        *buffer
            .offset(
                0 as libc::c_int as isize,
            ) = (0xe0 as libc::c_int
            + ((codepoint & 0xf000 as libc::c_int) >> 12 as libc::c_int))
            as libc::c_char;
        *buffer
            .offset(
                1 as libc::c_int as isize,
            ) = (0x80 as libc::c_int
            + ((codepoint & 0xfc0 as libc::c_int) >> 6 as libc::c_int)) as libc::c_char;
        *buffer
            .offset(
                2 as libc::c_int as isize,
            ) = (0x80 as libc::c_int + (codepoint & 0x3f as libc::c_int))
            as libc::c_char;
        *size = 3 as libc::c_int as size_t;
    } else if codepoint <= 0x10ffff as libc::c_int {
        *buffer
            .offset(
                0 as libc::c_int as isize,
            ) = (0xf0 as libc::c_int
            + ((codepoint & 0x1c0000 as libc::c_int) >> 18 as libc::c_int))
            as libc::c_char;
        *buffer
            .offset(
                1 as libc::c_int as isize,
            ) = (0x80 as libc::c_int
            + ((codepoint & 0x3f000 as libc::c_int) >> 12 as libc::c_int))
            as libc::c_char;
        *buffer
            .offset(
                2 as libc::c_int as isize,
            ) = (0x80 as libc::c_int
            + ((codepoint & 0xfc0 as libc::c_int) >> 6 as libc::c_int)) as libc::c_char;
        *buffer
            .offset(
                3 as libc::c_int as isize,
            ) = (0x80 as libc::c_int + (codepoint & 0x3f as libc::c_int))
            as libc::c_char;
        *size = 4 as libc::c_int as size_t;
    } else {
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_check_first(mut byte: libc::c_char) -> size_t {
    let mut u: libc::c_uchar = byte as libc::c_uchar;
    if (u as libc::c_int) < 0x80 as libc::c_int {
        return 1 as libc::c_int as size_t;
    }
    if 0x80 as libc::c_int <= u as libc::c_int && u as libc::c_int <= 0xbf as libc::c_int
    {
        return 0 as libc::c_int as size_t
    } else if u as libc::c_int == 0xc0 as libc::c_int
        || u as libc::c_int == 0xc1 as libc::c_int
    {
        return 0 as libc::c_int as size_t
    } else if 0xc2 as libc::c_int <= u as libc::c_int
        && u as libc::c_int <= 0xdf as libc::c_int
    {
        return 2 as libc::c_int as size_t
    } else if 0xe0 as libc::c_int <= u as libc::c_int
        && u as libc::c_int <= 0xef as libc::c_int
    {
        return 3 as libc::c_int as size_t
    } else if 0xf0 as libc::c_int <= u as libc::c_int
        && u as libc::c_int <= 0xf4 as libc::c_int
    {
        return 4 as libc::c_int as size_t
    } else {
        return 0 as libc::c_int as size_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn utf8_check_full(
    mut buffer: *const libc::c_char,
    mut size: size_t,
    mut codepoint: *mut int32_t,
) -> size_t {
    let mut i: size_t = 0;
    let mut value: int32_t = 0 as libc::c_int;
    let mut u: libc::c_uchar = *buffer.offset(0 as libc::c_int as isize)
        as libc::c_uchar;
    if size == 2 as libc::c_int as libc::c_ulong {
        value = u as libc::c_int & 0x1f as libc::c_int;
    } else if size == 3 as libc::c_int as libc::c_ulong {
        value = u as libc::c_int & 0xf as libc::c_int;
    } else if size == 4 as libc::c_int as libc::c_ulong {
        value = u as libc::c_int & 0x7 as libc::c_int;
    } else {
        return 0 as libc::c_int as size_t
    }
    i = 1 as libc::c_int as size_t;
    while i < size {
        u = *buffer.offset(i as isize) as libc::c_uchar;
        if (u as libc::c_int) < 0x80 as libc::c_int
            || u as libc::c_int > 0xbf as libc::c_int
        {
            return 0 as libc::c_int as size_t;
        }
        value = (value << 6 as libc::c_int) + (u as libc::c_int & 0x3f as libc::c_int);
        i = i.wrapping_add(1);
        i;
    }
    if value > 0x10ffff as libc::c_int {
        return 0 as libc::c_int as size_t
    } else if 0xd800 as libc::c_int <= value && value <= 0xdfff as libc::c_int {
        return 0 as libc::c_int as size_t
    } else if size == 2 as libc::c_int as libc::c_ulong && value < 0x80 as libc::c_int
        || size == 3 as libc::c_int as libc::c_ulong && value < 0x800 as libc::c_int
        || size == 4 as libc::c_int as libc::c_ulong && value < 0x10000 as libc::c_int
    {
        return 0 as libc::c_int as size_t
    }
    if !codepoint.is_null() {
        *codepoint = value;
    }
    return 1 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_iterate(
    mut buffer: *const libc::c_char,
    mut bufsize: size_t,
    mut codepoint: *mut int32_t,
) -> *const libc::c_char {
    let mut count: size_t = 0;
    let mut value: int32_t = 0;
    if bufsize == 0 {
        return buffer;
    }
    count = utf8_check_first(*buffer.offset(0 as libc::c_int as isize));
    if count <= 0 as libc::c_int as libc::c_ulong {
        return 0 as *const libc::c_char;
    }
    if count == 1 as libc::c_int as libc::c_ulong {
        value = *buffer.offset(0 as libc::c_int as isize) as libc::c_uchar as int32_t;
    } else if count > bufsize || utf8_check_full(buffer, count, &mut value) == 0 {
        return 0 as *const libc::c_char
    }
    if !codepoint.is_null() {
        *codepoint = value;
    }
    return buffer.offset(count as isize);
}
#[no_mangle]
pub unsafe extern "C" fn utf8_check_string(
    mut string: *const libc::c_char,
    mut length: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < length {
        let mut count: size_t = utf8_check_first(*string.offset(i as isize));
        if count == 0 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int
        } else if count > 1 as libc::c_int as libc::c_ulong {
            if count > length.wrapping_sub(i) {
                return 0 as libc::c_int;
            }
            if utf8_check_full(&*string.offset(i as isize), count, 0 as *mut int32_t)
                == 0
            {
                return 0 as libc::c_int;
            }
            i = (i as libc::c_ulong)
                .wrapping_add(count.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                as size_t as size_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as libc::c_int;
}
