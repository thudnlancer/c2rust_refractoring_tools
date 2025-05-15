use ::libc;
extern "C" {
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rpl_option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub const POSIXLY_CORRECT: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _getopt_data {
    pub rpl_optind: libc::c_int,
    pub rpl_opterr: libc::c_int,
    pub rpl_optopt: libc::c_int,
    pub rpl_optarg: *mut libc::c_char,
    pub __initialized: libc::c_int,
    pub __nextchar: *mut libc::c_char,
    pub __ordering: C2RustUnnamed,
    pub __posixly_correct: libc::c_int,
    pub __first_nonopt: libc::c_int,
    pub __last_nonopt: libc::c_int,
}
pub type C2RustUnnamed = libc::c_uint;
pub const RETURN_IN_ORDER: C2RustUnnamed = 2;
pub const PERMUTE: C2RustUnnamed = 1;
pub const REQUIRE_ORDER: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type size_t = libc::c_ulong;
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type C2RustUnnamed_0 = libc::c_uint;
#[no_mangle]
pub static mut rpl_optarg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut rpl_optind: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut rpl_opterr: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut rpl_optopt: libc::c_int = '?' as i32;
static mut getopt_data: _getopt_data = _getopt_data {
    rpl_optind: 0,
    rpl_opterr: 0,
    rpl_optopt: 0,
    rpl_optarg: 0 as *const libc::c_char as *mut libc::c_char,
    __initialized: 0,
    __nextchar: 0 as *const libc::c_char as *mut libc::c_char,
    __ordering: REQUIRE_ORDER,
    __posixly_correct: 0,
    __first_nonopt: 0,
    __last_nonopt: 0,
};
unsafe extern "C" fn exchange(
    mut argv: *mut *mut libc::c_char,
    mut d: *mut _getopt_data,
) {
    let mut bottom: libc::c_int = (*d).__first_nonopt;
    let mut middle: libc::c_int = (*d).__last_nonopt;
    let mut top: libc::c_int = (*d).rpl_optind;
    let mut tem: *mut libc::c_char = 0 as *mut libc::c_char;
    while top > middle && middle > bottom {
        if top - middle > middle - bottom {
            let mut len: libc::c_int = middle - bottom;
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
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
            let mut len_0: libc::c_int = top - middle;
            let mut i_0: libc::c_int = 0;
            i_0 = 0 as libc::c_int;
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
unsafe extern "C" fn _getopt_initialize(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut optstring: *const libc::c_char,
    mut posixly_correct: libc::c_int,
    mut d: *mut _getopt_data,
) -> *const libc::c_char {
    (*d).__last_nonopt = (*d).rpl_optind;
    (*d).__first_nonopt = (*d).__last_nonopt;
    (*d).__nextchar = 0 as *mut libc::c_char;
    (*d)
        .__posixly_correct = (posixly_correct != 0
        || !(getenv(b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char)).is_null())
        as libc::c_int;
    if *optstring.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
        (*d).__ordering = RETURN_IN_ORDER;
        optstring = optstring.offset(1);
        optstring;
    } else if *optstring.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32 {
        (*d).__ordering = REQUIRE_ORDER;
        optstring = optstring.offset(1);
        optstring;
    } else if (*d).__posixly_correct != 0 {
        (*d).__ordering = REQUIRE_ORDER;
    } else {
        (*d).__ordering = PERMUTE;
    }
    return optstring;
}
#[no_mangle]
pub unsafe extern "C" fn _getopt_internal_r(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut optstring: *const libc::c_char,
    mut longopts: *const rpl_option,
    mut longind: *mut libc::c_int,
    mut long_only: libc::c_int,
    mut posixly_correct: libc::c_int,
    mut d: *mut _getopt_data,
) -> libc::c_int {
    let mut print_errors: libc::c_int = (*d).rpl_opterr;
    if *optstring.offset(0 as libc::c_int as isize) as libc::c_int == ':' as i32 {
        print_errors = 0 as libc::c_int;
    }
    if argc < 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*d).rpl_optarg = 0 as *mut libc::c_char;
    if (*d).rpl_optind == 0 as libc::c_int || (*d).__initialized == 0 {
        if (*d).rpl_optind == 0 as libc::c_int {
            (*d).rpl_optind = 1 as libc::c_int;
        }
        optstring = _getopt_initialize(argc, argv, optstring, posixly_correct, d);
        (*d).__initialized = 1 as libc::c_int;
    }
    if ((*d).__nextchar).is_null() || *(*d).__nextchar as libc::c_int == '\0' as i32 {
        if (*d).__last_nonopt > (*d).rpl_optind {
            (*d).__last_nonopt = (*d).rpl_optind;
        }
        if (*d).__first_nonopt > (*d).rpl_optind {
            (*d).__first_nonopt = (*d).rpl_optind;
        }
        if (*d).__ordering as libc::c_uint == PERMUTE as libc::c_int as libc::c_uint {
            if (*d).__first_nonopt != (*d).__last_nonopt
                && (*d).__last_nonopt != (*d).rpl_optind
            {
                exchange(argv, d);
            } else if (*d).__last_nonopt != (*d).rpl_optind {
                (*d).__first_nonopt = (*d).rpl_optind;
            }
            while (*d).rpl_optind < argc
                && (*(*argv.offset((*d).rpl_optind as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32
                    || *(*argv.offset((*d).rpl_optind as isize))
                        .offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32)
            {
                (*d).rpl_optind += 1;
                (*d).rpl_optind;
            }
            (*d).__last_nonopt = (*d).rpl_optind;
        }
        if (*d).rpl_optind != argc
            && strcmp(
                *argv.offset((*d).rpl_optind as isize),
                b"--\0" as *const u8 as *const libc::c_char,
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
            return -(1 as libc::c_int);
        }
        if *(*argv.offset((*d).rpl_optind as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int != '-' as i32
            || *(*argv.offset((*d).rpl_optind as isize))
                .offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            if (*d).__ordering as libc::c_uint
                == REQUIRE_ORDER as libc::c_int as libc::c_uint
            {
                return -(1 as libc::c_int);
            }
            let fresh4 = (*d).rpl_optind;
            (*d).rpl_optind = (*d).rpl_optind + 1;
            (*d).rpl_optarg = *argv.offset(fresh4 as isize);
            return 1 as libc::c_int;
        }
        (*d)
            .__nextchar = (*argv.offset((*d).rpl_optind as isize))
            .offset(1 as libc::c_int as isize)
            .offset(
                (!longopts.is_null()
                    && *(*argv.offset((*d).rpl_optind as isize))
                        .offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32)
                    as libc::c_int as isize,
            );
    }
    if !longopts.is_null()
        && (*(*argv.offset((*d).rpl_optind as isize)).offset(1 as libc::c_int as isize)
            as libc::c_int == '-' as i32
            || long_only != 0
                && (*(*argv.offset((*d).rpl_optind as isize))
                    .offset(2 as libc::c_int as isize) as libc::c_int != 0
                    || (strchr(
                        optstring,
                        *(*argv.offset((*d).rpl_optind as isize))
                            .offset(1 as libc::c_int as isize) as libc::c_int,
                    ))
                        .is_null()))
    {
        let mut nameend: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p: *const rpl_option = 0 as *const rpl_option;
        let mut pfound: *const rpl_option = 0 as *const rpl_option;
        let mut exact: libc::c_int = 0 as libc::c_int;
        let mut ambig: libc::c_int = 0 as libc::c_int;
        let mut indfound: libc::c_int = -(1 as libc::c_int);
        let mut option_index: libc::c_int = 0;
        nameend = (*d).__nextchar;
        while *nameend as libc::c_int != 0 && *nameend as libc::c_int != '=' as i32 {
            nameend = nameend.offset(1);
            nameend;
        }
        p = longopts;
        option_index = 0 as libc::c_int;
        while !((*p).name).is_null() {
            if strncmp(
                (*p).name,
                (*d).__nextchar,
                nameend.offset_from((*d).__nextchar) as libc::c_long as libc::c_ulong,
            ) == 0
            {
                if nameend.offset_from((*d).__nextchar) as libc::c_long as libc::c_uint
                    == strlen((*p).name) as libc::c_uint
                {
                    pfound = p;
                    indfound = option_index;
                    exact = 1 as libc::c_int;
                    break;
                } else if pfound.is_null() {
                    pfound = p;
                    indfound = option_index;
                } else if long_only != 0 || (*pfound).has_arg != (*p).has_arg
                    || (*pfound).flag != (*p).flag || (*pfound).val != (*p).val
                {
                    ambig = 1 as libc::c_int;
                }
            }
            p = p.offset(1);
            p;
            option_index += 1;
            option_index;
        }
        if ambig != 0 && exact == 0 {
            if print_errors != 0 {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: option `%s' is ambiguous\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    *argv.offset(0 as libc::c_int as isize),
                    *argv.offset((*d).rpl_optind as isize),
                );
            }
            (*d).__nextchar = ((*d).__nextchar).offset(strlen((*d).__nextchar) as isize);
            (*d).rpl_optind += 1;
            (*d).rpl_optind;
            (*d).rpl_optopt = 0 as libc::c_int;
            return '?' as i32;
        }
        if !pfound.is_null() {
            option_index = indfound;
            (*d).rpl_optind += 1;
            (*d).rpl_optind;
            if *nameend != 0 {
                if (*pfound).has_arg != 0 {
                    (*d).rpl_optarg = nameend.offset(1 as libc::c_int as isize);
                } else {
                    if print_errors != 0 {
                        if *(*argv.offset(((*d).rpl_optind - 1 as libc::c_int) as isize))
                            .offset(1 as libc::c_int as isize) as libc::c_int
                            == '-' as i32
                        {
                            fprintf(
                                stderr,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"%s: option `--%s' doesn't allow an argument\n\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                *argv.offset(0 as libc::c_int as isize),
                                (*pfound).name,
                            );
                        } else {
                            fprintf(
                                stderr,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"%s: option `%c%s' doesn't allow an argument\n\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                *argv.offset(0 as libc::c_int as isize),
                                *(*argv
                                    .offset(((*d).rpl_optind - 1 as libc::c_int) as isize))
                                    .offset(0 as libc::c_int as isize) as libc::c_int,
                                (*pfound).name,
                            );
                        }
                    }
                    (*d)
                        .__nextchar = ((*d).__nextchar)
                        .offset(strlen((*d).__nextchar) as isize);
                    (*d).rpl_optopt = (*pfound).val;
                    return '?' as i32;
                }
            } else if (*pfound).has_arg == 1 as libc::c_int {
                if (*d).rpl_optind < argc {
                    let fresh5 = (*d).rpl_optind;
                    (*d).rpl_optind = (*d).rpl_optind + 1;
                    (*d).rpl_optarg = *argv.offset(fresh5 as isize);
                } else {
                    if print_errors != 0 {
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s: option `%s' requires an argument\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            *argv.offset(0 as libc::c_int as isize),
                            *argv.offset(((*d).rpl_optind - 1 as libc::c_int) as isize),
                        );
                    }
                    (*d)
                        .__nextchar = ((*d).__nextchar)
                        .offset(strlen((*d).__nextchar) as isize);
                    (*d).rpl_optopt = (*pfound).val;
                    return if *optstring.offset(0 as libc::c_int as isize) as libc::c_int
                        == ':' as i32
                    {
                        ':' as i32
                    } else {
                        '?' as i32
                    };
                }
            }
            (*d).__nextchar = ((*d).__nextchar).offset(strlen((*d).__nextchar) as isize);
            if !longind.is_null() {
                *longind = option_index;
            }
            if !((*pfound).flag).is_null() {
                *(*pfound).flag = (*pfound).val;
                return 0 as libc::c_int;
            }
            return (*pfound).val;
        }
        if long_only == 0
            || *(*argv.offset((*d).rpl_optind as isize))
                .offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32
            || (strchr(optstring, *(*d).__nextchar as libc::c_int)).is_null()
        {
            if print_errors != 0 {
                if *(*argv.offset((*d).rpl_optind as isize))
                    .offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32
                {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: unrecognized option `--%s'\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        *argv.offset(0 as libc::c_int as isize),
                        (*d).__nextchar,
                    );
                } else {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: unrecognized option `%c%s'\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        *argv.offset(0 as libc::c_int as isize),
                        *(*argv.offset((*d).rpl_optind as isize))
                            .offset(0 as libc::c_int as isize) as libc::c_int,
                        (*d).__nextchar,
                    );
                }
            }
            (*d)
                .__nextchar = b"\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            (*d).rpl_optind += 1;
            (*d).rpl_optind;
            (*d).rpl_optopt = 0 as libc::c_int;
            return '?' as i32;
        }
    }
    let fresh6 = (*d).__nextchar;
    (*d).__nextchar = ((*d).__nextchar).offset(1);
    let mut c: libc::c_char = *fresh6;
    let mut temp: *mut libc::c_char = strchr(optstring, c as libc::c_int);
    if *(*d).__nextchar as libc::c_int == '\0' as i32 {
        (*d).rpl_optind += 1;
        (*d).rpl_optind;
    }
    if temp.is_null() || c as libc::c_int == ':' as i32 {
        if print_errors != 0 {
            if (*d).__posixly_correct != 0 {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: illegal option -- %c\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    *argv.offset(0 as libc::c_int as isize),
                    c as libc::c_int,
                );
            } else {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: invalid option -- %c\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    *argv.offset(0 as libc::c_int as isize),
                    c as libc::c_int,
                );
            }
        }
        (*d).rpl_optopt = c as libc::c_int;
        return '?' as i32;
    }
    if *temp.offset(0 as libc::c_int as isize) as libc::c_int == 'W' as i32
        && *temp.offset(1 as libc::c_int as isize) as libc::c_int == ';' as i32
    {
        let mut nameend_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p_0: *const rpl_option = 0 as *const rpl_option;
        let mut pfound_0: *const rpl_option = 0 as *const rpl_option;
        let mut exact_0: libc::c_int = 0 as libc::c_int;
        let mut ambig_0: libc::c_int = 0 as libc::c_int;
        let mut indfound_0: libc::c_int = 0 as libc::c_int;
        let mut option_index_0: libc::c_int = 0;
        if *(*d).__nextchar as libc::c_int != '\0' as i32 {
            (*d).rpl_optarg = (*d).__nextchar;
            (*d).rpl_optind += 1;
            (*d).rpl_optind;
        } else if (*d).rpl_optind == argc {
            if print_errors != 0 {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: option requires an argument -- %c\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    *argv.offset(0 as libc::c_int as isize),
                    c as libc::c_int,
                );
            }
            (*d).rpl_optopt = c as libc::c_int;
            if *optstring.offset(0 as libc::c_int as isize) as libc::c_int == ':' as i32
            {
                c = ':' as i32 as libc::c_char;
            } else {
                c = '?' as i32 as libc::c_char;
            }
            return c as libc::c_int;
        } else {
            let fresh7 = (*d).rpl_optind;
            (*d).rpl_optind = (*d).rpl_optind + 1;
            (*d).rpl_optarg = *argv.offset(fresh7 as isize);
        }
        nameend_0 = (*d).rpl_optarg;
        (*d).__nextchar = nameend_0;
        while *nameend_0 as libc::c_int != 0 && *nameend_0 as libc::c_int != '=' as i32 {
            nameend_0 = nameend_0.offset(1);
            nameend_0;
        }
        p_0 = longopts;
        option_index_0 = 0 as libc::c_int;
        while !((*p_0).name).is_null() {
            if strncmp(
                (*p_0).name,
                (*d).__nextchar,
                nameend_0.offset_from((*d).__nextchar) as libc::c_long as libc::c_ulong,
            ) == 0
            {
                if nameend_0.offset_from((*d).__nextchar) as libc::c_long as libc::c_uint
                    as libc::c_ulong == strlen((*p_0).name)
                {
                    pfound_0 = p_0;
                    indfound_0 = option_index_0;
                    exact_0 = 1 as libc::c_int;
                    break;
                } else if pfound_0.is_null() {
                    pfound_0 = p_0;
                    indfound_0 = option_index_0;
                } else {
                    ambig_0 = 1 as libc::c_int;
                }
            }
            p_0 = p_0.offset(1);
            p_0;
            option_index_0 += 1;
            option_index_0;
        }
        if ambig_0 != 0 && exact_0 == 0 {
            if print_errors != 0 {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: option `-W %s' is ambiguous\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    *argv.offset(0 as libc::c_int as isize),
                    *argv.offset((*d).rpl_optind as isize),
                );
            }
            (*d).__nextchar = ((*d).__nextchar).offset(strlen((*d).__nextchar) as isize);
            (*d).rpl_optind += 1;
            (*d).rpl_optind;
            return '?' as i32;
        }
        if !pfound_0.is_null() {
            option_index_0 = indfound_0;
            if *nameend_0 != 0 {
                if (*pfound_0).has_arg != 0 {
                    (*d).rpl_optarg = nameend_0.offset(1 as libc::c_int as isize);
                } else {
                    if print_errors != 0 {
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s: option `-W %s' doesn't allow an argument\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            *argv.offset(0 as libc::c_int as isize),
                            (*pfound_0).name,
                        );
                    }
                    (*d)
                        .__nextchar = ((*d).__nextchar)
                        .offset(strlen((*d).__nextchar) as isize);
                    return '?' as i32;
                }
            } else if (*pfound_0).has_arg == 1 as libc::c_int {
                if (*d).rpl_optind < argc {
                    let fresh8 = (*d).rpl_optind;
                    (*d).rpl_optind = (*d).rpl_optind + 1;
                    (*d).rpl_optarg = *argv.offset(fresh8 as isize);
                } else {
                    if print_errors != 0 {
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s: option `%s' requires an argument\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            *argv.offset(0 as libc::c_int as isize),
                            *argv.offset(((*d).rpl_optind - 1 as libc::c_int) as isize),
                        );
                    }
                    (*d)
                        .__nextchar = ((*d).__nextchar)
                        .offset(strlen((*d).__nextchar) as isize);
                    return if *optstring.offset(0 as libc::c_int as isize) as libc::c_int
                        == ':' as i32
                    {
                        ':' as i32
                    } else {
                        '?' as i32
                    };
                }
            }
            (*d).__nextchar = ((*d).__nextchar).offset(strlen((*d).__nextchar) as isize);
            if !longind.is_null() {
                *longind = option_index_0;
            }
            if !((*pfound_0).flag).is_null() {
                *(*pfound_0).flag = (*pfound_0).val;
                return 0 as libc::c_int;
            }
            return (*pfound_0).val;
        }
        (*d).__nextchar = 0 as *mut libc::c_char;
        return 'W' as i32;
    }
    if *temp.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32 {
        if *temp.offset(2 as libc::c_int as isize) as libc::c_int == ':' as i32 {
            if *(*d).__nextchar as libc::c_int != '\0' as i32 {
                (*d).rpl_optarg = (*d).__nextchar;
                (*d).rpl_optind += 1;
                (*d).rpl_optind;
            } else {
                (*d).rpl_optarg = 0 as *mut libc::c_char;
            }
            (*d).__nextchar = 0 as *mut libc::c_char;
        } else {
            if *(*d).__nextchar as libc::c_int != '\0' as i32 {
                (*d).rpl_optarg = (*d).__nextchar;
                (*d).rpl_optind += 1;
                (*d).rpl_optind;
            } else if (*d).rpl_optind == argc {
                if print_errors != 0 {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: option requires an argument -- %c\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        *argv.offset(0 as libc::c_int as isize),
                        c as libc::c_int,
                    );
                }
                (*d).rpl_optopt = c as libc::c_int;
                if *optstring.offset(0 as libc::c_int as isize) as libc::c_int
                    == ':' as i32
                {
                    c = ':' as i32 as libc::c_char;
                } else {
                    c = '?' as i32 as libc::c_char;
                }
            } else {
                let fresh9 = (*d).rpl_optind;
                (*d).rpl_optind = (*d).rpl_optind + 1;
                (*d).rpl_optarg = *argv.offset(fresh9 as isize);
            }
            (*d).__nextchar = 0 as *mut libc::c_char;
        }
    }
    return c as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rpl_getopt_internal(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut optstring: *const libc::c_char,
    mut longopts: *const rpl_option,
    mut longind: *mut libc::c_int,
    mut long_only: libc::c_int,
    mut posixly_correct: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    getopt_data.rpl_optind = rpl_optind;
    getopt_data.rpl_opterr = rpl_opterr;
    result = _getopt_internal_r(
        argc,
        argv,
        optstring,
        longopts,
        longind,
        long_only,
        posixly_correct,
        &mut getopt_data,
    );
    rpl_optind = getopt_data.rpl_optind;
    rpl_optarg = getopt_data.rpl_optarg;
    rpl_optopt = getopt_data.rpl_optopt;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn rpl_getopt(
    mut argc: libc::c_int,
    mut argv: *const *mut libc::c_char,
    mut optstring: *const libc::c_char,
) -> libc::c_int {
    return rpl_getopt_internal(
        argc,
        argv as *mut *mut libc::c_char,
        optstring,
        0 as *const rpl_option,
        0 as *mut libc::c_int,
        0 as libc::c_int,
        POSIXLY_CORRECT as libc::c_int,
    );
}
