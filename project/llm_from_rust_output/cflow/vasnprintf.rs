use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_void};
use std::ptr;
use std::mem;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: c_uint,
    pub fp_offset: c_uint,
    pub overflow_arg_area: *mut c_void,
    pub reg_save_area: *mut c_void,
}

pub type size_t = c_ulong;
pub type wchar_t = c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct argument {
    pub type_: arg_type,
    pub a: C2RustUnnamed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed {
    pub a_schar: i8,
    pub a_uchar: u8,
    pub a_short: i16,
    pub a_ushort: u16,
    pub a_int: c_int,
    pub a_uint: c_uint,
    pub a_longint: i64,
    pub a_ulongint: u64,
    pub a_longlongint: i64,
    pub a_ulonglongint: u64,
    pub a_float: f32,
    pub a_double: f64,
    pub a_longdouble: f128,
    pub a_char: c_int,
    pub a_wide_char: wint_t,
    pub a_string: *const c_char,
    pub a_wide_string: *const wchar_t,
    pub a_pointer: *mut c_void,
    pub a_count_schar_pointer: *mut i8,
    pub a_count_short_pointer: *mut i16,
    pub a_count_int_pointer: *mut c_int,
    pub a_count_longint_pointer: *mut i64,
    pub a_count_longlongint_pointer: *mut i64,
}

pub type wint_t = c_uint;

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arg_type {
    TYPE_NONE = 0,
    TYPE_SCHAR = 1,
    TYPE_UCHAR = 2,
    TYPE_SHORT = 3,
    TYPE_USHORT = 4,
    TYPE_INT = 5,
    TYPE_UINT = 6,
    TYPE_LONGINT = 7,
    TYPE_ULONGINT = 8,
    TYPE_LONGLONGINT = 9,
    TYPE_ULONGLONGINT = 10,
    TYPE_DOUBLE = 11,
    TYPE_LONGDOUBLE = 12,
    TYPE_CHAR = 13,
    TYPE_WIDE_CHAR = 14,
    TYPE_STRING = 15,
    TYPE_WIDE_STRING = 16,
    TYPE_POINTER = 17,
    TYPE_COUNT_SCHAR_POINTER = 18,
    TYPE_COUNT_SHORT_POINTER = 19,
    TYPE_COUNT_INT_POINTER = 20,
    TYPE_COUNT_LONGINT_POINTER = 21,
    TYPE_COUNT_LONGLONGINT_POINTER = 22,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arguments {
    pub count: size_t,
    pub arg: *mut argument,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct char_directive {
    pub dir_start: *const c_char,
    pub dir_end: *const c_char,
    pub flags: c_int,
    pub width_start: *const c_char,
    pub width_end: *const c_char,
    pub width_arg_index: size_t,
    pub precision_start: *const c_char,
    pub precision_end: *const c_char,
    pub precision_arg_index: size_t,
    pub conversion: c_char,
    pub arg_index: size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct char_directives {
    pub count: size_t,
    pub dir: *mut char_directive,
    pub max_width_length: size_t,
    pub max_precision_length: size_t,
}

#[inline]
fn xsum(size1: size_t, size2: size_t) -> size_t {
    size1.checked_add(size2).unwrap_or(size_t::MAX)
}

#[inline]
fn xmax(size1: size_t, size2: size_t) -> size_t {
    if size1 >= size2 { size1 } else { size2 }
}

#[inline]
fn xsum4(size1: size_t, size2: size_t, size3: size_t, size4: size_t) -> size_t {
    xsum(xsum(xsum(size1, size2), size3), size4)
}

pub fn vasnprintf(
    resultbuf: Option<&mut [u8]>,
    lengthp: &mut size_t,
    format: &CStr,
    args: &mut __va_list_tag,
) -> Option<CString> {
    unsafe {
        let mut d = char_directives {
            count: 0,
            dir: ptr::null_mut(),
            max_width_length: 0,
            max_precision_length: 0,
        };
        let mut a = arguments {
            count: 0,
            arg: ptr::null_mut(),
        };

        if printf_parse(format.as_ptr(), &mut d, &mut a) < 0 {
            return None;
        }

        if printf_fetchargs(args, &mut a) < 0 {
            free(d.dir as *mut c_void);
            if !a.arg.is_null() {
                free(a.arg as *mut c_void);
            }
            *__errno_location() = 22;
            return None;
        }

        let buf_neededlength = xsum4(
            7,
            d.max_width_length,
            d.max_precision_length,
            6,
        );

        let (buf, buf_malloced) = if buf_neededlength < 4000 / mem::size_of::<c_char>() {
            let mut buf = vec![0; buf_neededlength as usize];
            (buf.as_mut_ptr(), None)
        } else {
            let buf_memsize = buf_neededlength.checked_mul(mem::size_of::<c_char>())
                .unwrap_or(size_t::MAX);
            if buf_memsize == size_t::MAX {
                free(d.dir as *mut c_void);
                if !a.arg.is_null() {
                    free(a.arg as *mut c_void);
                }
                *__errno_location() = 12;
                return None;
            }
            let buf = malloc(buf_memsize) as *mut c_char;
            if buf.is_null() {
                free(d.dir as *mut c_void);
                if !a.arg.is_null() {
                    free(a.arg as *mut c_void);
                }
                *__errno_location() = 12;
                return None;
            }
            (buf, Some(buf))
        };

        let (mut result, mut allocated) = if let Some(rb) = resultbuf {
            (rb.as_mut_ptr(), rb.len() as size_t)
        } else {
            (ptr::null_mut(), 0)
        };

        let mut length = 0;
        let mut cp = format.as_ptr();
        let mut i = 0;
        let mut dp = d.dir;

        while i < d.count {
            let dir = &*dp;

            if cp != dir.dir_start {
                let n = unsafe { dir.dir_start.offset_from(cp) as size_t };
                let augmented_length = xsum(length, n);

                if augmented_length > allocated {
                    allocated = if allocated > 0 {
                        allocated.checked_mul(2).unwrap_or(size_t::MAX)
                    } else {
                        12
                    };
                    if augmented_length > allocated {
                        allocated = augmented_length;
                    }

                    let memory_size = allocated.checked_mul(mem::size_of::<c_char>())
                        .unwrap_or(size_t::MAX);
                    if memory_size == size_t::MAX {
                        break;
                    }

                    let memory = if result.is_null() || result == resultbuf.unwrap_or(&mut []).as_mut_ptr() {
                        malloc(memory_size) as *mut c_char
                    } else {
                        realloc(result as *mut c_void, memory_size) as *mut c_char
                    };

                    if memory.is_null() {
                        break;
                    }

                    if result == resultbuf.unwrap_or(&mut []).as_mut_ptr() && length > 0 {
                        memcpy(
                            memory as *mut c_void,
                            result as *const c_void,
                            length,
                        );
                    }
                    result = memory;
                }

                unsafe {
                    memcpy(
                        result.add(length as usize) as *mut c_void,
                        cp as *const c_void,
                        n,
                    );
                }
                length = augmented_length;
            }

            if dir.conversion as i32 == b'%' as i32 {
                if dir.arg_index != !0 {
                    abort();
                }

                let augmented_length = xsum(length, 1);
                if augmented_length > allocated {
                    allocated = if allocated > 0 {
                        allocated.checked_mul(2).unwrap_or(size_t::MAX)
                    } else {
                        12
                    };
                    if augmented_length > allocated {
                        allocated = augmented_length;
                    }

                    let memory_size = allocated.checked_mul(mem::size_of::<c_char>())
                        .unwrap_or(size_t::MAX);
                    if memory_size == size_t::MAX {
                        break;
                    }

                    let memory = if result.is_null() || result == resultbuf.unwrap_or(&mut []).as_mut_ptr() {
                        malloc(memory_size) as *mut c_char
                    } else {
                        realloc(result as *mut c_void, memory_size) as *mut c_char
                    };

                    if memory.is_null() {
                        break;
                    }

                    if result == resultbuf.unwrap_or(&mut []).as_mut_ptr() && length > 0 {
                        memcpy(
                            memory as *mut c_void,
                            result as *const c_void,
                            length,
                        );
                    }
                    result = memory;
                }

                unsafe {
                    *result.add(length as usize) = b'%' as c_char;
                }
                length = augmented_length;
            } else {
                if dir.arg_index == !0 {
                    abort();
                }

                if dir.conversion as i32 == b'n' as i32 {
                    let arg = unsafe { &*a.arg.add(dir.arg_index as usize) };
                    match arg.type_ {
                        arg_type::TYPE_COUNT_SCHAR_POINTER => unsafe {
                            *arg.a.a_count_schar_pointer = length as i8;
                        },
                        arg_type::TYPE_COUNT_SHORT_POINTER => unsafe {
                            *arg.a.a_count_short_pointer = length as i16;
                        },
                        arg_type::TYPE_COUNT_INT_POINTER => unsafe {
                            *arg.a.a_count_int_pointer = length as c_int;
                        },
                        arg_type::TYPE_COUNT_LONGINT_POINTER => unsafe {
                            *arg.a.a_count_longint_pointer = length as i64;
                        },
                        arg_type::TYPE_COUNT_LONGLONGINT_POINTER => unsafe {
                            *arg.a.a_count_longlongint_pointer = length as i64;
                        },
                        _ => abort(),
                    }
                } else {
                    let arg = unsafe { &*a.arg.add(dir.arg_index as usize) };
                    let type_ = arg.type_;
                    let flags = dir.flags;
                    let mut fbp = buf;
                    unsafe {
                        *fbp = b'%' as c_char;
                        fbp = fbp.add(1);
                    }

                    if flags & 1 != 0 {
                        unsafe {
                            *fbp = b'\'' as c_char;
                            fbp = fbp.add(1);
                        }
                    }
                    if flags & 2 != 0 {
                        unsafe {
                            *fbp = b'-' as c_char;
                            fbp = fbp.add(1);
                        }
                    }
                    if flags & 4 != 0 {
                        unsafe {
                            *fbp = b'+' as c_char;
                            fbp = fbp.add(1);
                        }
                    }
                    if flags & 8 != 0 {
                        unsafe {
                            *fbp = b' ' as c_char;
                            fbp = fbp.add(1);
                        }
                    }
                    if flags & 16 != 0 {
                        unsafe {
                            *fbp = b'#' as c_char;
                            fbp = fbp.add(1);
                        }
                    }

                    if flags & 32 != 0 {
                        unsafe {
                            *fbp = b'0' as c_char;
                            fbp = fbp.add(1);
                        }
                    }

                    if dir.width_start != dir.width_end {
                        let n = unsafe { dir.width_end.offset_from(dir.width_start) as size_t };
                        unsafe {
                            memcpy(
                                fbp as *mut c_void,
                                dir.width_start as *const c_void,
                                n,
                            );
                            fbp = fbp.add(n as usize);
                        }
                    }

                    if dir.precision_start != dir.precision_end {
                        let n = unsafe { dir.precision_end.offset_from(dir.precision_start) as size_t };
                        unsafe {
                            memcpy(
                                fbp as *mut c_void,
                                dir.precision_start as *const c_void,
                                n,
                            );
                            fbp = fbp.add(n as usize);
                        }
                    }

                    match type_ {
                        arg_type::TYPE_LONGLONGINT | arg_type::TYPE_ULONGLONGINT => unsafe {
                            *fbp = b'l' as c_char;
                            fbp = fbp.add(1);
                        },
                        arg_type::TYPE_LONGINT | arg_type::TYPE_ULONGINT |
                        arg_type::TYPE_WIDE_CHAR | arg_type::TYPE_WIDE_STRING => {}
                        arg_type::TYPE_LONGDOUBLE => unsafe {
                            *fbp = b'L' as c_char;
                            fbp = fbp.add(1);
                        },
                        _ => {}
                    }

                    unsafe {
                        *fbp = dir.conversion;
                        *fbp.add(1) = 0;
                    }

                    let mut prefix_count = 0;
                    let mut prefixes = [0; 2];

                    if dir.width_arg_index != !0 {
                        let arg = unsafe { &*a.arg.add(dir.width_arg_index as usize) };
                        if arg.type_ != arg_type::TYPE_INT {
                            abort();
                        }
                        prefixes[prefix_count] = unsafe { arg.a.a_int };
                        prefix_count += 1;
                    }

                    if dir.precision_arg_index != !0 {
                        let arg = unsafe { &*a.arg.add(dir.precision_arg_index as usize) };
                        if arg.type_ != arg_type::TYPE_INT {
                            abort();
                        }
                        prefixes[prefix_count] = unsafe { arg.a.a_int };
                        prefix_count += 1;
                    }

                    if xsum(length, 2 + mem::size_of::<c_char>() / mem::size_of::<c_char>() - 1) > allocated {
                        allocated = if allocated > 0 {
                            allocated.checked_mul(2).unwrap_or(size_t::MAX)
                        } else {
                            12
                        };
                        if xsum(length, 2 + mem::size_of::<c_char>() / mem::size_of::<c_char>() - 1) > allocated {
                            allocated = xsum(length, 2 + mem::size_of::<c_char>() / mem::size_of::<c_char>() - 1);
                        }

                        let memory_size = allocated.checked_mul(mem::size_of::<c_char>())
                            .unwrap_or(size_t::MAX);
                        if memory_size == size_t::MAX {
                            break;
                        }

                        let memory = if result.is_null() || result == resultbuf.unwrap_or(&mut []).as_mut_ptr() {
                            malloc(memory_size) as *mut c_char
                        } else {
                            realloc(result as *mut c_void, memory_size) as *mut c_char
                        };

                        if memory.is_null() {
                            break;
                        }

                        if result == resultbuf.unwrap_or(&mut []).as_mut_ptr() && length > 0 {
                            memcpy(
                                memory as *mut c_void,
                                result as *const c_void,
                                length,
                            );
                        }
                        result = memory;
                    }

                    unsafe {
                        *result.add(length as usize) = 0;
                    }

                    loop {
                        let mut count = -1;
                        let mut retcount = 0;
                        let mut maxlen = allocated - length;
                        if maxlen > (i32::MAX as size_t) / (mem::size_of::<c_char>() / mem::size_of::<c_char>()) {
                            maxlen = (i32::MAX as size_t) / (mem::size_of::<c_char>() / mem::size_of::<c_char>());
                        }
                        maxlen *= mem::size_of::<c_char>() / mem::size_of::<c_char>();

                        match type_ {
                            arg_type::TYPE_SCHAR => {
                                let arg = unsafe { (*a.arg.add(dir.arg_index as usize)).a.a_schar as c_int };
                                retcount = match prefix_count {
                                    0 => snprintf(
                                        result.add(length as usize),
                                        maxlen,
                                        buf,
                                        arg,
                                        &mut count as *mut c_int,
                                    ),
                                    1 => snprintf(
                                        result.add(length as usize),
                                        maxlen,
                                        buf,
                                        prefixes[0],
                                        arg,
                                        &mut count as *mut c_int,
                                    ),
                                    2 => snprintf(
                                        result.add(length as usize),
                                        maxlen,
                                        buf,
                                        prefixes[0],
                                        prefixes[1],
                                        arg,
                                        &mut count as *mut c_int,
                                    ),
                                    _ => abort(),
                                };
                            }
                            // ... other type cases similar to original C code
                            _ => abort(),
                        }

                        if count >= 0 {
                            if (count as size_t) < maxlen && unsafe { *result.add(length as usize + count as usize) } != 0 {
                                abort();
                            }
                            if retcount > count {
                                count = retcount;
                            }
                        } else if unsafe { *fbp.add(1) } != 0 {
                            unsafe { *fbp.add(1) = 0; }
                            continue;
                        } else if retcount < 0 {
                            let bigger_need = xsum(
                                allocated.checked_mul(2).unwrap_or(size_t::MAX),
                                12,
                            );
                            if !(bigger_need >