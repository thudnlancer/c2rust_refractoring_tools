use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    pub type internal_state;
    fn _glp_zlib_adler32(adler: uLong, buf: *const Bytef, len: uInt) -> uLong;
    fn _glp_zlib_crc32(crc: uLong, buf: *const Bytef, len: uInt) -> uLong;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn _glp_zlib_zcfree(opaque: voidpf, ptr: voidpf);
    fn _glp_zlib_zcalloc(opaque: voidpf, items: u32, size: u32) -> voidpf;
    fn _glp_zlib_inflate_table(
        type_0: codetype,
        lens: *mut libc::c_ushort,
        codes: u32,
        table: *mut *mut code,
        bits: *mut u32,
        work: *mut libc::c_ushort,
    ) -> i32;
    fn _glp_zlib_inflate_fast(strm: z_streamp, start: u32);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct code {
    pub op: u8,
    pub bits: u8,
    pub val: libc::c_ushort,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum codetype {
    CODES,
    LENS,
    DISTS,
}
impl codetype {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            codetype::CODES => 0,
            codetype::LENS => 1,
            codetype::DISTS => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> codetype {
        match value {
            0 => codetype::CODES,
            1 => codetype::LENS,
            2 => codetype::DISTS,
            _ => panic!("Invalid value for codetype: {}", value),
        }
    }
}
impl AddAssign<u32> for codetype {
    fn add_assign(&mut self, rhs: u32) {
        *self = codetype::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for codetype {
    fn sub_assign(&mut self, rhs: u32) {
        *self = codetype::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for codetype {
    fn mul_assign(&mut self, rhs: u32) {
        *self = codetype::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for codetype {
    fn div_assign(&mut self, rhs: u32) {
        *self = codetype::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for codetype {
    fn rem_assign(&mut self, rhs: u32) {
        *self = codetype::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for codetype {
    type Output = codetype;
    fn add(self, rhs: u32) -> codetype {
        codetype::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for codetype {
    type Output = codetype;
    fn sub(self, rhs: u32) -> codetype {
        codetype::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for codetype {
    type Output = codetype;
    fn mul(self, rhs: u32) -> codetype {
        codetype::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for codetype {
    type Output = codetype;
    fn div(self, rhs: u32) -> codetype {
        codetype::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for codetype {
    type Output = codetype;
    fn rem(self, rhs: u32) -> codetype {
        codetype::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_inflateReset(mut strm: z_streamp) -> i32 {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if strm.is_null() || ((*strm).state).is_null() {
        return -(2 as i32);
    }
    state = (*strm).state as *mut inflate_state;
    (*state).total = 0 as i32 as u64;
    (*strm).total_out = (*state).total;
    (*strm).total_in = (*strm).total_out;
    (*strm).msg = 0 as *mut i8;
    (*strm).adler = 1 as i32 as uLong;
    (*state).mode = inflate_mode::HEAD;
    (*state).last = 0 as i32;
    (*state).havedict = 0 as i32;
    (*state).dmax = 32768 as u32;
    (*state).head = 0 as gz_headerp;
    (*state).wsize = 0 as i32 as u32;
    (*state).whave = 0 as i32 as u32;
    (*state).wnext = 0 as i32 as u32;
    (*state).hold = 0 as i32 as u64;
    (*state).bits = 0 as i32 as u32;
    (*state).next = ((*state).codes).as_mut_ptr();
    (*state).distcode = (*state).next;
    (*state).lencode = (*state).distcode;
    (*state).sane = 1 as i32;
    (*state).back = -(1 as i32);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_inflateReset2(
    mut strm: z_streamp,
    mut windowBits: i32,
) -> i32 {
    let mut wrap: i32 = 0;
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if strm.is_null() || ((*strm).state).is_null() {
        return -(2 as i32);
    }
    state = (*strm).state as *mut inflate_state;
    if windowBits < 0 as i32 {
        wrap = 0 as i32;
        windowBits = -windowBits;
    } else {
        wrap = (windowBits >> 4 as i32) + 1 as i32;
        if windowBits < 48 as i32 {
            windowBits &= 15 as i32;
        }
    }
    if windowBits != 0 && (windowBits < 8 as i32 || windowBits > 15 as i32) {
        return -(2 as i32);
    }
    if !((*state).window).is_null() && (*state).wbits != windowBits as u32 {
        (Some(((*strm).zfree).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )((*strm).opaque, (*state).window as voidpf);
        (*state).window = 0 as *mut u8;
    }
    (*state).wrap = wrap;
    (*state).wbits = windowBits as u32;
    return _glp_zlib_inflateReset(strm);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_inflateInit2_(
    mut strm: z_streamp,
    mut windowBits: i32,
    mut version: *const i8,
    mut stream_size: i32,
) -> i32 {
    let mut ret: i32 = 0;
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if version.is_null()
        || *version.offset(0 as i32 as isize) as i32
            != (*::core::mem::transmute::<
                &[u8; 6],
                &[i8; 6],
            >(b"1.2.5\0"))[0 as i32 as usize] as i32
        || stream_size != ::core::mem::size_of::<z_stream>() as u64 as i32
    {
        return -(6 as i32);
    }
    if strm.is_null() {
        return -(2 as i32);
    }
    (*strm).msg = 0 as *mut i8;
    if ((*strm).zalloc).is_none() {
        (*strm).zalloc = Some(
            _glp_zlib_zcalloc as unsafe extern "C" fn(voidpf, u32, u32) -> voidpf,
        );
        (*strm).opaque = 0 as voidpf;
    }
    if ((*strm).zfree).is_none() {
        (*strm).zfree = Some(
            _glp_zlib_zcfree as unsafe extern "C" fn(voidpf, voidpf) -> (),
        );
    }
    state = (Some(((*strm).zalloc).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*strm).opaque,
        1 as i32 as uInt,
        ::core::mem::size_of::<inflate_state>() as u64 as uInt,
    ) as *mut inflate_state;
    if state.is_null() {
        return -(4 as i32);
    }
    (*strm).state = state as *mut internal_state;
    (*state).window = 0 as *mut u8;
    ret = _glp_zlib_inflateReset2(strm, windowBits);
    if ret != 0 as i32 {
        (Some(((*strm).zfree).expect("non-null function pointer")))
            .expect("non-null function pointer")((*strm).opaque, state as voidpf);
        (*strm).state = 0 as *mut internal_state;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_inflateInit_(
    mut strm: z_streamp,
    mut version: *const i8,
    mut stream_size: i32,
) -> i32 {
    return _glp_zlib_inflateInit2_(strm, 15 as i32, version, stream_size);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_inflatePrime(
    mut strm: z_streamp,
    mut bits: i32,
    mut value: i32,
) -> i32 {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if strm.is_null() || ((*strm).state).is_null() {
        return -(2 as i32);
    }
    state = (*strm).state as *mut inflate_state;
    if bits < 0 as i32 {
        (*state).hold = 0 as i32 as u64;
        (*state).bits = 0 as i32 as u32;
        return 0 as i32;
    }
    if bits > 16 as i32 || ((*state).bits).wrapping_add(bits as u32) > 32 as i32 as u32 {
        return -(2 as i32);
    }
    value = (value as i64 & ((1 as i64) << bits) - 1 as i32 as i64) as i32;
    (*state).hold = ((*state).hold).wrapping_add((value << (*state).bits) as u64);
    (*state).bits = ((*state).bits).wrapping_add(bits as u32);
    return 0 as i32;
}
unsafe extern "C" fn fixedtables(mut state: *mut inflate_state) {
    static mut lenfix: [code; 512] = [
        {
            let mut init = code {
                op: 96 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 0 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 80 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 16 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 115 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 31 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 112 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 48 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 192 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 10 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 96 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 32 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 160 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 0 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 128 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 64 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 224 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 6 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 88 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 24 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 144 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 59 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 120 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 56 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 208 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 17 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 104 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 40 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 176 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 8 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 136 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 72 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 240 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 4 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 84 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 20 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 227 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 43 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 116 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 52 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 200 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 13 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 100 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 36 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 168 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 4 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 132 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 68 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 232 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 8 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 92 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 28 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 152 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 83 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 124 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 60 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 216 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 23 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 108 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 44 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 184 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 12 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 140 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 76 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 248 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 3 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 82 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 18 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 163 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 35 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 114 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 50 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 196 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 11 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 98 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 34 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 164 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 2 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 130 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 66 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 228 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 7 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 90 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 26 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 148 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 67 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 122 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 58 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 212 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 19 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 106 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 42 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 180 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 10 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 138 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 74 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 244 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 5 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 86 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 22 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 64 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 0 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 51 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 118 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 54 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 204 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 15 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 102 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 38 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 172 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 6 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 134 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 70 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 236 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 9 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 94 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 30 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 156 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 99 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 126 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 62 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 220 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 27 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 110 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 46 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 188 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 14 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 142 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 78 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 252 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 96 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 0 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 81 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 17 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 131 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 31 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 113 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 49 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 194 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 10 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 97 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 33 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 162 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 1 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 129 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 65 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 226 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 6 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 89 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 25 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 146 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 59 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 121 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 57 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 210 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 17 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 105 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 41 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 178 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 9 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 137 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 73 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 242 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 4 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 85 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 21 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 258 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 43 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 117 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 53 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 202 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 13 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 101 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 37 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 170 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 5 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 133 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 69 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 234 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 8 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 93 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 29 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 154 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 83 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 125 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 61 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 218 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 23 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 109 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 45 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 186 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 13 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 141 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 77 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 250 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 3 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 83 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 19 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 195 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 35 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 115 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 51 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 198 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 11 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 99 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 35 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 166 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 3 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 131 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 67 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 230 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 7 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 91 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 27 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 150 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 67 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 123 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 59 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 214 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 19 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 107 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 43 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 182 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 11 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 139 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 75 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 246 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 5 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 87 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 23 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 64 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 0 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 51 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 119 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 55 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 206 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 15 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 103 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 39 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 174 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 7 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 135 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 71 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 238 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 9 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 95 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 31 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 158 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 99 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 127 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 63 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 222 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 27 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 111 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 47 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 190 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 15 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 143 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 79 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 254 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 96 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 0 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 80 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 16 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 115 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 31 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 112 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 48 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 193 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 10 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 96 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 32 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 161 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 0 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 128 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 64 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 225 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 6 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 88 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 24 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 145 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 59 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 120 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 56 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 209 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 17 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 104 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 40 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 177 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 8 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 136 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 72 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 241 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 4 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 84 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 20 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 227 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 43 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 116 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 52 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 201 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 13 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 100 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 36 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 169 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 4 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 132 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 68 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 233 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 8 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 92 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 28 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 153 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 83 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 124 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 60 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 217 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 23 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 108 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 44 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 185 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 12 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 140 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 76 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 249 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 3 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 82 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 18 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 163 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 35 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 114 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 50 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 197 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 11 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 98 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 34 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 165 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 2 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 130 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 66 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 229 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 7 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 90 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 26 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 149 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 67 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 122 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 58 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 213 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 19 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 106 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 42 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 181 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 10 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 138 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 74 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 245 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 5 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 86 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 22 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 64 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 0 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 51 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 118 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 54 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 205 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 15 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 102 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 38 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 173 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 6 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 134 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 70 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 237 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 9 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 94 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 30 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 157 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 99 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 126 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 62 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 221 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 27 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 110 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 46 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 189 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 14 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 142 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 78 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 253 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 96 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 0 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 81 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 17 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 131 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 31 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 113 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 49 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 195 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 10 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 97 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 33 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 163 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 1 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 129 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 65 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 227 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 6 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 89 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 25 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 147 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 59 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 121 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 57 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 211 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 17 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 105 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 41 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 179 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 9 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 137 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 73 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 243 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 4 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 85 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 21 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 258 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 43 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 117 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 53 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 203 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 13 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 101 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 37 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 171 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 5 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 133 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 69 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 235 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 8 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 93 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 29 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 155 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 83 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 125 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 61 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 219 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 23 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 109 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 45 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 187 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 13 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 141 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 77 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 251 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 3 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 83 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 19 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 195 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 35 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 115 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 51 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 199 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 11 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 99 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 35 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 167 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 3 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 131 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 67 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 231 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 7 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 91 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 27 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 151 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 67 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 123 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 59 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 215 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 19 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 107 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 43 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 183 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 11 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 139 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 75 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 247 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 5 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 87 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 23 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 64 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 0 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 51 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 119 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 55 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 207 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 15 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 103 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 39 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 175 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 7 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 135 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 71 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 239 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 9 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 95 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 31 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 159 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 99 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 127 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 63 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 223 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 27 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 111 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 47 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 191 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 15 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 143 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 79 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 255 as i32 as libc::c_ushort,
            };
            init
        },
    ];
    static mut distfix: [code; 32] = [
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 1 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 23 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 257 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 17 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 27 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 4097 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 5 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 25 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 1025 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 65 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 29 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 16385 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 3 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 24 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 513 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 33 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 28 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 8193 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 9 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 26 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 2049 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 22 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 129 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 64 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 0 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 2 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 23 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 385 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 25 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 27 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 6145 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 7 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 25 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 1537 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 97 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 29 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 24577 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 4 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 24 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 769 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 49 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 28 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 12289 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 13 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 26 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 3073 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 22 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 193 as i32 as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 64 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 0 as i32 as libc::c_ushort,
            };
            init
        },
    ];
    (*state).lencode = lenfix.as_ptr();
    (*state).lenbits = 9 as i32 as u32;
    (*state).distcode = distfix.as_ptr();
    (*state).distbits = 5 as i32 as u32;
}
unsafe extern "C" fn updatewindow(mut strm: z_streamp, mut out: u32) -> i32 {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    let mut copy: u32 = 0;
    let mut dist: u32 = 0;
    state = (*strm).state as *mut inflate_state;
    if ((*state).window).is_null() {
        (*state).window = (Some(((*strm).zalloc).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*strm).opaque,
            (1 as u32) << (*state).wbits,
            ::core::mem::size_of::<u8>() as u64 as uInt,
        ) as *mut u8;
        if ((*state).window).is_null() {
            return 1 as i32;
        }
    }
    if (*state).wsize == 0 as i32 as u32 {
        (*state).wsize = (1 as u32) << (*state).wbits;
        (*state).wnext = 0 as i32 as u32;
        (*state).whave = 0 as i32 as u32;
    }
    copy = out.wrapping_sub((*strm).avail_out);
    if copy >= (*state).wsize {
        memcpy(
            (*state).window as *mut libc::c_void,
            ((*strm).next_out).offset(-((*state).wsize as isize)) as *const libc::c_void,
            (*state).wsize as u64,
        );
        (*state).wnext = 0 as i32 as u32;
        (*state).whave = (*state).wsize;
    } else {
        dist = ((*state).wsize).wrapping_sub((*state).wnext);
        if dist > copy {
            dist = copy;
        }
        memcpy(
            ((*state).window).offset((*state).wnext as isize) as *mut libc::c_void,
            ((*strm).next_out).offset(-(copy as isize)) as *const libc::c_void,
            dist as u64,
        );
        copy = copy.wrapping_sub(dist);
        if copy != 0 {
            memcpy(
                (*state).window as *mut libc::c_void,
                ((*strm).next_out).offset(-(copy as isize)) as *const libc::c_void,
                copy as u64,
            );
            (*state).wnext = copy;
            (*state).whave = (*state).wsize;
        } else {
            (*state).wnext = ((*state).wnext).wrapping_add(dist);
            if (*state).wnext == (*state).wsize {
                (*state).wnext = 0 as i32 as u32;
            }
            if (*state).whave < (*state).wsize {
                (*state).whave = ((*state).whave).wrapping_add(dist);
            }
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_inflate(mut strm: z_streamp, mut flush: i32) -> i32 {
    let mut current_block: u64;
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    let mut next: *mut u8 = 0 as *mut u8;
    let mut put: *mut u8 = 0 as *mut u8;
    let mut have: u32 = 0;
    let mut left: u32 = 0;
    let mut hold: u64 = 0;
    let mut bits: u32 = 0;
    let mut in_0: u32 = 0;
    let mut out: u32 = 0;
    let mut copy: u32 = 0;
    let mut from: *mut u8 = 0 as *mut u8;
    let mut here: code = code { op: 0, bits: 0, val: 0 };
    let mut last: code = code { op: 0, bits: 0, val: 0 };
    let mut len: u32 = 0;
    let mut ret: i32 = 0;
    let mut hbuf: [u8; 4] = [0; 4];
    static mut order: [libc::c_ushort; 19] = [
        16 as i32 as libc::c_ushort,
        17 as i32 as libc::c_ushort,
        18 as i32 as libc::c_ushort,
        0 as i32 as libc::c_ushort,
        8 as i32 as libc::c_ushort,
        7 as i32 as libc::c_ushort,
        9 as i32 as libc::c_ushort,
        6 as i32 as libc::c_ushort,
        10 as i32 as libc::c_ushort,
        5 as i32 as libc::c_ushort,
        11 as i32 as libc::c_ushort,
        4 as i32 as libc::c_ushort,
        12 as i32 as libc::c_ushort,
        3 as i32 as libc::c_ushort,
        13 as i32 as libc::c_ushort,
        2 as i32 as libc::c_ushort,
        14 as i32 as libc::c_ushort,
        1 as i32 as libc::c_ushort,
        15 as i32 as libc::c_ushort,
    ];
    if strm.is_null() || ((*strm).state).is_null() || ((*strm).next_out).is_null()
        || ((*strm).next_in).is_null() && (*strm).avail_in != 0 as i32 as u32
    {
        return -(2 as i32);
    }
    state = (*strm).state as *mut inflate_state;
    if (*state).mode as u32 == inflate_mode::TYPE as i32 as u32 {
        (*state).mode = inflate_mode::TYPEDO;
    }
    put = (*strm).next_out;
    left = (*strm).avail_out;
    next = (*strm).next_in;
    have = (*strm).avail_in;
    hold = (*state).hold;
    bits = (*state).bits;
    in_0 = have;
    out = left;
    ret = 0 as i32;
    's_88: loop {
        match (*state).mode as u32 {
            0 => {
                if (*state).wrap == 0 as i32 {
                    (*state).mode = inflate_mode::TYPEDO;
                    continue;
                } else {
                    while bits < 16 as i32 as u32 {
                        if have == 0 as i32 as u32 {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        have;
                        let fresh0 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh0 as u64) << bits);
                        bits = bits.wrapping_add(8 as i32 as u32);
                    }
                    if (*state).wrap & 2 as i32 != 0 && hold == 0x8b1f as i32 as u64 {
                        (*state).check = _glp_zlib_crc32(
                            0 as i64 as uLong,
                            0 as *const Bytef,
                            0 as i32 as uInt,
                        );
                        hbuf[0 as i32 as usize] = hold as u8;
                        hbuf[1 as i32 as usize] = (hold >> 8 as i32) as u8;
                        (*state).check = _glp_zlib_crc32(
                            (*state).check,
                            hbuf.as_mut_ptr(),
                            2 as i32 as uInt,
                        );
                        hold = 0 as i32 as u64;
                        bits = 0 as i32 as u32;
                        (*state).mode = inflate_mode::FLAGS;
                        continue;
                    } else {
                        (*state).flags = 0 as i32;
                        if !((*state).head).is_null() {
                            (*(*state).head).done = -(1 as i32);
                        }
                        if (*state).wrap & 1 as i32 == 0
                            || (((hold as u32
                                & ((1 as u32) << 8 as i32).wrapping_sub(1 as i32 as u32))
                                << 8 as i32) as u64)
                                .wrapping_add(hold >> 8 as i32)
                                .wrapping_rem(31 as i32 as u64) != 0
                        {
                            (*strm).msg = b"incorrect header check\0" as *const u8
                                as *const i8 as *mut i8;
                            (*state).mode = inflate_mode::BAD;
                            continue;
                        } else if hold as u32
                            & ((1 as u32) << 4 as i32).wrapping_sub(1 as i32 as u32)
                            != 8 as i32 as u32
                        {
                            (*strm).msg = b"unknown compression method\0" as *const u8
                                as *const i8 as *mut i8;
                            (*state).mode = inflate_mode::BAD;
                            continue;
                        } else {
                            hold >>= 4 as i32;
                            bits = bits.wrapping_sub(4 as i32 as u32);
                            len = (hold as u32
                                & ((1 as u32) << 4 as i32).wrapping_sub(1 as i32 as u32))
                                .wrapping_add(8 as i32 as u32);
                            if (*state).wbits == 0 as i32 as u32 {
                                (*state).wbits = len;
                            } else if len > (*state).wbits {
                                (*strm).msg = b"invalid window size\0" as *const u8
                                    as *const i8 as *mut i8;
                                (*state).mode = inflate_mode::BAD;
                                continue;
                            }
                            (*state).dmax = (1 as u32) << len;
                            (*state).check = _glp_zlib_adler32(
                                0 as i64 as uLong,
                                0 as *const Bytef,
                                0 as i32 as uInt,
                            );
                            (*strm).adler = (*state).check;
                            (*state).mode = inflate_mode::from_libc_c_uint(
                                (if hold & 0x200 as i32 as u64 != 0 {
                                    inflate_mode::DICTID as i32
                                } else {
                                    inflate_mode::TYPE as i32
                                }) as u32,
                            );
                            hold = 0 as i32 as u64;
                            bits = 0 as i32 as u32;
                            continue;
                        }
                    }
                }
            }
            1 => {
                while bits < 16 as i32 as u32 {
                    if have == 0 as i32 as u32 {
                        break 's_88;
                    }
                    have = have.wrapping_sub(1);
                    have;
                    let fresh1 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh1 as u64) << bits);
                    bits = bits.wrapping_add(8 as i32 as u32);
                }
                (*state).flags = hold as i32;
                if (*state).flags & 0xff as i32 != 8 as i32 {
                    (*strm).msg = b"unknown compression method\0" as *const u8
                        as *const i8 as *mut i8;
                    (*state).mode = inflate_mode::BAD;
                    continue;
                } else if (*state).flags & 0xe000 as i32 != 0 {
                    (*strm).msg = b"unknown header flags set\0" as *const u8 as *const i8
                        as *mut i8;
                    (*state).mode = inflate_mode::BAD;
                    continue;
                } else {
                    if !((*state).head).is_null() {
                        (*(*state).head).text = (hold >> 8 as i32 & 1 as i32 as u64)
                            as i32;
                    }
                    if (*state).flags & 0x200 as i32 != 0 {
                        hbuf[0 as i32 as usize] = hold as u8;
                        hbuf[1 as i32 as usize] = (hold >> 8 as i32) as u8;
                        (*state).check = _glp_zlib_crc32(
                            (*state).check,
                            hbuf.as_mut_ptr(),
                            2 as i32 as uInt,
                        );
                    }
                    hold = 0 as i32 as u64;
                    bits = 0 as i32 as u32;
                    (*state).mode = inflate_mode::TIME;
                }
                current_block = 14648606000749551097;
            }
            2 => {
                current_block = 14648606000749551097;
            }
            3 => {
                current_block = 4691324637564808323;
            }
            4 => {
                current_block = 9972291475702099212;
            }
            5 => {
                current_block = 4968302471276406058;
            }
            6 => {
                current_block = 10561208245340211210;
            }
            7 => {
                current_block = 1449162932215994610;
            }
            8 => {
                current_block = 6087309661972766253;
            }
            9 => {
                while bits < 32 as i32 as u32 {
                    if have == 0 as i32 as u32 {
                        break 's_88;
                    }
                    have = have.wrapping_sub(1);
                    have;
                    let fresh10 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh10 as u64) << bits);
                    bits = bits.wrapping_add(8 as i32 as u32);
                }
                (*state).check = (hold >> 24 as i32 & 0xff as i32 as u64)
                    .wrapping_add(hold >> 8 as i32 & 0xff00 as i32 as u64)
                    .wrapping_add((hold & 0xff00 as i32 as u64) << 8 as i32)
                    .wrapping_add((hold & 0xff as i32 as u64) << 24 as i32);
                (*strm).adler = (*state).check;
                hold = 0 as i32 as u64;
                bits = 0 as i32 as u32;
                (*state).mode = inflate_mode::DICT;
                current_block = 16168035904432745724;
            }
            10 => {
                current_block = 16168035904432745724;
            }
            11 => {
                current_block = 3609958504996442327;
            }
            12 => {
                current_block = 7343662559263759439;
            }
            13 => {
                hold >>= bits & 7 as i32 as u32;
                bits = bits.wrapping_sub(bits & 7 as i32 as u32);
                while bits < 32 as i32 as u32 {
                    if have == 0 as i32 as u32 {
                        break 's_88;
                    }
                    have = have.wrapping_sub(1);
                    have;
                    let fresh12 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh12 as u64) << bits);
                    bits = bits.wrapping_add(8 as i32 as u32);
                }
                if hold & 0xffff as i32 as u64
                    != hold >> 16 as i32 ^ 0xffff as i32 as u64
                {
                    (*strm).msg = b"invalid stored block lengths\0" as *const u8
                        as *const i8 as *mut i8;
                    (*state).mode = inflate_mode::BAD;
                    continue;
                } else {
                    (*state).length = hold as u32 & 0xffff as i32 as u32;
                    hold = 0 as i32 as u64;
                    bits = 0 as i32 as u32;
                    (*state).mode = inflate_mode::COPY_;
                    if flush == 6 as i32 {
                        break;
                    }
                }
                current_block = 5033545425852514390;
            }
            14 => {
                current_block = 5033545425852514390;
            }
            15 => {
                current_block = 2999491485055334776;
            }
            16 => {
                while bits < 14 as i32 as u32 {
                    if have == 0 as i32 as u32 {
                        break 's_88;
                    }
                    have = have.wrapping_sub(1);
                    have;
                    let fresh13 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh13 as u64) << bits);
                    bits = bits.wrapping_add(8 as i32 as u32);
                }
                (*state).nlen = (hold as u32
                    & ((1 as u32) << 5 as i32).wrapping_sub(1 as i32 as u32))
                    .wrapping_add(257 as i32 as u32);
                hold >>= 5 as i32;
                bits = bits.wrapping_sub(5 as i32 as u32);
                (*state).ndist = (hold as u32
                    & ((1 as u32) << 5 as i32).wrapping_sub(1 as i32 as u32))
                    .wrapping_add(1 as i32 as u32);
                hold >>= 5 as i32;
                bits = bits.wrapping_sub(5 as i32 as u32);
                (*state).ncode = (hold as u32
                    & ((1 as u32) << 4 as i32).wrapping_sub(1 as i32 as u32))
                    .wrapping_add(4 as i32 as u32);
                hold >>= 4 as i32;
                bits = bits.wrapping_sub(4 as i32 as u32);
                if (*state).nlen > 286 as i32 as u32 || (*state).ndist > 30 as i32 as u32
                {
                    (*strm).msg = b"too many length or distance symbols\0" as *const u8
                        as *const i8 as *mut i8;
                    (*state).mode = inflate_mode::BAD;
                    continue;
                } else {
                    (*state).have = 0 as i32 as u32;
                    (*state).mode = inflate_mode::LENLENS;
                }
                current_block = 13572305558904641610;
            }
            17 => {
                current_block = 13572305558904641610;
            }
            18 => {
                current_block = 16280550096250383041;
            }
            19 => {
                current_block = 16152771998566204523;
            }
            20 => {
                current_block = 16818959577490016211;
            }
            21 => {
                current_block = 6589414636452177323;
            }
            22 => {
                current_block = 18161743480530436584;
            }
            23 => {
                current_block = 16762320103137014335;
            }
            24 => {
                current_block = 12341693363195858590;
            }
            25 => {
                if left == 0 as i32 as u32 {
                    break;
                }
                let fresh33 = put;
                put = put.offset(1);
                *fresh33 = (*state).length as u8;
                left = left.wrapping_sub(1);
                left;
                (*state).mode = inflate_mode::LEN;
                continue;
            }
            26 => {
                if (*state).wrap != 0 {
                    while bits < 32 as i32 as u32 {
                        if have == 0 as i32 as u32 {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        have;
                        let fresh34 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh34 as u64) << bits);
                        bits = bits.wrapping_add(8 as i32 as u32);
                    }
                    out = out.wrapping_sub(left);
                    (*strm).total_out = ((*strm).total_out as u64)
                        .wrapping_add(out as u64) as uLong as uLong;
                    (*state).total = ((*state).total).wrapping_add(out as u64);
                    if out != 0 {
                        (*state).check = if (*state).flags != 0 {
                            _glp_zlib_crc32(
                                (*state).check,
                                put.offset(-(out as isize)),
                                out,
                            )
                        } else {
                            _glp_zlib_adler32(
                                (*state).check,
                                put.offset(-(out as isize)),
                                out,
                            )
                        };
                        (*strm).adler = (*state).check;
                    }
                    out = left;
                    if (if (*state).flags != 0 {
                        hold
                    } else {
                        (hold >> 24 as i32 & 0xff as i32 as u64)
                            .wrapping_add(hold >> 8 as i32 & 0xff00 as i32 as u64)
                            .wrapping_add((hold & 0xff00 as i32 as u64) << 8 as i32)
                            .wrapping_add((hold & 0xff as i32 as u64) << 24 as i32)
                    }) != (*state).check
                    {
                        (*strm).msg = b"incorrect data check\0" as *const u8 as *const i8
                            as *mut i8;
                        (*state).mode = inflate_mode::BAD;
                        continue;
                    } else {
                        hold = 0 as i32 as u64;
                        bits = 0 as i32 as u32;
                    }
                }
                (*state).mode = inflate_mode::LENGTH;
                current_block = 2334784937562835604;
            }
            27 => {
                current_block = 2334784937562835604;
            }
            28 => {
                current_block = 4046302689674688614;
            }
            29 => {
                ret = -(3 as i32);
                break;
            }
            30 => return -(4 as i32),
            31 | _ => return -(2 as i32),
        }
        match current_block {
            2334784937562835604 => {
                if (*state).wrap != 0 && (*state).flags != 0 {
                    while bits < 32 as i32 as u32 {
                        if have == 0 as i32 as u32 {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        have;
                        let fresh35 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh35 as u64) << bits);
                        bits = bits.wrapping_add(8 as i32 as u32);
                    }
                    if hold != (*state).total & 0xffffffff as u64 {
                        (*strm).msg = b"incorrect length check\0" as *const u8
                            as *const i8 as *mut i8;
                        (*state).mode = inflate_mode::BAD;
                        continue;
                    } else {
                        hold = 0 as i32 as u64;
                        bits = 0 as i32 as u32;
                    }
                }
                (*state).mode = inflate_mode::DONE;
                current_block = 4046302689674688614;
            }
            13572305558904641610 => {
                while (*state).have < (*state).ncode {
                    while bits < 3 as i32 as u32 {
                        if have == 0 as i32 as u32 {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        have;
                        let fresh14 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh14 as u64) << bits);
                        bits = bits.wrapping_add(8 as i32 as u32);
                    }
                    let fresh15 = (*state).have;
                    (*state).have = ((*state).have).wrapping_add(1);
                    (*state).lens[order[fresh15 as usize] as usize] = (hold as u32
                        & ((1 as u32) << 3 as i32).wrapping_sub(1 as i32 as u32))
                        as libc::c_ushort;
                    hold >>= 3 as i32;
                    bits = bits.wrapping_sub(3 as i32 as u32);
                }
                while (*state).have < 19 as i32 as u32 {
                    let fresh16 = (*state).have;
                    (*state).have = ((*state).have).wrapping_add(1);
                    (*state).lens[order[fresh16 as usize] as usize] = 0 as i32
                        as libc::c_ushort;
                }
                (*state).next = ((*state).codes).as_mut_ptr();
                (*state).lencode = (*state).next as *const code;
                (*state).lenbits = 7 as i32 as u32;
                ret = _glp_zlib_inflate_table(
                    codetype::CODES,
                    ((*state).lens).as_mut_ptr(),
                    19 as i32 as u32,
                    &mut (*state).next,
                    &mut (*state).lenbits,
                    ((*state).work).as_mut_ptr(),
                );
                if ret != 0 {
                    (*strm).msg = b"invalid code lengths set\0" as *const u8 as *const i8
                        as *mut i8;
                    (*state).mode = inflate_mode::BAD;
                    continue;
                } else {
                    (*state).have = 0 as i32 as u32;
                    (*state).mode = inflate_mode::CODELENS;
                }
                current_block = 16280550096250383041;
            }
            16168035904432745724 => {
                if (*state).havedict == 0 as i32 {
                    (*strm).next_out = put;
                    (*strm).avail_out = left;
                    (*strm).next_in = next;
                    (*strm).avail_in = have;
                    (*state).hold = hold;
                    (*state).bits = bits;
                    return 2 as i32;
                }
                (*state).check = _glp_zlib_adler32(
                    0 as i64 as uLong,
                    0 as *const Bytef,
                    0 as i32 as uInt,
                );
                (*strm).adler = (*state).check;
                (*state).mode = inflate_mode::TYPE;
                current_block = 3609958504996442327;
            }
            14648606000749551097 => {
                while bits < 32 as i32 as u32 {
                    if have == 0 as i32 as u32 {
                        break 's_88;
                    }
                    have = have.wrapping_sub(1);
                    have;
                    let fresh2 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh2 as u64) << bits);
                    bits = bits.wrapping_add(8 as i32 as u32);
                }
                if !((*state).head).is_null() {
                    (*(*state).head).time = hold;
                }
                if (*state).flags & 0x200 as i32 != 0 {
                    hbuf[0 as i32 as usize] = hold as u8;
                    hbuf[1 as i32 as usize] = (hold >> 8 as i32) as u8;
                    hbuf[2 as i32 as usize] = (hold >> 16 as i32) as u8;
                    hbuf[3 as i32 as usize] = (hold >> 24 as i32) as u8;
                    (*state).check = _glp_zlib_crc32(
                        (*state).check,
                        hbuf.as_mut_ptr(),
                        4 as i32 as uInt,
                    );
                }
                hold = 0 as i32 as u64;
                bits = 0 as i32 as u32;
                (*state).mode = inflate_mode::OS;
                current_block = 4691324637564808323;
            }
            5033545425852514390 => {
                (*state).mode = inflate_mode::COPY;
                current_block = 2999491485055334776;
            }
            _ => {}
        }
        match current_block {
            16280550096250383041 => {
                while (*state).have < ((*state).nlen).wrapping_add((*state).ndist) {
                    loop {
                        here = *((*state).lencode)
                            .offset(
                                (hold as u32
                                    & ((1 as u32) << (*state).lenbits)
                                        .wrapping_sub(1 as i32 as u32)) as isize,
                            );
                        if here.bits as u32 <= bits {
                            break;
                        }
                        if have == 0 as i32 as u32 {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        have;
                        let fresh17 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh17 as u64) << bits);
                        bits = bits.wrapping_add(8 as i32 as u32);
                    }
                    if (here.val as i32) < 16 as i32 {
                        while bits < here.bits as u32 {
                            if have == 0 as i32 as u32 {
                                break 's_88;
                            }
                            have = have.wrapping_sub(1);
                            have;
                            let fresh18 = next;
                            next = next.offset(1);
                            hold = hold.wrapping_add((*fresh18 as u64) << bits);
                            bits = bits.wrapping_add(8 as i32 as u32);
                        }
                        hold >>= here.bits as i32;
                        bits = bits.wrapping_sub(here.bits as u32);
                        let fresh19 = (*state).have;
                        (*state).have = ((*state).have).wrapping_add(1);
                        (*state).lens[fresh19 as usize] = here.val;
                    } else {
                        if here.val as i32 == 16 as i32 {
                            while bits < (here.bits as i32 + 2 as i32) as u32 {
                                if have == 0 as i32 as u32 {
                                    break 's_88;
                                }
                                have = have.wrapping_sub(1);
                                have;
                                let fresh20 = next;
                                next = next.offset(1);
                                hold = hold.wrapping_add((*fresh20 as u64) << bits);
                                bits = bits.wrapping_add(8 as i32 as u32);
                            }
                            hold >>= here.bits as i32;
                            bits = bits.wrapping_sub(here.bits as u32);
                            if (*state).have == 0 as i32 as u32 {
                                (*strm).msg = b"invalid bit length repeat\0" as *const u8
                                    as *const i8 as *mut i8;
                                (*state).mode = inflate_mode::BAD;
                                break;
                            } else {
                                len = (*state)
                                    .lens[((*state).have).wrapping_sub(1 as i32 as u32)
                                    as usize] as u32;
                                copy = (3 as i32 as u32)
                                    .wrapping_add(
                                        hold as u32
                                            & ((1 as u32) << 2 as i32).wrapping_sub(1 as i32 as u32),
                                    );
                                hold >>= 2 as i32;
                                bits = bits.wrapping_sub(2 as i32 as u32);
                            }
                        } else if here.val as i32 == 17 as i32 {
                            while bits < (here.bits as i32 + 3 as i32) as u32 {
                                if have == 0 as i32 as u32 {
                                    break 's_88;
                                }
                                have = have.wrapping_sub(1);
                                have;
                                let fresh21 = next;
                                next = next.offset(1);
                                hold = hold.wrapping_add((*fresh21 as u64) << bits);
                                bits = bits.wrapping_add(8 as i32 as u32);
                            }
                            hold >>= here.bits as i32;
                            bits = bits.wrapping_sub(here.bits as u32);
                            len = 0 as i32 as u32;
                            copy = (3 as i32 as u32)
                                .wrapping_add(
                                    hold as u32
                                        & ((1 as u32) << 3 as i32).wrapping_sub(1 as i32 as u32),
                                );
                            hold >>= 3 as i32;
                            bits = bits.wrapping_sub(3 as i32 as u32);
                        } else {
                            while bits < (here.bits as i32 + 7 as i32) as u32 {
                                if have == 0 as i32 as u32 {
                                    break 's_88;
                                }
                                have = have.wrapping_sub(1);
                                have;
                                let fresh22 = next;
                                next = next.offset(1);
                                hold = hold.wrapping_add((*fresh22 as u64) << bits);
                                bits = bits.wrapping_add(8 as i32 as u32);
                            }
                            hold >>= here.bits as i32;
                            bits = bits.wrapping_sub(here.bits as u32);
                            len = 0 as i32 as u32;
                            copy = (11 as i32 as u32)
                                .wrapping_add(
                                    hold as u32
                                        & ((1 as u32) << 7 as i32).wrapping_sub(1 as i32 as u32),
                                );
                            hold >>= 7 as i32;
                            bits = bits.wrapping_sub(7 as i32 as u32);
                        }
                        if ((*state).have).wrapping_add(copy)
                            > ((*state).nlen).wrapping_add((*state).ndist)
                        {
                            (*strm).msg = b"invalid bit length repeat\0" as *const u8
                                as *const i8 as *mut i8;
                            (*state).mode = inflate_mode::BAD;
                            break;
                        } else {
                            loop {
                                let fresh23 = copy;
                                copy = copy.wrapping_sub(1);
                                if !(fresh23 != 0) {
                                    break;
                                }
                                let fresh24 = (*state).have;
                                (*state).have = ((*state).have).wrapping_add(1);
                                (*state).lens[fresh24 as usize] = len as libc::c_ushort;
                            }
                        }
                    }
                }
                if (*state).mode as u32 == inflate_mode::BAD as i32 as u32 {
                    continue;
                }
                if (*state).lens[256 as i32 as usize] as i32 == 0 as i32 {
                    (*strm).msg = b"invalid code -- missing end-of-block\0" as *const u8
                        as *const i8 as *mut i8;
                    (*state).mode = inflate_mode::BAD;
                    continue;
                } else {
                    (*state).next = ((*state).codes).as_mut_ptr();
                    (*state).lencode = (*state).next as *const code;
                    (*state).lenbits = 9 as i32 as u32;
                    ret = _glp_zlib_inflate_table(
                        codetype::LENS,
                        ((*state).lens).as_mut_ptr(),
                        (*state).nlen,
                        &mut (*state).next,
                        &mut (*state).lenbits,
                        ((*state).work).as_mut_ptr(),
                    );
                    if ret != 0 {
                        (*strm).msg = b"invalid literal/lengths set\0" as *const u8
                            as *const i8 as *mut i8;
                        (*state).mode = inflate_mode::BAD;
                        continue;
                    } else {
                        (*state).distcode = (*state).next as *const code;
                        (*state).distbits = 6 as i32 as u32;
                        ret = _glp_zlib_inflate_table(
                            codetype::DISTS,
                            ((*state).lens).as_mut_ptr().offset((*state).nlen as isize),
                            (*state).ndist,
                            &mut (*state).next,
                            &mut (*state).distbits,
                            ((*state).work).as_mut_ptr(),
                        );
                        if ret != 0 {
                            (*strm).msg = b"invalid distances set\0" as *const u8
                                as *const i8 as *mut i8;
                            (*state).mode = inflate_mode::BAD;
                            continue;
                        } else {
                            (*state).mode = inflate_mode::LEN_;
                            if flush == 6 as i32 {
                                break;
                            }
                        }
                    }
                }
                current_block = 16152771998566204523;
            }
            2999491485055334776 => {
                copy = (*state).length;
                if copy != 0 {
                    if copy > have {
                        copy = have;
                    }
                    if copy > left {
                        copy = left;
                    }
                    if copy == 0 as i32 as u32 {
                        break;
                    }
                    memcpy(
                        put as *mut libc::c_void,
                        next as *const libc::c_void,
                        copy as u64,
                    );
                    have = have.wrapping_sub(copy);
                    next = next.offset(copy as isize);
                    left = left.wrapping_sub(copy);
                    put = put.offset(copy as isize);
                    (*state).length = ((*state).length).wrapping_sub(copy);
                    continue;
                } else {
                    (*state).mode = inflate_mode::TYPE;
                    continue;
                }
            }
            4691324637564808323 => {
                while bits < 16 as i32 as u32 {
                    if have == 0 as i32 as u32 {
                        break 's_88;
                    }
                    have = have.wrapping_sub(1);
                    have;
                    let fresh3 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh3 as u64) << bits);
                    bits = bits.wrapping_add(8 as i32 as u32);
                }
                if !((*state).head).is_null() {
                    (*(*state).head).xflags = (hold & 0xff as i32 as u64) as i32;
                    (*(*state).head).os = (hold >> 8 as i32) as i32;
                }
                if (*state).flags & 0x200 as i32 != 0 {
                    hbuf[0 as i32 as usize] = hold as u8;
                    hbuf[1 as i32 as usize] = (hold >> 8 as i32) as u8;
                    (*state).check = _glp_zlib_crc32(
                        (*state).check,
                        hbuf.as_mut_ptr(),
                        2 as i32 as uInt,
                    );
                }
                hold = 0 as i32 as u64;
                bits = 0 as i32 as u32;
                (*state).mode = inflate_mode::EXLEN;
                current_block = 9972291475702099212;
            }
            3609958504996442327 => {
                if flush == 5 as i32 || flush == 6 as i32 {
                    break;
                }
                current_block = 7343662559263759439;
            }
            4046302689674688614 => {
                ret = 1 as i32;
                break;
            }
            _ => {}
        }
        match current_block {
            7343662559263759439 => {
                if (*state).last != 0 {
                    hold >>= bits & 7 as i32 as u32;
                    bits = bits.wrapping_sub(bits & 7 as i32 as u32);
                    (*state).mode = inflate_mode::CHECK;
                    continue;
                } else {
                    while bits < 3 as i32 as u32 {
                        if have == 0 as i32 as u32 {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        have;
                        let fresh11 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh11 as u64) << bits);
                        bits = bits.wrapping_add(8 as i32 as u32);
                    }
                    (*state).last = (hold as u32
                        & ((1 as u32) << 1 as i32).wrapping_sub(1 as i32 as u32)) as i32;
                    hold >>= 1 as i32;
                    bits = bits.wrapping_sub(1 as i32 as u32);
                    match hold as u32
                        & ((1 as u32) << 2 as i32).wrapping_sub(1 as i32 as u32)
                    {
                        0 => {
                            (*state).mode = inflate_mode::STORED;
                        }
                        1 => {
                            fixedtables(state);
                            (*state).mode = inflate_mode::LEN_;
                            if flush == 6 as i32 {
                                hold >>= 2 as i32;
                                bits = bits.wrapping_sub(2 as i32 as u32);
                                break;
                            }
                        }
                        2 => {
                            (*state).mode = inflate_mode::TABLE;
                        }
                        3 => {
                            (*strm).msg = b"invalid block type\0" as *const u8
                                as *const i8 as *mut i8;
                            (*state).mode = inflate_mode::BAD;
                        }
                        _ => {}
                    }
                    hold >>= 2 as i32;
                    bits = bits.wrapping_sub(2 as i32 as u32);
                    continue;
                }
            }
            9972291475702099212 => {
                if (*state).flags & 0x400 as i32 != 0 {
                    while bits < 16 as i32 as u32 {
                        if have == 0 as i32 as u32 {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        have;
                        let fresh4 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh4 as u64) << bits);
                        bits = bits.wrapping_add(8 as i32 as u32);
                    }
                    (*state).length = hold as u32;
                    if !((*state).head).is_null() {
                        (*(*state).head).extra_len = hold as u32;
                    }
                    if (*state).flags & 0x200 as i32 != 0 {
                        hbuf[0 as i32 as usize] = hold as u8;
                        hbuf[1 as i32 as usize] = (hold >> 8 as i32) as u8;
                        (*state).check = _glp_zlib_crc32(
                            (*state).check,
                            hbuf.as_mut_ptr(),
                            2 as i32 as uInt,
                        );
                    }
                    hold = 0 as i32 as u64;
                    bits = 0 as i32 as u32;
                } else if !((*state).head).is_null() {
                    (*(*state).head).extra = 0 as *mut Bytef;
                }
                (*state).mode = inflate_mode::EXTRA;
                current_block = 4968302471276406058;
            }
            16152771998566204523 => {
                (*state).mode = inflate_mode::LEN;
                current_block = 16818959577490016211;
            }
            _ => {}
        }
        match current_block {
            16818959577490016211 => {
                if have >= 6 as i32 as u32 && left >= 258 as i32 as u32 {
                    (*strm).next_out = put;
                    (*strm).avail_out = left;
                    (*strm).next_in = next;
                    (*strm).avail_in = have;
                    (*state).hold = hold;
                    (*state).bits = bits;
                    _glp_zlib_inflate_fast(strm, out);
                    put = (*strm).next_out;
                    left = (*strm).avail_out;
                    next = (*strm).next_in;
                    have = (*strm).avail_in;
                    hold = (*state).hold;
                    bits = (*state).bits;
                    if (*state).mode as u32 == inflate_mode::TYPE as i32 as u32 {
                        (*state).back = -(1 as i32);
                    }
                    continue;
                } else {
                    (*state).back = 0 as i32;
                    loop {
                        here = *((*state).lencode)
                            .offset(
                                (hold as u32
                                    & ((1 as u32) << (*state).lenbits)
                                        .wrapping_sub(1 as i32 as u32)) as isize,
                            );
                        if here.bits as u32 <= bits {
                            break;
                        }
                        if have == 0 as i32 as u32 {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        have;
                        let fresh25 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh25 as u64) << bits);
                        bits = bits.wrapping_add(8 as i32 as u32);
                    }
                    if here.op as i32 != 0 && here.op as i32 & 0xf0 as i32 == 0 as i32 {
                        last = here;
                        loop {
                            here = *((*state).lencode)
                                .offset(
                                    (last.val as u32)
                                        .wrapping_add(
                                            (hold as u32
                                                & ((1 as u32) << last.bits as i32 + last.op as i32)
                                                    .wrapping_sub(1 as i32 as u32)) >> last.bits as i32,
                                        ) as isize,
                                );
                            if (last.bits as i32 + here.bits as i32) as u32 <= bits {
                                break;
                            }
                            if have == 0 as i32 as u32 {
                                break 's_88;
                            }
                            have = have.wrapping_sub(1);
                            have;
                            let fresh26 = next;
                            next = next.offset(1);
                            hold = hold.wrapping_add((*fresh26 as u64) << bits);
                            bits = bits.wrapping_add(8 as i32 as u32);
                        }
                        hold >>= last.bits as i32;
                        bits = bits.wrapping_sub(last.bits as u32);
                        (*state).back += last.bits as i32;
                    }
                    hold >>= here.bits as i32;
                    bits = bits.wrapping_sub(here.bits as u32);
                    (*state).back += here.bits as i32;
                    (*state).length = here.val as u32;
                    if here.op as i32 == 0 as i32 {
                        (*state).mode = inflate_mode::LIT;
                        continue;
                    } else if here.op as i32 & 32 as i32 != 0 {
                        (*state).back = -(1 as i32);
                        (*state).mode = inflate_mode::TYPE;
                        continue;
                    } else if here.op as i32 & 64 as i32 != 0 {
                        (*strm).msg = b"invalid literal/length code\0" as *const u8
                            as *const i8 as *mut i8;
                        (*state).mode = inflate_mode::BAD;
                        continue;
                    } else {
                        (*state).extra = here.op as u32 & 15 as i32 as u32;
                        (*state).mode = inflate_mode::LENEXT;
                    }
                }
                current_block = 6589414636452177323;
            }
            4968302471276406058 => {
                if (*state).flags & 0x400 as i32 != 0 {
                    copy = (*state).length;
                    if copy > have {
                        copy = have;
                    }
                    if copy != 0 {
                        if !((*state).head).is_null()
                            && !((*(*state).head).extra).is_null()
                        {
                            len = ((*(*state).head).extra_len)
                                .wrapping_sub((*state).length);
                            memcpy(
                                ((*(*state).head).extra).offset(len as isize)
                                    as *mut libc::c_void,
                                next as *const libc::c_void,
                                (if len.wrapping_add(copy) > (*(*state).head).extra_max {
                                    ((*(*state).head).extra_max).wrapping_sub(len)
                                } else {
                                    copy
                                }) as u64,
                            );
                        }
                        if (*state).flags & 0x200 as i32 != 0 {
                            (*state).check = _glp_zlib_crc32((*state).check, next, copy);
                        }
                        have = have.wrapping_sub(copy);
                        next = next.offset(copy as isize);
                        (*state).length = ((*state).length).wrapping_sub(copy);
                    }
                    if (*state).length != 0 {
                        break;
                    }
                }
                (*state).length = 0 as i32 as u32;
                (*state).mode = inflate_mode::NAME;
                current_block = 10561208245340211210;
            }
            _ => {}
        }
        match current_block {
            6589414636452177323 => {
                if (*state).extra != 0 {
                    while bits < (*state).extra {
                        if have == 0 as i32 as u32 {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        have;
                        let fresh27 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh27 as u64) << bits);
                        bits = bits.wrapping_add(8 as i32 as u32);
                    }
                    (*state).length = ((*state).length)
                        .wrapping_add(
                            hold as u32
                                & ((1 as u32) << (*state).extra)
                                    .wrapping_sub(1 as i32 as u32),
                        );
                    hold >>= (*state).extra;
                    bits = bits.wrapping_sub((*state).extra);
                    (*state).back = ((*state).back as u32).wrapping_add((*state).extra)
                        as i32 as i32;
                }
                (*state).was = (*state).length;
                (*state).mode = inflate_mode::DIST;
                current_block = 18161743480530436584;
            }
            10561208245340211210 => {
                if (*state).flags & 0x800 as i32 != 0 {
                    if have == 0 as i32 as u32 {
                        break;
                    }
                    copy = 0 as i32 as u32;
                    loop {
                        let fresh5 = copy;
                        copy = copy.wrapping_add(1);
                        len = *next.offset(fresh5 as isize) as u32;
                        if !((*state).head).is_null()
                            && !((*(*state).head).name).is_null()
                            && (*state).length < (*(*state).head).name_max
                        {
                            let fresh6 = (*state).length;
                            (*state).length = ((*state).length).wrapping_add(1);
                            *((*(*state).head).name).offset(fresh6 as isize) = len
                                as Bytef;
                        }
                        if !(len != 0 && copy < have) {
                            break;
                        }
                    }
                    if (*state).flags & 0x200 as i32 != 0 {
                        (*state).check = _glp_zlib_crc32((*state).check, next, copy);
                    }
                    have = have.wrapping_sub(copy);
                    next = next.offset(copy as isize);
                    if len != 0 {
                        break;
                    }
                } else if !((*state).head).is_null() {
                    (*(*state).head).name = 0 as *mut Bytef;
                }
                (*state).length = 0 as i32 as u32;
                (*state).mode = inflate_mode::COMMENT;
                current_block = 1449162932215994610;
            }
            _ => {}
        }
        match current_block {
            18161743480530436584 => {
                loop {
                    here = *((*state).distcode)
                        .offset(
                            (hold as u32
                                & ((1 as u32) << (*state).distbits)
                                    .wrapping_sub(1 as i32 as u32)) as isize,
                        );
                    if here.bits as u32 <= bits {
                        break;
                    }
                    if have == 0 as i32 as u32 {
                        break 's_88;
                    }
                    have = have.wrapping_sub(1);
                    have;
                    let fresh28 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh28 as u64) << bits);
                    bits = bits.wrapping_add(8 as i32 as u32);
                }
                if here.op as i32 & 0xf0 as i32 == 0 as i32 {
                    last = here;
                    loop {
                        here = *((*state).distcode)
                            .offset(
                                (last.val as u32)
                                    .wrapping_add(
                                        (hold as u32
                                            & ((1 as u32) << last.bits as i32 + last.op as i32)
                                                .wrapping_sub(1 as i32 as u32)) >> last.bits as i32,
                                    ) as isize,
                            );
                        if (last.bits as i32 + here.bits as i32) as u32 <= bits {
                            break;
                        }
                        if have == 0 as i32 as u32 {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        have;
                        let fresh29 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh29 as u64) << bits);
                        bits = bits.wrapping_add(8 as i32 as u32);
                    }
                    hold >>= last.bits as i32;
                    bits = bits.wrapping_sub(last.bits as u32);
                    (*state).back += last.bits as i32;
                }
                hold >>= here.bits as i32;
                bits = bits.wrapping_sub(here.bits as u32);
                (*state).back += here.bits as i32;
                if here.op as i32 & 64 as i32 != 0 {
                    (*strm).msg = b"invalid distance code\0" as *const u8 as *const i8
                        as *mut i8;
                    (*state).mode = inflate_mode::BAD;
                    continue;
                } else {
                    (*state).offset = here.val as u32;
                    (*state).extra = here.op as u32 & 15 as i32 as u32;
                    (*state).mode = inflate_mode::DISTEXT;
                }
                current_block = 16762320103137014335;
            }
            1449162932215994610 => {
                if (*state).flags & 0x1000 as i32 != 0 {
                    if have == 0 as i32 as u32 {
                        break;
                    }
                    copy = 0 as i32 as u32;
                    loop {
                        let fresh7 = copy;
                        copy = copy.wrapping_add(1);
                        len = *next.offset(fresh7 as isize) as u32;
                        if !((*state).head).is_null()
                            && !((*(*state).head).comment).is_null()
                            && (*state).length < (*(*state).head).comm_max
                        {
                            let fresh8 = (*state).length;
                            (*state).length = ((*state).length).wrapping_add(1);
                            *((*(*state).head).comment).offset(fresh8 as isize) = len
                                as Bytef;
                        }
                        if !(len != 0 && copy < have) {
                            break;
                        }
                    }
                    if (*state).flags & 0x200 as i32 != 0 {
                        (*state).check = _glp_zlib_crc32((*state).check, next, copy);
                    }
                    have = have.wrapping_sub(copy);
                    next = next.offset(copy as isize);
                    if len != 0 {
                        break;
                    }
                } else if !((*state).head).is_null() {
                    (*(*state).head).comment = 0 as *mut Bytef;
                }
                (*state).mode = inflate_mode::HCRC;
                current_block = 6087309661972766253;
            }
            _ => {}
        }
        match current_block {
            16762320103137014335 => {
                if (*state).extra != 0 {
                    while bits < (*state).extra {
                        if have == 0 as i32 as u32 {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        have;
                        let fresh30 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh30 as u64) << bits);
                        bits = bits.wrapping_add(8 as i32 as u32);
                    }
                    (*state).offset = ((*state).offset)
                        .wrapping_add(
                            hold as u32
                                & ((1 as u32) << (*state).extra)
                                    .wrapping_sub(1 as i32 as u32),
                        );
                    hold >>= (*state).extra;
                    bits = bits.wrapping_sub((*state).extra);
                    (*state).back = ((*state).back as u32).wrapping_add((*state).extra)
                        as i32 as i32;
                }
                (*state).mode = inflate_mode::MATCH;
            }
            6087309661972766253 => {
                if (*state).flags & 0x200 as i32 != 0 {
                    while bits < 16 as i32 as u32 {
                        if have == 0 as i32 as u32 {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        have;
                        let fresh9 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh9 as u64) << bits);
                        bits = bits.wrapping_add(8 as i32 as u32);
                    }
                    if hold != (*state).check & 0xffff as i32 as u64 {
                        (*strm).msg = b"header crc mismatch\0" as *const u8 as *const i8
                            as *mut i8;
                        (*state).mode = inflate_mode::BAD;
                        continue;
                    } else {
                        hold = 0 as i32 as u64;
                        bits = 0 as i32 as u32;
                    }
                }
                if !((*state).head).is_null() {
                    (*(*state).head).hcrc = (*state).flags >> 9 as i32 & 1 as i32;
                    (*(*state).head).done = 1 as i32;
                }
                (*state).check = _glp_zlib_crc32(
                    0 as i64 as uLong,
                    0 as *const Bytef,
                    0 as i32 as uInt,
                );
                (*strm).adler = (*state).check;
                (*state).mode = inflate_mode::TYPE;
                continue;
            }
            _ => {}
        }
        if left == 0 as i32 as u32 {
            break;
        }
        copy = out.wrapping_sub(left);
        if (*state).offset > copy {
            copy = ((*state).offset).wrapping_sub(copy);
            if copy > (*state).whave {
                if (*state).sane != 0 {
                    (*strm).msg = b"invalid distance too far back\0" as *const u8
                        as *const i8 as *mut i8;
                    (*state).mode = inflate_mode::BAD;
                    continue;
                }
            }
            if copy > (*state).wnext {
                copy = copy.wrapping_sub((*state).wnext);
                from = ((*state).window)
                    .offset(((*state).wsize).wrapping_sub(copy) as isize);
            } else {
                from = ((*state).window)
                    .offset(((*state).wnext).wrapping_sub(copy) as isize);
            }
            if copy > (*state).length {
                copy = (*state).length;
            }
        } else {
            from = put.offset(-((*state).offset as isize));
            copy = (*state).length;
        }
        if copy > left {
            copy = left;
        }
        left = left.wrapping_sub(copy);
        (*state).length = ((*state).length).wrapping_sub(copy);
        loop {
            let fresh31 = from;
            from = from.offset(1);
            let fresh32 = put;
            put = put.offset(1);
            *fresh32 = *fresh31;
            copy = copy.wrapping_sub(1);
            if !(copy != 0) {
                break;
            }
        }
        if (*state).length == 0 as i32 as u32 {
            (*state).mode = inflate_mode::LEN;
        }
    }
    (*strm).next_out = put;
    (*strm).avail_out = left;
    (*strm).next_in = next;
    (*strm).avail_in = have;
    (*state).hold = hold;
    (*state).bits = bits;
    if (*state).wsize != 0
        || ((*state).mode as u32) < inflate_mode::CHECK as i32 as u32
            && out != (*strm).avail_out
    {
        if updatewindow(strm, out) != 0 {
            (*state).mode = inflate_mode::MEM;
            return -(4 as i32);
        }
    }
    in_0 = in_0.wrapping_sub((*strm).avail_in);
    out = out.wrapping_sub((*strm).avail_out);
    (*strm).total_in = ((*strm).total_in as u64).wrapping_add(in_0 as u64) as uLong
        as uLong;
    (*strm).total_out = ((*strm).total_out as u64).wrapping_add(out as u64) as uLong
        as uLong;
    (*state).total = ((*state).total).wrapping_add(out as u64);
    if (*state).wrap != 0 && out != 0 {
        (*state).check = if (*state).flags != 0 {
            _glp_zlib_crc32(
                (*state).check,
                ((*strm).next_out).offset(-(out as isize)),
                out,
            )
        } else {
            _glp_zlib_adler32(
                (*state).check,
                ((*strm).next_out).offset(-(out as isize)),
                out,
            )
        };
        (*strm).adler = (*state).check;
    }
    (*strm).data_type = ((*state).bits)
        .wrapping_add((if (*state).last != 0 { 64 as i32 } else { 0 as i32 }) as u32)
        .wrapping_add(
            (if (*state).mode as u32 == inflate_mode::TYPE as i32 as u32 {
                128 as i32
            } else {
                0 as i32
            }) as u32,
        )
        .wrapping_add(
            (if (*state).mode as u32 == inflate_mode::LEN_ as i32 as u32
                || (*state).mode as u32 == inflate_mode::COPY_ as i32 as u32
            {
                256 as i32
            } else {
                0 as i32
            }) as u32,
        ) as i32;
    if (in_0 == 0 as i32 as u32 && out == 0 as i32 as u32 || flush == 4 as i32)
        && ret == 0 as i32
    {
        ret = -(5 as i32);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_inflateEnd(mut strm: z_streamp) -> i32 {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if strm.is_null() || ((*strm).state).is_null() || ((*strm).zfree).is_none() {
        return -(2 as i32);
    }
    state = (*strm).state as *mut inflate_state;
    if !((*state).window).is_null() {
        (Some(((*strm).zfree).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )((*strm).opaque, (*state).window as voidpf);
    }
    (Some(((*strm).zfree).expect("non-null function pointer")))
        .expect("non-null function pointer")((*strm).opaque, (*strm).state as voidpf);
    (*strm).state = 0 as *mut internal_state;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_inflateSetDictionary(
    mut strm: z_streamp,
    mut dictionary: *const Bytef,
    mut dictLength: uInt,
) -> i32 {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    let mut id: u64 = 0;
    if strm.is_null() || ((*strm).state).is_null() {
        return -(2 as i32);
    }
    state = (*strm).state as *mut inflate_state;
    if (*state).wrap != 0 as i32
        && (*state).mode as u32 != inflate_mode::DICT as i32 as u32
    {
        return -(2 as i32);
    }
    if (*state).mode as u32 == inflate_mode::DICT as i32 as u32 {
        id = _glp_zlib_adler32(0 as i64 as uLong, 0 as *const Bytef, 0 as i32 as uInt);
        id = _glp_zlib_adler32(id, dictionary, dictLength);
        if id != (*state).check {
            return -(3 as i32);
        }
    }
    if updatewindow(strm, (*strm).avail_out) != 0 {
        (*state).mode = inflate_mode::MEM;
        return -(4 as i32);
    }
    if dictLength > (*state).wsize {
        memcpy(
            (*state).window as *mut libc::c_void,
            dictionary.offset(dictLength as isize).offset(-((*state).wsize as isize))
                as *const libc::c_void,
            (*state).wsize as u64,
        );
        (*state).whave = (*state).wsize;
    } else {
        memcpy(
            ((*state).window)
                .offset((*state).wsize as isize)
                .offset(-(dictLength as isize)) as *mut libc::c_void,
            dictionary as *const libc::c_void,
            dictLength as u64,
        );
        (*state).whave = dictLength;
    }
    (*state).havedict = 1 as i32;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_inflateGetHeader(
    mut strm: z_streamp,
    mut head: gz_headerp,
) -> i32 {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if strm.is_null() || ((*strm).state).is_null() {
        return -(2 as i32);
    }
    state = (*strm).state as *mut inflate_state;
    if (*state).wrap & 2 as i32 == 0 as i32 {
        return -(2 as i32);
    }
    (*state).head = head;
    (*head).done = 0 as i32;
    return 0 as i32;
}
unsafe extern "C" fn syncsearch(
    mut have: *mut u32,
    mut buf: *mut u8,
    mut len: u32,
) -> u32 {
    let mut got: u32 = 0;
    let mut next: u32 = 0;
    got = *have;
    next = 0 as i32 as u32;
    while next < len && got < 4 as i32 as u32 {
        if *buf.offset(next as isize) as i32
            == (if got < 2 as i32 as u32 { 0 as i32 } else { 0xff as i32 })
        {
            got = got.wrapping_add(1);
            got;
        } else if *buf.offset(next as isize) != 0 {
            got = 0 as i32 as u32;
        } else {
            got = (4 as i32 as u32).wrapping_sub(got);
        }
        next = next.wrapping_add(1);
        next;
    }
    *have = got;
    return next;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_inflateSync(mut strm: z_streamp) -> i32 {
    let mut len: u32 = 0;
    let mut in_0: u64 = 0;
    let mut out: u64 = 0;
    let mut buf: [u8; 4] = [0; 4];
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if strm.is_null() || ((*strm).state).is_null() {
        return -(2 as i32);
    }
    state = (*strm).state as *mut inflate_state;
    if (*strm).avail_in == 0 as i32 as u32 && (*state).bits < 8 as i32 as u32 {
        return -(5 as i32);
    }
    if (*state).mode as u32 != inflate_mode::SYNC as i32 as u32 {
        (*state).mode = inflate_mode::SYNC;
        (*state).hold <<= (*state).bits & 7 as i32 as u32;
        (*state).bits = ((*state).bits).wrapping_sub((*state).bits & 7 as i32 as u32);
        len = 0 as i32 as u32;
        while (*state).bits >= 8 as i32 as u32 {
            let fresh36 = len;
            len = len.wrapping_add(1);
            buf[fresh36 as usize] = (*state).hold as u8;
            (*state).hold >>= 8 as i32;
            (*state).bits = ((*state).bits).wrapping_sub(8 as i32 as u32);
        }
        (*state).have = 0 as i32 as u32;
        syncsearch(&mut (*state).have, buf.as_mut_ptr(), len);
    }
    len = syncsearch(&mut (*state).have, (*strm).next_in, (*strm).avail_in);
    (*strm).avail_in = ((*strm).avail_in as u32).wrapping_sub(len) as uInt as uInt;
    (*strm).next_in = ((*strm).next_in).offset(len as isize);
    (*strm).total_in = ((*strm).total_in as u64).wrapping_add(len as u64) as uLong
        as uLong;
    if (*state).have != 4 as i32 as u32 {
        return -(3 as i32);
    }
    in_0 = (*strm).total_in;
    out = (*strm).total_out;
    _glp_zlib_inflateReset(strm);
    (*strm).total_in = in_0;
    (*strm).total_out = out;
    (*state).mode = inflate_mode::TYPE;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_inflateSyncPoint(mut strm: z_streamp) -> i32 {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if strm.is_null() || ((*strm).state).is_null() {
        return -(2 as i32);
    }
    state = (*strm).state as *mut inflate_state;
    return ((*state).mode as u32 == inflate_mode::STORED as i32 as u32
        && (*state).bits == 0 as i32 as u32) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_inflateCopy(
    mut dest: z_streamp,
    mut source: z_streamp,
) -> i32 {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    let mut copy: *mut inflate_state = 0 as *mut inflate_state;
    let mut window: *mut u8 = 0 as *mut u8;
    let mut wsize: u32 = 0;
    if dest.is_null() || source.is_null() || ((*source).state).is_null()
        || ((*source).zalloc).is_none() || ((*source).zfree).is_none()
    {
        return -(2 as i32);
    }
    state = (*source).state as *mut inflate_state;
    copy = (Some(((*source).zalloc).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*source).opaque,
        1 as i32 as uInt,
        ::core::mem::size_of::<inflate_state>() as u64 as uInt,
    ) as *mut inflate_state;
    if copy.is_null() {
        return -(4 as i32);
    }
    window = 0 as *mut u8;
    if !((*state).window).is_null() {
        window = (Some(((*source).zalloc).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*source).opaque,
            (1 as u32) << (*state).wbits,
            ::core::mem::size_of::<u8>() as u64 as uInt,
        ) as *mut u8;
        if window.is_null() {
            (Some(((*source).zfree).expect("non-null function pointer")))
                .expect("non-null function pointer")((*source).opaque, copy as voidpf);
            return -(4 as i32);
        }
    }
    memcpy(
        dest as *mut libc::c_void,
        source as *const libc::c_void,
        ::core::mem::size_of::<z_stream>() as u64,
    );
    memcpy(
        copy as *mut libc::c_void,
        state as *const libc::c_void,
        ::core::mem::size_of::<inflate_state>() as u64,
    );
    if (*state).lencode >= ((*state).codes).as_mut_ptr()
        && (*state).lencode
            <= ((*state).codes)
                .as_mut_ptr()
                .offset((852 as i32 + 592 as i32) as isize)
                .offset(-(1 as i32 as isize))
    {
        (*copy).lencode = ((*copy).codes)
            .as_mut_ptr()
            .offset(
                ((*state).lencode).offset_from(((*state).codes).as_mut_ptr()) as i64
                    as isize,
            );
        (*copy).distcode = ((*copy).codes)
            .as_mut_ptr()
            .offset(
                ((*state).distcode).offset_from(((*state).codes).as_mut_ptr()) as i64
                    as isize,
            );
    }
    (*copy).next = ((*copy).codes)
        .as_mut_ptr()
        .offset(
            ((*state).next).offset_from(((*state).codes).as_mut_ptr()) as i64 as isize,
        );
    if !window.is_null() {
        wsize = (1 as u32) << (*state).wbits;
        memcpy(
            window as *mut libc::c_void,
            (*state).window as *const libc::c_void,
            wsize as u64,
        );
    }
    (*copy).window = window;
    (*dest).state = copy as *mut internal_state;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_inflateUndermine(
    mut strm: z_streamp,
    mut subvert: i32,
) -> i32 {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if strm.is_null() || ((*strm).state).is_null() {
        return -(2 as i32);
    }
    state = (*strm).state as *mut inflate_state;
    (*state).sane = (subvert == 0) as i32;
    (*state).sane = 1 as i32;
    return -(3 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_inflateMark(mut strm: z_streamp) -> i64 {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if strm.is_null() || ((*strm).state).is_null() {
        return -(1 as i64) << 16 as i32;
    }
    state = (*strm).state as *mut inflate_state;
    return (((*state).back as i64) << 16 as i32)
        + (if (*state).mode as u32 == inflate_mode::COPY as i32 as u32 {
            (*state).length
        } else {
            (if (*state).mode as u32 == inflate_mode::MATCH as i32 as u32 {
                ((*state).was).wrapping_sub((*state).length)
            } else {
                0 as i32 as u32
            })
        }) as i64;
}