#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn abort() -> !;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    static mut exit_failure: libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum strtol_error {
    LONGINT_INVALID = 4,
    LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW = 3,
    LONGINT_INVALID_SUFFIX_CHAR = 2,
    LONGINT_OVERFLOW = 1,
    LONGINT_OK = 0,
impl strtol_error {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            strtol_error::LONGINT_INVALID => 4,
            strtol_error::LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW => 3,
            strtol_error::LONGINT_INVALID_SUFFIX_CHAR => 2,
            strtol_error::LONGINT_OVERFLOW => 1,
            strtol_error::LONGINT_OK => 0,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
unsafe extern "C" fn xstrtol_error(
    mut err: strtol_error,
    mut opt_idx: libc::c_int,
    mut c: libc::c_char,
    mut long_options: *const option,
    mut arg: *const libc::c_char,
    mut exit_status: libc::c_int,
) {
    let mut hyphens: *const libc::c_char = b"--\0" as *const u8 as *const libc::c_char;
    let mut msgid: *const libc::c_char = 0 as *const libc::c_char;
    let mut option: *const libc::c_char = 0 as *const libc::c_char;
    let mut option_buffer: [libc::c_char; 2] = [0; 2];
    match err as libc::c_uint {
        4 => {
            msgid = b"invalid %s%s argument '%s'\0" as *const u8 as *const libc::c_char;
        }
        2 | 3 => {
            msgid = b"invalid suffix in %s%s argument '%s'\0" as *const u8
                as *const libc::c_char;
        }
        1 => {
            msgid = b"%s%s argument '%s' too large\0" as *const u8
                as *const libc::c_char;
        }
        _ => {
            abort();
        }
    }
    if opt_idx < 0 as libc::c_int {
        hyphens = hyphens.offset(-(opt_idx as isize));
        option_buffer[0 as libc::c_int as usize] = c;
        option_buffer[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        option = option_buffer.as_mut_ptr();
    } else {
        option = (*long_options.offset(opt_idx as isize)).name;
    }
    error(
        exit_status,
        0 as libc::c_int,
        dcgettext(0 as *const libc::c_char, msgid, 5 as libc::c_int),
        hyphens,
        option,
        arg,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xstrtol_fatal(
    mut err: strtol_error,
    mut opt_idx: libc::c_int,
    mut c: libc::c_char,
    mut long_options: *const option,
    mut arg: *const libc::c_char,
) {
    xstrtol_error(err, opt_idx, c, long_options, arg, exit_failure);
    abort();
}
