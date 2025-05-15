use std::{
    ffi::{CStr, CString},
    fmt,
    io::{self, Write},
    mem,
    os::raw::{c_char, c_int, c_void},
    ptr,
};

#[repr(C)]
struct Directive {
    dir_start: *const c_char,
    dir_end: *const c_char,
    conversion: c_char,
    arg_index: c_int,
    width_start: *const c_char,
    width_end: *const c_char,
    width_arg_index: c_int,
    precision_start: *const c_char,
    precision_end: *const c_char,
    precision_arg_index: c_int,
    flags: c_int,
}

#[repr(C)]
struct Directives {
    dir: *mut Directive,
    count: c_int,
    max_width_length: c_int,
    max_precision_length: c_int,
}

#[repr(C)]
struct Arguments {
    arg: *mut Argument,
}

#[repr(C)]
union ArgumentData {
    a_schar: i8,
    a_uchar: u8,
    a_short: i16,
    a_ushort: u16,
    a_int: i32,
    a_uint: u32,
    a_longint: i64,
    a_ulongint: u64,
    a_longlongint: i64,
    a_ulonglongint: u64,
    a_double: f64,
    a_longdouble: f64,
    a_char: c_char,
    a_wide_char: u32,
    a_string: *const c_char,
    a_wide_string: *const u32,
    a_pointer: *mut c_void,
    a_count_schar_pointer: *mut i8,
    a_count_short_pointer: *mut i16,
    a_count_int_pointer: *mut i32,
    a_count_longint_pointer: *mut i64,
    a_count_longlongint_pointer: *mut i64,
    a_u8_string: *const u8,
    a_u16_string: *const u16,
    a_u32_string: *const u32,
}

#[repr(C)]
struct Argument {
    type_: c_int,
    a: ArgumentData,
}

const TYPE_SCHAR: c_int = 0;
const TYPE_UCHAR: c_int = 1;
const TYPE_SHORT: c_int = 2;
const TYPE_USHORT: c_int = 3;
const TYPE_INT: c_int = 4;
const TYPE_UINT: c_int = 5;
const TYPE_LONGINT: c_int = 6;
const TYPE_ULONGINT: c_int = 7;
const TYPE_LONGLONGINT: c_int = 8;
const TYPE_ULONGLONGINT: c_int = 9;
const TYPE_DOUBLE: c_int = 10;
const TYPE_LONGDOUBLE: c_int = 11;
const TYPE_CHAR: c_int = 12;
const TYPE_WIDE_CHAR: c_int = 13;
const TYPE_STRING: c_int = 14;
const TYPE_WIDE_STRING: c_int = 15;
const TYPE_POINTER: c_int = 16;
const TYPE_COUNT_SCHAR_POINTER: c_int = 17;
const TYPE_COUNT_SHORT_POINTER: c_int = 18;
const TYPE_COUNT_INT_POINTER: c_int = 19;
const TYPE_COUNT_LONGINT_POINTER: c_int = 20;
const TYPE_COUNT_LONGLONGINT_POINTER: c_int = 21;
const TYPE_U8_STRING: c_int = 22;
const TYPE_U16_STRING: c_int = 23;
const TYPE_U32_STRING: c_int = 24;

const FLAG_LEFT: c_int = 1;
const FLAG_SHOWSIGN: c_int = 2;
const FLAG_SPACE: c_int = 4;
const FLAG_ALT: c_int = 8;
const FLAG_ZERO: c_int = 16;
const FLAG_GROUP: c_int = 32;

const ARG_NONE: c_int = -1;

extern "C" {
    fn printf_parse(
        format: *const c_char,
        d: *mut Directives,
        a: *mut Arguments,
    ) -> c_int;
    fn printf_fetchargs(args: *mut c_void, a: *mut Arguments) -> c_int;
}

