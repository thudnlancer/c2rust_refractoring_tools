#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type inode_val;
    fn strlen(_: *const i8) -> u64;
    fn rpl_free(_: *mut libc::c_void);
    fn __errno_location() -> *mut i32;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn __lxstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn __xmknod(
        __ver: i32,
        __path: *const i8,
        __mode: __mode_t,
        __dev: *mut __dev_t,
    ) -> i32;
    fn close(__fd: i32) -> i32;
    fn lchown(__file: *const i8, __owner: __uid_t, __group: __gid_t) -> i32;
    fn link(__from: *const i8, __to: *const i8) -> i32;
    fn symlink(__from: *const i8, __to: *const i8) -> i32;
    fn readlink(__path: *const i8, __buf: *mut i8, __len: size_t) -> ssize_t;
    fn unlink(__name: *const i8) -> i32;
    fn rmdir(__path: *const i8) -> i32;
    fn __overflow(_: *mut _IO_FILE, _: i32) -> i32;
    static mut stdin: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn dcngettext(
        __domainname: *const i8,
        __msgid1: *const i8,
        __msgid2: *const i8,
        __n: u64,
        __category: i32,
    ) -> *mut i8;
    fn chown_error_details(name: *const i8, uid: uid_t, gid: gid_t);
    fn close_error(_: *const i8);
    fn mknod_error(_: *const i8);
    fn open_error(_: *const i8);
    fn readlink_error(_: *const i8);
    fn stat_error(_: *const i8);
    fn symlink_error(_: *const i8, _: *const i8);
    fn ds_free(string: *mut dynamic_string);
    fn ds_reset(s: *mut dynamic_string, len: size_t);
    fn ds_fgetstr(f: *mut FILE, s: *mut dynamic_string, eos: i8) -> *mut i8;
    fn ds_append(s: *mut dynamic_string, c: i32);
    fn ds_concat(s: *mut dynamic_string, str: *const i8);
    static mut reset_time_flag: i32;
    static mut io_block_size: i32;
    static mut create_dir_flag: i32;
    static mut unconditional_flag: i32;
    static mut verbose_flag: i32;
    static mut dot_flag: i32;
    static mut link_flag: i32;
    static mut set_owner_flag: i32;
    static mut set_owner: uid_t;
    static mut set_group_flag: i32;
    static mut set_group: gid_t;
    static mut no_chown_flag: i32;
    static mut quiet_flag: i32;
    static mut newdir_umask: mode_t;
    static mut output_bytes: off_t;
    static mut directory_name: *mut i8;
    static mut name_end: i8;
    static mut output_is_seekable: i8;
    static mut xstat: Option<unsafe extern "C" fn() -> i32>;
    static mut change_directory_option: *mut i8;
    fn disk_empty_output_buffer(out_des: i32, flush: bool);
    fn create_all_directories(name: *const i8);
    fn set_perms(fd: i32, header: *mut cpio_file_stat);
    fn stat_to_cpio(hdr: *mut cpio_file_stat, st: *mut stat);
    fn add_inode(
        node_num: ino_t,
        file_name: *mut i8,
        major_num: u64,
        minor_num: u64,
    ) -> *mut inode_val;
    fn find_inode_file(node_num: ino_t, major_num: u64, minor_num: u64) -> *mut i8;
    fn apply_delayed_set_stat();
    fn cpio_create_dir(file_hdr: *mut cpio_file_stat, existing_dir: i32) -> i32;
    fn warn_if_file_changed(
        file_name: *mut i8,
        old_file_size: off_t,
        old_file_mtime: time_t,
    );
    fn set_file_times(fd: i32, name: *const i8, atime: u64, mtime: u64);
    fn copy_files_disk_to_disk(
        in_des: i32,
        out_des: i32,
        num_bytes: off_t,
        filename: *mut i8,
    );
    fn change_dir();
    fn xgetcwd() -> *mut i8;
}
pub type __uint32_t = u32;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __ssize_t = i64;
pub type __syscall_slong_t = i64;
pub type ino_t = __ino_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: i32,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpio_file_stat {
    pub c_magic: libc::c_ushort,
    pub c_ino: ino_t,
    pub c_mode: mode_t,
    pub c_uid: uid_t,
    pub c_gid: gid_t,
    pub c_nlink: size_t,
    pub c_mtime: time_t,
    pub c_filesize: off_t,
    pub c_dev_maj: u32,
    pub c_dev_min: u32,
    pub c_rdev_maj: u32,
    pub c_rdev_min: u32,
    pub c_namesize: size_t,
    pub c_chksum: uint32_t,
    pub c_name: *mut i8,
    pub c_name_buflen: size_t,
    pub c_tar_linkname: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dynamic_string {
    pub ds_size: size_t,
    pub ds_idx: size_t,
    pub ds_string: *mut i8,
}
#[inline]
unsafe extern "C" fn gnu_dev_major(mut __dev: __dev_t) -> u32 {
    let mut __major: u32 = 0;
    __major = ((__dev & 0xfff00 as u32 as __dev_t) >> 8 as i32) as u32;
    __major = (__major as u64 | (__dev & 0xfffff00000000000 as u64) >> 32 as i32) as u32;
    return __major;
}
#[inline]
unsafe extern "C" fn gnu_dev_minor(mut __dev: __dev_t) -> u32 {
    let mut __minor: u32 = 0;
    __minor = ((__dev & 0xff as u32 as __dev_t) >> 0 as i32) as u32;
    __minor = (__minor as u64 | (__dev & 0xffffff00000 as u64) >> 12 as i32) as u32;
    return __minor;
}
#[inline]
unsafe extern "C" fn lstat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __lxstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn mknod(
    mut __path: *const i8,
    mut __mode: __mode_t,
    mut __dev: __dev_t,
) -> i32 {
    return __xmknod(0 as i32, __path, __mode, &mut __dev);
}
#[inline]
unsafe extern "C" fn fputc_unlocked(mut __c: i32, mut __stream: *mut FILE) -> i32 {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as i32 as i64 != 0
    {
        __overflow(__stream, __c as u8 as i32)
    } else {
        let fresh0 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
        *fresh0 = __c as i8;
        *fresh0 as u8 as i32
    };
}
unsafe extern "C" fn set_copypass_perms(
    mut fd: i32,
    mut name: *const i8,
    mut st: *mut stat,
) {
    let mut header: cpio_file_stat = cpio_file_stat {
        c_magic: 0,
        c_ino: 0,
        c_mode: 0,
        c_uid: 0,
        c_gid: 0,
        c_nlink: 0,
        c_mtime: 0,
        c_filesize: 0,
        c_dev_maj: 0,
        c_dev_min: 0,
        c_rdev_maj: 0,
        c_rdev_min: 0,
        c_namesize: 0,
        c_chksum: 0,
        c_name: 0 as *mut i8,
        c_name_buflen: 0,
        c_tar_linkname: 0 as *const i8,
    };
    header.c_name = name as *mut i8;
    stat_to_cpio(&mut header, st);
    set_perms(fd, &mut header);
}
#[no_mangle]
pub unsafe extern "C" fn process_copy_pass() {
    let mut input_name: dynamic_string = {
        let mut init = dynamic_string {
            ds_size: 0 as i32 as size_t,
            ds_idx: 0 as i32 as size_t,
            ds_string: 0 as *mut i8,
        };
        init
    };
    let mut output_name: dynamic_string = {
        let mut init = dynamic_string {
            ds_size: 0 as i32 as size_t,
            ds_idx: 0 as i32 as size_t,
            ds_string: 0 as *mut i8,
        };
        init
    };
    let mut dirname_len: size_t = 0;
    let mut res: i32 = 0;
    let mut slash: *mut i8 = 0 as *mut i8;
    let mut in_file_stat: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut out_file_stat: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut in_file_des: i32 = 0;
    let mut out_file_des: i32 = 0;
    let mut existing_dir: i32 = 0;
    newdir_umask = umask(0 as i32 as __mode_t);
    dirname_len = strlen(directory_name);
    if !change_directory_option.is_null()
        && !(*directory_name.offset(0 as i32 as isize) as i32 == '/' as i32)
    {
        let mut pwd: *mut i8 = xgetcwd();
        ds_concat(&mut output_name, pwd);
        ds_append(&mut output_name, '/' as i32);
    }
    ds_concat(&mut output_name, directory_name);
    ds_append(&mut output_name, '/' as i32);
    dirname_len = output_name.ds_idx;
    output_is_seekable = 1 as i32 as i8;
    change_dir();
    while !(ds_fgetstr(stdin, &mut input_name, name_end)).is_null() {
        let mut link_res: i32 = -(1 as i32);
        if *(input_name.ds_string).offset(0 as i32 as isize) as i32 == '\0' as i32 {
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"blank line ignored\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        } else {
            if *(input_name.ds_string).offset(0 as i32 as isize) as i32 == '.' as i32
                && (*(input_name.ds_string).offset(1 as i32 as isize) as i32
                    == '\0' as i32
                    || *(input_name.ds_string).offset(1 as i32 as isize) as i32
                        == '/' as i32
                        && *(input_name.ds_string).offset(2 as i32 as isize) as i32
                            == '\0' as i32)
            {
                continue;
            }
            if ::core::mem::transmute::<
                _,
                fn(_, _) -> i32,
            >(
                (Some(xstat.expect("non-null function pointer")))
                    .expect("non-null function pointer"),
            )(input_name.ds_string, &mut in_file_stat) < 0 as i32
            {
                stat_error(input_name.ds_string);
            } else {
                slash = input_name.ds_string;
                while *slash as i32 == '/' as i32 {
                    slash = slash.offset(1);
                    slash;
                }
                ds_reset(&mut output_name, dirname_len);
                ds_concat(&mut output_name, slash);
                existing_dir = 0 as i32;
                if lstat(output_name.ds_string, &mut out_file_stat) == 0 as i32 {
                    if out_file_stat.st_mode & 0o170000 as i32 as u32
                        == 0o40000 as i32 as u32
                        && in_file_stat.st_mode & 0o170000 as i32 as u32
                            == 0o40000 as i32 as u32
                    {
                        existing_dir = 1 as i32;
                    } else if unconditional_flag == 0
                        && in_file_stat.st_mtim.tv_sec <= out_file_stat.st_mtim.tv_sec
                    {
                        error(
                            0 as i32,
                            0 as i32,
                            dcgettext(
                                0 as *const i8,
                                b"%s not created: newer or same age version exists\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            output_name.ds_string,
                        );
                        continue;
                    } else if if out_file_stat.st_mode & 0o170000 as i32 as u32
                        == 0o40000 as i32 as u32
                    {
                        rmdir(output_name.ds_string)
                    } else {
                        unlink(output_name.ds_string)
                    } != 0
                    {
                        error(
                            0 as i32,
                            *__errno_location(),
                            dcgettext(
                                0 as *const i8,
                                b"cannot remove current %s\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            output_name.ds_string,
                        );
                        continue;
                    }
                }
                if in_file_stat.st_mode & 0o170000 as i32 as u32
                    == 0o100000 as i32 as u32
                {
                    if link_flag != 0 {
                        link_res = link_to_name(
                            output_name.ds_string,
                            input_name.ds_string,
                        );
                    }
                    if link_res < 0 as i32 && in_file_stat.st_nlink > 1 as i32 as u64 {
                        link_res = link_to_maj_min_ino(
                            output_name.ds_string,
                            gnu_dev_major(in_file_stat.st_dev) as i32,
                            gnu_dev_minor(in_file_stat.st_dev) as i32,
                            in_file_stat.st_ino,
                        );
                    }
                    if link_res < 0 as i32 {
                        in_file_des = open(
                            input_name.ds_string,
                            0 as i32 | 0 as i32,
                            0 as i32,
                        );
                        if in_file_des < 0 as i32 {
                            open_error(input_name.ds_string);
                            continue;
                        } else {
                            out_file_des = open(
                                output_name.ds_string,
                                0o100 as i32 | 0o1 as i32 | 0 as i32,
                                0o600 as i32,
                            );
                            if out_file_des < 0 as i32 && create_dir_flag != 0 {
                                create_all_directories(output_name.ds_string);
                                out_file_des = open(
                                    output_name.ds_string,
                                    0o100 as i32 | 0o1 as i32 | 0 as i32,
                                    0o600 as i32,
                                );
                            }
                            if out_file_des < 0 as i32 {
                                open_error(output_name.ds_string);
                                close(in_file_des);
                                continue;
                            } else {
                                copy_files_disk_to_disk(
                                    in_file_des,
                                    out_file_des,
                                    in_file_stat.st_size,
                                    input_name.ds_string,
                                );
                                disk_empty_output_buffer(out_file_des, 1 as i32 != 0);
                                set_copypass_perms(
                                    out_file_des,
                                    output_name.ds_string,
                                    &mut in_file_stat,
                                );
                                if reset_time_flag != 0 {
                                    set_file_times(
                                        in_file_des,
                                        input_name.ds_string,
                                        in_file_stat.st_atim.tv_sec as u64,
                                        in_file_stat.st_mtim.tv_sec as u64,
                                    );
                                    set_file_times(
                                        out_file_des,
                                        output_name.ds_string,
                                        in_file_stat.st_atim.tv_sec as u64,
                                        in_file_stat.st_mtim.tv_sec as u64,
                                    );
                                }
                                if close(in_file_des) < 0 as i32 {
                                    close_error(input_name.ds_string);
                                }
                                if close(out_file_des) < 0 as i32 {
                                    close_error(output_name.ds_string);
                                }
                                warn_if_file_changed(
                                    input_name.ds_string,
                                    in_file_stat.st_size,
                                    in_file_stat.st_mtim.tv_sec,
                                );
                            }
                        }
                    }
                } else if in_file_stat.st_mode & 0o170000 as i32 as u32
                    == 0o40000 as i32 as u32
                {
                    let mut file_stat: cpio_file_stat = cpio_file_stat {
                        c_magic: 0,
                        c_ino: 0,
                        c_mode: 0,
                        c_uid: 0,
                        c_gid: 0,
                        c_nlink: 0,
                        c_mtime: 0,
                        c_filesize: 0,
                        c_dev_maj: 0,
                        c_dev_min: 0,
                        c_rdev_maj: 0,
                        c_rdev_min: 0,
                        c_namesize: 0,
                        c_chksum: 0,
                        c_name: 0 as *mut i8,
                        c_name_buflen: 0,
                        c_tar_linkname: 0 as *const i8,
                    };
                    stat_to_cpio(&mut file_stat, &mut in_file_stat);
                    file_stat.c_name = output_name.ds_string;
                    cpio_create_dir(&mut file_stat, existing_dir);
                } else if in_file_stat.st_mode & 0o170000 as i32 as u32
                    == 0o20000 as i32 as u32
                    || in_file_stat.st_mode & 0o170000 as i32 as u32
                        == 0o60000 as i32 as u32
                    || in_file_stat.st_mode & 0o170000 as i32 as u32
                        == 0o10000 as i32 as u32
                    || in_file_stat.st_mode & 0o170000 as i32 as u32
                        == 0o140000 as i32 as u32 || 0 as i32 != 0
                {
                    if link_flag != 0 {
                        link_res = link_to_name(
                            output_name.ds_string,
                            input_name.ds_string,
                        );
                    }
                    if link_res < 0 as i32 && in_file_stat.st_nlink > 1 as i32 as u64 {
                        link_res = link_to_maj_min_ino(
                            output_name.ds_string,
                            gnu_dev_major(in_file_stat.st_dev) as i32,
                            gnu_dev_minor(in_file_stat.st_dev) as i32,
                            in_file_stat.st_ino,
                        );
                    }
                    if link_res < 0 as i32 {
                        res = mknod(
                            output_name.ds_string,
                            in_file_stat.st_mode,
                            in_file_stat.st_rdev,
                        );
                        if res < 0 as i32 && create_dir_flag != 0 {
                            create_all_directories(output_name.ds_string);
                            res = mknod(
                                output_name.ds_string,
                                in_file_stat.st_mode,
                                in_file_stat.st_rdev,
                            );
                        }
                        if res < 0 as i32 {
                            mknod_error(output_name.ds_string);
                            continue;
                        } else {
                            set_copypass_perms(
                                -(1 as i32),
                                output_name.ds_string,
                                &mut in_file_stat,
                            );
                        }
                    }
                } else if in_file_stat.st_mode & 0o170000 as i32 as u32
                    == 0o120000 as i32 as u32
                {
                    let mut link_name: *mut i8 = 0 as *mut i8;
                    let mut link_size: i32 = 0;
                    link_name = xmalloc(
                        (in_file_stat.st_size as u32).wrapping_add(1 as i32 as u32)
                            as size_t,
                    ) as *mut i8;
                    link_size = readlink(
                        input_name.ds_string,
                        link_name,
                        in_file_stat.st_size as size_t,
                    ) as i32;
                    if link_size < 0 as i32 {
                        readlink_error(input_name.ds_string);
                        rpl_free(link_name as *mut libc::c_void);
                        continue;
                    } else {
                        *link_name.offset(link_size as isize) = '\0' as i32 as i8;
                        res = symlink(link_name, output_name.ds_string);
                        if res < 0 as i32 && create_dir_flag != 0 {
                            create_all_directories(output_name.ds_string);
                            res = symlink(link_name, output_name.ds_string);
                        }
                        if res < 0 as i32 {
                            symlink_error(output_name.ds_string, link_name);
                            rpl_free(link_name as *mut libc::c_void);
                            continue;
                        } else {
                            if no_chown_flag == 0 {
                                let mut uid: uid_t = if set_owner_flag != 0 {
                                    set_owner
                                } else {
                                    in_file_stat.st_uid
                                };
                                let mut gid: gid_t = if set_group_flag != 0 {
                                    set_group
                                } else {
                                    in_file_stat.st_gid
                                };
                                if lchown(output_name.ds_string, uid, gid) < 0 as i32
                                    && *__errno_location() != 1 as i32
                                {
                                    chown_error_details(output_name.ds_string, uid, gid);
                                }
                            }
                            rpl_free(link_name as *mut libc::c_void);
                        }
                    }
                } else {
                    error(
                        0 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"%s: unknown file type\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        input_name.ds_string,
                    );
                }
                if verbose_flag != 0 {
                    fprintf(
                        stderr,
                        b"%s\n\0" as *const u8 as *const i8,
                        output_name.ds_string,
                    );
                }
                if dot_flag != 0 {
                    fputc_unlocked('.' as i32, stderr);
                }
            }
        }
    }
    if dot_flag != 0 {
        fputc_unlocked('\n' as i32, stderr);
    }
    apply_delayed_set_stat();
    if quiet_flag == 0 {
        let mut blocks: size_t = ((output_bytes + io_block_size as i64 - 1 as i32 as i64)
            / io_block_size as i64) as size_t;
        fprintf(
            stderr,
            dcngettext(
                0 as *const i8,
                b"%lu block\n\0" as *const u8 as *const i8,
                b"%lu blocks\n\0" as *const u8 as *const i8,
                blocks,
                5 as i32,
            ),
            blocks,
        );
    }
    ds_free(&mut input_name);
    ds_free(&mut output_name);
}
#[no_mangle]
pub unsafe extern "C" fn link_to_maj_min_ino(
    mut file_name: *mut i8,
    mut st_dev_maj: i32,
    mut st_dev_min: i32,
    mut st_ino: ino_t,
) -> i32 {
    let mut link_res: i32 = 0;
    let mut link_name: *mut i8 = 0 as *mut i8;
    link_res = -(1 as i32);
    link_name = find_inode_file(st_ino, st_dev_maj as u64, st_dev_min as u64);
    if link_name.is_null() {
        add_inode(st_ino, file_name, st_dev_maj as u64, st_dev_min as u64);
    } else {
        link_res = link_to_name(file_name, link_name);
    }
    return link_res;
}
#[no_mangle]
pub unsafe extern "C" fn link_to_name(
    mut link_name: *const i8,
    mut link_target: *const i8,
) -> i32 {
    let mut res: i32 = link(link_target, link_name);
    if res < 0 as i32 && create_dir_flag != 0 {
        create_all_directories(link_name);
        res = link(link_target, link_name);
    }
    if res == 0 as i32 {
        if verbose_flag != 0 {
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s linked to %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                link_target,
                link_name,
            );
        }
    } else if link_flag != 0 {
        error(
            0 as i32,
            *__errno_location(),
            dcgettext(
                0 as *const i8,
                b"cannot link %s to %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            link_target,
            link_name,
        );
    }
    return res;
}