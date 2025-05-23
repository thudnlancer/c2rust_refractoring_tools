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
                    current_block_7 = 18440977163753008528;
                }
                3 => {
                    current_block_7 = 18440977163753008528;
                }
                1025 => {
                    current_block_7 = 9214260670896495404;
                }
                9 => {
                    current_block_7 = 5276665666618202731;
                }
                1032 => {
                    current_block_7 = 16299334367793268400;
                }
                1034 => {
                    current_block_7 = 3231349158294193073;
                }
                11 => {
                    current_block_7 = 15909552630587231942;
                }
                1033 => {
                    current_block_7 = 6826757631989831244;
                }
                0 => {
                    current_block_7 = 6826757631989831244;
                }
                1030 => {
                    current_block_7 = 16217086138639784498;
                }
                1026 => {
                    current_block_7 = 16164524802667456711;
                }
                2 => {
                    current_block_7 = 10255222590023257813;
                }
                4 => {
                    current_block_7 = 8999665762562839288;
                }
                8 => {
                    current_block_7 = 3042746046207266158;
                }
                1031 => {
                    current_block_7 = 3042746046207266158;
                }
                1024 | 10 => {
                    current_block_7 = 9638792065705755893;
                }
                _ => {
                    let mut p: *mut libc::c_void = arg.arg::<*mut libc::c_void>();
                    result = fcntl(fd, action, p);
                    current_block_7 = 7175849428784450219;
                }
            }
            match current_block_7 {
                18440977163753008528 => {
                    current_block_7 = 9214260670896495404;
                }
                6826757631989831244 => {
                    current_block_7 = 16217086138639784498;
                }
                3042746046207266158 => {
                    current_block_7 = 9638792065705755893;
                }
                _ => {}
            }
            match current_block_7 {
                9214260670896495404 => {
                    current_block_7 = 5276665666618202731;
                }
                16217086138639784498 => {
                    current_block_7 = 16164524802667456711;
                }
                _ => {}
            }
            match current_block_7 {
                5276665666618202731 => {
                    current_block_7 = 16299334367793268400;
                }
                16164524802667456711 => {
                    current_block_7 = 10255222590023257813;
                }
                _ => {}
            }
            match current_block_7 {
                16299334367793268400 => {
                    current_block_7 = 3231349158294193073;
                }
                10255222590023257813 => {
                    current_block_7 = 8999665762562839288;
                }
                _ => {}
            }
            match current_block_7 {
                3231349158294193073 => {
                    current_block_7 = 15909552630587231942;
                }
                8999665762562839288 => {
                    current_block_7 = 9638792065705755893;
                }
                _ => {}
            }
            match current_block_7 {
                15909552630587231942 => {
                    result = fcntl(fd, action);
                }
                9638792065705755893 => {
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