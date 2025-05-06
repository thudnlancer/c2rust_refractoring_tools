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
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __errno_location() -> *mut i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: i32, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn unlink(__name: *const i8) -> i32;
    fn rpl_fcntl(fd: i32, action: i32, _: ...) -> i32;
    fn getenv(__name: *const i8) -> *mut i8;
    fn rpl_free(ptr: *mut libc::c_void);
    static mut stderr: *mut _IO_FILE;
    fn perror(__s: *const i8);
    fn rpl_fprintf(fp: *mut FILE, format: *const i8, _: ...) -> i32;
    fn _IO_putc(__c: i32, __fp: *mut _IO_FILE) -> i32;
    static mut inbuf: [uch; 0];
    static mut outbuf: [uch; 0];
    static mut window: [uch; 0];
    static mut insize: u32;
    static mut inptr: u32;
    static mut outcnt: u32;
    static mut bytes_in: off_t;
    static mut bytes_out: off_t;
    static mut ifd: i32;
    static mut ofd: i32;
    static mut ifname: [i8; 0];
    static mut ofname: [i8; 0];
    static mut program_name: *mut i8;
    static mut exit_code: i32;
    static mut quiet: i32;
    static mut test: i32;
    fn abort_gzip();
    fn strspn(_: *const i8, _: *const i8) -> u64;
    fn strcspn(_: *const i8, _: *const i8) -> u64;
    fn last_component(filename: *const i8) -> *mut i8;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
}
pub type __int32_t = i32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __ssize_t = i64;
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
pub type ssize_t = __ssize_t;
pub type size_t = u64;
pub type off_t = __off_t;
pub type voidp = *mut libc::c_void;
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
pub type uch = u8;
pub type ulg = u64;
#[inline]
unsafe extern "C" fn tolower(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
static mut crc_32_tab: [ulg; 256] = [
    0 as i64 as ulg,
    0x77073096 as i64 as ulg,
    0xee0e612c as i64 as ulg,
    0x990951ba as i64 as ulg,
    0x76dc419 as i64 as ulg,
    0x706af48f as i64 as ulg,
    0xe963a535 as i64 as ulg,
    0x9e6495a3 as i64 as ulg,
    0xedb8832 as i64 as ulg,
    0x79dcb8a4 as i64 as ulg,
    0xe0d5e91e as i64 as ulg,
    0x97d2d988 as i64 as ulg,
    0x9b64c2b as i64 as ulg,
    0x7eb17cbd as i64 as ulg,
    0xe7b82d07 as i64 as ulg,
    0x90bf1d91 as i64 as ulg,
    0x1db71064 as i64 as ulg,
    0x6ab020f2 as i64 as ulg,
    0xf3b97148 as i64 as ulg,
    0x84be41de as i64 as ulg,
    0x1adad47d as i64 as ulg,
    0x6ddde4eb as i64 as ulg,
    0xf4d4b551 as i64 as ulg,
    0x83d385c7 as i64 as ulg,
    0x136c9856 as i64 as ulg,
    0x646ba8c0 as i64 as ulg,
    0xfd62f97a as i64 as ulg,
    0x8a65c9ec as i64 as ulg,
    0x14015c4f as i64 as ulg,
    0x63066cd9 as i64 as ulg,
    0xfa0f3d63 as i64 as ulg,
    0x8d080df5 as i64 as ulg,
    0x3b6e20c8 as i64 as ulg,
    0x4c69105e as i64 as ulg,
    0xd56041e4 as i64 as ulg,
    0xa2677172 as i64 as ulg,
    0x3c03e4d1 as i64 as ulg,
    0x4b04d447 as i64 as ulg,
    0xd20d85fd as i64 as ulg,
    0xa50ab56b as i64 as ulg,
    0x35b5a8fa as i64 as ulg,
    0x42b2986c as i64 as ulg,
    0xdbbbc9d6 as i64 as ulg,
    0xacbcf940 as i64 as ulg,
    0x32d86ce3 as i64 as ulg,
    0x45df5c75 as i64 as ulg,
    0xdcd60dcf as i64 as ulg,
    0xabd13d59 as i64 as ulg,
    0x26d930ac as i64 as ulg,
    0x51de003a as i64 as ulg,
    0xc8d75180 as i64 as ulg,
    0xbfd06116 as i64 as ulg,
    0x21b4f4b5 as i64 as ulg,
    0x56b3c423 as i64 as ulg,
    0xcfba9599 as i64 as ulg,
    0xb8bda50f as i64 as ulg,
    0x2802b89e as i64 as ulg,
    0x5f058808 as i64 as ulg,
    0xc60cd9b2 as i64 as ulg,
    0xb10be924 as i64 as ulg,
    0x2f6f7c87 as i64 as ulg,
    0x58684c11 as i64 as ulg,
    0xc1611dab as i64 as ulg,
    0xb6662d3d as i64 as ulg,
    0x76dc4190 as i64 as ulg,
    0x1db7106 as i64 as ulg,
    0x98d220bc as i64 as ulg,
    0xefd5102a as i64 as ulg,
    0x71b18589 as i64 as ulg,
    0x6b6b51f as i64 as ulg,
    0x9fbfe4a5 as i64 as ulg,
    0xe8b8d433 as i64 as ulg,
    0x7807c9a2 as i64 as ulg,
    0xf00f934 as i64 as ulg,
    0x9609a88e as i64 as ulg,
    0xe10e9818 as i64 as ulg,
    0x7f6a0dbb as i64 as ulg,
    0x86d3d2d as i64 as ulg,
    0x91646c97 as i64 as ulg,
    0xe6635c01 as i64 as ulg,
    0x6b6b51f4 as i64 as ulg,
    0x1c6c6162 as i64 as ulg,
    0x856530d8 as i64 as ulg,
    0xf262004e as i64 as ulg,
    0x6c0695ed as i64 as ulg,
    0x1b01a57b as i64 as ulg,
    0x8208f4c1 as i64 as ulg,
    0xf50fc457 as i64 as ulg,
    0x65b0d9c6 as i64 as ulg,
    0x12b7e950 as i64 as ulg,
    0x8bbeb8ea as i64 as ulg,
    0xfcb9887c as i64 as ulg,
    0x62dd1ddf as i64 as ulg,
    0x15da2d49 as i64 as ulg,
    0x8cd37cf3 as i64 as ulg,
    0xfbd44c65 as i64 as ulg,
    0x4db26158 as i64 as ulg,
    0x3ab551ce as i64 as ulg,
    0xa3bc0074 as i64 as ulg,
    0xd4bb30e2 as i64 as ulg,
    0x4adfa541 as i64 as ulg,
    0x3dd895d7 as i64 as ulg,
    0xa4d1c46d as i64 as ulg,
    0xd3d6f4fb as i64 as ulg,
    0x4369e96a as i64 as ulg,
    0x346ed9fc as i64 as ulg,
    0xad678846 as i64 as ulg,
    0xda60b8d0 as i64 as ulg,
    0x44042d73 as i64 as ulg,
    0x33031de5 as i64 as ulg,
    0xaa0a4c5f as i64 as ulg,
    0xdd0d7cc9 as i64 as ulg,
    0x5005713c as i64 as ulg,
    0x270241aa as i64 as ulg,
    0xbe0b1010 as i64 as ulg,
    0xc90c2086 as i64 as ulg,
    0x5768b525 as i64 as ulg,
    0x206f85b3 as i64 as ulg,
    0xb966d409 as i64 as ulg,
    0xce61e49f as i64 as ulg,
    0x5edef90e as i64 as ulg,
    0x29d9c998 as i64 as ulg,
    0xb0d09822 as i64 as ulg,
    0xc7d7a8b4 as i64 as ulg,
    0x59b33d17 as i64 as ulg,
    0x2eb40d81 as i64 as ulg,
    0xb7bd5c3b as i64 as ulg,
    0xc0ba6cad as i64 as ulg,
    0xedb88320 as i64 as ulg,
    0x9abfb3b6 as i64 as ulg,
    0x3b6e20c as i64 as ulg,
    0x74b1d29a as i64 as ulg,
    0xead54739 as i64 as ulg,
    0x9dd277af as i64 as ulg,
    0x4db2615 as i64 as ulg,
    0x73dc1683 as i64 as ulg,
    0xe3630b12 as i64 as ulg,
    0x94643b84 as i64 as ulg,
    0xd6d6a3e as i64 as ulg,
    0x7a6a5aa8 as i64 as ulg,
    0xe40ecf0b as i64 as ulg,
    0x9309ff9d as i64 as ulg,
    0xa00ae27 as i64 as ulg,
    0x7d079eb1 as i64 as ulg,
    0xf00f9344 as i64 as ulg,
    0x8708a3d2 as i64 as ulg,
    0x1e01f268 as i64 as ulg,
    0x6906c2fe as i64 as ulg,
    0xf762575d as i64 as ulg,
    0x806567cb as i64 as ulg,
    0x196c3671 as i64 as ulg,
    0x6e6b06e7 as i64 as ulg,
    0xfed41b76 as i64 as ulg,
    0x89d32be0 as i64 as ulg,
    0x10da7a5a as i64 as ulg,
    0x67dd4acc as i64 as ulg,
    0xf9b9df6f as i64 as ulg,
    0x8ebeeff9 as i64 as ulg,
    0x17b7be43 as i64 as ulg,
    0x60b08ed5 as i64 as ulg,
    0xd6d6a3e8 as i64 as ulg,
    0xa1d1937e as i64 as ulg,
    0x38d8c2c4 as i64 as ulg,
    0x4fdff252 as i64 as ulg,
    0xd1bb67f1 as i64 as ulg,
    0xa6bc5767 as i64 as ulg,
    0x3fb506dd as i64 as ulg,
    0x48b2364b as i64 as ulg,
    0xd80d2bda as i64 as ulg,
    0xaf0a1b4c as i64 as ulg,
    0x36034af6 as i64 as ulg,
    0x41047a60 as i64 as ulg,
    0xdf60efc3 as i64 as ulg,
    0xa867df55 as i64 as ulg,
    0x316e8eef as i64 as ulg,
    0x4669be79 as i64 as ulg,
    0xcb61b38c as i64 as ulg,
    0xbc66831a as i64 as ulg,
    0x256fd2a0 as i64 as ulg,
    0x5268e236 as i64 as ulg,
    0xcc0c7795 as i64 as ulg,
    0xbb0b4703 as i64 as ulg,
    0x220216b9 as i64 as ulg,
    0x5505262f as i64 as ulg,
    0xc5ba3bbe as i64 as ulg,
    0xb2bd0b28 as i64 as ulg,
    0x2bb45a92 as i64 as ulg,
    0x5cb36a04 as i64 as ulg,
    0xc2d7ffa7 as i64 as ulg,
    0xb5d0cf31 as i64 as ulg,
    0x2cd99e8b as i64 as ulg,
    0x5bdeae1d as i64 as ulg,
    0x9b64c2b0 as i64 as ulg,
    0xec63f226 as i64 as ulg,
    0x756aa39c as i64 as ulg,
    0x26d930a as i64 as ulg,
    0x9c0906a9 as i64 as ulg,
    0xeb0e363f as i64 as ulg,
    0x72076785 as i64 as ulg,
    0x5005713 as i64 as ulg,
    0x95bf4a82 as i64 as ulg,
    0xe2b87a14 as i64 as ulg,
    0x7bb12bae as i64 as ulg,
    0xcb61b38 as i64 as ulg,
    0x92d28e9b as i64 as ulg,
    0xe5d5be0d as i64 as ulg,
    0x7cdcefb7 as i64 as ulg,
    0xbdbdf21 as i64 as ulg,
    0x86d3d2d4 as i64 as ulg,
    0xf1d4e242 as i64 as ulg,
    0x68ddb3f8 as i64 as ulg,
    0x1fda836e as i64 as ulg,
    0x81be16cd as i64 as ulg,
    0xf6b9265b as i64 as ulg,
    0x6fb077e1 as i64 as ulg,
    0x18b74777 as i64 as ulg,
    0x88085ae6 as i64 as ulg,
    0xff0f6a70 as i64 as ulg,
    0x66063bca as i64 as ulg,
    0x11010b5c as i64 as ulg,
    0x8f659eff as i64 as ulg,
    0xf862ae69 as i64 as ulg,
    0x616bffd3 as i64 as ulg,
    0x166ccf45 as i64 as ulg,
    0xa00ae278 as i64 as ulg,
    0xd70dd2ee as i64 as ulg,
    0x4e048354 as i64 as ulg,
    0x3903b3c2 as i64 as ulg,
    0xa7672661 as i64 as ulg,
    0xd06016f7 as i64 as ulg,
    0x4969474d as i64 as ulg,
    0x3e6e77db as i64 as ulg,
    0xaed16a4a as i64 as ulg,
    0xd9d65adc as i64 as ulg,
    0x40df0b66 as i64 as ulg,
    0x37d83bf0 as i64 as ulg,
    0xa9bcae53 as i64 as ulg,
    0xdebb9ec5 as i64 as ulg,
    0x47b2cf7f as i64 as ulg,
    0x30b5ffe9 as i64 as ulg,
    0xbdbdf21c as i64 as ulg,
    0xcabac28a as i64 as ulg,
    0x53b39330 as i64 as ulg,
    0x24b4a3a6 as i64 as ulg,
    0xbad03605 as i64 as ulg,
    0xcdd70693 as i64 as ulg,
    0x54de5729 as i64 as ulg,
    0x23d967bf as i64 as ulg,
    0xb3667a2e as i64 as ulg,
    0xc4614ab8 as i64 as ulg,
    0x5d681b02 as i64 as ulg,
    0x2a6f2b94 as i64 as ulg,
    0xb40bbe37 as i64 as ulg,
    0xc30c8ea1 as i64 as ulg,
    0x5a05df1b as i64 as ulg,
    0x2d02ef8d as i64 as ulg,
];
static mut crc: ulg = 0xffffffff as i64 as ulg;
#[no_mangle]
pub unsafe extern "C" fn copy(mut in_0: i32, mut out: i32) -> i32 {
    let mut got: i32 = 0;
    *__errno_location() = 0 as i32;
    while insize > inptr {
        write_buf(
            out,
            (inbuf.as_mut_ptr() as *mut i8).offset(inptr as isize) as voidp,
            insize.wrapping_sub(inptr),
        );
        got = read_buffer(
            in_0,
            inbuf.as_mut_ptr() as *mut i8 as voidp,
            0x40000 as i32 as u32,
        );
        if got == -(1 as i32) {
            read_error();
        }
        bytes_in += got as i64;
        insize = got as u32;
        inptr = 0 as i32 as u32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn updcrc(mut s: *const uch, mut n: u32) -> ulg {
    let mut c: ulg = 0;
    if s.is_null() {
        c = 0xffffffff as i64 as ulg;
    } else {
        c = crc;
        if n != 0 {
            loop {
                let fresh0 = s;
                s = s.offset(1);
                c = crc_32_tab[((c as i32 ^ *fresh0 as i32) & 0xff as i32) as usize]
                    ^ c >> 8 as i32;
                n = n.wrapping_sub(1);
                if !(n != 0) {
                    break;
                }
            }
        }
    }
    crc = c;
    return c ^ 0xffffffff as i64 as u64;
}
#[no_mangle]
pub unsafe extern "C" fn getcrc() -> ulg {
    return crc ^ 0xffffffff as i64 as u64;
}
#[no_mangle]
pub unsafe extern "C" fn clear_bufs() {
    outcnt = 0 as i32 as u32;
    inptr = 0 as i32 as u32;
    insize = inptr;
    bytes_out = 0 as i64;
    bytes_in = bytes_out;
}
#[no_mangle]
pub unsafe extern "C" fn fill_inbuf(mut eof_ok: i32) -> i32 {
    let mut len: i32 = 0;
    insize = 0 as i32 as u32;
    loop {
        len = read_buffer(
            ifd,
            (inbuf.as_mut_ptr() as *mut i8).offset(insize as isize) as voidp,
            (0x40000 as i32 as u32).wrapping_sub(insize),
        );
        if len == 0 as i32 {
            break;
        }
        if len == -(1 as i32) {
            read_error();
            break;
        } else {
            insize = insize.wrapping_add(len as u32);
            if !(insize < 0x40000 as i32 as u32) {
                break;
            }
        }
    }
    if insize == 0 as i32 as u32 {
        if eof_ok != 0 {
            return -(1 as i32);
        }
        flush_window();
        *__errno_location() = 0 as i32;
        read_error();
    }
    bytes_in += insize as off_t;
    inptr = 1 as i32 as u32;
    return *inbuf.as_mut_ptr().offset(0 as i32 as isize) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn read_buffer(mut fd: i32, mut buf: voidp, mut cnt: u32) -> i32 {
    let mut len: i32 = 0;
    if (2147483647 as i32 as u32) < cnt {
        cnt = 2147483647 as i32 as u32;
    }
    len = read(fd, buf, cnt as size_t) as i32;
    if len < 0 as i32 && *__errno_location() == 11 as i32 {
        let mut flags: i32 = rpl_fcntl(fd, 3 as i32);
        if 0 as i32 <= flags {
            if flags & 0o4000 as i32 == 0 {
                *__errno_location() = 11 as i32;
            } else if rpl_fcntl(fd, 4 as i32, flags & !(0o4000 as i32)) != -(1 as i32) {
                len = read(fd, buf, cnt as size_t) as i32;
            }
        }
    }
    return len;
}
unsafe extern "C" fn write_buffer(mut fd: i32, mut buf: voidp, mut cnt: u32) -> i32 {
    if (2147483647 as i32 as u32) < cnt {
        cnt = 2147483647 as i32 as u32;
    }
    return write(fd, buf as *const libc::c_void, cnt as size_t) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn flush_outbuf() {
    if outcnt == 0 as i32 as u32 {
        return;
    }
    write_buf(ofd, outbuf.as_mut_ptr() as voidp, outcnt);
    outcnt = 0 as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn flush_window() {
    if outcnt == 0 as i32 as u32 {
        return;
    }
    updcrc(window.as_mut_ptr(), outcnt);
    write_buf(ofd, window.as_mut_ptr() as voidp, outcnt);
    outcnt = 0 as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn write_buf(mut fd: i32, mut buf: voidp, mut cnt: u32) {
    let mut n: u32 = 0;
    bytes_out += cnt as i64;
    if test != 0 {
        return;
    }
    loop {
        n = write_buffer(fd, buf, cnt) as u32;
        if !(n != cnt) {
            break;
        }
        if n == -(1 as i32) as u32 {
            write_error();
        }
        cnt = cnt.wrapping_sub(n);
        buf = (buf as *mut i8).offset(n as isize) as voidp;
    };
}
#[no_mangle]
pub unsafe extern "C" fn strlwr(mut s: *mut i8) -> *mut i8 {
    let mut t: *mut i8 = 0 as *mut i8;
    t = s;
    while *t != 0 {
        *t = (if *(*__ctype_b_loc()).offset(*t as u8 as i32 as isize) as i32
            & _ISupper as i32 as libc::c_ushort as i32 != 0
        {
            ({
                let mut __res: i32 = 0;
                if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                    if 0 != 0 {
                        let mut __c: i32 = *t as u8 as i32;
                        __res = if __c < -(128 as i32) || __c > 255 as i32 {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = tolower(*t as u8 as i32);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc()).offset(*t as u8 as i32 as isize);
                }
                __res
            })
        } else {
            *t as u8 as i32
        }) as i8;
        t = t.offset(1);
        t;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn gzip_base_name(mut fname: *mut i8) -> *mut i8 {
    fname = last_component(fname);
    if 'A' as i32 == 'a' as i32 {
        strlwr(fname);
    }
    return fname;
}
#[no_mangle]
pub unsafe extern "C" fn xunlink(mut filename: *mut i8) -> i32 {
    let mut r: i32 = unlink(filename);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn add_envopt(
    mut argcp: *mut i32,
    mut argvp: *mut *mut *mut i8,
    mut envvar_name: *const i8,
) -> *mut i8 {
    let mut p: *mut i8 = 0 as *mut i8;
    let mut oargv: *mut *mut i8 = 0 as *mut *mut i8;
    let mut nargv: *mut *mut i8 = 0 as *mut *mut i8;
    let mut nargc: i32 = 0 as i32;
    let mut env_val: *mut i8 = 0 as *mut i8;
    env_val = getenv(envvar_name);
    if env_val.is_null() {
        return 0 as *mut i8;
    }
    env_val = xstrdup(env_val);
    p = env_val;
    while *p != 0 {
        p = p.offset(strspn(p, b" \t\0" as *const u8 as *const i8) as isize);
        if *p as i32 == '\0' as i32 {
            break;
        }
        p = p.offset(strcspn(p, b" \t\0" as *const u8 as *const i8) as isize);
        if *p != 0 {
            let fresh1 = p;
            p = p.offset(1);
            *fresh1 = '\0' as i32 as i8;
        }
        nargc += 1;
        nargc;
    }
    if nargc == 0 as i32 {
        rpl_free(env_val as *mut libc::c_void);
        return 0 as *mut i8;
    }
    *argcp = nargc + 1 as i32;
    nargv = xcalloc(
        (*argcp + 1 as i32) as size_t,
        ::core::mem::size_of::<*mut i8>() as u64,
    ) as *mut *mut i8;
    oargv = *argvp;
    *argvp = nargv;
    let fresh2 = nargv;
    nargv = nargv.offset(1);
    *fresh2 = *oargv;
    p = env_val;
    while nargc > 0 as i32 {
        p = p.offset(strspn(p, b" \t\0" as *const u8 as *const i8) as isize);
        let fresh3 = nargv;
        nargv = nargv.offset(1);
        *fresh3 = p;
        loop {
            let fresh4 = p;
            p = p.offset(1);
            if !(*fresh4 != 0) {
                break;
            }
        }
        nargc -= 1;
        nargc;
    }
    *nargv = 0 as *mut i8;
    return env_val;
}
#[no_mangle]
pub unsafe extern "C" fn gzip_error(mut m: *const i8) {
    rpl_fprintf(
        stderr,
        b"\n%s: %s: %s\n\0" as *const u8 as *const i8,
        program_name,
        ifname.as_mut_ptr(),
        m,
    );
    abort_gzip();
}
#[no_mangle]
pub unsafe extern "C" fn xalloc_die() {
    rpl_fprintf(
        stderr,
        b"\n%s: memory_exhausted\n\0" as *const u8 as *const i8,
        program_name,
    );
    abort_gzip();
}
#[no_mangle]
pub unsafe extern "C" fn warning(mut m: *const i8) {
    if quiet == 0 {
        rpl_fprintf(
            stderr,
            b"%s: %s: warning: %s\n\0" as *const u8 as *const i8,
            program_name,
            ifname.as_mut_ptr(),
            m,
        );
    }
    if exit_code == 0 as i32 {
        exit_code = 2 as i32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn read_error() {
    let mut e: i32 = *__errno_location();
    rpl_fprintf(stderr, b"\n%s: \0" as *const u8 as *const i8, program_name);
    if e != 0 as i32 {
        *__errno_location() = e;
        perror(ifname.as_mut_ptr());
    } else {
        rpl_fprintf(
            stderr,
            b"%s: unexpected end of file\n\0" as *const u8 as *const i8,
            ifname.as_mut_ptr(),
        );
    }
    abort_gzip();
}
#[no_mangle]
pub unsafe extern "C" fn write_error() {
    let mut e: i32 = *__errno_location();
    rpl_fprintf(stderr, b"\n%s: \0" as *const u8 as *const i8, program_name);
    *__errno_location() = e;
    perror(ofname.as_mut_ptr());
    abort_gzip();
}
#[no_mangle]
pub unsafe extern "C" fn display_ratio(
    mut num: off_t,
    mut den: off_t,
    mut file: *mut FILE,
) {
    rpl_fprintf(
        file,
        b"%5.1f%%\0" as *const u8 as *const i8,
        if den == 0 as i32 as i64 {
            0 as i32 as libc::c_double
        } else {
            100.0f64 * num as libc::c_double / den as libc::c_double
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn fprint_off(
    mut file: *mut FILE,
    mut offset: off_t,
    mut width: i32,
) {
    let mut buf: [i8; 64] = [0; 64];
    let mut p: *mut i8 = buf
        .as_mut_ptr()
        .offset(::core::mem::size_of::<[i8; 64]>() as u64 as isize);
    if offset < 0 as i32 as i64 {
        loop {
            p = p.offset(-1);
            *p = ('0' as i32 as i64 - offset % 10 as i32 as i64) as i8;
            offset /= 10 as i32 as i64;
            if !(offset != 0 as i32 as i64) {
                break;
            }
        }
        p = p.offset(-1);
        *p = '-' as i32 as i8;
    } else {
        loop {
            p = p.offset(-1);
            *p = ('0' as i32 as i64 + offset % 10 as i32 as i64) as i8;
            offset /= 10 as i32 as i64;
            if !(offset != 0 as i32 as i64) {
                break;
            }
        }
    }
    width = (width as i64
        - buf
            .as_mut_ptr()
            .offset(::core::mem::size_of::<[i8; 64]>() as u64 as isize)
            .offset_from(p) as i64) as i32;
    loop {
        let fresh5 = width;
        width = width - 1;
        if !((0 as i32) < fresh5) {
            break;
        }
        _IO_putc(' ' as i32, file);
    }
    while p < buf.as_mut_ptr().offset(::core::mem::size_of::<[i8; 64]>() as u64 as isize)
    {
        _IO_putc(*p as i32, file);
        p = p.offset(1);
        p;
    }
}