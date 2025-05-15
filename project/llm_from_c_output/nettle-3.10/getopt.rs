use std::env;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;

#[repr(C)]
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

pub static mut optarg: *mut c_char = ptr::null_mut();
pub static mut optind: c_int = 1;
pub static mut opterr: c_int = 1;
pub static mut optopt: c_int = '?' as c_int;

const no_argument: c_int = 0;
const required_argument: c_int = 1;
const optional_argument: c_int = 2;

struct GetoptData {
    __nextchar: *mut c_char,
    __first_nonopt: c_int,
    __last_nonopt: c_int,
    __ordering: Ordering,
    __posixly_correct: bool,
    __initialized: bool,
    __nonoption_flags_len: c_int,
    __nonoption_flags_max_len: c_int,
}

impl Default for GetoptData {
    fn default() -> Self {
        GetoptData {
            __nextchar: ptr::null_mut(),
            __first_nonopt: 0,
            __last_nonopt: 0,
            __ordering: Ordering::Permute,
            __posixly_correct: false,
            __initialized: false,
            __nonoption_flags_len: 0,
            __nonoption_flags_max_len: 0,
        }
    }
}

#[derive(PartialEq)]
enum Ordering {
    Permute,
    ReturnInOrder,
    RequireOrder,
}

static mut getopt_data: GetoptData = GetoptData {
    __nextchar: ptr::null_mut(),
    __first_nonopt: 0,
    __last_nonopt: 0,
    __ordering: Ordering::Permute,
    __posixly_correct: false,
    __initialized: false,
    __nonoption_flags_len: 0,
    __nonoption_flags_max_len: 0,
};

unsafe fn exchange(argv: *mut *mut c_char, d: *mut GetoptData) {
    let mut bottom = (*d).__first_nonopt;
    let mut middle = (*d).__last_nonopt;
    let mut top = (*d).optind;
    let mut tem: *mut c_char;

    while top > middle && middle > bottom {
        if top - middle > middle - bottom {
            let len = middle - bottom;
            for i in 0..len {
                tem = *argv.offset((bottom + i) as isize);
                *argv.offset((bottom + i) as isize) =
                    *argv.offset((top - (middle - bottom) + i) as isize);
                *argv.offset((top - (middle - bottom) + i) as isize) = tem;
            }
            top -= len;
        } else {
            let len = top - middle;
            for i in 0..len {
                tem = *argv.offset((bottom + i) as isize);
                *argv.offset((bottom + i) as isize) =
                    *argv.offset((middle + i) as isize);
                *argv.offset((middle + i) as isize) = tem;
            }
            bottom += len;
        }
    }

    (*d).__first_nonopt += ((*d).optind - (*d).__last_nonopt);
    (*d).__last_nonopt = (*d).optind;
}

unsafe fn _getopt_initialize(
    argc: c_int,
    argv: *mut *mut c_char,
    optstring: *const c_char,
    d: *mut GetoptData,
    posixly_correct: bool,
) -> *const c_char {
    (*d).__first_nonopt = (*d).__last_nonopt = (*d).optind;
    (*d).__nextchar = ptr::null_mut();

    (*d).__posixly_correct = posixly_correct || !env::var("POSIXLY_CORRECT").is_err();

    if *optstring == '-' as i8 {
        (*d).__ordering = Ordering::ReturnInOrder;
        optstring.offset(1)
    } else if *optstring == '+' as i8 {
        (*d).__ordering = Ordering::RequireOrder;
        optstring.offset(1)
    } else if (*d).__posixly_correct {
        (*d).__ordering = Ordering::RequireOrder;
        optstring
    } else {
        (*d).__ordering = Ordering::Permute;
        optstring
    }
}

