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
    fn abort() -> !;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    static mut exit_failure: i32;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
}
pub type strtol_error = u32;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const i8,
    pub has_arg: i32,
    pub flag: *mut i32,
    pub val: i32,
}
unsafe extern "C" fn xstrtol_error(
    mut err: strtol_error,
    mut opt_idx: i32,
    mut c: i8,
    mut long_options: *const option,
    mut arg: *const i8,
    mut exit_status: i32,
) {
    let mut hyphens: *const i8 = b"--\0" as *const u8 as *const i8;
    let mut msgid: *const i8 = 0 as *const i8;
    let mut option: *const i8 = 0 as *const i8;
    let mut option_buffer: [i8; 2] = [0; 2];
    match err as u32 {
        4 => {
            msgid = b"invalid %s%s argument '%s'\0" as *const u8 as *const i8;
        }
        2 | 3 => {
            msgid = b"invalid suffix in %s%s argument '%s'\0" as *const u8 as *const i8;
        }
        1 => {
            msgid = b"%s%s argument '%s' too large\0" as *const u8 as *const i8;
        }
        _ => {
            abort();
        }
    }
    if opt_idx < 0 as i32 {
        hyphens = hyphens.offset(-(opt_idx as isize));
        option_buffer[0 as i32 as usize] = c;
        option_buffer[1 as i32 as usize] = '\0' as i32 as i8;
        option = option_buffer.as_mut_ptr();
    } else {
        option = (*long_options.offset(opt_idx as isize)).name;
    }
    error(
        exit_status,
        0 as i32,
        dcgettext(0 as *const i8, msgid, 5 as i32),
        hyphens,
        option,
        arg,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xstrtol_fatal(
    mut err: strtol_error,
    mut opt_idx: i32,
    mut c: i8,
    mut long_options: *const option,
    mut arg: *const i8,
) {
    xstrtol_error(err, opt_idx, c, long_options, arg, exit_failure);
    abort();
}