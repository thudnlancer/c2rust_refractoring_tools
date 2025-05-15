use std::process::{Command, Stdio, exit};
use std::os::unix::process::CommandExt;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::path::Path;
use std::env;
use std::ffi::{CString, OsString};
use nix::unistd::{fork, ForkResult, pipe, dup2, close};
use nix::sys::wait::{waitpid, WaitStatus};
use nix::sys::signal::{self, Signal};
use libc::{STDIN_FILENO, STDOUT_FILENO};
use std::ptr;
use std::mem;
use std::os::unix::ffi::OsStrExt;

static mut GLOBAL_PID: i32 = -1;

fn xexec(cmd: &str) -> ! {
    Command::new("/bin/sh")
        .arg("-c")
        .arg(cmd)
        .exec();
    panic!("exec failed for command: {}", cmd);
}

#[cfg(target_os = "msdos")]
mod msdos {
    use super::*;
    
    pub fn sys_get_archive_stat() -> bool {
        false
    }
    
    pub fn sys_file_is_archive(_p: &tar_stat_info) -> bool {
        false
    }
    
    pub fn sys_save_archive_dev_ino() {
    }
    
    pub fn sys_detect_dev_null_output() {
        let dev_null = "nul";
        unsafe {
            dev_null_output = (archive_name_array[0] == dev_null || !_isrmt(archive));
        }
    }
    
    pub fn sys_wait_for_child(_child_pid: i32, _eof: bool) {
    }
    
    pub fn sys_spawn_shell() {
        let comspec = env::var("COMSPEC").unwrap_or_default();
        Command::new(comspec)
            .arg("-")
            .status()
            .unwrap();
    }
    
    pub fn sys_compare_uid(_a: &libc::stat, _b: &libc::stat) -> bool {
        true
    }
    
    pub fn sys_compare_gid(_a: &libc::stat, _b: &libc::stat) -> bool {
        true
    }
    
    pub fn sys_compare_links(_link_data: &libc::stat, _stat_data: &libc::stat) -> bool {
        true
    }
    
    pub fn sys_truncate(fd: i32) -> i32 {
        unsafe { libc::write(fd, ptr::null(), 0) }
    }
    
    pub fn sys_write_archive_buffer() -> usize {
        unsafe {
            libc::write(archive, record_start.buffer.as_ptr() as *const _, record_size)
        }
    }
    
    pub fn sys_child_open_for_compress() {
        panic!("Cannot use compressed or remote archives");
    }
    
    pub fn sys_child_open_for_uncompress() {
        panic!("Cannot use compressed or remote archives");
    }
}

#[cfg(not(target_os = "msdos"))]
mod unix {
    use super::*;
    use std::os::unix::fs::OpenOptionsExt;
    use nix::fcntl::{open, OFlag};
    use nix::sys::stat::Mode;
    
    static mut ARCHIVE_STAT: libc::stat = unsafe { mem::zeroed() };
    
    pub fn sys_get_archive_stat() -> bool {
        unsafe { libc::fstat(archive, &mut ARCHIVE_STAT) == 0 }
    }
    
    pub fn sys_file_is_archive(p: &tar_stat_info) -> bool {
        unsafe { ar_dev != 0 && p.stat.st_dev == ar_dev && p.stat.st_ino == ar_ino }
    }
    
    pub fn sys_save_archive_dev_ino() {
        unsafe {
            if !_isrmt(archive) && (ARCHIVE_STAT.st_mode & libc::S_IFMT) == libc::S_IFREG {
                ar_dev = ARCHIVE_STAT.st_dev;
                ar_ino = ARCHIVE_STAT.st_ino;
            } else {
                ar_dev = 0;
            }
        }
    }
    
    pub fn sys_detect_dev_null_output() {
        let dev_null = "/dev/null";
        let mut dev_null_stat: libc::stat = unsafe { mem::zeroed() };
        
        unsafe {
            dev_null_output = (archive_name_array[0] == dev_null || 
                (!_isrmt(archive) && 
                 (ARCHIVE_STAT.st_mode & libc::S_IFMT) == libc::S_IFCHR && 
                 libc::stat(dev_null.as_ptr() as *const _, &mut dev_null_stat) == 0 &&
                 ARCHIVE_STAT.st_dev == dev_null_stat.st_dev &&
                 ARCHIVE_STAT.st_ino == dev_null_stat.st_ino));
        }
    }
    
