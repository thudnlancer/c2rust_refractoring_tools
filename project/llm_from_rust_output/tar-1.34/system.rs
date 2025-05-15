use libc::{c_char, c_int, c_uint, c_ulong, c_void, dev_t, gid_t, mode_t, off_t, pid_t, size_t, ssize_t, stat, timespec, uid_t};
use std::ffi::{CStr, CString};
use std::mem;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::RawFd;
use std::ptr;

static mut archive_stat: stat = unsafe { mem::zeroed() };
static mut ar_dev: dev_t = 0;
static mut ar_ino: libc::ino_t = 0;

pub unsafe fn sys_get_archive_stat() -> bool {
    libc::fstat(archive, &mut archive_stat) == 0
}

pub unsafe fn sys_file_is_archive(p: *mut tar_stat_info) -> bool {
    ar_dev != 0 && (*p).stat.st_dev == ar_dev && (*p).stat.st_ino == ar_ino
}

pub unsafe fn sys_save_archive_dev_ino() {
    if !(archive >= (1 << 30)) && archive_stat.st_mode & 0o170000 == 0o100000 {
        ar_dev = archive_stat.st_dev;
        ar_ino = archive_stat.st_ino;
    } else {
        ar_dev = 0;
    }
}

pub unsafe fn sys_detect_dev_null_output() {
    let dev_null = b"/dev/null\0".as_ptr() as *const c_char;
    let mut dev_null_stat: stat = mem::zeroed();
    
    dev_null_output = libc::strcmp(*archive_name_array.offset(0), dev_null) == 0 ||
        !(archive >= (1 << 30)) && 
        archive_stat.st_mode & 0o170000 == 0o20000 &&
        libc::stat(dev_null, &mut dev_null_stat) == 0 &&
        archive_stat.st_dev == dev_null_stat.st_dev &&
        archive_stat.st_ino == dev_null_stat.st_ino;
}

pub unsafe fn sys_wait_for_child(child_pid: pid_t, eof: bool) {
    if child_pid != 0 {
        let mut wait_status: c_int = 0;
        while libc::waitpid(child_pid, &mut wait_status, 0) == -1 {
            if libc::__errno_location().read() != libc::EINTR {
                waitpid_error(use_compress_program_option);
                break;
            }
        }
        
        if ((wait_status & 0x7f) + 1) >> 1 > 0 {
            let sig = wait_status & 0x7f;
            if !(!eof && sig == libc::SIGPIPE) {
                if let Some(hook) = error_hook {
                    hook();
                }
                error(
                    0,
                    0,
                    dcgettext(
                        ptr::null(),
                        b"Child died with signal %d\0".as_ptr() as *const c_char,
                        5,
                    ),
                    sig,
                );
                fatal_exit();
            }
        } else if (wait_status & 0xff00) >> 8 != 0 {
            if let Some(hook) = error_hook {
                hook();
            }
            error(
                0,
                0,
                dcgettext(
                    ptr::null(),
                    b"Child returned status %d\0".as_ptr() as *const c_char,
                    5,
                ),
                (wait_status & 0xff00) >> 8,
            );
            fatal_exit();
        }
    }
}

pub unsafe fn sys_spawn_shell() {
    let shell = libc::getenv(b"SHELL\0".as_ptr() as *const c_char);
    let shell = if shell.is_null() {
        b"/bin/sh\0".as_ptr() as *const c_char
    } else {
        shell
    };

    let child = xfork();
    if child == 0 {
        priv_set_restore_linkdir();
        libc::execlp(
            shell,
            b"-sh\0".as_ptr() as *const c_char,
            b"-i\0".as_ptr() as *const c_char,
            ptr::null_mut(),
        );
        exec_fatal(shell);
    } else {
        let mut wait_status: c_int = 0;
        while libc::waitpid(child, &mut wait_status, 0) == -1 {
            if libc::__errno_location().read() != libc::EINTR {
                waitpid_error(shell);
                break;
            }
        }
    }
}

pub unsafe fn sys_compare_uid(a: *mut stat, b: *mut stat) -> bool {
    (*a).st_uid == (*b).st_uid
}

