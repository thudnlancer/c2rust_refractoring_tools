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
    static mut inbuf: [uch; 0];
    static mut outbuf: [uch; 0];
    static mut d_buf: [ush; 0];
    static mut window: [uch; 0];
    static mut prev: [ush; 0];
    static mut insize: u32;
    static mut inptr: u32;
    static mut ifd: i32;
    static mut ofd: i32;
    fn write_buf(fd: i32, buf: voidp, cnt: u32);
    fn gzip_error(m: *const i8);
    fn fill_inbuf(eof_ok: i32) -> i32;
}
pub type voidp = *mut libc::c_void;
pub type uch = u8;
pub type ush = libc::c_ushort;
static mut pt_len: [uch; 32] = [0; 32];
static mut blocksize: u32 = 0;
static mut pt_table: [ush; 256] = [0; 256];
static mut bitbuf: ush = 0;
static mut subbitbuf: u32 = 0;
static mut bitcount: i32 = 0;
unsafe extern "C" fn fillbuf(mut n: i32) {
    bitbuf = ((bitbuf as i32) << n) as ush;
    while n > bitcount {
        n -= bitcount;
        bitbuf = (bitbuf as u32 | subbitbuf << n) as ush;
        subbitbuf = (if inptr < insize {
            let fresh0 = inptr;
            inptr = inptr.wrapping_add(1);
            *inbuf.as_mut_ptr().offset(fresh0 as isize) as i32
        } else {
            fill_inbuf(1 as i32)
        }) as u32;
        if subbitbuf as i32 == -(1 as i32) {
            subbitbuf = 0 as i32 as u32;
        }
        bitcount = 8 as i32;
    }
    bitcount -= n;
    bitbuf = (bitbuf as u32 | subbitbuf >> bitcount) as ush;
}
unsafe extern "C" fn getbits(mut n: i32) -> u32 {
    let mut x: u32 = 0;
    x = (bitbuf as i32
        >> ((8 as i32 * 2 as i32) as u64)
            .wrapping_mul(::core::mem::size_of::<i8>() as u64)
            .wrapping_sub(n as u64)) as u32;
    fillbuf(n);
    return x;
}
unsafe extern "C" fn init_getbits() {
    bitbuf = 0 as i32 as ush;
    subbitbuf = 0 as i32 as u32;
    bitcount = 0 as i32;
    fillbuf(
        ((8 as i32 * 2 as i32) as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64)
            as i32,
    );
}
unsafe extern "C" fn make_table(
    mut nchar: i32,
    mut bitlen: *mut uch,
    mut tablebits: i32,
    mut table: *mut ush,
) {
    let mut count: [ush; 17] = [0; 17];
    let mut weight: [ush; 17] = [0; 17];
    let mut start: [ush; 18] = [0; 18];
    let mut p: *mut ush = 0 as *mut ush;
    let mut i: u32 = 0;
    let mut k: u32 = 0;
    let mut len: u32 = 0;
    let mut ch: u32 = 0;
    let mut jutbits: u32 = 0;
    let mut avail: u32 = 0;
    let mut nextcode: u32 = 0;
    let mut mask: u32 = 0;
    i = 1 as i32 as u32;
    while i <= 16 as i32 as u32 {
        count[i as usize] = 0 as i32 as ush;
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as i32 as u32;
    while i < nchar as u32 {
        count[*bitlen.offset(i as isize) as usize] = (count[*bitlen.offset(i as isize)
            as usize])
            .wrapping_add(1);
        count[*bitlen.offset(i as isize) as usize];
        i = i.wrapping_add(1);
        i;
    }
    start[1 as i32 as usize] = 0 as i32 as ush;
    i = 1 as i32 as u32;
    while i <= 16 as i32 as u32 {
        start[i.wrapping_add(1 as i32 as u32) as usize] = (start[i as usize] as i32
            + ((count[i as usize] as i32) << (16 as i32 as u32).wrapping_sub(i))) as ush;
        i = i.wrapping_add(1);
        i;
    }
    if start[17 as i32 as usize] as i32 & 0xffff as i32 != 0 as i32 {
        gzip_error(b"Bad table\n\0" as *const u8 as *const i8);
    }
    jutbits = (16 as i32 - tablebits) as u32;
    i = 1 as i32 as u32;
    while i <= tablebits as u32 {
        start[i as usize] = (start[i as usize] as i32 >> jutbits) as ush;
        weight[i as usize] = ((1 as i32 as u32) << (tablebits as u32).wrapping_sub(i))
            as ush;
        i = i.wrapping_add(1);
        i;
    }
    while i <= 16 as i32 as u32 {
        weight[i as usize] = ((1 as i32 as u32) << (16 as i32 as u32).wrapping_sub(i))
            as ush;
        i = i.wrapping_add(1);
        i;
    }
    i = (start[(tablebits + 1 as i32) as usize] as i32 >> jutbits) as u32;
    if i != 0 as i32 as u32 {
        k = ((1 as i32) << tablebits) as u32;
        while i != k {
            let fresh1 = i;
            i = i.wrapping_add(1);
            *table.offset(fresh1 as isize) = 0 as i32 as ush;
        }
    }
    avail = nchar as u32;
    mask = (1 as i32 as u32) << 15 as i32 - tablebits;
    ch = 0 as i32 as u32;
    while ch < nchar as u32 {
        len = *bitlen.offset(ch as isize) as u32;
        if !(len == 0 as i32 as u32) {
            nextcode = (start[len as usize] as i32 + weight[len as usize] as i32) as u32;
            if len <= tablebits as u32 {
                if (1 as i32 as u32) << tablebits < nextcode {
                    gzip_error(b"Bad table\n\0" as *const u8 as *const i8);
                }
                i = start[len as usize] as u32;
                while i < nextcode {
                    *table.offset(i as isize) = ch as ush;
                    i = i.wrapping_add(1);
                    i;
                }
            } else {
                k = start[len as usize] as u32;
                p = &mut *table.offset((k >> jutbits) as isize) as *mut ush;
                i = len.wrapping_sub(tablebits as u32);
                while i != 0 as i32 as u32 {
                    if *p as i32 == 0 as i32 {
                        let ref mut fresh2 = *prev.as_mut_ptr().offset(avail as isize);
                        *fresh2 = 0 as i32 as ush;
                        *prev
                            .as_mut_ptr()
                            .offset(0x8000 as i32 as isize)
                            .offset(avail as isize) = *fresh2;
                        let fresh3 = avail;
                        avail = avail.wrapping_add(1);
                        *p = fresh3 as ush;
                    }
                    if k & mask != 0 {
                        p = &mut *prev
                            .as_mut_ptr()
                            .offset(0x8000 as i32 as isize)
                            .offset(*p as isize) as *mut ush;
                    } else {
                        p = &mut *prev.as_mut_ptr().offset(*p as isize) as *mut ush;
                    }
                    k <<= 1 as i32;
                    i = i.wrapping_sub(1);
                    i;
                }
                *p = ch as ush;
            }
            start[len as usize] = nextcode as ush;
        }
        ch = ch.wrapping_add(1);
        ch;
    }
}
unsafe extern "C" fn read_pt_len(mut nn: i32, mut nbit: i32, mut i_special: i32) {
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    let mut n: i32 = 0;
    let mut mask: u32 = 0;
    n = getbits(nbit) as i32;
    if n == 0 as i32 {
        c = getbits(nbit) as i32;
        i = 0 as i32;
        while i < nn {
            pt_len[i as usize] = 0 as i32 as uch;
            i += 1;
            i;
        }
        i = 0 as i32;
        while i < 256 as i32 {
            pt_table[i as usize] = c as ush;
            i += 1;
            i;
        }
    } else {
        i = 0 as i32;
        while i < n {
            c = bitbuf as i32
                >> ((8 as i32 * 2 as i32) as u64)
                    .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                    .wrapping_sub(3 as i32 as u64);
            if c == 7 as i32 {
                mask = (1 as i32 as u32)
                    << ((8 as i32 * 2 as i32) as u64)
                        .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                        .wrapping_sub(1 as i32 as u64)
                        .wrapping_sub(3 as i32 as u64);
                while mask & bitbuf as u32 != 0 {
                    mask >>= 1 as i32;
                    c += 1;
                    c;
                }
                if (16 as i32) < c {
                    gzip_error(b"Bad table\n\0" as *const u8 as *const i8);
                }
            }
            fillbuf(if c < 7 as i32 { 3 as i32 } else { c - 3 as i32 });
            let fresh4 = i;
            i = i + 1;
            pt_len[fresh4 as usize] = c as uch;
            if i == i_special {
                c = getbits(2 as i32) as i32;
                loop {
                    c -= 1;
                    if !(c >= 0 as i32) {
                        break;
                    }
                    let fresh5 = i;
                    i = i + 1;
                    pt_len[fresh5 as usize] = 0 as i32 as uch;
                }
            }
        }
        while i < nn {
            let fresh6 = i;
            i = i + 1;
            pt_len[fresh6 as usize] = 0 as i32 as uch;
        }
        make_table(nn, pt_len.as_mut_ptr(), 8 as i32, pt_table.as_mut_ptr());
    };
}
unsafe extern "C" fn read_c_len() {
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    let mut n: i32 = 0;
    let mut mask: u32 = 0;
    n = getbits(9 as i32) as i32;
    if n == 0 as i32 {
        c = getbits(9 as i32) as i32;
        i = 0 as i32;
        while i < 255 as i32 + 256 as i32 + 2 as i32 - 3 as i32 {
            *outbuf.as_mut_ptr().offset(i as isize) = 0 as i32 as uch;
            i += 1;
            i;
        }
        i = 0 as i32;
        while i < 4096 as i32 {
            *d_buf.as_mut_ptr().offset(i as isize) = c as ush;
            i += 1;
            i;
        }
    } else {
        i = 0 as i32;
        while i < n {
            c = pt_table[(bitbuf as i32
                >> ((8 as i32 * 2 as i32) as u64)
                    .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                    .wrapping_sub(8 as i32 as u64)) as usize] as i32;
            if c >= 16 as i32 + 3 as i32 {
                mask = (1 as i32 as u32)
                    << ((8 as i32 * 2 as i32) as u64)
                        .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                        .wrapping_sub(1 as i32 as u64)
                        .wrapping_sub(8 as i32 as u64);
                loop {
                    if bitbuf as u32 & mask != 0 {
                        c = *prev
                            .as_mut_ptr()
                            .offset(0x8000 as i32 as isize)
                            .offset(c as isize) as i32;
                    } else {
                        c = *prev.as_mut_ptr().offset(c as isize) as i32;
                    }
                    mask >>= 1 as i32;
                    if !(c >= 16 as i32 + 3 as i32) {
                        break;
                    }
                }
            }
            fillbuf(pt_len[c as usize] as i32);
            if c <= 2 as i32 {
                if c == 0 as i32 {
                    c = 1 as i32;
                } else if c == 1 as i32 {
                    c = (getbits(4 as i32)).wrapping_add(3 as i32 as u32) as i32;
                } else {
                    c = (getbits(9 as i32)).wrapping_add(20 as i32 as u32) as i32;
                }
                loop {
                    c -= 1;
                    if !(c >= 0 as i32) {
                        break;
                    }
                    let fresh7 = i;
                    i = i + 1;
                    *outbuf.as_mut_ptr().offset(fresh7 as isize) = 0 as i32 as uch;
                }
            } else {
                let fresh8 = i;
                i = i + 1;
                *outbuf.as_mut_ptr().offset(fresh8 as isize) = (c - 2 as i32) as uch;
            }
        }
        while i < 255 as i32 + 256 as i32 + 2 as i32 - 3 as i32 {
            let fresh9 = i;
            i = i + 1;
            *outbuf.as_mut_ptr().offset(fresh9 as isize) = 0 as i32 as uch;
        }
        make_table(
            255 as i32 + 256 as i32 + 2 as i32 - 3 as i32,
            outbuf.as_mut_ptr(),
            12 as i32,
            d_buf.as_mut_ptr(),
        );
    };
}
unsafe extern "C" fn decode_c() -> u32 {
    let mut j_0: u32 = 0;
    let mut mask: u32 = 0;
    if blocksize == 0 as i32 as u32 {
        blocksize = getbits(16 as i32);
        if blocksize == 0 as i32 as u32 {
            return (255 as i32 + 256 as i32 + 2 as i32 - 3 as i32) as u32;
        }
        read_pt_len(16 as i32 + 3 as i32, 5 as i32, 3 as i32);
        read_c_len();
        read_pt_len(13 as i32 + 1 as i32, 4 as i32, -(1 as i32));
    }
    blocksize = blocksize.wrapping_sub(1);
    blocksize;
    j_0 = *d_buf
        .as_mut_ptr()
        .offset(
            (bitbuf as i32
                >> ((8 as i32 * 2 as i32) as u64)
                    .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                    .wrapping_sub(12 as i32 as u64)) as isize,
        ) as u32;
    if j_0 >= (255 as i32 + 256 as i32 + 2 as i32 - 3 as i32) as u32 {
        mask = (1 as i32 as u32)
            << ((8 as i32 * 2 as i32) as u64)
                .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub(1 as i32 as u64)
                .wrapping_sub(12 as i32 as u64);
        loop {
            if bitbuf as u32 & mask != 0 {
                j_0 = *prev
                    .as_mut_ptr()
                    .offset(0x8000 as i32 as isize)
                    .offset(j_0 as isize) as u32;
            } else {
                j_0 = *prev.as_mut_ptr().offset(j_0 as isize) as u32;
            }
            mask >>= 1 as i32;
            if !(j_0 >= (255 as i32 + 256 as i32 + 2 as i32 - 3 as i32) as u32) {
                break;
            }
        }
    }
    fillbuf(*outbuf.as_mut_ptr().offset(j_0 as isize) as i32);
    return j_0;
}
unsafe extern "C" fn decode_p() -> u32 {
    let mut j_0: u32 = 0;
    let mut mask: u32 = 0;
    j_0 = pt_table[(bitbuf as i32
        >> ((8 as i32 * 2 as i32) as u64)
            .wrapping_mul(::core::mem::size_of::<i8>() as u64)
            .wrapping_sub(8 as i32 as u64)) as usize] as u32;
    if j_0 >= (13 as i32 + 1 as i32) as u32 {
        mask = (1 as i32 as u32)
            << ((8 as i32 * 2 as i32) as u64)
                .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub(1 as i32 as u64)
                .wrapping_sub(8 as i32 as u64);
        loop {
            if bitbuf as u32 & mask != 0 {
                j_0 = *prev
                    .as_mut_ptr()
                    .offset(0x8000 as i32 as isize)
                    .offset(j_0 as isize) as u32;
            } else {
                j_0 = *prev.as_mut_ptr().offset(j_0 as isize) as u32;
            }
            mask >>= 1 as i32;
            if !(j_0 >= (13 as i32 + 1 as i32) as u32) {
                break;
            }
        }
    }
    fillbuf(pt_len[j_0 as usize] as i32);
    if j_0 != 0 as i32 as u32 {
        j_0 = ((1 as i32 as u32) << j_0.wrapping_sub(1 as i32 as u32))
            .wrapping_add(getbits(j_0.wrapping_sub(1 as i32 as u32) as i32));
    }
    return j_0;
}
unsafe extern "C" fn huf_decode_start() {
    init_getbits();
    blocksize = 0 as i32 as u32;
}
static mut j: i32 = 0;
static mut done: i32 = 0;
unsafe extern "C" fn decode_start() {
    huf_decode_start();
    j = 0 as i32;
    done = 0 as i32;
}
unsafe extern "C" fn decode(mut count: u32, mut buffer: *mut uch) -> u32 {
    static mut i: u32 = 0;
    let mut r: u32 = 0;
    let mut c: u32 = 0;
    r = 0 as i32 as u32;
    loop {
        j -= 1;
        if !(j >= 0 as i32) {
            break;
        }
        *buffer.offset(r as isize) = *buffer.offset(i as isize);
        i = i.wrapping_add(1 as i32 as u32)
            & ((1 as i32 as u32) << 13 as i32).wrapping_sub(1 as i32 as u32);
        r = r.wrapping_add(1);
        if r == count {
            return r;
        }
    }
    loop {
        c = decode_c();
        if c == (255 as i32 + 256 as i32 + 2 as i32 - 3 as i32) as u32 {
            done = 1 as i32;
            return r;
        }
        if c <= 255 as i32 as u32 {
            *buffer.offset(r as isize) = c as uch;
            r = r.wrapping_add(1);
            if r == count {
                return r;
            }
        } else {
            j = c.wrapping_sub((255 as i32 + 1 as i32 - 3 as i32) as u32) as i32;
            i = r.wrapping_sub(decode_p()).wrapping_sub(1 as i32 as u32)
                & ((1 as i32 as u32) << 13 as i32).wrapping_sub(1 as i32 as u32);
            loop {
                j -= 1;
                if !(j >= 0 as i32) {
                    break;
                }
                *buffer.offset(r as isize) = *buffer.offset(i as isize);
                i = i.wrapping_add(1 as i32 as u32)
                    & ((1 as i32 as u32) << 13 as i32).wrapping_sub(1 as i32 as u32);
                r = r.wrapping_add(1);
                if r == count {
                    return r;
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn unlzh(mut in_0: i32, mut out: i32) -> i32 {
    let mut n: u32 = 0;
    ifd = in_0;
    ofd = out;
    decode_start();
    while done == 0 {
        n = decode((1 as i32 as u32) << 13 as i32, window.as_mut_ptr());
        if n > 0 as i32 as u32 {
            write_buf(out, window.as_mut_ptr() as voidp, n);
        }
    }
    return 0 as i32;
}