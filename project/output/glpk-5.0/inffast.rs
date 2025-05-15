use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    pub type internal_state;
}
pub type Byte = u8;
pub type uInt = u32;
pub type uLong = u64;
pub type Bytef = Byte;
pub type voidpf = *mut libc::c_void;
pub type alloc_func = Option<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut i8,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: i32,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gz_header_s {
    pub text: i32,
    pub time: uLong,
    pub xflags: i32,
    pub os: i32,
    pub extra: *mut Bytef,
    pub extra_len: uInt,
    pub extra_max: uInt,
    pub name: *mut Bytef,
    pub name_max: uInt,
    pub comment: *mut Bytef,
    pub comm_max: uInt,
    pub hcrc: i32,
    pub done: i32,
}
pub type gz_header = gz_header_s;
pub type gz_headerp = *mut gz_header;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct code {
    pub op: u8,
    pub bits: u8,
    pub val: libc::c_ushort,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum inflate_mode {
    HEAD,
    FLAGS,
    TIME,
    OS,
    EXLEN,
    EXTRA,
    NAME,
    COMMENT,
    HCRC,
    DICTID,
    DICT,
    TYPE,
    TYPEDO,
    STORED,
    COPY_,
    COPY,
    TABLE,
    LENLENS,
    CODELENS,
    LEN_,
    LEN,
    LENEXT,
    DIST,
    DISTEXT,
    MATCH,
    LIT,
    CHECK,
    LENGTH,
    DONE,
    BAD,
    MEM,
    SYNC,
}
impl inflate_mode {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            inflate_mode::HEAD => 0,
            inflate_mode::FLAGS => 1,
            inflate_mode::TIME => 2,
            inflate_mode::OS => 3,
            inflate_mode::EXLEN => 4,
            inflate_mode::EXTRA => 5,
            inflate_mode::NAME => 6,
            inflate_mode::COMMENT => 7,
            inflate_mode::HCRC => 8,
            inflate_mode::DICTID => 9,
            inflate_mode::DICT => 10,
            inflate_mode::TYPE => 11,
            inflate_mode::TYPEDO => 12,
            inflate_mode::STORED => 13,
            inflate_mode::COPY_ => 14,
            inflate_mode::COPY => 15,
            inflate_mode::TABLE => 16,
            inflate_mode::LENLENS => 17,
            inflate_mode::CODELENS => 18,
            inflate_mode::LEN_ => 19,
            inflate_mode::LEN => 20,
            inflate_mode::LENEXT => 21,
            inflate_mode::DIST => 22,
            inflate_mode::DISTEXT => 23,
            inflate_mode::MATCH => 24,
            inflate_mode::LIT => 25,
            inflate_mode::CHECK => 26,
            inflate_mode::LENGTH => 27,
            inflate_mode::DONE => 28,
            inflate_mode::BAD => 29,
            inflate_mode::MEM => 30,
            inflate_mode::SYNC => 31,
        }
    }
    fn from_libc_c_uint(value: u32) -> inflate_mode {
        match value {
            0 => inflate_mode::HEAD,
            1 => inflate_mode::FLAGS,
            2 => inflate_mode::TIME,
            3 => inflate_mode::OS,
            4 => inflate_mode::EXLEN,
            5 => inflate_mode::EXTRA,
            6 => inflate_mode::NAME,
            7 => inflate_mode::COMMENT,
            8 => inflate_mode::HCRC,
            9 => inflate_mode::DICTID,
            10 => inflate_mode::DICT,
            11 => inflate_mode::TYPE,
            12 => inflate_mode::TYPEDO,
            13 => inflate_mode::STORED,
            14 => inflate_mode::COPY_,
            15 => inflate_mode::COPY,
            16 => inflate_mode::TABLE,
            17 => inflate_mode::LENLENS,
            18 => inflate_mode::CODELENS,
            19 => inflate_mode::LEN_,
            20 => inflate_mode::LEN,
            21 => inflate_mode::LENEXT,
            22 => inflate_mode::DIST,
            23 => inflate_mode::DISTEXT,
            24 => inflate_mode::MATCH,
            25 => inflate_mode::LIT,
            26 => inflate_mode::CHECK,
            27 => inflate_mode::LENGTH,
            28 => inflate_mode::DONE,
            29 => inflate_mode::BAD,
            30 => inflate_mode::MEM,
            31 => inflate_mode::SYNC,
            _ => panic!("Invalid value for inflate_mode: {}", value),
        }
    }
}
impl AddAssign<u32> for inflate_mode {
    fn add_assign(&mut self, rhs: u32) {
        *self = inflate_mode::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for inflate_mode {
    fn sub_assign(&mut self, rhs: u32) {
        *self = inflate_mode::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for inflate_mode {
    fn mul_assign(&mut self, rhs: u32) {
        *self = inflate_mode::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for inflate_mode {
    fn div_assign(&mut self, rhs: u32) {
        *self = inflate_mode::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for inflate_mode {
    fn rem_assign(&mut self, rhs: u32) {
        *self = inflate_mode::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for inflate_mode {
    type Output = inflate_mode;
    fn add(self, rhs: u32) -> inflate_mode {
        inflate_mode::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for inflate_mode {
    type Output = inflate_mode;
    fn sub(self, rhs: u32) -> inflate_mode {
        inflate_mode::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for inflate_mode {
    type Output = inflate_mode;
    fn mul(self, rhs: u32) -> inflate_mode {
        inflate_mode::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for inflate_mode {
    type Output = inflate_mode;
    fn div(self, rhs: u32) -> inflate_mode {
        inflate_mode::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for inflate_mode {
    type Output = inflate_mode;
    fn rem(self, rhs: u32) -> inflate_mode {
        inflate_mode::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inflate_state {
    pub mode: inflate_mode,
    pub last: i32,
    pub wrap: i32,
    pub havedict: i32,
    pub flags: i32,
    pub dmax: u32,
    pub check: u64,
    pub total: u64,
    pub head: gz_headerp,
    pub wbits: u32,
    pub wsize: u32,
    pub whave: u32,
    pub wnext: u32,
    pub window: *mut u8,
    pub hold: u64,
    pub bits: u32,
    pub length: u32,
    pub offset: u32,
    pub extra: u32,
    pub lencode: *const code,
    pub distcode: *const code,
    pub lenbits: u32,
    pub distbits: u32,
    pub ncode: u32,
    pub nlen: u32,
    pub ndist: u32,
    pub have: u32,
    pub next: *mut code,
    pub lens: [libc::c_ushort; 320],
    pub work: [libc::c_ushort; 288],
    pub codes: [code; 1444],
    pub sane: i32,
    pub back: i32,
    pub was: u32,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_inflate_fast(mut strm: z_streamp, mut start: u32) {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    let mut in_0: *mut u8 = 0 as *mut u8;
    let mut last: *mut u8 = 0 as *mut u8;
    let mut out: *mut u8 = 0 as *mut u8;
    let mut beg: *mut u8 = 0 as *mut u8;
    let mut end: *mut u8 = 0 as *mut u8;
    let mut wsize: u32 = 0;
    let mut whave: u32 = 0;
    let mut wnext: u32 = 0;
    let mut window: *mut u8 = 0 as *mut u8;
    let mut hold: u64 = 0;
    let mut bits: u32 = 0;
    let mut lcode: *const code = 0 as *const code;
    let mut dcode: *const code = 0 as *const code;
    let mut lmask: u32 = 0;
    let mut dmask: u32 = 0;
    let mut here: code = code { op: 0, bits: 0, val: 0 };
    let mut op: u32 = 0;
    let mut len: u32 = 0;
    let mut dist: u32 = 0;
    let mut from: *mut u8 = 0 as *mut u8;
    state = (*strm).state as *mut inflate_state;
    in_0 = ((*strm).next_in).offset(-(1 as i32 as isize));
    last = in_0.offset(((*strm).avail_in).wrapping_sub(5 as i32 as u32) as isize);
    out = ((*strm).next_out).offset(-(1 as i32 as isize));
    beg = out.offset(-(start.wrapping_sub((*strm).avail_out) as isize));
    end = out.offset(((*strm).avail_out).wrapping_sub(257 as i32 as u32) as isize);
    wsize = (*state).wsize;
    whave = (*state).whave;
    wnext = (*state).wnext;
    window = (*state).window;
    hold = (*state).hold;
    bits = (*state).bits;
    lcode = (*state).lencode;
    dcode = (*state).distcode;
    lmask = ((1 as u32) << (*state).lenbits).wrapping_sub(1 as i32 as u32);
    dmask = ((1 as u32) << (*state).distbits).wrapping_sub(1 as i32 as u32);
    let mut current_block_141: u64;
    's_94: loop {
        if bits < 15 as i32 as u32 {
            in_0 = in_0.offset(1);
            hold = hold.wrapping_add((*in_0 as u64) << bits);
            bits = bits.wrapping_add(8 as i32 as u32);
            in_0 = in_0.offset(1);
            hold = hold.wrapping_add((*in_0 as u64) << bits);
            bits = bits.wrapping_add(8 as i32 as u32);
        }
        here = *lcode.offset((hold & lmask as u64) as isize);
        loop {
            op = here.bits as u32;
            hold >>= op;
            bits = bits.wrapping_sub(op);
            op = here.op as u32;
            if op == 0 as i32 as u32 {
                out = out.offset(1);
                *out = here.val as u8;
                current_block_141 = 5689001924483802034;
                break;
            } else if op & 16 as i32 as u32 != 0 {
                len = here.val as u32;
                op &= 15 as i32 as u32;
                if op != 0 {
                    if bits < op {
                        in_0 = in_0.offset(1);
                        hold = hold.wrapping_add((*in_0 as u64) << bits);
                        bits = bits.wrapping_add(8 as i32 as u32);
                    }
                    len = len
                        .wrapping_add(
                            hold as u32
                                & ((1 as u32) << op).wrapping_sub(1 as i32 as u32),
                        );
                    hold >>= op;
                    bits = bits.wrapping_sub(op);
                }
                if bits < 15 as i32 as u32 {
                    in_0 = in_0.offset(1);
                    hold = hold.wrapping_add((*in_0 as u64) << bits);
                    bits = bits.wrapping_add(8 as i32 as u32);
                    in_0 = in_0.offset(1);
                    hold = hold.wrapping_add((*in_0 as u64) << bits);
                    bits = bits.wrapping_add(8 as i32 as u32);
                }
                here = *dcode.offset((hold & dmask as u64) as isize);
                current_block_141 = 4404580276390720931;
                break;
            } else if op & 64 as i32 as u32 == 0 as i32 as u32 {
                here = *lcode
                    .offset(
                        (here.val as u64)
                            .wrapping_add(
                                hold
                                    & ((1 as u32) << op).wrapping_sub(1 as i32 as u32) as u64,
                            ) as isize,
                    );
            } else if op & 32 as i32 as u32 != 0 {
                current_block_141 = 13505557363059842426;
                break;
            } else {
                current_block_141 = 9180031981464905198;
                break;
            }
        }
        match current_block_141 {
            4404580276390720931 => {
                loop {
                    op = here.bits as u32;
                    hold >>= op;
                    bits = bits.wrapping_sub(op);
                    op = here.op as u32;
                    if op & 16 as i32 as u32 != 0 {
                        dist = here.val as u32;
                        op &= 15 as i32 as u32;
                        if bits < op {
                            in_0 = in_0.offset(1);
                            hold = hold.wrapping_add((*in_0 as u64) << bits);
                            bits = bits.wrapping_add(8 as i32 as u32);
                            if bits < op {
                                in_0 = in_0.offset(1);
                                hold = hold.wrapping_add((*in_0 as u64) << bits);
                                bits = bits.wrapping_add(8 as i32 as u32);
                            }
                        }
                        dist = dist
                            .wrapping_add(
                                hold as u32
                                    & ((1 as u32) << op).wrapping_sub(1 as i32 as u32),
                            );
                        hold >>= op;
                        bits = bits.wrapping_sub(op);
                        op = out.offset_from(beg) as i64 as u32;
                        if dist > op {
                            current_block_141 = 5235537862154438448;
                            break;
                        } else {
                            current_block_141 = 6072622540298447352;
                            break;
                        }
                    } else if op & 64 as i32 as u32 == 0 as i32 as u32 {
                        here = *dcode
                            .offset(
                                (here.val as u64)
                                    .wrapping_add(
                                        hold
                                            & ((1 as u32) << op).wrapping_sub(1 as i32 as u32) as u64,
                                    ) as isize,
                            );
                    } else {
                        (*strm).msg = b"invalid distance code\0" as *const u8
                            as *const i8 as *mut i8;
                        (*state).mode = inflate_mode::BAD;
                        break 's_94;
                    }
                }
                match current_block_141 {
                    6072622540298447352 => {
                        from = out.offset(-(dist as isize));
                        loop {
                            from = from.offset(1);
                            out = out.offset(1);
                            *out = *from;
                            from = from.offset(1);
                            out = out.offset(1);
                            *out = *from;
                            from = from.offset(1);
                            out = out.offset(1);
                            *out = *from;
                            len = len.wrapping_sub(3 as i32 as u32);
                            if !(len > 2 as i32 as u32) {
                                break;
                            }
                        }
                        if len != 0 {
                            from = from.offset(1);
                            out = out.offset(1);
                            *out = *from;
                            if len > 1 as i32 as u32 {
                                from = from.offset(1);
                                out = out.offset(1);
                                *out = *from;
                            }
                        }
                    }
                    _ => {
                        op = dist.wrapping_sub(op);
                        if op > whave {
                            if (*state).sane != 0 {
                                (*strm).msg = b"invalid distance too far back\0"
                                    as *const u8 as *const i8 as *mut i8;
                                (*state).mode = inflate_mode::BAD;
                                break;
                            }
                        }
                        from = window.offset(-(1 as i32 as isize));
                        if wnext == 0 as i32 as u32 {
                            from = from.offset(wsize.wrapping_sub(op) as isize);
                            if op < len {
                                len = len.wrapping_sub(op);
                                loop {
                                    from = from.offset(1);
                                    out = out.offset(1);
                                    *out = *from;
                                    op = op.wrapping_sub(1);
                                    if !(op != 0) {
                                        break;
                                    }
                                }
                                from = out.offset(-(dist as isize));
                            }
                        } else if wnext < op {
                            from = from
                                .offset(
                                    wsize.wrapping_add(wnext).wrapping_sub(op) as isize,
                                );
                            op = op.wrapping_sub(wnext);
                            if op < len {
                                len = len.wrapping_sub(op);
                                loop {
                                    from = from.offset(1);
                                    out = out.offset(1);
                                    *out = *from;
                                    op = op.wrapping_sub(1);
                                    if !(op != 0) {
                                        break;
                                    }
                                }
                                from = window.offset(-(1 as i32 as isize));
                                if wnext < len {
                                    op = wnext;
                                    len = len.wrapping_sub(op);
                                    loop {
                                        from = from.offset(1);
                                        out = out.offset(1);
                                        *out = *from;
                                        op = op.wrapping_sub(1);
                                        if !(op != 0) {
                                            break;
                                        }
                                    }
                                    from = out.offset(-(dist as isize));
                                }
                            }
                        } else {
                            from = from.offset(wnext.wrapping_sub(op) as isize);
                            if op < len {
                                len = len.wrapping_sub(op);
                                loop {
                                    from = from.offset(1);
                                    out = out.offset(1);
                                    *out = *from;
                                    op = op.wrapping_sub(1);
                                    if !(op != 0) {
                                        break;
                                    }
                                }
                                from = out.offset(-(dist as isize));
                            }
                        }
                        while len > 2 as i32 as u32 {
                            from = from.offset(1);
                            out = out.offset(1);
                            *out = *from;
                            from = from.offset(1);
                            out = out.offset(1);
                            *out = *from;
                            from = from.offset(1);
                            out = out.offset(1);
                            *out = *from;
                            len = len.wrapping_sub(3 as i32 as u32);
                        }
                        if len != 0 {
                            from = from.offset(1);
                            out = out.offset(1);
                            *out = *from;
                            if len > 1 as i32 as u32 {
                                from = from.offset(1);
                                out = out.offset(1);
                                *out = *from;
                            }
                        }
                    }
                }
            }
            9180031981464905198 => {
                (*strm).msg = b"invalid literal/length code\0" as *const u8 as *const i8
                    as *mut i8;
                (*state).mode = inflate_mode::BAD;
                break;
            }
            13505557363059842426 => {
                (*state).mode = inflate_mode::TYPE;
                break;
            }
            _ => {}
        }
        if !(in_0 < last && out < end) {
            break;
        }
    }
    len = bits >> 3 as i32;
    in_0 = in_0.offset(-(len as isize));
    bits = bits.wrapping_sub(len << 3 as i32);
    hold &= ((1 as u32) << bits).wrapping_sub(1 as i32 as u32) as u64;
    (*strm).next_in = in_0.offset(1 as i32 as isize);
    (*strm).next_out = out.offset(1 as i32 as isize);
    (*strm).avail_in = (if in_0 < last {
        5 as i32 as i64 + last.offset_from(in_0) as i64
    } else {
        5 as i32 as i64 - in_0.offset_from(last) as i64
    }) as u32;
    (*strm).avail_out = (if out < end {
        257 as i32 as i64 + end.offset_from(out) as i64
    } else {
        257 as i32 as i64 - out.offset_from(end) as i64
    }) as u32;
    (*state).hold = hold;
    (*state).bits = bits;
}