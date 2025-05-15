use std::env;
use std::ffi::{CString, CStr};
use std::fs::File;
use std::io::{self, Read, Write};
use std::process;
use std::ptr;
use libc::{c_int, c_uint, c_char, c_void, c_uchar, c_long, c_ushort, c_schar, c_ulong};
use libc::{strtol, strcmp, strerror, fcntl, memset, printf, fflush, exit, abort};
use libc::{stdin, stdout, FILE, _IO_FILE, flock, __off_t, __pid_t};

type size_t = c_ulong;
type uint8_t = c_uchar;
type sexp_mode = c_uint;
type sexp_token = c_uint;
type sexp_char_type = c_uint;

const SEXP_TRANSPORT: sexp_mode = 2;
const SEXP_ADVANCED: sexp_mode = 1;
const SEXP_CANONICAL: sexp_mode = 0;

const SEXP_CODING_END: sexp_token = 9;
const SEXP_TRANSPORT_START: sexp_token = 8;
const SEXP_DISPLAY_END: sexp_token = 7;
const SEXP_DISPLAY_START: sexp_token = 6;
const SEXP_EOF: sexp_token = 5;
const SEXP_LIST_END: sexp_token = 4;
const SEXP_LIST_START: sexp_token = 3;
const SEXP_COMMENT: sexp_token = 2;
const SEXP_DISPLAY: sexp_token = 1;
const SEXP_STRING: sexp_token = 0;

const SEXP_END_CHAR: sexp_char_type = 2;
const SEXP_EOF_CHAR: sexp_char_type = 1;
const SEXP_NORMAL_CHAR: sexp_char_type = 0;

const OPT_HELP: c_uint = 303;
const OPT_LOCK: c_uint = 302;
const OPT_HASH: c_uint = 301;
const OPT_ONCE: c_uint = 300;

#[repr(C)]
struct NettleBuffer {
    contents: *mut uint8_t,
    alloc: size_t,
    realloc_ctx: *mut c_void,
    realloc: Option<extern "C" fn(*mut c_void, *mut c_void, size_t) -> *mut c_void>,
    size: size_t,
}

#[repr(C)]
struct NettleHash {
    name: *const c_char,
    context_size: c_uint,
    digest_size: c_uint,
    block_size: c_uint,
    init: Option<extern "C" fn(*mut c_void)>,
    update: Option<extern "C" fn(*mut c_void, size_t, *const uint8_t)>,
    digest: Option<extern "C" fn(*mut c_void, size_t, *mut uint8_t)>,
}

#[repr(C)]
struct NettleArmor {
    name: *const c_char,
    encode_context_size: c_uint,
    decode_context_size: c_uint,
    encode_final_length: c_uint,
    encode_init: Option<extern "C" fn(*mut c_void)>,
    encode_length: Option<extern "C" fn(size_t) -> size_t>,
    encode_update: Option<extern "C" fn(*mut c_void, *mut c_char, size_t, *const uint8_t) -> size_t>,
    encode_final: Option<extern "C" fn(*mut c_void, *mut c_char) -> size_t>,
    decode_init: Option<extern "C" fn(*mut c_void)>,
    decode_length: Option<extern "C" fn(size_t) -> size_t>,
    decode_update: Option<extern "C" fn(*mut c_void, *mut size_t, *mut uint8_t, size_t, *const c_char) -> c_int>,
    decode_final: Option<extern "C" fn(*mut c_void) -> c_int>,
}

#[repr(C)]
struct Option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

#[repr(C)]
struct Base16DecodeCtx {
    word: c_uchar,
    bits: c_uchar,
}

#[repr(C)]
struct Base64DecodeCtx {
    table: *const c_schar,
    word: c_ushort,
    bits: c_uchar,
    padding: c_uchar,
}

#[repr(C)]
union C2RustUnnamed {
    base64: Base64DecodeCtx,
    hex: Base16DecodeCtx,
}

