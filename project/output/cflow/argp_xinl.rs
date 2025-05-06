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
    static mut stderr: *mut _IO_FILE;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn argp_state_help(__state: *const argp_state, __stream: *mut FILE, __flags: u32);
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
pub type error_t = i32;
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
#[no_mangle]
pub unsafe extern "C" fn _option_is_end(mut __opt: *const argp_option) -> i32 {
    return ((*__opt).key == 0 && ((*__opt).name).is_null() && ((*__opt).doc).is_null()
        && (*__opt).group == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _option_is_short(mut __opt: *const argp_option) -> i32 {
    if (*__opt).flags & 0x8 as i32 != 0 {
        return 0 as i32
    } else {
        let mut __key: i32 = (*__opt).key;
        return (__key > 0 as i32 && __key <= 127 as i32 * 2 as i32 + 1 as i32
            && *(*__ctype_b_loc()).offset(__key as isize) as i32
                & _ISprint as i32 as libc::c_ushort as i32 != 0) as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn argp_usage(mut __state: *const argp_state) {
    argp_state_help(__state, stderr, (0x2 as i32 | 0x4 as i32 | 0x100 as i32) as u32);
}