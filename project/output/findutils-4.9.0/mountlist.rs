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
    fn rpl_fclose(stream: *mut FILE) -> i32;
    fn sscanf(_: *const i8, _: *const i8, _: ...) -> i32;
    fn __getdelim(
        __lineptr: *mut *mut i8,
        __n: *mut size_t,
        __delimiter: i32,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn rpl_free(ptr: *mut libc::c_void);
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strstr(_: *const i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn __errno_location() -> *mut i32;
    fn getmntent(__stream: *mut FILE) -> *mut mntent;
    fn hasmntopt(__mnt: *const mntent, __opt: *const i8) -> *mut i8;
    fn setmntent(__file: *const i8, __mode: *const i8) -> *mut FILE;
    fn endmntent(__stream: *mut FILE) -> i32;
}
pub type __dev_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __ssize_t = i64;
pub type dev_t = __dev_t;
pub type size_t = u64;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mount_entry {
    pub me_devname: *mut i8,
    pub me_mountdir: *mut i8,
    pub me_mntroot: *mut i8,
    pub me_type: *mut i8,
    pub me_dev: dev_t,
    #[bitfield(name = "me_dummy", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "me_remote", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "me_type_malloced", ty = "libc::c_uint", bits = "2..=2")]
    pub me_dummy_me_remote_me_type_malloced: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub me_next: *mut mount_entry,
}
pub type FILE = _IO_FILE;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mntent {
    pub mnt_fsname: *mut i8,
    pub mnt_dir: *mut i8,
    pub mnt_type: *mut i8,
    pub mnt_opts: *mut i8,
    pub mnt_freq: i32,
    pub mnt_passno: i32,
}
#[inline]
unsafe extern "C" fn gnu_dev_makedev(mut __major: u32, mut __minor: u32) -> __dev_t {
    let mut __dev: __dev_t = 0;
    __dev = ((__major & 0xfff as u32) as __dev_t) << 8 as i32;
    __dev |= ((__major & 0xfffff000 as u32) as __dev_t) << 32 as i32;
    __dev |= ((__minor & 0xff as u32) as __dev_t) << 0 as i32;
    __dev |= ((__minor & 0xffffff00 as u32) as __dev_t) << 12 as i32;
    return __dev;
}
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut i8,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> i32 {
    return ((*__stream)._flags & 0x20 as i32 != 0 as i32) as i32;
}
unsafe extern "C" fn dev_from_mount_options(mut mount_options: *const i8) -> dev_t {
    return -(1 as i32) as dev_t;
}
unsafe extern "C" fn unescape_tab(mut str: *mut i8) {
    let mut i: size_t = 0;
    let mut j: size_t = 0 as i32 as size_t;
    let mut len: size_t = (strlen(str)).wrapping_add(1 as i32 as u64);
    i = 0 as i32 as size_t;
    while i < len {
        if *str.offset(i as isize) as i32 == '\\' as i32
            && i.wrapping_add(4 as i32 as u64) < len
            && *str.offset(i.wrapping_add(1 as i32 as u64) as isize) as i32 >= '0' as i32
            && *str.offset(i.wrapping_add(1 as i32 as u64) as isize) as i32 <= '3' as i32
            && *str.offset(i.wrapping_add(2 as i32 as u64) as isize) as i32 >= '0' as i32
            && *str.offset(i.wrapping_add(2 as i32 as u64) as isize) as i32 <= '7' as i32
            && *str.offset(i.wrapping_add(3 as i32 as u64) as isize) as i32 >= '0' as i32
            && *str.offset(i.wrapping_add(3 as i32 as u64) as isize) as i32 <= '7' as i32
        {
            let fresh0 = j;
            j = j.wrapping_add(1);
            *str.offset(fresh0 as isize) = ((*str
                .offset(i.wrapping_add(1 as i32 as u64) as isize) as i32 - '0' as i32)
                * 64 as i32
                + (*str.offset(i.wrapping_add(2 as i32 as u64) as isize) as i32
                    - '0' as i32) * 8 as i32
                + (*str.offset(i.wrapping_add(3 as i32 as u64) as isize) as i32
                    - '0' as i32)) as i8;
            i = (i as u64).wrapping_add(3 as i32 as u64) as size_t as size_t;
        } else {
            let fresh1 = j;
            j = j.wrapping_add(1);
            *str.offset(fresh1 as isize) = *str.offset(i as isize);
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn terminate_at_blank(mut str: *mut i8) -> *mut i8 {
    let mut s: *mut i8 = strchr(str, ' ' as i32);
    if !s.is_null() {
        *s = '\0' as i32 as i8;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn read_file_system_list(
    mut need_fs_type: bool,
) -> *mut mount_entry {
    let mut current_block: u64;
    let mut mount_list: *mut mount_entry = 0 as *mut mount_entry;
    let mut me: *mut mount_entry = 0 as *mut mount_entry;
    let mut mtail: *mut *mut mount_entry = &mut mount_list;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut mountinfo: *const i8 = b"/proc/self/mountinfo\0" as *const u8 as *const i8;
    fp = fopen(mountinfo, b"re\0" as *const u8 as *const i8);
    if !fp.is_null() {
        let mut line: *mut i8 = 0 as *mut i8;
        let mut buf_size: size_t = 0 as i32 as size_t;
        while getline(&mut line, &mut buf_size, fp) != -(1 as i32) as i64 {
            let mut devmaj: u32 = 0;
            let mut devmin: u32 = 0;
            let mut rc: i32 = 0;
            let mut mntroot_s: i32 = 0;
            rc = sscanf(
                line,
                b"%*u %*u %u:%u %n\0" as *const u8 as *const i8,
                &mut devmaj as *mut u32,
                &mut devmin as *mut u32,
                &mut mntroot_s as *mut i32,
            );
            if rc != 2 as i32 && rc != 3 as i32 {
                continue;
            }
            let mut mntroot: *mut i8 = line.offset(mntroot_s as isize);
            let mut blank: *mut i8 = terminate_at_blank(mntroot);
            if blank.is_null() {
                continue;
            }
            let mut target: *mut i8 = blank.offset(1 as i32 as isize);
            blank = terminate_at_blank(target);
            if blank.is_null() {
                continue;
            }
            let mut dash: *mut i8 = strstr(
                blank.offset(1 as i32 as isize),
                b" - \0" as *const u8 as *const i8,
            );
            if dash.is_null() {
                continue;
            }
            let mut fstype: *mut i8 = dash.offset(3 as i32 as isize);
            blank = terminate_at_blank(fstype);
            if blank.is_null() {
                continue;
            }
            let mut source: *mut i8 = blank.offset(1 as i32 as isize);
            if (terminate_at_blank(source)).is_null() {
                continue;
            }
            unescape_tab(source);
            unescape_tab(target);
            unescape_tab(mntroot);
            unescape_tab(fstype);
            me = xmalloc(::core::mem::size_of::<mount_entry>() as u64)
                as *mut mount_entry;
            (*me).me_devname = xstrdup(source);
            (*me).me_mountdir = xstrdup(target);
            (*me).me_mntroot = xstrdup(mntroot);
            (*me).me_type = xstrdup(fstype);
            (*me).set_me_type_malloced(1 as i32 as u32);
            (*me).me_dev = gnu_dev_makedev(devmaj, devmin);
            (*me)
                .set_me_dummy(
                    (strcmp((*me).me_type, b"autofs\0" as *const u8 as *const i8)
                        == 0 as i32
                        || strcmp((*me).me_type, b"proc\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp((*me).me_type, b"subfs\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp((*me).me_type, b"debugfs\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp((*me).me_type, b"devpts\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp((*me).me_type, b"fusectl\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp(
                            (*me).me_type,
                            b"fuse.portal\0" as *const u8 as *const i8,
                        ) == 0 as i32
                        || strcmp((*me).me_type, b"mqueue\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp(
                            (*me).me_type,
                            b"rpc_pipefs\0" as *const u8 as *const i8,
                        ) == 0 as i32
                        || strcmp((*me).me_type, b"sysfs\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp((*me).me_type, b"devfs\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp((*me).me_type, b"kernfs\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp((*me).me_type, b"ignore\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp((*me).me_type, b"none\0" as *const u8 as *const i8)
                            == 0 as i32 && 0 as i32 == 0) as i32 as u32,
                );
            (*me)
                .set_me_remote(
                    (!(strchr((*me).me_devname, ':' as i32)).is_null()
                        || *((*me).me_devname).offset(0 as i32 as isize) as i32
                            == '/' as i32
                            && *((*me).me_devname).offset(1 as i32 as isize) as i32
                                == '/' as i32
                            && (strcmp(
                                (*me).me_type,
                                b"smbfs\0" as *const u8 as *const i8,
                            ) == 0 as i32
                                || strcmp(
                                    (*me).me_type,
                                    b"smb3\0" as *const u8 as *const i8,
                                ) == 0 as i32
                                || strcmp(
                                    (*me).me_type,
                                    b"cifs\0" as *const u8 as *const i8,
                                ) == 0 as i32)
                        || strcmp((*me).me_type, b"acfs\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp((*me).me_type, b"afs\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp((*me).me_type, b"coda\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp(
                            (*me).me_type,
                            b"auristorfs\0" as *const u8 as *const i8,
                        ) == 0 as i32
                        || strcmp((*me).me_type, b"fhgfs\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp((*me).me_type, b"gpfs\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp((*me).me_type, b"ibrix\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp((*me).me_type, b"ocfs2\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp((*me).me_type, b"vxfs\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp(
                            b"-hosts\0" as *const u8 as *const i8,
                            (*me).me_devname,
                        ) == 0 as i32) as i32 as u32,
                );
            *mtail = me;
            mtail = &mut (*me).me_next;
        }
        rpl_free(line as *mut libc::c_void);
        if ferror_unlocked(fp) != 0 {
            let mut saved_errno: i32 = *__errno_location();
            rpl_fclose(fp);
            *__errno_location() = saved_errno;
            current_block = 12497913735442871383;
        } else if rpl_fclose(fp) == -(1 as i32) {
            current_block = 12497913735442871383;
        } else {
            current_block = 11626999923138678822;
        }
    } else {
        let mut mnt: *mut mntent = 0 as *mut mntent;
        let mut table: *const i8 = b"/etc/mtab\0" as *const u8 as *const i8;
        fp = setmntent(table, b"r\0" as *const u8 as *const i8);
        if fp.is_null() {
            return 0 as *mut mount_entry;
        }
        loop {
            mnt = getmntent(fp);
            if mnt.is_null() {
                break;
            }
            let mut bind: bool = !(hasmntopt(mnt, b"bind\0" as *const u8 as *const i8))
                .is_null();
            me = xmalloc(::core::mem::size_of::<mount_entry>() as u64)
                as *mut mount_entry;
            (*me).me_devname = xstrdup((*mnt).mnt_fsname);
            (*me).me_mountdir = xstrdup((*mnt).mnt_dir);
            (*me).me_mntroot = 0 as *mut i8;
            (*me).me_type = xstrdup((*mnt).mnt_type);
            (*me).set_me_type_malloced(1 as i32 as u32);
            (*me)
                .set_me_dummy(
                    (strcmp((*me).me_type, b"autofs\0" as *const u8 as *const i8)
                        == 0 as i32
                        || strcmp((*me).me_type, b"proc\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp((*me).me_type, b"subfs\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp((*me).me_type, b"debugfs\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp((*me).me_type, b"devpts\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp((*me).me_type, b"fusectl\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp(
                            (*me).me_type,
                            b"fuse.portal\0" as *const u8 as *const i8,
                        ) == 0 as i32
                        || strcmp((*me).me_type, b"mqueue\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp(
                            (*me).me_type,
                            b"rpc_pipefs\0" as *const u8 as *const i8,
                        ) == 0 as i32
                        || strcmp((*me).me_type, b"sysfs\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp((*me).me_type, b"devfs\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp((*me).me_type, b"kernfs\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp((*me).me_type, b"ignore\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp((*me).me_type, b"none\0" as *const u8 as *const i8)
                            == 0 as i32 && !bind) as i32 as u32,
                );
            (*me)
                .set_me_remote(
                    (!(strchr((*me).me_devname, ':' as i32)).is_null()
                        || *((*me).me_devname).offset(0 as i32 as isize) as i32
                            == '/' as i32
                            && *((*me).me_devname).offset(1 as i32 as isize) as i32
                                == '/' as i32
                            && (strcmp(
                                (*me).me_type,
                                b"smbfs\0" as *const u8 as *const i8,
                            ) == 0 as i32
                                || strcmp(
                                    (*me).me_type,
                                    b"smb3\0" as *const u8 as *const i8,
                                ) == 0 as i32
                                || strcmp(
                                    (*me).me_type,
                                    b"cifs\0" as *const u8 as *const i8,
                                ) == 0 as i32)
                        || strcmp((*me).me_type, b"acfs\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp((*me).me_type, b"afs\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp((*me).me_type, b"coda\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp(
                            (*me).me_type,
                            b"auristorfs\0" as *const u8 as *const i8,
                        ) == 0 as i32
                        || strcmp((*me).me_type, b"fhgfs\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp((*me).me_type, b"gpfs\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp((*me).me_type, b"ibrix\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp((*me).me_type, b"ocfs2\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp((*me).me_type, b"vxfs\0" as *const u8 as *const i8)
                            == 0 as i32
                        || strcmp(
                            b"-hosts\0" as *const u8 as *const i8,
                            (*me).me_devname,
                        ) == 0 as i32) as i32 as u32,
                );
            (*me).me_dev = dev_from_mount_options((*mnt).mnt_opts);
            *mtail = me;
            mtail = &mut (*me).me_next;
        }
        if endmntent(fp) == 0 as i32 {
            current_block = 12497913735442871383;
        } else {
            current_block = 11626999923138678822;
        }
    }
    match current_block {
        12497913735442871383 => {
            let mut saved_errno_0: i32 = *__errno_location();
            *mtail = 0 as *mut mount_entry;
            while !mount_list.is_null() {
                me = (*mount_list).me_next;
                free_mount_entry(mount_list);
                mount_list = me;
            }
            *__errno_location() = saved_errno_0;
            return 0 as *mut mount_entry;
        }
        _ => {
            *mtail = 0 as *mut mount_entry;
            return mount_list;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn free_mount_entry(mut me: *mut mount_entry) {
    rpl_free((*me).me_devname as *mut libc::c_void);
    rpl_free((*me).me_mountdir as *mut libc::c_void);
    rpl_free((*me).me_mntroot as *mut libc::c_void);
    if (*me).me_type_malloced() != 0 {
        rpl_free((*me).me_type as *mut libc::c_void);
    }
    rpl_free(me as *mut libc::c_void);
}