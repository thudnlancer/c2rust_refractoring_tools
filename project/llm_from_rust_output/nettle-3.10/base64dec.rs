use std::io::{self, Read, Write};
use std::ffi::CString;
use std::process;
use std::ptr;
use std::slice;
use libc::{c_int, c_void, size_t, uint8_t};

extern "C" {
    fn nettle_base64_decode_init(ctx: *mut base64_decode_ctx);
    fn nettle_base64_decode_update(
        ctx: *mut base64_decode_ctx,
        dst_length: *mut size_t,
        dst: *mut uint8_t,
        src_length: size_t,
        src: *const libc::c_char,
    ) -> c_int;
    fn nettle_base64_decode_final(ctx: *mut base64_decode_ctx) -> c_int;
}

#[repr(C)]
pub struct base64_decode_ctx {
    pub table: *const libc::c_schar,
    pub word: libc::c_ushort,
    pub bits: libc::c_uchar,
    pub padding: libc::c_uchar,
}

fn main() -> io::Result<()> {
    let mut buffer = vec![0u8; 16392];
    let mut result = vec![0u8; (16392 + 1) * 6 / 8];
    let mut b64_ctx = unsafe {
        let mut ctx = base64_decode_ctx {
            table: ptr::null(),
            word: 0,
            bits: 0,
            padding: 0,
        };
        nettle_base64_decode_init(&mut ctx);
        ctx
    };

    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();
    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();

    loop {
        let nbytes = stdin_handle.read(&mut buffer)?;
        if nbytes == 0 {
            break;
        }

        let mut decoded_bytes = 0;
        let success = unsafe {
            nettle_base64_decode_update(
                &mut b64_ctx,
                &mut decoded_bytes,
                result.as_mut_ptr(),
                nbytes as size_t,
                buffer.as_ptr() as *const libc::c_char,
            )
        };

        if success == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Error decoding input (not base64?)",
            ));
        }

        stdout_handle.write_all(&result[..decoded_bytes as usize])?;

        if nbytes < buffer.len() {
            let final_success = unsafe { nettle_base64_decode_final(&mut b64_ctx) };
            if final_success == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Decoding did not finish properly",
                ));
            }
            break;
        }
    }

    stdout_handle.flush()?;
    Ok(())
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    main()?;
    Ok(())
}

fn main_wrapper() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}