pub unsafe fn sys_compare_gid(a: *mut stat, b: *mut stat) -> bool {
    (*a).st_gid == (*b).st_gid
}

pub unsafe fn sys_compare_links(link_data: *mut stat, stat_data: *mut stat) -> bool {
    (*stat_data).st_dev == (*link_data).st_dev && (*stat_data).st_ino == (*link_data).st_ino
}

pub unsafe fn sys_truncate(fd: c_int) -> c_int {
    let pos = libc::lseek(fd, 0, libc::SEEK_CUR);
    if pos < 0 {
        -1
    } else {
        libc::ftruncate(fd, pos)
    }
}

unsafe fn is_regular_file(name: *const c_char) -> c_int {
    let mut stbuf: stat = mem::zeroed();
    if libc::stat(name, &mut stbuf) == 0 {
        (stbuf.st_mode & 0o170000 == 0o100000) as c_int
    } else {
        (libc::__errno_location().read() == libc::ENOENT) as c_int
    }
}

pub unsafe fn sys_write_archive_buffer() -> size_t {
    if archive >= (1 << 30) {
        rmt_write__(
            archive - (1 << 30),
            (*record_start).buffer.as_mut_ptr(),
            record_size,
        )
    } else {
        full_write(
            archive,
            (*record_start).buffer.as_mut_ptr() as *const c_void,
            record_size,
        )
    }
}

unsafe fn xdup2(from: c_int, into: c_int) {
    if from != into {
        let status = libc::close(into);
        if status != 0 && libc::__errno_location().read() != libc::EBADF {
            let e = libc::__errno_location().read();
            if let Some(hook) = error_hook {
                hook();
            }
            error(
                0,
                e,
                dcgettext(
                    ptr::null(),
                    b"Cannot close\0".as_ptr() as *const c_char,
                    5,
                ),
            );
            fatal_exit();
        }
        
        let status = libc::dup(from);
        if status != into {
            if status < 0 {
                let e = libc::__errno_location().read();
                if let Some(hook) = error_hook {
                    hook();
                }
                error(
                    0,
                    e,
                    dcgettext(
                        ptr::null(),
                        b"Cannot dup\0".as_ptr() as *const c_char,
                        5,
                    ),
                );
                fatal_exit();
            }
            libc::abort();
        }
        xclose(from);
    }
}

unsafe fn wait_for_grandchild(pid: pid_t) -> ! {
    let mut wait_status: c_int = 0;
    let mut exit_code: c_int = 0;
    
    while libc::waitpid(pid, &mut wait_status, 0) == -1 {
        if libc::__errno_location().read() != libc::EINTR {
            waitpid_error(use_compress_program_option);
            break;
        }
    }
    
    if ((wait_status & 0x7f) + 1) >> 1 > 0 {
        libc::raise(wait_status & 0x7f);
    } else if (wait_status & 0xff00) >> 8 != 0 {
        exit_code = (wait_status & 0xff00) >> 8;
    }
    
    libc::exit(exit_code);
}

