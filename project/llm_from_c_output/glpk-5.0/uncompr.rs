use libz_sys::{z_stream, Z_OK, Z_BUF_ERROR, Z_STREAM_END, Z_NEED_DICT, Z_DATA_ERROR, inflateInit_, inflate, inflateEnd, ZLIB_VERSION};

/// Decompresses the source buffer into the destination buffer.
/// 
/// `source_len` is the byte length of the source buffer. Upon entry, `dest_len` is the total
/// size of the destination buffer, which must be large enough to hold the entire uncompressed data.
/// (The size of the uncompressed data must have been saved previously by the compressor and
/// transmitted to the decompressor by some mechanism outside the scope of this compression library.)
/// Upon exit, `dest_len` is the actual size of the compressed buffer.
///
/// Returns `Z_OK` if success, `Z_MEM_ERROR` if there was not enough memory,
/// `Z_BUF_ERROR` if there was not enough room in the output buffer,
/// or `Z_DATA_ERROR` if the input data was corrupted.
pub fn uncompress(
    dest: &mut [u8],
    dest_len: &mut u32,
    source: &[u8],
    source_len: u32,
) -> i32 {
    let mut stream: z_stream = unsafe { std::mem::zeroed() };

    stream.next_in = source.as_ptr() as *mut _;
    stream.avail_in = source_len as u32;
    // Check for source > 64K on 16-bit machine:
    if stream.avail_in as u32 != source_len {
        return Z_BUF_ERROR;
    }

    stream.next_out = dest.as_mut_ptr();
    stream.avail_out = *dest_len as u32;
    if stream.avail_out as u32 != *dest_len {
        return Z_BUF_ERROR;
    }

    stream.zalloc = None;
    stream.zfree = None;

    unsafe {
        let err = inflateInit_(&mut stream, ZLIB_VERSION as *const i8, std::mem::size_of::<z_stream>() as i32);
        if err != Z_OK {
            return err;
        }

        let err = inflate(&mut stream, libz_sys::Z_FINISH);
        if err != Z_STREAM_END {
            inflateEnd(&mut stream);
            if err == Z_NEED_DICT || (err == Z_BUF_ERROR && stream.avail_in == 0) {
                return Z_DATA_ERROR;
            }
            return err;
        }
        *dest_len = stream.total_out;

        inflateEnd(&mut stream)
    }
}