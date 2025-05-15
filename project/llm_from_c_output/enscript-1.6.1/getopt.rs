use std::env;
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int};
use std::ptr;
use std::str;

#[repr(C)]
pub struct Option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

pub static mut optarg: *mut c_char = ptr::null_mut();
pub static mut optind: c_int = 1;
pub static mut opterr: c_int = 1;
pub static mut optopt: c_int = '?' as c_int;

static mut nextchar: *const c_char = ptr::null();
static mut first_nonopt: c_int = 0;
static mut last_nonopt: c_int = 0;

#[derive(PartialEq)]
enum Ordering {
    RequireOrder,
    Permute,
    ReturnInOrder,
}

static mut ordering: Ordering = Ordering::Permute;

unsafe fn my_index(s: *const c_char, c: c_char) -> *const c_char {
    let mut p = s;
    while *p != 0 {
        if *p == c {
            return p;
        }
        p = p.offset(1);
    }
    ptr::null()
}

unsafe fn exchange(argv: *mut *mut c_char) {
    let mut bottom = first_nonopt;
    let mut middle = last_nonopt;
    let mut top = optind;

    while top > middle && middle > bottom {
        if top - middle > middle - bottom {
            let len = middle - bottom;
            for i in 0..len {
                let tem = *argv.offset((bottom + i) as isize);
                *argv.offset((bottom + i) as isize) = *argv.offset((top - (middle - bottom) + i) as isize);
                *argv.offset((top - (middle - bottom) + i) as isize) = tem;
            }
            top -= len;
        } else {
            let len = top - middle;
            for i in 0..len {
                let tem = *argv.offset((bottom + i) as isize);
                *argv.offset((bottom + i) as isize) = *argv.offset((middle + i) as isize);
                *argv.offset((middle + i) as isize) = tem;
            }
            bottom += len;
        }
    }

    first_nonopt += optind - last_nonopt;
    last_nonopt = optind;
}

unsafe fn _getopt_initialize(
    argc: c_int,
    argv: *mut *mut c_char,
    optstring: *const c_char,
) -> *const c_char {
    first_nonopt = 1;
    last_nonopt = 1;
    optind = 1;
    nextchar = ptr::null();

    let posixly_correct = env::var("POSIXLY_CORRECT").ok();

    if *optstring == '-' as c_char {
        ordering = Ordering::ReturnInOrder;
        optstring.offset(1)
    } else if *optstring == '+' as c_char {
        ordering = Ordering::RequireOrder;
        optstring.offset(1)
    } else if posixly_correct.is_some() {
        ordering = Ordering::RequireOrder;
        optstring
    } else {
        ordering = Ordering::Permute;
        optstring
    }
}

#[no_mangle]
pub unsafe extern "C" fn getopt(
    argc: c_int,
    argv: *mut *mut c_char,
    optstring: *const c_char,
) -> c_int {
    _getopt_internal(argc, argv, optstring, ptr::null(), ptr::null_mut(), 0)
}

#[no_mangle]
pub unsafe extern "C" fn getopt_long(
    argc: c_int,
    argv: *mut *mut c_char,
    optstring: *const c_char,
    longopts: *const Option,
    longind: *mut c_int,
) -> c_int {
    _getopt_internal(argc, argv, optstring, longopts, longind, 0)
}

#[no_mangle]
pub unsafe extern "C" fn getopt_long_only(
    argc: c_int,
    argv: *mut *mut c_char,
    optstring: *const c_char,
    longopts: *const Option,
    longind: *mut c_int,
) -> c_int {
    _getopt_internal(argc, argv, optstring, longopts, longind, 1)
}