#[repr(C)]
struct SexpInput {
    f: *mut FILE,
    ctype: sexp_char_type,
    c: uint8_t,
    coding: *const NettleArmor,
    state: C2RustUnnamed,
    terminator: uint8_t,
    token: sexp_token,
}

#[repr(C)]
struct SexpOutput {
    f: *mut FILE,
    line_width: c_uint,
    coding: *const NettleArmor,
    coding_indent: c_uint,
    prefer_hex: c_int,
    hash: *const NettleHash,
    ctx: *mut c_void,
    base64: Base64DecodeCtx,
    pos: c_uint,
    soft_newline: c_int,
}

#[repr(C)]
struct SexpCompoundToken {
    type_: sexp_token,
    display: NettleBuffer,
    string: NettleBuffer,
}

#[repr(C)]
struct SexpParser {
    input: *mut SexpInput,
    mode: sexp_mode,
    level: c_uint,
    transport: c_uint,
}

#[repr(C)]
struct ConvOptions {
    mode: sexp_mode,
    prefer_hex: c_int,
    once: c_int,
    lock: c_int,
    width: c_uint,
    hash: *const NettleHash,
}

extern "C" {
    static nettle_md5: NettleHash;
    static nettle_sha1: NettleHash;
    static nettle_sha256: NettleHash;
    static nettle_base64: NettleArmor;
    
    fn sexp_input_init(input: *mut SexpInput, f: *mut FILE);
    fn sexp_get_char(input: *mut SexpInput);
    fn sexp_output_init(output: *mut SexpOutput, f: *mut FILE, width: c_uint, prefer_hex: c_int);
    fn sexp_output_hash_init(output: *mut SexpOutput, hash: *const NettleHash, ctx: *mut c_void);
    fn sexp_put_newline(output: *mut SexpOutput, indent: c_uint);
    fn sexp_put_soft_newline(output: *mut SexpOutput, indent: c_uint);
    fn sexp_put_char(output: *mut SexpOutput, c: uint8_t);
    fn sexp_put_data(output: *mut SexpOutput, length: c_uint, data: *const uint8_t);
    fn sexp_put_code_start(output: *mut SexpOutput, coding: *const NettleArmor);
    fn sexp_put_code_end(output: *mut SexpOutput);
    fn sexp_put_string(output: *mut SexpOutput, mode: sexp_mode, string: *mut NettleBuffer);
    fn sexp_put_digest(output: *mut SexpOutput);
    fn sexp_compound_token_init(token: *mut SexpCompoundToken);
    fn sexp_compound_token_clear(token: *mut SexpCompoundToken);
    fn sexp_parse_init(parser: *mut SexpParser, input: *mut SexpInput, mode: sexp_mode);
    fn sexp_parse(parser: *mut SexpParser, token: *mut SexpCompoundToken);
    fn getopt_long(argc: c_int, argv: *const *mut c_char, shortopts: *const c_char, longopts: *const Option, longind: *mut c_int) -> c_int;
    fn die(format: *const c_char, ...) -> !;
    fn werror(format: *const c_char, ...);
    fn xalloc(size: size_t) -> *mut c_void;
}

fn match_argument(given: &str, name: &str) -> bool {
    given == name
}

