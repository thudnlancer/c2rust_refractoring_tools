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
    fn rpl_free(ptr: *mut libc::c_void);
    fn malloc(_: u64) -> *mut libc::c_void;
    fn flush_window();
    fn fill_inbuf(eof_ok: i32) -> i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    static mut inbuf: [uch; 0];
    static mut window: [uch; 0];
    static mut insize: u32;
    static mut inptr: u32;
    static mut outcnt: u32;
}
pub type voidp = *mut libc::c_void;
pub type uch = u8;
pub type ush = libc::c_ushort;
pub type ulg = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct huft {
    pub e: uch,
    pub b: uch,
    pub v: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub n: ush,
    pub t: *mut huft,
}
static mut border: [u32; 19] = [
    16 as i32 as u32,
    17 as i32 as u32,
    18 as i32 as u32,
    0 as i32 as u32,
    8 as i32 as u32,
    7 as i32 as u32,
    9 as i32 as u32,
    6 as i32 as u32,
    10 as i32 as u32,
    5 as i32 as u32,
    11 as i32 as u32,
    4 as i32 as u32,
    12 as i32 as u32,
    3 as i32 as u32,
    13 as i32 as u32,
    2 as i32 as u32,
    14 as i32 as u32,
    1 as i32 as u32,
    15 as i32 as u32,
];
static mut cplens: [ush; 31] = [
    3 as i32 as ush,
    4 as i32 as ush,
    5 as i32 as ush,
    6 as i32 as ush,
    7 as i32 as ush,
    8 as i32 as ush,
    9 as i32 as ush,
    10 as i32 as ush,
    11 as i32 as ush,
    13 as i32 as ush,
    15 as i32 as ush,
    17 as i32 as ush,
    19 as i32 as ush,
    23 as i32 as ush,
    27 as i32 as ush,
    31 as i32 as ush,
    35 as i32 as ush,
    43 as i32 as ush,
    51 as i32 as ush,
    59 as i32 as ush,
    67 as i32 as ush,
    83 as i32 as ush,
    99 as i32 as ush,
    115 as i32 as ush,
    131 as i32 as ush,
    163 as i32 as ush,
    195 as i32 as ush,
    227 as i32 as ush,
    258 as i32 as ush,
    0 as i32 as ush,
    0 as i32 as ush,
];
static mut cplext: [ush; 31] = [
    0 as i32 as ush,
    0 as i32 as ush,
    0 as i32 as ush,
    0 as i32 as ush,
    0 as i32 as ush,
    0 as i32 as ush,
    0 as i32 as ush,
    0 as i32 as ush,
    1 as i32 as ush,
    1 as i32 as ush,
    1 as i32 as ush,
    1 as i32 as ush,
    2 as i32 as ush,
    2 as i32 as ush,
    2 as i32 as ush,
    2 as i32 as ush,
    3 as i32 as ush,
    3 as i32 as ush,
    3 as i32 as ush,
    3 as i32 as ush,
    4 as i32 as ush,
    4 as i32 as ush,
    4 as i32 as ush,
    4 as i32 as ush,
    5 as i32 as ush,
    5 as i32 as ush,
    5 as i32 as ush,
    5 as i32 as ush,
    0 as i32 as ush,
    99 as i32 as ush,
    99 as i32 as ush,
];
static mut cpdist: [ush; 30] = [
    1 as i32 as ush,
    2 as i32 as ush,
    3 as i32 as ush,
    4 as i32 as ush,
    5 as i32 as ush,
    7 as i32 as ush,
    9 as i32 as ush,
    13 as i32 as ush,
    17 as i32 as ush,
    25 as i32 as ush,
    33 as i32 as ush,
    49 as i32 as ush,
    65 as i32 as ush,
    97 as i32 as ush,
    129 as i32 as ush,
    193 as i32 as ush,
    257 as i32 as ush,
    385 as i32 as ush,
    513 as i32 as ush,
    769 as i32 as ush,
    1025 as i32 as ush,
    1537 as i32 as ush,
    2049 as i32 as ush,
    3073 as i32 as ush,
    4097 as i32 as ush,
    6145 as i32 as ush,
    8193 as i32 as ush,
    12289 as i32 as ush,
    16385 as i32 as ush,
    24577 as i32 as ush,
];
static mut cpdext: [ush; 30] = [
    0 as i32 as ush,
    0 as i32 as ush,
    0 as i32 as ush,
    0 as i32 as ush,
    1 as i32 as ush,
    1 as i32 as ush,
    2 as i32 as ush,
    2 as i32 as ush,
    3 as i32 as ush,
    3 as i32 as ush,
    4 as i32 as ush,
    4 as i32 as ush,
    5 as i32 as ush,
    5 as i32 as ush,
    6 as i32 as ush,
    6 as i32 as ush,
    7 as i32 as ush,
    7 as i32 as ush,
    8 as i32 as ush,
    8 as i32 as ush,
    9 as i32 as ush,
    9 as i32 as ush,
    10 as i32 as ush,
    10 as i32 as ush,
    11 as i32 as ush,
    11 as i32 as ush,
    12 as i32 as ush,
    12 as i32 as ush,
    13 as i32 as ush,
    13 as i32 as ush,
];
static mut bb: ulg = 0;
static mut bk: u32 = 0;
static mut mask_bits: [ush; 17] = [
    0 as i32 as ush,
    0x1 as i32 as ush,
    0x3 as i32 as ush,
    0x7 as i32 as ush,
    0xf as i32 as ush,
    0x1f as i32 as ush,
    0x3f as i32 as ush,
    0x7f as i32 as ush,
    0xff as i32 as ush,
    0x1ff as i32 as ush,
    0x3ff as i32 as ush,
    0x7ff as i32 as ush,
    0xfff as i32 as ush,
    0x1fff as i32 as ush,
    0x3fff as i32 as ush,
    0x7fff as i32 as ush,
    0xffff as i32 as ush,
];
static mut lbits: i32 = 9 as i32;
static mut dbits: i32 = 6 as i32;
static mut hufts: u32 = 0;
unsafe extern "C" fn huft_build(
    mut b: *mut u32,
    mut n: u32,
    mut s: u32,
    mut d: *mut ush,
    mut e: *mut ush,
    mut t: *mut *mut huft,
    mut m: *mut i32,
) -> i32 {
    let mut a: u32 = 0;
    let mut c: [u32; 17] = [0; 17];
    let mut f: u32 = 0;
    let mut g: i32 = 0;
    let mut h: i32 = 0;
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut p: *mut u32 = 0 as *mut u32;
    let mut q: *mut huft = 0 as *mut huft;
    let mut r: huft = huft {
        e: 0,
        b: 0,
        v: C2RustUnnamed { n: 0 },
    };
    let mut u: [*mut huft; 16] = [0 as *mut huft; 16];
    let mut v: [u32; 288] = [0; 288];
    let mut w: i32 = 0;
    let mut x: [u32; 17] = [0; 17];
    let mut xp: *mut u32 = 0 as *mut u32;
    let mut y: i32 = 0;
    let mut z: u32 = 0;
    memset(
        c.as_mut_ptr() as voidp,
        0 as i32,
        ::core::mem::size_of::<[u32; 17]>() as u64,
    );
    p = b;
    i = n;
    loop {
        c[*p as usize] = (c[*p as usize]).wrapping_add(1);
        c[*p as usize];
        p = p.offset(1);
        p;
        i = i.wrapping_sub(1);
        if !(i != 0) {
            break;
        }
    }
    if c[0 as i32 as usize] == n {
        q = malloc((3 as i32 as u64).wrapping_mul(::core::mem::size_of::<huft>() as u64))
            as *mut huft;
        if q.is_null() {
            return 3 as i32;
        }
        hufts = hufts.wrapping_add(3 as i32 as u32);
        let ref mut fresh0 = (*q.offset(0 as i32 as isize)).v.t;
        *fresh0 = 0 as *mut libc::c_void as *mut huft;
        (*q.offset(1 as i32 as isize)).e = 99 as i32 as uch;
        (*q.offset(1 as i32 as isize)).b = 1 as i32 as uch;
        (*q.offset(2 as i32 as isize)).e = 99 as i32 as uch;
        (*q.offset(2 as i32 as isize)).b = 1 as i32 as uch;
        *t = q.offset(1 as i32 as isize);
        *m = 1 as i32;
        return 0 as i32;
    }
    l = *m;
    j = 1 as i32 as u32;
    while j <= 16 as i32 as u32 {
        if c[j as usize] != 0 {
            break;
        }
        j = j.wrapping_add(1);
        j;
    }
    k = j as i32;
    if (l as u32) < j {
        l = j as i32;
    }
    i = 16 as i32 as u32;
    while i != 0 {
        if c[i as usize] != 0 {
            break;
        }
        i = i.wrapping_sub(1);
        i;
    }
    g = i as i32;
    if l as u32 > i {
        l = i as i32;
    }
    *m = l;
    y = (1 as i32) << j;
    while j < i {
        y = (y as u32).wrapping_sub(c[j as usize]) as i32 as i32;
        if y < 0 as i32 {
            return 2 as i32;
        }
        j = j.wrapping_add(1);
        j;
        y <<= 1 as i32;
    }
    y = (y as u32).wrapping_sub(c[i as usize]) as i32 as i32;
    if y < 0 as i32 {
        return 2 as i32;
    }
    c[i as usize] = (c[i as usize]).wrapping_add(y as u32);
    j = 0 as i32 as u32;
    x[1 as i32 as usize] = j;
    p = c.as_mut_ptr().offset(1 as i32 as isize);
    xp = x.as_mut_ptr().offset(2 as i32 as isize);
    loop {
        i = i.wrapping_sub(1);
        if !(i != 0) {
            break;
        }
        let fresh1 = p;
        p = p.offset(1);
        j = j.wrapping_add(*fresh1);
        let fresh2 = xp;
        xp = xp.offset(1);
        *fresh2 = j;
    }
    p = b;
    i = 0 as i32 as u32;
    loop {
        let fresh3 = p;
        p = p.offset(1);
        j = *fresh3;
        if j != 0 as i32 as u32 {
            let fresh4 = x[j as usize];
            x[j as usize] = (x[j as usize]).wrapping_add(1);
            v[fresh4 as usize] = i;
        }
        i = i.wrapping_add(1);
        if !(i < n) {
            break;
        }
    }
    n = x[g as usize];
    i = 0 as i32 as u32;
    x[0 as i32 as usize] = i;
    p = v.as_mut_ptr();
    h = -(1 as i32);
    w = -l;
    u[0 as i32 as usize] = 0 as *mut libc::c_void as *mut huft;
    q = 0 as *mut libc::c_void as *mut huft;
    z = 0 as i32 as u32;
    while k <= g {
        a = c[k as usize];
        loop {
            let fresh5 = a;
            a = a.wrapping_sub(1);
            if !(fresh5 != 0) {
                break;
            }
            while k > w + l {
                h += 1;
                h;
                w += l;
                z = (g - w) as u32;
                z = if z > l as u32 { l as u32 } else { z };
                j = (k - w) as u32;
                f = ((1 as i32) << j) as u32;
                if f > a.wrapping_add(1 as i32 as u32) {
                    f = f.wrapping_sub(a.wrapping_add(1 as i32 as u32));
                    xp = c.as_mut_ptr().offset(k as isize);
                    if j < z {
                        loop {
                            j = j.wrapping_add(1);
                            if !(j < z) {
                                break;
                            }
                            f <<= 1 as i32;
                            xp = xp.offset(1);
                            if f <= *xp {
                                break;
                            }
                            f = f.wrapping_sub(*xp);
                        }
                    }
                }
                z = ((1 as i32) << j) as u32;
                q = malloc(
                    (z.wrapping_add(1 as i32 as u32) as u64)
                        .wrapping_mul(::core::mem::size_of::<huft>() as u64),
                ) as *mut huft;
                if q.is_null() {
                    if h != 0 {
                        huft_free(u[0 as i32 as usize]);
                    }
                    return 3 as i32;
                }
                hufts = hufts.wrapping_add(z.wrapping_add(1 as i32 as u32));
                *t = q.offset(1 as i32 as isize);
                t = &mut (*q).v.t;
                *t = 0 as *mut libc::c_void as *mut huft;
                q = q.offset(1);
                u[h as usize] = q;
                if h != 0 {
                    x[h as usize] = i;
                    r.b = l as uch;
                    r.e = (16 as i32 as u32).wrapping_add(j) as uch;
                    r.v.t = q;
                    j = i >> w - l;
                    *(u[(h - 1 as i32) as usize]).offset(j as isize) = r;
                }
            }
            r.b = (k - w) as uch;
            if p >= v.as_mut_ptr().offset(n as isize) {
                r.e = 99 as i32 as uch;
            } else if *p < s {
                r.e = (if *p < 256 as i32 as u32 { 16 as i32 } else { 15 as i32 })
                    as uch;
                r.v.n = *p as ush;
                p = p.offset(1);
                p;
            } else {
                r.e = *e.offset((*p).wrapping_sub(s) as isize) as uch;
                let fresh6 = p;
                p = p.offset(1);
                r.v.n = *d.offset((*fresh6).wrapping_sub(s) as isize);
            }
            f = ((1 as i32) << k - w) as u32;
            j = i >> w;
            while j < z {
                *q.offset(j as isize) = r;
                j = j.wrapping_add(f);
            }
            j = ((1 as i32) << k - 1 as i32) as u32;
            while i & j != 0 {
                i ^= j;
                j >>= 1 as i32;
            }
            i ^= j;
            while i & (((1 as i32) << w) - 1 as i32) as u32 != x[h as usize] {
                h -= 1;
                h;
                w -= l;
            }
        }
        k += 1;
        k;
    }
    return (y != 0 as i32 && g != 1 as i32) as i32;
}
unsafe extern "C" fn huft_free(mut t: *mut huft) -> i32 {
    let mut p: *mut huft = 0 as *mut huft;
    let mut q: *mut huft = 0 as *mut huft;
    p = t;
    while !p.is_null() {
        p = p.offset(-1);
        q = (*p).v.t;
        rpl_free(p as *mut libc::c_void);
        p = q;
    }
    return 0 as i32;
}
unsafe extern "C" fn inflate_codes(
    mut tl: *mut huft,
    mut td: *mut huft,
    mut bl: i32,
    mut bd: i32,
) -> i32 {
    let mut e: u32 = 0;
    let mut n: u32 = 0;
    let mut d: u32 = 0;
    let mut w: u32 = 0;
    let mut t: *mut huft = 0 as *mut huft;
    let mut ml: u32 = 0;
    let mut md: u32 = 0;
    let mut b: ulg = 0;
    let mut k: u32 = 0;
    b = bb;
    k = bk;
    w = outcnt;
    ml = mask_bits[bl as usize] as u32;
    md = mask_bits[bd as usize] as u32;
    loop {
        while k < bl as u32 {
            b
                |= ((if inptr < insize {
                    let fresh7 = inptr;
                    inptr = inptr.wrapping_add(1);
                    *inbuf.as_mut_ptr().offset(fresh7 as isize) as i32
                } else {
                    outcnt = w;
                    fill_inbuf(0 as i32)
                }) as uch as ulg) << k;
            k = k.wrapping_add(8 as i32 as u32);
        }
        t = tl.offset((b as u32 & ml) as isize);
        e = (*t).e as u32;
        if e > 16 as i32 as u32 {
            loop {
                if e == 99 as i32 as u32 {
                    return 1 as i32;
                }
                b >>= (*t).b as i32;
                k = k.wrapping_sub((*t).b as u32);
                e = e.wrapping_sub(16 as i32 as u32);
                while k < e {
                    b
                        |= ((if inptr < insize {
                            let fresh8 = inptr;
                            inptr = inptr.wrapping_add(1);
                            *inbuf.as_mut_ptr().offset(fresh8 as isize) as i32
                        } else {
                            outcnt = w;
                            fill_inbuf(0 as i32)
                        }) as uch as ulg) << k;
                    k = k.wrapping_add(8 as i32 as u32);
                }
                t = ((*t).v.t)
                    .offset((b as u32 & mask_bits[e as usize] as u32) as isize);
                e = (*t).e as u32;
                if !(e > 16 as i32 as u32) {
                    break;
                }
            }
        }
        b >>= (*t).b as i32;
        k = k.wrapping_sub((*t).b as u32);
        if e == 16 as i32 as u32 {
            let fresh9 = w;
            w = w.wrapping_add(1);
            *window.as_mut_ptr().offset(fresh9 as isize) = (*t).v.n as uch;
            if w == 0x8000 as i32 as u32 {
                outcnt = w;
                flush_window();
                w = 0 as i32 as u32;
            }
        } else {
            if e == 15 as i32 as u32 {
                break;
            }
            while k < e {
                b
                    |= ((if inptr < insize {
                        let fresh10 = inptr;
                        inptr = inptr.wrapping_add(1);
                        *inbuf.as_mut_ptr().offset(fresh10 as isize) as i32
                    } else {
                        outcnt = w;
                        fill_inbuf(0 as i32)
                    }) as uch as ulg) << k;
                k = k.wrapping_add(8 as i32 as u32);
            }
            n = ((*t).v.n as u32).wrapping_add(b as u32 & mask_bits[e as usize] as u32);
            b >>= e;
            k = k.wrapping_sub(e);
            while k < bd as u32 {
                b
                    |= ((if inptr < insize {
                        let fresh11 = inptr;
                        inptr = inptr.wrapping_add(1);
                        *inbuf.as_mut_ptr().offset(fresh11 as isize) as i32
                    } else {
                        outcnt = w;
                        fill_inbuf(0 as i32)
                    }) as uch as ulg) << k;
                k = k.wrapping_add(8 as i32 as u32);
            }
            t = td.offset((b as u32 & md) as isize);
            e = (*t).e as u32;
            if e > 16 as i32 as u32 {
                loop {
                    if e == 99 as i32 as u32 {
                        return 1 as i32;
                    }
                    b >>= (*t).b as i32;
                    k = k.wrapping_sub((*t).b as u32);
                    e = e.wrapping_sub(16 as i32 as u32);
                    while k < e {
                        b
                            |= ((if inptr < insize {
                                let fresh12 = inptr;
                                inptr = inptr.wrapping_add(1);
                                *inbuf.as_mut_ptr().offset(fresh12 as isize) as i32
                            } else {
                                outcnt = w;
                                fill_inbuf(0 as i32)
                            }) as uch as ulg) << k;
                        k = k.wrapping_add(8 as i32 as u32);
                    }
                    t = ((*t).v.t)
                        .offset((b as u32 & mask_bits[e as usize] as u32) as isize);
                    e = (*t).e as u32;
                    if !(e > 16 as i32 as u32) {
                        break;
                    }
                }
            }
            b >>= (*t).b as i32;
            k = k.wrapping_sub((*t).b as u32);
            while k < e {
                b
                    |= ((if inptr < insize {
                        let fresh13 = inptr;
                        inptr = inptr.wrapping_add(1);
                        *inbuf.as_mut_ptr().offset(fresh13 as isize) as i32
                    } else {
                        outcnt = w;
                        fill_inbuf(0 as i32)
                    }) as uch as ulg) << k;
                k = k.wrapping_add(8 as i32 as u32);
            }
            d = w
                .wrapping_sub((*t).v.n as u32)
                .wrapping_sub(b as u32 & mask_bits[e as usize] as u32);
            b >>= e;
            k = k.wrapping_sub(e);
            loop {
                d &= (0x8000 as i32 - 1 as i32) as u32;
                e = (0x8000 as i32 as u32).wrapping_sub((if d > w { d } else { w }));
                e = if e > n { n } else { e };
                n = n.wrapping_sub(e);
                if e <= (if d < w { w.wrapping_sub(d) } else { d.wrapping_sub(w) }) {
                    memcpy(
                        window.as_mut_ptr().offset(w as isize) as *mut libc::c_void,
                        window.as_mut_ptr().offset(d as isize) as *const libc::c_void,
                        e as u64,
                    );
                    w = w.wrapping_add(e);
                    d = d.wrapping_add(e);
                } else {
                    loop {
                        let fresh14 = d;
                        d = d.wrapping_add(1);
                        let fresh15 = w;
                        w = w.wrapping_add(1);
                        *window.as_mut_ptr().offset(fresh15 as isize) = *window
                            .as_mut_ptr()
                            .offset(fresh14 as isize);
                        e = e.wrapping_sub(1);
                        if !(e != 0) {
                            break;
                        }
                    }
                }
                if w == 0x8000 as i32 as u32 {
                    outcnt = w;
                    flush_window();
                    w = 0 as i32 as u32;
                }
                if !(n != 0) {
                    break;
                }
            }
        }
    }
    outcnt = w;
    bb = b;
    bk = k;
    return 0 as i32;
}
unsafe extern "C" fn inflate_stored() -> i32 {
    let mut n: u32 = 0;
    let mut w: u32 = 0;
    let mut b: ulg = 0;
    let mut k: u32 = 0;
    b = bb;
    k = bk;
    w = outcnt;
    n = k & 7 as i32 as u32;
    b >>= n;
    k = k.wrapping_sub(n);
    while k < 16 as i32 as u32 {
        b
            |= ((if inptr < insize {
                let fresh16 = inptr;
                inptr = inptr.wrapping_add(1);
                *inbuf.as_mut_ptr().offset(fresh16 as isize) as i32
            } else {
                outcnt = w;
                fill_inbuf(0 as i32)
            }) as uch as ulg) << k;
        k = k.wrapping_add(8 as i32 as u32);
    }
    n = b as u32 & 0xffff as i32 as u32;
    b >>= 16 as i32;
    k = k.wrapping_sub(16 as i32 as u32);
    while k < 16 as i32 as u32 {
        b
            |= ((if inptr < insize {
                let fresh17 = inptr;
                inptr = inptr.wrapping_add(1);
                *inbuf.as_mut_ptr().offset(fresh17 as isize) as i32
            } else {
                outcnt = w;
                fill_inbuf(0 as i32)
            }) as uch as ulg) << k;
        k = k.wrapping_add(8 as i32 as u32);
    }
    if n != (!b & 0xffff as i32 as u64) as u32 {
        return 1 as i32;
    }
    b >>= 16 as i32;
    k = k.wrapping_sub(16 as i32 as u32);
    loop {
        let fresh18 = n;
        n = n.wrapping_sub(1);
        if !(fresh18 != 0) {
            break;
        }
        while k < 8 as i32 as u32 {
            b
                |= ((if inptr < insize {
                    let fresh19 = inptr;
                    inptr = inptr.wrapping_add(1);
                    *inbuf.as_mut_ptr().offset(fresh19 as isize) as i32
                } else {
                    outcnt = w;
                    fill_inbuf(0 as i32)
                }) as uch as ulg) << k;
            k = k.wrapping_add(8 as i32 as u32);
        }
        let fresh20 = w;
        w = w.wrapping_add(1);
        *window.as_mut_ptr().offset(fresh20 as isize) = b as uch;
        if w == 0x8000 as i32 as u32 {
            outcnt = w;
            flush_window();
            w = 0 as i32 as u32;
        }
        b >>= 8 as i32;
        k = k.wrapping_sub(8 as i32 as u32);
    }
    outcnt = w;
    bb = b;
    bk = k;
    return 0 as i32;
}
unsafe extern "C" fn inflate_fixed() -> i32 {
    let mut i: i32 = 0;
    let mut tl: *mut huft = 0 as *mut huft;
    let mut td: *mut huft = 0 as *mut huft;
    let mut bl: i32 = 0;
    let mut bd: i32 = 0;
    let mut l: [u32; 288] = [0; 288];
    i = 0 as i32;
    while i < 144 as i32 {
        l[i as usize] = 8 as i32 as u32;
        i += 1;
        i;
    }
    while i < 256 as i32 {
        l[i as usize] = 9 as i32 as u32;
        i += 1;
        i;
    }
    while i < 280 as i32 {
        l[i as usize] = 7 as i32 as u32;
        i += 1;
        i;
    }
    while i < 288 as i32 {
        l[i as usize] = 8 as i32 as u32;
        i += 1;
        i;
    }
    bl = 7 as i32;
    i = huft_build(
        l.as_mut_ptr(),
        288 as i32 as u32,
        257 as i32 as u32,
        cplens.as_mut_ptr(),
        cplext.as_mut_ptr(),
        &mut tl,
        &mut bl,
    );
    if i != 0 as i32 {
        return i;
    }
    i = 0 as i32;
    while i < 30 as i32 {
        l[i as usize] = 5 as i32 as u32;
        i += 1;
        i;
    }
    bd = 5 as i32;
    i = huft_build(
        l.as_mut_ptr(),
        30 as i32 as u32,
        0 as i32 as u32,
        cpdist.as_mut_ptr(),
        cpdext.as_mut_ptr(),
        &mut td,
        &mut bd,
    );
    if i > 1 as i32 {
        huft_free(tl);
        return i;
    }
    if inflate_codes(tl, td, bl, bd) != 0 {
        return 1 as i32;
    }
    huft_free(tl);
    huft_free(td);
    return 0 as i32;
}
unsafe extern "C" fn inflate_dynamic() -> i32 {
    let mut i: i32 = 0;
    let mut j: u32 = 0;
    let mut l: u32 = 0;
    let mut m: u32 = 0;
    let mut n: u32 = 0;
    let mut w: u32 = 0;
    let mut tl: *mut huft = 0 as *mut huft;
    let mut td: *mut huft = 0 as *mut huft;
    let mut bl: i32 = 0;
    let mut bd: i32 = 0;
    let mut nb: u32 = 0;
    let mut nl: u32 = 0;
    let mut nd: u32 = 0;
    let mut ll: [u32; 316] = [0; 316];
    let mut b: ulg = 0;
    let mut k: u32 = 0;
    b = bb;
    k = bk;
    w = outcnt;
    while k < 5 as i32 as u32 {
        b
            |= ((if inptr < insize {
                let fresh21 = inptr;
                inptr = inptr.wrapping_add(1);
                *inbuf.as_mut_ptr().offset(fresh21 as isize) as i32
            } else {
                outcnt = w;
                fill_inbuf(0 as i32)
            }) as uch as ulg) << k;
        k = k.wrapping_add(8 as i32 as u32);
    }
    nl = (257 as i32 as u32).wrapping_add(b as u32 & 0x1f as i32 as u32);
    b >>= 5 as i32;
    k = k.wrapping_sub(5 as i32 as u32);
    while k < 5 as i32 as u32 {
        b
            |= ((if inptr < insize {
                let fresh22 = inptr;
                inptr = inptr.wrapping_add(1);
                *inbuf.as_mut_ptr().offset(fresh22 as isize) as i32
            } else {
                outcnt = w;
                fill_inbuf(0 as i32)
            }) as uch as ulg) << k;
        k = k.wrapping_add(8 as i32 as u32);
    }
    nd = (1 as i32 as u32).wrapping_add(b as u32 & 0x1f as i32 as u32);
    b >>= 5 as i32;
    k = k.wrapping_sub(5 as i32 as u32);
    while k < 4 as i32 as u32 {
        b
            |= ((if inptr < insize {
                let fresh23 = inptr;
                inptr = inptr.wrapping_add(1);
                *inbuf.as_mut_ptr().offset(fresh23 as isize) as i32
            } else {
                outcnt = w;
                fill_inbuf(0 as i32)
            }) as uch as ulg) << k;
        k = k.wrapping_add(8 as i32 as u32);
    }
    nb = (4 as i32 as u32).wrapping_add(b as u32 & 0xf as i32 as u32);
    b >>= 4 as i32;
    k = k.wrapping_sub(4 as i32 as u32);
    if nl > 286 as i32 as u32 || nd > 30 as i32 as u32 {
        return 1 as i32;
    }
    j = 0 as i32 as u32;
    while j < nb {
        while k < 3 as i32 as u32 {
            b
                |= ((if inptr < insize {
                    let fresh24 = inptr;
                    inptr = inptr.wrapping_add(1);
                    *inbuf.as_mut_ptr().offset(fresh24 as isize) as i32
                } else {
                    outcnt = w;
                    fill_inbuf(0 as i32)
                }) as uch as ulg) << k;
            k = k.wrapping_add(8 as i32 as u32);
        }
        ll[border[j as usize] as usize] = b as u32 & 7 as i32 as u32;
        b >>= 3 as i32;
        k = k.wrapping_sub(3 as i32 as u32);
        j = j.wrapping_add(1);
        j;
    }
    while j < 19 as i32 as u32 {
        ll[border[j as usize] as usize] = 0 as i32 as u32;
        j = j.wrapping_add(1);
        j;
    }
    bl = 7 as i32;
    i = huft_build(
        ll.as_mut_ptr(),
        19 as i32 as u32,
        19 as i32 as u32,
        0 as *mut ush,
        0 as *mut ush,
        &mut tl,
        &mut bl,
    );
    if i != 0 as i32 {
        if i == 1 as i32 {
            huft_free(tl);
        }
        return i;
    }
    if tl.is_null() {
        return 2 as i32;
    }
    n = nl.wrapping_add(nd);
    m = mask_bits[bl as usize] as u32;
    l = 0 as i32 as u32;
    i = l as i32;
    while (i as u32) < n {
        while k < bl as u32 {
            b
                |= ((if inptr < insize {
                    let fresh25 = inptr;
                    inptr = inptr.wrapping_add(1);
                    *inbuf.as_mut_ptr().offset(fresh25 as isize) as i32
                } else {
                    outcnt = w;
                    fill_inbuf(0 as i32)
                }) as uch as ulg) << k;
            k = k.wrapping_add(8 as i32 as u32);
        }
        td = tl.offset((b as u32 & m) as isize);
        j = (*td).b as u32;
        b >>= j;
        k = k.wrapping_sub(j);
        if (*td).e as i32 == 99 as i32 {
            huft_free(tl);
            return 2 as i32;
        }
        j = (*td).v.n as u32;
        if j < 16 as i32 as u32 {
            l = j;
            let fresh26 = i;
            i = i + 1;
            ll[fresh26 as usize] = l;
        } else if j == 16 as i32 as u32 {
            while k < 2 as i32 as u32 {
                b
                    |= ((if inptr < insize {
                        let fresh27 = inptr;
                        inptr = inptr.wrapping_add(1);
                        *inbuf.as_mut_ptr().offset(fresh27 as isize) as i32
                    } else {
                        outcnt = w;
                        fill_inbuf(0 as i32)
                    }) as uch as ulg) << k;
                k = k.wrapping_add(8 as i32 as u32);
            }
            j = (3 as i32 as u32).wrapping_add(b as u32 & 3 as i32 as u32);
            b >>= 2 as i32;
            k = k.wrapping_sub(2 as i32 as u32);
            if (i as u32).wrapping_add(j) > n {
                return 1 as i32;
            }
            loop {
                let fresh28 = j;
                j = j.wrapping_sub(1);
                if !(fresh28 != 0) {
                    break;
                }
                let fresh29 = i;
                i = i + 1;
                ll[fresh29 as usize] = l;
            }
        } else if j == 17 as i32 as u32 {
            while k < 3 as i32 as u32 {
                b
                    |= ((if inptr < insize {
                        let fresh30 = inptr;
                        inptr = inptr.wrapping_add(1);
                        *inbuf.as_mut_ptr().offset(fresh30 as isize) as i32
                    } else {
                        outcnt = w;
                        fill_inbuf(0 as i32)
                    }) as uch as ulg) << k;
                k = k.wrapping_add(8 as i32 as u32);
            }
            j = (3 as i32 as u32).wrapping_add(b as u32 & 7 as i32 as u32);
            b >>= 3 as i32;
            k = k.wrapping_sub(3 as i32 as u32);
            if (i as u32).wrapping_add(j) > n {
                return 1 as i32;
            }
            loop {
                let fresh31 = j;
                j = j.wrapping_sub(1);
                if !(fresh31 != 0) {
                    break;
                }
                let fresh32 = i;
                i = i + 1;
                ll[fresh32 as usize] = 0 as i32 as u32;
            }
            l = 0 as i32 as u32;
        } else {
            while k < 7 as i32 as u32 {
                b
                    |= ((if inptr < insize {
                        let fresh33 = inptr;
                        inptr = inptr.wrapping_add(1);
                        *inbuf.as_mut_ptr().offset(fresh33 as isize) as i32
                    } else {
                        outcnt = w;
                        fill_inbuf(0 as i32)
                    }) as uch as ulg) << k;
                k = k.wrapping_add(8 as i32 as u32);
            }
            j = (11 as i32 as u32).wrapping_add(b as u32 & 0x7f as i32 as u32);
            b >>= 7 as i32;
            k = k.wrapping_sub(7 as i32 as u32);
            if (i as u32).wrapping_add(j) > n {
                return 1 as i32;
            }
            loop {
                let fresh34 = j;
                j = j.wrapping_sub(1);
                if !(fresh34 != 0) {
                    break;
                }
                let fresh35 = i;
                i = i + 1;
                ll[fresh35 as usize] = 0 as i32 as u32;
            }
            l = 0 as i32 as u32;
        }
    }
    huft_free(tl);
    bb = b;
    bk = k;
    bl = lbits;
    i = huft_build(
        ll.as_mut_ptr(),
        nl,
        257 as i32 as u32,
        cplens.as_mut_ptr(),
        cplext.as_mut_ptr(),
        &mut tl,
        &mut bl,
    );
    if i != 0 as i32 {
        if i == 1 as i32 {
            huft_free(tl);
        }
        return i;
    }
    bd = dbits;
    i = huft_build(
        ll.as_mut_ptr().offset(nl as isize),
        nd,
        0 as i32 as u32,
        cpdist.as_mut_ptr(),
        cpdext.as_mut_ptr(),
        &mut td,
        &mut bd,
    );
    if i != 0 as i32 {
        if i == 1 as i32 {
            huft_free(td);
        }
        huft_free(tl);
        return i;
    }
    let mut err: i32 = if inflate_codes(tl, td, bl, bd) != 0 {
        1 as i32
    } else {
        0 as i32
    };
    huft_free(tl);
    huft_free(td);
    return err;
}
unsafe extern "C" fn inflate_block(mut e: *mut i32) -> i32 {
    let mut t: u32 = 0;
    let mut w: u32 = 0;
    let mut b: ulg = 0;
    let mut k: u32 = 0;
    b = bb;
    k = bk;
    w = outcnt;
    while k < 1 as i32 as u32 {
        b
            |= ((if inptr < insize {
                let fresh36 = inptr;
                inptr = inptr.wrapping_add(1);
                *inbuf.as_mut_ptr().offset(fresh36 as isize) as i32
            } else {
                outcnt = w;
                fill_inbuf(0 as i32)
            }) as uch as ulg) << k;
        k = k.wrapping_add(8 as i32 as u32);
    }
    *e = b as i32 & 1 as i32;
    b >>= 1 as i32;
    k = k.wrapping_sub(1 as i32 as u32);
    while k < 2 as i32 as u32 {
        b
            |= ((if inptr < insize {
                let fresh37 = inptr;
                inptr = inptr.wrapping_add(1);
                *inbuf.as_mut_ptr().offset(fresh37 as isize) as i32
            } else {
                outcnt = w;
                fill_inbuf(0 as i32)
            }) as uch as ulg) << k;
        k = k.wrapping_add(8 as i32 as u32);
    }
    t = b as u32 & 3 as i32 as u32;
    b >>= 2 as i32;
    k = k.wrapping_sub(2 as i32 as u32);
    bb = b;
    bk = k;
    if t == 2 as i32 as u32 {
        return inflate_dynamic();
    }
    if t == 0 as i32 as u32 {
        return inflate_stored();
    }
    if t == 1 as i32 as u32 {
        return inflate_fixed();
    }
    return 2 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn inflate() -> i32 {
    let mut e: i32 = 0;
    let mut r: i32 = 0;
    let mut h: u32 = 0;
    outcnt = 0 as i32 as u32;
    bk = 0 as i32 as u32;
    bb = 0 as i32 as ulg;
    h = 0 as i32 as u32;
    loop {
        hufts = 0 as i32 as u32;
        r = inflate_block(&mut e);
        if r != 0 as i32 {
            return r;
        }
        if hufts > h {
            h = hufts;
        }
        if !(e == 0) {
            break;
        }
    }
    while bk >= 8 as i32 as u32 {
        bk = bk.wrapping_sub(8 as i32 as u32);
        inptr = inptr.wrapping_sub(1);
        inptr;
    }
    outcnt = outcnt;
    flush_window();
    return 0 as i32;
}