unsafe fn _getopt_internal_r(
    argc: c_int,
    argv: *mut *mut c_char,
    optstring: *const c_char,
    longopts: *const option,
    longind: *mut c_int,
    long_only: bool,
    d: *mut GetoptData,
    posixly_correct: bool,
) -> c_int {
    let mut print_errors = (*d).opterr;

    if argc < 1 {
        return -1;
    }

    (*d).optarg = ptr::null_mut();

    if (*d).optind == 0 || !(*d).__initialized {
        if (*d).optind == 0 {
            (*d).optind = 1;
        }
        let new_optstring = _getopt_initialize(argc, argv, optstring, d, posixly_correct);
        (*d).__initialized = true;
        optstring = new_optstring;
    } else if *optstring == '-' as i8 || *optstring == '+' as i8 {
        optstring = optstring.offset(1);
    }
    if *optstring == ':' as i8 {
        print_errors = 0;
    }

    if (*d).__nextchar.is_null() || *(*d).__nextchar == '\0' as i8 {
        if (*d).__last_nonopt > (*d).optind {
            (*d).__last_nonopt = (*d).optind;
        }
        if (*d).__first_nonopt > (*d).optind {
            (*d).__first_nonopt = (*d).optind;
        }

        if (*d).__ordering == Ordering::Permute {
            if (*d).__first_nonopt != (*d).__last_nonopt && (*d).__last_nonopt != (*d).optind {
                exchange(argv, d);
            } else if (*d).__last_nonopt != (*d).optind {
                (*d).__first_nonopt = (*d).optind;
            }

            while (*d).optind < argc
                && (*(*argv.offset((*d).optind as isize)).offset(0) != '-' as i8
                || *(*argv.offset((*d).optind as isize)).offset(1) == '\0' as i8
            {
                (*d).optind += 1;
            }
            (*d).__last_nonopt = (*d).optind;
        }

        if (*d).optind != argc
            && CStr::from_ptr(*argv.offset((*d).optind as isize))
                == CStr::from_bytes_with_nul(b"--\0").unwrap()
        {
            (*d).optind += 1;

            if (*d).__first_nonopt != (*d).__last_nonopt && (*d).__last_nonopt != (*d).optind {
                exchange(argv, d);
            } else if (*d).__first_nonopt == (*d).__last_nonopt {
                (*d).__first_nonopt = (*d).optind;
            }
            (*d).__last_nonopt = argc;

            (*d).optind = argc;
        }

        if (*d).optind == argc {
            if (*d).__first_nonopt != (*d).__last_nonopt {
                (*d).optind = (*d).__first_nonopt;
            }
            return -1;
        }

        if (*(*argv.offset((*d).optind as isize)).offset(0) != '-' as i8
            || *(*argv.offset((*d).optind as isize)).offset(1) == '\0' as i8
        {
            if (*d).__ordering == Ordering::RequireOrder {
                return -1;
            }
            (*d).optarg = *argv.offset((*d).optind as isize);
            (*d).optind += 1;
            return 1;
        }

        (*d).__nextchar = (*argv.offset((*d).optind as isize)).offset(1)
            + if !longopts.is_null()
                && *(*argv.offset((*d).optind as isize)).offset(1) == '-' as i8
            {
                1
            } else {
                0
            };
    }

    if !longopts.is_null()
        && (*(*argv.offset((*d).optind as isize)).offset(1) == '-' as i8
            || (long_only
                && (*(*argv.offset((*d).optind as isize)).offset(2) != '\0' as i8
                || libc::strchr(optstring, *(*argv.offset((*d).optind as isize)).offset(1)).is_null())
    {
        let mut nameend = (*d).__nextchar;
        while *nameend != '\0' as i8 && *nameend != '=' as i8 {
            nameend = nameend.offset(1);
        }
        let namelen = nameend.offset_from((*d).__nextchar) as usize;

        let mut pfound: *const option = ptr::null();
        let mut exact = 0;
        let mut ambig = 0;
        let mut indfound = 0;
        let mut option_index = 0;

        let mut p = longopts;
        while !(*p).name.is_null() {
            if libc::strncmp((*p).name, (*d).__nextchar, namelen) == 0 {
                if namelen == libc::strlen((*p).name) {
                    pfound = p;
                    indfound = option_index;
                    exact = 1;
                    break;
                } else if pfound.is_null() {
                    pfound = p;
                    indfound = option_index;
                } else if long_only
                    || (*pfound).has_arg != (*p).has_arg
                    || (*pfound).flag != (*p).flag
                    || (*pfound).val != (*p).val
                {
                    ambig = 1;
                }
            }
            p = p.offset(1);
            option_index += 1;
        }

        if ambig != 0 && exact == 0 {
            if print_errors != 0 {
                eprintln!(
                    "{}: option '{}' is ambiguous",
                    CStr::from_ptr(*argv).to_str().unwrap(),
                    CStr::from_ptr(*argv.offset((*d).optind as isize))
                        .to_str()
                        .unwrap()
                );
            }
            (*d).__nextchar = libc::strchr((*d).__nextchar, '\0' as i32);
            (*d).optind += 1;
            (*d).optopt = 0;
            return '?' as i32;
        }

        if !pfound.is_null() {
            option_index = indfound;
            (*d).optind += 1;
            if *nameend != '\0' as i8 {
                if (*pfound).has_arg != 0 {
                    (*d).optarg = nameend.offset(1);
                } else {
                    if print_errors != 0 {
                        if *(*argv.offset((*d).optind - 1)).offset(1) == '-' as i8 {
                            eprintln!(
                                "{}: option '--{}' doesn't allow an argument",
                                CStr::from_ptr(*argv).to_str().unwrap(),
                                CStr::from_ptr((*pfound).name).to_str().unwrap()
                            );
                        } else {
                            eprintln!(
                                "{}: option '-{}' doesn't allow an argument",
                                CStr::from_ptr(*argv).to_str().unwrap(),
                                CStr::from_ptr((*pfound).name).to_str().unwrap()
                            );
                        }
                    }
                    (*d).__nextchar = libc::strchr((*d).__nextchar, '\0' as i32);
                    (*d).optopt = (*pfound).val;
                    return '?' as i32;
                }
            } else if (*pfound).has_arg == 1 {
                if (*d).optind < argc {
                    (*d).optarg = *argv.offset((*d).optind as isize);
                    (*d).optind += 1;
                } else {
                    if print_errors != 0 {
                        eprintln!(
                            "{}: option '--{}' requires an argument",
                            CStr::from_ptr(*argv).to_str().unwrap(),
                            CStr::from_ptr((*pfound).name).to_str().unwrap()
                        );
                    }
                    (*d).__nextchar = libc::strchr((*d).__nextchar, '\0' as i32);
                    (*d).optopt = (*pfound).val;
                    return if *optstring == ':' as i8 { ':' as i32 } else { '?' as i32 };
                }
            }
            (*d).__nextchar = libc::strchr((*d).__nextchar, '\0' as i32);
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
            || *(*argv.offset((*d).optind as isize)).offset(1) == '-' as i8
            || libc::strchr(optstring, *(*d).__nextchar).is_null()
        {
            if print_errors != 0 {
                if *(*argv.offset((*d).optind as isize)).offset(1) == '-' as i8 {
                    eprintln!(
                        "{}: unrecognized option '--{}'",
                        CStr::from_ptr(*argv).to_str().unwrap(),
                        CStr::from_ptr((*d).__nextchar).to_str().unwrap()
                    );
                } else {
                    eprintln!(
                        "{}: unrecognized option '-{}'",
                        CStr::from_ptr(*argv).to_str().unwrap(),
                        CStr::from_ptr((*d).__nextchar).to_str().unwrap()
                    );
                }
            }
            (*d).__nextchar = CString::new("").unwrap().into_raw();
            (*d).optind += 1;
            (*d).optopt = 0;
            return '?' as i32;
        }
    }

    let c = *(*d).__nextchar;
    (*d).__nextchar = (*d).__nextchar.offset(1);
    if *(*d).__nextchar == '\0' as i8 {
        (*d).optind += 1;
    }

    let mut temp = libc::strchr(optstring, c as i32);
    if temp.is_null() || c == ':' as i8 || c == ';' as i8 {
        if print_errors != 0 {
            eprintln!(
                "{}: invalid option -- '{}'",
                CStr::from_ptr(*argv).to_str().unwrap(),
                c as u8 as char
            );
        }
        (*d).optopt = c as i32;
        return '?' as i32;
    }

    if *temp.offset(1) == ':' as i8 {
        if *temp.offset(2) == ':' as i8 {
            if *(*d).__nextchar != '\0' as i8 {
                (*d).optarg = (*d).__nextchar;
                (*d).optind += 1;
            } else {
                (*d).optarg = ptr::null_mut();
            }
            (*d).__nextchar = ptr::null_mut();
        } else {
            if *(*d).__nextchar != '\0' as i8 {
                (*d).optarg = (*d).__nextchar;
                (*d).optind += 1;
            } else if (*d).optind == argc {
                if print_errors != 0 {
                    eprintln!(
                        "{}: option requires an argument -- '{}'",
                        CStr::from_ptr(*argv).to_str().unwrap(),
                        c as u8 as char
                    );
                }
                (*d).optopt = c as i32;
                return if *optstring == ':' as i8 { ':' as i32 } else { '?' as i32 };
            } else {
                (*d).optarg = *argv.offset((*d).optind as isize);
                (*d).optind += 1;
            }
            (*d).__nextchar = ptr::null_mut();
        }
    }
    c as i32
}

#[no_mangle]
pub unsafe extern "C" fn getopt(
    argc: c_int,
    argv: *mut *mut c_char,
    optstring: *const c_char,
) -> c_int {
    _getopt_internal(
        argc,
        argv,
        optstring,
        ptr::null(),
        ptr::null_mut(),
        0,
        0,
    )
}

unsafe fn _getopt_internal(
    argc: c_int,
    argv: *mut *mut c_char,
    optstring: *const c_char,
    longopts: *const option,
    longind: *mut c_int,
    long_only: bool,
    posixly_correct: bool,
) -> c_int {
    let result = _getopt_internal_r(
        argc,
        argv,
        optstring,
        longopts,
        longind,
        long_only,
        &mut getopt_data,
        posixly_correct,
    );

    optind = getopt_data.optind;
    optarg = getopt_data.optarg;
    optopt = getopt_data.optopt;

