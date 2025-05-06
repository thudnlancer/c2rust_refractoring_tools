#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types, label_break_value)]
extern "C" {
    pub type doscp_t;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn wcschr(_: *const i32, _: i32) -> *mut i32;
    fn wcspbrk(__wcs: *const wchar_t, __accept: *const wchar_t) -> *mut wchar_t;
    fn wcslen(_: *const i32) -> u64;
    fn putwc(__wc: wchar_t, __stream: *mut __FILE) -> wint_t;
    fn _IO_putc(__c: i32, __fp: *mut _IO_FILE) -> i32;
    fn getDrive(Stream: *mut Stream_t) -> i8;
    fn getDirentry(Stream: *mut Stream_t) -> *mut direntry_t;
    fn wchar_to_native(
        wchar: *const wchar_t,
        native: *mut i8,
        len: size_t,
        out_len: size_t,
    ) -> size_t;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __ssize_t = i64;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = u64;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type wchar_t = i32;
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
pub type __FILE = _IO_FILE;
pub type FILE = _IO_FILE;
pub type wint_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stream_t {
    pub Class: *mut Class_t,
    pub refs: i32,
    pub Next: *mut Stream_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Class_t {
    pub read: Option<unsafe extern "C" fn(*mut Stream_t, *mut i8, size_t) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(*mut Stream_t, *mut i8, size_t) -> ssize_t>,
    pub pread: Option<
        unsafe extern "C" fn(*mut Stream_t, *mut i8, mt_off_t, size_t) -> ssize_t,
    >,
    pub pwrite: Option<
        unsafe extern "C" fn(*mut Stream_t, *mut i8, mt_off_t, size_t) -> ssize_t,
    >,
    pub flush: Option<unsafe extern "C" fn(*mut Stream_t) -> i32>,
    pub freeFunc: Option<unsafe extern "C" fn(*mut Stream_t) -> i32>,
    pub set_geom: Option<
        unsafe extern "C" fn(*mut Stream_t, *mut device_t, *mut device_t) -> i32,
    >,
    pub get_data: Option<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut time_t,
            *mut mt_off_t,
            *mut i32,
            *mut uint32_t,
        ) -> i32,
    >,
    pub pre_allocate: Option<unsafe extern "C" fn(*mut Stream_t, mt_off_t) -> i32>,
    pub get_dosConvert: Option<unsafe extern "C" fn(*mut Stream_t) -> *mut doscp_t>,
    pub discard: Option<unsafe extern "C" fn(*mut Stream_t) -> i32>,
}
pub type mt_off_t = off_t;
pub type device_t = device;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct device {
    pub name: *const i8,
    pub drive: i8,
    pub fat_bits: i32,
    pub mode: i32,
    pub tracks: u32,
    pub heads: uint16_t,
    pub sectors: uint16_t,
    pub hidden: u32,
    pub offset: mt_off_t,
    pub partition: u32,
    pub misc_flags: u32,
    pub ssize: uint8_t,
    pub use_2m: u32,
    pub precmd: *mut i8,
    pub file_nr: i32,
    pub blocksize: u32,
    pub codepage: u32,
    pub data_map: *const i8,
    pub tot_sectors: uint32_t,
    pub sector_size: uint16_t,
    pub postcmd: *mut i8,
    pub cfg_filename: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct directory {
    pub name: [i8; 8],
    pub ext: [i8; 3],
    pub attr: u8,
    pub Case: u8,
    pub ctime_ms: u8,
    pub ctime: [u8; 2],
    pub cdate: [u8; 2],
    pub adate: [u8; 2],
    pub startHi: [u8; 2],
    pub time: [u8; 2],
    pub date: [u8; 2],
    pub start: [u8; 2],
    pub size: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct direntry_t {
    pub Dir: *mut Stream_t,
    pub entry: i32,
    pub dir: directory,
    pub name: [wchar_t; 256],
    pub beginSlot: u32,
    pub endSlot: u32,
}
#[no_mangle]
pub unsafe extern "C" fn initializeDirentry(
    mut entry: *mut direntry_t,
    mut Dir: *mut Stream_t,
) {
    memset(
        entry as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<direntry_t>() as u64,
    );
    (*entry).entry = -(1 as i32);
    (*entry).Dir = Dir;
    (*entry).beginSlot = 0 as i32 as u32;
    (*entry).endSlot = 0 as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn isNotFound(mut entry: *mut direntry_t) -> i32 {
    return ((*entry).entry == -(2 as i32)) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn isRootEntry(mut entry: *mut direntry_t) -> i32 {
    return ((*entry).entry == -(3 as i32)) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn setEntryForIteration(
    mut entry: *mut direntry_t,
    mut in_0: u32,
) {
    let mut out: i32 = in_0 as i32;
    if out >= 0 as i32 {} else {
        __assert_fail(
            b"out >= 0\0" as *const u8 as *const i8,
            b"direntry.c\0" as *const u8 as *const i8,
            45 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 54],
                &[i8; 54],
            >(b"void setEntryForIteration(direntry_t *, unsigned int)\0"))
                .as_ptr(),
        );
    }
    'c_7352: {
        if out >= 0 as i32 {} else {
            __assert_fail(
                b"out >= 0\0" as *const u8 as *const i8,
                b"direntry.c\0" as *const u8 as *const i8,
                45 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 54],
                    &[i8; 54],
                >(b"void setEntryForIteration(direntry_t *, unsigned int)\0"))
                    .as_ptr(),
            );
        }
    };
    (*entry).entry = out - 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn setEntryToPos(mut entry: *mut direntry_t, mut in_0: u32) {
    let mut out: i32 = in_0 as i32;
    if out >= 0 as i32 {} else {
        __assert_fail(
            b"out >= 0\0" as *const u8 as *const i8,
            b"direntry.c\0" as *const u8 as *const i8,
            51 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 47],
                &[i8; 47],
            >(b"void setEntryToPos(direntry_t *, unsigned int)\0"))
                .as_ptr(),
        );
    }
    'c_7410: {
        if out >= 0 as i32 {} else {
            __assert_fail(
                b"out >= 0\0" as *const u8 as *const i8,
                b"direntry.c\0" as *const u8 as *const i8,
                51 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 47],
                    &[i8; 47],
                >(b"void setEntryToPos(direntry_t *, unsigned int)\0"))
                    .as_ptr(),
            );
        }
    };
    (*entry).entry = out;
}
#[no_mangle]
pub unsafe extern "C" fn getEntryAsPos(mut entry: *mut direntry_t) -> u32 {
    let mut pos: i32 = (*entry).entry;
    if pos >= 0 as i32 {} else {
        __assert_fail(
            b"pos >= 0\0" as *const u8 as *const i8,
            b"direntry.c\0" as *const u8 as *const i8,
            57 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[i8; 41],
            >(b"unsigned int getEntryAsPos(direntry_t *)\0"))
                .as_ptr(),
        );
    }
    'c_7462: {
        if pos >= 0 as i32 {} else {
            __assert_fail(
                b"pos >= 0\0" as *const u8 as *const i8,
                b"direntry.c\0" as *const u8 as *const i8,
                57 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 41],
                    &[i8; 41],
                >(b"unsigned int getEntryAsPos(direntry_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    return pos as u32;
}
#[no_mangle]
pub unsafe extern "C" fn getNextEntryAsPos(mut entry: *mut direntry_t) -> u32 {
    let mut pos: i32 = (*entry).entry + 1 as i32;
    if pos >= 0 as i32 {} else {
        __assert_fail(
            b"pos >= 0\0" as *const u8 as *const i8,
            b"direntry.c\0" as *const u8 as *const i8,
            63 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 45],
                &[i8; 45],
            >(b"unsigned int getNextEntryAsPos(direntry_t *)\0"))
                .as_ptr(),
        );
    }
    'c_7515: {
        if pos >= 0 as i32 {} else {
            __assert_fail(
                b"pos >= 0\0" as *const u8 as *const i8,
                b"direntry.c\0" as *const u8 as *const i8,
                63 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 45],
                    &[i8; 45],
                >(b"unsigned int getNextEntryAsPos(direntry_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    return pos as u32;
}
#[no_mangle]
pub unsafe extern "C" fn getParent(mut entry: *mut direntry_t) -> *mut direntry_t {
    return getDirentry((*entry).Dir);
}
unsafe extern "C" fn getPathLen(mut entry: *mut direntry_t) -> size_t {
    let mut length: size_t = 0 as i32 as size_t;
    loop {
        if isRootEntry(entry) != 0 {
            return length.wrapping_add(3 as i32 as u64);
        }
        length = (length as u64)
            .wrapping_add(
                (1 as i32 as u64).wrapping_add(wcslen(((*entry).name).as_mut_ptr())),
            ) as size_t as size_t;
        entry = getDirentry((*entry).Dir);
    };
}
unsafe extern "C" fn sprintPwd(
    mut entry: *mut direntry_t,
    mut ptr: *mut i8,
    mut len_available: *mut size_t,
) -> *mut i8 {
    if isRootEntry(entry) != 0 {
        let fresh0 = ptr;
        ptr = ptr.offset(1);
        *fresh0 = getDrive((*entry).Dir);
        let fresh1 = ptr;
        ptr = ptr.offset(1);
        *fresh1 = ':' as i32 as i8;
        let fresh2 = ptr;
        ptr = ptr.offset(1);
        *fresh2 = '/' as i32 as i8;
        *len_available = (*len_available as u64).wrapping_sub(3 as i32 as u64) as size_t
            as size_t;
    } else {
        let mut bytes_converted: size_t = 0;
        ptr = sprintPwd(getDirentry((*entry).Dir), ptr, len_available);
        if *ptr.offset(-(1 as i32) as isize) as i32 != '/' as i32 {
            let fresh3 = ptr;
            ptr = ptr.offset(1);
            *fresh3 = '/' as i32 as i8;
            *len_available = (*len_available).wrapping_sub(1);
            *len_available;
        }
        bytes_converted = wchar_to_native(
            ((*entry).name).as_mut_ptr(),
            ptr,
            255 as i32 as size_t,
            *len_available,
        );
        ptr = ptr.offset(bytes_converted as isize);
        *len_available = (*len_available as u64).wrapping_sub(bytes_converted) as size_t
            as size_t;
    }
    return ptr;
}
unsafe extern "C" fn mt_fprintPwd(
    mut f: *mut FILE,
    mut entry: *mut direntry_t,
    mut recurs: i32,
    mut escape: i32,
) {
    if isRootEntry(entry) != 0 {
        _IO_putc(getDrive((*entry).Dir) as i32, f);
        _IO_putc(':' as i32, f);
        if recurs == 0 {
            _IO_putc('/' as i32, f);
        }
    } else {
        mt_fprintPwd(f, getDirentry((*entry).Dir), 1 as i32, escape);
        if escape != 0
            && !(wcspbrk(
                ((*entry).name).as_mut_ptr(),
                (*::core::mem::transmute::<
                    &[u8; 16],
                    &[i32; 4],
                >(b"\"\0\0\0$\0\0\0\\\0\0\0\0\0\0\0"))
                    .as_ptr(),
            ))
                .is_null()
        {
            let mut ptr: *mut wchar_t = 0 as *mut wchar_t;
            _IO_putc('/' as i32, f);
            ptr = ((*entry).name).as_mut_ptr();
            while *ptr != 0 {
                if !(wcschr(
                    (*::core::mem::transmute::<
                        &[u8; 16],
                        &[i32; 4],
                    >(b"\"\0\0\0$\0\0\0\\\0\0\0\0\0\0\0"))
                        .as_ptr(),
                    *ptr,
                ))
                    .is_null()
                {
                    _IO_putc('\\' as i32, f);
                }
                putwc(*ptr, f);
                ptr = ptr.offset(1);
                ptr;
            }
        } else {
            let mut tmp: [i8; 1021] = [0; 1021];
            wchar_to_native(
                ((*entry).name).as_mut_ptr(),
                tmp.as_mut_ptr(),
                255 as i32 as size_t,
                ::core::mem::size_of::<[i8; 1021]>() as u64,
            );
            fprintf(f, b"/%s\0" as *const u8 as *const i8, tmp.as_mut_ptr());
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn fprintPwd(
    mut f: *mut FILE,
    mut entry: *mut direntry_t,
    mut escape: i32,
) {
    if escape != 0 {
        _IO_putc('"' as i32, f);
    }
    mt_fprintPwd(f, entry, 0 as i32, escape);
    if escape != 0 {
        _IO_putc('"' as i32, f);
    }
}
unsafe extern "C" fn mt_fprintShortPwd(
    mut f: *mut FILE,
    mut entry: *mut direntry_t,
    mut recurs: i32,
) {
    if isRootEntry(entry) != 0 {
        _IO_putc(getDrive((*entry).Dir) as i32, f);
        _IO_putc(':' as i32, f);
        if recurs == 0 {
            _IO_putc('/' as i32, f);
        }
    } else {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        mt_fprintShortPwd(f, getDirentry((*entry).Dir), 1 as i32);
        _IO_putc('/' as i32, f);
        i = 7 as i32;
        while i >= 0 as i32 && (*entry).dir.name[i as usize] as i32 == ' ' as i32 {
            i -= 1;
            i;
        }
        j = 0 as i32;
        while j <= i {
            _IO_putc((*entry).dir.name[j as usize] as i32, f);
            j += 1;
            j;
        }
        i = 2 as i32;
        while i >= 0 as i32 && (*entry).dir.ext[i as usize] as i32 == ' ' as i32 {
            i -= 1;
            i;
        }
        if i > 0 as i32 {
            _IO_putc('.' as i32, f);
        }
        j = 0 as i32;
        while j <= i {
            _IO_putc((*entry).dir.ext[j as usize] as i32, f);
            j += 1;
            j;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn fprintShortPwd(mut f: *mut FILE, mut entry: *mut direntry_t) {
    mt_fprintShortPwd(f, entry, 0 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn getPwd(mut entry: *mut direntry_t) -> *mut i8 {
    let mut size: size_t = 0;
    let mut ret: *mut i8 = 0 as *mut i8;
    let mut end: *mut i8 = 0 as *mut i8;
    let mut buf_size: size_t = 0;
    size = getPathLen(entry);
    buf_size = size.wrapping_mul(4 as i32 as u64).wrapping_add(1 as i32 as u64);
    ret = malloc(buf_size) as *mut i8;
    if ret.is_null() {
        return 0 as *mut i8;
    }
    end = sprintPwd(entry, ret, &mut buf_size);
    *end = '\0' as i32 as i8;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn isSubdirOf(
    mut inside: *mut Stream_t,
    mut outside: *mut Stream_t,
) -> i32 {
    loop {
        if inside == outside {
            return 1 as i32;
        }
        if isRootEntry(getDirentry(inside)) != 0 {
            return 0 as i32;
        }
        inside = (*getDirentry(inside)).Dir;
    };
}