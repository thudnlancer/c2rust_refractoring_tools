#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn pma_free(ptr: *mut libc::c_void);
    fn pma_malloc(size: size_t) -> *mut libc::c_void;
    fn getenv(__name: *const i8) -> *mut i8;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fputc(__c: i32, __stream: *mut FILE) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
}
pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
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
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const i8,
    pub has_arg: i32,
    pub flag: *mut i32,
    pub val: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _getopt_data {
    pub optind: i32,
    pub opterr: i32,
    pub optopt: i32,
    pub optarg: *mut i8,
    pub __initialized: i32,
    pub __nextchar: *mut i8,
    pub __ordering: C2RustUnnamed,
    pub __posixly_correct: i32,
    pub __first_nonopt: i32,
    pub __last_nonopt: i32,
}
pub type C2RustUnnamed = u32;
pub const RETURN_IN_ORDER: C2RustUnnamed = 2;
pub const PERMUTE: C2RustUnnamed = 1;
pub const REQUIRE_ORDER: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option_list {
    pub p: *const option,
    pub next: *mut option_list,
    pub needs_free: i32,
}
#[no_mangle]
pub static mut optarg: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut optind: i32 = 1 as i32;
#[no_mangle]
pub static mut opterr: i32 = 1 as i32;
#[no_mangle]
pub static mut optopt: i32 = '?' as i32;
static mut getopt_data: _getopt_data = _getopt_data {
    optind: 0,
    opterr: 0,
    optopt: 0,
    optarg: 0 as *const i8 as *mut i8,
    __initialized: 0,
    __nextchar: 0 as *const i8 as *mut i8,
    __ordering: REQUIRE_ORDER,
    __posixly_correct: 0,
    __first_nonopt: 0,
    __last_nonopt: 0,
};
unsafe extern "C" fn exchange(mut argv: *mut *mut i8, mut d: *mut _getopt_data) {
    let mut bottom: i32 = (*d).__first_nonopt;
    let mut middle: i32 = (*d).__last_nonopt;
    let mut top: i32 = (*d).optind;
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
    (*d).__first_nonopt += (*d).optind - (*d).__last_nonopt;
    (*d).__last_nonopt = (*d).optind;
}
unsafe extern "C" fn _getopt_initialize(
    mut argc: i32,
    mut argv: *const *mut i8,
    mut optstring: *const i8,
    mut d: *mut _getopt_data,
    mut posixly_correct: i32,
) -> *const i8 {
    (*d).__last_nonopt = (*d).optind;
    (*d).__first_nonopt = (*d).__last_nonopt;
    (*d).__nextchar = 0 as *mut i8;
    (*d).__posixly_correct = posixly_correct
        | !(getenv(b"POSIXLY_CORRECT\0" as *const u8 as *const i8)).is_null() as i32;
    if *optstring.offset(0 as i32 as isize) as i32 == '-' as i32 {
        (*d).__ordering = RETURN_IN_ORDER;
        optstring = optstring.offset(1);
        optstring;
    } else if *optstring.offset(0 as i32 as isize) as i32 == '+' as i32 {
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
    mut argc: i32,
    mut argv: *const *mut i8,
    mut optstring: *const i8,
    mut longopts: *const option,
    mut longind: *mut i32,
    mut long_only: i32,
    mut d: *mut _getopt_data,
    mut posixly_correct: i32,
) -> i32 {
    let mut print_errors: i32 = (*d).opterr;
    if argc < 1 as i32 {
        return -(1 as i32);
    }
    (*d).optarg = 0 as *mut i8;
    if (*d).optind == 0 as i32 || (*d).__initialized == 0 {
        if (*d).optind == 0 as i32 {
            (*d).optind = 1 as i32;
        }
        optstring = _getopt_initialize(argc, argv, optstring, d, posixly_correct);
        (*d).__initialized = 1 as i32;
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
        if (*d).__last_nonopt > (*d).optind {
            (*d).__last_nonopt = (*d).optind;
        }
        if (*d).__first_nonopt > (*d).optind {
            (*d).__first_nonopt = (*d).optind;
        }
        if (*d).__ordering as u32 == PERMUTE as i32 as u32 {
            if (*d).__first_nonopt != (*d).__last_nonopt
                && (*d).__last_nonopt != (*d).optind
            {
                exchange(argv as *mut *mut i8, d);
            } else if (*d).__last_nonopt != (*d).optind {
                (*d).__first_nonopt = (*d).optind;
            }
            while (*d).optind < argc
                && (*(*argv.offset((*d).optind as isize)).offset(0 as i32 as isize)
                    as i32 != '-' as i32
                    || *(*argv.offset((*d).optind as isize)).offset(1 as i32 as isize)
                        as i32 == '\0' as i32)
            {
                (*d).optind += 1;
                (*d).optind;
            }
            (*d).__last_nonopt = (*d).optind;
        }
        if (*d).optind != argc
            && strcmp(
                *argv.offset((*d).optind as isize),
                b"--\0" as *const u8 as *const i8,
            ) == 0
        {
            (*d).optind += 1;
            (*d).optind;
            if (*d).__first_nonopt != (*d).__last_nonopt
                && (*d).__last_nonopt != (*d).optind
            {
                exchange(argv as *mut *mut i8, d);
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
            return -(1 as i32);
        }
        if *(*argv.offset((*d).optind as isize)).offset(0 as i32 as isize) as i32
            != '-' as i32
            || *(*argv.offset((*d).optind as isize)).offset(1 as i32 as isize) as i32
                == '\0' as i32
        {
            if (*d).__ordering as u32 == REQUIRE_ORDER as i32 as u32 {
                return -(1 as i32);
            }
            let fresh4 = (*d).optind;
            (*d).optind = (*d).optind + 1;
            (*d).optarg = *argv.offset(fresh4 as isize);
            return 1 as i32;
        }
        (*d).__nextchar = (*argv.offset((*d).optind as isize))
            .offset(1 as i32 as isize)
            .offset(
                (!longopts.is_null()
                    && *(*argv.offset((*d).optind as isize)).offset(1 as i32 as isize)
                        as i32 == '-' as i32) as i32 as isize,
            );
    }
    if !longopts.is_null()
        && (*(*argv.offset((*d).optind as isize)).offset(1 as i32 as isize) as i32
            == '-' as i32
            || long_only != 0
                && (*(*argv.offset((*d).optind as isize)).offset(2 as i32 as isize)
                    as i32 != 0
                    || (strchr(
                        optstring,
                        *(*argv.offset((*d).optind as isize)).offset(1 as i32 as isize)
                            as i32,
                    ))
                        .is_null()))
    {
        let mut nameend: *mut i8 = 0 as *mut i8;
        let mut namelen: u32 = 0;
        let mut p: *const option = 0 as *const option;
        let mut pfound: *const option = 0 as *const option;
        let mut ambig_list: *mut option_list = 0 as *mut option_list;
        let mut exact: i32 = 0 as i32;
        let mut indfound: i32 = -(1 as i32);
        let mut option_index: i32 = 0;
        nameend = (*d).__nextchar;
        while *nameend as i32 != 0 && *nameend as i32 != '=' as i32 {
            nameend = nameend.offset(1);
            nameend;
        }
        namelen = nameend.offset_from((*d).__nextchar) as i64 as u32;
        p = longopts;
        option_index = 0 as i32;
        while !((*p).name).is_null() {
            if strncmp((*p).name, (*d).__nextchar, namelen as u64) == 0 {
                if namelen == strlen((*p).name) as u32 {
                    pfound = p;
                    indfound = option_index;
                    exact = 1 as i32;
                    break;
                } else if pfound.is_null() {
                    pfound = p;
                    indfound = option_index;
                } else if long_only != 0 || (*pfound).has_arg != (*p).has_arg
                    || (*pfound).flag != (*p).flag || (*pfound).val != (*p).val
                {
                    let mut newp: *mut option_list = pma_malloc(
                        ::core::mem::size_of::<option_list>() as u64,
                    ) as *mut option_list;
                    (*newp).p = p;
                    (*newp).needs_free = 1 as i32;
                    (*newp).next = ambig_list;
                    ambig_list = newp;
                }
            }
            p = p.offset(1);
            p;
            option_index += 1;
            option_index;
        }
        if !ambig_list.is_null() && exact == 0 {
            if print_errors != 0 {
                let mut first: option_list = option_list {
                    p: 0 as *const option,
                    next: 0 as *mut option_list,
                    needs_free: 0,
                };
                first.p = pfound;
                first.next = ambig_list;
                first.needs_free = 0 as i32;
                ambig_list = &mut first;
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"%s: option '%s' is ambiguous; possibilities:\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    *argv.offset(0 as i32 as isize),
                    *argv.offset((*d).optind as isize),
                );
                loop {
                    let mut tmp_next: *mut option_list = 0 as *mut option_list;
                    fprintf(
                        stderr,
                        b" '--%s'\0" as *const u8 as *const i8,
                        (*(*ambig_list).p).name,
                    );
                    tmp_next = (*ambig_list).next;
                    if (*ambig_list).needs_free != 0 {
                        pma_free(ambig_list as *mut libc::c_void);
                    }
                    ambig_list = tmp_next;
                    if ambig_list.is_null() {
                        break;
                    }
                }
                fputc('\n' as i32, stderr);
            }
            (*d).__nextchar = ((*d).__nextchar).offset(strlen((*d).__nextchar) as isize);
            (*d).optind += 1;
            (*d).optind;
            (*d).optopt = 0 as i32;
            return '?' as i32;
        }
        if !pfound.is_null() {
            option_index = indfound;
            (*d).optind += 1;
            (*d).optind;
            if *nameend != 0 {
                if (*pfound).has_arg != 0 {
                    (*d).optarg = nameend.offset(1 as i32 as isize);
                } else {
                    if print_errors != 0 {
                        if *(*argv.offset(((*d).optind - 1 as i32) as isize))
                            .offset(1 as i32 as isize) as i32 == '-' as i32
                        {
                            fprintf(
                                stderr,
                                dcgettext(
                                    0 as *const i8,
                                    b"%s: option '--%s' doesn't allow an argument\n\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                                *argv.offset(0 as i32 as isize),
                                (*pfound).name,
                            );
                        } else {
                            fprintf(
                                stderr,
                                dcgettext(
                                    0 as *const i8,
                                    b"%s: option '%c%s' doesn't allow an argument\n\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                                *argv.offset(0 as i32 as isize),
                                *(*argv.offset(((*d).optind - 1 as i32) as isize))
                                    .offset(0 as i32 as isize) as i32,
                                (*pfound).name,
                            );
                        }
                    }
                    (*d).__nextchar = ((*d).__nextchar)
                        .offset(strlen((*d).__nextchar) as isize);
                    (*d).optopt = (*pfound).val;
                    return '?' as i32;
                }
            } else if (*pfound).has_arg == 1 as i32 {
                if (*d).optind < argc {
                    let fresh5 = (*d).optind;
                    (*d).optind = (*d).optind + 1;
                    (*d).optarg = *argv.offset(fresh5 as isize);
                } else {
                    if print_errors != 0 {
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const i8,
                                b"%s: option '--%s' requires an argument\n\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                            *argv.offset(0 as i32 as isize),
                            (*pfound).name,
                        );
                    }
                    (*d).__nextchar = ((*d).__nextchar)
                        .offset(strlen((*d).__nextchar) as isize);
                    (*d).optopt = (*pfound).val;
                    return if *optstring.offset(0 as i32 as isize) as i32 == ':' as i32 {
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
                return 0 as i32;
            }
            return (*pfound).val;
        }
        if long_only == 0
            || *(*argv.offset((*d).optind as isize)).offset(1 as i32 as isize) as i32
                == '-' as i32 || (strchr(optstring, *(*d).__nextchar as i32)).is_null()
        {
            if print_errors != 0 {
                if *(*argv.offset((*d).optind as isize)).offset(1 as i32 as isize) as i32
                    == '-' as i32
                {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"%s: unrecognized option '--%s'\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        *argv.offset(0 as i32 as isize),
                        (*d).__nextchar,
                    );
                } else {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"%s: unrecognized option '%c%s'\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        *argv.offset(0 as i32 as isize),
                        *(*argv.offset((*d).optind as isize)).offset(0 as i32 as isize)
                            as i32,
                        (*d).__nextchar,
                    );
                }
            }
            (*d).__nextchar = b"\0" as *const u8 as *const i8 as *mut i8;
            (*d).optind += 1;
            (*d).optind;
            (*d).optopt = 0 as i32;
            return '?' as i32;
        }
    }
    let fresh6 = (*d).__nextchar;
    (*d).__nextchar = ((*d).__nextchar).offset(1);
    let mut c: i8 = *fresh6;
    let mut temp: *mut i8 = strchr(optstring, c as i32);
    if *(*d).__nextchar as i32 == '\0' as i32 {
        (*d).optind += 1;
        (*d).optind;
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
        (*d).optopt = c as i32;
        return '?' as i32;
    }
    if *temp.offset(0 as i32 as isize) as i32 == 'W' as i32
        && *temp.offset(1 as i32 as isize) as i32 == ';' as i32
    {
        let mut nameend_0: *mut i8 = 0 as *mut i8;
        let mut p_0: *const option = 0 as *const option;
        let mut pfound_0: *const option = 0 as *const option;
        let mut exact_0: i32 = 0 as i32;
        let mut ambig: i32 = 0 as i32;
        let mut indfound_0: i32 = 0 as i32;
        let mut option_index_0: i32 = 0;
        if !longopts.is_null() {
            if *(*d).__nextchar as i32 != '\0' as i32 {
                (*d).optarg = (*d).__nextchar;
                (*d).optind += 1;
                (*d).optind;
            } else if (*d).optind == argc {
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
                (*d).optopt = c as i32;
                if *optstring.offset(0 as i32 as isize) as i32 == ':' as i32 {
                    c = ':' as i32 as i8;
                } else {
                    c = '?' as i32 as i8;
                }
                return c as i32;
            } else {
                let fresh7 = (*d).optind;
                (*d).optind = (*d).optind + 1;
                (*d).optarg = *argv.offset(fresh7 as isize);
            }
            nameend_0 = (*d).optarg;
            (*d).__nextchar = nameend_0;
            while *nameend_0 as i32 != 0 && *nameend_0 as i32 != '=' as i32 {
                nameend_0 = nameend_0.offset(1);
                nameend_0;
            }
            p_0 = longopts;
            option_index_0 = 0 as i32;
            while !((*p_0).name).is_null() {
                if strncmp(
                    (*p_0).name,
                    (*d).__nextchar,
                    nameend_0.offset_from((*d).__nextchar) as i64 as u64,
                ) == 0
                {
                    if nameend_0.offset_from((*d).__nextchar) as i64 as u32 as u64
                        == strlen((*p_0).name)
                    {
                        pfound_0 = p_0;
                        indfound_0 = option_index_0;
                        exact_0 = 1 as i32;
                        break;
                    } else if pfound_0.is_null() {
                        pfound_0 = p_0;
                        indfound_0 = option_index_0;
                    } else if long_only != 0 || (*pfound_0).has_arg != (*p_0).has_arg
                        || (*pfound_0).flag != (*p_0).flag
                        || (*pfound_0).val != (*p_0).val
                    {
                        ambig = 1 as i32;
                    }
                }
                p_0 = p_0.offset(1);
                p_0;
                option_index_0 += 1;
                option_index_0;
            }
            if ambig != 0 && exact_0 == 0 {
                if print_errors != 0 {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"%s: option '-W %s' is ambiguous\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        *argv.offset(0 as i32 as isize),
                        (*d).optarg,
                    );
                }
                (*d).__nextchar = ((*d).__nextchar)
                    .offset(strlen((*d).__nextchar) as isize);
                (*d).optind += 1;
                (*d).optind;
                return '?' as i32;
            }
            if !pfound_0.is_null() {
                option_index_0 = indfound_0;
                if *nameend_0 != 0 {
                    if (*pfound_0).has_arg != 0 {
                        (*d).optarg = nameend_0.offset(1 as i32 as isize);
                    } else {
                        if print_errors != 0 {
                            fprintf(
                                stderr,
                                dcgettext(
                                    0 as *const i8,
                                    b"%s: option '-W %s' doesn't allow an argument\n\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                                *argv.offset(0 as i32 as isize),
                                (*pfound_0).name,
                            );
                        }
                        (*d).__nextchar = ((*d).__nextchar)
                            .offset(strlen((*d).__nextchar) as isize);
                        return '?' as i32;
                    }
                } else if (*pfound_0).has_arg == 1 as i32 {
                    if (*d).optind < argc {
                        let fresh8 = (*d).optind;
                        (*d).optind = (*d).optind + 1;
                        (*d).optarg = *argv.offset(fresh8 as isize);
                    } else {
                        if print_errors != 0 {
                            fprintf(
                                stderr,
                                dcgettext(
                                    0 as *const i8,
                                    b"%s: option '-W %s' requires an argument\n\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                                *argv.offset(0 as i32 as isize),
                                (*pfound_0).name,
                            );
                        }
                        (*d).__nextchar = ((*d).__nextchar)
                            .offset(strlen((*d).__nextchar) as isize);
                        return if *optstring.offset(0 as i32 as isize) as i32
                            == ':' as i32
                        {
                            ':' as i32
                        } else {
                            '?' as i32
                        };
                    }
                } else {
                    (*d).optarg = 0 as *mut i8;
                }
                (*d).__nextchar = ((*d).__nextchar)
                    .offset(strlen((*d).__nextchar) as isize);
                if !longind.is_null() {
                    *longind = option_index_0;
                }
                if !((*pfound_0).flag).is_null() {
                    *(*pfound_0).flag = (*pfound_0).val;
                    return 0 as i32;
                }
                return (*pfound_0).val;
            }
        }
        (*d).__nextchar = 0 as *mut i8;
        return 'W' as i32;
    }
    if *temp.offset(1 as i32 as isize) as i32 == ':' as i32 {
        if *temp.offset(2 as i32 as isize) as i32 == ':' as i32 {
            if *(*d).__nextchar as i32 != '\0' as i32 {
                (*d).optarg = (*d).__nextchar;
                (*d).optind += 1;
                (*d).optind;
            } else {
                (*d).optarg = 0 as *mut i8;
            }
            (*d).__nextchar = 0 as *mut i8;
        } else {
            if *(*d).__nextchar as i32 != '\0' as i32 {
                (*d).optarg = (*d).__nextchar;
                (*d).optind += 1;
                (*d).optind;
            } else if (*d).optind == argc {
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
                (*d).optopt = c as i32;
                if *optstring.offset(0 as i32 as isize) as i32 == ':' as i32 {
                    c = ':' as i32 as i8;
                } else {
                    c = '?' as i32 as i8;
                }
            } else {
                let fresh9 = (*d).optind;
                (*d).optind = (*d).optind + 1;
                (*d).optarg = *argv.offset(fresh9 as isize);
            }
            (*d).__nextchar = 0 as *mut i8;
        }
    }
    return c as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _getopt_internal(
    mut argc: i32,
    mut argv: *const *mut i8,
    mut optstring: *const i8,
    mut longopts: *const option,
    mut longind: *mut i32,
    mut long_only: i32,
    mut posixly_correct: i32,
) -> i32 {
    let mut result: i32 = 0;
    getopt_data.optind = optind;
    getopt_data.opterr = opterr;
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
    optind = getopt_data.optind;
    optarg = getopt_data.optarg;
    optopt = getopt_data.optopt;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn getopt(
    mut argc: i32,
    mut argv: *const *mut i8,
    mut optstring: *const i8,
) -> i32 {
    return _getopt_internal(
        argc,
        argv,
        optstring,
        0 as *const option,
        0 as *mut i32,
        0 as i32,
        0 as i32,
    );
}