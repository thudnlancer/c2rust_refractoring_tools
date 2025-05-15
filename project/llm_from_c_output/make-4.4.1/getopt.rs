use std::env;
use std::ffi::{CString, OsString};
use std::os::unix::ffi::OsStringExt;
use std::ptr;

#[derive(Debug, Clone, Copy)]
pub enum ArgumentRequirement {
    NoArgument,
    RequiredArgument,
    OptionalArgument,
}

#[derive(Debug)]
pub struct Option {
    pub name: String,
    pub has_arg: ArgumentRequirement,
    pub flag: Option<*mut i32>,
    pub val: i32,
}

pub static mut optarg: *mut libc::c_char = ptr::null_mut();
pub static mut optind: libc::c_int = 1;
pub static mut opterr: libc::c_int = 1;
pub static mut optopt: libc::c_int = '?' as i32;

static mut __getopt_initialized: libc::c_int = 0;
static mut nextchar: *const libc::c_char = ptr::null();
static mut first_nonopt: libc::c_int = 0;
static mut last_nonopt: libc::c_int = 0;

#[derive(Debug, Clone, Copy)]
enum Ordering {
    RequireOrder,
    Permute,
    ReturnInOrder,
}

static mut ordering: Ordering = Ordering::Permute;

unsafe fn exchange(argv: *mut *mut libc::c_char) {
    let bottom = first_nonopt;
    let middle = last_nonopt;
    let top = optind;
    let mut tem: *mut libc::c_char;

    while top > middle && middle > bottom {
        if top - middle > middle - bottom {
            let len = middle - bottom;
            for i in 0..len {
                tem = *argv.offset((bottom + i) as isize);
                *argv.offset((bottom + i) as isize) =
                    *argv.offset((top - (middle - bottom) + i) as isize);
                *argv.offset((top - (middle - bottom) + i) as isize = tem;
            }
            first_nonopt += (optind - last_nonopt);
            last_nonopt = optind;
        } else {
            let len = top - middle;
            for i in 0..len {
                tem = *argv.offset((bottom + i) as isize);
                *argv.offset((bottom + i) as isize) =
                    *argv.offset((middle + i) as isize);
                *argv.offset((middle + i) as isize) = tem;
            }
            first_nonopt += (optind - last_nonopt);
            last_nonopt = optind;
        }
    }
}

unsafe fn _getopt_initialize(
    argc: libc::c_int,
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
) -> *const libc::c_char {
    first_nonopt = last_nonopt = optind;
    nextchar = ptr::null();

    let posixly_correct = env::var_os("POSIXLY_CORRECT");
    if !posixly_correct.is_none() {
        ordering = Ordering::RequireOrder;
    }

    if !optstring.is_null() && (*optstring == '-' as libc::c_char) {
        ordering = Ordering::ReturnInOrder;
        optstring.offset(1)
    } else if !optstring.is_null() && (*optstring == '+' as libc::c_char) {
        ordering = Ordering::RequireOrder;
        optstring.offset(1)
    } else {
        optstring
    }
}

pub unsafe fn getopt(
    argc: libc::c_int,
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
) -> libc::c_int {
    _getopt_internal(argc, argv, optstring, ptr::null(), ptr::null_mut(), 0)
}

pub unsafe fn getopt_long(
    argc: libc::c_int,
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const Option,
    longind: *mut libc::c_int,
) -> libc::c_int {
    _getopt_internal(argc, argv, optstring, longopts, longind, 0)
}

pub unsafe fn getopt_long_only(
    argc: libc::c_int,
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const Option,
    longind: *mut libc::c_int,
) -> libc::c_int {
    _getopt_internal(argc, argv, optstring, longopts, longind, 1)
}

unsafe fn _getopt_internal(
    argc: libc::c_int,
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const Option,
    longind: *mut libc::c_int,
    long_only: libc::c_int,
) -> libc::c_int {
    optarg = ptr::null_mut();

    if optind == 0 || __getopt_initialized == 0 {
        if optind == 0 {
            optind = 1;
        }
        _getopt_initialize(argc, argv, optstring);
        __getopt_initialized = 1;
    }

    if nextchar.is_null() || *nextchar == '\0' as libc::c_char {
        if last_nonopt > optind {
            last_nonopt = optind;
        }
        if first_nonopt > optind {
            first_nonopt = optind;
        }

        if ordering == Ordering::Permute {
            if first_nonopt != last_nonopt && last_nonopt != optind {
                exchange(argv);
            } else if last_nonopt != optind {
                first_nonopt = optind;
            }

            while optind < argc {
                let arg = *argv.offset(optind as isize);
                if arg.is_null() || *arg != '-' as libc::c_char || *arg.offset(1) == '\0' as libc::c_char {
                    break;
                }
                optind += 1;
            }
            last_nonopt = optind;
        }

        if optind != argc {
            let arg = *argv.offset(optind as isize);
            if !arg.is_null() && libc::strcmp(arg, b"--\0".as_ptr() as *const libc::c_char) == 0 {
                optind += 1;
                if first_nonopt != last_nonopt && last_nonopt != optind {
                    exchange(argv);
                } else if first_nonopt == last_nonopt {
                    first_nonopt = optind;
                }
                last_nonopt = argc;
                optind = argc;
            }
        }

        if optind == argc {
            if first_nonopt != last_nonopt {
                optind = first_nonopt;
            }
            return -1;
        }

        let arg = *argv.offset(optind as isize);
        if arg.is_null() || *arg != '-' as libc::c_char || *arg.offset(1) == '\0' as libc::c_char {
            if ordering == Ordering::RequireOrder {
                return -1;
            }
            optarg = *argv.offset(optind as isize);
            optind += 1;
            return 1;
        }

        nextchar = arg.offset(1);
        if !longopts.is_null() && *arg.offset(1) == '-' as libc::c_char {
            nextchar = arg.offset(2);
        }
    }

    if !longopts.is_null()
        && (*argv.offset(optind as isize).offset(1) == '-' as libc::c_char
            || (long_only != 0
                && (argv.offset(optind as isize).offset(2) != ptr::null_mut()
                    || libc::strchr(optstring, *argv.offset(optind as isize).offset(1)).is_null()))
    {
        let mut nameend = nextchar;
        while *nameend != '\0' as libc::c_char && *nameend != '=' as libc::c_char {
            nameend = nameend.offset(1);
        }

        let mut pfound: *const Option = ptr::null();
        let mut exact = 0;
        let mut ambig = 0;
        let mut indfound = -1;
        let mut option_index = 0;

        let mut p = longopts;
        while !(*p).name.is_empty() {
            if libc::strncmp(
                (*p).name.as_ptr() as *const libc::c_char,
                nextchar,
                nameend.offset_from(nextchar) as usize,
            ) == 0
            {
                if (nameend.offset_from(nextchar) as usize == (*p).name.len() {
                    pfound = p;
                    indfound = option_index;
                    exact = 1;
                    break;
                } else if pfound.is_null() {
                    pfound = p;
                    indfound = option_index;
                } else {
                    ambig = 1;
                }
            }
            p = p.offset(1);
            option_index += 1;
        }

        if ambig != 0 && exact == 0 {
            if opterr != 0 {
                eprintln!(
                    "{}: option '{}' is ambiguous",
                    CString::from_raw(*argv).into_string().unwrap(),
                    CString::from_raw(*argv.offset(optind as isize)).into_string().unwrap()
                );
            }
            nextchar = nextchar.offset(libc::strlen(nextchar) as isize);
            optind += 1;
            optopt = 0;
            return '?' as i32;
        }

        if !pfound.is_null() {
            option_index = indfound;
            optind += 1;
            if *nameend != '\0' as libc::c_char {
                if (*pfound).has_arg != ArgumentRequirement::NoArgument {
                    optarg = nameend.offset(1);
                } else {
                    if opterr != 0 {
                        if *argv.offset(optind - 1).offset(1) == '-' as libc::c_char {
                            eprintln!(
                                "{}: option '--{}' doesn't allow an argument",
                                CString::from_raw(*argv).into_string().unwrap(),
                                (*pfound).name
                            );
                        } else {
                            eprintln!(
                                "{}: option '-{}' doesn't allow an argument",
                                CString::from_raw(*argv).into_string().unwrap(),
                                (*pfound).name
                            );
                        }
                    }
                    nextchar = nextchar.offset(libc::strlen(nextchar) as isize);
                    optopt = (*pfound).val;
                    return '?' as i32;
                }
            } else if (*pfound).has_arg == ArgumentRequirement::RequiredArgument {
                if optind < argc {
                    optarg = *argv.offset(optind as isize);
                    optind += 1;
                } else {
                    if opterr != 0 {
                        eprintln!(
                            "{}: option '{}' requires an argument",
                            CString::from_raw(*argv).into_string().unwrap(),
                            CString::from_raw(*argv.offset(optind - 1)).into_string().unwrap()
                        );
                    }
                    nextchar = nextchar.offset(libc::strlen(nextchar) as isize);
                    optopt = (*pfound).val;
                    return if *optstring == ':' as libc::c_char {
                        ':' as i32
                    } else {
                        '?' as i32
                    };
                }
            }
            nextchar = nextchar.offset(libc::strlen(nextchar) as isize);
            if !longind.is_null() {
                *longind = option_index;
            }
            if !(*pfound).flag.is_none() {
                *(*pfound).flag.unwrap() = (*pfound).val;
                return 0;
            }
            return (*pfound).val;
        }

        if long_only == 0
            || *argv.offset(optind as isize).offset(1) == '-' as libc::c_char
            || libc::strchr(optstring, *nextchar).is_null()
        {
            if opterr != 0 {
                if *argv.offset(optind as isize).offset(1) == '-' as libc::c_char {
                    eprintln!(
                        "{}: unrecognized option '--{}'",
                        CString::from_raw(*argv).into_string().unwrap(),
                        CString::from_raw(nextchar as *mut libc::c_char).into_string().unwrap()
                    );
                } else {
                    eprintln!(
                        "{}: unrecognized option '-{}'",
                        CString::from_raw(*argv).into_string().unwrap(),
                        CString::from_raw(nextchar as *mut libc::c_char).into_string().unwrap()
                    );
                }
            }
            nextchar = b"\0".as_ptr() as *const libc::c_char;
            optind += 1;
            optopt = 0;
            return '?' as i32;
        }
    }

    let c = *nextchar;
    nextchar = nextchar.offset(1);
    if *nextchar == '\0' as libc::c_char {
        optind += 1;
    }

    let mut temp = libc::strchr(optstring, c);
    if temp.is_null() || c == ':' as libc::c_char {
        if opterr != 0 {
            eprintln!(
                "{}: invalid option -- '{}'",
                CString::from_raw(*argv).into_string().unwrap(),
                c as u8 as char
            );
        }
        optopt = c as i32;
        return '?' as i32;
    }

    if *temp.offset(1) == ':' as libc::c_char {
        if *temp.offset(2) == ':' as libc::c_char {
            if *nextchar != '\0' as libc::c_char {
                optarg = nextchar as *mut libc::c_char;
                optind += 1;
            } else {
                optarg = ptr::null_mut();
            }
            nextchar = ptr::null();
        } else {
            if *nextchar != '\0' as libc::c_char {
                optarg = nextchar as *mut libc::c_char;
                optind += 1;
            } else if optind == argc {
                if opterr != 0 {
                    eprintln!(
                        "{}: option requires an argument -- '{}'",
                        CString::from_raw(*argv).into_string().unwrap(),
                        c as u8 as char
                    );
                }
                optopt = c as i32;
                return if *optstring == ':' as libc::c_char {
                    ':' as i32
                } else {
                    '?' as i32
                };
            } else {
                optarg = *argv.offset(optind as isize);
                optind += 1;
            }
            nextchar = ptr::null();
        }
    }
    c as i32
}