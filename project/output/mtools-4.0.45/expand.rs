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
    fn exit(_: i32) -> !;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn perror(__s: *const i8);
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut i32) -> i32;
    fn dup(__fd: i32) -> i32;
    fn execvp(__file: *const i8, __argv: *const *mut i8) -> i32;
    fn fork() -> __pid_t;
    fn kill(__pid: __pid_t, __sig: i32) -> i32;
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn strpbrk(_: *const i8, _: *const i8) -> *mut i8;
    fn wait(__stat_loc: *mut i32) -> __pid_t;
    fn destroy_privs();
}
pub type __pid_t = i32;
pub type __ssize_t = i64;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn safePopenOut(
    mut command: *const *const i8,
    mut output: *mut i8,
    mut len: size_t,
) -> ssize_t {
    let mut pipefd: [i32; 2] = [0; 2];
    let mut pid: pid_t = 0;
    let mut status: i32 = 0;
    let mut last: ssize_t = 0;
    if pipe(pipefd.as_mut_ptr()) != 0 {
        return -(2 as i32) as ssize_t;
    }
    pid = fork();
    match pid {
        -1 => return -(2 as i32) as ssize_t,
        0 => {
            close(pipefd[0 as i32 as usize]);
            destroy_privs();
            close(1 as i32);
            close(2 as i32);
            if dup(pipefd[1 as i32 as usize]) < 0 as i32 {
                perror(b"Dup error\0" as *const u8 as *const i8);
                exit(1 as i32);
            }
            close(pipefd[1 as i32 as usize]);
            execvp(
                *command.offset(0 as i32 as isize),
                command.offset(1 as i32 as isize) as *const *mut i8,
            );
            exit(1 as i32);
        }
        _ => {
            close(pipefd[1 as i32 as usize]);
        }
    }
    last = read(pipefd[0 as i32 as usize], output as *mut libc::c_void, len);
    kill(pid, 9 as i32);
    wait(&mut status);
    if last < 0 as i32 as i64 {
        return -(1 as i32) as ssize_t;
    }
    return last;
}
#[no_mangle]
pub unsafe extern "C" fn expand(mut input: *const i8, mut ans: *mut i8) -> *const i8 {
    let mut last: ssize_t = 0;
    let mut buf: [i8; 256] = [0; 256];
    let mut command: [*const i8; 5] = [
        b"/bin/sh\0" as *const u8 as *const i8,
        b"sh\0" as *const u8 as *const i8,
        b"-c\0" as *const u8 as *const i8,
        0 as *const i8,
        0 as *const i8,
    ];
    *ans.offset((2048 as i32 - 1 as i32) as isize) = '\0' as i32 as i8;
    if input.is_null() {
        return 0 as *const i8;
    }
    if *input as i32 == '\0' as i32 {
        return b"\0" as *const u8 as *const i8;
    }
    if (strpbrk(input, b"$*(){}[]\\?`~\0" as *const u8 as *const i8)).is_null() {
        strncpy(ans, input, (2048 as i32 - 1 as i32) as u64);
        return ans;
    }
    snprintf(
        buf.as_mut_ptr(),
        255 as i32 as u64,
        b"echo %s\0" as *const u8 as *const i8,
        input,
    );
    command[3 as i32 as usize] = buf.as_mut_ptr();
    last = safePopenOut(command.as_mut_ptr(), ans, (2048 as i32 - 1 as i32) as size_t);
    if last < 0 as i32 as i64 {
        perror(b"Pipe read error\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    if last != 0 {
        *ans.offset((last - 1 as i32 as i64) as isize) = '\0' as i32 as i8;
    } else {
        strncpy(ans, input, (2048 as i32 - 1 as i32) as u64);
    }
    return ans;
}