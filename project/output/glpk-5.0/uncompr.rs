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
    fn _glp_zlib_inflate(strm: z_streamp, flush: i32) -> i32;
    fn _glp_zlib_inflateEnd(strm: z_streamp) -> i32;
    fn _glp_zlib_inflateInit_(
        strm: z_streamp,
        version: *const i8,
        stream_size: i32,
    ) -> i32;
}
pub type Byte = u8;
pub type uInt = u32;
pub type uLong = u64;
pub type Bytef = Byte;
pub type uLongf = uLong;
pub type voidpf = *mut libc::c_void;
pub type alloc_func = Option<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct internal_state {
    pub dummy: i32,
}
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
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_uncompress(
    mut dest: *mut Bytef,
    mut destLen: *mut uLongf,
    mut source: *const Bytef,
    mut sourceLen: uLong,
) -> i32 {
    let mut stream: z_stream = z_stream {
        next_in: 0 as *mut Bytef,
        avail_in: 0,
        total_in: 0,
        next_out: 0 as *mut Bytef,
        avail_out: 0,
        total_out: 0,
        msg: 0 as *mut i8,
        state: 0 as *mut internal_state,
        zalloc: None,
        zfree: None,
        opaque: 0 as *mut libc::c_void,
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    let mut err: i32 = 0;
    stream.next_in = source as *mut Bytef;
    stream.avail_in = sourceLen as uInt;
    if stream.avail_in as uLong != sourceLen {
        return -(5 as i32);
    }
    stream.next_out = dest;
    stream.avail_out = *destLen as uInt;
    if stream.avail_out as uLong != *destLen {
        return -(5 as i32);
    }
    stream.zalloc = None;
    stream.zfree = None;
    err = _glp_zlib_inflateInit_(
        &mut stream,
        b"1.2.5\0" as *const u8 as *const i8,
        ::core::mem::size_of::<z_stream>() as u64 as i32,
    );
    if err != 0 as i32 {
        return err;
    }
    err = _glp_zlib_inflate(&mut stream, 4 as i32);
    if err != 1 as i32 {
        _glp_zlib_inflateEnd(&mut stream);
        if err == 2 as i32 || err == -(5 as i32) && stream.avail_in == 0 as i32 as u32 {
            return -(3 as i32);
        }
        return err;
    }
    *destLen = stream.total_out;
    err = _glp_zlib_inflateEnd(&mut stream);
    return err;
}