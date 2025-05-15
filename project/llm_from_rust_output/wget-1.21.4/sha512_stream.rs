use std::io::{Read, Error, ErrorKind};
use std::mem::MaybeUninit;
use std::ptr;

const SHA384_DIGEST_SIZE: usize = 48;
const SHA512_DIGEST_SIZE: usize = 64;

#[derive(Default)]
struct Sha512Ctx {
    state: [u64; 8],
    total: [u64; 2],
    buflen: usize,
    buffer: [u64; 32],
}

trait ShaAlgorithm {
    fn init_ctx(ctx: &mut Sha512Ctx);
    fn finish_ctx(ctx: &mut Sha512Ctx, resblock: &mut [u8]);
    fn digest_size() -> usize;
}

struct Sha512;
struct Sha384;

impl ShaAlgorithm for Sha512 {
    fn init_ctx(ctx: &mut Sha512Ctx) {
        unsafe { sha512_init_ctx(ctx) }
    }

    fn finish_ctx(ctx: &mut Sha512Ctx, resblock: &mut [u8]) {
        unsafe {
            sha512_finish_ctx(ctx, resblock.as_mut_ptr() as *mut libc::c_void);
        }
    }

    fn digest_size() -> usize {
        SHA512_DIGEST_SIZE
    }
}

impl ShaAlgorithm for Sha384 {
    fn init_ctx(ctx: &mut Sha512Ctx) {
        unsafe { sha384_init_ctx(ctx) }
    }

    fn finish_ctx(ctx: &mut Sha512Ctx, resblock: &mut [u8]) {
        unsafe {
            sha384_finish_ctx(ctx, resblock.as_mut_ptr() as *mut libc::c_void);
        }
    }

    fn digest_size() -> usize {
        SHA384_DIGEST_SIZE
    }
}

fn sha_stream<R: Read, A: ShaAlgorithm>(
    mut stream: R,
) -> Result<[u8; A::digest_size()], Error> {
    let mut buffer = vec![0u8; 32768 + 72];
    let mut ctx = Sha512Ctx::default();
    A::init_ctx(&mut ctx);

    loop {
        let bytes_read = stream.read(&mut buffer[..32768])?;
        if bytes_read == 0 {
            break;
        }

        if bytes_read == 32768 {
            unsafe {
                sha512_process_block(
                    buffer.as_ptr() as *const libc::c_void,
                    bytes_read,
                    &mut ctx,
                );
            }
        } else {
            unsafe {
                sha512_process_bytes(
                    buffer.as_ptr() as *const libc::c_void,
                    bytes_read,
                    &mut ctx,
                );
            }
        }
    }

    let mut result = [0u8; A::digest_size()];
    A::finish_ctx(&mut ctx, &mut result);
    Ok(result)
}

pub fn sha512_stream<R: Read>(stream: R) -> Result<[u8; SHA512_DIGEST_SIZE], Error> {
    sha_stream::<R, Sha512>(stream)
}

pub fn sha384_stream<R: Read>(stream: R) -> Result<[u8; SHA384_DIGEST_SIZE], Error> {
    sha_stream::<R, Sha384>(stream)
}

// These would be implemented elsewhere as unsafe extern "C" functions
extern "C" {
    fn sha384_init_ctx(ctx: *mut Sha512Ctx);
    fn sha512_init_ctx(ctx: *mut Sha512Ctx);
    fn sha512_finish_ctx(ctx: *mut Sha512Ctx, resbuf: *mut libc::c_void) -> *mut libc::c_void;
    fn sha384_finish_ctx(ctx: *mut Sha512Ctx, resbuf: *mut libc::c_void) -> *mut libc::c_void;
    fn sha512_process_bytes(buffer: *const libc::c_void, len: usize, ctx: *mut Sha512Ctx);
    fn sha512_process_block(buffer: *const libc::c_void, len: usize, ctx: *mut Sha512Ctx);
}