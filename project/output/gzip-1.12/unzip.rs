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
    static mut stderr: *mut _IO_FILE;
    fn rpl_fprintf(fp: *mut FILE, format: *const i8, _: ...) -> i32;
    static mut method: i32;
    static mut inbuf: [uch; 0];
    static mut outbuf: [uch; 0];
    static mut window: [uch; 0];
    static mut insize: u32;
    static mut inptr: u32;
    static mut outcnt: u32;
    static mut bytes_out: off_t;
    static mut ifd: i32;
    static mut ofd: i32;
    static mut ifname: [i8; 0];
    static mut program_name: *mut i8;
    static mut exit_code: i32;
    static mut quiet: i32;
    static mut test: i32;
    static mut to_stdout: i32;
    fn abort_gzip();
    fn updcrc(s: *const uch, n: u32) -> ulg;
    fn fill_inbuf(eof_ok: i32) -> i32;
    fn gzip_error(m: *const i8);
    fn flush_window();
    fn inflate() -> i32;
    fn xalloc_die();
}
pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
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
pub type off_t = __off_t;
pub type uch = u8;
pub type ush = libc::c_ushort;
pub type ulg = u64;
#[no_mangle]
pub static mut unzip_crc: ulg = 0;
static mut decrypt: i32 = 0;
static mut pkzip: i32 = 0 as i32;
static mut ext_header: i32 = 0 as i32;
#[no_mangle]
pub unsafe extern "C" fn check_zipfile(mut in_0: i32) -> i32 {
    let mut h: *mut uch = inbuf.as_mut_ptr().offset(inptr as isize);
    ifd = in_0;
    inptr = inptr
        .wrapping_add(
            (30 as i32
                + (*h.offset(26 as i32 as isize).offset(0 as i32 as isize) as ush as i32
                    | (*h.offset(26 as i32 as isize).offset(1 as i32 as isize) as ush
                        as i32) << 8 as i32)
                + (*h.offset(28 as i32 as isize).offset(0 as i32 as isize) as ush as i32
                    | (*h.offset(28 as i32 as isize).offset(1 as i32 as isize) as ush
                        as i32) << 8 as i32)) as u32,
        );
    if inptr > insize
        || (*h.offset(0 as i32 as isize) as ush as i32
            | (*h.offset(1 as i32 as isize) as ush as i32) << 8 as i32) as ulg
            | ((*h.offset(2 as i32 as isize).offset(0 as i32 as isize) as ush as i32
                | (*h.offset(2 as i32 as isize).offset(1 as i32 as isize) as ush as i32)
                    << 8 as i32) as ulg) << 16 as i32 != 0x4034b50 as i64 as u64
    {
        rpl_fprintf(
            stderr,
            b"\n%s: %s: not a valid zip file\n\0" as *const u8 as *const i8,
            program_name,
            ifname.as_mut_ptr(),
        );
        exit_code = 1 as i32;
        return 1 as i32;
    }
    method = *h.offset(8 as i32 as isize) as i32;
    if method != 0 as i32 && method != 8 as i32 {
        rpl_fprintf(
            stderr,
            b"\n%s: %s: first entry not deflated or stored -- use unzip\n\0" as *const u8
                as *const i8,
            program_name,
            ifname.as_mut_ptr(),
        );
        exit_code = 1 as i32;
        return 1 as i32;
    }
    decrypt = *h.offset(6 as i32 as isize) as i32 & 1 as i32;
    if decrypt != 0 as i32 {
        rpl_fprintf(
            stderr,
            b"\n%s: %s: encrypted file -- use unzip\n\0" as *const u8 as *const i8,
            program_name,
            ifname.as_mut_ptr(),
        );
        exit_code = 1 as i32;
        return 1 as i32;
    }
    ext_header = (*h.offset(6 as i32 as isize) as i32 & 8 as i32 != 0 as i32) as i32;
    pkzip = 1 as i32;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn unzip(mut in_0: i32, mut out: i32) -> i32 {
    let mut orig_crc: ulg = 0 as i32 as ulg;
    let mut orig_len: ulg = 0 as i32 as ulg;
    let mut n: i32 = 0;
    let mut buf: [uch; 16] = [0; 16];
    let mut err: i32 = 0 as i32;
    ifd = in_0;
    ofd = out;
    updcrc(0 as *const uch, 0 as i32 as u32);
    if pkzip != 0 && ext_header == 0 {
        orig_crc = (*inbuf
            .as_mut_ptr()
            .offset(14 as i32 as isize)
            .offset(0 as i32 as isize) as ush as i32
            | (*inbuf.as_mut_ptr().offset(14 as i32 as isize).offset(1 as i32 as isize)
                as ush as i32) << 8 as i32) as ulg
            | ((*inbuf
                .as_mut_ptr()
                .offset(14 as i32 as isize)
                .offset(2 as i32 as isize)
                .offset(0 as i32 as isize) as ush as i32
                | (*inbuf
                    .as_mut_ptr()
                    .offset(14 as i32 as isize)
                    .offset(2 as i32 as isize)
                    .offset(1 as i32 as isize) as ush as i32) << 8 as i32) as ulg)
                << 16 as i32;
        orig_len = (*inbuf
            .as_mut_ptr()
            .offset(22 as i32 as isize)
            .offset(0 as i32 as isize) as ush as i32
            | (*inbuf.as_mut_ptr().offset(22 as i32 as isize).offset(1 as i32 as isize)
                as ush as i32) << 8 as i32) as ulg
            | ((*inbuf
                .as_mut_ptr()
                .offset(22 as i32 as isize)
                .offset(2 as i32 as isize)
                .offset(0 as i32 as isize) as ush as i32
                | (*inbuf
                    .as_mut_ptr()
                    .offset(22 as i32 as isize)
                    .offset(2 as i32 as isize)
                    .offset(1 as i32 as isize) as ush as i32) << 8 as i32) as ulg)
                << 16 as i32;
    }
    if method == 8 as i32 {
        let mut res: i32 = inflate();
        if res == 3 as i32 {
            xalloc_die();
        } else if res != 0 as i32 {
            gzip_error(
                b"invalid compressed data--format violated\0" as *const u8 as *const i8,
            );
        }
    } else if pkzip != 0 && method == 0 as i32 {
        let mut n_0: ulg = (*inbuf
            .as_mut_ptr()
            .offset(22 as i32 as isize)
            .offset(0 as i32 as isize) as ush as i32
            | (*inbuf.as_mut_ptr().offset(22 as i32 as isize).offset(1 as i32 as isize)
                as ush as i32) << 8 as i32) as ulg
            | ((*inbuf
                .as_mut_ptr()
                .offset(22 as i32 as isize)
                .offset(2 as i32 as isize)
                .offset(0 as i32 as isize) as ush as i32
                | (*inbuf
                    .as_mut_ptr()
                    .offset(22 as i32 as isize)
                    .offset(2 as i32 as isize)
                    .offset(1 as i32 as isize) as ush as i32) << 8 as i32) as ulg)
                << 16 as i32;
        if n_0
            != ((*inbuf.as_mut_ptr().offset(18 as i32 as isize).offset(0 as i32 as isize)
                as ush as i32
                | (*inbuf
                    .as_mut_ptr()
                    .offset(18 as i32 as isize)
                    .offset(1 as i32 as isize) as ush as i32) << 8 as i32) as ulg
                | ((*inbuf
                    .as_mut_ptr()
                    .offset(18 as i32 as isize)
                    .offset(2 as i32 as isize)
                    .offset(0 as i32 as isize) as ush as i32
                    | (*inbuf
                        .as_mut_ptr()
                        .offset(18 as i32 as isize)
                        .offset(2 as i32 as isize)
                        .offset(1 as i32 as isize) as ush as i32) << 8 as i32) as ulg)
                    << 16 as i32)
                .wrapping_sub((if decrypt != 0 { 12 as i32 } else { 0 as i32 }) as u64)
        {
            rpl_fprintf(
                stderr,
                b"len %lu, siz %lu\n\0" as *const u8 as *const i8,
                n_0,
                (*inbuf.as_mut_ptr().offset(18 as i32 as isize).offset(0 as i32 as isize)
                    as ush as i32
                    | (*inbuf
                        .as_mut_ptr()
                        .offset(18 as i32 as isize)
                        .offset(1 as i32 as isize) as ush as i32) << 8 as i32) as ulg
                    | ((*inbuf
                        .as_mut_ptr()
                        .offset(18 as i32 as isize)
                        .offset(2 as i32 as isize)
                        .offset(0 as i32 as isize) as ush as i32
                        | (*inbuf
                            .as_mut_ptr()
                            .offset(18 as i32 as isize)
                            .offset(2 as i32 as isize)
                            .offset(1 as i32 as isize) as ush as i32) << 8 as i32)
                        as ulg) << 16 as i32,
            );
            gzip_error(
                b"invalid compressed data--length mismatch\0" as *const u8 as *const i8,
            );
        }
        loop {
            let fresh0 = n_0;
            n_0 = n_0.wrapping_sub(1);
            if !(fresh0 != 0) {
                break;
            }
            let mut c: uch = (if inptr < insize {
                let fresh1 = inptr;
                inptr = inptr.wrapping_add(1);
                *inbuf.as_mut_ptr().offset(fresh1 as isize) as i32
            } else {
                fill_inbuf(0 as i32)
            }) as uch;
            let fresh2 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *window.as_mut_ptr().offset(fresh2 as isize) = c;
            if outcnt == 0x8000 as i32 as u32 {
                flush_window();
            }
        }
        flush_window();
    } else {
        gzip_error(b"internal error, invalid method\0" as *const u8 as *const i8);
    }
    if pkzip == 0 {
        n = 0 as i32;
        while n < 8 as i32 {
            buf[n as usize] = (if inptr < insize {
                let fresh3 = inptr;
                inptr = inptr.wrapping_add(1);
                *inbuf.as_mut_ptr().offset(fresh3 as isize) as i32
            } else {
                fill_inbuf(0 as i32)
            }) as uch;
            n += 1;
            n;
        }
        orig_crc = (buf[0 as i32 as usize] as ush as i32
            | (buf[1 as i32 as usize] as ush as i32) << 8 as i32) as ulg
            | ((*buf.as_mut_ptr().offset(2 as i32 as isize).offset(0 as i32 as isize)
                as ush as i32
                | (*buf.as_mut_ptr().offset(2 as i32 as isize).offset(1 as i32 as isize)
                    as ush as i32) << 8 as i32) as ulg) << 16 as i32;
        orig_len = (*buf.as_mut_ptr().offset(4 as i32 as isize).offset(0 as i32 as isize)
            as ush as i32
            | (*buf.as_mut_ptr().offset(4 as i32 as isize).offset(1 as i32 as isize)
                as ush as i32) << 8 as i32) as ulg
            | ((*buf
                .as_mut_ptr()
                .offset(4 as i32 as isize)
                .offset(2 as i32 as isize)
                .offset(0 as i32 as isize) as ush as i32
                | (*buf
                    .as_mut_ptr()
                    .offset(4 as i32 as isize)
                    .offset(2 as i32 as isize)
                    .offset(1 as i32 as isize) as ush as i32) << 8 as i32) as ulg)
                << 16 as i32;
    } else if ext_header != 0 {
        n = 0 as i32;
        while n < 16 as i32 {
            buf[n as usize] = (if inptr < insize {
                let fresh4 = inptr;
                inptr = inptr.wrapping_add(1);
                *inbuf.as_mut_ptr().offset(fresh4 as isize) as i32
            } else {
                fill_inbuf(0 as i32)
            }) as uch;
            n += 1;
            n;
        }
        orig_crc = (*buf.as_mut_ptr().offset(4 as i32 as isize).offset(0 as i32 as isize)
            as ush as i32
            | (*buf.as_mut_ptr().offset(4 as i32 as isize).offset(1 as i32 as isize)
                as ush as i32) << 8 as i32) as ulg
            | ((*buf
                .as_mut_ptr()
                .offset(4 as i32 as isize)
                .offset(2 as i32 as isize)
                .offset(0 as i32 as isize) as ush as i32
                | (*buf
                    .as_mut_ptr()
                    .offset(4 as i32 as isize)
                    .offset(2 as i32 as isize)
                    .offset(1 as i32 as isize) as ush as i32) << 8 as i32) as ulg)
                << 16 as i32;
        orig_len = (*buf
            .as_mut_ptr()
            .offset(12 as i32 as isize)
            .offset(0 as i32 as isize) as ush as i32
            | (*buf.as_mut_ptr().offset(12 as i32 as isize).offset(1 as i32 as isize)
                as ush as i32) << 8 as i32) as ulg
            | ((*buf
                .as_mut_ptr()
                .offset(12 as i32 as isize)
                .offset(2 as i32 as isize)
                .offset(0 as i32 as isize) as ush as i32
                | (*buf
                    .as_mut_ptr()
                    .offset(12 as i32 as isize)
                    .offset(2 as i32 as isize)
                    .offset(1 as i32 as isize) as ush as i32) << 8 as i32) as ulg)
                << 16 as i32;
    }
    if orig_crc != updcrc(outbuf.as_mut_ptr(), 0 as i32 as u32) {
        rpl_fprintf(
            stderr,
            b"\n%s: %s: invalid compressed data--crc error\n\0" as *const u8
                as *const i8,
            program_name,
            ifname.as_mut_ptr(),
        );
        err = 1 as i32;
    }
    if orig_len != (bytes_out & 0xffffffff as u32 as i64) as ulg {
        rpl_fprintf(
            stderr,
            b"\n%s: %s: invalid compressed data--length error\n\0" as *const u8
                as *const i8,
            program_name,
            ifname.as_mut_ptr(),
        );
        err = 1 as i32;
    }
    if pkzip != 0 && inptr.wrapping_add(4 as i32 as u32) < insize
        && (*inbuf.as_mut_ptr().offset(inptr as isize).offset(0 as i32 as isize) as ush
            as i32
            | (*inbuf.as_mut_ptr().offset(inptr as isize).offset(1 as i32 as isize)
                as ush as i32) << 8 as i32) as ulg
            | ((*inbuf
                .as_mut_ptr()
                .offset(inptr as isize)
                .offset(2 as i32 as isize)
                .offset(0 as i32 as isize) as ush as i32
                | (*inbuf
                    .as_mut_ptr()
                    .offset(inptr as isize)
                    .offset(2 as i32 as isize)
                    .offset(1 as i32 as isize) as ush as i32) << 8 as i32) as ulg)
                << 16 as i32 == 0x4034b50 as i64 as u64
    {
        if to_stdout != 0 {
            if quiet == 0 {
                rpl_fprintf(
                    stderr,
                    b"%s: %s has more than one entry--rest ignored\n\0" as *const u8
                        as *const i8,
                    program_name,
                    ifname.as_mut_ptr(),
                );
            }
            if exit_code == 0 as i32 {
                exit_code = 2 as i32;
            }
        } else {
            rpl_fprintf(
                stderr,
                b"%s: %s has more than one entry -- unchanged\n\0" as *const u8
                    as *const i8,
                program_name,
                ifname.as_mut_ptr(),
            );
            err = 1 as i32;
        }
    }
    pkzip = 0 as i32;
    ext_header = pkzip;
    unzip_crc = orig_crc;
    if err == 0 as i32 {
        return 0 as i32;
    }
    exit_code = 1 as i32;
    if test == 0 {
        abort_gzip();
    }
    return err;
}