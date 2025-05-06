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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn close(__fd: i32) -> i32;
    fn access(__name: *const i8, __type: i32) -> i32;
}
pub type size_t = u64;
pub const PROC_SELF_FD_DIR_SIZE_BOUND: C2RustUnnamed = 27;
pub type C2RustUnnamed = u32;
#[no_mangle]
pub unsafe extern "C" fn openat_proc_name(
    mut buf: *mut i8,
    mut fd: i32,
    mut file: *const i8,
) -> *mut i8 {
    let mut result: *mut i8 = buf;
    let mut dirlen: i32 = 0;
    if *file == 0 {
        *buf.offset(0 as i32 as isize) = '\0' as i32 as i8;
        return buf;
    }
    static mut proc_status: i32 = 0 as i32;
    if proc_status == 0 {
        let mut proc_self_fd: i32 = open(
            b"/proc/self/fd\0" as *const u8 as *const i8,
            0 as i32 | 0o200000 as i32 | 0o400 as i32 | 0o4000 as i32 | 0o2000000 as i32,
        );
        if proc_self_fd < 0 as i32 {
            proc_status = -(1 as i32);
        } else {
            let mut dotdot_buf: [i8; 32] = [0; 32];
            sprintf(
                dotdot_buf.as_mut_ptr(),
                b"/proc/self/fd/%d/../fd\0" as *const u8 as *const i8,
                proc_self_fd,
            );
            proc_status = if access(dotdot_buf.as_mut_ptr(), 0 as i32) != 0 {
                -(1 as i32)
            } else {
                1 as i32
            };
            close(proc_self_fd);
        }
    }
    if proc_status < 0 as i32 {
        return 0 as *mut i8
    } else {
        let mut bufsize: size_t = (PROC_SELF_FD_DIR_SIZE_BOUND as i32 as u64)
            .wrapping_add(strlen(file));
        if ((if (4096 as i32) < 4096 as i32 - 64 as i32 {
            4096 as i32
        } else {
            4096 as i32 - 64 as i32
        }) as u64) < bufsize
        {
            result = malloc(bufsize) as *mut i8;
            if result.is_null() {
                return 0 as *mut i8;
            }
        }
        dirlen = sprintf(result, b"/proc/self/fd/%d/\0" as *const u8 as *const i8, fd);
    }
    strcpy(result.offset(dirlen as isize), file);
    return result;
}