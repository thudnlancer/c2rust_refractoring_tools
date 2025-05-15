//! GNU getopt implementation in Rust

use std::env;
use std::ffi::{CString, OsString};
use std::os::unix::ffi::OsStringExt;
use std::ptr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArgumentRequirement {
    NoArgument,
    RequiredArgument,
    OptionalArgument,
}

#[derive(Debug, Clone)]
pub struct Option {
    pub name: String,
    pub has_arg: ArgumentRequirement,
    pub flag: Option<*mut i32>,
    pub val: i32,
}

pub static mut optarg: *mut libc::c_char = ptr::null_mut();
pub static mut optind: libc::c_int = 1;
pub static mut opterr: libc::c_int = 1;
pub static mut optopt: libc::c_int = '?' as libc::c_int;

struct GetoptData {
    __first_nonopt: libc::c_int,
    __last_nonopt: libc::c_int,
    __nextchar: *const libc::c_char,
    __ordering: Ordering,
    __posixly_correct: bool,
    __initialized: bool,
    __nonoption_flags_len: libc::c_int,
    __nonoption_flags_max_len: libc::c_int,
}

impl Default for GetoptData {
    fn default() -> Self {
        GetoptData {
            __first_nonopt: 0,
            __last_nonopt: 0,
            __nextchar: ptr::null(),
            __ordering: Ordering::Permute,
            __posixly_correct: false,
            __initialized: false,
            __nonoption_flags_len: 0,
            __nonoption_flags_max_len: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Ordering {
    Permute,
    ReturnInOrder,
    RequireOrder,
}

static mut getopt_data: GetoptData = GetoptData {
    __first_nonopt: 0,
    __last_nonopt: 0,
    __nextchar: ptr::null(),
    __ordering: Ordering::Permute,
    __posixly_correct: false,
    __initialized: false,
    __nonoption_flags_len: 0,
    __nonoption_flags_max_len: 0,
};

fn exchange(argv: *mut *mut libc::c_char, d: &mut GetoptData) {
    let mut bottom = d.__first_nonopt;
    let mut middle = d.__last_nonopt;
    let mut top = d.optind;
    let mut tem: *mut libc::c_char;

    while top > middle && middle > bottom {
        if top - middle > middle - bottom {
            let len = middle - bottom;
            for i in 0..len {
                unsafe {
                    tem = *argv.offset((bottom + i) as isize);
                    *argv.offset((bottom + i) as isize) =
                        *argv.offset((top - (middle - bottom) + i as isize);
                    *argv.offset((top - (middle - bottom) + i as isize) = tem;
                }
            }
            top -= len;
        } else {
            let len = top - middle;
            for i in 0..len {
                unsafe {
                    tem = *argv.offset((bottom + i) as isize);
                    *argv.offset((bottom + i) as isize) =
                        *argv.offset((middle + i) as isize);
                    *argv.offset((middle + i) as isize) = tem;
                }
            }
            bottom += len;
        }
    }

    d.__first_nonopt += d.optind - d.__last_nonopt;
    d.__last_nonopt = d.optind;
}

fn _getopt_initialize(
    argc: libc::c_int,
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    d: &mut GetoptData,
    posixly_correct: bool,
) -> *const libc::c_char {
    d.__first_nonopt = d.optind;
    d.__last_nonopt = d.optind;
    d.__nextchar = ptr::null();

    d.__posixly_correct = posixly_correct || env::var("POSIXLY_CORRECT").is_ok();

    let mut optstring = optstring;
    unsafe {
        if *optstring == '-' as libc::c_char {
            d.__ordering = Ordering::ReturnInOrder;
            optstring = optstring.offset(1);
        } else if *optstring == '+' as libc::c_char {
            d.__ordering = Ordering::RequireOrder;
            optstring = optstring.offset(1);
        } else if d.__posixly_correct {
            d.__ordering = Ordering::RequireOrder;
        } else {
            d.__ordering = Ordering::Permute;
        }
    }

    optstring
}

fn _getopt_internal_r(
    argc: libc::c_int,
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const Option,
    longind: *mut libc::c_int,
    long_only: bool,
    d: &mut GetoptData,
    posixly_correct: bool,
) -> libc::c_int {
    let print_errors = d.opterr != 0;

    if argc < 1 {
        return -1;
    }

    unsafe {
        d.optarg = ptr::null_mut();

        if d.optind == 0 || !d.__initialized {
            if d.optind == 0 {
                d.optind = 1;
            }
            let optstring = _getopt_initialize(argc, argv, optstring, d, posixly_correct);
            d.__initialized = true;
        } else if *optstring == '-' as libc::c_char || *optstring == '+' as libc::c_char {
            optstring = optstring.offset(1);
        }
        if *optstring == ':' as libc::c_char {
            print_errors = false;
        }

        if d.__nextchar.is_null() || *d.__nextchar == '\0' as libc::c_char {
            if d.__last_nonopt > d.optind {
                d.__last_nonopt = d.optind;
            }
            if d.__first_nonopt > d.optind {
                d.__first_nonopt = d.optind;
            }

            if d.__ordering == Ordering::Permute {
                if d.__first_nonopt != d.__last_nonopt && d.__last_nonopt != d.optind {
                    exchange(argv, d);
                } else if d.__last_nonopt != d.optind {
                    d.__first_nonopt = d.optind;
                }

                while d.optind < argc {
                    let arg = *argv.offset(d.optind as isize);
                    if *arg != '-' as libc::c_char || *arg.offset(1) == '\0' as libc::c_char {
                        d.optind += 1;
                    } else {
                        break;
                    }
                }
                d.__last_nonopt = d.optind;
            }

            if d.optind != argc && libc::strcmp(*argv.offset(d.optind as isize), c_str!("--")) == 0 {
                d.optind += 1;

                if d.__first_nonopt != d.__last_nonopt && d.__last_nonopt != d.optind {
                    exchange(argv, d);
                } else if d.__first_nonopt == d.__last_nonopt {
                    d.__first_nonopt = d.optind;
                }
                d.__last_nonopt = argc;

                d.optind = argc;
            }

            if d.optind == argc {
                if d.__first_nonopt != d.__last_nonopt {
                    d.optind = d.__first_nonopt;
                }
                return -1;
            }

            let arg = *argv.offset(d.optind as isize);
            if *arg != '-' as libc::c_char || *arg.offset(1) == '\0' as libc::c_char {
                if d.__ordering == Ordering::RequireOrder {
                    return -1;
                }
                d.optarg = *argv.offset(d.optind as isize);
                d.optind += 1;
                return 1;
            }

            d.__nextchar = arg.offset(1);
            if longopts.is_null() || (*arg.offset(1) != '-' as libc::c_char
                || *arg.offset(2) == '\0' as libc::c_char
            {
                if !long_only
                    || (*arg.offset(2) != '\0' as libc::c_char
                    || libc::strchr(optstring, *d.__nextchar).is_null()
                {
                    let c = *d.__nextchar;
                    d.__nextchar = c_str!("");
                    d.optind += 1;
                    d.optopt = c;
                    if print_errors {
                        eprintln!(
                            "{}: invalid option -- '{}'",
                            CStr::from_ptr(*argv).to_str().unwrap(),
                            c as char
                        );
                    }
                    return '?' as libc::c_int;
                }
            }
        }

        if !longopts.is_null()
            && ((*argv.offset(d.optind as isize)).offset(1) == '-' as libc::c_char
                || (long_only
                    && ((*argv.offset(d.optind as isize)).offset(2) != '\0' as libc::c_char
                        || libc::strchr(optstring, *d.__nextchar).is_null())))
        {
            let mut nameend = d.__nextchar;
            while *nameend != '\0' as libc::c_char && *nameend != '=' as libc::c_char {
                nameend = nameend.offset(1);
            }
            let namelen = nameend as usize - d.__nextchar as usize;

            let mut pfound: *const Option = ptr::null();
            let mut indfound: libc::c_int = 0;
            let mut exact = false;
            let mut ambig = false;

            let mut p = longopts;
            let mut option_index = 0;
            while !(*p).name.is_empty() {
                if libc::strncmp(
                    (*p).name.as_ptr() as *const libc::c_char,
                    d.__nextchar,
                    namelen,
                ) == 0
                {
                    if namelen == (*p).name.len() {
                        pfound = p;
                        indfound = option_index;
                        exact = true;
                        break;
                    } else if pfound.is_null() {
                        pfound = p;
                        indfound = option_index;
                    } else {
                        ambig = true;
                    }
                }
                p = p.offset(1);
                option_index += 1;
            }

            if ambig && !exact {
                if print_errors {
                    eprintln!(
                        "{}: option '--{}' is ambiguous",
                        CStr::from_ptr(*argv).to_str().unwrap(),
                        CStr::from_ptr(d.__nextchar).to_str().unwrap()
                    );
                }
                d.__nextchar = libc::strchr(d.__nextchar, '\0' as libc::c_char);
                d.optind += 1;
                d.optopt = 0;
                return '?' as libc::c_int;
            }

            if !pfound.is_null() {
                option_index = indfound;
                d.optind += 1;
                if *nameend != '\0' as libc::c_char {
                    if (*pfound).has_arg != ArgumentRequirement::NoArgument {
                        d.optarg = nameend.offset(1) as *mut libc::c_char;
                    } else {
                        if print_errors {
                            eprintln!(
                                "{}: option '--{}' doesn't allow an argument",
                                CStr::from_ptr(*argv).to_str().unwrap(),
                                (*pfound).name
                            );
                        }
                        d.__nextchar = libc::strchr(d.__nextchar, '\0' as libc::c_char);
                        d.optopt = (*pfound).val;
                        return '?' as libc::c_int;
                    }
                } else if (*pfound).has_arg == ArgumentRequirement::RequiredArgument {
                    if d.optind < argc {
                        d.optarg = *argv.offset(d.optind as isize);
                        d.optind += 1;
                    } else {
                        if print_errors {
                            eprintln!(
                                "{}: option '--{}' requires an argument",
                                CStr::from_ptr(*argv).to_str().unwrap(),
                                (*pfound).name
                            );
                        }
                        d.__nextchar = libc::strchr(d.__nextchar, '\0' as libc::c_char);
                        d.optopt = (*pfound).val;
                        return if *optstring == ':' as libc::c_char {
                            ':' as libc::c_int
                        } else {
                            '?' as libc::c_int
                        };
                    }
                }
                d.__nextchar = libc::strchr(d.__nextchar, '\0' as libc::c_char);
                if !longind.is_null() {
                    *longind = option_index;
                }
                if !(*pfound).flag.is_null() {
                    *(*pfound).flag = (*pfound).val;
                    return 0;
                }
                return (*pfound).val;
            }

            if !long_only
                || (*argv.offset(d.optind as isize)).offset(1) == '-' as libc::c_char
                || libc::strchr(optstring, *d.__nextchar).is_null()
            {
                if print_errors {
                    eprintln!(
                        "{}: unrecognized option '--{}'",
                        CStr::from_ptr(*argv).to_str().unwrap(),
                        CStr::from_ptr(d.__nextchar).to_str().unwrap()
                    );
                }
                d.__nextchar = c_str!("");
                d.optind += 1;
                d.optopt = 0;
                return '?' as libc::c_int;
            }
        }

        let c = *d.__nextchar;
        d.__nextchar = d.__nextchar.offset(1);
        if *d.__nextchar == '\0' as libc::c_char {
            d.optind += 1;
        }

        let temp = libc::strchr(optstring, c);
        if temp.is_null() || c == ':' as libc::c_char || c == ';' as libc::c_char {
            if print_errors {
                eprintln!(
                    "{}: invalid option -- '{}'",
                    CStr::from_ptr(*argv).to_str().unwrap(),
                    c as char
                );
            }
            d.optopt = c;
            return '?' as libc::c_int;
        }

        if *temp.offset(1) == ':' as libc::c_char {
            if *temp.offset(2) == ':' as libc::c_char {
                if *d.__nextchar != '\0' as libc::c_char {
                    d.optarg = d.__nextchar as *mut libc::c_char;
                    d.optind += 1;
                } else {
                    d.optarg = ptr::null_mut();
                }
                d.__nextchar = ptr::null();
            } else {
                if *d.__nextchar != '\0' as libc::c_char {
                    d.optarg = d.__nextchar as *mut libc::c_char;
                    d.optind += 1;
                } else if d.optind == argc {
                    if print_errors {
                        eprintln!(
                            "{}: option requires an argument -- '{}'",
                            CStr::from_ptr(*argv).to_str().unwrap(),
                            c as char
                        );
                    }
                    d.optopt = c;
                    d.__nextchar = ptr::null();
                    return if *optstring == ':' as libc::c_char {
                        ':' as libc::c_int
                    } else {
                        '?' as libc::c_int
                    };
                } else {
                    d.optarg = *argv.offset(d.optind as isize);
                    d.optind += 1;
                }
                d.__nextchar = ptr::null();
            }
        }
        c as libc::c_int
    }
}

pub fn getopt(
    argc: libc::c_int,
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
) -> libc::c_int {
    unsafe {
        getopt_data.optind = optind;
        getopt_data.opterr = opterr;

        let result = _getopt_internal_r(
            argc,
            argv,
            optstring,
            ptr::null(),
            ptr::null_mut(),
            false,
            &mut getopt_data,
            false,
        );

        optind = getopt_data.optind;
        optarg = getopt_data.optarg;
        optopt = getopt_data.optopt;

        result
    }
}

pub fn getopt_long(
    argc: libc::c_int,
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const Option,
    longind: *mut libc::c_int,
) -> libc::c_int {
    unsafe {
        getopt_data.optind = optind;
        getopt_data.opterr = opterr;

        let result = _getopt_internal_r(
            argc,
            argv,
            optstring,
            longopts,
            longind,
            false,
            &mut getopt_data,
            false,
        );

        optind = getopt_data.optind;
        optarg = getopt_data.optarg;
        optopt = getopt_data.optopt;

        result
    }
}

pub fn getopt_long_only(
    argc: libc::c_int,
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longop