    pub fn sys_wait_for_child(child_pid: i32, eof: bool) {
        if child_pid > 0 {
            let mut status = 0;
            loop {
                match waitpid(child_pid, Some(&mut status)) {
                    Ok(_) => break,
                    Err(nix::Error::Sys(nix::errno::Errno::EINTR)) => continue,
                    Err(e) => {
                        waitpid_error(use_compress_program_option);
                        break;
                    }
                }
            }
            
            if let WaitStatus::Signaled(_, sig, _) = WaitStatus::from_raw(status) {
                if !(!eof && sig == Signal::SIGPIPE) {
                    panic!("Child died with signal {}", sig);
                }
            } else if let WaitStatus::Exited(_, code) = WaitStatus::from_raw(status) {
                if code != 0 {
                    panic!("Child returned status {}", code);
                }
            }
        }
    }
    
    pub fn sys_spawn_shell() {
        let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
        match unsafe { fork() } {
            Ok(ForkResult::Child) => {
                priv_set_restore_linkdir();
                Command::new(shell)
                    .arg("-sh")
                    .arg("-i")
                    .exec();
                panic!("exec failed for shell: {}", shell);
            }
            Ok(ForkResult::Parent { child }) => {
                let mut status = 0;
                loop {
                    match waitpid(child, Some(&mut status)) {
                        Ok(_) => break,
                        Err(nix::Error::Sys(nix::errno::Errno::EINTR)) => continue,
                        Err(e) => {
                            waitpid_error(&shell);
                            break;
                        }
                    }
                }
            }
            Err(_) => panic!("fork failed"),
        }
    }
    
    pub fn sys_compare_uid(a: &libc::stat, b: &libc::stat) -> bool {
        a.st_uid == b.st_uid
    }
    
    pub fn sys_compare_gid(a: &libc::stat, b: &libc::stat) -> bool {
        a.st_gid == b.st_gid
    }
    
    pub fn sys_compare_links(link_data: &libc::stat, stat_data: &libc::stat) -> bool {
        stat_data.st_dev == link_data.st_dev && stat_data.st_ino == link_data.st_ino
    }
    
    pub fn sys_truncate(fd: i32) -> i32 {
        unsafe {
            let pos = libc::lseek(fd, 0, libc::SEEK_CUR);
            if pos < 0 { -1 } else { libc::ftruncate(fd, pos) }
        }
    }
    
    fn is_regular_file(name: &str) -> bool {
        let mut stbuf: libc::stat = unsafe { mem::zeroed() };
        unsafe {
            if libc::stat(name.as_ptr() as *const _, &mut stbuf) == 0 {
                (stbuf.st_mode & libc::S_IFMT) == libc::S_IFREG
            } else {
                nix::errno::errno() == nix::errno::Errno::ENOENT as i32
            }
        }
    }
    
    pub fn sys_write_archive_buffer() -> usize {
        unsafe {
            rmtwrite(archive, record_start.buffer.as_ptr() as *const _, record_size)
        }
    }
    
    fn xdup2(from: i32, into: i32) {
        if from != into {
            let status = unsafe { libc::close(into) };
            if status != 0 && nix::errno::errno() != nix::errno::Errno::EBADF as i32 {
                panic!("Cannot close");
            }
            let status = unsafe { libc::dup(from) };
            if status != into {
                if status < 0 {
                    panic!("Cannot dup");
                }
                panic!("dup returned wrong fd");
            }
            unsafe { libc::close(from) };
        }
    }
    
    fn wait_for_grandchild(pid: i32) -> ! {
        let mut status = 0;
        let mut exit_code = 0;
        
        loop {
            match waitpid(pid, Some(&mut status)) {
                Ok(_) => break,
                Err(nix::Error::Sys(nix::errno::Errno::EINTR)) => continue,
                Err(_) => {
                    waitpid_error(use_compress_program_option);
                    break;
                }
            }
        }
        
        if let WaitStatus::Signaled(_, sig, _) = WaitStatus::from_raw(status) {
            unsafe { libc::raise(sig as i32) };
        } else if let WaitStatus::Exited(_, code) = WaitStatus::from_raw(status) {
            exit_code = code;
        }
        
        exit(exit_code);
    }
    
