use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn _glp_zlib_tr_init(s: *mut deflate_state);
    fn _glp_zlib_zcfree(opaque: voidpf, ptr: voidpf);
    fn _glp_zlib_zcalloc(opaque: voidpf, items: u32, size: u32) -> voidpf;
    static _glp_zlib_z_errmsg: [*const i8; 10];
    fn _glp_zlib_adler32(adler: uLong, buf: *const Bytef, len: uInt) -> uLong;
    fn _glp_zlib_crc32(crc: uLong, buf: *const Bytef, len: uInt) -> uLong;
    static _glp_zlib_length_code: [uch; 0];
    static _glp_zlib_dist_code: [uch; 0];
    fn _glp_zlib_tr_flush_block(
        s: *mut deflate_state,
        buf: *mut charf,
        stored_len: ulg,
        last: i32,
    );
    fn _glp_zlib_tr_align(s: *mut deflate_state);
    fn _glp_zlib_tr_stored_block(
        s: *mut deflate_state,
        buf: *mut charf,
        stored_len: ulg,
        last: i32,
    );
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
}
pub type Byte = u8;
pub type uInt = u32;
pub type uLong = u64;
pub type Bytef = Byte;
pub type charf = i8;
pub type voidpf = *mut libc::c_void;
pub type alloc_func = Option<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct internal_state {
    pub strm: z_streamp,
    pub status: i32,
    pub pending_buf: *mut Bytef,
    pub pending_buf_size: ulg,
    pub pending_out: *mut Bytef,
    pub pending: uInt,
    pub wrap: i32,
    pub gzhead: gz_headerp,
    pub gzindex: uInt,
    pub method: Byte,
    pub last_flush: i32,
    pub w_size: uInt,
    pub w_bits: uInt,
    pub w_mask: uInt,
    pub window: *mut Bytef,
    pub window_size: ulg,
    pub prev: *mut Posf,
    pub head: *mut Posf,
    pub ins_h: uInt,
    pub hash_size: uInt,
    pub hash_bits: uInt,
    pub hash_mask: uInt,
    pub hash_shift: uInt,
    pub block_start: i64,
    pub match_length: uInt,
    pub prev_match: IPos,
    pub match_available: i32,
    pub strstart: uInt,
    pub match_start: uInt,
    pub lookahead: uInt,
    pub prev_length: uInt,
    pub max_chain_length: uInt,
    pub max_lazy_match: uInt,
    pub level: i32,
    pub strategy: i32,
    pub good_match: uInt,
    pub nice_match: i32,
    pub dyn_ltree: [ct_data_s; 573],
    pub dyn_dtree: [ct_data_s; 61],
    pub bl_tree: [ct_data_s; 39],
    pub l_desc: tree_desc_s,
    pub d_desc: tree_desc_s,
    pub bl_desc: tree_desc_s,
    pub bl_count: [ush; 16],
    pub heap: [i32; 573],
    pub heap_len: i32,
    pub heap_max: i32,
    pub depth: [uch; 573],
    pub l_buf: *mut uchf,
    pub lit_bufsize: uInt,
    pub last_lit: uInt,
    pub d_buf: *mut ushf,
    pub opt_len: ulg,
    pub static_len: ulg,
    pub matches: uInt,
    pub last_eob_len: i32,
    pub bi_buf: ush,
    pub bi_valid: i32,
    pub high_water: ulg,
}
pub type ulg = u64;
pub type ush = libc::c_ushort;
pub type ushf = ush;
pub type uchf = uch;
pub type uch = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tree_desc_s {
    pub dyn_tree: *mut ct_data,
    pub max_code: i32,
    pub stat_desc: *mut static_tree_desc,
}
pub type static_tree_desc = static_tree_desc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct static_tree_desc_s {
    pub dummy: i32,
}
pub type ct_data = ct_data_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ct_data_s {
    pub fc: C2RustUnnamed_0,
    pub dl: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub dad: ush,
    pub len: ush,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub freq: ush,
    pub code: ush,
}
pub type IPos = u32;
pub type Posf = Pos;
pub type Pos = ush;
pub type gz_headerp = *mut gz_header;
pub type gz_header = gz_header_s;
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
pub type z_streamp = *mut z_stream;
pub type z_stream = z_stream_s;
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
pub type deflate_state = internal_state;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum block_state {
    need_more,
    block_done,
    finish_started,
    finish_done,
}
impl block_state {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            block_state::need_more => 0,
            block_state::block_done => 1,
            block_state::finish_started => 2,
            block_state::finish_done => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> block_state {
        match value {
            0 => block_state::need_more,
            1 => block_state::block_done,
            2 => block_state::finish_started,
            3 => block_state::finish_done,
            _ => panic!("Invalid value for block_state: {}", value),
        }
    }
}
impl AddAssign<u32> for block_state {
    fn add_assign(&mut self, rhs: u32) {
        *self = block_state::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for block_state {
    fn sub_assign(&mut self, rhs: u32) {
        *self = block_state::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for block_state {
    fn mul_assign(&mut self, rhs: u32) {
        *self = block_state::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for block_state {
    fn div_assign(&mut self, rhs: u32) {
        *self = block_state::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for block_state {
    fn rem_assign(&mut self, rhs: u32) {
        *self = block_state::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for block_state {
    type Output = block_state;
    fn add(self, rhs: u32) -> block_state {
        block_state::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for block_state {
    type Output = block_state;
    fn sub(self, rhs: u32) -> block_state {
        block_state::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for block_state {
    type Output = block_state;
    fn mul(self, rhs: u32) -> block_state {
        block_state::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for block_state {
    type Output = block_state;
    fn div(self, rhs: u32) -> block_state {
        block_state::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for block_state {
    type Output = block_state;
    fn rem(self, rhs: u32) -> block_state {
        block_state::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type compress_func = Option<
    unsafe extern "C" fn(*mut deflate_state, i32) -> block_state,
>;
pub type config = config_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_s {
    pub good_length: ush,
    pub max_lazy: ush,
    pub nice_length: ush,
    pub max_chain: ush,
    pub func: compress_func,
}
#[no_mangle]
pub static mut _glp_zlib_deflate_copyright: [i8; 68] = unsafe {
    *::core::mem::transmute::<
        &[u8; 68],
        &[i8; 68],
    >(b" deflate 1.2.5 Copyright 1995-2010 Jean-loup Gailly and Mark Adler \0")
};
static mut configuration_table: [config; 10] = unsafe {
    [
        {
            let mut init = config_s {
                good_length: 0 as i32 as ush,
                max_lazy: 0 as i32 as ush,
                nice_length: 0 as i32 as ush,
                max_chain: 0 as i32 as ush,
                func: Some(
                    deflate_stored
                        as unsafe extern "C" fn(*mut deflate_state, i32) -> block_state,
                ),
            };
            init
        },
        {
            let mut init = config_s {
                good_length: 4 as i32 as ush,
                max_lazy: 4 as i32 as ush,
                nice_length: 8 as i32 as ush,
                max_chain: 4 as i32 as ush,
                func: Some(
                    deflate_fast
                        as unsafe extern "C" fn(*mut deflate_state, i32) -> block_state,
                ),
            };
            init
        },
        {
            let mut init = config_s {
                good_length: 4 as i32 as ush,
                max_lazy: 5 as i32 as ush,
                nice_length: 16 as i32 as ush,
                max_chain: 8 as i32 as ush,
                func: Some(
                    deflate_fast
                        as unsafe extern "C" fn(*mut deflate_state, i32) -> block_state,
                ),
            };
            init
        },
        {
            let mut init = config_s {
                good_length: 4 as i32 as ush,
                max_lazy: 6 as i32 as ush,
                nice_length: 32 as i32 as ush,
                max_chain: 32 as i32 as ush,
                func: Some(
                    deflate_fast
                        as unsafe extern "C" fn(*mut deflate_state, i32) -> block_state,
                ),
            };
            init
        },
        {
            let mut init = config_s {
                good_length: 4 as i32 as ush,
                max_lazy: 4 as i32 as ush,
                nice_length: 16 as i32 as ush,
                max_chain: 16 as i32 as ush,
                func: Some(
                    deflate_slow
                        as unsafe extern "C" fn(*mut deflate_state, i32) -> block_state,
                ),
            };
            init
        },
        {
            let mut init = config_s {
                good_length: 8 as i32 as ush,
                max_lazy: 16 as i32 as ush,
                nice_length: 32 as i32 as ush,
                max_chain: 32 as i32 as ush,
                func: Some(
                    deflate_slow
                        as unsafe extern "C" fn(*mut deflate_state, i32) -> block_state,
                ),
            };
            init
        },
        {
            let mut init = config_s {
                good_length: 8 as i32 as ush,
                max_lazy: 16 as i32 as ush,
                nice_length: 128 as i32 as ush,
                max_chain: 128 as i32 as ush,
                func: Some(
                    deflate_slow
                        as unsafe extern "C" fn(*mut deflate_state, i32) -> block_state,
                ),
            };
            init
        },
        {
            let mut init = config_s {
                good_length: 8 as i32 as ush,
                max_lazy: 32 as i32 as ush,
                nice_length: 128 as i32 as ush,
                max_chain: 256 as i32 as ush,
                func: Some(
                    deflate_slow
                        as unsafe extern "C" fn(*mut deflate_state, i32) -> block_state,
                ),
            };
            init
        },
        {
            let mut init = config_s {
                good_length: 32 as i32 as ush,
                max_lazy: 128 as i32 as ush,
                nice_length: 258 as i32 as ush,
                max_chain: 1024 as i32 as ush,
                func: Some(
                    deflate_slow
                        as unsafe extern "C" fn(*mut deflate_state, i32) -> block_state,
                ),
            };
            init
        },
        {
            let mut init = config_s {
                good_length: 32 as i32 as ush,
                max_lazy: 258 as i32 as ush,
                nice_length: 258 as i32 as ush,
                max_chain: 4096 as i32 as ush,
                func: Some(
                    deflate_slow
                        as unsafe extern "C" fn(*mut deflate_state, i32) -> block_state,
                ),
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_deflateInit_(
    mut strm: z_streamp,
    mut level: i32,
    mut version: *const i8,
    mut stream_size: i32,
) -> i32 {
    return _glp_zlib_deflateInit2_(
        strm,
        level,
        8 as i32,
        15 as i32,
        8 as i32,
        0 as i32,
        version,
        stream_size,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_deflateInit2_(
    mut strm: z_streamp,
    mut level: i32,
    mut method: i32,
    mut windowBits: i32,
    mut memLevel: i32,
    mut strategy: i32,
    mut version: *const i8,
    mut stream_size: i32,
) -> i32 {
    let mut s: *mut deflate_state = 0 as *mut deflate_state;
    let mut wrap: i32 = 1 as i32;
    static mut my_version: [i8; 6] = unsafe {
        *::core::mem::transmute::<&[u8; 6], &[i8; 6]>(b"1.2.5\0")
    };
    let mut overlay: *mut ushf = 0 as *mut ushf;
    if version.is_null()
        || *version.offset(0 as i32 as isize) as i32
            != my_version[0 as i32 as usize] as i32
        || stream_size as u64 != ::core::mem::size_of::<z_stream>() as u64
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
    if level == -(1 as i32) {
        level = 6 as i32;
    }
    if windowBits < 0 as i32 {
        wrap = 0 as i32;
        windowBits = -windowBits;
    } else if windowBits > 15 as i32 {
        wrap = 2 as i32;
        windowBits -= 16 as i32;
    }
    if memLevel < 1 as i32 || memLevel > 9 as i32 || method != 8 as i32
        || windowBits < 8 as i32 || windowBits > 15 as i32 || level < 0 as i32
        || level > 9 as i32 || strategy < 0 as i32 || strategy > 4 as i32
    {
        return -(2 as i32);
    }
    if windowBits == 8 as i32 {
        windowBits = 9 as i32;
    }
    s = (Some(((*strm).zalloc).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*strm).opaque,
        1 as i32 as uInt,
        ::core::mem::size_of::<deflate_state>() as u64 as uInt,
    ) as *mut deflate_state;
    if s.is_null() {
        return -(4 as i32);
    }
    (*strm).state = s as *mut internal_state;
    (*s).strm = strm;
    (*s).wrap = wrap;
    (*s).gzhead = 0 as gz_headerp;
    (*s).w_bits = windowBits as uInt;
    (*s).w_size = ((1 as i32) << (*s).w_bits) as uInt;
    (*s).w_mask = ((*s).w_size).wrapping_sub(1 as i32 as u32);
    (*s).hash_bits = (memLevel + 7 as i32) as uInt;
    (*s).hash_size = ((1 as i32) << (*s).hash_bits) as uInt;
    (*s).hash_mask = ((*s).hash_size).wrapping_sub(1 as i32 as u32);
    (*s).hash_shift = ((*s).hash_bits)
        .wrapping_add(3 as i32 as u32)
        .wrapping_sub(1 as i32 as u32)
        .wrapping_div(3 as i32 as u32);
    (*s).window = (Some(((*strm).zalloc).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*strm).opaque,
        (*s).w_size,
        (2 as i32 as u64).wrapping_mul(::core::mem::size_of::<Byte>() as u64) as uInt,
    ) as *mut Bytef;
    (*s).prev = (Some(((*strm).zalloc).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((*strm).opaque, (*s).w_size, ::core::mem::size_of::<Pos>() as u64 as uInt)
        as *mut Posf;
    (*s).head = (Some(((*strm).zalloc).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((*strm).opaque, (*s).hash_size, ::core::mem::size_of::<Pos>() as u64 as uInt)
        as *mut Posf;
    (*s).high_water = 0 as i32 as ulg;
    (*s).lit_bufsize = ((1 as i32) << memLevel + 6 as i32) as uInt;
    overlay = (Some(((*strm).zalloc).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*strm).opaque,
        (*s).lit_bufsize,
        (::core::mem::size_of::<ush>() as u64).wrapping_add(2 as i32 as u64) as uInt,
    ) as *mut ushf;
    (*s).pending_buf = overlay as *mut uchf;
    (*s).pending_buf_size = ((*s).lit_bufsize as ulg)
        .wrapping_mul(
            (::core::mem::size_of::<ush>() as u64).wrapping_add(2 as i64 as u64),
        );
    if ((*s).window).is_null() || ((*s).prev).is_null() || ((*s).head).is_null()
        || ((*s).pending_buf).is_null()
    {
        (*s).status = 666 as i32;
        (*strm).msg = _glp_zlib_z_errmsg[(2 as i32 - -(4 as i32)) as usize] as *mut i8;
        _glp_zlib_deflateEnd(strm);
        return -(4 as i32);
    }
    (*s).d_buf = overlay
        .offset(
            ((*s).lit_bufsize as u64).wrapping_div(::core::mem::size_of::<ush>() as u64)
                as isize,
        );
    (*s).l_buf = ((*s).pending_buf)
        .offset(
            (1 as i32 as u64)
                .wrapping_add(::core::mem::size_of::<ush>() as u64)
                .wrapping_mul((*s).lit_bufsize as u64) as isize,
        );
    (*s).level = level;
    (*s).strategy = strategy;
    (*s).method = method as Byte;
    return _glp_zlib_deflateReset(strm);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_deflateSetDictionary(
    mut strm: z_streamp,
    mut dictionary: *const Bytef,
    mut dictLength: uInt,
) -> i32 {
    let mut s: *mut deflate_state = 0 as *mut deflate_state;
    let mut length: uInt = dictLength;
    let mut n: uInt = 0;
    let mut hash_head: IPos = 0 as i32 as IPos;
    if strm.is_null() || ((*strm).state).is_null() || dictionary.is_null()
        || (*(*strm).state).wrap == 2 as i32
        || (*(*strm).state).wrap == 1 as i32 && (*(*strm).state).status != 42 as i32
    {
        return -(2 as i32);
    }
    s = (*strm).state;
    if (*s).wrap != 0 {
        (*strm).adler = _glp_zlib_adler32((*strm).adler, dictionary, dictLength);
    }
    if length < 3 as i32 as u32 {
        return 0 as i32;
    }
    if length > (*s).w_size {
        length = (*s).w_size;
        dictionary = dictionary.offset(dictLength.wrapping_sub(length) as isize);
    }
    memcpy(
        (*s).window as *mut libc::c_void,
        dictionary as *const libc::c_void,
        length as u64,
    );
    (*s).strstart = length;
    (*s).block_start = length as i64;
    (*s).ins_h = *((*s).window).offset(0 as i32 as isize) as uInt;
    (*s).ins_h = ((*s).ins_h << (*s).hash_shift
        ^ *((*s).window).offset(1 as i32 as isize) as u32) & (*s).hash_mask;
    n = 0 as i32 as uInt;
    while n <= length.wrapping_sub(3 as i32 as u32) {
        (*s).ins_h = ((*s).ins_h << (*s).hash_shift
            ^ *((*s).window)
                .offset(n.wrapping_add((3 as i32 - 1 as i32) as u32) as isize) as u32)
            & (*s).hash_mask;
        let ref mut fresh0 = *((*s).prev).offset((n & (*s).w_mask) as isize);
        *fresh0 = *((*s).head).offset((*s).ins_h as isize);
        hash_head = *fresh0 as IPos;
        *((*s).head).offset((*s).ins_h as isize) = n as Pos;
        n = n.wrapping_add(1);
        n;
    }
    if hash_head != 0 {
        hash_head = 0 as i32 as IPos;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_deflateReset(mut strm: z_streamp) -> i32 {
    let mut s: *mut deflate_state = 0 as *mut deflate_state;
    if strm.is_null() || ((*strm).state).is_null() || ((*strm).zalloc).is_none()
        || ((*strm).zfree).is_none()
    {
        return -(2 as i32);
    }
    (*strm).total_out = 0 as i32 as uLong;
    (*strm).total_in = (*strm).total_out;
    (*strm).msg = 0 as *mut i8;
    (*strm).data_type = 2 as i32;
    s = (*strm).state as *mut deflate_state;
    (*s).pending = 0 as i32 as uInt;
    (*s).pending_out = (*s).pending_buf;
    if (*s).wrap < 0 as i32 {
        (*s).wrap = -(*s).wrap;
    }
    (*s).status = if (*s).wrap != 0 { 42 as i32 } else { 113 as i32 };
    (*strm).adler = if (*s).wrap == 2 as i32 {
        _glp_zlib_crc32(0 as i64 as uLong, 0 as *const Bytef, 0 as i32 as uInt)
    } else {
        _glp_zlib_adler32(0 as i64 as uLong, 0 as *const Bytef, 0 as i32 as uInt)
    };
    (*s).last_flush = 0 as i32;
    _glp_zlib_tr_init(s);
    lm_init(s);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_deflateSetHeader(
    mut strm: z_streamp,
    mut head: gz_headerp,
) -> i32 {
    if strm.is_null() || ((*strm).state).is_null() {
        return -(2 as i32);
    }
    if (*(*strm).state).wrap != 2 as i32 {
        return -(2 as i32);
    }
    (*(*strm).state).gzhead = head;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_deflatePrime(
    mut strm: z_streamp,
    mut bits: i32,
    mut value: i32,
) -> i32 {
    if strm.is_null() || ((*strm).state).is_null() {
        return -(2 as i32);
    }
    (*(*strm).state).bi_valid = bits;
    (*(*strm).state).bi_buf = (value & ((1 as i32) << bits) - 1 as i32) as ush;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_deflateParams(
    mut strm: z_streamp,
    mut level: i32,
    mut strategy: i32,
) -> i32 {
    let mut s: *mut deflate_state = 0 as *mut deflate_state;
    let mut func: compress_func = None;
    let mut err: i32 = 0 as i32;
    if strm.is_null() || ((*strm).state).is_null() {
        return -(2 as i32);
    }
    s = (*strm).state;
    if level == -(1 as i32) {
        level = 6 as i32;
    }
    if level < 0 as i32 || level > 9 as i32 || strategy < 0 as i32 || strategy > 4 as i32
    {
        return -(2 as i32);
    }
    func = configuration_table[(*s).level as usize].func;
    if (strategy != (*s).strategy || func != configuration_table[level as usize].func)
        && (*strm).total_in != 0 as i32 as u64
    {
        err = _glp_zlib_deflate(strm, 5 as i32);
    }
    if (*s).level != level {
        (*s).level = level;
        (*s).max_lazy_match = configuration_table[level as usize].max_lazy as uInt;
        (*s).good_match = configuration_table[level as usize].good_length as uInt;
        (*s).nice_match = configuration_table[level as usize].nice_length as i32;
        (*s).max_chain_length = configuration_table[level as usize].max_chain as uInt;
    }
    (*s).strategy = strategy;
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_deflateTune(
    mut strm: z_streamp,
    mut good_length: i32,
    mut max_lazy: i32,
    mut nice_length: i32,
    mut max_chain: i32,
) -> i32 {
    let mut s: *mut deflate_state = 0 as *mut deflate_state;
    if strm.is_null() || ((*strm).state).is_null() {
        return -(2 as i32);
    }
    s = (*strm).state;
    (*s).good_match = good_length as uInt;
    (*s).max_lazy_match = max_lazy as uInt;
    (*s).nice_match = nice_length;
    (*s).max_chain_length = max_chain as uInt;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_deflateBound(
    mut strm: z_streamp,
    mut sourceLen: uLong,
) -> uLong {
    let mut s: *mut deflate_state = 0 as *mut deflate_state;
    let mut complen: uLong = 0;
    let mut wraplen: uLong = 0;
    let mut str: *mut Bytef = 0 as *mut Bytef;
    complen = sourceLen
        .wrapping_add(sourceLen.wrapping_add(7 as i32 as u64) >> 3 as i32)
        .wrapping_add(sourceLen.wrapping_add(63 as i32 as u64) >> 6 as i32)
        .wrapping_add(5 as i32 as u64);
    if strm.is_null() || ((*strm).state).is_null() {
        return complen.wrapping_add(6 as i32 as u64);
    }
    s = (*strm).state;
    match (*s).wrap {
        0 => {
            wraplen = 0 as i32 as uLong;
        }
        1 => {
            wraplen = (6 as i32 + (if (*s).strstart != 0 { 4 as i32 } else { 0 as i32 }))
                as uLong;
        }
        2 => {
            wraplen = 18 as i32 as uLong;
            if !((*s).gzhead).is_null() {
                if !((*(*s).gzhead).extra).is_null() {
                    wraplen = (wraplen as u64)
                        .wrapping_add(
                            (2 as i32 as u32).wrapping_add((*(*s).gzhead).extra_len)
                                as u64,
                        ) as uLong as uLong;
                }
                str = (*(*s).gzhead).name;
                if !str.is_null() {
                    loop {
                        wraplen = wraplen.wrapping_add(1);
                        wraplen;
                        let fresh1 = str;
                        str = str.offset(1);
                        if !(*fresh1 != 0) {
                            break;
                        }
                    }
                }
                str = (*(*s).gzhead).comment;
                if !str.is_null() {
                    loop {
                        wraplen = wraplen.wrapping_add(1);
                        wraplen;
                        let fresh2 = str;
                        str = str.offset(1);
                        if !(*fresh2 != 0) {
                            break;
                        }
                    }
                }
                if (*(*s).gzhead).hcrc != 0 {
                    wraplen = (wraplen as u64).wrapping_add(2 as i32 as u64) as uLong
                        as uLong;
                }
            }
        }
        _ => {
            wraplen = 6 as i32 as uLong;
        }
    }
    if (*s).w_bits != 15 as i32 as u32 || (*s).hash_bits != (8 as i32 + 7 as i32) as u32
    {
        return complen.wrapping_add(wraplen);
    }
    return sourceLen
        .wrapping_add(sourceLen >> 12 as i32)
        .wrapping_add(sourceLen >> 14 as i32)
        .wrapping_add(sourceLen >> 25 as i32)
        .wrapping_add(13 as i32 as u64)
        .wrapping_sub(6 as i32 as u64)
        .wrapping_add(wraplen);
}
unsafe extern "C" fn putShortMSB(mut s: *mut deflate_state, mut b: uInt) {
    let fresh3 = (*s).pending;
    (*s).pending = ((*s).pending).wrapping_add(1);
    *((*s).pending_buf).offset(fresh3 as isize) = (b >> 8 as i32) as Byte;
    let fresh4 = (*s).pending;
    (*s).pending = ((*s).pending).wrapping_add(1);
    *((*s).pending_buf).offset(fresh4 as isize) = (b & 0xff as i32 as u32) as Byte;
}
unsafe extern "C" fn flush_pending(mut strm: z_streamp) {
    let mut len: u32 = (*(*strm).state).pending;
    if len > (*strm).avail_out {
        len = (*strm).avail_out;
    }
    if len == 0 as i32 as u32 {
        return;
    }
    memcpy(
        (*strm).next_out as *mut libc::c_void,
        (*(*strm).state).pending_out as *const libc::c_void,
        len as u64,
    );
    (*strm).next_out = ((*strm).next_out).offset(len as isize);
    (*(*strm).state).pending_out = ((*(*strm).state).pending_out).offset(len as isize);
    (*strm).total_out = ((*strm).total_out as u64).wrapping_add(len as u64) as uLong
        as uLong;
    (*strm).avail_out = ((*strm).avail_out as u32).wrapping_sub(len) as uInt as uInt;
    (*(*strm).state).pending = ((*(*strm).state).pending as u32).wrapping_sub(len)
        as uInt as uInt;
    if (*(*strm).state).pending == 0 as i32 as u32 {
        (*(*strm).state).pending_out = (*(*strm).state).pending_buf;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_deflate(mut strm: z_streamp, mut flush: i32) -> i32 {
    let mut old_flush: i32 = 0;
    let mut s: *mut deflate_state = 0 as *mut deflate_state;
    if strm.is_null() || ((*strm).state).is_null() || flush > 5 as i32
        || flush < 0 as i32
    {
        return -(2 as i32);
    }
    s = (*strm).state;
    if ((*strm).next_out).is_null()
        || ((*strm).next_in).is_null() && (*strm).avail_in != 0 as i32 as u32
        || (*s).status == 666 as i32 && flush != 4 as i32
    {
        (*strm).msg = _glp_zlib_z_errmsg[(2 as i32 - -(2 as i32)) as usize] as *mut i8;
        return -(2 as i32);
    }
    if (*strm).avail_out == 0 as i32 as u32 {
        (*strm).msg = _glp_zlib_z_errmsg[(2 as i32 - -(5 as i32)) as usize] as *mut i8;
        return -(5 as i32);
    }
    (*s).strm = strm;
    old_flush = (*s).last_flush;
    (*s).last_flush = flush;
    if (*s).status == 42 as i32 {
        if (*s).wrap == 2 as i32 {
            (*strm).adler = _glp_zlib_crc32(
                0 as i64 as uLong,
                0 as *const Bytef,
                0 as i32 as uInt,
            );
            let fresh5 = (*s).pending;
            (*s).pending = ((*s).pending).wrapping_add(1);
            *((*s).pending_buf).offset(fresh5 as isize) = 31 as i32 as Bytef;
            let fresh6 = (*s).pending;
            (*s).pending = ((*s).pending).wrapping_add(1);
            *((*s).pending_buf).offset(fresh6 as isize) = 139 as i32 as Bytef;
            let fresh7 = (*s).pending;
            (*s).pending = ((*s).pending).wrapping_add(1);
            *((*s).pending_buf).offset(fresh7 as isize) = 8 as i32 as Bytef;
            if ((*s).gzhead).is_null() {
                let fresh8 = (*s).pending;
                (*s).pending = ((*s).pending).wrapping_add(1);
                *((*s).pending_buf).offset(fresh8 as isize) = 0 as i32 as Bytef;
                let fresh9 = (*s).pending;
                (*s).pending = ((*s).pending).wrapping_add(1);
                *((*s).pending_buf).offset(fresh9 as isize) = 0 as i32 as Bytef;
                let fresh10 = (*s).pending;
                (*s).pending = ((*s).pending).wrapping_add(1);
                *((*s).pending_buf).offset(fresh10 as isize) = 0 as i32 as Bytef;
                let fresh11 = (*s).pending;
                (*s).pending = ((*s).pending).wrapping_add(1);
                *((*s).pending_buf).offset(fresh11 as isize) = 0 as i32 as Bytef;
                let fresh12 = (*s).pending;
                (*s).pending = ((*s).pending).wrapping_add(1);
                *((*s).pending_buf).offset(fresh12 as isize) = 0 as i32 as Bytef;
                let fresh13 = (*s).pending;
                (*s).pending = ((*s).pending).wrapping_add(1);
                *((*s).pending_buf).offset(fresh13 as isize) = (if (*s).level == 9 as i32
                {
                    2 as i32
                } else if (*s).strategy >= 2 as i32 || (*s).level < 2 as i32 {
                    4 as i32
                } else {
                    0 as i32
                }) as Bytef;
                let fresh14 = (*s).pending;
                (*s).pending = ((*s).pending).wrapping_add(1);
                *((*s).pending_buf).offset(fresh14 as isize) = 0x3 as i32 as Bytef;
                (*s).status = 113 as i32;
            } else {
                let fresh15 = (*s).pending;
                (*s).pending = ((*s).pending).wrapping_add(1);
                *((*s).pending_buf).offset(fresh15 as isize) = ((if (*(*s).gzhead).text
                    != 0
                {
                    1 as i32
                } else {
                    0 as i32
                }) + (if (*(*s).gzhead).hcrc != 0 { 2 as i32 } else { 0 as i32 })
                    + (if ((*(*s).gzhead).extra).is_null() {
                        0 as i32
                    } else {
                        4 as i32
                    })
                    + (if ((*(*s).gzhead).name).is_null() { 0 as i32 } else { 8 as i32 })
                    + (if ((*(*s).gzhead).comment).is_null() {
                        0 as i32
                    } else {
                        16 as i32
                    })) as Bytef;
                let fresh16 = (*s).pending;
                (*s).pending = ((*s).pending).wrapping_add(1);
                *((*s).pending_buf).offset(fresh16 as isize) = ((*(*s).gzhead).time
                    & 0xff as i32 as u64) as Byte;
                let fresh17 = (*s).pending;
                (*s).pending = ((*s).pending).wrapping_add(1);
                *((*s).pending_buf).offset(fresh17 as isize) = ((*(*s).gzhead).time
                    >> 8 as i32 & 0xff as i32 as u64) as Byte;
                let fresh18 = (*s).pending;
                (*s).pending = ((*s).pending).wrapping_add(1);
                *((*s).pending_buf).offset(fresh18 as isize) = ((*(*s).gzhead).time
                    >> 16 as i32 & 0xff as i32 as u64) as Byte;
                let fresh19 = (*s).pending;
                (*s).pending = ((*s).pending).wrapping_add(1);
                *((*s).pending_buf).offset(fresh19 as isize) = ((*(*s).gzhead).time
                    >> 24 as i32 & 0xff as i32 as u64) as Byte;
                let fresh20 = (*s).pending;
                (*s).pending = ((*s).pending).wrapping_add(1);
                *((*s).pending_buf).offset(fresh20 as isize) = (if (*s).level == 9 as i32
                {
                    2 as i32
                } else if (*s).strategy >= 2 as i32 || (*s).level < 2 as i32 {
                    4 as i32
                } else {
                    0 as i32
                }) as Bytef;
                let fresh21 = (*s).pending;
                (*s).pending = ((*s).pending).wrapping_add(1);
                *((*s).pending_buf).offset(fresh21 as isize) = ((*(*s).gzhead).os
                    & 0xff as i32) as Bytef;
                if !((*(*s).gzhead).extra).is_null() {
                    let fresh22 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf).offset(fresh22 as isize) = ((*(*s).gzhead)
                        .extra_len & 0xff as i32 as u32) as Bytef;
                    let fresh23 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf).offset(fresh23 as isize) = ((*(*s).gzhead)
                        .extra_len >> 8 as i32 & 0xff as i32 as u32) as Bytef;
                }
                if (*(*s).gzhead).hcrc != 0 {
                    (*strm).adler = _glp_zlib_crc32(
                        (*strm).adler,
                        (*s).pending_buf,
                        (*s).pending,
                    );
                }
                (*s).gzindex = 0 as i32 as uInt;
                (*s).status = 69 as i32;
            }
        } else {
            let mut header: uInt = (8 as i32 as u32)
                .wrapping_add(((*s).w_bits).wrapping_sub(8 as i32 as u32) << 4 as i32)
                << 8 as i32;
            let mut level_flags: uInt = 0;
            if (*s).strategy >= 2 as i32 || (*s).level < 2 as i32 {
                level_flags = 0 as i32 as uInt;
            } else if (*s).level < 6 as i32 {
                level_flags = 1 as i32 as uInt;
            } else if (*s).level == 6 as i32 {
                level_flags = 2 as i32 as uInt;
            } else {
                level_flags = 3 as i32 as uInt;
            }
            header |= level_flags << 6 as i32;
            if (*s).strstart != 0 as i32 as u32 {
                header |= 0x20 as i32 as u32;
            }
            header = (header as u32)
                .wrapping_add(
                    (31 as i32 as u32)
                        .wrapping_sub(header.wrapping_rem(31 as i32 as u32)),
                ) as uInt as uInt;
            (*s).status = 113 as i32;
            putShortMSB(s, header);
            if (*s).strstart != 0 as i32 as u32 {
                putShortMSB(s, ((*strm).adler >> 16 as i32) as uInt);
                putShortMSB(s, ((*strm).adler & 0xffff as i32 as u64) as uInt);
            }
            (*strm).adler = _glp_zlib_adler32(
                0 as i64 as uLong,
                0 as *const Bytef,
                0 as i32 as uInt,
            );
        }
    }
    if (*s).status == 69 as i32 {
        if !((*(*s).gzhead).extra).is_null() {
            let mut beg: uInt = (*s).pending;
            while (*s).gzindex < (*(*s).gzhead).extra_len & 0xffff as i32 as u32 {
                if (*s).pending as u64 == (*s).pending_buf_size {
                    if (*(*s).gzhead).hcrc != 0 && (*s).pending > beg {
                        (*strm).adler = _glp_zlib_crc32(
                            (*strm).adler,
                            ((*s).pending_buf).offset(beg as isize),
                            ((*s).pending).wrapping_sub(beg),
                        );
                    }
                    flush_pending(strm);
                    beg = (*s).pending;
                    if (*s).pending as u64 == (*s).pending_buf_size {
                        break;
                    }
                }
                let fresh24 = (*s).pending;
                (*s).pending = ((*s).pending).wrapping_add(1);
                *((*s).pending_buf).offset(fresh24 as isize) = *((*(*s).gzhead).extra)
                    .offset((*s).gzindex as isize);
                (*s).gzindex = ((*s).gzindex).wrapping_add(1);
                (*s).gzindex;
            }
            if (*(*s).gzhead).hcrc != 0 && (*s).pending > beg {
                (*strm).adler = _glp_zlib_crc32(
                    (*strm).adler,
                    ((*s).pending_buf).offset(beg as isize),
                    ((*s).pending).wrapping_sub(beg),
                );
            }
            if (*s).gzindex == (*(*s).gzhead).extra_len {
                (*s).gzindex = 0 as i32 as uInt;
                (*s).status = 73 as i32;
            }
        } else {
            (*s).status = 73 as i32;
        }
    }
    if (*s).status == 73 as i32 {
        if !((*(*s).gzhead).name).is_null() {
            let mut beg_0: uInt = (*s).pending;
            let mut val: i32 = 0;
            loop {
                if (*s).pending as u64 == (*s).pending_buf_size {
                    if (*(*s).gzhead).hcrc != 0 && (*s).pending > beg_0 {
                        (*strm).adler = _glp_zlib_crc32(
                            (*strm).adler,
                            ((*s).pending_buf).offset(beg_0 as isize),
                            ((*s).pending).wrapping_sub(beg_0),
                        );
                    }
                    flush_pending(strm);
                    beg_0 = (*s).pending;
                    if (*s).pending as u64 == (*s).pending_buf_size {
                        val = 1 as i32;
                        break;
                    }
                }
                let fresh25 = (*s).gzindex;
                (*s).gzindex = ((*s).gzindex).wrapping_add(1);
                val = *((*(*s).gzhead).name).offset(fresh25 as isize) as i32;
                let fresh26 = (*s).pending;
                (*s).pending = ((*s).pending).wrapping_add(1);
                *((*s).pending_buf).offset(fresh26 as isize) = val as Bytef;
                if !(val != 0 as i32) {
                    break;
                }
            }
            if (*(*s).gzhead).hcrc != 0 && (*s).pending > beg_0 {
                (*strm).adler = _glp_zlib_crc32(
                    (*strm).adler,
                    ((*s).pending_buf).offset(beg_0 as isize),
                    ((*s).pending).wrapping_sub(beg_0),
                );
            }
            if val == 0 as i32 {
                (*s).gzindex = 0 as i32 as uInt;
                (*s).status = 91 as i32;
            }
        } else {
            (*s).status = 91 as i32;
        }
    }
    if (*s).status == 91 as i32 {
        if !((*(*s).gzhead).comment).is_null() {
            let mut beg_1: uInt = (*s).pending;
            let mut val_0: i32 = 0;
            loop {
                if (*s).pending as u64 == (*s).pending_buf_size {
                    if (*(*s).gzhead).hcrc != 0 && (*s).pending > beg_1 {
                        (*strm).adler = _glp_zlib_crc32(
                            (*strm).adler,
                            ((*s).pending_buf).offset(beg_1 as isize),
                            ((*s).pending).wrapping_sub(beg_1),
                        );
                    }
                    flush_pending(strm);
                    beg_1 = (*s).pending;
                    if (*s).pending as u64 == (*s).pending_buf_size {
                        val_0 = 1 as i32;
                        break;
                    }
                }
                let fresh27 = (*s).gzindex;
                (*s).gzindex = ((*s).gzindex).wrapping_add(1);
                val_0 = *((*(*s).gzhead).comment).offset(fresh27 as isize) as i32;
                let fresh28 = (*s).pending;
                (*s).pending = ((*s).pending).wrapping_add(1);
                *((*s).pending_buf).offset(fresh28 as isize) = val_0 as Bytef;
                if !(val_0 != 0 as i32) {
                    break;
                }
            }
            if (*(*s).gzhead).hcrc != 0 && (*s).pending > beg_1 {
                (*strm).adler = _glp_zlib_crc32(
                    (*strm).adler,
                    ((*s).pending_buf).offset(beg_1 as isize),
                    ((*s).pending).wrapping_sub(beg_1),
                );
            }
            if val_0 == 0 as i32 {
                (*s).status = 103 as i32;
            }
        } else {
            (*s).status = 103 as i32;
        }
    }
    if (*s).status == 103 as i32 {
        if (*(*s).gzhead).hcrc != 0 {
            if ((*s).pending).wrapping_add(2 as i32 as u32) as u64
                > (*s).pending_buf_size
            {
                flush_pending(strm);
            }
            if ((*s).pending).wrapping_add(2 as i32 as u32) as u64
                <= (*s).pending_buf_size
            {
                let fresh29 = (*s).pending;
                (*s).pending = ((*s).pending).wrapping_add(1);
                *((*s).pending_buf).offset(fresh29 as isize) = ((*strm).adler
                    & 0xff as i32 as u64) as Byte;
                let fresh30 = (*s).pending;
                (*s).pending = ((*s).pending).wrapping_add(1);
                *((*s).pending_buf).offset(fresh30 as isize) = ((*strm).adler >> 8 as i32
                    & 0xff as i32 as u64) as Byte;
                (*strm).adler = _glp_zlib_crc32(
                    0 as i64 as uLong,
                    0 as *const Bytef,
                    0 as i32 as uInt,
                );
                (*s).status = 113 as i32;
            }
        } else {
            (*s).status = 113 as i32;
        }
    }
    if (*s).pending != 0 as i32 as u32 {
        flush_pending(strm);
        if (*strm).avail_out == 0 as i32 as u32 {
            (*s).last_flush = -(1 as i32);
            return 0 as i32;
        }
    } else if (*strm).avail_in == 0 as i32 as u32 && flush <= old_flush
        && flush != 4 as i32
    {
        (*strm).msg = _glp_zlib_z_errmsg[(2 as i32 - -(5 as i32)) as usize] as *mut i8;
        return -(5 as i32);
    }
    if (*s).status == 666 as i32 && (*strm).avail_in != 0 as i32 as u32 {
        (*strm).msg = _glp_zlib_z_errmsg[(2 as i32 - -(5 as i32)) as usize] as *mut i8;
        return -(5 as i32);
    }
    if (*strm).avail_in != 0 as i32 as u32 || (*s).lookahead != 0 as i32 as u32
        || flush != 0 as i32 && (*s).status != 666 as i32
    {
        let mut bstate: block_state = block_state::need_more;
        bstate = block_state::from_libc_c_uint(
            (if (*s).strategy == 2 as i32 {
                deflate_huff(s, flush) as u32
            } else if (*s).strategy == 3 as i32 {
                deflate_rle(s, flush) as u32
            } else {
                (Some(
                    ((*configuration_table.as_ptr().offset((*s).level as isize)).func)
                        .expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(s, flush) as u32
            }) as u32,
        );
        if bstate as u32 == block_state::finish_started as i32 as u32
            || bstate as u32 == block_state::finish_done as i32 as u32
        {
            (*s).status = 666 as i32;
        }
        if bstate as u32 == block_state::need_more as i32 as u32
            || bstate as u32 == block_state::finish_started as i32 as u32
        {
            if (*strm).avail_out == 0 as i32 as u32 {
                (*s).last_flush = -(1 as i32);
            }
            return 0 as i32;
        }
        if bstate as u32 == block_state::block_done as i32 as u32 {
            if flush == 1 as i32 {
                _glp_zlib_tr_align(s);
            } else if flush != 5 as i32 {
                _glp_zlib_tr_stored_block(s, 0 as *mut i8, 0 as i64 as ulg, 0 as i32);
                if flush == 3 as i32 {
                    *((*s).head)
                        .offset(
                            ((*s).hash_size).wrapping_sub(1 as i32 as u32) as isize,
                        ) = 0 as i32 as Posf;
                    memset(
                        (*s).head as *mut Bytef as *mut libc::c_void,
                        0 as i32,
                        (((*s).hash_size).wrapping_sub(1 as i32 as u32) as u64)
                            .wrapping_mul(::core::mem::size_of::<Posf>() as u64),
                    );
                    if (*s).lookahead == 0 as i32 as u32 {
                        (*s).strstart = 0 as i32 as uInt;
                        (*s).block_start = 0 as i64;
                    }
                }
            }
            flush_pending(strm);
            if (*strm).avail_out == 0 as i32 as u32 {
                (*s).last_flush = -(1 as i32);
                return 0 as i32;
            }
        }
    }
    if flush != 4 as i32 {
        return 0 as i32;
    }
    if (*s).wrap <= 0 as i32 {
        return 1 as i32;
    }
    if (*s).wrap == 2 as i32 {
        let fresh31 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh31 as isize) = ((*strm).adler
            & 0xff as i32 as u64) as Byte;
        let fresh32 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh32 as isize) = ((*strm).adler >> 8 as i32
            & 0xff as i32 as u64) as Byte;
        let fresh33 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh33 as isize) = ((*strm).adler >> 16 as i32
            & 0xff as i32 as u64) as Byte;
        let fresh34 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh34 as isize) = ((*strm).adler >> 24 as i32
            & 0xff as i32 as u64) as Byte;
        let fresh35 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh35 as isize) = ((*strm).total_in
            & 0xff as i32 as u64) as Byte;
        let fresh36 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh36 as isize) = ((*strm).total_in >> 8 as i32
            & 0xff as i32 as u64) as Byte;
        let fresh37 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh37 as isize) = ((*strm).total_in >> 16 as i32
            & 0xff as i32 as u64) as Byte;
        let fresh38 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh38 as isize) = ((*strm).total_in >> 24 as i32
            & 0xff as i32 as u64) as Byte;
    } else {
        putShortMSB(s, ((*strm).adler >> 16 as i32) as uInt);
        putShortMSB(s, ((*strm).adler & 0xffff as i32 as u64) as uInt);
    }
    flush_pending(strm);
    if (*s).wrap > 0 as i32 {
        (*s).wrap = -(*s).wrap;
    }
    return if (*s).pending != 0 as i32 as u32 { 0 as i32 } else { 1 as i32 };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_deflateEnd(mut strm: z_streamp) -> i32 {
    let mut status: i32 = 0;
    if strm.is_null() || ((*strm).state).is_null() {
        return -(2 as i32);
    }
    status = (*(*strm).state).status;
    if status != 42 as i32 && status != 69 as i32 && status != 73 as i32
        && status != 91 as i32 && status != 103 as i32 && status != 113 as i32
        && status != 666 as i32
    {
        return -(2 as i32);
    }
    if !((*(*strm).state).pending_buf).is_null() {
        (Some(((*strm).zfree).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )((*strm).opaque, (*(*strm).state).pending_buf as voidpf);
    }
    if !((*(*strm).state).head).is_null() {
        (Some(((*strm).zfree).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )((*strm).opaque, (*(*strm).state).head as voidpf);
    }
    if !((*(*strm).state).prev).is_null() {
        (Some(((*strm).zfree).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )((*strm).opaque, (*(*strm).state).prev as voidpf);
    }
    if !((*(*strm).state).window).is_null() {
        (Some(((*strm).zfree).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )((*strm).opaque, (*(*strm).state).window as voidpf);
    }
    (Some(((*strm).zfree).expect("non-null function pointer")))
        .expect("non-null function pointer")((*strm).opaque, (*strm).state as voidpf);
    (*strm).state = 0 as *mut internal_state;
    return if status == 113 as i32 { -(3 as i32) } else { 0 as i32 };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_deflateCopy(
    mut dest: z_streamp,
    mut source: z_streamp,
) -> i32 {
    let mut ds: *mut deflate_state = 0 as *mut deflate_state;
    let mut ss: *mut deflate_state = 0 as *mut deflate_state;
    let mut overlay: *mut ushf = 0 as *mut ushf;
    if source.is_null() || dest.is_null() || ((*source).state).is_null() {
        return -(2 as i32);
    }
    ss = (*source).state;
    memcpy(
        dest as *mut libc::c_void,
        source as *const libc::c_void,
        ::core::mem::size_of::<z_stream>() as u64,
    );
    ds = (Some(((*dest).zalloc).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*dest).opaque,
        1 as i32 as uInt,
        ::core::mem::size_of::<deflate_state>() as u64 as uInt,
    ) as *mut deflate_state;
    if ds.is_null() {
        return -(4 as i32);
    }
    (*dest).state = ds as *mut internal_state;
    memcpy(
        ds as *mut libc::c_void,
        ss as *const libc::c_void,
        ::core::mem::size_of::<deflate_state>() as u64,
    );
    (*ds).strm = dest;
    (*ds).window = (Some(((*dest).zalloc).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*dest).opaque,
        (*ds).w_size,
        (2 as i32 as u64).wrapping_mul(::core::mem::size_of::<Byte>() as u64) as uInt,
    ) as *mut Bytef;
    (*ds).prev = (Some(((*dest).zalloc).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((*dest).opaque, (*ds).w_size, ::core::mem::size_of::<Pos>() as u64 as uInt)
        as *mut Posf;
    (*ds).head = (Some(((*dest).zalloc).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((*dest).opaque, (*ds).hash_size, ::core::mem::size_of::<Pos>() as u64 as uInt)
        as *mut Posf;
    overlay = (Some(((*dest).zalloc).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*dest).opaque,
        (*ds).lit_bufsize,
        (::core::mem::size_of::<ush>() as u64).wrapping_add(2 as i32 as u64) as uInt,
    ) as *mut ushf;
    (*ds).pending_buf = overlay as *mut uchf;
    if ((*ds).window).is_null() || ((*ds).prev).is_null() || ((*ds).head).is_null()
        || ((*ds).pending_buf).is_null()
    {
        _glp_zlib_deflateEnd(dest);
        return -(4 as i32);
    }
    memcpy(
        (*ds).window as *mut libc::c_void,
        (*ss).window as *const libc::c_void,
        (((*ds).w_size).wrapping_mul(2 as i32 as u32) as u64)
            .wrapping_mul(::core::mem::size_of::<Byte>() as u64),
    );
    memcpy(
        (*ds).prev as *mut libc::c_void,
        (*ss).prev as *const libc::c_void,
        ((*ds).w_size as u64).wrapping_mul(::core::mem::size_of::<Pos>() as u64),
    );
    memcpy(
        (*ds).head as *mut libc::c_void,
        (*ss).head as *const libc::c_void,
        ((*ds).hash_size as u64).wrapping_mul(::core::mem::size_of::<Pos>() as u64),
    );
    memcpy(
        (*ds).pending_buf as *mut libc::c_void,
        (*ss).pending_buf as *const libc::c_void,
        (*ds).pending_buf_size as uInt as u64,
    );
    (*ds).pending_out = ((*ds).pending_buf)
        .offset(((*ss).pending_out).offset_from((*ss).pending_buf) as i64 as isize);
    (*ds).d_buf = overlay
        .offset(
            ((*ds).lit_bufsize as u64).wrapping_div(::core::mem::size_of::<ush>() as u64)
                as isize,
        );
    (*ds).l_buf = ((*ds).pending_buf)
        .offset(
            (1 as i32 as u64)
                .wrapping_add(::core::mem::size_of::<ush>() as u64)
                .wrapping_mul((*ds).lit_bufsize as u64) as isize,
        );
    (*ds).l_desc.dyn_tree = ((*ds).dyn_ltree).as_mut_ptr();
    (*ds).d_desc.dyn_tree = ((*ds).dyn_dtree).as_mut_ptr();
    (*ds).bl_desc.dyn_tree = ((*ds).bl_tree).as_mut_ptr();
    return 0 as i32;
}
unsafe extern "C" fn read_buf(
    mut strm: z_streamp,
    mut buf: *mut Bytef,
    mut size: u32,
) -> i32 {
    let mut len: u32 = (*strm).avail_in;
    if len > size {
        len = size;
    }
    if len == 0 as i32 as u32 {
        return 0 as i32;
    }
    (*strm).avail_in = ((*strm).avail_in as u32).wrapping_sub(len) as uInt as uInt;
    if (*(*strm).state).wrap == 1 as i32 {
        (*strm).adler = _glp_zlib_adler32((*strm).adler, (*strm).next_in, len);
    } else if (*(*strm).state).wrap == 2 as i32 {
        (*strm).adler = _glp_zlib_crc32((*strm).adler, (*strm).next_in, len);
    }
    memcpy(buf as *mut libc::c_void, (*strm).next_in as *const libc::c_void, len as u64);
    (*strm).next_in = ((*strm).next_in).offset(len as isize);
    (*strm).total_in = ((*strm).total_in as u64).wrapping_add(len as u64) as uLong
        as uLong;
    return len as i32;
}
unsafe extern "C" fn lm_init(mut s: *mut deflate_state) {
    (*s).window_size = (2 as i64 as ulg).wrapping_mul((*s).w_size as u64);
    *((*s).head).offset(((*s).hash_size).wrapping_sub(1 as i32 as u32) as isize) = 0
        as i32 as Posf;
    memset(
        (*s).head as *mut Bytef as *mut libc::c_void,
        0 as i32,
        (((*s).hash_size).wrapping_sub(1 as i32 as u32) as u64)
            .wrapping_mul(::core::mem::size_of::<Posf>() as u64),
    );
    (*s).max_lazy_match = configuration_table[(*s).level as usize].max_lazy as uInt;
    (*s).good_match = configuration_table[(*s).level as usize].good_length as uInt;
    (*s).nice_match = configuration_table[(*s).level as usize].nice_length as i32;
    (*s).max_chain_length = configuration_table[(*s).level as usize].max_chain as uInt;
    (*s).strstart = 0 as i32 as uInt;
    (*s).block_start = 0 as i64;
    (*s).lookahead = 0 as i32 as uInt;
    (*s).prev_length = (3 as i32 - 1 as i32) as uInt;
    (*s).match_length = (*s).prev_length;
    (*s).match_available = 0 as i32;
    (*s).ins_h = 0 as i32 as uInt;
}
unsafe extern "C" fn longest_match(
    mut s: *mut deflate_state,
    mut cur_match: IPos,
) -> uInt {
    let mut chain_length: u32 = (*s).max_chain_length;
    let mut scan: *mut Bytef = ((*s).window).offset((*s).strstart as isize);
    let mut match_0: *mut Bytef = 0 as *mut Bytef;
    let mut len: i32 = 0;
    let mut best_len: i32 = (*s).prev_length as i32;
    let mut nice_match: i32 = (*s).nice_match;
    let mut limit: IPos = if (*s).strstart
        > ((*s).w_size).wrapping_sub((258 as i32 + 3 as i32 + 1 as i32) as u32)
    {
        ((*s).strstart)
            .wrapping_sub(
                ((*s).w_size).wrapping_sub((258 as i32 + 3 as i32 + 1 as i32) as u32),
            )
    } else {
        0 as i32 as u32
    };
    let mut prev: *mut Posf = (*s).prev;
    let mut wmask: uInt = (*s).w_mask;
    let mut strend: *mut Bytef = ((*s).window)
        .offset((*s).strstart as isize)
        .offset(258 as i32 as isize);
    let mut scan_end1: Byte = *scan.offset((best_len - 1 as i32) as isize);
    let mut scan_end: Byte = *scan.offset(best_len as isize);
    if (*s).prev_length >= (*s).good_match {
        chain_length >>= 2 as i32;
    }
    if nice_match as uInt > (*s).lookahead {
        nice_match = (*s).lookahead as i32;
    }
    loop {
        match_0 = ((*s).window).offset(cur_match as isize);
        if !(*match_0.offset(best_len as isize) as i32 != scan_end as i32
            || *match_0.offset((best_len - 1 as i32) as isize) as i32 != scan_end1 as i32
            || *match_0 as i32 != *scan as i32
            || {
                match_0 = match_0.offset(1);
                *match_0 as i32 != *scan.offset(1 as i32 as isize) as i32
            })
        {
            scan = scan.offset(2 as i32 as isize);
            match_0 = match_0.offset(1);
            match_0;
            loop {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                if !(*scan as i32 == *match_0 as i32
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        *scan as i32 == *match_0 as i32
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        *scan as i32 == *match_0 as i32
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        *scan as i32 == *match_0 as i32
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        *scan as i32 == *match_0 as i32
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        *scan as i32 == *match_0 as i32
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        *scan as i32 == *match_0 as i32
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        *scan as i32 == *match_0 as i32
                    } && scan < strend)
                {
                    break;
                }
            }
            len = 258 as i32 - strend.offset_from(scan) as i64 as i32;
            scan = strend.offset(-(258 as i32 as isize));
            if len > best_len {
                (*s).match_start = cur_match;
                best_len = len;
                if len >= nice_match {
                    break;
                }
                scan_end1 = *scan.offset((best_len - 1 as i32) as isize);
                scan_end = *scan.offset(best_len as isize);
            }
        }
        cur_match = *prev.offset((cur_match & wmask) as isize) as IPos;
        if !(cur_match > limit
            && {
                chain_length = chain_length.wrapping_sub(1);
                chain_length != 0 as i32 as u32
            })
        {
            break;
        }
    }
    if best_len as uInt <= (*s).lookahead {
        return best_len as uInt;
    }
    return (*s).lookahead;
}
unsafe extern "C" fn fill_window(mut s: *mut deflate_state) {
    let mut n: u32 = 0;
    let mut m: u32 = 0;
    let mut p: *mut Posf = 0 as *mut Posf;
    let mut more: u32 = 0;
    let mut wsize: uInt = (*s).w_size;
    loop {
        more = ((*s).window_size)
            .wrapping_sub((*s).lookahead as ulg)
            .wrapping_sub((*s).strstart as ulg) as u32;
        if ::core::mem::size_of::<i32>() as u64 <= 2 as i32 as u64 {
            if more == 0 as i32 as u32 && (*s).strstart == 0 as i32 as u32
                && (*s).lookahead == 0 as i32 as u32
            {
                more = wsize;
            } else if more == -(1 as i32) as u32 {
                more = more.wrapping_sub(1);
                more;
            }
        }
        if (*s).strstart
            >= wsize
                .wrapping_add(
                    ((*s).w_size).wrapping_sub((258 as i32 + 3 as i32 + 1 as i32) as u32),
                )
        {
            memcpy(
                (*s).window as *mut libc::c_void,
                ((*s).window).offset(wsize as isize) as *const libc::c_void,
                wsize as u64,
            );
            (*s).match_start = ((*s).match_start as u32).wrapping_sub(wsize) as uInt
                as uInt;
            (*s).strstart = ((*s).strstart as u32).wrapping_sub(wsize) as uInt as uInt;
            (*s).block_start -= wsize as i64;
            n = (*s).hash_size;
            p = &mut *((*s).head).offset(n as isize) as *mut Posf;
            loop {
                p = p.offset(-1);
                m = *p as u32;
                *p = (if m >= wsize { m.wrapping_sub(wsize) } else { 0 as i32 as u32 })
                    as Pos;
                n = n.wrapping_sub(1);
                if !(n != 0) {
                    break;
                }
            }
            n = wsize;
            p = &mut *((*s).prev).offset(n as isize) as *mut Posf;
            loop {
                p = p.offset(-1);
                m = *p as u32;
                *p = (if m >= wsize { m.wrapping_sub(wsize) } else { 0 as i32 as u32 })
                    as Pos;
                n = n.wrapping_sub(1);
                if !(n != 0) {
                    break;
                }
            }
            more = more.wrapping_add(wsize);
        }
        if (*(*s).strm).avail_in == 0 as i32 as u32 {
            return;
        }
        n = read_buf(
            (*s).strm,
            ((*s).window).offset((*s).strstart as isize).offset((*s).lookahead as isize),
            more,
        ) as u32;
        (*s).lookahead = ((*s).lookahead as u32).wrapping_add(n) as uInt as uInt;
        if (*s).lookahead >= 3 as i32 as u32 {
            (*s).ins_h = *((*s).window).offset((*s).strstart as isize) as uInt;
            (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                ^ *((*s).window)
                    .offset(((*s).strstart).wrapping_add(1 as i32 as u32) as isize)
                    as u32) & (*s).hash_mask;
        }
        if !((*s).lookahead < (258 as i32 + 3 as i32 + 1 as i32) as u32
            && (*(*s).strm).avail_in != 0 as i32 as u32)
        {
            break;
        }
    }
    if (*s).high_water < (*s).window_size {
        let mut curr: ulg = ((*s).strstart as u64).wrapping_add((*s).lookahead as ulg);
        let mut init: ulg = 0;
        if (*s).high_water < curr {
            init = ((*s).window_size).wrapping_sub(curr);
            if init > 258 as i32 as u64 {
                init = 258 as i32 as ulg;
            }
            memset(
                ((*s).window).offset(curr as isize) as *mut libc::c_void,
                0 as i32,
                init as u32 as u64,
            );
            (*s).high_water = curr.wrapping_add(init);
        } else if (*s).high_water < curr.wrapping_add(258 as i32 as u64) {
            init = curr.wrapping_add(258 as i32 as u64).wrapping_sub((*s).high_water);
            if init > ((*s).window_size).wrapping_sub((*s).high_water) {
                init = ((*s).window_size).wrapping_sub((*s).high_water);
            }
            memset(
                ((*s).window).offset((*s).high_water as isize) as *mut libc::c_void,
                0 as i32,
                init as u32 as u64,
            );
            (*s).high_water = ((*s).high_water as u64).wrapping_add(init) as ulg as ulg;
        }
    }
}
unsafe extern "C" fn deflate_stored(
    mut s: *mut deflate_state,
    mut flush: i32,
) -> block_state {
    let mut max_block_size: ulg = 0xffff as i32 as ulg;
    let mut max_start: ulg = 0;
    if max_block_size > ((*s).pending_buf_size).wrapping_sub(5 as i32 as u64) {
        max_block_size = ((*s).pending_buf_size).wrapping_sub(5 as i32 as u64);
    }
    loop {
        if (*s).lookahead <= 1 as i32 as u32 {
            fill_window(s);
            if (*s).lookahead == 0 as i32 as u32 && flush == 0 as i32 {
                return block_state::need_more;
            }
            if (*s).lookahead == 0 as i32 as u32 {
                break;
            }
        }
        (*s).strstart = ((*s).strstart as u32).wrapping_add((*s).lookahead) as uInt
            as uInt;
        (*s).lookahead = 0 as i32 as uInt;
        max_start = ((*s).block_start as u64).wrapping_add(max_block_size);
        if (*s).strstart == 0 as i32 as u32 || (*s).strstart as ulg >= max_start {
            (*s).lookahead = ((*s).strstart as u64).wrapping_sub(max_start) as uInt;
            (*s).strstart = max_start as uInt;
            _glp_zlib_tr_flush_block(
                s,
                if (*s).block_start >= 0 as i64 {
                    &mut *((*s).window).offset((*s).block_start as u32 as isize)
                        as *mut Bytef as *mut charf
                } else {
                    0 as *mut charf
                },
                ((*s).strstart as i64 - (*s).block_start) as ulg,
                0 as i32,
            );
            (*s).block_start = (*s).strstart as i64;
            flush_pending((*s).strm);
            if (*(*s).strm).avail_out == 0 as i32 as u32 {
                return block_state::from_libc_c_uint(
                    (if 0 as i32 != 0 {
                        block_state::finish_started as i32
                    } else {
                        block_state::need_more as i32
                    }) as u32,
                );
            }
        }
        if ((*s).strstart).wrapping_sub((*s).block_start as uInt)
            >= ((*s).w_size).wrapping_sub((258 as i32 + 3 as i32 + 1 as i32) as u32)
        {
            _glp_zlib_tr_flush_block(
                s,
                if (*s).block_start >= 0 as i64 {
                    &mut *((*s).window).offset((*s).block_start as u32 as isize)
                        as *mut Bytef as *mut charf
                } else {
                    0 as *mut charf
                },
                ((*s).strstart as i64 - (*s).block_start) as ulg,
                0 as i32,
            );
            (*s).block_start = (*s).strstart as i64;
            flush_pending((*s).strm);
            if (*(*s).strm).avail_out == 0 as i32 as u32 {
                return block_state::from_libc_c_uint(
                    (if 0 as i32 != 0 {
                        block_state::finish_started as i32
                    } else {
                        block_state::need_more as i32
                    }) as u32,
                );
            }
        }
    }
    _glp_zlib_tr_flush_block(
        s,
        if (*s).block_start >= 0 as i64 {
            &mut *((*s).window).offset((*s).block_start as u32 as isize) as *mut Bytef
                as *mut charf
        } else {
            0 as *mut charf
        },
        ((*s).strstart as i64 - (*s).block_start) as ulg,
        (flush == 4 as i32) as i32,
    );
    (*s).block_start = (*s).strstart as i64;
    flush_pending((*s).strm);
    if (*(*s).strm).avail_out == 0 as i32 as u32 {
        return block_state::from_libc_c_uint(
            (if flush == 4 as i32 {
                block_state::finish_started as i32
            } else {
                block_state::need_more as i32
            }) as u32,
        );
    }
    return block_state::from_libc_c_uint(
        (if flush == 4 as i32 {
            block_state::finish_done as i32
        } else {
            block_state::block_done as i32
        }) as u32,
    );
}
unsafe extern "C" fn deflate_fast(
    mut s: *mut deflate_state,
    mut flush: i32,
) -> block_state {
    let mut hash_head: IPos = 0;
    let mut bflush: i32 = 0;
    loop {
        if (*s).lookahead < (258 as i32 + 3 as i32 + 1 as i32) as u32 {
            fill_window(s);
            if (*s).lookahead < (258 as i32 + 3 as i32 + 1 as i32) as u32
                && flush == 0 as i32
            {
                return block_state::need_more;
            }
            if (*s).lookahead == 0 as i32 as u32 {
                break;
            }
        }
        hash_head = 0 as i32 as IPos;
        if (*s).lookahead >= 3 as i32 as u32 {
            (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                ^ *((*s).window)
                    .offset(
                        ((*s).strstart).wrapping_add((3 as i32 - 1 as i32) as u32)
                            as isize,
                    ) as u32) & (*s).hash_mask;
            let ref mut fresh39 = *((*s).prev)
                .offset(((*s).strstart & (*s).w_mask) as isize);
            *fresh39 = *((*s).head).offset((*s).ins_h as isize);
            hash_head = *fresh39 as IPos;
            *((*s).head).offset((*s).ins_h as isize) = (*s).strstart as Pos;
        }
        if hash_head != 0 as i32 as u32
            && ((*s).strstart).wrapping_sub(hash_head)
                <= ((*s).w_size).wrapping_sub((258 as i32 + 3 as i32 + 1 as i32) as u32)
        {
            (*s).match_length = longest_match(s, hash_head);
        }
        if (*s).match_length >= 3 as i32 as u32 {
            let mut len: uch = ((*s).match_length).wrapping_sub(3 as i32 as u32) as uch;
            let mut dist: ush = ((*s).strstart).wrapping_sub((*s).match_start) as ush;
            *((*s).d_buf).offset((*s).last_lit as isize) = dist;
            let fresh40 = (*s).last_lit;
            (*s).last_lit = ((*s).last_lit).wrapping_add(1);
            *((*s).l_buf).offset(fresh40 as isize) = len;
            dist = dist.wrapping_sub(1);
            dist;
            (*s)
                .dyn_ltree[(*_glp_zlib_length_code.as_ptr().offset(len as isize) as i32
                    + 256 as i32 + 1 as i32) as usize]
                .fc
                .freq = ((*s)
                .dyn_ltree[(*_glp_zlib_length_code.as_ptr().offset(len as isize) as i32
                    + 256 as i32 + 1 as i32) as usize]
                .fc
                .freq)
                .wrapping_add(1);
            (*s)
                .dyn_ltree[(*_glp_zlib_length_code.as_ptr().offset(len as isize) as i32
                    + 256 as i32 + 1 as i32) as usize]
                .fc
                .freq;
            (*s)
                .dyn_dtree[(if (dist as i32) < 256 as i32 {
                    *_glp_zlib_dist_code.as_ptr().offset(dist as isize) as i32
                } else {
                    *_glp_zlib_dist_code
                        .as_ptr()
                        .offset((256 as i32 + (dist as i32 >> 7 as i32)) as isize) as i32
                }) as usize]
                .fc
                .freq = ((*s)
                .dyn_dtree[(if (dist as i32) < 256 as i32 {
                    *_glp_zlib_dist_code.as_ptr().offset(dist as isize) as i32
                } else {
                    *_glp_zlib_dist_code
                        .as_ptr()
                        .offset((256 as i32 + (dist as i32 >> 7 as i32)) as isize) as i32
                }) as usize]
                .fc
                .freq)
                .wrapping_add(1);
            (*s)
                .dyn_dtree[(if (dist as i32) < 256 as i32 {
                    *_glp_zlib_dist_code.as_ptr().offset(dist as isize) as i32
                } else {
                    *_glp_zlib_dist_code
                        .as_ptr()
                        .offset((256 as i32 + (dist as i32 >> 7 as i32)) as isize) as i32
                }) as usize]
                .fc
                .freq;
            bflush = ((*s).last_lit == ((*s).lit_bufsize).wrapping_sub(1 as i32 as u32))
                as i32;
            (*s).lookahead = ((*s).lookahead as u32).wrapping_sub((*s).match_length)
                as uInt as uInt;
            if (*s).match_length <= (*s).max_lazy_match
                && (*s).lookahead >= 3 as i32 as u32
            {
                (*s).match_length = ((*s).match_length).wrapping_sub(1);
                (*s).match_length;
                loop {
                    (*s).strstart = ((*s).strstart).wrapping_add(1);
                    (*s).strstart;
                    (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                        ^ *((*s).window)
                            .offset(
                                ((*s).strstart).wrapping_add((3 as i32 - 1 as i32) as u32)
                                    as isize,
                            ) as u32) & (*s).hash_mask;
                    let ref mut fresh41 = *((*s).prev)
                        .offset(((*s).strstart & (*s).w_mask) as isize);
                    *fresh41 = *((*s).head).offset((*s).ins_h as isize);
                    hash_head = *fresh41 as IPos;
                    *((*s).head).offset((*s).ins_h as isize) = (*s).strstart as Pos;
                    (*s).match_length = ((*s).match_length).wrapping_sub(1);
                    if !((*s).match_length != 0 as i32 as u32) {
                        break;
                    }
                }
                (*s).strstart = ((*s).strstart).wrapping_add(1);
                (*s).strstart;
            } else {
                (*s).strstart = ((*s).strstart as u32).wrapping_add((*s).match_length)
                    as uInt as uInt;
                (*s).match_length = 0 as i32 as uInt;
                (*s).ins_h = *((*s).window).offset((*s).strstart as isize) as uInt;
                (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                    ^ *((*s).window)
                        .offset(((*s).strstart).wrapping_add(1 as i32 as u32) as isize)
                        as u32) & (*s).hash_mask;
            }
        } else {
            let mut cc: uch = *((*s).window).offset((*s).strstart as isize);
            *((*s).d_buf).offset((*s).last_lit as isize) = 0 as i32 as ushf;
            let fresh42 = (*s).last_lit;
            (*s).last_lit = ((*s).last_lit).wrapping_add(1);
            *((*s).l_buf).offset(fresh42 as isize) = cc;
            (*s).dyn_ltree[cc as usize].fc.freq = ((*s).dyn_ltree[cc as usize].fc.freq)
                .wrapping_add(1);
            (*s).dyn_ltree[cc as usize].fc.freq;
            bflush = ((*s).last_lit == ((*s).lit_bufsize).wrapping_sub(1 as i32 as u32))
                as i32;
            (*s).lookahead = ((*s).lookahead).wrapping_sub(1);
            (*s).lookahead;
            (*s).strstart = ((*s).strstart).wrapping_add(1);
            (*s).strstart;
        }
        if bflush != 0 {
            _glp_zlib_tr_flush_block(
                s,
                if (*s).block_start >= 0 as i64 {
                    &mut *((*s).window).offset((*s).block_start as u32 as isize)
                        as *mut Bytef as *mut charf
                } else {
                    0 as *mut charf
                },
                ((*s).strstart as i64 - (*s).block_start) as ulg,
                0 as i32,
            );
            (*s).block_start = (*s).strstart as i64;
            flush_pending((*s).strm);
            if (*(*s).strm).avail_out == 0 as i32 as u32 {
                return block_state::from_libc_c_uint(
                    (if 0 as i32 != 0 {
                        block_state::finish_started as i32
                    } else {
                        block_state::need_more as i32
                    }) as u32,
                );
            }
        }
    }
    _glp_zlib_tr_flush_block(
        s,
        if (*s).block_start >= 0 as i64 {
            &mut *((*s).window).offset((*s).block_start as u32 as isize) as *mut Bytef
                as *mut charf
        } else {
            0 as *mut charf
        },
        ((*s).strstart as i64 - (*s).block_start) as ulg,
        (flush == 4 as i32) as i32,
    );
    (*s).block_start = (*s).strstart as i64;
    flush_pending((*s).strm);
    if (*(*s).strm).avail_out == 0 as i32 as u32 {
        return block_state::from_libc_c_uint(
            (if flush == 4 as i32 {
                block_state::finish_started as i32
            } else {
                block_state::need_more as i32
            }) as u32,
        );
    }
    return block_state::from_libc_c_uint(
        (if flush == 4 as i32 {
            block_state::finish_done as i32
        } else {
            block_state::block_done as i32
        }) as u32,
    );
}
unsafe extern "C" fn deflate_slow(
    mut s: *mut deflate_state,
    mut flush: i32,
) -> block_state {
    let mut hash_head: IPos = 0;
    let mut bflush: i32 = 0;
    loop {
        if (*s).lookahead < (258 as i32 + 3 as i32 + 1 as i32) as u32 {
            fill_window(s);
            if (*s).lookahead < (258 as i32 + 3 as i32 + 1 as i32) as u32
                && flush == 0 as i32
            {
                return block_state::need_more;
            }
            if (*s).lookahead == 0 as i32 as u32 {
                break;
            }
        }
        hash_head = 0 as i32 as IPos;
        if (*s).lookahead >= 3 as i32 as u32 {
            (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                ^ *((*s).window)
                    .offset(
                        ((*s).strstart).wrapping_add((3 as i32 - 1 as i32) as u32)
                            as isize,
                    ) as u32) & (*s).hash_mask;
            let ref mut fresh43 = *((*s).prev)
                .offset(((*s).strstart & (*s).w_mask) as isize);
            *fresh43 = *((*s).head).offset((*s).ins_h as isize);
            hash_head = *fresh43 as IPos;
            *((*s).head).offset((*s).ins_h as isize) = (*s).strstart as Pos;
        }
        (*s).prev_length = (*s).match_length;
        (*s).prev_match = (*s).match_start;
        (*s).match_length = (3 as i32 - 1 as i32) as uInt;
        if hash_head != 0 as i32 as u32 && (*s).prev_length < (*s).max_lazy_match
            && ((*s).strstart).wrapping_sub(hash_head)
                <= ((*s).w_size).wrapping_sub((258 as i32 + 3 as i32 + 1 as i32) as u32)
        {
            (*s).match_length = longest_match(s, hash_head);
            if (*s).match_length <= 5 as i32 as u32
                && ((*s).strategy == 1 as i32
                    || (*s).match_length == 3 as i32 as u32
                        && ((*s).strstart).wrapping_sub((*s).match_start)
                            > 4096 as i32 as u32)
            {
                (*s).match_length = (3 as i32 - 1 as i32) as uInt;
            }
        }
        if (*s).prev_length >= 3 as i32 as u32 && (*s).match_length <= (*s).prev_length {
            let mut max_insert: uInt = ((*s).strstart)
                .wrapping_add((*s).lookahead)
                .wrapping_sub(3 as i32 as u32);
            let mut len: uch = ((*s).prev_length).wrapping_sub(3 as i32 as u32) as uch;
            let mut dist: ush = ((*s).strstart)
                .wrapping_sub(1 as i32 as u32)
                .wrapping_sub((*s).prev_match) as ush;
            *((*s).d_buf).offset((*s).last_lit as isize) = dist;
            let fresh44 = (*s).last_lit;
            (*s).last_lit = ((*s).last_lit).wrapping_add(1);
            *((*s).l_buf).offset(fresh44 as isize) = len;
            dist = dist.wrapping_sub(1);
            dist;
            (*s)
                .dyn_ltree[(*_glp_zlib_length_code.as_ptr().offset(len as isize) as i32
                    + 256 as i32 + 1 as i32) as usize]
                .fc
                .freq = ((*s)
                .dyn_ltree[(*_glp_zlib_length_code.as_ptr().offset(len as isize) as i32
                    + 256 as i32 + 1 as i32) as usize]
                .fc
                .freq)
                .wrapping_add(1);
            (*s)
                .dyn_ltree[(*_glp_zlib_length_code.as_ptr().offset(len as isize) as i32
                    + 256 as i32 + 1 as i32) as usize]
                .fc
                .freq;
            (*s)
                .dyn_dtree[(if (dist as i32) < 256 as i32 {
                    *_glp_zlib_dist_code.as_ptr().offset(dist as isize) as i32
                } else {
                    *_glp_zlib_dist_code
                        .as_ptr()
                        .offset((256 as i32 + (dist as i32 >> 7 as i32)) as isize) as i32
                }) as usize]
                .fc
                .freq = ((*s)
                .dyn_dtree[(if (dist as i32) < 256 as i32 {
                    *_glp_zlib_dist_code.as_ptr().offset(dist as isize) as i32
                } else {
                    *_glp_zlib_dist_code
                        .as_ptr()
                        .offset((256 as i32 + (dist as i32 >> 7 as i32)) as isize) as i32
                }) as usize]
                .fc
                .freq)
                .wrapping_add(1);
            (*s)
                .dyn_dtree[(if (dist as i32) < 256 as i32 {
                    *_glp_zlib_dist_code.as_ptr().offset(dist as isize) as i32
                } else {
                    *_glp_zlib_dist_code
                        .as_ptr()
                        .offset((256 as i32 + (dist as i32 >> 7 as i32)) as isize) as i32
                }) as usize]
                .fc
                .freq;
            bflush = ((*s).last_lit == ((*s).lit_bufsize).wrapping_sub(1 as i32 as u32))
                as i32;
            (*s).lookahead = ((*s).lookahead as u32)
                .wrapping_sub(((*s).prev_length).wrapping_sub(1 as i32 as u32)) as uInt
                as uInt;
            (*s).prev_length = ((*s).prev_length as u32).wrapping_sub(2 as i32 as u32)
                as uInt as uInt;
            loop {
                (*s).strstart = ((*s).strstart).wrapping_add(1);
                if (*s).strstart <= max_insert {
                    (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                        ^ *((*s).window)
                            .offset(
                                ((*s).strstart).wrapping_add((3 as i32 - 1 as i32) as u32)
                                    as isize,
                            ) as u32) & (*s).hash_mask;
                    let ref mut fresh45 = *((*s).prev)
                        .offset(((*s).strstart & (*s).w_mask) as isize);
                    *fresh45 = *((*s).head).offset((*s).ins_h as isize);
                    hash_head = *fresh45 as IPos;
                    *((*s).head).offset((*s).ins_h as isize) = (*s).strstart as Pos;
                }
                (*s).prev_length = ((*s).prev_length).wrapping_sub(1);
                if !((*s).prev_length != 0 as i32 as u32) {
                    break;
                }
            }
            (*s).match_available = 0 as i32;
            (*s).match_length = (3 as i32 - 1 as i32) as uInt;
            (*s).strstart = ((*s).strstart).wrapping_add(1);
            (*s).strstart;
            if bflush != 0 {
                _glp_zlib_tr_flush_block(
                    s,
                    if (*s).block_start >= 0 as i64 {
                        &mut *((*s).window).offset((*s).block_start as u32 as isize)
                            as *mut Bytef as *mut charf
                    } else {
                        0 as *mut charf
                    },
                    ((*s).strstart as i64 - (*s).block_start) as ulg,
                    0 as i32,
                );
                (*s).block_start = (*s).strstart as i64;
                flush_pending((*s).strm);
                if (*(*s).strm).avail_out == 0 as i32 as u32 {
                    return block_state::from_libc_c_uint(
                        (if 0 as i32 != 0 {
                            block_state::finish_started as i32
                        } else {
                            block_state::need_more as i32
                        }) as u32,
                    );
                }
            }
        } else if (*s).match_available != 0 {
            let mut cc: uch = *((*s).window)
                .offset(((*s).strstart).wrapping_sub(1 as i32 as u32) as isize);
            *((*s).d_buf).offset((*s).last_lit as isize) = 0 as i32 as ushf;
            let fresh46 = (*s).last_lit;
            (*s).last_lit = ((*s).last_lit).wrapping_add(1);
            *((*s).l_buf).offset(fresh46 as isize) = cc;
            (*s).dyn_ltree[cc as usize].fc.freq = ((*s).dyn_ltree[cc as usize].fc.freq)
                .wrapping_add(1);
            (*s).dyn_ltree[cc as usize].fc.freq;
            bflush = ((*s).last_lit == ((*s).lit_bufsize).wrapping_sub(1 as i32 as u32))
                as i32;
            if bflush != 0 {
                _glp_zlib_tr_flush_block(
                    s,
                    if (*s).block_start >= 0 as i64 {
                        &mut *((*s).window).offset((*s).block_start as u32 as isize)
                            as *mut Bytef as *mut charf
                    } else {
                        0 as *mut charf
                    },
                    ((*s).strstart as i64 - (*s).block_start) as ulg,
                    0 as i32,
                );
                (*s).block_start = (*s).strstart as i64;
                flush_pending((*s).strm);
            }
            (*s).strstart = ((*s).strstart).wrapping_add(1);
            (*s).strstart;
            (*s).lookahead = ((*s).lookahead).wrapping_sub(1);
            (*s).lookahead;
            if (*(*s).strm).avail_out == 0 as i32 as u32 {
                return block_state::need_more;
            }
        } else {
            (*s).match_available = 1 as i32;
            (*s).strstart = ((*s).strstart).wrapping_add(1);
            (*s).strstart;
            (*s).lookahead = ((*s).lookahead).wrapping_sub(1);
            (*s).lookahead;
        }
    }
    if (*s).match_available != 0 {
        let mut cc_0: uch = *((*s).window)
            .offset(((*s).strstart).wrapping_sub(1 as i32 as u32) as isize);
        *((*s).d_buf).offset((*s).last_lit as isize) = 0 as i32 as ushf;
        let fresh47 = (*s).last_lit;
        (*s).last_lit = ((*s).last_lit).wrapping_add(1);
        *((*s).l_buf).offset(fresh47 as isize) = cc_0;
        (*s).dyn_ltree[cc_0 as usize].fc.freq = ((*s).dyn_ltree[cc_0 as usize].fc.freq)
            .wrapping_add(1);
        (*s).dyn_ltree[cc_0 as usize].fc.freq;
        bflush = ((*s).last_lit == ((*s).lit_bufsize).wrapping_sub(1 as i32 as u32))
            as i32;
        (*s).match_available = 0 as i32;
    }
    _glp_zlib_tr_flush_block(
        s,
        if (*s).block_start >= 0 as i64 {
            &mut *((*s).window).offset((*s).block_start as u32 as isize) as *mut Bytef
                as *mut charf
        } else {
            0 as *mut charf
        },
        ((*s).strstart as i64 - (*s).block_start) as ulg,
        (flush == 4 as i32) as i32,
    );
    (*s).block_start = (*s).strstart as i64;
    flush_pending((*s).strm);
    if (*(*s).strm).avail_out == 0 as i32 as u32 {
        return block_state::from_libc_c_uint(
            (if flush == 4 as i32 {
                block_state::finish_started as i32
            } else {
                block_state::need_more as i32
            }) as u32,
        );
    }
    return block_state::from_libc_c_uint(
        (if flush == 4 as i32 {
            block_state::finish_done as i32
        } else {
            block_state::block_done as i32
        }) as u32,
    );
}
unsafe extern "C" fn deflate_rle(
    mut s: *mut deflate_state,
    mut flush: i32,
) -> block_state {
    let mut bflush: i32 = 0;
    let mut prev: uInt = 0;
    let mut scan: *mut Bytef = 0 as *mut Bytef;
    let mut strend: *mut Bytef = 0 as *mut Bytef;
    loop {
        if (*s).lookahead < 258 as i32 as u32 {
            fill_window(s);
            if (*s).lookahead < 258 as i32 as u32 && flush == 0 as i32 {
                return block_state::need_more;
            }
            if (*s).lookahead == 0 as i32 as u32 {
                break;
            }
        }
        (*s).match_length = 0 as i32 as uInt;
        if (*s).lookahead >= 3 as i32 as u32 && (*s).strstart > 0 as i32 as u32 {
            scan = ((*s).window)
                .offset((*s).strstart as isize)
                .offset(-(1 as i32 as isize));
            prev = *scan as uInt;
            scan = scan.offset(1);
            if prev == *scan as u32
                && {
                    scan = scan.offset(1);
                    prev == *scan as u32
                }
                && {
                    scan = scan.offset(1);
                    prev == *scan as u32
                }
            {
                strend = ((*s).window)
                    .offset((*s).strstart as isize)
                    .offset(258 as i32 as isize);
                loop {
                    scan = scan.offset(1);
                    if !(prev == *scan as u32
                        && {
                            scan = scan.offset(1);
                            prev == *scan as u32
                        }
                        && {
                            scan = scan.offset(1);
                            prev == *scan as u32
                        }
                        && {
                            scan = scan.offset(1);
                            prev == *scan as u32
                        }
                        && {
                            scan = scan.offset(1);
                            prev == *scan as u32
                        }
                        && {
                            scan = scan.offset(1);
                            prev == *scan as u32
                        }
                        && {
                            scan = scan.offset(1);
                            prev == *scan as u32
                        }
                        && {
                            scan = scan.offset(1);
                            prev == *scan as u32
                        } && scan < strend)
                    {
                        break;
                    }
                }
                (*s).match_length = (258 as i32 - strend.offset_from(scan) as i64 as i32)
                    as uInt;
                if (*s).match_length > (*s).lookahead {
                    (*s).match_length = (*s).lookahead;
                }
            }
        }
        if (*s).match_length >= 3 as i32 as u32 {
            let mut len: uch = ((*s).match_length).wrapping_sub(3 as i32 as u32) as uch;
            let mut dist: ush = 1 as i32 as ush;
            *((*s).d_buf).offset((*s).last_lit as isize) = dist;
            let fresh48 = (*s).last_lit;
            (*s).last_lit = ((*s).last_lit).wrapping_add(1);
            *((*s).l_buf).offset(fresh48 as isize) = len;
            dist = dist.wrapping_sub(1);
            dist;
            (*s)
                .dyn_ltree[(*_glp_zlib_length_code.as_ptr().offset(len as isize) as i32
                    + 256 as i32 + 1 as i32) as usize]
                .fc
                .freq = ((*s)
                .dyn_ltree[(*_glp_zlib_length_code.as_ptr().offset(len as isize) as i32
                    + 256 as i32 + 1 as i32) as usize]
                .fc
                .freq)
                .wrapping_add(1);
            (*s)
                .dyn_ltree[(*_glp_zlib_length_code.as_ptr().offset(len as isize) as i32
                    + 256 as i32 + 1 as i32) as usize]
                .fc
                .freq;
            (*s)
                .dyn_dtree[(if (dist as i32) < 256 as i32 {
                    *_glp_zlib_dist_code.as_ptr().offset(dist as isize) as i32
                } else {
                    *_glp_zlib_dist_code
                        .as_ptr()
                        .offset((256 as i32 + (dist as i32 >> 7 as i32)) as isize) as i32
                }) as usize]
                .fc
                .freq = ((*s)
                .dyn_dtree[(if (dist as i32) < 256 as i32 {
                    *_glp_zlib_dist_code.as_ptr().offset(dist as isize) as i32
                } else {
                    *_glp_zlib_dist_code
                        .as_ptr()
                        .offset((256 as i32 + (dist as i32 >> 7 as i32)) as isize) as i32
                }) as usize]
                .fc
                .freq)
                .wrapping_add(1);
            (*s)
                .dyn_dtree[(if (dist as i32) < 256 as i32 {
                    *_glp_zlib_dist_code.as_ptr().offset(dist as isize) as i32
                } else {
                    *_glp_zlib_dist_code
                        .as_ptr()
                        .offset((256 as i32 + (dist as i32 >> 7 as i32)) as isize) as i32
                }) as usize]
                .fc
                .freq;
            bflush = ((*s).last_lit == ((*s).lit_bufsize).wrapping_sub(1 as i32 as u32))
                as i32;
            (*s).lookahead = ((*s).lookahead as u32).wrapping_sub((*s).match_length)
                as uInt as uInt;
            (*s).strstart = ((*s).strstart as u32).wrapping_add((*s).match_length)
                as uInt as uInt;
            (*s).match_length = 0 as i32 as uInt;
        } else {
            let mut cc: uch = *((*s).window).offset((*s).strstart as isize);
            *((*s).d_buf).offset((*s).last_lit as isize) = 0 as i32 as ushf;
            let fresh49 = (*s).last_lit;
            (*s).last_lit = ((*s).last_lit).wrapping_add(1);
            *((*s).l_buf).offset(fresh49 as isize) = cc;
            (*s).dyn_ltree[cc as usize].fc.freq = ((*s).dyn_ltree[cc as usize].fc.freq)
                .wrapping_add(1);
            (*s).dyn_ltree[cc as usize].fc.freq;
            bflush = ((*s).last_lit == ((*s).lit_bufsize).wrapping_sub(1 as i32 as u32))
                as i32;
            (*s).lookahead = ((*s).lookahead).wrapping_sub(1);
            (*s).lookahead;
            (*s).strstart = ((*s).strstart).wrapping_add(1);
            (*s).strstart;
        }
        if bflush != 0 {
            _glp_zlib_tr_flush_block(
                s,
                if (*s).block_start >= 0 as i64 {
                    &mut *((*s).window).offset((*s).block_start as u32 as isize)
                        as *mut Bytef as *mut charf
                } else {
                    0 as *mut charf
                },
                ((*s).strstart as i64 - (*s).block_start) as ulg,
                0 as i32,
            );
            (*s).block_start = (*s).strstart as i64;
            flush_pending((*s).strm);
            if (*(*s).strm).avail_out == 0 as i32 as u32 {
                return block_state::from_libc_c_uint(
                    (if 0 as i32 != 0 {
                        block_state::finish_started as i32
                    } else {
                        block_state::need_more as i32
                    }) as u32,
                );
            }
        }
    }
    _glp_zlib_tr_flush_block(
        s,
        if (*s).block_start >= 0 as i64 {
            &mut *((*s).window).offset((*s).block_start as u32 as isize) as *mut Bytef
                as *mut charf
        } else {
            0 as *mut charf
        },
        ((*s).strstart as i64 - (*s).block_start) as ulg,
        (flush == 4 as i32) as i32,
    );
    (*s).block_start = (*s).strstart as i64;
    flush_pending((*s).strm);
    if (*(*s).strm).avail_out == 0 as i32 as u32 {
        return block_state::from_libc_c_uint(
            (if flush == 4 as i32 {
                block_state::finish_started as i32
            } else {
                block_state::need_more as i32
            }) as u32,
        );
    }
    return block_state::from_libc_c_uint(
        (if flush == 4 as i32 {
            block_state::finish_done as i32
        } else {
            block_state::block_done as i32
        }) as u32,
    );
}
unsafe extern "C" fn deflate_huff(
    mut s: *mut deflate_state,
    mut flush: i32,
) -> block_state {
    let mut bflush: i32 = 0;
    loop {
        if (*s).lookahead == 0 as i32 as u32 {
            fill_window(s);
            if (*s).lookahead == 0 as i32 as u32 {
                if flush == 0 as i32 {
                    return block_state::need_more;
                }
                break;
            }
        }
        (*s).match_length = 0 as i32 as uInt;
        let mut cc: uch = *((*s).window).offset((*s).strstart as isize);
        *((*s).d_buf).offset((*s).last_lit as isize) = 0 as i32 as ushf;
        let fresh50 = (*s).last_lit;
        (*s).last_lit = ((*s).last_lit).wrapping_add(1);
        *((*s).l_buf).offset(fresh50 as isize) = cc;
        (*s).dyn_ltree[cc as usize].fc.freq = ((*s).dyn_ltree[cc as usize].fc.freq)
            .wrapping_add(1);
        (*s).dyn_ltree[cc as usize].fc.freq;
        bflush = ((*s).last_lit == ((*s).lit_bufsize).wrapping_sub(1 as i32 as u32))
            as i32;
        (*s).lookahead = ((*s).lookahead).wrapping_sub(1);
        (*s).lookahead;
        (*s).strstart = ((*s).strstart).wrapping_add(1);
        (*s).strstart;
        if bflush != 0 {
            _glp_zlib_tr_flush_block(
                s,
                if (*s).block_start >= 0 as i64 {
                    &mut *((*s).window).offset((*s).block_start as u32 as isize)
                        as *mut Bytef as *mut charf
                } else {
                    0 as *mut charf
                },
                ((*s).strstart as i64 - (*s).block_start) as ulg,
                0 as i32,
            );
            (*s).block_start = (*s).strstart as i64;
            flush_pending((*s).strm);
            if (*(*s).strm).avail_out == 0 as i32 as u32 {
                return block_state::from_libc_c_uint(
                    (if 0 as i32 != 0 {
                        block_state::finish_started as i32
                    } else {
                        block_state::need_more as i32
                    }) as u32,
                );
            }
        }
    }
    _glp_zlib_tr_flush_block(
        s,
        if (*s).block_start >= 0 as i64 {
            &mut *((*s).window).offset((*s).block_start as u32 as isize) as *mut Bytef
                as *mut charf
        } else {
            0 as *mut charf
        },
        ((*s).strstart as i64 - (*s).block_start) as ulg,
        (flush == 4 as i32) as i32,
    );
    (*s).block_start = (*s).strstart as i64;
    flush_pending((*s).strm);
    if (*(*s).strm).avail_out == 0 as i32 as u32 {
        return block_state::from_libc_c_uint(
            (if flush == 4 as i32 {
                block_state::finish_started as i32
            } else {
                block_state::need_more as i32
            }) as u32,
        );
    }
    return block_state::from_libc_c_uint(
        (if flush == 4 as i32 {
            block_state::finish_done as i32
        } else {
            block_state::block_done as i32
        }) as u32,
    );
}