use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::mem;
use std::slice;
use getopts::Options;
use nettle_sys::{
    nettle_hash, nettle_hashes, nettle_lookup_hash,
    nettle_hash_init, nettle_hash_update, nettle_hash_digest,
    BASE16_ENCODE_LENGTH
};

const BUFSIZE: usize = 16384;

fn list_algorithms() {
    println!("{:10} digestsize (internal block size, context size), in units of octets", "name");
    
    let mut i = 0;
    while let Some(alg) = unsafe { *nettle_hashes.offset(i) } {
        let name = unsafe { CStr::from_ptr(alg.name) }.to_string_lossy();
        println!("{:10} {} ({} {})",
            name,
            alg.digest_size,
            alg.block_size,
            alg.context_size
        );
        i += 1;
    }
}

fn hash_file(hash: *const nettle_hash, ctx: *mut libc::c_void, mut f: &File) -> io::Result<()> {
    let mut buffer = [0u8; BUFSIZE];
    loop {
        let res = f.read(&mut buffer)?;
        if res == 0 {
            break Ok(());
        }
        unsafe {
            nettle_hash_update(hash, ctx, res, buffer.as_ptr());
        }
    }
}

fn digest_file(
    alg: *const nettle_hash,
    digest_length: usize,
    raw: bool,
    mut f: &File,
) -> io::Result<()> {
    let ctx_size = unsafe { (*alg).context_size as usize };
    let mut ctx = vec![0u8; ctx_size];
    
    unsafe {
        nettle_hash_init(alg, ctx.as_mut_ptr() as *mut libc::c_void);
    }
    
    hash_file(alg, ctx.as_mut_ptr() as *mut libc::c_void, &f)?;
    
    let mut digest = vec![0u8; digest_length];
    unsafe {
        nettle_hash_digest(
            alg,
            ctx.as_mut_ptr() as *mut libc::c_void,
            digest_length,
            digest.as_mut_ptr(),
        );
    }
    
    if raw {
        io::stdout().write_all(&digest)?;
    } else {
        let mut hex = vec![0u8; BASE16_ENCODE_LENGTH(8) + 1];
        let mut i = 0;
        while i + 8 < digest_length {
            unsafe {
                base16_encode_update(
                    hex.as_mut_ptr() as *mut c_char,
                    8,
                    digest[i..].as_ptr(),
                );
            }
            hex[BASE16_ENCODE_LENGTH(8)] = 0;
            print!("{} ", String::from_utf8_lossy(&hex[..BASE16_ENCODE_LENGTH(8)]));
            i += 8;
        }
        
        unsafe {
            base16_encode_update(
                hex.as_mut_ptr() as *mut c_char,
                (digest_length - i) as usize,
                digest[i..].as_ptr(),
            );
        }
        hex[BASE16_ENCODE_LENGTH(digest_length - i)] = 0;
        let name = unsafe { CStr::from_ptr((*alg).name) }.to_string_lossy();
        println!(
            "{} {}",
            String::from_utf8_lossy(&hex[..BASE16_ENCODE_LENGTH(digest_length - i)]),
            name
        );
    }
    
    Ok(())
}

fn usage(f: &mut dyn Write) {
    writeln!(f, "Usage: nettle-hash -a ALGORITHM [OPTIONS] [FILE ...]\n\
                Options:\n\
                  --help              Show this help.\n\
                  -V, --version       Show version information.\n\
                  --list              List supported hash algorithms.\n\
                  -a, --algorithm=ALG Hash algorithm to use.\n\
                  -l, --length=LENGTH Desired digest length (octets)\n\
                  --raw               Raw binary output.").unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = Options::new();
    opts.optflag("", "help", "Show this help.");
    opts.optflag("V", "version", "Show version information.");
    opts.optopt("a", "algorithm", "Hash algorithm to use.", "ALG");
    opts.optopt("l", "length", "Desired digest length (octets)", "LENGTH");
    opts.optflag("", "list", "List supported hash algorithms.");
    opts.optflag("", "raw", "Raw binary output.");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    if matches.opt_present("help") {
        usage(&mut io::stdout());
        process::exit(0);
    }

    if matches.opt_present("version") {
        println!("nettle-hash ({})", env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    if matches.opt_present("list") {
        list_algorithms();
        process::exit(0);
    }

    let alg_name = match matches.opt_str("a") {
        Some(name) => name,
        None => {
            eprintln!("Algorithm argument (-a option) is mandatory.\n\
                     See nettle-hash --help for further information.");
            process::exit(1);
        }
    };

    let alg = unsafe { nettle_lookup_hash(alg_name.as_ptr() as *const c_char) };
    if alg.is_null() {
        eprintln!("Hash algorithm `{}' not supported.\n\
                  Use nettle-hash --list to list available algorithms.", alg_name);
        process::exit(1);
    }

    let length = match matches.opt_str("l") {
        Some(l) => match l.parse::<usize>() {
            Ok(l) if l > 0 => l,
            _ => {
                eprintln!("Invalid length argument: `{}'", l);
                process::exit(1);
            }
        },
        None => unsafe { (*alg).digest_size as usize },
    };

    if length > unsafe { (*alg).digest_size as usize } {
        eprintln!("Length argument {} too large for selected algorithm.", length);
        process::exit(1);
    }

    let raw = matches.opt_present("raw");
    let files = matches.free;

    if files.is_empty() {
        if let Err(e) = digest_file(alg, length, raw, &io::stdin()) {
            eprintln!("Error processing stdin: {}", e);
            process::exit(1);
        }
    } else {
        for file in files {
            match File::open(&file) {
                Ok(f) => {
                    print!("{}: ", file);
                    if let Err(e) = digest_file(alg, length, raw, &f) {
                        eprintln!("Error processing file `{}': {}", file, e);
                        process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Cannot open `{}': {}", file, e);
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