    pub fn sys_child_open_for_compress() -> i32 {
        unsafe {
            signal::signal(signal::Signal::SIGPIPE, signal::SigHandler::SigIgn).unwrap();
        }
        
        let (parent_read, parent_write) = pipe().unwrap();
        let child_pid = unsafe { fork() }.unwrap();
        
        match child_pid {
            ForkResult::Parent { child } => {
                unsafe {
                    archive = parent_write;
                    libc::close(parent_read);
                }
                child.as_raw() as i32
            }
            ForkResult::Child => {
                set_program_name("tar (child)");
                unsafe {
                    signal::signal(signal::Signal::SIGPIPE, signal::SigHandler::SigDfl).unwrap();
                }
                
                xdup2(parent_read, STDIN_FILENO);
                unsafe { libc::close(parent_write) };
                
                if !_remdev(archive_name_array[0]) && is_regular_file(archive_name_array[0]) {
                    if backup_option {
                        maybe_backup_file(archive_name_array[0], true);
                    }
                    
                    if archive_name_array[0] != "-" {
                        unsafe {
                            archive = libc::creat(archive_name_array[0].as_ptr() as *const _, MODE_RW);
                            if archive < 0 {
                                let saved_errno = nix::errno::errno();
                                if backup_option {
                                    undo_last_backup();
                                }
                                panic!("open failed with errno {}", saved_errno);
                            }
                            xdup2(archive, STDOUT_FILENO);
                        }
                    }
                    priv_set_restore_linkdir();
                    xexec(use_compress_program_option);
                }
                
                let (child_read, child_write) = pipe().unwrap();
                let grandchild_pid = unsafe { fork() }.unwrap();
                
                match grandchild_pid {
                    ForkResult::Child => {
                        set_program_name("tar (grandchild)");
                        xdup2(child_write, STDOUT_FILENO);
                        unsafe { libc::close(child_read) };
                        priv_set_restore_linkdir();
                        xexec(use_compress_program_option);
                    }
                    ForkResult::Parent { child } => {
                        xdup2(child_read, STDIN_FILENO);
                        unsafe { libc::close(child_write) };
                        
                        if archive_name_array[0] == "-" {
                            unsafe { archive = STDIN_FILENO };
                        } else {
                            unsafe {
                                archive = rmtcreat(archive_name_array[0], MODE_RW, rsh_command_option);
                                if archive < 0 {
                                    panic!("open failed");
                                }
                            }
                        }
                        
                        loop {
                            let mut status = 0;
                            let mut cursor = unsafe { record_start.buffer.as_mut_ptr() };
                            let mut length = 0;
                            
                            while length < record_size {
                                let size = record_size - length;
                                status = unsafe {
                                    safe_read(STDIN_FILENO, cursor, size)
                                };
                                if status == SAFE_READ_ERROR {
                                    read_fatal(use_compress_program_option);
                                }
                                if status == 0 {
                                    break;
                                }
                                length += status;
                                cursor = unsafe { cursor.add(status) };
                            }
                            
                            if status == 0 {
                                if length > 0 {
                                    unsafe {
                                        libc::memset(record_start.buffer.as_mut_ptr().add(length), 0, record_size - length);
                                        status = sys_write_archive_buffer();
                                        if status != record_size {
                                            archive_write_error(status);
                                        }
                                    }
                                }
                                break;
                            }
                            
                            status = sys_write_archive_buffer();
                            if status != record_size {
                                archive_write_error(status);
                            }
                        }
                        
                        wait_for_grandchild(child.as_raw() as i32);
                    }
                }
            }
        }
    }
    
    fn run_decompress_program() -> ! {
        // Implementation omitted for brevity
        panic!("run_decompress_program not implemented");
    }
    
