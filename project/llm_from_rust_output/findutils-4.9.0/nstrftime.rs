use libc::{c_char, c_int, c_long, c_uint, c_ulong, c_void, size_t, tm, time_t};
use std::ptr;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm_zone {
    pub next: *mut tm_zone,
    pub tz_is_set: c_char,
    pub abbrs: [c_char; 0],
}

pub type timezone_t = *mut tm_zone;

#[inline]
fn tolower(c: c_int) -> c_int {
    if c >= -128 && c < 256 {
        unsafe { *(*__ctype_tolower_loc()).offset(c as isize) }
    } else {
        c
    }
}

#[inline]
fn toupper(c: c_int) -> c_int {
    if c >= -128 && c < 256 {
        unsafe { *(*__ctype_toupper_loc()).offset(c as isize) }
    } else {
        c
    }
}

fn memcpy_lowcase(dest: *mut c_char, src: *const c_char, len: size_t) -> *mut c_char {
    for i in (0..len).rev() {
        unsafe {
            *dest.offset(i as isize) = {
                let c = *src.offset(i as isize) as c_uint as c_int;
                if c >= -128 && c < 256 {
                    *(*__ctype_tolower_loc()).offset(c as isize) as c_char
                } else {
                    c as c_char
                }
            };
        }
    }
    dest
}

fn memcpy_uppcase(dest: *mut c_char, src: *const c_char, len: size_t) -> *mut c_char {
    for i in (0..len).rev() {
        unsafe {
            *dest.offset(i as isize) = {
                let c = *src.offset(i as isize) as c_uint as c_int;
                if c >= -128 && c < 256 {
                    *(*__ctype_toupper_loc()).offset(c as isize) as c_char
                } else {
                    c as c_char
                }
            };
        }
    }
    dest
}

#[inline]
fn iso_week_days(yday: c_int, wday: c_int) -> c_int {
    let big_enough_multiple_of_7 = (366 / 7 + 2) * 7;
    yday - (yday - wday + 4 + big_enough_multiple_of_7) % 7 + 3
}

pub unsafe fn nstrftime(
    s: *mut c_char,
    maxsize: size_t,
    format: *const c_char,
    tp: *const tm,
    tz: timezone_t,
    ns: c_int,
) -> size_t {
    let tzset_called = false;
    __strftime_internal(
        s,
        maxsize,
        format,
        tp,
        false,
        0,
        -1,
        &mut (tzset_called as bool),
        tz,
        ns,
    )
}