pub fn vasnprintf(
    resultbuf: Option<&mut [u8]>,
    lengthp: &mut usize,
    format: &CStr,
    args: *mut c_void,
) -> Result<Vec<u8>, io::Error> {
    unsafe {
        let mut d = Directives {
            dir: ptr::null_mut(),
            count: 0,
            max_width_length: 0,
            max_precision_length: 0,
        };
        let mut a = Arguments {
            arg: ptr::null_mut(),
        };

        if printf_parse(format.as_ptr(), &mut d, &mut a) < 0 {
            return Err(io::Error::last_os_error());
        }

        if printf_fetchargs(args, &mut a) < 0 {
            free_directives(&mut d);
            free_arguments(&mut a);
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid arguments"));
        }

        let mut result = if let Some(buf) = resultbuf {
            buf.to_vec()
        } else {
            Vec::new()
        };
        let mut length = 0;

        let mut cp = format.as_ptr();
        for i in 0..d.count {
            let dp = &*d.dir.offset(i as isize);

            if cp != dp.dir_start {
                let n = dp.dir_start.offset_from(cp) as usize;
                result.resize(length + n, 0);
                ptr::copy_nonoverlapping(cp, result.as_mut_ptr().add(length), n);
                length += n;
            }

            cp = dp.dir_end;

            if dp.conversion == b'%' as c_char {
                if dp.arg_index != ARG_NONE {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "invalid argument index for %",
                    ));
                }
                result.push(b'%');
                length += 1;
            } else if dp.conversion == b'n' as c_char {
                if dp.arg_index == ARG_NONE {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "missing argument index for %n",
                    ));
                }
                let arg = &mut *a.arg.offset(dp.arg_index as isize);
                match arg.type_ {
                    TYPE_COUNT_SCHAR_POINTER => *arg.a.a_count_schar_pointer = length as i8,
                    TYPE_COUNT_SHORT_POINTER => *arg.a.a_count_short_pointer = length as i16,
                    TYPE_COUNT_INT_POINTER => *arg.a.a_count_int_pointer = length as i32,
                    TYPE_COUNT_LONGINT_POINTER => *arg.a.a_count_longint_pointer = length as i64,
                    TYPE_COUNT_LONGLONGINT_POINTER => {
                        *arg.a.a_count_longlongint_pointer = length as i64
                    }
                    _ => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "invalid argument type for %n",
                        ))
                    }
                }
            } else {
                if dp.arg_index == ARG_NONE {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "missing argument index",
                    ));
                }

                let arg = &*a.arg.offset(dp.arg_index as isize);
                let mut buf = Vec::new();
                let mut writer = Cursor::new(&mut buf);

                match arg.type_ {
                    TYPE_SCHAR => write!(writer, "{}", arg.a.a_schar)?,
                    TYPE_UCHAR => write!(writer, "{}", arg.a.a_uchar)?,
                    TYPE_SHORT => write!(writer, "{}", arg.a.a_short)?,
                    TYPE_USHORT => write!(writer, "{}", arg.a.a_ushort)?,
                    TYPE_INT => write!(writer, "{}", arg.a.a_int)?,
                    TYPE_UINT => write!(writer, "{}", arg.a.a_uint)?,
                    TYPE_LONGINT => write!(writer, "{}", arg.a.a_longint)?,
                    TYPE_ULONGINT => write!(writer, "{}", arg.a.a_ulongint)?,
                    TYPE_LONGLONGINT => write!(writer, "{}", arg.a.a_longlongint)?,
                    TYPE_ULONGLONGINT => write!(writer, "{}", arg.a.a_ulonglongint)?,
                    TYPE_DOUBLE => write!(writer, "{}", arg.a.a_double)?,
                    TYPE_LONGDOUBLE => write!(writer, "{}", arg.a.a_longdouble)?,
                    TYPE_CHAR => write!(writer, "{}", arg.a.a_char as u8 as char)?,
                    TYPE_WIDE_CHAR => write!(writer, "{}", char::from_u32(arg.a.a_wide_char).unwrap())?,
                    TYPE_STRING => {
                        let s = CStr::from_ptr(arg.a.a_string);
                        writer.write_all(s.to_bytes())?
                    }
                    TYPE_WIDE_STRING => {
                        let mut p = arg.a.a_wide_string;
                        while *p != 0 {
                            let ch = char::from_u32(*p).unwrap();
                            write!(writer, "{}", ch)?;
                            p = p.offset(1);
                        }
                    }
                    TYPE_POINTER => write!(writer, "{:p}", arg.a.a_pointer)?,
                    _ => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "unsupported argument type",
                        ))
                    }
                }

                result.extend_from_slice(&buf);
                length += buf.len();
            }
        }

        if cp != format.as_ptr().add(format.to_bytes().len()) {
            let n = format.as_ptr().add(format.to_bytes().len()).offset_from(cp) as usize;
            result.resize(length + n, 0);
            ptr::copy_nonoverlapping(cp, result.as_mut_ptr().add(length), n);
            length += n;
        }

        free_directives(&mut d);
        free_arguments(&mut a);

        *lengthp = length;
        Ok(result)
    }
}

unsafe fn free_directives(d: &mut Directives) {
    if !d.dir.is_null() {
        Vec::from_raw_parts(d.dir, d.count as usize, d.count as usize);
        d.dir = ptr::null_mut();
    }
}

unsafe fn free_arguments(a: &mut Arguments) {
    if !a.arg.is_null() {
        Vec::from_raw_parts(a.arg, 0, 0);
        a.arg = ptr::null_mut();
    }
}

struct Cursor<'a> {
    buf: &'a mut Vec<u8>,
}

impl<'a> Cursor<'a> {
    fn new(buf: &'a mut Vec<u8>) -> Self {
        Self { buf }
    }
}

impl<'a> Write for Cursor<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}