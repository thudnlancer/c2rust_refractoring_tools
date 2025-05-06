#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, label_break_value)]
extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    static mut program_invocation_short_name: *mut i8;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn exit(_: i32) -> !;
    fn getenv(__name: *const i8) -> *mut i8;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn rpl_free(ptr: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strndup(_: *const i8, _: u64) -> *mut i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strchrnul(__s: *const i8, __c: i32) -> *mut i8;
    fn mempcpy(
        __dest: *mut libc::c_void,
        __src: *const libc::c_void,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn strerror_r(__errnum: i32, __buf: *mut i8, __buflen: size_t) -> *mut i8;
    fn strcasecmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn __overflow(_: *mut _IO_FILE, _: i32) -> i32;
    static mut stderr: *mut _IO_FILE;
    fn vfprintf(_: *mut FILE, _: *const i8, _: ::core::ffi::VaList) -> i32;
    fn fputs_unlocked(__s: *const i8, __stream: *mut FILE) -> i32;
    static mut argp_program_bug_address: *const i8;
    static mut argp_err_exit_status: error_t;
    fn _argp_input(__argp: *const argp, __state: *const argp_state) -> *mut libc::c_void;
    fn flockfile(__stream: *mut FILE);
    fn funlockfile(__stream: *mut FILE);
    fn argp_make_fmtstream(
        __stream: *mut FILE,
        __lmargin: size_t,
        __rmargin: size_t,
        __wmargin: ssize_t,
    ) -> argp_fmtstream_t;
    fn argp_fmtstream_free(__fs: argp_fmtstream_t);
    fn _argp_fmtstream_ensure(__fs: argp_fmtstream_t, __amount: size_t) -> i32;
    fn argp_fmtstream_printf(
        __fs: argp_fmtstream_t,
        __fmt: *const i8,
        _: ...
    ) -> ssize_t;
    fn _argp_fmtstream_update(__fs: argp_fmtstream_t);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type error_t = i32;
pub type size_t = u64;
pub type __int32_t = i32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __ssize_t = i64;
pub type ssize_t = __ssize_t;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
pub type va_list = __builtin_va_list;
pub type C2RustUnnamed = u32;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub struct argp_option {
    pub name: *const i8,
    pub key: i32,
    pub arg: *const i8,
    pub flags: i32,
    pub doc: *const i8,
    pub group: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp {
    pub options: *const argp_option,
    pub parser: argp_parser_t,
    pub args_doc: *const i8,
    pub doc: *const i8,
    pub children: *const argp_child,
    pub help_filter: Option<
        unsafe extern "C" fn(i32, *const i8, *mut libc::c_void) -> *mut i8,
    >,
    pub argp_domain: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_child {
    pub argp: *const argp,
    pub flags: i32,
    pub header: *const i8,
    pub group: i32,
}
pub type argp_parser_t = Option<
    unsafe extern "C" fn(i32, *mut i8, *mut argp_state) -> error_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_state {
    pub root_argp: *const argp,
    pub argc: i32,
    pub argv: *mut *mut i8,
    pub next: i32,
    pub flags: u32,
    pub arg_num: u32,
    pub quoted: i32,
    pub input: *mut libc::c_void,
    pub child_inputs: *mut *mut libc::c_void,
    pub hook: *mut libc::c_void,
    pub name: *mut i8,
    pub err_stream: *mut FILE,
    pub out_stream: *mut FILE,
    pub pstate: *mut libc::c_void,
}
pub type argp_fmtstream_t = *mut argp_fmtstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_fmtstream {
    pub stream: *mut FILE,
    pub lmargin: size_t,
    pub rmargin: size_t,
    pub wmargin: ssize_t,
    pub point_offs: size_t,
    pub point_col: ssize_t,
    pub buf: *mut i8,
    pub p: *mut i8,
    pub end: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hol {
    pub entries: *mut hol_entry,
    pub num_entries: u32,
    pub short_options: *mut i8,
    pub clusters: *mut hol_cluster,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hol_cluster {
    pub header: *const i8,
    pub index: i32,
    pub group: i32,
    pub parent: *mut hol_cluster,
    pub argp: *const argp,
    pub depth: i32,
    pub next: *mut hol_cluster,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hol_entry {
    pub opt: *const argp_option,
    pub num: u32,
    pub short_options: *mut i8,
    pub group: i32,
    pub cluster: *mut hol_cluster,
    pub argp: *const argp,
    pub ord: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uparams {
    pub dup_args: i32,
    pub dup_args_note: i32,
    pub short_opt_col: i32,
    pub long_opt_col: i32,
    pub doc_opt_col: i32,
    pub opt_doc_col: i32,
    pub header_col: i32,
    pub usage_indent: i32,
    pub rmargin: i32,
    pub valid: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hol_help_state {
    pub prev_entry: *mut hol_entry,
    pub sep_groups: i32,
    pub suppressed_dup_arg: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pentry_state {
    pub entry: *const hol_entry,
    pub stream: argp_fmtstream_t,
    pub hhstate: *mut hol_help_state,
    pub first: i32,
    pub state: *const argp_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uparam_name {
    pub name: [i8; 14],
    pub is_bool: bool,
    pub uparams_offs: u8,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut i8, 10 as i32) as i32;
}
#[inline]
unsafe extern "C" fn tolower(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn putc_unlocked(mut __c: i32, mut __stream: *mut FILE) -> i32 {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as i32 as i64 != 0
    {
        __overflow(__stream, __c as u8 as i32)
    } else {
        let fresh0 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
        *fresh0 = __c as i8;
        *fresh0 as u8 as i32
    };
}
#[inline]
unsafe extern "C" fn _option_is_end(mut __opt: *const argp_option) -> i32 {
    return ((*__opt).key == 0 && ((*__opt).name).is_null() && ((*__opt).doc).is_null()
        && (*__opt).group == 0) as i32;
}
#[inline]
unsafe extern "C" fn _option_is_short(mut __opt: *const argp_option) -> i32 {
    if (*__opt).flags & 0x8 as i32 != 0 {
        return 0 as i32
    } else {
        let mut __key: i32 = (*__opt).key;
        return (__key > 0 as i32 && __key <= 127 as i32 * 2 as i32 + 1 as i32
            && *(*__ctype_b_loc()).offset(__key as isize) as i32
                & _ISprint as i32 as libc::c_ushort as i32 != 0) as i32;
    };
}
#[inline]
unsafe extern "C" fn argp_fmtstream_write(
    mut __fs: argp_fmtstream_t,
    mut __str: *const i8,
    mut __len: size_t,
) -> size_t {
    if ((*__fs).p).offset(__len as isize) <= (*__fs).end
        || _argp_fmtstream_ensure(__fs, __len) != 0
    {
        memcpy((*__fs).p as *mut libc::c_void, __str as *const libc::c_void, __len);
        (*__fs).p = ((*__fs).p).offset(__len as isize);
        return __len;
    } else {
        return 0 as i32 as size_t
    };
}
#[inline]
unsafe extern "C" fn argp_fmtstream_puts(
    mut __fs: argp_fmtstream_t,
    mut __str: *const i8,
) -> i32 {
    let mut __len: size_t = strlen(__str);
    if __len != 0 {
        let mut __wrote: size_t = argp_fmtstream_write(__fs, __str, __len);
        return if __wrote == __len { 0 as i32 } else { -(1 as i32) };
    } else {
        return 0 as i32
    };
}
#[inline]
unsafe extern "C" fn argp_fmtstream_putc(
    mut __fs: argp_fmtstream_t,
    mut __ch: i32,
) -> i32 {
    if (*__fs).p < (*__fs).end || _argp_fmtstream_ensure(__fs, 1 as i32 as size_t) != 0 {
        let fresh1 = (*__fs).p;
        (*__fs).p = ((*__fs).p).offset(1);
        *fresh1 = __ch as i8;
        return *fresh1 as i32;
    } else {
        return -(1 as i32)
    };
}
#[inline]
unsafe extern "C" fn argp_fmtstream_set_lmargin(
    mut __fs: argp_fmtstream_t,
    mut __lmargin: size_t,
) -> size_t {
    let mut __old: size_t = 0;
    if ((*__fs).p).offset_from((*__fs).buf) as i64 as size_t > (*__fs).point_offs {
        _argp_fmtstream_update(__fs);
    }
    __old = (*__fs).lmargin;
    (*__fs).lmargin = __lmargin;
    return __old;
}
#[inline]
unsafe extern "C" fn argp_fmtstream_set_wmargin(
    mut __fs: argp_fmtstream_t,
    mut __wmargin: size_t,
) -> size_t {
    let mut __old: size_t = 0;
    if ((*__fs).p).offset_from((*__fs).buf) as i64 as size_t > (*__fs).point_offs {
        _argp_fmtstream_update(__fs);
    }
    __old = (*__fs).wmargin as size_t;
    (*__fs).wmargin = __wmargin as ssize_t;
    return __old;
}
#[inline]
unsafe extern "C" fn argp_fmtstream_point(mut __fs: argp_fmtstream_t) -> size_t {
    if ((*__fs).p).offset_from((*__fs).buf) as i64 as size_t > (*__fs).point_offs {
        _argp_fmtstream_update(__fs);
    }
    return (if (*__fs).point_col >= 0 as i32 as i64 {
        (*__fs).point_col
    } else {
        0 as i32 as i64
    }) as size_t;
}
static mut uparams: uparams = {
    let mut init = uparams {
        dup_args: 0 as i32,
        dup_args_note: 1 as i32,
        short_opt_col: 2 as i32,
        long_opt_col: 6 as i32,
        doc_opt_col: 2 as i32,
        opt_doc_col: 29 as i32,
        header_col: 1 as i32,
        usage_indent: 12 as i32,
        rmargin: 79 as i32,
        valid: 0,
    };
    init
};
static mut uparam_names: [uparam_name; 9] = unsafe {
    [
        {
            let mut init = uparam_name {
                name: *::core::mem::transmute::<
                    &[u8; 14],
                    &[i8; 14],
                >(b"dup-args\0\0\0\0\0\0"),
                is_bool: 1 as i32 != 0,
                uparams_offs: 0 as u64 as u8,
            };
            init
        },
        {
            let mut init = uparam_name {
                name: *::core::mem::transmute::<
                    &[u8; 14],
                    &[i8; 14],
                >(b"dup-args-note\0"),
                is_bool: 1 as i32 != 0,
                uparams_offs: 4 as u64 as u8,
            };
            init
        },
        {
            let mut init = uparam_name {
                name: *::core::mem::transmute::<
                    &[u8; 14],
                    &[i8; 14],
                >(b"short-opt-col\0"),
                is_bool: 0 as i32 != 0,
                uparams_offs: 8 as u64 as u8,
            };
            init
        },
        {
            let mut init = uparam_name {
                name: *::core::mem::transmute::<
                    &[u8; 14],
                    &[i8; 14],
                >(b"long-opt-col\0\0"),
                is_bool: 0 as i32 != 0,
                uparams_offs: 12 as u64 as u8,
            };
            init
        },
        {
            let mut init = uparam_name {
                name: *::core::mem::transmute::<
                    &[u8; 14],
                    &[i8; 14],
                >(b"doc-opt-col\0\0\0"),
                is_bool: 0 as i32 != 0,
                uparams_offs: 16 as u64 as u8,
            };
            init
        },
        {
            let mut init = uparam_name {
                name: *::core::mem::transmute::<
                    &[u8; 14],
                    &[i8; 14],
                >(b"opt-doc-col\0\0\0"),
                is_bool: 0 as i32 != 0,
                uparams_offs: 20 as u64 as u8,
            };
            init
        },
        {
            let mut init = uparam_name {
                name: *::core::mem::transmute::<
                    &[u8; 14],
                    &[i8; 14],
                >(b"header-col\0\0\0\0"),
                is_bool: 0 as i32 != 0,
                uparams_offs: 24 as u64 as u8,
            };
            init
        },
        {
            let mut init = uparam_name {
                name: *::core::mem::transmute::<
                    &[u8; 14],
                    &[i8; 14],
                >(b"usage-indent\0\0"),
                is_bool: 0 as i32 != 0,
                uparams_offs: 28 as u64 as u8,
            };
            init
        },
        {
            let mut init = uparam_name {
                name: *::core::mem::transmute::<
                    &[u8; 14],
                    &[i8; 14],
                >(b"rmargin\0\0\0\0\0\0\0"),
                is_bool: 0 as i32 != 0,
                uparams_offs: 32 as u64 as u8,
            };
            init
        },
    ]
};
unsafe extern "C" fn validate_uparams(
    mut state: *const argp_state,
    mut upptr: *mut uparams,
) {
    let mut up: *const uparam_name = 0 as *const uparam_name;
    up = uparam_names.as_ptr();
    while up
        < uparam_names
            .as_ptr()
            .offset(
                (::core::mem::size_of::<[uparam_name; 9]>() as u64)
                    .wrapping_div(::core::mem::size_of::<uparam_name>() as u64) as isize,
            )
    {
        if !((*up).is_bool as i32 != 0 || (*up).uparams_offs as u64 == 32 as u64) {
            if *((upptr as *mut i8).offset((*up).uparams_offs as i32 as isize)
                as *mut i32) >= (*upptr).rmargin
            {
                argp_failure(
                    state,
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        if state.is_null() {
                            0 as *const i8
                        } else {
                            (*(*state).root_argp).argp_domain
                        },
                        b"ARGP_HELP_FMT: %s value is less than or equal to %s\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    b"rmargin\0" as *const u8 as *const i8,
                    ((*up).name).as_ptr(),
                );
                return;
            }
        }
        up = up.offset(1);
        up;
    }
    uparams = *upptr;
    uparams.valid = 1 as i32;
}
unsafe extern "C" fn fill_in_uparams(mut state: *const argp_state) {
    let mut var: *const i8 = getenv(b"ARGP_HELP_FMT\0" as *const u8 as *const i8);
    let mut new_params: uparams = uparams;
    if !var.is_null() {
        while *var != 0 {
            while *(*__ctype_b_loc()).offset(*var as u8 as i32 as isize) as i32
                & _ISspace as i32 as libc::c_ushort as i32 != 0
            {
                var = var.offset(1);
                var;
            }
            if *(*__ctype_b_loc()).offset(*var as u8 as i32 as isize) as i32
                & _ISalpha as i32 as libc::c_ushort as i32 != 0
            {
                let mut var_len: size_t = 0;
                let mut un: *const uparam_name = 0 as *const uparam_name;
                let mut unspec: i32 = 0 as i32;
                let mut val: i32 = 0 as i32;
                let mut arg_0: *const i8 = var;
                while *(*__ctype_b_loc()).offset(*arg_0 as u8 as i32 as isize) as i32
                    & _ISalnum as i32 as libc::c_ushort as i32 != 0
                    || *arg_0 as i32 == '-' as i32 || *arg_0 as i32 == '_' as i32
                {
                    arg_0 = arg_0.offset(1);
                    arg_0;
                }
                var_len = arg_0.offset_from(var) as i64 as size_t;
                while *(*__ctype_b_loc()).offset(*arg_0 as u8 as i32 as isize) as i32
                    & _ISspace as i32 as libc::c_ushort as i32 != 0
                {
                    arg_0 = arg_0.offset(1);
                    arg_0;
                }
                if *arg_0 as i32 == '\0' as i32 || *arg_0 as i32 == ',' as i32 {
                    unspec = 1 as i32;
                } else if *arg_0 as i32 == '=' as i32 {
                    arg_0 = arg_0.offset(1);
                    arg_0;
                    while *(*__ctype_b_loc()).offset(*arg_0 as u8 as i32 as isize) as i32
                        & _ISspace as i32 as libc::c_ushort as i32 != 0
                    {
                        arg_0 = arg_0.offset(1);
                        arg_0;
                    }
                }
                if unspec != 0 {
                    if *var.offset(0 as i32 as isize) as i32 == 'n' as i32
                        && *var.offset(1 as i32 as isize) as i32 == 'o' as i32
                        && *var.offset(2 as i32 as isize) as i32 == '-' as i32
                    {
                        val = 0 as i32;
                        var = var.offset(3 as i32 as isize);
                        var_len = (var_len as u64).wrapping_sub(3 as i32 as u64)
                            as size_t as size_t;
                    } else {
                        val = 1 as i32;
                    }
                } else if *(*__ctype_b_loc()).offset(*arg_0 as u8 as i32 as isize) as i32
                    & _ISdigit as i32 as libc::c_ushort as i32 != 0
                {
                    val = atoi(arg_0);
                    while *(*__ctype_b_loc()).offset(*arg_0 as u8 as i32 as isize) as i32
                        & _ISdigit as i32 as libc::c_ushort as i32 != 0
                    {
                        arg_0 = arg_0.offset(1);
                        arg_0;
                    }
                    while *(*__ctype_b_loc()).offset(*arg_0 as u8 as i32 as isize) as i32
                        & _ISspace as i32 as libc::c_ushort as i32 != 0
                    {
                        arg_0 = arg_0.offset(1);
                        arg_0;
                    }
                }
                un = uparam_names.as_ptr();
                while un
                    < uparam_names
                        .as_ptr()
                        .offset(
                            (::core::mem::size_of::<[uparam_name; 9]>() as u64)
                                .wrapping_div(::core::mem::size_of::<uparam_name>() as u64)
                                as isize,
                        )
                {
                    if strlen(((*un).name).as_ptr()) == var_len
                        && strncmp(var, ((*un).name).as_ptr(), var_len) == 0 as i32
                    {
                        if unspec != 0 && !(*un).is_bool {
                            argp_failure(
                                state,
                                0 as i32,
                                0 as i32,
                                dcgettext(
                                    if state.is_null() {
                                        0 as *const i8
                                    } else {
                                        (*(*state).root_argp).argp_domain
                                    },
                                    b"%.*s: ARGP_HELP_FMT parameter requires a value\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                                var_len as i32,
                                var,
                            );
                        } else {
                            *((&mut new_params as *mut uparams as *mut i8)
                                .offset((*un).uparams_offs as i32 as isize) as *mut i32) = val;
                        }
                        break;
                    } else {
                        un = un.offset(1);
                        un;
                    }
                }
                if un
                    == uparam_names
                        .as_ptr()
                        .offset(
                            (::core::mem::size_of::<[uparam_name; 9]>() as u64)
                                .wrapping_div(::core::mem::size_of::<uparam_name>() as u64)
                                as isize,
                        )
                {
                    argp_failure(
                        state,
                        0 as i32,
                        0 as i32,
                        dcgettext(
                            if state.is_null() {
                                0 as *const i8
                            } else {
                                (*(*state).root_argp).argp_domain
                            },
                            b"%.*s: Unknown ARGP_HELP_FMT parameter\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        var_len as i32,
                        var,
                    );
                }
                var = arg_0;
                if *var as i32 == ',' as i32 {
                    var = var.offset(1);
                    var;
                }
            } else {
                if !(*var != 0) {
                    continue;
                }
                argp_failure(
                    state,
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        if state.is_null() {
                            0 as *const i8
                        } else {
                            (*(*state).root_argp).argp_domain
                        },
                        b"Garbage in ARGP_HELP_FMT: %s\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    var,
                );
                break;
            }
        }
        validate_uparams(state, &mut new_params);
    }
}
unsafe extern "C" fn find_char(mut ch: i8, mut beg: *mut i8, mut end: *mut i8) -> i32 {
    while beg < end {
        if *beg as i32 == ch as i32 {
            return 1 as i32
        } else {
            beg = beg.offset(1);
            beg;
        }
    }
    return 0 as i32;
}
unsafe extern "C" fn make_hol(
    mut argp: *const argp,
    mut cluster: *mut hol_cluster,
) -> *mut hol {
    let mut so: *mut i8 = 0 as *mut i8;
    let mut o: *const argp_option = 0 as *const argp_option;
    let mut opts: *const argp_option = (*argp).options;
    let mut entry: *mut hol_entry = 0 as *mut hol_entry;
    let mut num_short_options: u32 = 0 as i32 as u32;
    let mut hol: *mut hol = malloc(::core::mem::size_of::<hol>() as u64) as *mut hol;
    if !hol.is_null() {} else {
        __assert_fail(
            b"hol\0" as *const u8 as *const i8,
            b"argp-help.c\0" as *const u8 as *const i8,
            449 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 64],
                &[i8; 64],
            >(b"struct hol *make_hol(const struct argp *, struct hol_cluster *)\0"))
                .as_ptr(),
        );
    }
    'c_8949: {
        if !hol.is_null() {} else {
            __assert_fail(
                b"hol\0" as *const u8 as *const i8,
                b"argp-help.c\0" as *const u8 as *const i8,
                449 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 64],
                    &[i8; 64],
                >(b"struct hol *make_hol(const struct argp *, struct hol_cluster *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*hol).num_entries = 0 as i32 as u32;
    (*hol).clusters = 0 as *mut hol_cluster;
    if !opts.is_null() {
        let mut cur_group: i32 = 0 as i32;
        if (*opts).flags & 0x4 as i32 == 0 {} else {
            __assert_fail(
                b"! oalias (opts)\0" as *const u8 as *const i8,
                b"argp-help.c\0" as *const u8 as *const i8,
                459 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 64],
                    &[i8; 64],
                >(b"struct hol *make_hol(const struct argp *, struct hol_cluster *)\0"))
                    .as_ptr(),
            );
        }
        'c_8889: {
            if (*opts).flags & 0x4 as i32 == 0 {} else {
                __assert_fail(
                    b"! oalias (opts)\0" as *const u8 as *const i8,
                    b"argp-help.c\0" as *const u8 as *const i8,
                    459 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 64],
                        &[i8; 64],
                    >(
                        b"struct hol *make_hol(const struct argp *, struct hol_cluster *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        o = opts;
        while _option_is_end(o) == 0 {
            if (*o).flags & 0x4 as i32 == 0 {
                (*hol).num_entries = ((*hol).num_entries).wrapping_add(1);
                (*hol).num_entries;
            }
            if _option_is_short(o) != 0 {
                num_short_options = num_short_options.wrapping_add(1);
                num_short_options;
            }
            o = o.offset(1);
            o;
        }
        (*hol).entries = malloc(
            (::core::mem::size_of::<hol_entry>() as u64)
                .wrapping_mul((*hol).num_entries as u64),
        ) as *mut hol_entry;
        (*hol).short_options = malloc(
            num_short_options.wrapping_add(1 as i32 as u32) as u64,
        ) as *mut i8;
        if !((*hol).entries).is_null() && !((*hol).short_options).is_null() {} else {
            __assert_fail(
                b"hol->entries && hol->short_options\0" as *const u8 as *const i8,
                b"argp-help.c\0" as *const u8 as *const i8,
                473 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 64],
                    &[i8; 64],
                >(b"struct hol *make_hol(const struct argp *, struct hol_cluster *)\0"))
                    .as_ptr(),
            );
        }
        'c_8772: {
            if !((*hol).entries).is_null() && !((*hol).short_options).is_null() {} else {
                __assert_fail(
                    b"hol->entries && hol->short_options\0" as *const u8 as *const i8,
                    b"argp-help.c\0" as *const u8 as *const i8,
                    473 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 64],
                        &[i8; 64],
                    >(
                        b"struct hol *make_hol(const struct argp *, struct hol_cluster *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if -(1 as i32) as size_t
            <= (2147483647 as i32 as u32).wrapping_mul(2 as u32).wrapping_add(1 as u32)
                as u64
        {
            if (*hol).num_entries as u64
                <= (-(1 as i32) as size_t)
                    .wrapping_div(::core::mem::size_of::<hol_entry>() as u64)
            {} else {
                __assert_fail(
                    b"hol->num_entries <= SIZE_MAX / sizeof (struct hol_entry)\0"
                        as *const u8 as *const i8,
                    b"argp-help.c\0" as *const u8 as *const i8,
                    475 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 64],
                        &[i8; 64],
                    >(
                        b"struct hol *make_hol(const struct argp *, struct hol_cluster *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_8700: {
                if (*hol).num_entries as u64
                    <= (-(1 as i32) as size_t)
                        .wrapping_div(::core::mem::size_of::<hol_entry>() as u64)
                {} else {
                    __assert_fail(
                        b"hol->num_entries <= SIZE_MAX / sizeof (struct hol_entry)\0"
                            as *const u8 as *const i8,
                        b"argp-help.c\0" as *const u8 as *const i8,
                        475 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 64],
                            &[i8; 64],
                        >(
                            b"struct hol *make_hol(const struct argp *, struct hol_cluster *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
        }
        so = (*hol).short_options;
        o = opts;
        entry = (*hol).entries;
        while _option_is_end(o) == 0 {
            (*entry).opt = o;
            (*entry).num = 0 as i32 as u32;
            (*entry).short_options = so;
            cur_group = if (*o).group != 0 {
                (*o).group
            } else if ((*o).name).is_null() && (*o).key == 0 {
                cur_group + 1 as i32
            } else {
                cur_group
            };
            (*entry).group = cur_group;
            (*entry).cluster = cluster;
            (*entry).argp = argp;
            loop {
                (*entry).num = ((*entry).num).wrapping_add(1);
                (*entry).num;
                if _option_is_short(o) != 0
                    && find_char((*o).key as i8, (*hol).short_options, so) == 0
                {
                    let fresh2 = so;
                    so = so.offset(1);
                    *fresh2 = (*o).key as i8;
                }
                o = o.offset(1);
                o;
                if !(_option_is_end(o) == 0 && (*o).flags & 0x4 as i32 != 0) {
                    break;
                }
            }
            entry = entry.offset(1);
            entry;
        }
        *so = '\0' as i32 as i8;
    }
    return hol;
}
unsafe extern "C" fn hol_add_cluster(
    mut hol: *mut hol,
    mut group: i32,
    mut header: *const i8,
    mut index: i32,
    mut parent: *mut hol_cluster,
    mut argp: *const argp,
) -> *mut hol_cluster {
    let mut cl: *mut hol_cluster = malloc(::core::mem::size_of::<hol_cluster>() as u64)
        as *mut hol_cluster;
    if !cl.is_null() {
        (*cl).group = group;
        (*cl).header = header;
        (*cl).index = index;
        (*cl).parent = parent;
        (*cl).argp = argp;
        (*cl).depth = if !parent.is_null() {
            (*parent).depth + 1 as i32
        } else {
            0 as i32
        };
        (*cl).next = (*hol).clusters;
        (*hol).clusters = cl;
    }
    return cl;
}
unsafe extern "C" fn hol_free(mut hol: *mut hol) {
    let mut cl: *mut hol_cluster = (*hol).clusters;
    while !cl.is_null() {
        let mut next: *mut hol_cluster = (*cl).next;
        rpl_free(cl as *mut libc::c_void);
        cl = next;
    }
    if (*hol).num_entries > 0 as i32 as u32 {
        rpl_free((*hol).entries as *mut libc::c_void);
        rpl_free((*hol).short_options as *mut libc::c_void);
    }
    rpl_free(hol as *mut libc::c_void);
}
unsafe extern "C" fn hol_entry_short_iterate(
    mut entry: *const hol_entry,
    mut func: Option<
        unsafe extern "C" fn(
            *const argp_option,
            *const argp_option,
            *const i8,
            *mut libc::c_void,
        ) -> i32,
    >,
    mut domain: *const i8,
    mut cookie: *mut libc::c_void,
) -> i32 {
    let mut nopts: u32 = 0;
    let mut val: i32 = 0 as i32;
    let mut opt: *const argp_option = 0 as *const argp_option;
    let mut real: *const argp_option = (*entry).opt;
    let mut so: *mut i8 = (*entry).short_options;
    opt = real;
    nopts = (*entry).num;
    while nopts > 0 as i32 as u32 && val == 0 {
        if _option_is_short(opt) != 0 && *so as i32 == (*opt).key {
            if (*opt).flags & 0x4 as i32 == 0 {
                real = opt;
            }
            if (*opt).flags & 0x2 as i32 == 0 {
                val = (Some(func.expect("non-null function pointer")))
                    .expect("non-null function pointer")(opt, real, domain, cookie);
            }
            so = so.offset(1);
            so;
        }
        opt = opt.offset(1);
        opt;
        nopts = nopts.wrapping_sub(1);
        nopts;
    }
    return val;
}
#[inline(always)]
unsafe extern "C" fn hol_entry_long_iterate(
    mut entry: *const hol_entry,
    mut func: Option<
        unsafe extern "C" fn(
            *const argp_option,
            *const argp_option,
            *const i8,
            *mut libc::c_void,
        ) -> i32,
    >,
    mut domain: *const i8,
    mut cookie: *mut libc::c_void,
) -> i32 {
    let mut nopts: u32 = 0;
    let mut val: i32 = 0 as i32;
    let mut opt: *const argp_option = 0 as *const argp_option;
    let mut real: *const argp_option = (*entry).opt;
    opt = real;
    nopts = (*entry).num;
    while nopts > 0 as i32 as u32 && val == 0 {
        if !((*opt).name).is_null() {
            if (*opt).flags & 0x4 as i32 == 0 {
                real = opt;
            }
            if (*opt).flags & 0x2 as i32 == 0 {
                val = (Some(func.expect("non-null function pointer")))
                    .expect("non-null function pointer")(opt, real, domain, cookie);
            }
        }
        opt = opt.offset(1);
        opt;
        nopts = nopts.wrapping_sub(1);
        nopts;
    }
    return val;
}
unsafe extern "C" fn until_short(
    mut opt: *const argp_option,
    mut real: *const argp_option,
    mut domain: *const i8,
    mut cookie: *mut libc::c_void,
) -> i32 {
    return if _option_is_short(opt) != 0 { (*opt).key } else { 0 as i32 };
}
unsafe extern "C" fn hol_entry_first_short(mut entry: *const hol_entry) -> i8 {
    return hol_entry_short_iterate(
        entry,
        Some(
            until_short
                as unsafe extern "C" fn(
                    *const argp_option,
                    *const argp_option,
                    *const i8,
                    *mut libc::c_void,
                ) -> i32,
        ),
        (*(*entry).argp).argp_domain,
        0 as *mut libc::c_void,
    ) as i8;
}
unsafe extern "C" fn hol_entry_first_long(mut entry: *const hol_entry) -> *const i8 {
    let mut opt: *const argp_option = 0 as *const argp_option;
    let mut num: u32 = 0;
    opt = (*entry).opt;
    num = (*entry).num;
    while num > 0 as i32 as u32 {
        if !((*opt).name).is_null() && (*opt).flags & 0x2 as i32 == 0 {
            return (*opt).name;
        }
        opt = opt.offset(1);
        opt;
        num = num.wrapping_sub(1);
        num;
    }
    return 0 as *const i8;
}
unsafe extern "C" fn hol_find_entry(
    mut hol: *mut hol,
    mut name: *const i8,
) -> *mut hol_entry {
    let mut entry: *mut hol_entry = (*hol).entries;
    let mut num_entries: u32 = (*hol).num_entries;
    loop {
        let fresh3 = num_entries;
        num_entries = num_entries.wrapping_sub(1);
        if !(fresh3 > 0 as i32 as u32) {
            break;
        }
        let mut opt: *const argp_option = (*entry).opt;
        let mut num_opts: u32 = (*entry).num;
        loop {
            let fresh4 = num_opts;
            num_opts = num_opts.wrapping_sub(1);
            if !(fresh4 > 0 as i32 as u32) {
                break;
            }
            if !((*opt).name).is_null() && (*opt).flags & 0x2 as i32 == 0
                && strcmp((*opt).name, name) == 0 as i32
            {
                return entry
            } else {
                opt = opt.offset(1);
                opt;
            }
        }
        entry = entry.offset(1);
        entry;
    }
    return 0 as *mut hol_entry;
}
unsafe extern "C" fn hol_set_group(
    mut hol: *mut hol,
    mut name: *const i8,
    mut group: i32,
) {
    let mut entry: *mut hol_entry = hol_find_entry(hol, name);
    if !entry.is_null() {
        (*entry).group = group;
    }
}
unsafe extern "C" fn group_cmp(mut group1: i32, mut group2: i32) -> i32 {
    if group1 < 0 as i32 && group2 < 0 as i32 || group1 >= 0 as i32 && group2 >= 0 as i32
    {
        return group1 - group2
    } else {
        return group2 - group1
    };
}
unsafe extern "C" fn hol_sibling_cluster_cmp(
    mut cl1: *const hol_cluster,
    mut cl2: *const hol_cluster,
) -> i32 {
    let mut cmp: i32 = group_cmp((*cl1).group, (*cl2).group);
    if cmp != 0 as i32 {
        return cmp;
    }
    return (*cl2).index - (*cl1).index;
}
unsafe extern "C" fn hol_cousin_cluster_cmp(
    mut cl1: *const hol_cluster,
    mut cl2: *const hol_cluster,
) -> i32 {
    if (*cl1).parent == (*cl2).parent {
        return hol_sibling_cluster_cmp(cl1, cl2)
    } else {
        let mut cmp: i32 = hol_cousin_cluster_cmp((*cl1).parent, (*cl2).parent);
        if cmp != 0 as i32 {
            return cmp;
        }
        cmp = group_cmp((*cl1).group, (*cl2).group);
        if cmp != 0 as i32 {
            return cmp;
        }
        return (*cl2).index - (*cl1).index;
    };
}
unsafe extern "C" fn hol_cluster_cmp(
    mut cl1: *const hol_cluster,
    mut cl2: *const hol_cluster,
) -> i32 {
    if (*cl1).depth > (*cl2).depth {
        loop {
            cl1 = (*cl1).parent;
            if !((*cl1).depth > (*cl2).depth) {
                break;
            }
        }
        let mut cmp: i32 = hol_cousin_cluster_cmp(cl1, cl2);
        if cmp != 0 as i32 {
            return cmp;
        }
        return 1 as i32;
    } else if (*cl1).depth < (*cl2).depth {
        loop {
            cl2 = (*cl2).parent;
            if !((*cl1).depth < (*cl2).depth) {
                break;
            }
        }
        let mut cmp_0: i32 = hol_cousin_cluster_cmp(cl1, cl2);
        if cmp_0 != 0 as i32 {
            return cmp_0;
        }
        return -(1 as i32);
    } else {
        return hol_cousin_cluster_cmp(cl1, cl2)
    };
}
unsafe extern "C" fn hol_cluster_base(mut cl: *mut hol_cluster) -> *mut hol_cluster {
    while !((*cl).parent).is_null() {
        cl = (*cl).parent;
    }
    return cl;
}
unsafe extern "C" fn canon_doc_option(mut name: *mut *const i8) -> i32 {
    let mut non_opt: i32 = 0;
    while *(*__ctype_b_loc()).offset(**name as u8 as i32 as isize) as i32
        & _ISspace as i32 as libc::c_ushort as i32 != 0
    {
        *name = (*name).offset(1);
        *name;
    }
    non_opt = (**name as i32 != '-' as i32) as i32;
    while **name as i32 != 0
        && *(*__ctype_b_loc()).offset(**name as u8 as i32 as isize) as i32
            & _ISalnum as i32 as libc::c_ushort as i32 == 0
    {
        *name = (*name).offset(1);
        *name;
    }
    return non_opt;
}
unsafe extern "C" fn hol_entry_cmp(
    mut entry1: *const hol_entry,
    mut entry2: *const hol_entry,
) -> i32 {
    let mut group1: i32 = if !((*entry1).cluster).is_null() {
        (*hol_cluster_base((*entry1).cluster)).group
    } else {
        (*entry1).group
    };
    let mut group2: i32 = if !((*entry2).cluster).is_null() {
        (*hol_cluster_base((*entry2).cluster)).group
    } else {
        (*entry2).group
    };
    let mut cmp: i32 = group_cmp(group1, group2);
    if cmp != 0 as i32 {
        return cmp;
    }
    cmp = ((*entry1).cluster != 0 as *mut libc::c_void as *mut hol_cluster) as i32
        - ((*entry2).cluster != 0 as *mut libc::c_void as *mut hol_cluster) as i32;
    if cmp != 0 as i32 {
        return cmp;
    }
    if !((*entry1).cluster).is_null() {
        cmp = hol_cluster_cmp((*entry1).cluster, (*entry2).cluster);
        if cmp != 0 as i32 {
            return cmp;
        }
    }
    cmp = group_cmp((*entry1).group, (*entry2).group);
    if cmp != 0 as i32 {
        return cmp;
    }
    let mut long1: *const i8 = hol_entry_first_long(entry1);
    let mut long2: *const i8 = hol_entry_first_long(entry2);
    let mut doc1: i32 = if (*(*entry1).opt).flags & 0x8 as i32 != 0 {
        (!long1.is_null() && canon_doc_option(&mut long1) != 0) as i32
    } else {
        0 as i32
    };
    let mut doc2: i32 = if (*(*entry2).opt).flags & 0x8 as i32 != 0 {
        (!long2.is_null() && canon_doc_option(&mut long2) != 0) as i32
    } else {
        0 as i32
    };
    cmp = doc1 - doc2;
    if cmp != 0 as i32 {
        return cmp;
    }
    let mut short1: i32 = hol_entry_first_short(entry1) as i32;
    let mut short2: i32 = hol_entry_first_short(entry2) as i32;
    let mut first1: u8 = (if short1 != 0 {
        short1
    } else if !long1.is_null() {
        *long1 as i32
    } else {
        0 as i32
    }) as u8;
    let mut first2: u8 = (if short2 != 0 {
        short2
    } else if !long2.is_null() {
        *long2 as i32
    } else {
        0 as i32
    }) as u8;
    cmp = ({
        let mut __res: i32 = 0;
        if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
            if 0 != 0 {
                let mut __c: i32 = first1 as i32;
                __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                    __c
                } else {
                    *(*__ctype_tolower_loc()).offset(__c as isize)
                });
            } else {
                __res = tolower(first1 as i32);
            }
        } else {
            __res = *(*__ctype_tolower_loc()).offset(first1 as i32 as isize);
        }
        __res
    })
        - ({
            let mut __res: i32 = 0;
            if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                if 0 != 0 {
                    let mut __c: i32 = first2 as i32;
                    __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    });
                } else {
                    __res = tolower(first2 as i32);
                }
            } else {
                __res = *(*__ctype_tolower_loc()).offset(first2 as i32 as isize);
            }
            __res
        });
    if cmp != 0 as i32 {
        return cmp;
    }
    cmp = first2 as i32 - first1 as i32;
    if cmp != 0 as i32 {
        return cmp;
    }
    cmp = (short1 != 0 as i32) as i32 - (short2 != 0 as i32) as i32;
    if cmp != 0 as i32 {
        return cmp;
    }
    if short1 == 0 as i32 {
        cmp = (long1 != 0 as *mut libc::c_void as *const i8) as i32
            - (long2 != 0 as *mut libc::c_void as *const i8) as i32;
        if cmp != 0 as i32 {
            return cmp;
        }
        if !long1.is_null() {
            cmp = strcasecmp(long1, long2);
            if cmp != 0 as i32 {
                return cmp;
            }
        }
    }
    return 0 as i32;
}
unsafe extern "C" fn hol_entry_qcmp(
    mut entry1_v: *const libc::c_void,
    mut entry2_v: *const libc::c_void,
) -> i32 {
    return hol_entry_cmp(entry1_v as *const hol_entry, entry2_v as *const hol_entry);
}
unsafe extern "C" fn hol_sort(mut hol: *mut hol) {
    if (*hol).num_entries > 0 as i32 as u32 {
        let mut i: u32 = 0;
        let mut e: *mut hol_entry = 0 as *mut hol_entry;
        i = 0 as i32 as u32;
        e = (*hol).entries;
        while i < (*hol).num_entries {
            (*e).ord = i;
            i = i.wrapping_add(1);
            i;
            e = e.offset(1);
            e;
        }
        qsort(
            (*hol).entries as *mut libc::c_void,
            (*hol).num_entries as size_t,
            ::core::mem::size_of::<hol_entry>() as u64,
            Some(
                hol_entry_qcmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> i32,
            ),
        );
    }
}
unsafe extern "C" fn hol_append(mut hol: *mut hol, mut more: *mut hol) {
    let mut cl_end: *mut *mut hol_cluster = &mut (*hol).clusters;
    while !(*cl_end).is_null() {
        cl_end = &mut (**cl_end).next;
    }
    *cl_end = (*more).clusters;
    (*more).clusters = 0 as *mut hol_cluster;
    if (*more).num_entries > 0 as i32 as u32 {
        if (*hol).num_entries == 0 as i32 as u32 {
            (*hol).num_entries = (*more).num_entries;
            (*hol).entries = (*more).entries;
            (*hol).short_options = (*more).short_options;
            (*more).num_entries = 0 as i32 as u32;
        } else {
            let mut left: u32 = 0;
            let mut so: *mut i8 = 0 as *mut i8;
            let mut more_so: *mut i8 = 0 as *mut i8;
            let mut e: *mut hol_entry = 0 as *mut hol_entry;
            let mut num_entries: u32 = ((*hol).num_entries)
                .wrapping_add((*more).num_entries);
            let mut entries: *mut hol_entry = malloc(
                (num_entries as u64)
                    .wrapping_mul(::core::mem::size_of::<hol_entry>() as u64),
            ) as *mut hol_entry;
            let mut hol_so_len: u32 = strlen((*hol).short_options) as u32;
            let mut short_options: *mut i8 = malloc(
                (hol_so_len as u64)
                    .wrapping_add(strlen((*more).short_options))
                    .wrapping_add(1 as i32 as u64),
            ) as *mut i8;
            if !entries.is_null() && !short_options.is_null() {} else {
                __assert_fail(
                    b"entries && short_options\0" as *const u8 as *const i8,
                    b"argp-help.c\0" as *const u8 as *const i8,
                    969 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 44],
                        &[i8; 44],
                    >(b"void hol_append(struct hol *, struct hol *)\0"))
                        .as_ptr(),
                );
            }
            'c_9544: {
                if !entries.is_null() && !short_options.is_null() {} else {
                    __assert_fail(
                        b"entries && short_options\0" as *const u8 as *const i8,
                        b"argp-help.c\0" as *const u8 as *const i8,
                        969 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 44],
                            &[i8; 44],
                        >(b"void hol_append(struct hol *, struct hol *)\0"))
                            .as_ptr(),
                    );
                }
            };
            if -(1 as i32) as size_t
                <= (2147483647 as i32 as u32)
                    .wrapping_mul(2 as u32)
                    .wrapping_add(1 as u32) as u64
            {
                if num_entries as u64
                    <= (-(1 as i32) as size_t)
                        .wrapping_div(::core::mem::size_of::<hol_entry>() as u64)
                {} else {
                    __assert_fail(
                        b"num_entries <= SIZE_MAX / sizeof (struct hol_entry)\0"
                            as *const u8 as *const i8,
                        b"argp-help.c\0" as *const u8 as *const i8,
                        971 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 44],
                            &[i8; 44],
                        >(b"void hol_append(struct hol *, struct hol *)\0"))
                            .as_ptr(),
                    );
                }
                'c_9482: {
                    if num_entries as u64
                        <= (-(1 as i32) as size_t)
                            .wrapping_div(::core::mem::size_of::<hol_entry>() as u64)
                    {} else {
                        __assert_fail(
                            b"num_entries <= SIZE_MAX / sizeof (struct hol_entry)\0"
                                as *const u8 as *const i8,
                            b"argp-help.c\0" as *const u8 as *const i8,
                            971 as i32 as u32,
                            (*::core::mem::transmute::<
                                &[u8; 44],
                                &[i8; 44],
                            >(b"void hol_append(struct hol *, struct hol *)\0"))
                                .as_ptr(),
                        );
                    }
                };
            }
            mempcpy(
                mempcpy(
                    entries as *mut libc::c_void,
                    (*hol).entries as *const libc::c_void,
                    ((*hol).num_entries as u64)
                        .wrapping_mul(::core::mem::size_of::<hol_entry>() as u64),
                ),
                (*more).entries as *const libc::c_void,
                ((*more).num_entries as u64)
                    .wrapping_mul(::core::mem::size_of::<hol_entry>() as u64),
            );
            mempcpy(
                short_options as *mut libc::c_void,
                (*hol).short_options as *const libc::c_void,
                hol_so_len as size_t,
            );
            e = entries;
            left = (*hol).num_entries;
            while left > 0 as i32 as u32 {
                (*e).short_options = short_options
                    .offset(
                        ((*e).short_options).offset_from((*hol).short_options) as i64
                            as isize,
                    );
                e = e.offset(1);
                e;
                left = left.wrapping_sub(1);
                left;
            }
            so = short_options.offset(hol_so_len as isize);
            more_so = (*more).short_options;
            left = (*more).num_entries;
            while left > 0 as i32 as u32 {
                let mut opts_left: i32 = 0;
                let mut opt: *const argp_option = 0 as *const argp_option;
                (*e).short_options = so;
                opts_left = (*e).num as i32;
                opt = (*e).opt;
                while opts_left != 0 {
                    let mut ch: i32 = *more_so as i32;
                    if _option_is_short(opt) != 0 && ch == (*opt).key {
                        if find_char(
                            ch as i8,
                            short_options,
                            short_options.offset(hol_so_len as isize),
                        ) == 0
                        {
                            let fresh5 = so;
                            so = so.offset(1);
                            *fresh5 = ch as i8;
                        }
                        more_so = more_so.offset(1);
                        more_so;
                    }
                    opt = opt.offset(1);
                    opt;
                    opts_left -= 1;
                    opts_left;
                }
                e = e.offset(1);
                e;
                left = left.wrapping_sub(1);
                left;
            }
            *so = '\0' as i32 as i8;
            rpl_free((*hol).entries as *mut libc::c_void);
            rpl_free((*hol).short_options as *mut libc::c_void);
            (*hol).entries = entries;
            (*hol).num_entries = num_entries;
            (*hol).short_options = short_options;
        }
    }
    hol_free(more);
}
unsafe extern "C" fn argp_hol(
    mut argp: *const argp,
    mut cluster: *mut hol_cluster,
) -> *mut hol {
    let mut child: *const argp_child = (*argp).children;
    let mut hol: *mut hol = make_hol(argp, cluster);
    if !child.is_null() {
        while !((*child).argp).is_null() {
            let mut child_cluster: *mut hol_cluster = if (*child).group != 0
                || !((*child).header).is_null()
            {
                hol_add_cluster(
                    hol,
                    (*child).group,
                    (*child).header,
                    child.offset_from((*argp).children) as i64 as i32,
                    cluster,
                    argp,
                )
            } else {
                cluster
            };
            hol_append(hol, argp_hol((*child).argp, child_cluster));
            child = child.offset(1);
            child;
        }
    }
    return hol;
}
unsafe extern "C" fn indent_to(mut stream: argp_fmtstream_t, mut col: u32) {
    let mut needed: i32 = (col as u64).wrapping_sub(argp_fmtstream_point(stream)) as i32;
    loop {
        let fresh6 = needed;
        needed = needed - 1;
        if !(fresh6 > 0 as i32) {
            break;
        }
        argp_fmtstream_putc(stream, ' ' as i32);
    };
}
unsafe extern "C" fn space(mut stream: argp_fmtstream_t, mut ensure: size_t) {
    if (argp_fmtstream_point(stream)).wrapping_add(ensure) >= (*stream).rmargin {
        argp_fmtstream_putc(stream, '\n' as i32);
    } else {
        argp_fmtstream_putc(stream, ' ' as i32);
    };
}
unsafe extern "C" fn arg(
    mut real: *const argp_option,
    mut req_fmt: *const i8,
    mut opt_fmt: *const i8,
    mut domain: *const i8,
    mut stream: argp_fmtstream_t,
) {
    if !((*real).arg).is_null() {
        if (*real).flags & 0x1 as i32 != 0 {
            argp_fmtstream_printf(
                stream,
                opt_fmt,
                dcgettext(domain, (*real).arg, 5 as i32),
            );
        } else {
            argp_fmtstream_printf(
                stream,
                req_fmt,
                dcgettext(domain, (*real).arg, 5 as i32),
            );
        }
    }
}
unsafe extern "C" fn filter_doc(
    mut doc: *const i8,
    mut key: i32,
    mut argp: *const argp,
    mut state: *const argp_state,
) -> *const i8 {
    if !argp.is_null() && ((*argp).help_filter).is_some() {
        let mut input: *mut libc::c_void = _argp_input(argp, state);
        return (Some(((*argp).help_filter).expect("non-null function pointer")))
            .expect("non-null function pointer")(key, doc, input);
    } else {
        return doc
    };
}
unsafe extern "C" fn print_header(
    mut str: *const i8,
    mut argp: *const argp,
    mut pest: *mut pentry_state,
) {
    let mut tstr: *const i8 = if !str.is_null() {
        dcgettext((*argp).argp_domain, str, 5 as i32)
    } else {
        0 as *mut i8
    };
    let mut fstr: *const i8 = filter_doc(tstr, 0x2000003 as i32, argp, (*pest).state);
    if !fstr.is_null() {
        if *fstr != 0 {
            if !((*(*pest).hhstate).prev_entry).is_null() {
                argp_fmtstream_putc((*pest).stream, '\n' as i32);
            }
            indent_to((*pest).stream, uparams.header_col as u32);
            argp_fmtstream_set_lmargin((*pest).stream, uparams.header_col as size_t);
            argp_fmtstream_set_wmargin((*pest).stream, uparams.header_col as size_t);
            argp_fmtstream_puts((*pest).stream, fstr);
            argp_fmtstream_set_lmargin((*pest).stream, 0 as i32 as size_t);
            argp_fmtstream_putc((*pest).stream, '\n' as i32);
        }
        (*(*pest).hhstate).sep_groups = 1 as i32;
    }
    if fstr != tstr {
        rpl_free(fstr as *mut i8 as *mut libc::c_void);
    }
}
unsafe extern "C" fn hol_cluster_is_child(
    mut cl1: *const hol_cluster,
    mut cl2: *const hol_cluster,
) -> i32 {
    while !cl1.is_null() && cl1 != cl2 {
        cl1 = (*cl1).parent;
    }
    return (cl1 == cl2) as i32;
}
unsafe extern "C" fn comma(mut col: u32, mut pest: *mut pentry_state) {
    if (*pest).first != 0 {
        let mut pe: *const hol_entry = (*(*pest).hhstate).prev_entry;
        let mut cl: *const hol_cluster = (*(*pest).entry).cluster;
        if (*(*pest).hhstate).sep_groups != 0 && !pe.is_null()
            && (*(*pest).entry).group != (*pe).group
        {
            argp_fmtstream_putc((*pest).stream, '\n' as i32);
        }
        if !cl.is_null() && !((*cl).header).is_null() && *(*cl).header as i32 != 0
            && (pe.is_null()
                || (*pe).cluster != cl as *mut hol_cluster
                    && hol_cluster_is_child((*pe).cluster, cl) == 0)
        {
            let mut old_wm: i32 = (*(*pest).stream).wmargin as i32;
            print_header((*cl).header, (*cl).argp, pest);
            argp_fmtstream_set_wmargin((*pest).stream, old_wm as size_t);
        }
        (*pest).first = 0 as i32;
    } else {
        argp_fmtstream_puts((*pest).stream, b", \0" as *const u8 as *const i8);
    }
    indent_to((*pest).stream, col);
}
unsafe extern "C" fn hol_entry_help(
    mut entry: *mut hol_entry,
    mut state: *const argp_state,
    mut stream: argp_fmtstream_t,
    mut hhstate: *mut hol_help_state,
) {
    let mut current_block: u64;
    let mut num: u32 = 0;
    let mut real: *const argp_option = (*entry).opt;
    let mut opt: *const argp_option = 0 as *const argp_option;
    let mut so: *mut i8 = (*entry).short_options;
    let mut have_long_opt: i32 = 0 as i32;
    let mut old_lm: i32 = argp_fmtstream_set_lmargin(stream, 0 as i32 as size_t) as i32;
    let mut old_wm: i32 = (*stream).wmargin as i32;
    let mut pest: pentry_state = {
        let mut init = pentry_state {
            entry: entry,
            stream: stream,
            hhstate: hhstate,
            first: 1 as i32,
            state: state,
        };
        init
    };
    if (*real).flags & 0x8 as i32 == 0 {
        opt = real;
        num = (*entry).num;
        while num > 0 as i32 as u32 {
            if !((*opt).name).is_null() && (*opt).flags & 0x2 as i32 == 0 {
                have_long_opt = 1 as i32;
                break;
            } else {
                opt = opt.offset(1);
                opt;
                num = num.wrapping_sub(1);
                num;
            }
        }
    }
    argp_fmtstream_set_wmargin(stream, uparams.short_opt_col as size_t);
    opt = real;
    num = (*entry).num;
    while num > 0 as i32 as u32 {
        if _option_is_short(opt) != 0 && (*opt).key == *so as i32 {
            if (*opt).flags & 0x2 as i32 == 0 {
                comma(uparams.short_opt_col as u32, &mut pest);
                argp_fmtstream_putc(stream, '-' as i32);
                argp_fmtstream_putc(stream, *so as i32);
                if have_long_opt == 0 || uparams.dup_args != 0 {
                    arg(
                        real,
                        b" %s\0" as *const u8 as *const i8,
                        b"[%s]\0" as *const u8 as *const i8,
                        if state.is_null() {
                            0 as *const i8
                        } else {
                            (*(*state).root_argp).argp_domain
                        },
                        stream,
                    );
                } else if !((*real).arg).is_null() {
                    (*hhstate).suppressed_dup_arg = 1 as i32;
                }
            }
            so = so.offset(1);
            so;
        }
        opt = opt.offset(1);
        opt;
        num = num.wrapping_sub(1);
        num;
    }
    if (*real).flags & 0x8 as i32 != 0 {
        argp_fmtstream_set_wmargin(stream, uparams.doc_opt_col as size_t);
        opt = real;
        num = (*entry).num;
        while num > 0 as i32 as u32 {
            if !((*opt).name).is_null() && (*opt).flags & 0x2 as i32 == 0 {
                comma(uparams.doc_opt_col as u32, &mut pest);
                argp_fmtstream_puts(
                    stream,
                    dcgettext(
                        if state.is_null() {
                            0 as *const i8
                        } else {
                            (*(*state).root_argp).argp_domain
                        },
                        (*opt).name,
                        5 as i32,
                    ),
                );
            }
            opt = opt.offset(1);
            opt;
            num = num.wrapping_sub(1);
            num;
        }
    } else {
        argp_fmtstream_set_wmargin(stream, uparams.long_opt_col as size_t);
        opt = real;
        num = (*entry).num;
        while num > 0 as i32 as u32 {
            if !((*opt).name).is_null() && (*opt).flags & 0x2 as i32 == 0 {
                comma(uparams.long_opt_col as u32, &mut pest);
                argp_fmtstream_printf(
                    stream,
                    b"--%s\0" as *const u8 as *const i8,
                    (*opt).name,
                );
                arg(
                    real,
                    b"=%s\0" as *const u8 as *const i8,
                    b"[=%s]\0" as *const u8 as *const i8,
                    if state.is_null() {
                        0 as *const i8
                    } else {
                        (*(*state).root_argp).argp_domain
                    },
                    stream,
                );
            }
            opt = opt.offset(1);
            opt;
            num = num.wrapping_sub(1);
            num;
        }
    }
    argp_fmtstream_set_lmargin(stream, 0 as i32 as size_t);
    if pest.first != 0 {
        if _option_is_short(real) == 0 && ((*real).name).is_null() {
            print_header((*real).doc, (*entry).argp, &mut pest);
            current_block = 10095721787123848864;
        } else {
            current_block = 15439365785507037250;
        }
    } else {
        let mut tstr: *const i8 = if !((*real).doc).is_null() {
            dcgettext(
                if state.is_null() {
                    0 as *const i8
                } else {
                    (*(*state).root_argp).argp_domain
                },
                (*real).doc,
                5 as i32,
            )
        } else {
            0 as *mut i8
        };
        let mut fstr: *const i8 = filter_doc(tstr, (*real).key, (*entry).argp, state);
        if !fstr.is_null() && *fstr as i32 != 0 {
            let mut col: u32 = argp_fmtstream_point(stream) as u32;
            argp_fmtstream_set_lmargin(stream, uparams.opt_doc_col as size_t);
            argp_fmtstream_set_wmargin(stream, uparams.opt_doc_col as size_t);
            if col > (uparams.opt_doc_col + 3 as i32) as u32 {
                argp_fmtstream_putc(stream, '\n' as i32);
            } else if col >= uparams.opt_doc_col as u32 {
                argp_fmtstream_puts(stream, b"   \0" as *const u8 as *const i8);
            } else {
                indent_to(stream, uparams.opt_doc_col as u32);
            }
            argp_fmtstream_puts(stream, fstr);
        }
        if !fstr.is_null() && fstr != tstr {
            rpl_free(fstr as *mut i8 as *mut libc::c_void);
        }
        argp_fmtstream_set_lmargin(stream, 0 as i32 as size_t);
        argp_fmtstream_putc(stream, '\n' as i32);
        current_block = 10095721787123848864;
    }
    match current_block {
        10095721787123848864 => {
            (*hhstate).prev_entry = entry;
        }
        _ => {}
    }
    argp_fmtstream_set_lmargin(stream, old_lm as size_t);
    argp_fmtstream_set_wmargin(stream, old_wm as size_t);
}
unsafe extern "C" fn hol_help(
    mut hol: *mut hol,
    mut state: *const argp_state,
    mut stream: argp_fmtstream_t,
) {
    let mut num: u32 = 0;
    let mut entry: *mut hol_entry = 0 as *mut hol_entry;
    let mut hhstate: hol_help_state = {
        let mut init = hol_help_state {
            prev_entry: 0 as *mut hol_entry,
            sep_groups: 0 as i32,
            suppressed_dup_arg: 0 as i32,
        };
        init
    };
    entry = (*hol).entries;
    num = (*hol).num_entries;
    while num > 0 as i32 as u32 {
        hol_entry_help(entry, state, stream, &mut hhstate);
        entry = entry.offset(1);
        entry;
        num = num.wrapping_sub(1);
        num;
    }
    if hhstate.suppressed_dup_arg != 0 && uparams.dup_args_note != 0 {
        let mut tstr: *const i8 = dcgettext(
            if state.is_null() {
                0 as *const i8
            } else {
                (*(*state).root_argp).argp_domain
            },
            b"Mandatory or optional arguments to long options are also mandatory or optional for any corresponding short options.\0"
                as *const u8 as *const i8,
            5 as i32,
        );
        let mut fstr: *const i8 = filter_doc(
            tstr,
            0x2000005 as i32,
            if !state.is_null() { (*state).root_argp } else { 0 as *const argp },
            state,
        );
        if !fstr.is_null() && *fstr as i32 != 0 {
            argp_fmtstream_putc(stream, '\n' as i32);
            argp_fmtstream_puts(stream, fstr);
            argp_fmtstream_putc(stream, '\n' as i32);
        }
        if !fstr.is_null() && fstr != tstr {
            rpl_free(fstr as *mut i8 as *mut libc::c_void);
        }
    }
}
unsafe extern "C" fn add_argless_short_opt(
    mut opt: *const argp_option,
    mut real: *const argp_option,
    mut domain: *const i8,
    mut cookie: *mut libc::c_void,
) -> i32 {
    let mut snao_end: *mut *mut i8 = cookie as *mut *mut i8;
    if !(!((*opt).arg).is_null() || !((*real).arg).is_null())
        && ((*opt).flags | (*real).flags) & 0x10 as i32 == 0
    {
        let fresh7 = *snao_end;
        *snao_end = (*snao_end).offset(1);
        *fresh7 = (*opt).key as i8;
    }
    return 0 as i32;
}
unsafe extern "C" fn usage_argful_short_opt(
    mut opt: *const argp_option,
    mut real: *const argp_option,
    mut domain: *const i8,
    mut cookie: *mut libc::c_void,
) -> i32 {
    let mut stream: argp_fmtstream_t = cookie as argp_fmtstream_t;
    let mut arg_0: *const i8 = (*opt).arg;
    let mut flags: i32 = (*opt).flags | (*real).flags;
    if arg_0.is_null() {
        arg_0 = (*real).arg;
    }
    if !arg_0.is_null() && flags & 0x10 as i32 == 0 {
        arg_0 = dcgettext(domain, arg_0, 5 as i32);
        if flags & 0x1 as i32 != 0 {
            argp_fmtstream_printf(
                stream,
                b" [-%c[%s]]\0" as *const u8 as *const i8,
                (*opt).key,
                arg_0,
            );
        } else {
            space(stream, (6 as i32 as u64).wrapping_add(strlen(arg_0)));
            argp_fmtstream_printf(
                stream,
                b"[-%c %s]\0" as *const u8 as *const i8,
                (*opt).key,
                arg_0,
            );
        }
    }
    return 0 as i32;
}
unsafe extern "C" fn usage_long_opt(
    mut opt: *const argp_option,
    mut real: *const argp_option,
    mut domain: *const i8,
    mut cookie: *mut libc::c_void,
) -> i32 {
    let mut stream: argp_fmtstream_t = cookie as argp_fmtstream_t;
    let mut arg_0: *const i8 = (*opt).arg;
    let mut flags: i32 = (*opt).flags | (*real).flags;
    if arg_0.is_null() {
        arg_0 = (*real).arg;
    }
    if flags & 0x10 as i32 == 0 {
        if !arg_0.is_null() {
            arg_0 = dcgettext(domain, arg_0, 5 as i32);
            if flags & 0x1 as i32 != 0 {
                argp_fmtstream_printf(
                    stream,
                    b" [--%s[=%s]]\0" as *const u8 as *const i8,
                    (*opt).name,
                    arg_0,
                );
            } else {
                argp_fmtstream_printf(
                    stream,
                    b" [--%s=%s]\0" as *const u8 as *const i8,
                    (*opt).name,
                    arg_0,
                );
            }
        } else {
            argp_fmtstream_printf(
                stream,
                b" [--%s]\0" as *const u8 as *const i8,
                (*opt).name,
            );
        }
    }
    return 0 as i32;
}
unsafe extern "C" fn hol_usage(mut hol: *mut hol, mut stream: argp_fmtstream_t) {
    if (*hol).num_entries > 0 as i32 as u32 {
        let mut nentries: u32 = 0;
        let mut entry: *mut hol_entry = 0 as *mut hol_entry;
        let mut fresh8 = ::std::vec::from_elem(
            0,
            (strlen((*hol).short_options)).wrapping_add(1 as i32 as u64) as usize,
        );
        let mut short_no_arg_opts: *mut i8 = fresh8.as_mut_ptr() as *mut i8;
        let mut snao_end: *mut i8 = short_no_arg_opts;
        entry = (*hol).entries;
        nentries = (*hol).num_entries;
        while nentries > 0 as i32 as u32 {
            hol_entry_short_iterate(
                entry,
                Some(
                    add_argless_short_opt
                        as unsafe extern "C" fn(
                            *const argp_option,
                            *const argp_option,
                            *const i8,
                            *mut libc::c_void,
                        ) -> i32,
                ),
                (*(*entry).argp).argp_domain,
                &mut snao_end as *mut *mut i8 as *mut libc::c_void,
            );
            entry = entry.offset(1);
            entry;
            nentries = nentries.wrapping_sub(1);
            nentries;
        }
        if snao_end > short_no_arg_opts {
            let fresh9 = snao_end;
            snao_end = snao_end.offset(1);
            *fresh9 = 0 as i32 as i8;
            argp_fmtstream_printf(
                stream,
                b" [-%s]\0" as *const u8 as *const i8,
                short_no_arg_opts,
            );
        }
        entry = (*hol).entries;
        nentries = (*hol).num_entries;
        while nentries > 0 as i32 as u32 {
            hol_entry_short_iterate(
                entry,
                Some(
                    usage_argful_short_opt
                        as unsafe extern "C" fn(
                            *const argp_option,
                            *const argp_option,
                            *const i8,
                            *mut libc::c_void,
                        ) -> i32,
                ),
                (*(*entry).argp).argp_domain,
                stream as *mut libc::c_void,
            );
            entry = entry.offset(1);
            entry;
            nentries = nentries.wrapping_sub(1);
            nentries;
        }
        entry = (*hol).entries;
        nentries = (*hol).num_entries;
        while nentries > 0 as i32 as u32 {
            hol_entry_long_iterate(
                entry,
                Some(
                    usage_long_opt
                        as unsafe extern "C" fn(
                            *const argp_option,
                            *const argp_option,
                            *const i8,
                            *mut libc::c_void,
                        ) -> i32,
                ),
                (*(*entry).argp).argp_domain,
                stream as *mut libc::c_void,
            );
            entry = entry.offset(1);
            entry;
            nentries = nentries.wrapping_sub(1);
            nentries;
        }
    }
}
unsafe extern "C" fn argp_args_levels(mut argp: *const argp) -> size_t {
    let mut levels: size_t = 0 as i32 as size_t;
    let mut child: *const argp_child = (*argp).children;
    if !((*argp).args_doc).is_null()
        && !(strchr((*argp).args_doc, '\n' as i32)).is_null()
    {
        levels = levels.wrapping_add(1);
        levels;
    }
    if !child.is_null() {
        while !((*child).argp).is_null() {
            let fresh10 = child;
            child = child.offset(1);
            levels = (levels as u64).wrapping_add(argp_args_levels((*fresh10).argp))
                as size_t as size_t;
        }
    }
    return levels;
}
unsafe extern "C" fn argp_args_usage(
    mut argp: *const argp,
    mut state: *const argp_state,
    mut levels: *mut *mut i8,
    mut advance: i32,
    mut stream: argp_fmtstream_t,
) -> i32 {
    let mut our_level: *mut i8 = *levels;
    let mut multiple: i32 = 0 as i32;
    let mut child: *const argp_child = (*argp).children;
    let mut tdoc: *const i8 = if !((*argp).args_doc).is_null() {
        dcgettext((*argp).argp_domain, (*argp).args_doc, 5 as i32)
    } else {
        0 as *mut i8
    };
    let mut fdoc: *const i8 = filter_doc(tdoc, 0x2000006 as i32, argp, state);
    let mut nl: *const i8 = 0 as *const i8;
    if !fdoc.is_null() {
        let mut cp: *const i8 = fdoc;
        nl = strchrnul(cp, '\n' as i32);
        if *nl as i32 != '\0' as i32 {
            let mut i: i32 = 0;
            multiple = 1 as i32;
            i = 0 as i32;
            while i < *our_level as i32 {
                cp = nl.offset(1 as i32 as isize);
                nl = strchrnul(cp, '\n' as i32);
                i += 1;
                i;
            }
            *levels = (*levels).offset(1);
            *levels;
        }
        space(stream, nl.offset(1 as i32 as isize).offset_from(cp) as i64 as size_t);
        argp_fmtstream_write(stream, cp, nl.offset_from(cp) as i64 as size_t);
    }
    if !fdoc.is_null() && fdoc != tdoc {
        rpl_free(fdoc as *mut i8 as *mut libc::c_void);
    }
    if !child.is_null() {
        while !((*child).argp).is_null() {
            let fresh11 = child;
            child = child.offset(1);
            advance = (argp_args_usage((*fresh11).argp, state, levels, advance, stream)
                == 0) as i32;
        }
    }
    if advance != 0 && multiple != 0 {
        if *nl != 0 {
            *our_level += 1;
            *our_level;
            advance = 0 as i32;
        } else if *our_level as i32 > 0 as i32 {
            *our_level = 0 as i32 as i8;
        }
    }
    return (advance == 0) as i32;
}
unsafe extern "C" fn argp_doc(
    mut argp: *const argp,
    mut state: *const argp_state,
    mut post: i32,
    mut pre_blank: i32,
    mut first_only: i32,
    mut stream: argp_fmtstream_t,
) -> i32 {
    let mut text: *const i8 = 0 as *const i8;
    let mut inp_text: *const i8 = 0 as *const i8;
    let mut input: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut anything: i32 = 0 as i32;
    let mut inp_text_limit: size_t = 0 as i32 as size_t;
    let mut doc: *const i8 = if !((*argp).doc).is_null() {
        dcgettext((*argp).argp_domain, (*argp).doc, 5 as i32)
    } else {
        0 as *mut i8
    };
    let mut child: *const argp_child = (*argp).children;
    if !doc.is_null() {
        let mut vt: *mut i8 = strchr(doc, '\u{b}' as i32);
        inp_text = if post != 0 {
            if !vt.is_null() { vt.offset(1 as i32 as isize) } else { 0 as *mut i8 }
        } else {
            doc
        };
        inp_text_limit = (if post == 0 && !vt.is_null() {
            vt.offset_from(doc) as i64
        } else {
            0 as i32 as i64
        }) as size_t;
    } else {
        inp_text = 0 as *const i8;
    }
    if ((*argp).help_filter).is_some() {
        if inp_text_limit != 0 {
            inp_text = strndup(inp_text, inp_text_limit);
        }
        input = _argp_input(argp, state);
        text = (Some(((*argp).help_filter).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            if post != 0 { 0x2000002 as i32 } else { 0x2000001 as i32 },
            inp_text,
            input,
        );
    } else {
        text = inp_text;
    }
    if !text.is_null() {
        if pre_blank != 0 {
            argp_fmtstream_putc(stream, '\n' as i32);
        }
        if text == inp_text && inp_text_limit != 0 {
            argp_fmtstream_write(stream, inp_text, inp_text_limit);
        } else {
            argp_fmtstream_puts(stream, text);
        }
        if argp_fmtstream_point(stream) > (*stream).lmargin {
            argp_fmtstream_putc(stream, '\n' as i32);
        }
        anything = 1 as i32;
    }
    if !text.is_null() && text != inp_text {
        rpl_free(text as *mut i8 as *mut libc::c_void);
    }
    if !inp_text.is_null() && inp_text_limit != 0 && ((*argp).help_filter).is_some() {
        rpl_free(inp_text as *mut i8 as *mut libc::c_void);
    }
    if post != 0 && ((*argp).help_filter).is_some() {
        text = (Some(((*argp).help_filter).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(0x2000004 as i32, 0 as *const i8, input);
        if !text.is_null() {
            if anything != 0 || pre_blank != 0 {
                argp_fmtstream_putc(stream, '\n' as i32);
            }
            argp_fmtstream_puts(stream, text);
            rpl_free(text as *mut i8 as *mut libc::c_void);
            if argp_fmtstream_point(stream) > (*stream).lmargin {
                argp_fmtstream_putc(stream, '\n' as i32);
            }
            anything = 1 as i32;
        }
    }
    if !child.is_null() {
        while !((*child).argp).is_null() && !(first_only != 0 && anything != 0) {
            let fresh12 = child;
            child = child.offset(1);
            anything
                |= argp_doc(
                    (*fresh12).argp,
                    state,
                    post,
                    (anything != 0 || pre_blank != 0) as i32,
                    first_only,
                    stream,
                );
        }
    }
    return anything;
}
unsafe extern "C" fn _help(
    mut argp: *const argp,
    mut state: *const argp_state,
    mut stream: *mut FILE,
    mut flags: u32,
    mut name: *mut i8,
) {
    let mut anything: i32 = 0 as i32;
    let mut hol: *mut hol = 0 as *mut hol;
    let mut fs: argp_fmtstream_t = 0 as *mut argp_fmtstream;
    if stream.is_null() {
        return;
    }
    flockfile(stream);
    if uparams.valid == 0 {
        fill_in_uparams(state);
    }
    fs = argp_make_fmtstream(
        stream,
        0 as i32 as size_t,
        uparams.rmargin as size_t,
        0 as i32 as ssize_t,
    );
    if fs.is_null() {
        funlockfile(stream);
        return;
    }
    if flags & (0x1 as i32 | 0x2 as i32 | 0x8 as i32) as u32 != 0 {
        hol = argp_hol(argp, 0 as *mut hol_cluster);
        hol_set_group(hol, b"help\0" as *const u8 as *const i8, -(1 as i32));
        hol_set_group(hol, b"version\0" as *const u8 as *const i8, -(1 as i32));
        hol_sort(hol);
    }
    if flags & (0x1 as i32 | 0x2 as i32) as u32 != 0 {
        let mut first_pattern: i32 = 1 as i32;
        let mut more_patterns: i32 = 0;
        let mut num_pattern_levels: size_t = argp_args_levels(argp);
        let mut fresh13 = ::std::vec::from_elem(0, num_pattern_levels as usize);
        let mut pattern_levels: *mut i8 = fresh13.as_mut_ptr() as *mut i8;
        memset(pattern_levels as *mut libc::c_void, 0 as i32, num_pattern_levels);
        loop {
            let mut old_lm: i32 = 0;
            let mut old_wm: i32 = argp_fmtstream_set_wmargin(
                fs,
                uparams.usage_indent as size_t,
            ) as i32;
            let mut levels: *mut i8 = pattern_levels;
            if first_pattern != 0 {
                argp_fmtstream_printf(
                    fs,
                    b"%s %s\0" as *const u8 as *const i8,
                    dcgettext(
                        (*argp).argp_domain,
                        b"Usage:\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    name,
                );
            } else {
                argp_fmtstream_printf(
                    fs,
                    b"%s %s\0" as *const u8 as *const i8,
                    dcgettext(
                        (*argp).argp_domain,
                        b"  or: \0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    name,
                );
            }
            old_lm = argp_fmtstream_set_lmargin(fs, uparams.usage_indent as size_t)
                as i32;
            if flags & 0x2 as i32 as u32 != 0 {
                if (*hol).num_entries > 0 as i32 as u32 {
                    argp_fmtstream_puts(
                        fs,
                        dcgettext(
                            (*argp).argp_domain,
                            b" [OPTION...]\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                }
            } else {
                hol_usage(hol, fs);
                flags |= 0x2 as i32 as u32;
            }
            more_patterns = argp_args_usage(argp, state, &mut levels, 1 as i32, fs);
            argp_fmtstream_set_wmargin(fs, old_wm as size_t);
            argp_fmtstream_set_lmargin(fs, old_lm as size_t);
            argp_fmtstream_putc(fs, '\n' as i32);
            anything = 1 as i32;
            first_pattern = 0 as i32;
            if !(more_patterns != 0) {
                break;
            }
        }
    }
    if flags & 0x10 as i32 as u32 != 0 {
        anything |= argp_doc(argp, state, 0 as i32, 0 as i32, 1 as i32, fs);
    }
    if flags & 0x4 as i32 as u32 != 0 {
        argp_fmtstream_printf(
            fs,
            dcgettext(
                (*argp).argp_domain,
                b"Try '%s --help' or '%s --usage' for more information.\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            name,
            name,
        );
        anything = 1 as i32;
    }
    if flags & 0x8 as i32 as u32 != 0 {
        if (*hol).num_entries > 0 as i32 as u32 {
            if anything != 0 {
                argp_fmtstream_putc(fs, '\n' as i32);
            }
            hol_help(hol, state, fs);
            anything = 1 as i32;
        }
    }
    if flags & 0x20 as i32 as u32 != 0 {
        anything |= argp_doc(argp, state, 1 as i32, anything, 0 as i32, fs);
    }
    if flags & 0x40 as i32 as u32 != 0 && !argp_program_bug_address.is_null() {
        if anything != 0 {
            argp_fmtstream_putc(fs, '\n' as i32);
        }
        argp_fmtstream_printf(
            fs,
            dcgettext(
                (*argp).argp_domain,
                b"Report bugs to %s.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            argp_program_bug_address,
        );
        anything = 1 as i32;
    }
    funlockfile(stream);
    if !hol.is_null() {
        hol_free(hol);
    }
    argp_fmtstream_free(fs);
}
#[no_mangle]
pub unsafe extern "C" fn argp_help(
    mut argp: *const argp,
    mut stream: *mut FILE,
    mut flags: u32,
    mut name: *mut i8,
) {
    _help(argp, 0 as *const argp_state, stream, flags, name);
}
#[no_mangle]
pub unsafe extern "C" fn argp_state_help(
    mut state: *const argp_state,
    mut stream: *mut FILE,
    mut flags: u32,
) {
    if (state.is_null() || (*state).flags & 0x2 as i32 as u32 == 0) && !stream.is_null()
    {
        if !state.is_null() && (*state).flags & 0x40 as i32 as u32 != 0 {
            flags |= 0x80 as i32 as u32;
        }
        _help(
            if !state.is_null() { (*state).root_argp } else { 0 as *const argp },
            state,
            stream,
            flags,
            if !state.is_null() { (*state).name } else { program_invocation_short_name },
        );
        if state.is_null() || (*state).flags & 0x20 as i32 as u32 == 0 {
            if flags & 0x100 as i32 as u32 != 0 {
                exit(argp_err_exit_status);
            }
            if flags & 0x200 as i32 as u32 != 0 {
                exit(0 as i32);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn argp_error(
    mut state: *const argp_state,
    mut fmt: *const i8,
    mut args: ...
) {
    if state.is_null() || (*state).flags & 0x2 as i32 as u32 == 0 {
        let mut stream: *mut FILE = if !state.is_null() {
            (*state).err_stream
        } else {
            stderr
        };
        if !stream.is_null() {
            let mut ap: ::core::ffi::VaListImpl;
            flockfile(stream);
            ap = args.clone();
            fputs_unlocked(
                if !state.is_null() {
                    (*state).name
                } else {
                    program_invocation_short_name
                },
                stream,
            );
            putc_unlocked(':' as i32, stream);
            putc_unlocked(' ' as i32, stream);
            vfprintf(stream, fmt, ap.as_va_list());
            putc_unlocked('\n' as i32, stream);
            argp_state_help(state, stream, (0x4 as i32 | 0x100 as i32) as u32);
            funlockfile(stream);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn argp_failure(
    mut state: *const argp_state,
    mut status: i32,
    mut errnum: i32,
    mut fmt: *const i8,
    mut args: ...
) {
    if state.is_null() || (*state).flags & 0x2 as i32 as u32 == 0 {
        let mut stream: *mut FILE = if !state.is_null() {
            (*state).err_stream
        } else {
            stderr
        };
        if !stream.is_null() {
            flockfile(stream);
            fputs_unlocked(
                if !state.is_null() {
                    (*state).name
                } else {
                    program_invocation_short_name
                },
                stream,
            );
            if !fmt.is_null() {
                let mut ap: ::core::ffi::VaListImpl;
                ap = args.clone();
                putc_unlocked(':' as i32, stream);
                putc_unlocked(' ' as i32, stream);
                vfprintf(stream, fmt, ap.as_va_list());
            }
            if errnum != 0 {
                let mut buf: [i8; 200] = [0; 200];
                let mut s: *const i8 = 0 as *const i8;
                putc_unlocked(':' as i32, stream);
                putc_unlocked(' ' as i32, stream);
                s = strerror_r(
                    errnum,
                    buf.as_mut_ptr(),
                    ::core::mem::size_of::<[i8; 200]>() as u64,
                );
                if s.is_null()
                    && {
                        s = strerror(errnum);
                        s.is_null()
                    }
                {
                    s = dcgettext(
                        (*(*state).root_argp).argp_domain,
                        b"Unknown system error\0" as *const u8 as *const i8,
                        5 as i32,
                    );
                }
                fputs_unlocked(s, stream);
            }
            putc_unlocked('\n' as i32, stream);
            funlockfile(stream);
            if status != 0
                && (state.is_null() || (*state).flags & 0x20 as i32 as u32 == 0)
            {
                exit(status);
            }
        }
    }
}