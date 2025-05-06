#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic)]
extern "C" {
    fn fcntl(__fd: i32, __cmd: i32, _: ...) -> i32;
    fn __errno_location() -> *mut i32;
    fn close(__fd: i32) -> i32;
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
pub type va_list = __builtin_va_list;
#[no_mangle]
pub unsafe extern "C" fn rpl_fcntl(mut fd: i32, mut action: i32, mut args: ...) -> i32 {
    let mut arg: ::core::ffi::VaListImpl;
    let mut result: i32 = -(1 as i32);
    arg = args.clone();
    match action {
        0 => {
            let mut target: i32 = arg.arg::<i32>();
            result = rpl_fcntl_DUPFD(fd, target);
        }
        1030 => {
            let mut target_0: i32 = arg.arg::<i32>();
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
                    let mut x: i32 = arg.arg::<i32>();
                    result = fcntl(fd, action, x);
                }
                _ => {}
            }
        }
    }
    return result;
}
unsafe extern "C" fn rpl_fcntl_DUPFD(mut fd: i32, mut target: i32) -> i32 {
    let mut result: i32 = 0;
    result = fcntl(fd, 0 as i32, target);
    return result;
}
static mut have_dupfd_cloexec: i32 = 0;
unsafe extern "C" fn rpl_fcntl_DUPFD_CLOEXEC(mut fd: i32, mut target: i32) -> i32 {
    let mut result: i32 = 0;
    if 0 as i32 <= have_dupfd_cloexec {
        result = fcntl(fd, 1030 as i32, target);
        if 0 as i32 <= result || *__errno_location() != 22 as i32 {
            have_dupfd_cloexec = 1 as i32;
        } else {
            result = rpl_fcntl_DUPFD(fd, target);
            if result >= 0 as i32 {
                have_dupfd_cloexec = -(1 as i32);
            }
        }
    } else {
        result = rpl_fcntl_DUPFD(fd, target);
    }
    if 0 as i32 <= result && have_dupfd_cloexec == -(1 as i32) {
        let mut flags: i32 = fcntl(result, 1 as i32);
        if flags < 0 as i32 || fcntl(result, 2 as i32, flags | 1 as i32) == -(1 as i32) {
            let mut saved_errno: i32 = *__errno_location();
            close(result);
            *__errno_location() = saved_errno;
            result = -(1 as i32);
        }
    }
    return result;
}
unsafe extern "C" fn run_static_initializers() {
    have_dupfd_cloexec = if 0 as i32 != 0 { -(1 as i32) } else { 0 as i32 };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];