pub unsafe fn sys_child_open_for_compress() -> pid_t {
    let mut parent_pipe: [c_int; 2] = [0; 2];
    let mut child_pipe: [c_int; 2] = [0; 2];
    let mut grandchild_pid: pid_t = 0;
    let mut child_pid: pid_t = 0;
    
    libc::signal(
        libc::SIGPIPE,
        libc::SIG_IGN,
    );
    
    xpipe(parent_pipe.as_mut_ptr());
    child_pid = xfork();
    
    if child_pid > 0 {
        archive = parent_pipe[1];
        xclose(parent_pipe[0]);
        return child_pid;
    }
    
    set_program_name(
        dcgettext(
            ptr::null(),
            b"tar (child)\0".as_ptr() as *const c_char,
            5,
        ),
    );
    
    libc::signal(libc::SIGPIPE, None);
    xdup2(parent_pipe[0], 0);
    xclose(parent_pipe[1]);
    
    if !(!force_local_option && {
        rmt_dev_name__ = libc::strchr(
            *archive_name_array.offset(0),
            ':' as i32,
        );
        !rmt_dev_name__.is_null()
    } && rmt_dev_name__ > *archive_name_array.offset(0) &&
        libc::memchr(
            *archive_name_array.offset(0) as *const c_void,
            '/' as i32,
            rmt_dev_name__.offset_from(*archive_name_array.offset(0)) as size_t,
        ).is_null())
    && is_regular_file(*archive_name_array.offset(0)) != 0
    {
        if backup_option {
            maybe_backup_file(
                *archive_name_array.offset(0),
                true,
            );
        }
        
        if libc::strcmp(
            *archive_name_array.offset(0),
            b"-\0".as_ptr() as *const c_char,
        ) != 0
        {
            archive = libc::creat(
                *archive_name_array.offset(0),
                (0o200 | 0o200 >> 3 | 0o200 >> 6 | (0o400 | 0o400 >> 3 | 0o400 >> 6)) as mode_t,
            );
            
            if archive < 0 {
                let saved_errno = libc::__errno_location().read();
                if backup_option {
                    undo_last_backup();
                }
                libc::__errno_location().write(saved_errno);
                open_fatal(*archive_name_array.offset(0));
            }
            xdup2(archive, 1);
        }
        
        priv_set_restore_linkdir();
        xexec(use_compress_program_option);
    }
    
    xpipe(child_pipe.as_mut_ptr());
    grandchild_pid = xfork();
    
    if grandchild_pid == 0 {
        set_program_name(
            dcgettext(
                ptr::null(),
                b"tar (grandchild)\0".as_ptr() as *const c_char,
                5,
            ),
        );
        xdup2(child_pipe[1], 1);
        xclose(child_pipe[0]);
        priv_set_restore_linkdir();
        xexec(use_compress_program_option);
    }
    
    xdup2(child_pipe[0], 0);
    xclose(child_pipe[1]);
    
    if libc::strcmp(
        *archive_name_array.offset(0),
        b"-\0".as_ptr() as *const c_char,
    ) == 0
    {
        archive = 1;
    } else {
        archive = if !force_local_option && {
            rmt_dev_name__ = libc::strchr(
                *archive_name_array.offset(0),
                ':' as i32,
            );
            !rmt_dev_name__.is_null()
        } && rmt_dev_name__ > *archive_name_array.offset(0) &&
            libc::memchr(
                *archive_name_array.offset(0) as *const c_void,
                '/' as i32,
                rmt_dev_name__.offset_from(*archive_name_array.offset(0)) as size_t,
            ).is_null()
        {
            rmt_open__(
                *archive_name_array.offset(0),
                0o100 | 0o1,
                1 << 30,
                rsh_command_option,
            )
        } else {
            libc::creat(
                *archive_name_array.offset(0),
                (0o200 | 0o200 >> 3 | 0o200 >> 6 | (0o400 | 0o400 >> 3 | 0o400 >> 6)) as mode_t,
            )
        };
        
        if archive < 0 {
            open_fatal(*archive_name_array.offset(0));
        }
    }
    
    loop {
        let mut status: size_t = 0;
        let mut cursor: *mut c_char = (*record_start).buffer.as_mut_ptr();
        let mut length: size_t = 0;
        
        while length < record_size {
            let size = record_size - length;
            status = safe_read(0, cursor as *mut c_void, size);
            
            if status == (-1i32) as size_t {
                read_fatal(use_compress_program_option);
            }
            
            if status == 0 {
                break;
            }
            
            length += status;
            cursor = cursor.add(status as usize);
        }
        
        if status == 0 {
            if length > 0 {
                libc::memset(
                    (*record_start).buffer.as_mut_ptr().add(length as usize) as *mut c_void,
                    0,
                    record_size - length,
                );
                
                status = sys_write_archive_buffer();
                if status != record_size {
                    archive_write_error(status as ssize_t);
                }
            }
            break;
        } else {
            status = sys_write_archive_buffer();
            if status != record_size {
                archive_write_error(status as ssize_t);
            }
        }
    }
    
    wait_for_grandchild(grandchild_pid);
}