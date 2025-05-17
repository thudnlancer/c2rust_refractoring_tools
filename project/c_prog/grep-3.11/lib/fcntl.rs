use ::libc;
extern "C" {
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
#[no_mangle]
pub unsafe extern "C" fn rpl_fcntl(
    mut fd: libc::c_int,
    mut action: libc::c_int,
    mut args: ...
) -> libc::c_int {
    let mut arg: ::core::ffi::VaListImpl;
    let mut result: libc::c_int = -(1 as libc::c_int);
    arg = args.clone();
    match action {
        0 => {
            let mut target: libc::c_int = arg.arg::<libc::c_int>();
            result = rpl_fcntl_DUPFD(fd, target);
        }
        1030 => {
            let mut target_0: libc::c_int = arg.arg::<libc::c_int>();
            result = rpl_fcntl_DUPFD_CLOEXEC(fd, target_0);
        }
        _ => {
            let mut current_block_7: u64;
            match action {
                1 => {
                    current_block_7 = 1564046446010750164;
                }
                3 => {
                    current_block_7 = 1564046446010750164;
                }
                1025 => {
                    current_block_7 = 7855401249801537662;
                }
                9 => {
                    current_block_7 = 10517383140502681368;
                }
                1032 => {
                    current_block_7 = 11550881225573520240;
                }
                1034 => {
                    current_block_7 = 11790557091572251899;
                }
                11 => {
                    current_block_7 = 9970570182710028309;
                }
                1033 => {
                    current_block_7 = 12036211968134102509;
                }
                0 => {
                    current_block_7 = 12036211968134102509;
                }
                1030 => {
                    current_block_7 = 902018269713424055;
                }
                1026 => {
                    current_block_7 = 1854762670214663432;
                }
                2 => {
                    current_block_7 = 14228439038192055820;
                }
                4 => {
                    current_block_7 = 3601299106550578414;
                }
                8 => {
                    current_block_7 = 15755210520329575065;
                }
                1031 => {
                    current_block_7 = 15755210520329575065;
                }
                1024 | 10 => {
                    current_block_7 = 16691889613549544923;
                }
                _ => {
                    let mut p: *mut libc::c_void = arg.arg::<*mut libc::c_void>();
                    result = fcntl(fd, action, p);
                    current_block_7 = 7175849428784450219;
                }
            }
            match current_block_7 {
                1564046446010750164 => {
                    current_block_7 = 7855401249801537662;
                }
                12036211968134102509 => {
                    current_block_7 = 902018269713424055;
                }
                15755210520329575065 => {
                    current_block_7 = 16691889613549544923;
                }
                _ => {}
            }
            match current_block_7 {
                7855401249801537662 => {
                    current_block_7 = 10517383140502681368;
                }
                902018269713424055 => {
                    current_block_7 = 1854762670214663432;
                }
                _ => {}
            }
            match current_block_7 {
                10517383140502681368 => {
                    current_block_7 = 11550881225573520240;
                }
                1854762670214663432 => {
                    current_block_7 = 14228439038192055820;
                }
                _ => {}
            }
            match current_block_7 {
                11550881225573520240 => {
                    current_block_7 = 11790557091572251899;
                }
                14228439038192055820 => {
                    current_block_7 = 3601299106550578414;
                }
                _ => {}
            }
            match current_block_7 {
                11790557091572251899 => {
                    current_block_7 = 9970570182710028309;
                }
                3601299106550578414 => {
                    current_block_7 = 16691889613549544923;
                }
                _ => {}
            }
            match current_block_7 {
                9970570182710028309 => {
                    result = fcntl(fd, action);
                }
                16691889613549544923 => {
                    let mut x: libc::c_int = arg.arg::<libc::c_int>();
                    result = fcntl(fd, action, x);
                }
                _ => {}
            }
        }
    }
    return result;
}
unsafe extern "C" fn rpl_fcntl_DUPFD(
    mut fd: libc::c_int,
    mut target: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    result = fcntl(fd, 0 as libc::c_int, target);
    return result;
}
static mut have_dupfd_cloexec: libc::c_int = 0;
unsafe extern "C" fn rpl_fcntl_DUPFD_CLOEXEC(
    mut fd: libc::c_int,
    mut target: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    if 0 as libc::c_int <= have_dupfd_cloexec {
        result = fcntl(fd, 1030 as libc::c_int, target);
        if 0 as libc::c_int <= result || *__errno_location() != 22 as libc::c_int {
            have_dupfd_cloexec = 1 as libc::c_int;
        } else {
            result = rpl_fcntl_DUPFD(fd, target);
            if result >= 0 as libc::c_int {
                have_dupfd_cloexec = -(1 as libc::c_int);
            }
        }
    } else {
        result = rpl_fcntl_DUPFD(fd, target);
    }
    if 0 as libc::c_int <= result && have_dupfd_cloexec == -(1 as libc::c_int) {
        let mut flags: libc::c_int = fcntl(result, 1 as libc::c_int);
        if flags < 0 as libc::c_int
            || fcntl(result, 2 as libc::c_int, flags | 1 as libc::c_int)
                == -(1 as libc::c_int)
        {
            let mut saved_errno: libc::c_int = *__errno_location();
            close(result);
            *__errno_location() = saved_errno;
            result = -(1 as libc::c_int);
        }
    }
    return result;
}
unsafe extern "C" fn run_static_initializers() {
    have_dupfd_cloexec = if 0 as libc::c_int != 0 {
        -(1 as libc::c_int)
    } else {
        0 as libc::c_int
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