unsafe fn _getopt_internal(
    argc: c_int,
    argv: *mut *mut c_char,
    optstring: *const c_char,
    longopts: *const Option,
    longind: *mut c_int,
    long_only: c_int,
) -> c_int {
    optarg = ptr::null_mut();

    if optind == 0 {
        optstring = _getopt_initialize(argc, argv, optstring);
        optind = 1;
    }

    if nextchar.is_null() || *nextchar == 0 {
        if ordering == Ordering::Permute {
            if first_nonopt != last_nonopt && last_nonopt != optind {
                exchange(argv);
            } else if last_nonopt != optind {
                first_nonopt = optind;
            }

            while optind < argc {
                let arg = *argv.offset(optind as isize);
                if *arg != '-' as c_char || *arg.offset(1) == 0 {
                    optind += 1;
                } else {
                    break;
                }
            }
            last_nonopt = optind;
        }

        if optind != argc && strcmp(*argv.offset(optind as isize), c_str!("--")) == 0 {
            optind += 1;
            if first_nonopt != last_nonopt && last_nonopt != optind {
                exchange(argv);
            } else if first_nonopt == last_nonopt {
                first_nonopt = optind;
            }
            last_nonopt = argc;
            optind = argc;
        }

        if optind == argc {
            if first_nonopt != last_nonopt {
                optind = first_nonopt;
            }
            return -1;
        }

        let arg = *argv.offset(optind as isize);
        if *arg != '-' as c_char || *arg.offset(1) == 0 {
            if ordering == Ordering::RequireOrder {
                return -1;
            }
            optarg = *argv.offset(optind as isize);
            optind += 1;
            return 1;
        }

        nextchar = arg.offset(1);
        if longopts.as_ref().is_some() && (*arg.offset(1) == '-' as c_char {
            nextchar = arg.offset(2);
        }
    }

    if !longopts.is_null()
        && ((*argv.offset(optind as isize)).offset(1) == '-' as c_char
        || (long_only != 0
            && ((*argv.offset(optind as isize)).offset(2) != 0
            || my_index(optstring, *nextchar).is_null())
    {
        let mut nameend = nextchar;
        while *nameend != 0 && *nameend != '=' as c_char {
            nameend = nameend.offset(1);
        }

        let mut pfound: *const Option = ptr::null();
        let mut exact = 0;
        let mut ambig = 0;
        let mut indfound = 0;

        let mut p = longopts;
        let mut option_index = 0;
        while !(*p).name.is_null() {
            if strncmp(
                (*p).name,
                nextchar,
                nameend.offset_from(nextchar) as usize,
            ) == 0
            {
                if (nameend.offset_from(nextchar) as usize == strlen((*p).name) {
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
                    c_str_to_str(*argv),
                    c_str_to_str(*argv.offset(optind as isize))
            );
            }
            nextchar = nextchar.add(strlen(nextchar));
            optind += 1;
            optopt = 0;
            return '?' as c_int;
        }

        if !pfound.is_null() {
            option_index = indfound;
            optind += 1;
            if *nameend != 0 {
                if (*pfound).has_arg != 0 {
                    optarg = nameend.offset(1) as *mut c_char;
                } else {
                    if opterr != 0 {
                        if (*argv.offset((optind - 1) as isize)).offset(1) == '-' as c_char {
                            eprintln!(
                                "{}: option '--{}' doesn't allow an argument",
                                c_str_to_str(*argv),
                                c_str_to_str((*pfound).name)
                            );
                        } else {
                            eprintln!(
                                "{}: option '-{}' doesn't allow an argument",
                                c_str_to_str(*argv),
                                *(*argv.offset((optind - 1) as isize)),
                                c_str_to_str((*pfound).name)
                            );
                        }
                    }
                    nextchar = nextchar.add(strlen(nextchar));
                    optopt = (*pfound).val;
                    return '?' as c_int;
                }
            } else if (*pfound).has_arg == 1 {
                if optind < argc {
                    optarg = *argv.offset(optind as isize);
                    optind += 1;
                } else {
                    if opterr != 0 {
                        eprintln!(
                            "{}: option '{}' requires an argument",
                            c_str_to_str(*argv),
                            c_str_to_str(*argv.offset((optind - 1) as isize))
                        );
                    }
                    nextchar = nextchar.add(strlen(nextchar));
                    optopt = (*pfound).val;
                    return if *optstring == ':' as c_char {
                        ':' as c_int
                    } else {
                        '?' as c_int
                    };
                }
            }
            nextchar = nextchar.add(strlen(nextchar));
            if !longind.is_null() {
                *longind = option_index as c_int;
            }
            if !(*pfound).flag.is_null() {
                *(*pfound).flag = (*pfound).val;
                return 0;
            }
            return (*pfound).val;
        }

        if long_only == 0
            || (*argv.offset(optind as isize)).offset(1) == '-' as c_char
            || my_index(optstring, *nextchar).is_null()
        {
            if opterr != 0 {
                if (*argv.offset(optind as isize)).offset(1) == '-' as c_char {
                    eprintln!(
                        "{}: unrecognized option '--{}'",
                        c_str_to_str(*argv),
                        c_str_to_str(nextchar)
                    );
                } else {
                    eprintln!(
                        "{}: unrecognized option '-{}'",
                        c_str_to_str(*argv),
                        *(*argv.offset(optind as isize)),
                        c_str_to_str(nextchar)
                    );
                }
            }
            nextchar = c_str!("");
            optind += 1;
            optopt = 0;
            return '?' as c_int;
        }
    }

    let c = *nextchar;
    nextchar = nextchar.offset(1);
    if *nextchar == 0 {
        optind += 1;
    }

    let temp = my_index(optstring, c);
    if temp.is_null() || c == ':' as c_char {
        if opterr != 0 {
            eprintln!(
                "{}: invalid option -- '{}'",
                c_str_to_str(*argv),
                c as char
            );
        }
        optopt = c;
        return '?' as c_int;
    }

    if *temp.offset(1) == ':' as c_char {
        if *temp.offset(2) == ':' as c_char {
            if *nextchar != 0 {
                optarg = nextchar as *mut c_char;
                optind += 1;
            } else {
                optarg = ptr::null_mut();
            }
            nextchar = ptr::null();
        } else {
            if *nextchar != 0 {
                optarg = nextchar as *mut c_char;
                optind += 1;
            } else if optind == argc {
                if opterr != 0 {
                    eprintln!(
                        "{}: option requires an argument -- '{}'",
                        c_str_to_str(*argv),
                        c as char
                    );
                }
                optopt = c;
                return if *optstring == ':' as c_char {
                    ':' as c_int
                } else {
                    '?' as c_int
                };
            } else {
                optarg = *argv.offset(optind as isize);
                optind += 1;
            }
            nextchar = ptr::null();
        }
    }
    c as c_int
}

// Helper functions
fn c_str(s: &str) -> *const c_char {
    CString::new(s).unwrap().into_raw()
}

unsafe fn c_str_to_str(s: *const c_char) -> &'static str {
    str::from_utf8_unchecked(CStr::from_ptr(s).to_bytes())
}

unsafe fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int {
    let mut p1 = s1;
    let mut p2 = s2;
    while *p1 != 0 && *p2 != 0 && *p1 == *p2 {
        p1 = p1.offset(1);
        p2 = p2.offset(1);
    }
    (*p1 - *p2) as c_int
}

unsafe fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int {
    let mut p1 = s1;
    let mut p2 = s2;
    let mut i = 0;
    while *p1 != 0 && *p2 != 0 && *p1 == *p2 && i < n {
        p1 = p1.offset(1);
        p2 = p2.offset(1);
        i += 1;
    }
    if i == n {
        0
    } else {
        (*p1 - *p2) as c_int
    }
}

unsafe fn strlen(s: *const c_char) -> usize {
    let mut p = s;
    while *p != 0 {
        p = p.offset(1);
    }
    p.offset_from(s) as usize
}