use std::ffi::CString;
use std::mem;
use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_ulong, c_void};

type Byte = c_uchar;
type uInt = c_uint;
type uLong = c_ulong;
type Bytef = Byte;
type uLongf = uLong;
type voidpf = *mut c_void;

#[derive(Debug)]
struct InternalState {
    dummy: c_int,
}

#[derive(Debug)]
struct ZStream {
    next_in: *mut Bytef,
    avail_in: uInt,
    total_in: uLong,
    next_out: *mut Bytef,
    avail_out: uInt,
    total_out: uLong,
    msg: *mut c_char,
    state: *mut InternalState,
    zalloc: Option<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>,
    zfree: Option<unsafe extern "C" fn(voidpf, voidpf) -> ()>,
    opaque: voidpf,
    data_type: c_int,
    adler: uLong,
    reserved: uLong,
}

impl Default for ZStream {
    fn default() -> Self {
        ZStream {
            next_in: std::ptr::null_mut(),
            avail_in: 0,
            total_in: 0,
            next_out: std::ptr::null_mut(),
            avail_out: 0,
            total_out: 0,
            msg: std::ptr::null_mut(),
            state: std::ptr::null_mut(),
            zalloc: None,
            zfree: None,
            opaque: std::ptr::null_mut(),
            data_type: 0,
            adler: 0,
            reserved: 0,
        }
    }
}

#[link(name = "z")]
extern "C" {
    fn _glp_zlib_inflate(strm: *mut ZStream, flush: c_int) -> c_int;
    fn _glp_zlib_inflateEnd(strm: *mut ZStream) -> c_int;
    fn _glp_zlib_inflateInit_(
        strm: *mut ZStream,
        version: *const c_char,
        stream_size: c_int,
    ) -> c_int;
}

pub fn zlib_uncompress(
    dest: &mut [Bytef],
    source: &[Bytef],
) -> Result<uLongf, c_int> {
    let mut stream = ZStream::default();
    let mut dest_len = dest.len() as uLongf;
    
    unsafe {
        stream.next_in = source.as_ptr() as *mut Bytef;
        stream.avail_in = source.len() as uInt;
        if stream.avail_in as uLong != source.len() as uLong {
            return Err(-5);
        }

        stream.next_out = dest.as_mut_ptr();
        stream.avail_out = dest_len as uInt;
        if stream.avail_out as uLong != dest_len {
            return Err(-5);
        }

        let version = CString::new("1.2.5").unwrap();
        let err = _glp_zlib_inflateInit_(
            &mut stream,
            version.as_ptr(),
            mem::size_of::<ZStream>() as c_int,
        );
        if err != 0 {
            return Err(err);
        }

        let err = _glp_zlib_inflate(&mut stream, 4);
        if err != 1 {
            _glp_zlib_inflateEnd(&mut stream);
            if err == 2 || (err == -5 && stream.avail_in == 0) {
                return Err(-3);
            }
            return Err(err);
        }

        dest_len = stream.total_out;
        let err = _glp_zlib_inflateEnd(&mut stream);
        if err != 0 {
            return Err(err);
        }
    }

    Ok(dest_len)
}