unsafe fn __strftime_internal(
    mut s: *mut c_char,
    maxsize: size_t,
    format: *const c_char,
    tp: *const tm,
    upcase: bool,
    yr_spec: c_int,
    mut width: c_int,
    tzset_called: *mut bool,
    tz: timezone_t,
    ns: c_int,
) -> size_t {
    let saved_errno = *__errno_location();
    let mut hour12 = (*tp).tm_hour;
    let mut zone = (*tp).tm_zone;
    if zone.is_null() {
        zone = b"\0".as_ptr() as *const c_char;
    }

    if hour12 > 12 {
        hour12 -= 12;
    } else if hour12 == 0 {
        hour12 = 12;
    }

    let mut i = 0;
    let mut p = s;
    let mut f = format;

    while *f != 0 {
        let mut pad = 0;
        let mut modifier = 0;
        let mut digits = 0;
        let mut number_value = 0;
        let mut u_number_value = 0;
        let mut negative_number = false;
        let mut always_output_a_sign = false;
        let mut tz_colon_mask = 0;
        let mut subfmt = ptr::null();
        let mut bufp = ptr::null_mut();
        let mut buf = [0; 23];
        let mut to_lowcase = false;
        let mut to_uppcase = upcase;
        let mut colons = 0;
        let mut change_case = false;
        let mut format_char = 0;
        let mut subwidth = 0;

        if *f != b'%' as c_char {
            let n = 1;
            let w = if pad == b'-' as c_int || width < 0 {
                0
            } else {
                width
            } as size_t;
            let incr = if n < w { w } else { n };
            if incr >= maxsize.wrapping_sub(i) {
                *__errno_location() = 34;
                return 0;
            }
            if !p.is_null() {
                if n < w {
                    let delta = w - n;
                    if pad == b'0' as c_int || pad == b'+' as c_int {
                        memset(p as *mut c_void, b'0' as c_int, delta);
                        p = p.add(delta);
                    } else {
                        memset(p as *mut c_void, b' ' as c_int, delta);
                        p = p.add(delta);
                    }
                }
                *p = *f;
                p = p.add(n);
            }
            i = i.wrapping_add(incr);
        } else {
            let percent = f;
            loop {
                f = f.add(1);
                match *f {
                    b'_' | b'-' | b'+' | b'0' => pad = *f as c_int,
                    b'^' => to_uppcase = true,
                    b'#' => change_case = true,
                    _ => break,
                }
            }

            if (*f as c_uint - b'0' as c_uint) <= 9 {
                width = 0;
                loop {
                    let digit = *f as c_int - b'0' as c_int;
                    if width > c_int::MAX / 10 || (width == c_int::MAX / 10 && digit > c_int::MAX % 10) {
                        width = c_int::MAX;
                    } else {
                        width = width * 10 + digit;
                    }
                    f = f.add(1);
                    if !(*f as c_uint - b'0' as c_uint <= 9) {
                        break;
                    }
                }
            }

            match *f {
                b'E' | b'O' => {
                    modifier = *f as c_int;
                    f = f.add(1);
                }
                _ => modifier = 0,
            }

            format_char = *f as c_int;
            match format_char {
                b'%' => {
                    if f.offset(-1) != percent {
                        f = f.offset(-1);
                        continue;
                    } else {
                        let n = 1;
                        let w = if pad == b'-' as c_int || width < 0 {
                            0
                        } else {
                            width
                        } as size_t;
                        let incr = if n < w { w } else { n };
                        if incr >= maxsize.wrapping_sub(i) {
                            *__errno_location() = 34;
                            return 0;
                        }
                        if !p.is_null() {
                            if n < w {
                                let delta = w - n;
                                if pad == b'0' as c_int || pad == b'+' as c_int {
                                    memset(p as *mut c_void, b'0' as c_int, delta);
                                    p = p.add(delta);
                                } else {
                                    memset(p as *mut c_void, b' ' as c_int, delta);
                                    p = p.add(delta);
                                }
                            }
                            *p = *f;
                            p = p.add(n);
                        }
                        i = i.wrapping_add(incr);
                    }
                }
                b'a' | b'A' => {
                    if modifier != 0 {
                        continue;
                    }
                    if change_case {
                        to_uppcase = true;
                        to_lowcase = false;
                    }
                    // Handle a/A format
                }
                b'b' | b'h' | b'B' => {
                    if change_case {
                        to_uppcase = true;
                        to_lowcase = false;
                    }
                    if modifier == b'E' as c_int {
                        continue;
                    }
                    // Handle b/h/B format
                }
                b'c' => {
                    if modifier == b'O' as c_int {
                        continue;
                    }
                    // Handle c format
                }
                b'C' => {
                    if modifier == b'E' as c_int {
                        continue;
                    }
                    let negative_year = (*tp).tm_year < -1900;
                    let zero_thru_1899 = !negative_year && (*tp).tm_year < 0;
                    let century = ((*tp).tm_year - 99 * zero_thru_1899 as c_int) / 100 + 1900 / 100;
                    digits = 2;
                    negative_number = negative_year;
                    u_number_value = century as c_uint;
                    // Handle C format
                }
                b'd' | b'e' | b'H' | b'I' | b'k' | b'l' | b'm' | b'M' | b'S' | b'u' | b'U' | b'V' | b'w' | b'W' | b'y' => {
                    // Handle numeric formats
                }
                b'D' => {
                    if modifier != 0 {
                        continue;
                    }
                    subfmt = b"%m/%d/%y\0".as_ptr() as *const c_char;
                    // Handle D format
                }
                b'F' => {
                    if modifier != 0 {
                        continue;
                    }
                    if pad == 0 && width < 0 {
                        pad = b'+' as c_int;
                        subwidth = 4;
                    } else {
                        subwidth = width - 6;
                        if subwidth < 0 {
                            subwidth = 0;
                        }
                    }
                    subfmt = b"%Y-%m-%d\0".as_ptr() as *const c_char;
                    // Handle F format
                }
                b'g' | b'G' | b'Y' => {
                    if modifier == b'E' as c_int {
                        continue;
                    }
                    // Handle g/G/Y formats
                }
                b'j' => {
                    if modifier == b'E' as c_int {
                        continue;
                    }
                    digits = 3;
                    negative_number = (*tp).tm_yday < -1;
                    u_number_value = ((*tp).tm_yday as c_uint).wrapping_add(1);
                    // Handle j format
                }
                b'n' => {
                    let n = 1;
                    let w = if pad == b'-' as c_int || width < 0 {
                        0
                    } else {
                        width
                    } as size_t;
                    let incr = if n < w { w } else { n };
                    if incr >= maxsize.wrapping_sub(i) {
                        *__errno_location() = 34;
                        return 0;
                    }
                    if !p.is_null() {
                        if n < w {
                            let delta = w - n;
                            if pad == b'0' as c_int || pad == b'+' as c_int {
                                memset(p as *mut c_void, b'0' as c_int, delta);
                                p = p.add(delta);
                            } else {
                                memset(p as *mut c_void, b' ' as c_int, delta);
                                p = p.add(delta);
                            }
                        }
                        *p = b'\n' as c_char;
                        p = p.add(n);
                    }
                    i = i.wrapping_add(incr);
                }
                b'N' => {
                    if modifier == b'E' as c_int {
                        continue;
                    }
                    let mut n = ns;
                    let ns_digits = 9;
                    if width <= 0 {
                        width = ns_digits;
                    }
                    let mut ndigs = ns_digits;
                    while width < ndigs || (1 < ndigs && n % 10 == 0) {
                        ndigs -= 1;
                        n /= 10;
                    }
                    let mut j = ndigs;
                    while 0 < j {
                        buf[(j - 1) as usize] = (n % 10 + b'0' as c_int) as c_char;
                        n /= 10;
                        j -= 1;
                    }
                    if pad == 0 {
                        pad = b'0' as c_int;
                    }
                    let n = ndigs as size_t;
                    let w = if pad == b'-' as c_int || 0 < 0 {
                        0
                    } else {
                        0
                    } as size_t;
                    let incr = if n < w { w } else { n };
                    if incr >= maxsize.wrapping_sub(i) {
                        *__errno_location() = 34;
                        return 0;
                    }
                    if !p.is_null() {
                        if n < w {
                            let delta = w - n;
                            if pad == b'0' as c_int || pad == b'+' as c_int {
                                memset(p as *mut c_void, b'0' as c_int, delta);
                                p = p.add(delta);
                            } else {
                                memset(p as *mut c_void, b' ' as c_int, delta);
                                p = p.add(delta);
                            }
                        }
                        if to_lowcase {
                            memcpy_lowcase(p, buf.as_mut_ptr(), n);
                        } else if to_uppcase {
                            memcpy_uppcase(p, buf.as_mut_ptr(), n);
                        } else {
                            memcpy(p as *mut c_void, buf.as_mut_ptr() as *const c_void, n);
                        }
                        p = p.add(n);
                    }
                    i = i.wrapping_add(incr);
                    let n = 0;
                    let w = if pad == b'-' as c_int || width - ndigs < 0 {
                        0
                    } else {
                        width - ndigs
                    } as size_t;
                    let incr = if n < w { w } else { n };
                    if incr >= maxsize.wrapping_sub(i) {
                        *__errno_location() = 34;
                        return 0;
                    }
                    if !p.is_null() {
                        if n < w {
                            let delta = w - n;
                            if pad == b'0' as c_int || pad == b'+' as c_int {
                                memset(p as *mut c_void, b'0' as c_int, delta);
                                p = p.add(delta);
                            } else {
                                memset(p as *mut c_void, b' ' as c_int, delta);
                                p = p.add(delta);
                            }
                        }
                        p = p.add(n);
                    }
                    i = i.wrapping_add(incr);
                }
                b'P' => {
                    to_lowcase = true;
                    format_char = b'p' as c_int;
                    // Handle P format
                }
                b'p' => {
                    // Handle p format
                }
                b'q' => {
                    digits = 1;
                    negative_number = false;
                    u_number_value = (((*tp).tm_mon * 11 >> 5) + 1) as c_uint;
                    // Handle q format
                }
                b'r' => {
                    // Handle r format
                }
                b'R' => {
                    subfmt = b"%H:%M\0".as_ptr() as *const c_char;
                    // Handle R format
                }
                b's' => {
                    let mut ltm = tm {
                        tm_sec: 0,
                        tm_min: 0,
                        tm_hour: 0,
                        tm_mday: 0,
                        tm_mon: 0,
                        tm_year: 0,
                        tm_wday: 0,
                        tm_yday: 0,
                        tm_isdst: 0,
                        tm_gmtoff: 0,
                        tm_zone: ptr::null(),
                    };
                    let mut t = 0;
                    ltm = *tp;
                    ltm.tm_yday = -1;
                    t = mktime_z(tz, &mut ltm);
                    if ltm.tm_yday < 0 {
                        *__errno_location() = 75;
                        return 0;
                    }
                    bufp = buf.as_mut_ptr().add(buf.len());
                    negative_number = t < 0;
                    loop {
                        let d = (t % 10) as c_int;
                        t /= 10;
                        bufp = bufp.offset(-1);
                        *bufp = (if negative_number { -d } else { d } + b'0' as c_int) as c_char;
                        if t == 0 {
                            break;
                        }
                    }
                    digits = 1;
                    always_output_a_sign = false;
                    // Handle s format
                }
                b'T' => {
                    subfmt = b"%H:%M:%S\0".as_ptr() as *const c_char;
                    // Handle T format
                }
                b't' => {
                    let n = 1;
                    let w = if pad == b'-' as c_int || width < 0 {
                        0
                    } else {
                        width
                    } as size_t;
                    let incr = if n < w { w } else { n };
                    if incr >= maxsize.wrapping_sub(i) {
                        *__errno_location() = 34;
                        return 0;
                    }
                    if !p.is_null() {
                        if n < w {
                            let delta = w - n;
                            if pad == b'0' as c_int || pad == b'+' as c_int {
                                memset(p as *mut c_void, b'0' as c_int, delta);
                                p = p.add(delta);
                            } else {
                                memset(p as *mut c_void, b' ' as c_int, delta);
                                p = p.add