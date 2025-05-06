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
    fn read_error();
    fn gzip_error(m: *const i8);
    fn read_buffer(fd: i32, buf: voidp, cnt: u32) -> i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    static mut inbuf: [uch; 0];
    static mut outbuf: [uch; 0];
    static mut d_buf: [ush; 0];
    static mut window: [uch; 0];
    static mut prev: [ush; 0];
    static mut insize: u32;
    static mut inptr: u32;
    static mut bytes_in: off_t;
    static mut ifname: [i8; 0];
    static mut program_name: *mut i8;
    static mut exit_code: i32;
    static mut quiet: i32;
    static mut to_stdout: i32;
    fn fill_inbuf(eof_ok: i32) -> i32;
    fn write_buf(fd: i32, buf: voidp, cnt: u32);
    static mut maxbits: i32;
}
pub type __off_t = i64;
pub type __off64_t = i64;
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
pub type ush = libc::c_ushort;
pub type code_int = i64;
pub type char_type = u8;
pub type cmp_code_int = u64;
static mut block_mode: i32 = 0x80 as i32;
#[no_mangle]
pub unsafe extern "C" fn unlzw(mut in_0: i32, mut out: i32) -> i32 {
    let mut stackp: *mut char_type = 0 as *mut char_type;
    let mut code: code_int = 0;
    let mut finchar: i32 = 0;
    let mut oldcode: code_int = 0;
    let mut incode: code_int = 0;
    let mut inbits: i64 = 0;
    let mut posbits: i64 = 0;
    let mut outpos: i32 = 0;
    let mut bitmask: u32 = 0;
    let mut free_ent: code_int = 0;
    let mut maxcode: code_int = 0;
    let mut maxmaxcode: code_int = 0;
    let mut n_bits: i32 = 0;
    let mut rsize: i32 = 0;
    maxbits = if inptr < insize {
        let fresh0 = inptr;
        inptr = inptr.wrapping_add(1);
        *inbuf.as_mut_ptr().offset(fresh0 as isize) as i32
    } else {
        fill_inbuf(0 as i32)
    };
    block_mode = maxbits & 0x80 as i32;
    if maxbits & 0x60 as i32 != 0 as i32 {
        if quiet == 0 {
            rpl_fprintf(
                stderr,
                b"\n%s: %s: warning, unknown flags 0x%x\n\0" as *const u8 as *const i8,
                program_name,
                ifname.as_mut_ptr(),
                maxbits as u32 & 0x60 as i32 as u32,
            );
        }
        if exit_code == 0 as i32 {
            exit_code = 2 as i32;
        }
    }
    maxbits &= 0x1f as i32;
    maxmaxcode = (1 as i64) << maxbits;
    if maxbits > 16 as i32 {
        rpl_fprintf(
            stderr,
            b"\n%s: %s: compressed with %d bits, can only handle %d bits\n\0"
                as *const u8 as *const i8,
            program_name,
            ifname.as_mut_ptr(),
            maxbits,
            16 as i32,
        );
        exit_code = 1 as i32;
        return 1 as i32;
    }
    rsize = insize as i32;
    n_bits = 9 as i32;
    maxcode = ((1 as i64) << n_bits) - 1 as i32 as i64;
    bitmask = (((1 as i32) << n_bits) - 1 as i32) as u32;
    oldcode = -(1 as i32) as code_int;
    finchar = 0 as i32;
    outpos = 0 as i32;
    posbits = (inptr << 3 as i32) as i64;
    free_ent = (if block_mode != 0 { 256 as i32 + 1 as i32 } else { 256 as i32 })
        as code_int;
    memset(prev.as_mut_ptr() as voidp, 0 as i32, 256 as i32 as u64);
    code = 255 as i32 as code_int;
    while code >= 0 as i32 as i64 {
        *window.as_mut_ptr().offset(code as isize) = code as char_type;
        code -= 1;
        code;
    }
    loop {
        let mut i: i32 = 0;
        let mut e: i32 = 0;
        let mut o: i32 = 0;
        '_resetbuf: loop {
            o = (posbits >> 3 as i32) as i32;
            e = (if o as u32 <= insize {
                insize.wrapping_sub(o as u32)
            } else {
                0 as i32 as u32
            }) as i32;
            i = 0 as i32;
            while i < e {
                *inbuf.as_mut_ptr().offset(i as isize) = *inbuf
                    .as_mut_ptr()
                    .offset((i + o) as isize);
                i += 1;
                i;
            }
            insize = e as u32;
            posbits = 0 as i32 as i64;
            if insize < 64 as i32 as u32 {
                rsize = read_buffer(
                    in_0,
                    (inbuf.as_mut_ptr() as *mut i8).offset(insize as isize) as voidp,
                    0x40000 as i32 as u32,
                );
                if rsize == -(1 as i32) {
                    read_error();
                }
                insize = insize.wrapping_add(rsize as u32);
                bytes_in += rsize as off_t;
            }
            inbits = if rsize != 0 as i32 {
                (insize as i64 - insize.wrapping_rem(n_bits as u32) as i64) << 3 as i32
            } else {
                ((insize as i64) << 3 as i32) - (n_bits - 1 as i32) as i64
            };
            loop {
                if !(inbits > posbits) {
                    break '_resetbuf;
                }
                if free_ent > maxcode {
                    posbits = posbits - 1 as i32 as i64
                        + ((n_bits << 3 as i32) as i64
                            - (posbits - 1 as i32 as i64 + (n_bits << 3 as i32) as i64)
                                % (n_bits << 3 as i32) as i64);
                    n_bits += 1;
                    n_bits;
                    if n_bits == maxbits {
                        maxcode = maxmaxcode;
                    } else {
                        maxcode = ((1 as i64) << n_bits) - 1 as i32 as i64;
                    }
                    bitmask = (((1 as i32) << n_bits) - 1 as i32) as u32;
                    break;
                } else {
                    let mut p: *mut char_type = &mut *inbuf
                        .as_mut_ptr()
                        .offset((posbits >> 3 as i32) as isize) as *mut uch;
                    code = (*p.offset(0 as i32 as isize) as i64
                        | (*p.offset(1 as i32 as isize) as i64) << 8 as i32
                        | (*p.offset(2 as i32 as isize) as i64) << 16 as i32)
                        >> (posbits & 0x7 as i32 as i64) & bitmask as i64;
                    posbits += n_bits as i64;
                    if oldcode == -(1 as i32) as i64 {
                        if 256 as i32 as i64 <= code {
                            gzip_error(b"corrupt input.\0" as *const u8 as *const i8);
                        }
                        oldcode = code;
                        finchar = oldcode as i32;
                        let fresh1 = outpos;
                        outpos = outpos + 1;
                        *outbuf.as_mut_ptr().offset(fresh1 as isize) = finchar
                            as char_type;
                    } else if code == 256 as i32 as i64 && block_mode != 0 {
                        memset(prev.as_mut_ptr() as voidp, 0 as i32, 256 as i32 as u64);
                        free_ent = (256 as i32 + 1 as i32 - 1 as i32) as code_int;
                        posbits = posbits - 1 as i32 as i64
                            + ((n_bits << 3 as i32) as i64
                                - (posbits - 1 as i32 as i64 + (n_bits << 3 as i32) as i64)
                                    % (n_bits << 3 as i32) as i64);
                        n_bits = 9 as i32;
                        maxcode = ((1 as i64) << n_bits) - 1 as i32 as i64;
                        bitmask = (((1 as i32) << n_bits) - 1 as i32) as u32;
                        break;
                    } else {
                        incode = code;
                        stackp = &mut *d_buf
                            .as_mut_ptr()
                            .offset((0x8000 as i32 - 1 as i32) as isize) as *mut ush
                            as *mut char_type;
                        if code >= free_ent {
                            if code > free_ent {
                                if outpos > 0 as i32 {
                                    write_buf(out, outbuf.as_mut_ptr() as voidp, outpos as u32);
                                }
                                gzip_error(
                                    if to_stdout != 0 {
                                        b"corrupt input.\0" as *const u8 as *const i8
                                    } else {
                                        b"corrupt input. Use zcat to recover some data.\0"
                                            as *const u8 as *const i8
                                    },
                                );
                            }
                            stackp = stackp.offset(-1);
                            *stackp = finchar as char_type;
                            code = oldcode;
                        }
                        while code as cmp_code_int >= 256 as i32 as cmp_code_int {
                            stackp = stackp.offset(-1);
                            *stackp = *window.as_mut_ptr().offset(code as isize);
                            code = *prev.as_mut_ptr().offset(code as isize) as code_int;
                        }
                        finchar = *window.as_mut_ptr().offset(code as isize) as i32;
                        stackp = stackp.offset(-1);
                        *stackp = finchar as char_type;
                        let mut i_0: i32 = 0;
                        i_0 = (&mut *d_buf
                            .as_mut_ptr()
                            .offset((0x8000 as i32 - 1 as i32) as isize) as *mut ush
                            as *mut char_type)
                            .offset_from(stackp) as i64 as i32;
                        if outpos + i_0 >= 0x40000 as i32 {
                            loop {
                                if i_0 > 0x40000 as i32 - outpos {
                                    i_0 = 0x40000 as i32 - outpos;
                                }
                                if i_0 > 0 as i32 {
                                    memcpy(
                                        outbuf.as_mut_ptr().offset(outpos as isize)
                                            as *mut libc::c_void,
                                        stackp as *const libc::c_void,
                                        i_0 as u64,
                                    );
                                    outpos += i_0;
                                }
                                if outpos >= 0x40000 as i32 {
                                    write_buf(out, outbuf.as_mut_ptr() as voidp, outpos as u32);
                                    outpos = 0 as i32;
                                }
                                stackp = stackp.offset(i_0 as isize);
                                i_0 = (&mut *d_buf
                                    .as_mut_ptr()
                                    .offset((0x8000 as i32 - 1 as i32) as isize) as *mut ush
                                    as *mut char_type)
                                    .offset_from(stackp) as i64 as i32;
                                if !(i_0 > 0 as i32) {
                                    break;
                                }
                            }
                        } else {
                            memcpy(
                                outbuf.as_mut_ptr().offset(outpos as isize)
                                    as *mut libc::c_void,
                                stackp as *const libc::c_void,
                                i_0 as u64,
                            );
                            outpos += i_0;
                        }
                        code = free_ent;
                        if code < maxmaxcode {
                            *prev.as_mut_ptr().offset(code as isize) = oldcode
                                as libc::c_ushort;
                            *window.as_mut_ptr().offset(code as isize) = finchar
                                as char_type;
                            free_ent = code + 1 as i32 as i64;
                        }
                        oldcode = incode;
                    }
                }
            }
        }
        if !(rsize != 0 as i32) {
            break;
        }
    }
    if outpos > 0 as i32 {
        write_buf(out, outbuf.as_mut_ptr() as voidp, outpos as u32);
    }
    return 0 as i32;
}