// compress.rs -- compress a memory buffer
// Translated from C to Rust while maintaining original functionality
// Original C code Copyright (C) 1995-2005 Jean-loup Gailly

use libz_sys::{
    z_stream, Z_OK, Z_BUF_ERROR, Z_STREAM_ERROR, Z_MEM_ERROR, Z_STREAM_END,
    Z_DEFAULT_COMPRESSION, deflateInit, deflate, deflateEnd
};
use std::ptr;

/* ===========================================================================
     Compresses the source buffer into the destination buffer. The level
   parameter has the same meaning as in deflateInit.  sourceLen is the byte
   length of the source buffer. Upon entry, destLen is the total size of the
   destination buffer, which must be at least 0.1% larger than sourceLen plus
   12 bytes. Upon exit, destLen is the actual size of the compressed buffer.

     compress2 returns Z_OK if success, Z_MEM_ERROR if there was not enough
   memory, Z_BUF_ERROR if there was not enough room in the output buffer,
   Z_STREAM_ERROR if the level parameter is invalid.
*/
pub fn compress2(
    dest: &mut [u8],
    dest_len: &mut u32,
    source: &[u8],
    level: i32
) -> i32 {
    let mut stream = z_stream {
        next_in: source.as_ptr() as *mut u8,
        avail_in: source.len() as u32,
        next_out: dest.as_mut_ptr(),
        avail_out: *dest_len,
        zalloc: None,
        zfree: None,
        opaque: ptr::null_mut(),
        total_in: 0,
        total_out: 0,
        msg: ptr::null_mut(),
        state: ptr::null_mut(),
        data_type: 0,
        adler: 0,
        reserved: 0,
    };

    // Check for source > 64K on 16-bit machine (preserved from original C code)
    if stream.avail_in as usize != source.len() {
        return Z_BUF_ERROR;
    }
    if stream.avail_out as u32 != *dest_len {
        return Z_BUF_ERROR;
    }

    let err = unsafe { deflateInit(&mut stream, level) };
    if err != Z_OK {
        return err;
    }

    let err = unsafe { deflate(&mut stream, libz_sys::Z_FINISH) };
    if err != Z_STREAM_END {
        unsafe { deflateEnd(&mut stream) };
        return if err == Z_OK { Z_BUF_ERROR } else { err };
    }

    *dest_len = stream.total_out;

    unsafe { deflateEnd(&mut stream) }
}

/* ===========================================================================
 */
pub fn compress(
    dest: &mut [u8],
    dest_len: &mut u32,
    source: &[u8]
) -> i32 {
    compress2(dest, dest_len, source, Z_DEFAULT_COMPRESSION)
}

/* ===========================================================================
     If the default memLevel or windowBits for deflateInit() is changed, then
   this function needs to be updated.
 */
pub fn compress_bound(source_len: u32) -> u32 {
    source_len + (source_len >> 12) + (source_len >> 14) +
    (source_len >> 25) + 13
}