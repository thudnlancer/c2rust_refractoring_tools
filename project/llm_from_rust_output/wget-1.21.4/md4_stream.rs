use std::io::{self, Read};
use std::slice;

#[derive(Default, Clone, Copy)]
struct Md4Context {
    A: u32,
    B: u32,
    C: u32,
    D: u32,
    total: [u32; 2],
    buflen: u32,
    buffer: [u32; 32],
}

impl Md4Context {
    fn new() -> Self {
        let mut ctx = Self::default();
        md4_init_ctx(&mut ctx);
        ctx
    }

    fn process_bytes(&mut self, buffer: &[u8]) {
        unsafe {
            md4_process_bytes(
                buffer.as_ptr() as *const libc::c_void,
                buffer.len(),
                self as *mut _,
            );
        }
    }

    fn process_block(&mut self, buffer: &[u8]) {
        unsafe {
            md4_process_block(
                buffer.as_ptr() as *const libc::c_void,
                buffer.len(),
                self as *mut _,
            );
        }
    }

    fn finish(&mut self, resblock: &mut [u8; 16]) {
        unsafe {
            md4_finish_ctx(
                self as *mut _,
                resblock.as_mut_ptr() as *mut libc::c_void,
            );
        }
    }
}

fn md4_stream(stream: &mut dyn Read) -> io::Result<[u8; 16]> {
    let mut ctx = Md4Context::new();
    let mut buffer = vec![0u8; 32768 + 72];

    loop {
        let mut sum = 0;
        loop {
            let n = stream.read(&mut buffer[sum..32768])?;
            sum += n;

            if sum == 32768 {
                break;
            }
            if n == 0 {
                break;
            }
        }

        if sum == 0 {
            break;
        }

        if sum == 32768 {
            ctx.process_block(&buffer[..32768]);
        } else {
            ctx.process_bytes(&buffer[..sum]);
            break;
        }
    }

    let mut resblock = [0u8; 16];
    ctx.finish(&mut resblock);
    Ok(resblock)
}

// These would be implemented elsewhere as unsafe extern functions
extern "C" {
    fn md4_init_ctx(ctx: *mut Md4Context);
    fn md4_process_bytes(buffer: *const libc::c_void, len: usize, ctx: *mut Md4Context);
    fn md4_process_block(buffer: *const libc::c_void, len: usize, ctx: *mut Md4Context);
    fn md4_finish_ctx(ctx: *mut Md4Context, resbuf: *mut libc::c_void) -> *mut libc::c_void;
}