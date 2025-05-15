use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    static mut stderr: *mut _IO_FILE;
    fn funlockfile(__stream: *mut FILE);
    fn flockfile(__stream: *mut FILE);
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn abort() -> !;
    fn getenv(__name: *const i8) -> *mut i8;
    fn rpl_free(ptr: *mut libc::c_void);
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rpl_option {
    pub name: *const i8,
    pub has_arg: i32,
    pub flag: *mut i32,
    pub val: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _getopt_data {
    pub rpl_optind: i32,
    pub rpl_opterr: i32,
    pub rpl_optopt: i32,
    pub rpl_optarg: *mut i8,
    pub __initialized: i32,
    pub __nextchar: *mut i8,
    pub __ordering: __ord,
    pub __first_nonopt: i32,
    pub __last_nonopt: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum __ord {
    REQUIRE_ORDER,
    PERMUTE,
    RETURN_IN_ORDER,
}
impl __ord {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            __ord::REQUIRE_ORDER => 0,
            __ord::PERMUTE => 1,
            __ord::RETURN_IN_ORDER => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> __ord {
        match value {
            0 => __ord::REQUIRE_ORDER,
            1 => __ord::PERMUTE,
            2 => __ord::RETURN_IN_ORDER,
            _ => panic!("Invalid value for __ord: {}", value),
        }
    }
}
impl AddAssign<u32> for __ord {
    fn add_assign(&mut self, rhs: u32) {
        *self = __ord::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for __ord {
    fn sub_assign(&mut self, rhs: u32) {
        *self = __ord::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for __ord {
    fn mul_assign(&mut self, rhs: u32) {
        *self = __ord::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for __ord {
    fn div_assign(&mut self, rhs: u32) {
        *self = __ord::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for __ord {
    fn rem_assign(&mut self, rhs: u32) {
        *self = __ord::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for __ord {
    type Output = __ord;
    fn add(self, rhs: u32) -> __ord {
        __ord::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for __ord {
    type Output = __ord;
    fn sub(self, rhs: u32) -> __ord {
        __ord::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for __ord {
    type Output = __ord;
    fn mul(self, rhs: u32) -> __ord {
        __ord::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for __ord {
    type Output = __ord;
    fn div(self, rhs: u32) -> __ord {
        __ord::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for __ord {
    type Output = __ord;
    fn rem(self, rhs: u32) -> __ord {
        __ord::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type size_t = u64;
pub type __off64_t = i64;
pub type _IO_lock_t = ();
pub type __off_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
#[no_mangle]
pub static mut rpl_optarg: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut rpl_optind: i32 = 1 as i32;
#[no_mangle]
pub static mut rpl_opterr: i32 = 1 as i32;
#[no_mangle]
pub static mut rpl_optopt: i32 = '?' as i32;
static mut getopt_data: _getopt_data = _getopt_data {
    rpl_optind: 0,
    rpl_opterr: 0,
    rpl_optopt: 0,
    rpl_optarg: 0 as *const i8 as *mut i8,
    __initialized: 0,
    __nextchar: 0 as *const i8 as *mut i8,
    __ordering: __ord::REQUIRE_ORDER,
    __first_nonopt: 0,
    __last_nonopt: 0,
};
unsafe extern "C" fn exchange(mut argv: *mut *mut i8, mut d: *mut _getopt_data) {
    let mut bottom: i32 = (*d).__first_nonopt;
    let mut middle: i32 = (*d).__last_nonopt;
    let mut top: i32 = (*d).rpl_optind;
    let mut tem: *mut i8 = 0 as *mut i8;
    while top > middle && middle > bottom {
        if top - middle > middle - bottom {
            let mut len: i32 = middle - bottom;
            let mut i: i32 = 0;
            i = 0 as i32;
            while i < len {
                tem = *argv.offset((bottom + i) as isize);
                let ref mut fresh0 = *argv.offset((bottom + i) as isize);
                *fresh0 = *argv.offset((top - (middle - bottom) + i) as isize);
                let ref mut fresh1 = *argv
                    .offset((top - (middle - bottom) + i) as isize);
                *fresh1 = tem;
                i += 1;
                i;
            }
            top -= len;
        } else {
            let mut len_0: i32 = top - middle;
            let mut i_0: i32 = 0;
            i_0 = 0 as i32;
            while i_0 < len_0 {
                tem = *argv.offset((bottom + i_0) as isize);
                let ref mut fresh2 = *argv.offset((bottom + i_0) as isize);
                *fresh2 = *argv.offset((middle + i_0) as isize);
                let ref mut fresh3 = *argv.offset((middle + i_0) as isize);
                *fresh3 = tem;
                i_0 += 1;
                i_0;
            }
            bottom += len_0;
        }
    }
    (*d).__first_nonopt += (*d).rpl_optind - (*d).__last_nonopt;
    (*d).__last_nonopt = (*d).rpl_optind;
}
unsafe extern "C" fn process_long_option(
    mut argc: i32,
    mut argv: *mut *mut i8,
    mut optstring: *const i8,
    mut longopts: *const rpl_option,
    mut longind: *mut i32,
    mut long_only: i32,
    mut d: *mut _getopt_data,
    mut print_errors: i32,
    mut prefix: *const i8,
) -> i32 {
    let mut nameend: *mut i8 = 0 as *mut i8;
    let mut namelen: size_t = 0;
    let mut p: *const rpl_option = 0 as *const rpl_option;
    let mut pfound: *const rpl_option = 0 as *const rpl_option;
    let mut n_options: i32 = 0;
    let mut option_index: i32 = 0;
    nameend = (*d).__nextchar;
    while *nameend as i32 != 0 && *nameend as i32 != '=' as i32 {
        nameend = nameend.offset(1);
        nameend;
    }
    namelen = nameend.offset_from((*d).__nextchar) as i64 as size_t;
    p = longopts;
    n_options = 0 as i32;
    while !((*p).name).is_null() {
        if strncmp((*p).name, (*d).__nextchar, namelen) == 0
            && namelen == strlen((*p).name)
        {
            pfound = p;
            option_index = n_options;
            break;
        } else {
            p = p.offset(1);
            p;
            n_options += 1;
            n_options;
        }
    }
    if pfound.is_null() {
        let mut ambig_set: *mut u8 = 0 as *mut u8;
        let mut ambig_malloced: i32 = 0 as i32;
        let mut ambig_fallback: i32 = 0 as i32;
        let mut indfound: i32 = -(1 as i32);
        p = longopts;
        option_index = 0 as i32;
        while !((*p).name).is_null() {
            if strncmp((*p).name, (*d).__nextchar, namelen) == 0 {
                if pfound.is_null() {
                    pfound = p;
                    indfound = option_index;
                } else if long_only != 0 || (*pfound).has_arg != (*p).has_arg
                    || (*pfound).flag != (*p).flag || (*pfound).val != (*p).val
                {
                    if ambig_fallback == 0 {
                        if print_errors == 0 {
                            ambig_fallback = 1 as i32;
                        } else if ambig_set.is_null() {
                            ambig_set = malloc(n_options as u64) as *mut u8;
                            if ambig_set.is_null() {
                                ambig_fallback = 1 as i32;
                            } else {
                                ambig_malloced = 1 as i32;
                            }
                            if !ambig_set.is_null() {
                                memset(
                                    ambig_set as *mut libc::c_void,
                                    0 as i32,
                                    n_options as u64,
                                );
                                *ambig_set.offset(indfound as isize) = 1 as i32 as u8;
                            }
                        }
                        if !ambig_set.is_null() {
                            *ambig_set.offset(option_index as isize) = 1 as i32 as u8;
                        }
                    }
                }
            }
            p = p.offset(1);
            p;
            option_index += 1;
            option_index;
        }
        if !ambig_set.is_null() || ambig_fallback != 0 {
            if print_errors != 0 {
                if ambig_fallback != 0 {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"%s: option '%s%s' is ambiguous\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        *argv.offset(0 as i32 as isize),
                        prefix,
                        (*d).__nextchar,
                    );
                } else {
                    flockfile(stderr);
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"%s: option '%s%s' is ambiguous; possibilities:\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        *argv.offset(0 as i32 as isize),
                        prefix,
                        (*d).__nextchar,
                    );
                    option_index = 0 as i32;
                    while option_index < n_options {
                        if *ambig_set.offset(option_index as isize) != 0 {
                            fprintf(
                                stderr,
                                b" '%s%s'\0" as *const u8 as *const i8,
                                prefix,
                                (*longopts.offset(option_index as isize)).name,
                            );
                        }
                        option_index += 1;
                        option_index;
                    }
                    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                    funlockfile(stderr);
                }
            }
            if ambig_malloced != 0 {
                rpl_free(ambig_set as *mut libc::c_void);
            }
            (*d).__nextchar = ((*d).__nextchar).offset(strlen((*d).__nextchar) as isize);
            (*d).rpl_optind += 1;
            (*d).rpl_optind;
            (*d).rpl_optopt = 0 as i32;
            return '?' as i32;
        }
        option_index = indfound;
    }
    if pfound.is_null() {
        if long_only == 0
            || *(*argv.offset((*d).rpl_optind as isize)).offset(1 as i32 as isize) as i32
                == '-' as i32 || (strchr(optstring, *(*d).__nextchar as i32)).is_null()
        {
            if print_errors != 0 {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"%s: unrecognized option '%s%s'\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    *argv.offset(0 as i32 as isize),
                    prefix,
                    (*d).__nextchar,
                );
            }
            (*d).__nextchar = 0 as *mut i8;
            (*d).rpl_optind += 1;
            (*d).rpl_optind;
            (*d).rpl_optopt = 0 as i32;
            return '?' as i32;
        }
        return -(1 as i32);
    }
    (*d).rpl_optind += 1;
    (*d).rpl_optind;
    (*d).__nextchar = 0 as *mut i8;
    if *nameend != 0 {
        if (*pfound).has_arg != 0 {
            (*d).rpl_optarg = nameend.offset(1 as i32 as isize);
        } else {
            if print_errors != 0 {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"%s: option '%s%s' doesn't allow an argument\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    *argv.offset(0 as i32 as isize),
                    prefix,
                    (*pfound).name,
                );
            }
            (*d).rpl_optopt = (*pfound).val;
            return '?' as i32;
        }
    } else if (*pfound).has_arg == 1 as i32 {
        if (*d).rpl_optind < argc {
            let fresh4 = (*d).rpl_optind;
            (*d).rpl_optind = (*d).rpl_optind + 1;
            (*d).rpl_optarg = *argv.offset(fresh4 as isize);
        } else {
            if print_errors != 0 {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"%s: option '%s%s' requires an argument\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    *argv.offset(0 as i32 as isize),
                    prefix,
                    (*pfound).name,
                );
            }
            (*d).rpl_optopt = (*pfound).val;
            return if *optstring.offset(0 as i32 as isize) as i32 == ':' as i32 {
                ':' as i32
            } else {
                '?' as i32
            };
        }
    }
    if !longind.is_null() {
        *longind = option_index;
    }
    if !((*pfound).flag).is_null() {
        *(*pfound).flag = (*pfound).val;
        return 0 as i32;
    }
    return (*pfound).val;
}
unsafe extern "C" fn _getopt_initialize(
    mut argc: i32,
    mut argv: *mut *mut i8,
    mut optstring: *const i8,
    mut d: *mut _getopt_data,
    mut posixly_correct: i32,
) -> *const i8 {
    if (*d).rpl_optind == 0 as i32 {
        (*d).rpl_optind = 1 as i32;
    }
    (*d).__last_nonopt = (*d).rpl_optind;
    (*d).__first_nonopt = (*d).__last_nonopt;
    (*d).__nextchar = 0 as *mut i8;
    if *optstring.offset(0 as i32 as isize) as i32 == '-' as i32 {
        (*d).__ordering = __ord::RETURN_IN_ORDER;
        optstring = optstring.offset(1);
        optstring;
    } else if *optstring.offset(0 as i32 as isize) as i32 == '+' as i32 {
        (*d).__ordering = __ord::REQUIRE_ORDER;
        optstring = optstring.offset(1);
        optstring;
    } else if posixly_correct != 0
        || !(getenv(b"POSIXLY_CORRECT\0" as *const u8 as *const i8)).is_null()
    {
        (*d).__ordering = __ord::REQUIRE_ORDER;
    } else {
        (*d).__ordering = __ord::PERMUTE;
    }
    (*d).__initialized = 1 as i32;
    return optstring;
}
#[no_mangle]
pub unsafe extern "C" fn _getopt_internal_r(
    mut argc: i32,
    mut argv: *mut *mut i8,
    mut optstring: *const i8,
    mut longopts: *const rpl_option,
    mut longind: *mut i32,
    mut long_only: i32,
    mut d: *mut _getopt_data,
    mut posixly_correct: i32,
) -> i32 {
    let mut print_errors: i32 = (*d).rpl_opterr;
    if argc < 1 as i32 {
        return -(1 as i32);
    }
    (*d).rpl_optarg = 0 as *mut i8;
    if (*d).rpl_optind == 0 as i32 || (*d).__initialized == 0 {
        optstring = _getopt_initialize(argc, argv, optstring, d, posixly_correct);
    } else if *optstring.offset(0 as i32 as isize) as i32 == '-' as i32
        || *optstring.offset(0 as i32 as isize) as i32 == '+' as i32
    {
        optstring = optstring.offset(1);
        optstring;
    }
    if *optstring.offset(0 as i32 as isize) as i32 == ':' as i32 {
        print_errors = 0 as i32;
    }
    if ((*d).__nextchar).is_null() || *(*d).__nextchar as i32 == '\0' as i32 {
        if (*d).__last_nonopt > (*d).rpl_optind {
            (*d).__last_nonopt = (*d).rpl_optind;
        }
        if (*d).__first_nonopt > (*d).rpl_optind {
            (*d).__first_nonopt = (*d).rpl_optind;
        }
        if (*d).__ordering as u32 == __ord::PERMUTE as i32 as u32 {
            if (*d).__first_nonopt != (*d).__last_nonopt
                && (*d).__last_nonopt != (*d).rpl_optind
            {
                exchange(argv, d);
            } else if (*d).__last_nonopt != (*d).rpl_optind {
                (*d).__first_nonopt = (*d).rpl_optind;
            }
            while (*d).rpl_optind < argc
                && (*(*argv.offset((*d).rpl_optind as isize)).offset(0 as i32 as isize)
                    as i32 != '-' as i32
                    || *(*argv.offset((*d).rpl_optind as isize))
                        .offset(1 as i32 as isize) as i32 == '\0' as i32)
            {
                (*d).rpl_optind += 1;
                (*d).rpl_optind;
            }
            (*d).__last_nonopt = (*d).rpl_optind;
        }
        if (*d).rpl_optind != argc
            && strcmp(
                *argv.offset((*d).rpl_optind as isize),
                b"--\0" as *const u8 as *const i8,
            ) == 0
        {
            (*d).rpl_optind += 1;
            (*d).rpl_optind;
            if (*d).__first_nonopt != (*d).__last_nonopt
                && (*d).__last_nonopt != (*d).rpl_optind
            {
                exchange(argv, d);
            } else if (*d).__first_nonopt == (*d).__last_nonopt {
                (*d).__first_nonopt = (*d).rpl_optind;
            }
            (*d).__last_nonopt = argc;
            (*d).rpl_optind = argc;
        }
        if (*d).rpl_optind == argc {
            if (*d).__first_nonopt != (*d).__last_nonopt {
                (*d).rpl_optind = (*d).__first_nonopt;
            }
            return -(1 as i32);
        }
        if *(*argv.offset((*d).rpl_optind as isize)).offset(0 as i32 as isize) as i32
            != '-' as i32
            || *(*argv.offset((*d).rpl_optind as isize)).offset(1 as i32 as isize) as i32
                == '\0' as i32
        {
            if (*d).__ordering as u32 == __ord::REQUIRE_ORDER as i32 as u32 {
                return -(1 as i32);
            }
            let fresh5 = (*d).rpl_optind;
            (*d).rpl_optind = (*d).rpl_optind + 1;
            (*d).rpl_optarg = *argv.offset(fresh5 as isize);
            return 1 as i32;
        }
        if !longopts.is_null() {
            if *(*argv.offset((*d).rpl_optind as isize)).offset(1 as i32 as isize) as i32
                == '-' as i32
            {
                (*d).__nextchar = (*argv.offset((*d).rpl_optind as isize))
                    .offset(2 as i32 as isize);
                return process_long_option(
                    argc,
                    argv,
                    optstring,
                    longopts,
                    longind,
                    long_only,
                    d,
                    print_errors,
                    b"--\0" as *const u8 as *const i8,
                );
            }
            if long_only != 0
                && (*(*argv.offset((*d).rpl_optind as isize)).offset(2 as i32 as isize)
                    as i32 != 0
                    || (strchr(
                        optstring,
                        *(*argv.offset((*d).rpl_optind as isize))
                            .offset(1 as i32 as isize) as i32,
                    ))
                        .is_null())
            {
                let mut code: i32 = 0;
                (*d).__nextchar = (*argv.offset((*d).rpl_optind as isize))
                    .offset(1 as i32 as isize);
                code = process_long_option(
                    argc,
                    argv,
                    optstring,
                    longopts,
                    longind,
                    long_only,
                    d,
                    print_errors,
                    b"-\0" as *const u8 as *const i8,
                );
                if code != -(1 as i32) {
                    return code;
                }
            }
        }
        (*d).__nextchar = (*argv.offset((*d).rpl_optind as isize))
            .offset(1 as i32 as isize);
    }
    let fresh6 = (*d).__nextchar;
    (*d).__nextchar = ((*d).__nextchar).offset(1);
    let mut c: i8 = *fresh6;
    let mut temp: *const i8 = strchr(optstring, c as i32);
    if *(*d).__nextchar as i32 == '\0' as i32 {
        (*d).rpl_optind += 1;
        (*d).rpl_optind;
    }
    if temp.is_null() || c as i32 == ':' as i32 || c as i32 == ';' as i32 {
        if print_errors != 0 {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"%s: invalid option -- '%c'\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                *argv.offset(0 as i32 as isize),
                c as i32,
            );
        }
        (*d).rpl_optopt = c as i32;
        return '?' as i32;
    }
    if *temp.offset(0 as i32 as isize) as i32 == 'W' as i32
        && *temp.offset(1 as i32 as isize) as i32 == ';' as i32 && !longopts.is_null()
    {
        if *(*d).__nextchar as i32 != '\0' as i32 {
            (*d).rpl_optarg = (*d).__nextchar;
        } else if (*d).rpl_optind == argc {
            if print_errors != 0 {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"%s: option requires an argument -- '%c'\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    *argv.offset(0 as i32 as isize),
                    c as i32,
                );
            }
            (*d).rpl_optopt = c as i32;
            if *optstring.offset(0 as i32 as isize) as i32 == ':' as i32 {
                c = ':' as i32 as i8;
            } else {
                c = '?' as i32 as i8;
            }
            return c as i32;
        } else {
            (*d).rpl_optarg = *argv.offset((*d).rpl_optind as isize);
        }
        (*d).__nextchar = (*d).rpl_optarg;
        (*d).rpl_optarg = 0 as *mut i8;
        return process_long_option(
            argc,
            argv,
            optstring,
            longopts,
            longind,
            0 as i32,
            d,
            print_errors,
            b"-W \0" as *const u8 as *const i8,
        );
    }
    if *temp.offset(1 as i32 as isize) as i32 == ':' as i32 {
        if *temp.offset(2 as i32 as isize) as i32 == ':' as i32 {
            if *(*d).__nextchar as i32 != '\0' as i32 {
                (*d).rpl_optarg = (*d).__nextchar;
                (*d).rpl_optind += 1;
                (*d).rpl_optind;
            } else {
                (*d).rpl_optarg = 0 as *mut i8;
            }
            (*d).__nextchar = 0 as *mut i8;
        } else {
            if *(*d).__nextchar as i32 != '\0' as i32 {
                (*d).rpl_optarg = (*d).__nextchar;
                (*d).rpl_optind += 1;
                (*d).rpl_optind;
            } else if (*d).rpl_optind == argc {
                if print_errors != 0 {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"%s: option requires an argument -- '%c'\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        *argv.offset(0 as i32 as isize),
                        c as i32,
                    );
                }
                (*d).rpl_optopt = c as i32;
                if *optstring.offset(0 as i32 as isize) as i32 == ':' as i32 {
                    c = ':' as i32 as i8;
                } else {
                    c = '?' as i32 as i8;
                }
            } else {
                let fresh7 = (*d).rpl_optind;
                (*d).rpl_optind = (*d).rpl_optind + 1;
                (*d).rpl_optarg = *argv.offset(fresh7 as isize);
            }
            (*d).__nextchar = 0 as *mut i8;
        }
    }
    return c as i32;
}
#[no_mangle]
pub unsafe extern "C" fn rpl_getopt_internal(
    mut argc: i32,
    mut argv: *mut *mut i8,
    mut optstring: *const i8,
    mut longopts: *const rpl_option,
    mut longind: *mut i32,
    mut long_only: i32,
    mut posixly_correct: i32,
) -> i32 {
    let mut result: i32 = 0;
    getopt_data.rpl_optind = rpl_optind;
    getopt_data.rpl_opterr = rpl_opterr;
    result = _getopt_internal_r(
        argc,
        argv,
        optstring,
        longopts,
        longind,
        long_only,
        &mut getopt_data,
        posixly_correct,
    );
    rpl_optind = getopt_data.rpl_optind;
    rpl_optarg = getopt_data.rpl_optarg;
    rpl_optopt = getopt_data.rpl_optopt;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn rpl_getopt(
    mut argc: i32,
    mut argv: *const *mut i8,
    mut optstring: *const i8,
) -> i32 {
    return rpl_getopt_internal(
        argc,
        argv as *mut *mut i8,
        optstring,
        0 as *const rpl_option,
        0 as *mut i32,
        0 as i32,
        1 as i32,
    );
}