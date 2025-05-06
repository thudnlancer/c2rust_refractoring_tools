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
    fn flush_outbuf();
    fn file_read(buf: *mut i8, size: u32) -> i32;
    static mut outcnt: u32;
    static mut outbuf: [uch; 0];
}
pub type uch = u8;
pub type ush = libc::c_ushort;
pub type file_t = i32;
static mut zfile: file_t = 0;
static mut bi_buf: libc::c_ushort = 0;
static mut bi_valid: i32 = 0;
#[no_mangle]
pub static mut read_buf: Option<unsafe extern "C" fn(*mut i8, u32) -> i32> = None;
#[no_mangle]
pub unsafe extern "C" fn bi_init(mut zipfile: file_t) {
    zfile = zipfile;
    bi_buf = 0 as i32 as libc::c_ushort;
    bi_valid = 0 as i32;
    if zfile != -(1 as i32) {
        read_buf = Some(file_read as unsafe extern "C" fn(*mut i8, u32) -> i32);
    }
}
#[no_mangle]
pub unsafe extern "C" fn send_bits(mut value: i32, mut length: i32) {
    if bi_valid
        > ((8 as i32 * 2 as i32) as u64)
            .wrapping_mul(::core::mem::size_of::<i8>() as u64) as i32 - length
    {
        bi_buf = (bi_buf as i32 | value << bi_valid) as libc::c_ushort;
        if outcnt < (0x40000 as i32 - 2 as i32) as u32 {
            let fresh0 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf.as_mut_ptr().offset(fresh0 as isize) = (bi_buf as i32 & 0xff as i32)
                as uch;
            let fresh1 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf.as_mut_ptr().offset(fresh1 as isize) = (bi_buf as i32 >> 8 as i32)
                as uch;
        } else {
            let fresh2 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf.as_mut_ptr().offset(fresh2 as isize) = (bi_buf as i32 & 0xff as i32)
                as uch;
            if outcnt == 0x40000 as i32 as u32 {
                flush_outbuf();
            }
            let fresh3 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf.as_mut_ptr().offset(fresh3 as isize) = (bi_buf as i32 >> 8 as i32)
                as uch;
            if outcnt == 0x40000 as i32 as u32 {
                flush_outbuf();
            }
        }
        bi_buf = (value as ush as i32
            >> ((8 as i32 * 2 as i32) as u64)
                .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub(bi_valid as u64)) as libc::c_ushort;
        bi_valid = (bi_valid as u64)
            .wrapping_add(
                (length as u64)
                    .wrapping_sub(
                        ((8 as i32 * 2 as i32) as u64)
                            .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                    ),
            ) as i32 as i32;
    } else {
        bi_buf = (bi_buf as i32 | value << bi_valid) as libc::c_ushort;
        bi_valid += length;
    };
}
#[no_mangle]
pub unsafe extern "C" fn bi_reverse(mut code: u32, mut len: i32) -> u32 {
    let mut res: u32 = 0 as i32 as u32;
    loop {
        res |= code & 1 as i32 as u32;
        code >>= 1 as i32;
        res <<= 1 as i32;
        len -= 1;
        if !(len > 0 as i32) {
            break;
        }
    }
    return res >> 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn bi_windup() {
    if bi_valid > 8 as i32 {
        if outcnt < (0x40000 as i32 - 2 as i32) as u32 {
            let fresh4 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf.as_mut_ptr().offset(fresh4 as isize) = (bi_buf as i32 & 0xff as i32)
                as uch;
            let fresh5 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf.as_mut_ptr().offset(fresh5 as isize) = (bi_buf as i32 >> 8 as i32)
                as uch;
        } else {
            let fresh6 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf.as_mut_ptr().offset(fresh6 as isize) = (bi_buf as i32 & 0xff as i32)
                as uch;
            if outcnt == 0x40000 as i32 as u32 {
                flush_outbuf();
            }
            let fresh7 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf.as_mut_ptr().offset(fresh7 as isize) = (bi_buf as i32 >> 8 as i32)
                as uch;
            if outcnt == 0x40000 as i32 as u32 {
                flush_outbuf();
            }
        }
    } else if bi_valid > 0 as i32 {
        let fresh8 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf.as_mut_ptr().offset(fresh8 as isize) = bi_buf as uch;
        if outcnt == 0x40000 as i32 as u32 {
            flush_outbuf();
        }
    }
    bi_buf = 0 as i32 as libc::c_ushort;
    bi_valid = 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn copy_block(mut buf: *mut i8, mut len: u32, mut header: i32) {
    bi_windup();
    if header != 0 {
        if outcnt < (0x40000 as i32 - 2 as i32) as u32 {
            let fresh9 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf.as_mut_ptr().offset(fresh9 as isize) = (len as ush as i32
                & 0xff as i32) as uch;
            let fresh10 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf.as_mut_ptr().offset(fresh10 as isize) = (len as ush as i32
                >> 8 as i32) as uch;
        } else {
            let fresh11 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf.as_mut_ptr().offset(fresh11 as isize) = (len as ush as i32
                & 0xff as i32) as uch;
            if outcnt == 0x40000 as i32 as u32 {
                flush_outbuf();
            }
            let fresh12 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf.as_mut_ptr().offset(fresh12 as isize) = (len as ush as i32
                >> 8 as i32) as uch;
            if outcnt == 0x40000 as i32 as u32 {
                flush_outbuf();
            }
        }
        if outcnt < (0x40000 as i32 - 2 as i32) as u32 {
            let fresh13 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf.as_mut_ptr().offset(fresh13 as isize) = (!len as ush as i32
                & 0xff as i32) as uch;
            let fresh14 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf.as_mut_ptr().offset(fresh14 as isize) = (!len as ush as i32
                >> 8 as i32) as uch;
        } else {
            let fresh15 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf.as_mut_ptr().offset(fresh15 as isize) = (!len as ush as i32
                & 0xff as i32) as uch;
            if outcnt == 0x40000 as i32 as u32 {
                flush_outbuf();
            }
            let fresh16 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf.as_mut_ptr().offset(fresh16 as isize) = (!len as ush as i32
                >> 8 as i32) as uch;
            if outcnt == 0x40000 as i32 as u32 {
                flush_outbuf();
            }
        }
    }
    loop {
        let fresh17 = len;
        len = len.wrapping_sub(1);
        if !(fresh17 != 0) {
            break;
        }
        let fresh18 = buf;
        buf = buf.offset(1);
        let fresh19 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf.as_mut_ptr().offset(fresh19 as isize) = *fresh18 as uch;
        if outcnt == 0x40000 as i32 as u32 {
            flush_outbuf();
        }
    };
}