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
    fn mt_basename(filename: *const i8) -> *const i8;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strpbrk(_: *const i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strcasecmp(_: *const i8, _: *const i8) -> i32;
    static mut mversion: *const i8;
    static mut mdate: *const i8;
}
pub type __int32_t = i32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
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
#[inline]
unsafe extern "C" fn tolower(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn ch_tolower(mut ch: i8) -> i8 {
    return ({
        let mut __res: i32 = 0;
        if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
            if 0 != 0 {
                let mut __c: i32 = ch as u8 as i32;
                __res = if __c < -(128 as i32) || __c > 255 as i32 {
                    __c
                } else {
                    *(*__ctype_tolower_loc()).offset(__c as isize)
                };
            } else {
                __res = tolower(ch as u8 as i32);
            }
        } else {
            __res = *(*__ctype_tolower_loc()).offset(ch as u8 as i32 as isize);
        }
        __res
    }) as i8;
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut i: i32 = 0;
    let mut name: *const i8 = 0 as *const i8;
    let mut new_name: *mut i8 = 0 as *mut i8;
    if argc >= 2 as i32
        && strcmp(*argv.offset(1 as i32 as isize), b"-V\0" as *const u8 as *const i8)
            == 0 as i32
    {
        printf(
            b"Mtools version %s, dated %s\n\0" as *const u8 as *const i8,
            mversion,
            mdate,
        );
        return 0 as i32;
    }
    if argc == 1 as i32 {
        fprintf(
            stderr,
            b"Usage: mkmanifest [-V] <list-of-files>\n\0" as *const u8 as *const i8,
        );
        return 1 as i32;
    }
    i = 1 as i32;
    while i < argc {
        name = mt_basename(*argv.offset(i as isize));
        new_name = dos_name2(name);
        if strcasecmp(new_name, name) != 0 {
            printf(b"mv %s %s\n\0" as *const u8 as *const i8, new_name, name);
        }
        i += 1;
        i;
    }
    return 0 as i32;
}
unsafe extern "C" fn dos_name2(mut name: *const i8) -> *mut i8 {
    static mut dev: [*const i8; 9] = [
        b"con\0" as *const u8 as *const i8,
        b"aux\0" as *const u8 as *const i8,
        b"com1\0" as *const u8 as *const i8,
        b"com2\0" as *const u8 as *const i8,
        b"lpt1\0" as *const u8 as *const i8,
        b"prn\0" as *const u8 as *const i8,
        b"lpt2\0" as *const u8 as *const i8,
        b"lpt3\0" as *const u8 as *const i8,
        b"nul\0" as *const u8 as *const i8,
    ];
    let mut s: *mut i8 = 0 as *mut i8;
    let mut ext: *mut i8 = 0 as *mut i8;
    let mut temp: *mut i8 = 0 as *mut i8;
    let mut buf: [i8; 128] = [0; 128];
    let mut i: i32 = 0;
    let mut dot: i32 = 0;
    static mut ans: [i8; 13] = [0; 13];
    strncpy(buf.as_mut_ptr(), name, (128 as i32 - 1 as i32) as u64);
    temp = buf.as_mut_ptr();
    ext = 0 as *mut i8;
    dot = 0 as i32;
    i = strlen(buf.as_mut_ptr()) as i32 - 1 as i32;
    while i >= 0 as i32 {
        if buf[i as usize] as i32 == '.' as i32 && dot == 0 {
            dot = 1 as i32;
            buf[i as usize] = '\0' as i32 as i8;
            ext = &mut *buf.as_mut_ptr().offset((i + 1 as i32) as isize) as *mut i8;
        }
        if *(*__ctype_b_loc()).offset(buf[i as usize] as u8 as i32 as isize) as i32
            & _ISupper as i32 as libc::c_ushort as i32 != 0
        {
            buf[i as usize] = ch_tolower(buf[i as usize]);
        }
        i -= 1;
        i;
    }
    if *temp as i32 == '\0' as i32 {
        strcpy(ans.as_mut_ptr(), b"x\0" as *const u8 as *const i8);
    } else {
        i = 0 as i32;
        while i < 9 as i32 {
            if strcasecmp(temp, dev[i as usize]) == 0 {
                *temp = 'x' as i32 as i8;
            }
            i += 1;
            i;
        }
        if strlen(temp) > 8 as i32 as u64 {
            *temp.offset(8 as i32 as isize) = '\0' as i32 as i8;
        }
        if !ext.is_null() && strlen(ext) > 3 as i32 as u64 {
            *ext.offset(3 as i32 as isize) = '\0' as i32 as i8;
        }
        loop {
            s = strpbrk(temp, b"^+=/[]:',?*\\<>|\". \0" as *const u8 as *const i8);
            if s.is_null() {
                break;
            }
            *s = 'x' as i32 as i8;
        }
        while !ext.is_null()
            && {
                s = strpbrk(ext, b"^+=/[]:',?*\\<>|\". \0" as *const u8 as *const i8);
                !s.is_null()
            }
        {
            *s = 'x' as i32 as i8;
        }
        strncpy(ans.as_mut_ptr(), temp, 12 as i32 as u64);
        ans[12 as i32 as usize] = '\0' as i32 as i8;
    }
    if !ext.is_null() && *ext as i32 != 0 {
        strcat(ans.as_mut_ptr(), b".\0" as *const u8 as *const i8);
        strcat(ans.as_mut_ptr(), ext);
    }
    return ans.as_mut_ptr();
}
pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32,
        )
    }
}