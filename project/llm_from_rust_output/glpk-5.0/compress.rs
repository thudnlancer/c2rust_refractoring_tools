use std::mem;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_ulong, c_void};

type Byte = c_uchar;
type uInt = c_uint;
type uLong = c_ulong;
type Bytef = Byte;
type uLongf = uLong;
type voidpf = *mut c_void;

#[repr(C)]
struct InternalState {
    dummy: c_int,
}

#[repr(C)]
struct ZStream {
    next_in: *mut Bytef,
    avail_in: uInt,
    total_in: uLong,
    next_out: *mut Bytef,
    avail_out: uInt,
    total_out: uLong,
    msg: *mut c_char,
    state: *mut InternalState,
    zalloc: Option<extern "C" fn(voidpf, uInt, uInt) -> voidpf>,
    zfree: Option<extern "C" fn(voidpf, voidpf)>,
    opaque: voidpf,
    data_type: c_int,
    adler: uLong,
    reserved: uLong,
}

fn zlib_deflate_init(stream: &mut ZStream, level: c_int) -> c_int {
    unsafe {
        let version = CString::new("1.2.5").unwrap();
        _glp_zlib_deflateInit_(
            stream,
            level,
            version.as_ptr(),
            mem::size_of::<ZStream>() as c_int,
        )
    }
}

fn zlib_deflate(stream: &mut ZStream, flush: c_int) -> c_int {
    unsafe { _glp_zlib_deflate(stream, flush) }
}

fn zlib_deflate_end(stream: &mut ZStream) -> c_int {
    unsafe { _glp_zlib_deflateEnd(stream) }
}

pub fn compress2(
    dest: &mut [Bytef],
    source: &[Bytef],
    level: c_int,
) -> Result<uLongf, c_int> {
    let mut stream = ZStream {
        next_in: source.as_ptr() as *mut Bytef,
        avail_in: source.len() as uInt,
        next_out: dest.as_mut_ptr(),
        avail_out: dest.len() as uInt,
        total_in: 0,
        total_out: 0,
        msg: std::ptr::null_mut(),
        state: std::ptr::null_mut(),
        zalloc: None,
        zfree: None,
        opaque: std::ptr::null_mut(),
        data_type: 0,
        adler: 0,
        reserved: 0,
    };

    if stream.avail_out as uLong != dest.len() as uLong {
        return Err(-5);
    }

    let err = zlib_deflate_init(&mut stream, level);
    if err != 0 {
        return Err(err);
    }

    let err = zlib_deflate(&mut stream, 4);
    if err != 1 {
        zlib_deflate_end(&mut stream);
        return if err == 0 { Err(-5) } else { Err(err) };
    }

    let compressed_size = stream.total_out;
    let err = zlib_deflate_end(&mut stream);
    if err != 0 {
        return Err(err);
    }

    Ok(compressed_size)
}

pub fn compress(dest: &mut [Bytef], source: &[Bytef]) -> Result<uLongf, c_int> {
    compress2(dest, source, -1)
}

pub fn compress_bound(source_len: uLong) -> uLong {
    source_len
        .wrapping_add(source_len >> 12)
        .wrapping_add(source_len >> 14)
        .wrapping_add(source_len >> 25)
        .wrapping_add(13)
}

extern "C" {
    fn _glp_zlib_deflate(strm: *mut ZStream, flush: c_int) -> c_int;
    fn _glp_zlib_deflateEnd(strm: *mut ZStream) -> c_int;
    fn _glp_zlib_deflateInit_(
        strm: *mut ZStream,
        level: c_int,
        version: *const c_char,
        stream_size: c_int,
    ) -> c_int;
}