    pub fn sys_child_open_for_uncompress() -> i32 {
        let (parent_read, parent_write) = pipe().unwrap();
        let child_pid = unsafe { fork() }.unwrap();
        
        match child_pid {
            ForkResult::Parent { child } => {
                unsafe {
                    archive = parent_read;
                    libc::close(parent_write);
                }
                child.as_raw() as i32
            }
            ForkResult::Child => {
                set_program_name("tar (child)");
                unsafe {
                    signal::signal(signal::Signal::SIGPIPE, signal::SigHandler::SigDfl).unwrap();
                }
                
                xdup2(parent_write, STDOUT_FILENO);
                unsafe { libc::close(parent_read) };
                
                if archive_name_array[0] != "-" && !_remdev(archive_name_array[0]) && is_regular_file(archive_name_array[0]) {
                    unsafe {
                        archive = open(archive_name_array[0], OFlag::O_RDONLY | OFlag::O_BINARY, Mode::empty()).unwrap();
                        xdup2(archive, STDIN_FILENO);
                    }
                    priv_set_restore_linkdir();
                    run_decompress_program();
                }
                
                let (child_read, child_write) = pipe().unwrap();
                let grandchild_pid = unsafe { fork() }.unwrap();
                
                match grandchild_pid {
                    ForkResult::Child => {
                        set_program_name("tar (grandchild)");
                        xdup2(child_read, STDIN_FILENO);
                        unsafe { libc::close(child_write) };
                        priv_set_restore_linkdir();
                        run_decompress_program();
                    }
                    ForkResult::Parent { child } => {
                        xdup2(child_write, STDOUT_FILENO);
                        unsafe { libc::close(child_read) };
                        
                        if archive_name_array[0] == "-" {
                            unsafe { archive = STDIN_FILENO };
                        } else {
                            unsafe {
                                archive = rmtopen(archive_name_array[0], OFlag::O_RDONLY | OFlag::O_BINARY, Mode::empty(), rsh_command_option);
                                if archive < 0 {
                                    panic!("open failed");
                                }
                            }
                        }
                        
                        loop {
                            clear_read_error_count();
                            
                            let status = unsafe {
                                rmtread(archive, record_start.buffer.as_mut_ptr() as *mut _, record_size)
                            };
                            
                            if status == SAFE_READ_ERROR {
                                archive_read_error();
                                continue;
                            }
                            
                            if status == 0 {
                                break;
                            }
                            
                            let mut cursor = unsafe { record_start.buffer.as_mut_ptr() };
                            let mut maximum = status;
                            
                            while maximum > 0 {
                                let count = if maximum < BLOCKSIZE { maximum } else { BLOCKSIZE };
                                if full_write(STDOUT_FILENO, cursor, count) != count {
                                    write_error(use_compress_program_option);
                                }
                                maximum -= count;
                                cursor = unsafe { cursor.add(count) };
                            }
                        }
                        
                        unsafe { libc::close(STDOUT_FILENO) };
                        wait_for_grandchild(child.as_raw() as i32);
                    }
                }
            }
        }
    }
    
    pub fn sys_exec_command(file_name: &str, typechar: char, st: &tar_stat_info) -> i32 {
        let (read_fd, write_fd) = pipe().unwrap();
        unsafe {
            pipe_handler = signal::signal(signal::Signal::SIGPIPE, signal::SigHandler::SigIgn).unwrap();
            GLOBAL_PID = fork().unwrap().as_raw() as i32;
        }
        
        if unsafe { GLOBAL_PID } != 0 {
            unsafe { libc::close(read_fd) };
            write_fd
        } else {
            xdup2(read_fd, STDIN_FILENO);
            unsafe { libc::close(write_fd) };
            
            stat_to_env(file_name, typechar, st);
            priv_set_restore_linkdir();
            xexec(to_command_option);
        }
    }
    
    pub fn sys_wait_command() {
        if unsafe { GLOBAL_PID } < 0 {
            return;
        }
        
        unsafe {
            signal::signal(signal::Signal::SIGPIPE, pipe_handler).unwrap();
        }
        
        let mut status = 0;
        loop {
            match unsafe { waitpid(GLOBAL_P