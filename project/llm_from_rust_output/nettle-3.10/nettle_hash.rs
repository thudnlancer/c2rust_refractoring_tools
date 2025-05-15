use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{self, Read, Write};
use std::process;
use std::ptr;
use std::str;

use libc::{c_char, c_int, c_uint, c_void, size_t};
use nettle_sys::{
    nettle_base16_encode_update, nettle_get_hashes, nettle_hash, nettle_lookup_hash,
    nettle_hash_init_func, nettle_hash_update_func, nettle_hash_digest_func,
};

const OPT_HELP: c_int = 768;
const OPT_RAW: c_int = 769;
const OPT_LIST: c_int = 770;

struct Options {
    alg_name: Option<String>,
    length: c_uint,
    raw: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            alg_name: None,
            length: 0,
            raw: false,
        }
    }
}

fn list_algorithms() {
    println!("{:10} digestsize (internal block size, context size), in units of octets", "name");
    
    unsafe {
        let mut i = 0;
        loop {
            let alg = *nettle_get_hashes().offset(i);
            if alg.is_null() {
                break;
            }
            println!(
                "{:10} {} ({} {})",
                CStr::from_ptr((*alg).name).to_str().unwrap(),
                (*alg).digest_size,
                (*alg).block_size,
                (*alg).context_size
            );
            i += 1;
        }
    }
}

fn hash_file(hash: &nettle_hash, ctx: *mut c_void, mut file: File) -> io::Result<()> {
    let mut buffer = [0u8; 16384];
    loop {
        let res = file.read(&mut buffer)?;
        if res == 0 {
            break;
        }
        unsafe {
            ((*hash).update.expect("non-null function pointer"))(
                ctx,
                res as size_t,
                buffer.as_ptr(),
            );
        }
    }
    Ok(())
}

fn digest_file(alg: &nettle_hash, digest_length: c_uint, raw: bool, file: File) -> io::Result<()> {
    let ctx = unsafe {
        let ctx = libc::malloc(alg.context_size as size_t);
        if ctx.is_null() {
            return Err(io::Error::new(io::ErrorKind::Other, "Memory allocation failed"));
        }
        (alg.init.expect("non-null function pointer"))(ctx);
        ctx
    };

    if let Err(e) = hash_file(alg, ctx, file) {
        unsafe { libc::free(ctx) };
        return Err(e);
    }

    let digest = unsafe {
        let digest = libc::malloc(digest_length as size_t) as *mut u8;
        if digest.is_null() {
            libc::free(ctx);
            return Err(io::Error::new(io::ErrorKind::Other, "Memory allocation failed"));
        }
        (alg.digest.expect("non-null function pointer"))(ctx, digest_length as size_t, digest);
        libc::free(ctx);
        digest
    };

    if raw {
        unsafe {
            let slice = std::slice::from_raw_parts(digest, digest_length as usize);
            io::stdout().write_all(slice)?;
        }
    } else {
        let mut hex = [0u8; 17];
        let mut i = 0;
        while i + 8 < digest_length {
            unsafe {
                nettle_base16_encode_update(
                    hex.as_mut_ptr() as *mut c_char,
                    8,
                    digest.offset(i as isize),
                );
            }
            hex[16] = 0;
            print!(
                "{} ",
                CStr::from_ptr(hex.as_ptr() as *const c_char).to_str().unwrap()
            );
            i += 8;
        }
        unsafe {
            nettle_base16_encode_update(
                hex.as_mut_ptr() as *mut c_char,
                (digest_length - i) as size_t,
                digest.offset(i as isize),
            );
        }
        hex[(digest_length - i) as usize * 2] = 0;
        println!(
            "{} {}",
            CStr::from_ptr(hex.as_ptr() as *const c_char).to_str().unwrap(),
            CStr::from_ptr(alg.name).to_str().unwrap()
        );
    }

    unsafe { libc::free(digest as *mut c_void) };
    Ok(())
}

fn usage() {
    println!(
        r#"Usage: nettle-hash -a ALGORITHM [OPTIONS] [FILE ...]
Options:
  --help              Show this help.
  -V, --version       Show version information.
  --list              List supported hash algorithms.
  -a, --algorithm=ALG Hash algorithm to use.
  -l, --length=LENGTH Desired digest length (octets)
  --raw               Raw binary output."#
    );
}

fn parse_options(args: &[String]) -> Result<Options, String> {
    let mut options = Options::default();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--help" => {
                usage();
                process::exit(0);
            }
            "-V" | "--version" => {
                println!("nettle-hash (nettle 3.10)");
                process::exit(0);
            }
            "--list" => {
                list_algorithms();
                process::exit(0);
            }
            "-a" | "--algorithm" => {
                i += 1;
                if i >= args.len() {
                    return Err("Missing algorithm argument".to_string());
                }
                options.alg_name = Some(args[i].clone());
            }
            "-l" | "--length" => {
                i += 1;
                if i >= args.len() {
                    return Err("Missing length argument".to_string());
                }
                options.length = args[i]
                    .parse()
                    .map_err(|_| format!("Invalid length argument: `{}`", args[i]))?;
                if options.length == 0 {
                    return Err(format!("Invalid length argument: `{}`", args[i]));
                }
            }
            "--raw" => {
                options.raw = true;
            }
            _ => {
                if args[i].starts_with('-') {
                    return Err(format!("Unknown option: {}", args[i]));
                } else {
                    break;
                }
            }
        }
        i += 1;
    }

    if options.alg_name.is_none() {
        return Err(
            "Algorithm argument (-a option) is mandatory.\nSee nettle-hash --help for further information."
                .to_string(),
        );
    }

    Ok(options)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let options = match parse_options(&args) {
        Ok(opts) => opts,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let alg_name = CString::new(options.alg_name.unwrap()).unwrap();
    let alg = unsafe { nettle_lookup_hash(alg_name.as_ptr()) };
    if alg.is_null() {
        eprintln!(
            "Hash algorithm `{}` not supported.\nUse nettle-hash --list to list available algorithms.",
            alg_name.to_str().unwrap()
        );
        process::exit(1);
    }

    let alg_ref = unsafe { &*alg };
    let digest_length = if options.length == 0 {
        alg_ref.digest_size
    } else if options.length > alg_ref.digest_size {
        eprintln!(
            "Length argument {} too large for selected algorithm.",
            options.length
        );
        process::exit(1);
    } else {
        options.length
    };

    let files = if args.len() > 1 {
        &args[1..]
    } else {
        &[]
    };

    if files.is_empty() {
        if let Err(e) = digest_file(alg_ref, digest_length, options.raw, io::stdin()) {
            eprintln!("Error processing stdin: {}", e);
            process::exit(1);
        }
    } else {
        for file in files {
            match File::open(file) {
                Ok(f) => {
                    print!("{}: ", file);
                    if let Err(e) = digest_file(alg_ref, digest_length, options.raw, f) {
                        eprintln!("Error processing {}: {}", file, e);
                        process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Cannot open `{}`: {}", file, e);
                    process::exit(1);
                }
            }
        }
    }

    if let Err(e) = io::stdout().flush() {
        eprintln!("Write failed: {}", e);
        process::exit(1);
    }
}