fn parse_options(o: &mut ConvOptions, argc: c_int, argv: *mut *mut c_char) {
    o.mode = SEXP_ADVANCED;
    o.prefer_hex = 0;
    o.once = 0;
    o.lock = 0;
    o.hash = ptr::null();
    o.width = 72;

    let hashes = [
        &nettle_md5 as *const NettleHash,
        &nettle_sha1 as *const NettleHash,
        &nettle_sha256 as *const NettleHash,
        ptr::null(),
    ];

    let options = [
        Option {
            name: b"help\0".as_ptr() as *const c_char,
            has_arg: 0,
            flag: ptr::null_mut(),
            val: OPT_HELP as c_int,
        },
        Option {
            name: b"version\0".as_ptr() as *const c_char,
            has_arg: 0,
            flag: ptr::null_mut(),
            val: 'V' as i32,
        },
        Option {
            name: b"once\0".as_ptr() as *const c_char,
            has_arg: 0,
            flag: ptr::null_mut(),
            val: OPT_ONCE as c_int,
        },
        Option {
            name: b"syntax\0".as_ptr() as *const c_char,
            has_arg: 1,
            flag: ptr::null_mut(),
            val: 's' as i32,
        },
        Option {
            name: b"hash\0".as_ptr() as *const c_char,
            has_arg: 2,
            flag: ptr::null_mut(),
            val: OPT_HASH as c_int,
        },
        Option {
            name: b"raw-hash\0".as_ptr() as *const c_char,
            has_arg: 2,
            flag: ptr::null_mut(),
            val: OPT_HASH as c_int,
        },
        Option {
            name: b"width\0".as_ptr() as *const c_char,
            has_arg: 1,
            flag: ptr::null_mut(),
            val: 'w' as i32,
        },
        Option {
            name: b"lock\0".as_ptr() as *const c_char,
            has_arg: 0,
            flag: ptr::null_mut(),
            val: OPT_LOCK as c_int,
        },
        Option {
            name: ptr::null(),
            has_arg: 0,
            flag: ptr::null_mut(),
            val: 0,
        },
    ];

    loop {
        let mut option_index = 0;
        let c = unsafe {
            getopt_long(
                argc,
                argv,
                b"Vs:w:\0".as_ptr() as *const c_char,
                options.as_ptr(),
                &mut option_index,
            )
        };

        match c {
            -1 => {
                if unsafe { optind } != argc {
                    unsafe {
                        die(
                            b"sexp-conv: Command line takes no arguments, only options.\n\0"
                                .as_ptr() as *const c_char,
                        );
                    }
                }
                return;
            }
            63 => process::exit(1),
            119 => {
                let optarg = unsafe { optarg };
                if optarg.is_null() {
                    panic!("optarg is null");
                }

                let mut end = ptr::null_mut();
                let width = unsafe { strtol(optarg, &mut end, 0) };

                if unsafe { *optarg } == 0 || unsafe { *end } != 0 || width < 0 {
                    unsafe {
                        die(
                            b"sexp-conv: Invalid width `%s'.\n\0".as_ptr() as *const c_char,
                            optarg,
                        );
                    }
                }
                o.width = width as c_uint;
            }
            115 => {
                if !o.hash.is_null() {
                    unsafe {
                        werror(
                            b"sexp-conv: Combining --hash and -s usually makes no sense.\n\0"
                                .as_ptr() as *const c_char,
                        );
                    }
                }

                let optarg = unsafe { CStr::from_ptr(optarg) }.to_str().unwrap();
                if match_argument(optarg, "advanced") {
                    o.mode = SEXP_ADVANCED;
                } else if match_argument(optarg, "transport") {
                    o.mode = SEXP_TRANSPORT;
                } else if match_argument(optarg, "canonical") {
                    o.mode = SEXP_CANONICAL;
                } else if match_argument(optarg, "hex") {
                    o.mode = SEXP_ADVANCED;
                    o.prefer_hex = 1;
                } else {
                    unsafe {
                        die(
                            b"Available syntax variants: advanced, transport, canonical\n\0"
                                .as_ptr() as *const c_char,
                        );
                    }
                }
            }
            300 => o.once = 1,
            301 => {
                o.mode = SEXP_CANONICAL;
                let optarg = unsafe { optarg };
                if optarg.is_null() {
                    o.hash = &nettle_sha1;
                } else {
                    let mut i = 0;
                    loop {
                        if hashes[i].is_null() {
                            unsafe {
                                die(
                                    b"sexp_conv: Unknown hash algorithm `%s'\n\0"
                                        .as_ptr() as *const c_char,
                                    optarg,
                                );
                            }
                        }
                        let hash_name = unsafe { CStr::from_ptr((*hashes[i]).name) }.to_str().unwrap();
                        let arg = unsafe { CStr::from_ptr(optarg) }.to_str().unwrap();
                        if match_argument(arg, hash_name) {
                            o.hash = hashes[i];
                            break;
                        }
                        i += 1;
                    }
                }
            }
            302 => o.lock = 1,
            303 => {
                unsafe {
                    printf(
                        b"Usage: sexp-conv [OPTION...]\n  Conversion:     sexp-conv [OPTION...] <INPUT-SEXP\n  Fingerprinting: sexp-conv --hash=HASH <INPUT-SEXP\n\nReads an s-expression on stdin, and outputs the same\nsexp on stdout, possibly with a different syntax.\n\n       --hash[=ALGORITHM]   Outputs only the hash of the expression.\n                            Available hash algorithms:\n                            \0"
                            .as_ptr() as *const c_char,
                    );
                }
                let mut i = 0;
                while !hashes[i].is_null() {
                    if i != 0 {
                        unsafe { printf(b", \0".as_ptr() as *const c_char) };
                    }
                    unsafe { printf(b"%s\0".as_ptr() as *const c_char, (*hashes[i]).name) };
                    i += 1;
                }
                unsafe {
                    printf(
                        b" (default is sha1).\n   -s, --syntax=SYNTAX      The syntax used for the output. Available\n                            variants: advanced, hex, transport, canonical\n       --once               Process only the first s-expression.\n   -w, --width=WIDTH        Linewidth for base64 encoded data.\n                            Zero means no limit.\n       --lock               Lock output file.\n       --raw-hash           Alias for --hash, for compatibility\n                            with lsh-1.x.\n\nReport bugs to nettle-bugs@lists.lysator.liu.se.\n\0"
                            .as_ptr() as *const c_char,
                    );
                    exit(0);
                }
            }
            86 => {
                unsafe {
                    printf(b"sexp-conv (nettle 3.10)\n\0".as_ptr() as *const c_char);
                    exit(0);
                }
            }
            _ => unsafe { abort() },
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut c_args: Vec<*mut c_char> = args
        .iter()
        .map(|arg| CString::new(arg.as_str()).unwrap().into_raw())
        .collect();
    c_args.push(ptr::null_mut());

    let mut options = ConvOptions {
        mode: SEXP_CANONICAL,
        prefer_hex: 0,
        once: 0,
        lock: 0,
        width: 0,
        hash: ptr::null(),
    };

    unsafe {
        parse_options(
            &mut options,
            (c_args.len() - 1) as c_int,
            c_args.as_mut_ptr(),
        );

        let mut input = SexpInput {
            f: stdin,
            ctype: SEXP_NORMAL_CHAR,
            c: 0,
            coding: ptr::null(),
            state: C2RustUnnamed {
                base64: Base64DecodeCtx {
                    table: ptr::null(),
                    word: 0,
                    bits: 0,
                    padding: 0,
                },
            },
            terminator: 0,
            token: SEXP_STRING,
        };

        let mut parser = SexpParser {
            input: &mut input,
            mode: SEXP_ADVANCED,
            level: 0,
            transport: 0,
        };

        let mut token = SexpCompoundToken {
            type_: SEXP_STRING,
            display: NettleBuffer {
                contents: ptr::null_mut(),
                alloc: 0,
                realloc_ctx: ptr::null_mut(),
                realloc: None,
                size: 0,
            },
            string: NettleBuffer {
                contents: ptr::null_mut(),
                alloc: 0,
                realloc_ctx: ptr::null_mut(),
                realloc: None,
                size: 0,
            },
        };

        let mut output = SexpOutput {
            f: stdout,
            line_width: options.width,
            coding: ptr::null(),
            coding_indent: 0,
            prefer_hex: options.prefer_hex,
            hash: