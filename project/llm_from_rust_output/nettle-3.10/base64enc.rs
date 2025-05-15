use std::error::Error;
use std::ffi::CString;
use std::io::{self, Read, Write};
use std::process;

struct Base64EncodeContext {
    alphabet: *const libc::c_char,
    word: libc::c_ushort,
    bits: libc::c_uchar,
}

extern "C" {
    fn nettle_base64_encode_init(ctx: *mut Base64EncodeContext);
    fn nettle_base64_encode_update(
        ctx: *mut Base64EncodeContext,
        dst: *mut libc::c_char,
        length: libc::size_t,
        src: *const libc::uint8_t,
    ) -> libc::size_t;
    fn nettle_base64_encode_final(
        ctx: *mut Base64EncodeContext,
        dst: *mut libc::c_char,
    ) -> libc::size_t;
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut b64_ctx = Base64EncodeContext {
        alphabet: std::ptr::null(),
        word: 0,
        bits: 0,
    };
    
    unsafe {
        nettle_base64_encode_init(&mut b64_ctx);
    }

    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    loop {
        let mut buffer = [0u8; 54];
        let mut result = [0i8; 76];
        
        let nbytes = stdin.read(&mut buffer)?;
        if nbytes == 0 {
            break;
        }

        let encoded_bytes = unsafe {
            nettle_base64_encode_update(
                &mut b64_ctx,
                result.as_mut_ptr(),
                nbytes as libc::size_t,
                buffer.as_ptr(),
            ) as usize
        };

        if nbytes < buffer.len() {
            let final_bytes = unsafe {
                nettle_base64_encode_final(
                    &mut b64_ctx,
                    result.as_mut_ptr().add(encoded_bytes),
                ) as usize
            };
            let total_bytes = encoded_bytes + final_bytes;
            result[total_bytes] = b'\n' as i8;
            
            stdout.write_all(unsafe {
                std::slice::from_raw_parts(result.as_ptr() as *const u8, total_bytes + 1)
            })?;
            stdout.flush()?;
            return Ok(());
        }

        result[encoded_bytes] = b'\n' as i8;
        stdout.write_all(unsafe {
            std::slice::from_raw_parts(result.as_ptr() as *const u8, encoded_bytes + 1)
        })?;
    }

    Ok(())
}

fn real_main() -> Result<(), Box<dyn Error>> {
    main()
}

fn main_wrapper() {
    if let Err(e) = real_main() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}