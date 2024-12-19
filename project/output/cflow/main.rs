#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type table_entry;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn sourcerc(_: *mut libc::c_int, _: *mut *mut *mut libc::c_char);
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtol(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_long;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut rpl_optarg: *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn output();
    fn yyparse() -> libc::c_int;
    fn init_parse();
    fn pp_option(arg: *const libc::c_char);
    fn set_preprocessor(arg: *const libc::c_char);
    static mut token_stack_length: libc::c_int;
    static mut input_file_count: libc::c_uint;
    fn lookup(_: *const libc::c_char) -> *mut Symbol;
    fn install(_: *mut libc::c_char, _: libc::c_int) -> *mut Symbol;
    fn linked_list_append(plist: *mut *mut linked_list, data: *mut libc::c_void);
    fn source(name: *mut libc::c_char) -> libc::c_int;
    fn init_lex(debug_level: libc::c_int);
    fn register_output(
        name: *const libc::c_char,
        handler: Option::<
            unsafe extern "C" fn(
                cflow_output_command,
                *mut FILE,
                libc::c_int,
                *mut libc::c_void,
                *mut libc::c_void,
            ) -> libc::c_int,
        >,
        handler_data: *mut libc::c_void,
    ) -> libc::c_int;
    fn select_output_driver(name: *const libc::c_char) -> libc::c_int;
    fn output_init();
    fn gnu_output_handler(
        cmd: cflow_output_command,
        outfile: *mut FILE,
        line: libc::c_int,
        data: *mut libc::c_void,
        handler_data: *mut libc::c_void,
    ) -> libc::c_int;
    fn posix_output_handler(
        cmd: cflow_output_command,
        outfile: *mut FILE,
        line: libc::c_int,
        data: *mut libc::c_void,
        handler_data: *mut libc::c_void,
    ) -> libc::c_int;
    fn argp_parse(
        __argp: *const argp,
        _: libc::c_int,
        _: *mut *mut libc::c_char,
        __flags: libc::c_uint,
        __arg_index: *mut libc::c_int,
        __input: *mut libc::c_void,
    ) -> error_t;
    fn argp_error(__state: *const argp_state, __fmt: *const libc::c_char, _: ...);
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn argp_version_setup(
        name: *const libc::c_char,
        authors: *const *const libc::c_char,
    );
    fn set_program_name(argv0: *const libc::c_char);
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type error_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linked_list_entry {
    pub next: *mut linked_list_entry,
    pub prev: *mut linked_list_entry,
    pub list: *mut linked_list,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linked_list {
    pub free_data: linked_list_free_data_fp,
    pub head: *mut linked_list_entry,
    pub tail: *mut linked_list_entry,
}
pub type linked_list_free_data_fp = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> (),
>;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum symtype {
    SymIdentifier,
    SymToken,
    SymUndefined,
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum storage {
    AnyStorage,
    AutoStorage,
    StaticStorage,
    ExplicitExternStorage,
    ExternStorage,
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum symbol_flag {
    symbol_alias,
    symbol_parm,
    symbol_temp,
    symbol_none,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub struct symbol {
    pub owner: *mut table_entry,
    pub next: *mut Symbol,
    pub entry: *mut linked_list_entry,
    pub type_0: symtype,
    pub name: *mut libc::c_char,
    pub flag: symbol_flag,
    pub alias: *mut symbol,
    pub active: libc::c_int,
    pub expand_line: libc::c_int,
    pub token_type: libc::c_int,
    pub source: *mut libc::c_char,
    pub def_line: libc::c_int,
    pub ref_line: *mut linked_list,
    pub level: libc::c_int,
    pub decl: *mut libc::c_char,
    pub storage: storage,
    pub arity: libc::c_int,
    pub recursive: libc::c_int,
    pub ord: size_t,
    pub caller: *mut linked_list,
    pub callee: *mut linked_list,
}
pub type Symbol = symbol;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum cflow_output_command {
    cflow_output_text,
    cflow_output_symbol,
    cflow_output_separator,
    cflow_output_newline,
    cflow_output_end,
    cflow_output_begin,
    cflow_output_init,
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    _ISalnum,
    _ISpunct,
    _IScntrl,
    _ISblank,
    _ISgraph,
    _ISprint,
    _ISspace,
    _ISxdigit,
    _ISdigit,
    _ISalpha,
    _ISlower,
    _ISupper,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_option {
    pub name: *const libc::c_char,
    pub key: libc::c_int,
    pub arg: *const libc::c_char,
    pub flags: libc::c_int,
    pub doc: *const libc::c_char,
    pub group: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp {
    pub options: *const argp_option,
    pub parser: argp_parser_t,
    pub args_doc: *const libc::c_char,
    pub doc: *const libc::c_char,
    pub children: *const argp_child,
    pub help_filter: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_char,
            *mut libc::c_void,
        ) -> *mut libc::c_char,
    >,
    pub argp_domain: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_child {
    pub argp: *const argp,
    pub flags: libc::c_int,
    pub header: *const libc::c_char,
    pub group: libc::c_int,
}
pub type argp_parser_t = Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_char, *mut argp_state) -> error_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_state {
    pub root_argp: *const argp,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub next: libc::c_int,
    pub flags: libc::c_uint,
    pub arg_num: libc::c_uint,
    pub quoted: libc::c_int,
    pub input: *mut libc::c_void,
    pub child_inputs: *mut *mut libc::c_void,
    pub hook: *mut libc::c_void,
    pub name: *mut libc::c_char,
    pub err_stream: *mut FILE,
    pub out_stream: *mut FILE,
    pub pstate: *mut libc::c_void,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum option_code {
    OPT_NO_OMIT_SYMBOL_NAMES,
    OPT_OMIT_SYMBOL_NAMES,
    OPT_NO_OMIT_ARGUMENTS,
    OPT_OMIT_ARGUMENTS,
    OPT_NO_REVERSE,
    OPT_NO_PRINT_LEVEL,
    OPT_NO_NUMBER,
    OPT_NO_VERBOSE,
    OPT_NO_EMACS,
    OPT_NO_BRIEF,
    OPT_NO_TREE,
    OPT_NO_ANSI,
    OPT_NO_USE_INDENTATION,
    OPT_EMACS,
    OPT_NO_PREPROCESS,
    OPT_PREPROCESS,
    OPT_DEBUG,
    OPT_LEVEL_INDENT,
    OPT_DEFINES,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub struct option_type {
    pub str_0: *mut libc::c_char,
    pub min_match: libc::c_int,
    pub type_0: libc::c_int,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[no_mangle]
pub static mut argp_program_bug_address: *const libc::c_char = b"<bug-cflow@gnu.org>\0"
    as *const u8 as *const libc::c_char;
static mut doc: [libc::c_char; 180] = unsafe {
    *::core::mem::transmute::<
        &[u8; 180],
        &mut [libc::c_char; 180],
    >(
        b"generate a program flowgraph\x0B* The effect of each option marked with an asterisk is reversed if the option's long name is prefixed with `no-'. For example, --no-cpp cancels --cpp.\0",
    )
};
#[no_mangle]
pub static mut program_authors: [*const libc::c_char; 2] = [
    b"Sergey Poznyakoff\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut options: [argp_option; 49] = [
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"General options:\0" as *const u8 as *const libc::c_char,
            group: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"depth\0" as *const u8 as *const libc::c_char,
            key: 'd' as i32,
            arg: b"NUMBER\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Set the depth at which the flowgraph is cut off\0" as *const u8
                as *const libc::c_char,
            group: 0 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"include\0" as *const u8 as *const libc::c_char,
            key: 'i' as i32,
            arg: b"CLASSES\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Include specified classes of symbols (see below). Prepend CLASSES with ^ or - to exclude them from the output\0"
                as *const u8 as *const libc::c_char,
            group: 0 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"format\0" as *const u8 as *const libc::c_char,
            key: 'f' as i32,
            arg: b"NAME\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Use given output format NAME. Valid names are `gnu' (default) and `posix'\0"
                as *const u8 as *const libc::c_char,
            group: 0 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"reverse\0" as *const u8 as *const libc::c_char,
            key: 'r' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"* Print reverse call tree\0" as *const u8 as *const libc::c_char,
            group: 0 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"xref\0" as *const u8 as *const libc::c_char,
            key: 'x' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Produce cross-reference listing only\0" as *const u8
                as *const libc::c_char,
            group: 0 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"print\0" as *const u8 as *const libc::c_char,
            key: 'P' as i32,
            arg: b"OPT\0" as *const u8 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
            doc: b"Set printing option to OPT. Valid OPT values are: xref (or cross-ref), tree. Any unambiguous abbreviation of the above is also accepted\0"
                as *const u8 as *const libc::c_char,
            group: 0 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"output\0" as *const u8 as *const libc::c_char,
            key: 'o' as i32,
            arg: b"FILE\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Set output file name (default -, meaning stdout)\0" as *const u8
                as *const libc::c_char,
            group: 0 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Symbols classes for --include argument\0" as *const u8
                as *const libc::c_char,
            group: 0 as libc::c_int + 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"  x\0" as *const u8 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x8 as libc::c_int | 0x20 as libc::c_int,
            doc: b"all data symbols, both external and static\0" as *const u8
                as *const libc::c_char,
            group: 0 as libc::c_int + 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"  _\0" as *const u8 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x8 as libc::c_int | 0x20 as libc::c_int,
            doc: b"symbols whose names begin with an underscore\0" as *const u8
                as *const libc::c_char,
            group: 0 as libc::c_int + 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"  s\0" as *const u8 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x8 as libc::c_int | 0x20 as libc::c_int,
            doc: b"static symbols\0" as *const u8 as *const libc::c_char,
            group: 0 as libc::c_int + 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"  t\0" as *const u8 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x8 as libc::c_int | 0x20 as libc::c_int,
            doc: b"typedefs (for cross-references only)\0" as *const u8
                as *const libc::c_char,
            group: 0 as libc::c_int + 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Parser control:\0" as *const u8 as *const libc::c_char,
            group: 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"use-indentation\0" as *const u8 as *const libc::c_char,
            key: 'S' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"* Rely on indentation\0" as *const u8 as *const libc::c_char,
            group: 10 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-use-indentation\0" as *const u8 as *const libc::c_char,
            key: OPT_NO_USE_INDENTATION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
            doc: b"\0" as *const u8 as *const libc::c_char,
            group: 10 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"ansi\0" as *const u8 as *const libc::c_char,
            key: 'a' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"* Accept only sources in ANSI C\0" as *const u8
                as *const libc::c_char,
            group: 10 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-ansi\0" as *const u8 as *const libc::c_char,
            key: OPT_NO_ANSI as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
            doc: b"\0" as *const u8 as *const libc::c_char,
            group: 10 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"pushdown\0" as *const u8 as *const libc::c_char,
            key: 'p' as i32,
            arg: b"NUMBER\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Set initial token stack size to NUMBER\0" as *const u8
                as *const libc::c_char,
            group: 10 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"symbol\0" as *const u8 as *const libc::c_char,
            key: 's' as i32,
            arg: b"SYMBOL:[=]TYPE\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Register SYMBOL with given TYPE, or define an alias (if := is used). Valid types are: keyword (or kw), modifier, qualifier, identifier, type, wrapper. Any unambiguous abbreviation of the above is also accepted\0"
                as *const u8 as *const libc::c_char,
            group: 10 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"main\0" as *const u8 as *const libc::c_char,
            key: 'm' as i32,
            arg: b"NAME\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Assume main function to be called NAME\0" as *const u8
                as *const libc::c_char,
            group: 10 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"define\0" as *const u8 as *const libc::c_char,
            key: 'D' as i32,
            arg: b"NAME[=DEFN]\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Predefine NAME as a macro\0" as *const u8 as *const libc::c_char,
            group: 10 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"undefine\0" as *const u8 as *const libc::c_char,
            key: 'U' as i32,
            arg: b"NAME\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Cancel any previous definition of NAME\0" as *const u8
                as *const libc::c_char,
            group: 10 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"include-dir\0" as *const u8 as *const libc::c_char,
            key: 'I' as i32,
            arg: b"DIR\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Add the directory DIR to the list of directories to be searched for header files.\0"
                as *const u8 as *const libc::c_char,
            group: 10 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"preprocess\0" as *const u8 as *const libc::c_char,
            key: OPT_PREPROCESS as libc::c_int,
            arg: b"COMMAND\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int,
            doc: b"* Run the specified preprocessor command\0" as *const u8
                as *const libc::c_char,
            group: 10 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"cpp\0" as *const u8 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x4 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: 10 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-preprocess\0" as *const u8 as *const libc::c_char,
            key: OPT_NO_PREPROCESS as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
            doc: b"\0" as *const u8 as *const libc::c_char,
            group: 10 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-cpp\0" as *const u8 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x4 as libc::c_int | 0x2 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: 10 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Output control:\0" as *const u8 as *const libc::c_char,
            group: 20 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"number\0" as *const u8 as *const libc::c_char,
            key: 'n' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"* Print line numbers\0" as *const u8 as *const libc::c_char,
            group: 20 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-number\0" as *const u8 as *const libc::c_char,
            key: OPT_NO_NUMBER as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
            doc: b"\0" as *const u8 as *const libc::c_char,
            group: 20 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"print-level\0" as *const u8 as *const libc::c_char,
            key: 'l' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"* Print nesting level along with the call tree\0" as *const u8
                as *const libc::c_char,
            group: 20 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-print-level\0" as *const u8 as *const libc::c_char,
            key: OPT_NO_PRINT_LEVEL as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
            doc: b"\0" as *const u8 as *const libc::c_char,
            group: 20 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"level-indent\0" as *const u8 as *const libc::c_char,
            key: OPT_LEVEL_INDENT as libc::c_int,
            arg: b"ELEMENT\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Control graph appearance\0" as *const u8 as *const libc::c_char,
            group: 20 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"tree\0" as *const u8 as *const libc::c_char,
            key: 'T' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"* Draw ASCII art tree\0" as *const u8 as *const libc::c_char,
            group: 20 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-tree\0" as *const u8 as *const libc::c_char,
            key: OPT_NO_TREE as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
            doc: b"\0" as *const u8 as *const libc::c_char,
            group: 20 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"brief\0" as *const u8 as *const libc::c_char,
            key: 'b' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"* Brief output\0" as *const u8 as *const libc::c_char,
            group: 20 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-brief\0" as *const u8 as *const libc::c_char,
            key: OPT_NO_BRIEF as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
            doc: b"\0" as *const u8 as *const libc::c_char,
            group: 20 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"emacs\0" as *const u8 as *const libc::c_char,
            key: OPT_EMACS as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"* Additionally format output for use with GNU Emacs\0" as *const u8
                as *const libc::c_char,
            group: 20 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-emacs\0" as *const u8 as *const libc::c_char,
            key: OPT_NO_EMACS as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
            doc: b"\0" as *const u8 as *const libc::c_char,
            group: 20 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"omit-arguments\0" as *const u8 as *const libc::c_char,
            key: OPT_OMIT_ARGUMENTS as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"* Do not print argument lists in function declarations\0" as *const u8
                as *const libc::c_char,
            group: 20 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-ignore-arguments\0" as *const u8 as *const libc::c_char,
            key: OPT_NO_OMIT_ARGUMENTS as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
            doc: b"\0" as *const u8 as *const libc::c_char,
            group: 20 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"omit-symbol-names\0" as *const u8 as *const libc::c_char,
            key: OPT_OMIT_SYMBOL_NAMES as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"* Do not print symbol names in declaration strings\0" as *const u8
                as *const libc::c_char,
            group: 20 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-omit-symbol-names\0" as *const u8 as *const libc::c_char,
            key: OPT_NO_OMIT_SYMBOL_NAMES as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
            doc: b"\0" as *const u8 as *const libc::c_char,
            group: 20 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Informational options:\0" as *const u8 as *const libc::c_char,
            group: 30 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"verbose\0" as *const u8 as *const libc::c_char,
            key: 'v' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"* Verbose error diagnostics\0" as *const u8 as *const libc::c_char,
            group: 30 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-verbose\0" as *const u8 as *const libc::c_char,
            key: OPT_NO_VERBOSE as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
            doc: b"\0" as *const u8 as *const libc::c_char,
            group: 30 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"debug\0" as *const u8 as *const libc::c_char,
            key: OPT_DEBUG as libc::c_int,
            arg: b"NUMBER\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int,
            doc: b"Set debugging level\0" as *const u8 as *const libc::c_char,
            group: 30 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0,
            arg: 0 as *const libc::c_char,
            flags: 0,
            doc: 0 as *const libc::c_char,
            group: 0,
        };
        init
    },
];
#[no_mangle]
pub static mut debug: libc::c_int = 0;
#[no_mangle]
pub static mut outname: *mut libc::c_char = b"-\0" as *const u8 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut print_option: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut verbose: libc::c_int = 0;
#[no_mangle]
pub static mut use_indentation: libc::c_int = 0;
#[no_mangle]
pub static mut record_defines: libc::c_int = 0;
#[no_mangle]
pub static mut strict_ansi: libc::c_int = 0;
#[no_mangle]
pub static mut print_line_numbers: libc::c_int = 0;
#[no_mangle]
pub static mut print_levels: libc::c_int = 0;
#[no_mangle]
pub static mut print_as_tree: libc::c_int = 0;
#[no_mangle]
pub static mut brief_listing: libc::c_int = 0;
#[no_mangle]
pub static mut reverse_tree: libc::c_int = 0;
#[no_mangle]
pub static mut max_depth: libc::c_int = 0;
#[no_mangle]
pub static mut emacs_option: libc::c_int = 0;
#[no_mangle]
pub static mut omit_arguments_option: libc::c_int = 0;
#[no_mangle]
pub static mut omit_symbol_names_option: libc::c_int = 0;
#[no_mangle]
pub static mut symbol_map: libc::c_int = 0;
#[no_mangle]
pub static mut level_indent: [*mut libc::c_char; 2] = [
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub static mut level_end: [*mut libc::c_char; 2] = [
    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub static mut level_begin: *mut libc::c_char = b"\0" as *const u8 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut preprocess_option: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut start_name: *mut libc::c_char = b"main\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut arglist: *mut linked_list = 0 as *const linked_list as *mut linked_list;
unsafe extern "C" fn find_option_type(
    mut optype: *mut option_type,
    mut str: *const libc::c_char,
    mut len: libc::c_int,
) -> libc::c_int {
    if len == 0 as libc::c_int {
        len = strlen(str) as libc::c_int;
    }
    while !((*optype).str_0).is_null() {
        if len >= (*optype).min_match
            && memcmp(
                str as *const libc::c_void,
                (*optype).str_0 as *const libc::c_void,
                len as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            return (*optype).type_0;
        }
        optype = optype.offset(1);
        optype;
    }
    return 0 as libc::c_int;
}
static mut symbol_optype: [option_type; 8] = [
    {
        let mut init = option_type {
            str_0: b"keyword\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            min_match: 2 as libc::c_int,
            type_0: 257 as libc::c_int,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: b"kw\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            min_match: 2 as libc::c_int,
            type_0: 257 as libc::c_int,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: b"modifier\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            min_match: 1 as libc::c_int,
            type_0: 265 as libc::c_int,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: b"identifier\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            min_match: 1 as libc::c_int,
            type_0: 260 as libc::c_int,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: b"type\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            min_match: 1 as libc::c_int,
            type_0: 270 as libc::c_int,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: b"wrapper\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            min_match: 1 as libc::c_int,
            type_0: 272 as libc::c_int,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: b"qualifier\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            min_match: 1 as libc::c_int,
            type_0: 273 as libc::c_int,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: 0 as *const libc::c_char as *mut libc::c_char,
            min_match: 0,
            type_0: 0,
        };
        init
    },
];
unsafe extern "C" fn symbol_override(mut str: *const libc::c_char) {
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sp: *mut Symbol = 0 as *mut Symbol;
    ptr = strchr(str, ':' as i32);
    if ptr.is_null() {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: no symbol type supplied\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            str,
        );
        return;
    } else {
        name = strndup(str, ptr.offset_from(str) as libc::c_long as libc::c_ulong);
        if *ptr.offset(1 as libc::c_int as isize) as libc::c_int == '=' as i32 {
            let mut alias: *mut Symbol = lookup(ptr.offset(2 as libc::c_int as isize));
            if alias.is_null() {
                alias = install(
                    xstrdup(ptr.offset(2 as libc::c_int as isize)),
                    0x1 as libc::c_int,
                );
                (*alias).type_0 = SymToken;
                (*alias).token_type = 0 as libc::c_int;
                (*alias).source = 0 as *mut libc::c_char;
                (*alias).def_line = -(1 as libc::c_int);
                (*alias).ref_line = 0 as *mut linked_list;
            }
            sp = install(name, 0x1 as libc::c_int);
            (*sp).type_0 = SymToken;
            (*sp).alias = alias;
            (*sp).flag = symbol_alias;
        } else {
            let mut type_0: libc::c_int = find_option_type(
                symbol_optype.as_mut_ptr(),
                ptr.offset(1 as libc::c_int as isize),
                0 as libc::c_int,
            );
            if type_0 == 0 as libc::c_int {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"unknown symbol type: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    ptr.offset(1 as libc::c_int as isize),
                );
                return;
            }
            sp = install(name, 0x1 as libc::c_int);
            (*sp).type_0 = SymToken;
            (*sp).token_type = type_0;
        }
        (*sp).source = 0 as *mut libc::c_char;
        (*sp).def_line = -(1 as libc::c_int);
        (*sp).ref_line = 0 as *mut linked_list;
    };
}
static mut print_optype: [option_type; 4] = [
    {
        let mut init = option_type {
            str_0: b"xref\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            min_match: 1 as libc::c_int,
            type_0: 0x1 as libc::c_int,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: b"cross-ref\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            min_match: 1 as libc::c_int,
            type_0: 0x1 as libc::c_int,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: b"tree\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            min_match: 1 as libc::c_int,
            type_0: 0x2 as libc::c_int,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: 0 as *const libc::c_char as *mut libc::c_char,
            min_match: 0,
            type_0: 0,
        };
        init
    },
];
unsafe extern "C" fn set_print_option(mut str: *mut libc::c_char) {
    let mut opt: libc::c_int = 0;
    opt = find_option_type(print_optype.as_mut_ptr(), str, 0 as libc::c_int);
    if opt == 0 as libc::c_int {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"unknown print option: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            str,
        );
        return;
    }
    print_option |= opt;
}
unsafe extern "C" fn number(
    mut str_ptr: *mut *const libc::c_char,
    mut base: libc::c_int,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    let mut str: *const libc::c_char = *str_ptr;
    n = 0 as libc::c_int;
    while *str as libc::c_int != 0 && count != 0 {
        let fresh0 = str;
        str = str.offset(1);
        c = *fresh0 as libc::c_int;
        if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            i = (c - '0' as i32) as libc::c_uint;
        } else {
            i = (({
                let mut __res: libc::c_int = 0;
                if ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = c;
                        __res = (if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        });
                    } else {
                        __res = toupper(c);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc()).offset(c as isize);
                }
                __res
            }) - 'A' as i32 + 10 as libc::c_int) as libc::c_uint;
        }
        if i > base as libc::c_uint {
            break;
        }
        n = ((n * base) as libc::c_uint).wrapping_add(i) as libc::c_int;
        count -= 1;
        count;
    }
    *str_ptr = str.offset(-(1 as libc::c_int as isize));
    return n;
}
static mut level_indent_optype: [option_type; 6] = [
    {
        let mut init = option_type {
            str_0: b"begin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            min_match: 1 as libc::c_int,
            type_0: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: b"start\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            min_match: 1 as libc::c_int,
            type_0: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            min_match: 1 as libc::c_int,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            min_match: 1 as libc::c_int,
            type_0: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: b"end0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            min_match: 4 as libc::c_int,
            type_0: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: b"end1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            min_match: 4 as libc::c_int,
            type_0: 5 as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn parse_level_string(
    mut str: *const libc::c_char,
    mut return_ptr: *mut *mut libc::c_char,
) {
    static mut text: [libc::c_char; 216] = [0; 216];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    p = text.as_mut_ptr();
    memset(
        text.as_mut_ptr() as *mut libc::c_void,
        ' ' as i32,
        ::core::mem::size_of::<[libc::c_char; 216]>() as libc::c_ulong,
    );
    text[(::core::mem::size_of::<[libc::c_char; 216]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = 0 as libc::c_int as libc::c_char;
    while *str != 0 {
        let mut current_block_31: u64;
        match *str as libc::c_int {
            92 => {
                str = str.offset(1);
                match *str as libc::c_int {
                    97 => {
                        let fresh1 = p;
                        p = p.offset(1);
                        *fresh1 = '\u{7}' as i32 as libc::c_char;
                    }
                    98 => {
                        let fresh2 = p;
                        p = p.offset(1);
                        *fresh2 = '\u{8}' as i32 as libc::c_char;
                    }
                    101 => {
                        let fresh3 = p;
                        p = p.offset(1);
                        *fresh3 = '\u{1b}' as i32 as libc::c_char;
                    }
                    102 => {
                        let fresh4 = p;
                        p = p.offset(1);
                        *fresh4 = '\u{c}' as i32 as libc::c_char;
                    }
                    110 => {
                        let fresh5 = p;
                        p = p.offset(1);
                        *fresh5 = '\n' as i32 as libc::c_char;
                    }
                    114 => {
                        let fresh6 = p;
                        p = p.offset(1);
                        *fresh6 = '\r' as i32 as libc::c_char;
                    }
                    116 => {
                        let fresh7 = p;
                        p = p.offset(1);
                        *fresh7 = '\t' as i32 as libc::c_char;
                    }
                    120 | 88 => {
                        str = str.offset(1);
                        str;
                        let fresh8 = p;
                        p = p.offset(1);
                        *fresh8 = number(&mut str, 16 as libc::c_int, 2 as libc::c_int)
                            as libc::c_char;
                    }
                    48 => {
                        str = str.offset(1);
                        str;
                        let fresh9 = p;
                        p = p.offset(1);
                        *fresh9 = number(&mut str, 8 as libc::c_int, 3 as libc::c_int)
                            as libc::c_char;
                    }
                    _ => {
                        let fresh10 = p;
                        p = p.offset(1);
                        *fresh10 = *str;
                    }
                }
                str = str.offset(1);
                str;
                current_block_31 = 6450636197030046351;
            }
            120 => {
                if p == text.as_mut_ptr() {
                    current_block_31 = 174307006595874281;
                } else {
                    num = strtol(
                        str.offset(1 as libc::c_int as isize),
                        &mut str as *mut *const libc::c_char as *mut *mut libc::c_char,
                        10 as libc::c_int,
                    ) as libc::c_int;
                    c = *p.offset(-(1 as libc::c_int) as isize) as libc::c_int;
                    i = 1 as libc::c_int;
                    while i < num {
                        let fresh11 = p;
                        p = p.offset(1);
                        *fresh11 = c as libc::c_char;
                        if *p as libc::c_int == 0 as libc::c_int {
                            error(
                                1 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"level indent string is too long\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                            return;
                        }
                        i += 1;
                        i;
                    }
                    current_block_31 = 6450636197030046351;
                }
            }
            _ => {
                current_block_31 = 174307006595874281;
            }
        }
        match current_block_31 {
            174307006595874281 => {
                let fresh12 = str;
                str = str.offset(1);
                let fresh13 = p;
                p = p.offset(1);
                *fresh13 = *fresh12;
                if *p as libc::c_int == 0 as libc::c_int {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"level indent string is too long\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    return;
                }
            }
            _ => {}
        }
    }
    *p = 0 as libc::c_int as libc::c_char;
    *return_ptr = strdup(text.as_mut_ptr());
}
unsafe extern "C" fn set_level_indent(mut str: *const libc::c_char) {
    let mut n: libc::c_long = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    n = strtol(str, &mut q, 0 as libc::c_int);
    if *q as libc::c_int == 0 as libc::c_int && n > 0 as libc::c_int as libc::c_long {
        let mut s: *mut libc::c_char = xmalloc(
            (n + 1 as libc::c_int as libc::c_long) as size_t,
        ) as *mut libc::c_char;
        memset(
            s as *mut libc::c_void,
            ' ' as i32,
            (n - 1 as libc::c_int as libc::c_long) as libc::c_ulong,
        );
        *s
            .offset(
                (n - 1 as libc::c_int as libc::c_long) as isize,
            ) = 0 as libc::c_int as libc::c_char;
        level_indent[1 as libc::c_int as usize] = s;
        level_indent[0 as libc::c_int
            as usize] = level_indent[1 as libc::c_int as usize];
        return;
    }
    p = str;
    while *p as libc::c_int != '=' as i32 {
        if *p as libc::c_int == 0 as libc::c_int {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"level-indent syntax\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return;
        }
        p = p.offset(1);
        p;
    }
    p = p.offset(1);
    p;
    match find_option_type(
        level_indent_optype.as_mut_ptr(),
        str,
        (p.offset_from(str) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as libc::c_int,
    ) {
        1 => {
            parse_level_string(p, &mut level_begin);
        }
        2 => {
            parse_level_string(
                p,
                &mut *level_indent.as_mut_ptr().offset(0 as libc::c_int as isize),
            );
        }
        3 => {
            parse_level_string(
                p,
                &mut *level_indent.as_mut_ptr().offset(1 as libc::c_int as isize),
            );
        }
        4 => {
            parse_level_string(
                p,
                &mut *level_end.as_mut_ptr().offset(0 as libc::c_int as isize),
            );
        }
        5 => {
            parse_level_string(
                p,
                &mut *level_end.as_mut_ptr().offset(1 as libc::c_int as isize),
            );
        }
        _ => {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"unknown level indent option: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                str,
            );
        }
    };
}
unsafe extern "C" fn add_name(mut name: *const libc::c_char) {
    linked_list_append(&mut arglist, name as *mut libc::c_void);
}
unsafe extern "C" fn add_preproc_option(
    mut key: libc::c_int,
    mut arg: *const libc::c_char,
) {
    let mut opt: *mut libc::c_char = xmalloc(
        (3 as libc::c_int as libc::c_ulong).wrapping_add(strlen(arg)),
    ) as *mut libc::c_char;
    sprintf(opt, b"-%c%s\0" as *const u8 as *const libc::c_char, key, arg);
    add_name(opt);
    preprocess_option = 1 as libc::c_int;
}
unsafe extern "C" fn parse_opt(
    mut key: libc::c_int,
    mut arg: *mut libc::c_char,
    mut state: *mut argp_state,
) -> error_t {
    let mut num: libc::c_int = 0;
    match key {
        97 => {
            strict_ansi = 1 as libc::c_int;
        }
        263 => {
            strict_ansi = 0 as libc::c_int;
        }
        258 => {
            debug = if !arg.is_null() { atoi(arg) } else { 1 as libc::c_int };
        }
        80 => {
            set_print_option(arg);
        }
        83 => {
            use_indentation = 1 as libc::c_int;
        }
        262 => {
            use_indentation = 0 as libc::c_int;
        }
        84 => {
            print_as_tree = 1 as libc::c_int;
            set_level_indent(b"0=  \0" as *const u8 as *const libc::c_char);
            set_level_indent(b"1=| \0" as *const u8 as *const libc::c_char);
            set_level_indent(b"end0=+-\0" as *const u8 as *const libc::c_char);
            set_level_indent(b"end1=\\\\-\0" as *const u8 as *const libc::c_char);
        }
        264 => {
            print_as_tree = 0 as libc::c_int;
            level_indent[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
            level_indent[0 as libc::c_int
                as usize] = level_indent[1 as libc::c_int as usize];
            level_end[0 as libc::c_int as usize] = 0 as *mut libc::c_char;
            level_end[0 as libc::c_int as usize] = level_end[0 as libc::c_int as usize];
        }
        98 => {
            brief_listing = 1 as libc::c_int;
        }
        265 => {
            brief_listing = 0 as libc::c_int;
        }
        100 => {
            max_depth = atoi(arg);
            if max_depth < 0 as libc::c_int {
                max_depth = 0 as libc::c_int;
            }
        }
        256 => {
            record_defines = 1 as libc::c_int;
        }
        261 => {
            emacs_option = 1 as libc::c_int;
        }
        266 => {
            emacs_option = 0 as libc::c_int;
        }
        102 => {
            if select_output_driver(arg) != 0 {
                argp_error(
                    state,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: No such output driver\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    rpl_optarg,
                );
            }
            output_init();
        }
        257 => {
            set_level_indent(arg);
        }
        105 => {
            num = 1 as libc::c_int;
            while *arg != 0 {
                match *arg as libc::c_int {
                    45 | 94 => {
                        num = 0 as libc::c_int;
                    }
                    43 => {
                        num = 1 as libc::c_int;
                    }
                    120 | 95 | 115 | 116 | 117 => {
                        if num != 0 {
                            symbol_map
                                |= if *arg as libc::c_int == 'x' as i32 {
                                    0x2 as libc::c_int
                                } else if *arg as libc::c_int == '_' as i32 {
                                    0x8 as libc::c_int
                                } else if *arg as libc::c_int == 's' as i32 {
                                    0x4 as libc::c_int
                                } else if *arg as libc::c_int == 't' as i32 {
                                    0x10 as libc::c_int
                                } else if *arg as libc::c_int == 'u' as i32 {
                                    0x20 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                };
                        } else {
                            symbol_map
                                &= !if *arg as libc::c_int == 'x' as i32 {
                                    0x2 as libc::c_int
                                } else if *arg as libc::c_int == '_' as i32 {
                                    0x8 as libc::c_int
                                } else if *arg as libc::c_int == 's' as i32 {
                                    0x4 as libc::c_int
                                } else if *arg as libc::c_int == 't' as i32 {
                                    0x10 as libc::c_int
                                } else if *arg as libc::c_int == 'u' as i32 {
                                    0x20 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                };
                        }
                    }
                    _ => {
                        argp_error(
                            state,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Unknown symbol class: %c\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            *arg as libc::c_int,
                        );
                    }
                }
                arg = arg.offset(1);
                arg;
            }
        }
        271 => {
            omit_arguments_option = 1 as libc::c_int;
        }
        272 => {
            omit_arguments_option = 0 as libc::c_int;
        }
        273 => {
            omit_symbol_names_option = 1 as libc::c_int;
        }
        274 => {
            omit_symbol_names_option = 0 as libc::c_int;
        }
        108 => {
            print_levels = 1 as libc::c_int;
        }
        269 => {
            print_levels = 0 as libc::c_int;
        }
        109 => {
            start_name = strdup(arg);
        }
        110 => {
            print_line_numbers = 1 as libc::c_int;
        }
        268 => {
            print_line_numbers = 0 as libc::c_int;
        }
        111 => {
            outname = strdup(arg);
        }
        112 => {
            num = atoi(arg);
            if num > 0 as libc::c_int {
                token_stack_length = num;
            }
        }
        114 => {
            reverse_tree = 1 as libc::c_int;
        }
        270 => {
            reverse_tree = 0 as libc::c_int;
        }
        115 => {
            symbol_override(arg);
        }
        118 => {
            verbose = 1 as libc::c_int;
        }
        267 => {
            verbose = 0 as libc::c_int;
        }
        120 => {
            print_option = 0x1 as libc::c_int;
        }
        259 => {
            preprocess_option = 1 as libc::c_int;
            set_preprocessor(
                if !arg.is_null() {
                    arg
                } else {
                    b"/usr/bin/cpp\0" as *const u8 as *const libc::c_char
                },
            );
        }
        260 => {
            preprocess_option = 0 as libc::c_int;
        }
        0 => {
            add_name(arg);
        }
        73 | 68 | 85 => {
            add_preproc_option(key, arg);
        }
        _ => return 7 as libc::c_int,
    }
    return 0 as libc::c_int;
}
static mut argp: argp = unsafe {
    {
        let mut init = argp {
            options: options.as_ptr() as *mut _,
            parser: Some(
                parse_opt
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *mut libc::c_char,
                        *mut argp_state,
                    ) -> error_t,
            ),
            args_doc: b"[FILE]...\0" as *const u8 as *const libc::c_char,
            doc: doc.as_ptr() as *mut _,
            children: 0 as *const argp_child,
            help_filter: None,
            argp_domain: 0 as *const libc::c_char,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn globals_only() -> libc::c_int {
    return (symbol_map & 0x4 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn include_symbol(mut sym: *mut Symbol) -> libc::c_int {
    let mut type_0: libc::c_int = 0 as libc::c_int;
    if sym.is_null() {
        return 0 as libc::c_int;
    }
    if (*sym).type_0 as libc::c_uint == SymIdentifier as libc::c_int as libc::c_uint {
        if *((*sym).name).offset(0 as libc::c_int as isize) as libc::c_int == '_' as i32
            && symbol_map & 0x8 as libc::c_int == 0
        {
            return 0 as libc::c_int;
        }
        if (*sym).storage as libc::c_uint == StaticStorage as libc::c_int as libc::c_uint
        {
            type_0 |= 0x4 as libc::c_int;
        }
        if (*sym).arity == -(1 as libc::c_int)
            && (*sym).storage as libc::c_uint
                != AutoStorage as libc::c_int as libc::c_uint
        {
            type_0 |= 0x2 as libc::c_int;
        } else if (*sym).arity >= 0 as libc::c_int {
            type_0 |= 0x1 as libc::c_int;
        }
        if ((*sym).source).is_null() {
            type_0 |= 0x20 as libc::c_int;
        }
    } else if (*sym).type_0 as libc::c_uint == SymToken as libc::c_int as libc::c_uint {
        if (*sym).token_type == 270 as libc::c_int && !((*sym).source).is_null() {
            type_0 |= 0x10 as libc::c_int;
        } else {
            return 0 as libc::c_int
        }
    }
    return (symbol_map & type_0 == type_0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xalloc_die() -> ! {
    error(
        1 as libc::c_int,
        12 as libc::c_int,
        dcgettext(
            0 as *const libc::c_char,
            b"Exiting\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn init() {
    if (level_indent[0 as libc::c_int as usize]).is_null() {
        level_indent[0 as libc::c_int
            as usize] = b"    \0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    if (level_indent[1 as libc::c_int as usize]).is_null() {
        level_indent[1 as libc::c_int
            as usize] = level_indent[0 as libc::c_int as usize];
    }
    if (level_end[0 as libc::c_int as usize]).is_null() {
        level_end[0 as libc::c_int
            as usize] = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (level_end[1 as libc::c_int as usize]).is_null() {
        level_end[1 as libc::c_int
            as usize] = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    init_lex((debug > 1 as libc::c_int) as libc::c_int);
    init_parse();
}
#[no_mangle]
pub static mut version_etc_copyright: [libc::c_char; 63] = unsafe {
    *::core::mem::transmute::<
        &[u8; 63],
        &[libc::c_char; 63],
    >(b"Copyright %s 2005, 2006, 2009, 2010, 2011 %d Sergey Poznyakoff\0")
};
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut index: libc::c_int = 0;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    argp_version_setup(
        b"cflow\0" as *const u8 as *const libc::c_char,
        program_authors.as_mut_ptr(),
    );
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"cflow\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"cflow\0" as *const u8 as *const libc::c_char);
    register_output(
        b"gnu\0" as *const u8 as *const libc::c_char,
        Some(
            gnu_output_handler
                as unsafe extern "C" fn(
                    cflow_output_command,
                    *mut FILE,
                    libc::c_int,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        0 as *mut libc::c_void,
    );
    register_output(
        b"posix\0" as *const u8 as *const libc::c_char,
        Some(
            posix_output_handler
                as unsafe extern "C" fn(
                    cflow_output_command,
                    *mut FILE,
                    libc::c_int,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        0 as *mut libc::c_void,
    );
    symbol_map = 0x1 as libc::c_int | 0x4 as libc::c_int | 0x20 as libc::c_int;
    if !(getenv(b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char)).is_null() {
        if select_output_driver(b"posix\0" as *const u8 as *const libc::c_char) != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: No such output driver\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"posix\0" as *const u8 as *const libc::c_char,
            );
        }
        output_init();
    }
    sourcerc(&mut argc, &mut argv);
    if argp_parse(
        &mut argp,
        argc,
        argv,
        0x8 as libc::c_int as libc::c_uint,
        &mut index,
        0 as *mut libc::c_void,
    ) != 0
    {
        exit(1 as libc::c_int);
    }
    if print_option == 0 as libc::c_int {
        print_option = 0x2 as libc::c_int;
    }
    init();
    if !arglist.is_null() {
        let mut p: *mut linked_list_entry = 0 as *mut linked_list_entry;
        p = (*arglist).head;
        while !p.is_null() {
            let mut s: *mut libc::c_char = (*p).data as *mut libc::c_char;
            if *s.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
                pp_option(s);
            } else if source(s) == 0 as libc::c_int {
                yyparse();
            }
            p = (*p).next;
        }
    }
    argc -= index;
    argv = argv.offset(index as isize);
    loop {
        let fresh14 = argc;
        argc = argc - 1;
        if !(fresh14 != 0) {
            break;
        }
        let fresh15 = argv;
        argv = argv.offset(1);
        if source(*fresh15) == 0 as libc::c_int {
            yyparse();
        }
    }
    if input_file_count == 0 as libc::c_int as libc::c_uint {
        error(
            1 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"no input files\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    output();
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
