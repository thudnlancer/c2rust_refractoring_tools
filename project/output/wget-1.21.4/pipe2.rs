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
    fn close(__fd: i32) -> i32;
    fn pipe(__pipedes: *mut i32) -> i32;
    fn pipe2(__pipedes: *mut i32, __flags: i32) -> i32;
    fn __errno_location() -> *mut i32;
    fn rpl_fcntl(fd: i32, action: i32, _: ...) -> i32;
}
#[no_mangle]
pub unsafe extern "C" fn rpl_pipe2(mut fd: *mut i32, mut flags: i32) -> i32 {
    let mut current_block: u64;
    let mut tmp: [i32; 2] = [0; 2];
    tmp[0 as i32 as usize] = *fd.offset(0 as i32 as isize);
    tmp[1 as i32 as usize] = *fd.offset(1 as i32 as isize);
    static mut have_pipe2_really: i32 = 0;
    if have_pipe2_really >= 0 as i32 {
        let mut result: i32 = pipe2(fd, flags);
        if !(result < 0 as i32 && *__errno_location() == 38 as i32) {
            have_pipe2_really = 1 as i32;
            return result;
        }
        have_pipe2_really = -(1 as i32);
    }
    if flags & !(0o2000000 as i32 | 0o4000 as i32 | 0 as i32 | 0 as i32) != 0 as i32 {
        *__errno_location() = 22 as i32;
        return -(1 as i32);
    }
    if pipe(fd) < 0 as i32 {
        return -(1 as i32);
    }
    if flags & 0o4000 as i32 != 0 {
        let mut fcntl_flags: i32 = 0;
        fcntl_flags = rpl_fcntl(*fd.offset(1 as i32 as isize), 3 as i32, 0 as i32);
        if fcntl_flags < 0 as i32
            || rpl_fcntl(
                *fd.offset(1 as i32 as isize),
                4 as i32,
                fcntl_flags | 0o4000 as i32,
            ) == -(1 as i32)
            || {
                fcntl_flags = rpl_fcntl(
                    *fd.offset(0 as i32 as isize),
                    3 as i32,
                    0 as i32,
                );
                fcntl_flags < 0 as i32
            }
            || rpl_fcntl(
                *fd.offset(0 as i32 as isize),
                4 as i32,
                fcntl_flags | 0o4000 as i32,
            ) == -(1 as i32)
        {
            current_block = 6736766248929800599;
        } else {
            current_block = 8236137900636309791;
        }
    } else {
        current_block = 8236137900636309791;
    }
    match current_block {
        8236137900636309791 => {
            if flags & 0o2000000 as i32 != 0 {
                let mut fcntl_flags_0: i32 = 0;
                fcntl_flags_0 = rpl_fcntl(
                    *fd.offset(1 as i32 as isize),
                    1 as i32,
                    0 as i32,
                );
                if fcntl_flags_0 < 0 as i32
                    || rpl_fcntl(
                        *fd.offset(1 as i32 as isize),
                        2 as i32,
                        fcntl_flags_0 | 1 as i32,
                    ) == -(1 as i32)
                    || {
                        fcntl_flags_0 = rpl_fcntl(
                            *fd.offset(0 as i32 as isize),
                            1 as i32,
                            0 as i32,
                        );
                        fcntl_flags_0 < 0 as i32
                    }
                    || rpl_fcntl(
                        *fd.offset(0 as i32 as isize),
                        2 as i32,
                        fcntl_flags_0 | 1 as i32,
                    ) == -(1 as i32)
                {
                    current_block = 6736766248929800599;
                } else {
                    current_block = 17833034027772472439;
                }
            } else {
                current_block = 17833034027772472439;
            }
            match current_block {
                6736766248929800599 => {}
                _ => return 0 as i32,
            }
        }
        _ => {}
    }
    let mut saved_errno: i32 = *__errno_location();
    close(*fd.offset(0 as i32 as isize));
    close(*fd.offset(1 as i32 as isize));
    *fd.offset(0 as i32 as isize) = tmp[0 as i32 as usize];
    *fd.offset(1 as i32 as isize) = tmp[1 as i32 as usize];
    *__errno_location() = saved_errno;
    return -(1 as i32);
}