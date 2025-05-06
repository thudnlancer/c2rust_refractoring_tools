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
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strlen(_: *const i8) -> u64;
    static mut argmatch_die: argmatch_exit_fn;
    fn __xargmatch_internal(
        context: *const i8,
        arg: *const i8,
        arglist: *const *const i8,
        vallist: *const libc::c_void,
        valsize: size_t,
        exit_fn: argmatch_exit_fn,
    ) -> ptrdiff_t;
}
pub type size_t = u64;
pub type ptrdiff_t = i64;
pub type argmatch_exit_fn = Option<unsafe extern "C" fn() -> ()>;
static mut warning_args: [*const i8; 26] = [
    b"all\0" as *const u8 as *const i8,
    b"alone-zero-block\0" as *const u8 as *const i8,
    b"bad-dumpdir\0" as *const u8 as *const i8,
    b"cachedir\0" as *const u8 as *const i8,
    b"contiguous-cast\0" as *const u8 as *const i8,
    b"file-changed\0" as *const u8 as *const i8,
    b"file-ignored\0" as *const u8 as *const i8,
    b"file-removed\0" as *const u8 as *const i8,
    b"file-shrank\0" as *const u8 as *const i8,
    b"file-unchanged\0" as *const u8 as *const i8,
    b"filename-with-nuls\0" as *const u8 as *const i8,
    b"ignore-archive\0" as *const u8 as *const i8,
    b"ignore-newer\0" as *const u8 as *const i8,
    b"new-directory\0" as *const u8 as *const i8,
    b"rename-directory\0" as *const u8 as *const i8,
    b"symlink-cast\0" as *const u8 as *const i8,
    b"timestamp\0" as *const u8 as *const i8,
    b"unknown-cast\0" as *const u8 as *const i8,
    b"unknown-keyword\0" as *const u8 as *const i8,
    b"xdev\0" as *const u8 as *const i8,
    b"decompress-program\0" as *const u8 as *const i8,
    b"existing-file\0" as *const u8 as *const i8,
    b"xattr-write\0" as *const u8 as *const i8,
    b"record-size\0" as *const u8 as *const i8,
    b"failed-read\0" as *const u8 as *const i8,
    0 as *const i8,
];
static mut warning_types: [i32; 25] = [
    !(0x2000 as i32 | 0x1000 as i32 | 0x80000 as i32 | 0x100000 as i32
        | 0x400000 as i32),
    0x1 as i32,
    0x2 as i32,
    0x4 as i32,
    0x8 as i32,
    0x10 as i32,
    0x20 as i32,
    0x40 as i32,
    0x80 as i32,
    0x100 as i32,
    0x200 as i32,
    0x400 as i32,
    0x800 as i32,
    0x1000 as i32,
    0x2000 as i32,
    0x4000 as i32,
    0x8000 as i32,
    0x10000 as i32,
    0x20000 as i32,
    0x40000 as i32,
    0x80000 as i32,
    0x100000 as i32,
    0x200000 as i32,
    0x400000 as i32,
    0x800000 as i32,
];
#[no_mangle]
pub static mut warning_option: i32 = !(0x2000 as i32 | 0x1000 as i32 | 0x80000 as i32
    | 0x100000 as i32 | 0x400000 as i32);
#[no_mangle]
pub unsafe extern "C" fn set_warning_option(mut arg: *const i8) {
    let mut negate: i32 = 0 as i32;
    let mut option: i32 = 0;
    if strcmp(arg, b"none\0" as *const u8 as *const i8) == 0 as i32 {
        warning_option = 0 as i32;
        return;
    }
    if strlen(arg) > 2 as i32 as u64
        && memcmp(
            arg as *const libc::c_void,
            b"no-\0" as *const u8 as *const i8 as *const libc::c_void,
            3 as i32 as u64,
        ) == 0 as i32
    {
        negate = 1 as i32;
        arg = arg.offset(3 as i32 as isize);
    }
    option = warning_types[__xargmatch_internal(
        b"--warning\0" as *const u8 as *const i8,
        arg,
        warning_args.as_ptr(),
        warning_types.as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<i32>() as u64,
        argmatch_die,
    ) as usize];
    if negate != 0 {
        warning_option &= !option;
    } else {
        warning_option |= option;
    };
}