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
    fn __errno_location() -> *mut i32;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fputc(__c: i32, __stream: *mut FILE) -> i32;
    fn ferror(__stream: *mut FILE) -> i32;
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_printf(fmt: *const i8, _: ...);
    fn _glp_xstrerr(_: i32) -> *mut i8;
}
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
pub type glp_errfunc = Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
unsafe extern "C" fn put_byte(mut fp: *mut FILE, mut c: i32) {
    fputc(c, fp);
}
unsafe extern "C" fn put_word(mut fp: *mut FILE, mut w: i32) {
    put_byte(fp, w);
    put_byte(fp, w >> 8 as i32);
}
unsafe extern "C" fn put_dword(mut fp: *mut FILE, mut d: i32) {
    put_word(fp, d);
    put_word(fp, d >> 16 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_rgr_write_bmp16(
    mut fname: *const i8,
    mut m: i32,
    mut n: i32,
    mut map: *const i8,
) -> i32 {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut offset: i32 = 0;
    let mut bmsize: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut b: i32 = 0;
    let mut ret: i32 = 0 as i32;
    if !(1 as i32 <= m && m <= 32767 as i32) {
        (glp_error_(b"misc/rgr.c\0" as *const u8 as *const i8, 92 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"rgr_write_bmp16: m = %d; invalid height\n\0" as *const u8 as *const i8,
            m,
        );
    }
    if !(1 as i32 <= n && n <= 32767 as i32) {
        (glp_error_(b"misc/rgr.c\0" as *const u8 as *const i8, 94 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"rgr_write_bmp16: n = %d; invalid width\n\0" as *const u8 as *const i8,
            n,
        );
    }
    fp = fopen(fname, b"wb\0" as *const u8 as *const i8);
    if fp.is_null() {
        glp_printf(
            b"rgr_write_bmp16: unable to create '%s' - %s\n\0" as *const u8 as *const i8,
            fname,
            _glp_xstrerr(*__errno_location()),
        );
        ret = 1 as i32;
    } else {
        offset = 14 as i32 + 40 as i32 + 16 as i32 * 4 as i32;
        bmsize = (4 as i32 * n + 31 as i32) / 32 as i32;
        put_byte(fp, 'B' as i32);
        put_byte(fp, 'M' as i32);
        put_dword(fp, offset + bmsize * 4 as i32);
        put_word(fp, 0 as i32);
        put_word(fp, 0 as i32);
        put_dword(fp, offset);
        put_dword(fp, 40 as i32);
        put_dword(fp, n);
        put_dword(fp, m);
        put_word(fp, 1 as i32);
        put_word(fp, 4 as i32);
        put_dword(fp, 0 as i32);
        put_dword(fp, 0 as i32);
        put_dword(fp, 2953 as i32);
        put_dword(fp, 2953 as i32);
        put_dword(fp, 0 as i32);
        put_dword(fp, 0 as i32);
        put_dword(fp, 0 as i32);
        put_dword(fp, 0x80 as i32);
        put_dword(fp, 0x8000 as i32);
        put_dword(fp, 0x8080 as i32);
        put_dword(fp, 0x800000 as i32);
        put_dword(fp, 0x800080 as i32);
        put_dword(fp, 0x808000 as i32);
        put_dword(fp, 0xc0c0c0 as i32);
        put_dword(fp, 0x808080 as i32);
        put_dword(fp, 0xff as i32);
        put_dword(fp, 0xff00 as i32);
        put_dword(fp, 0xffff as i32);
        put_dword(fp, 0xff0000 as i32);
        put_dword(fp, 0xff00ff as i32);
        put_dword(fp, 0xffff00 as i32);
        put_dword(fp, 0xffffff as i32);
        b = 0 as i32;
        i = m - 1 as i32;
        while i >= 0 as i32 {
            j = 0 as i32;
            while j < (n + 7 as i32) / 8 as i32 * 8 as i32 {
                b <<= 4 as i32;
                b
                    |= if j < n {
                        *map.offset((i * n + j) as isize) as i32 & 15 as i32
                    } else {
                        0 as i32
                    };
                if j & 1 as i32 != 0 {
                    put_byte(fp, b);
                }
                j += 1;
                j;
            }
            i -= 1;
            i;
        }
        fflush(fp);
        if ferror(fp) != 0 {
            glp_printf(
                b"rgr_write_bmp16: write error on '%s' - %s\n\0" as *const u8
                    as *const i8,
                fname,
                _glp_xstrerr(*__errno_location()),
            );
            ret = 1 as i32;
        }
    }
    if !fp.is_null() {
        fclose(fp);
    }
    return ret;
}