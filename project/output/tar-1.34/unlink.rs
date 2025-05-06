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
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn __errno_location() -> *mut i32;
    fn unlinkat(__fd: i32, __name: *const i8, __flag: i32) -> i32;
    fn rpl_free(ptr: *mut libc::c_void);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn rmdir_error(_: *const i8);
    fn unlink_error(_: *const i8);
    static mut records_written: off_t;
    fn normalize_filename_x(name: *mut i8);
    fn tar_dirname() -> *const i8;
    static mut chdir_current: i32;
    static mut chdir_fd: i32;
    fn chdir_do(dir: i32);
}
pub type __off_t = i64;
pub type off_t = __off_t;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct deferred_unlink {
    pub next: *mut deferred_unlink,
    pub dir_idx: i32,
    pub file_name: *mut i8,
    pub is_dir: bool,
    pub records_written: off_t,
}
static mut dunlink_head: *mut deferred_unlink = 0 as *const deferred_unlink
    as *mut deferred_unlink;
static mut dunlink_tail: *mut deferred_unlink = 0 as *const deferred_unlink
    as *mut deferred_unlink;
static mut dunlink_count: size_t = 0;
static mut dunlink_avail: *mut deferred_unlink = 0 as *const deferred_unlink
    as *mut deferred_unlink;
static mut deferred_unlink_delay: size_t = 0 as i32 as size_t;
unsafe extern "C" fn dunlink_alloc() -> *mut deferred_unlink {
    let mut p: *mut deferred_unlink = 0 as *mut deferred_unlink;
    if !dunlink_avail.is_null() {
        p = dunlink_avail;
        dunlink_avail = (*p).next;
        (*p).next = 0 as *mut deferred_unlink;
    } else {
        p = xmalloc(::core::mem::size_of::<deferred_unlink>() as u64)
            as *mut deferred_unlink;
    }
    return p;
}
unsafe extern "C" fn dunlink_insert(
    mut anchor: *mut deferred_unlink,
    mut p: *mut deferred_unlink,
) {
    if !anchor.is_null() {
        (*p).next = (*anchor).next;
        (*anchor).next = p;
    } else {
        (*p).next = dunlink_head;
        dunlink_head = p;
    }
    if ((*p).next).is_null() {
        dunlink_tail = p;
    }
    dunlink_count = dunlink_count.wrapping_add(1);
    dunlink_count;
}
unsafe extern "C" fn dunlink_reclaim(mut p: *mut deferred_unlink) {
    rpl_free((*p).file_name as *mut libc::c_void);
    (*p).next = dunlink_avail;
    dunlink_avail = p;
}
unsafe extern "C" fn flush_deferred_unlinks(mut force: bool) {
    let mut p: *mut deferred_unlink = 0 as *mut deferred_unlink;
    let mut prev: *mut deferred_unlink = 0 as *mut deferred_unlink;
    let mut saved_chdir: i32 = chdir_current;
    let mut current_block_19: u64;
    p = dunlink_head;
    while !p.is_null() {
        let mut next: *mut deferred_unlink = (*p).next;
        if force as i32 != 0
            || records_written as u64
                > ((*p).records_written as u64).wrapping_add(deferred_unlink_delay)
        {
            chdir_do((*p).dir_idx);
            if (*p).is_dir {
                let mut fname: *const i8 = 0 as *const i8;
                if (*p).dir_idx != 0
                    && ((*p).is_dir as i32 != 0
                        && (*((*p).file_name).offset(0 as i32 as isize) as i32
                            == 0 as i32
                            || strcmp((*p).file_name, b".\0" as *const u8 as *const i8)
                                == 0 as i32))
                {
                    prev = p;
                    p = next;
                    continue;
                } else {
                    fname = (*p).file_name;
                    if unlinkat(chdir_fd, fname, 0x200 as i32) != 0 as i32 {
                        match *__errno_location() {
                            2 => {}
                            17 | 39 => {
                                current_block_19 = 491107816642363305;
                                match current_block_19 {
                                    534499147496501303 => {
                                        rmdir_error(fname);
                                    }
                                    _ => {
                                        prev = p;
                                        p = next;
                                        continue;
                                    }
                                }
                            }
                            _ => {
                                current_block_19 = 534499147496501303;
                                match current_block_19 {
                                    534499147496501303 => {
                                        rmdir_error(fname);
                                    }
                                    _ => {
                                        prev = p;
                                        p = next;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            } else if unlinkat(chdir_fd, (*p).file_name, 0 as i32) != 0 as i32
                && *__errno_location() != 2 as i32
            {
                unlink_error((*p).file_name);
            }
            dunlink_reclaim(p);
            dunlink_count = dunlink_count.wrapping_sub(1);
            dunlink_count;
            p = next;
            if !prev.is_null() {
                (*prev).next = p;
            } else {
                dunlink_head = p;
            }
        } else {
            prev = p;
            p = next;
        }
    }
    if dunlink_head.is_null() {
        dunlink_tail = 0 as *mut deferred_unlink;
    } else if force {
        p = dunlink_head;
        while !p.is_null() {
            let mut next_0: *mut deferred_unlink = (*p).next;
            let mut fname_0: *const i8 = 0 as *const i8;
            chdir_do((*p).dir_idx);
            if (*p).dir_idx != 0
                && ((*p).is_dir as i32 != 0
                    && (*((*p).file_name).offset(0 as i32 as isize) as i32 == 0 as i32
                        || strcmp((*p).file_name, b".\0" as *const u8 as *const i8)
                            == 0 as i32))
            {
                fname_0 = tar_dirname();
                chdir_do((*p).dir_idx - 1 as i32);
            } else {
                fname_0 = (*p).file_name;
            }
            if unlinkat(chdir_fd, fname_0, 0x200 as i32) != 0 as i32 {
                if *__errno_location() != 2 as i32 {
                    rmdir_error(fname_0);
                }
            }
            dunlink_reclaim(p);
            dunlink_count = dunlink_count.wrapping_sub(1);
            dunlink_count;
            p = next_0;
        }
        dunlink_tail = 0 as *mut deferred_unlink;
        dunlink_head = dunlink_tail;
    }
    chdir_do(saved_chdir);
}
#[no_mangle]
pub unsafe extern "C" fn finish_deferred_unlinks() {
    flush_deferred_unlinks(1 as i32 != 0);
    while !dunlink_avail.is_null() {
        let mut next: *mut deferred_unlink = (*dunlink_avail).next;
        rpl_free(dunlink_avail as *mut libc::c_void);
        dunlink_avail = next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn queue_deferred_unlink(mut name: *const i8, mut is_dir: bool) {
    let mut p: *mut deferred_unlink = 0 as *mut deferred_unlink;
    if !dunlink_head.is_null()
        && records_written as u64
            > ((*dunlink_head).records_written as u64)
                .wrapping_add(deferred_unlink_delay)
    {
        flush_deferred_unlinks(0 as i32 != 0);
    }
    p = dunlink_alloc();
    (*p).next = 0 as *mut deferred_unlink;
    (*p).dir_idx = chdir_current;
    (*p).file_name = xstrdup(name);
    normalize_filename_x((*p).file_name);
    (*p).is_dir = is_dir;
    (*p).records_written = records_written;
    if (*p).is_dir as i32 != 0
        && (*((*p).file_name).offset(0 as i32 as isize) as i32 == 0 as i32
            || strcmp((*p).file_name, b".\0" as *const u8 as *const i8) == 0 as i32)
    {
        let mut q: *mut deferred_unlink = 0 as *mut deferred_unlink;
        let mut prev: *mut deferred_unlink = 0 as *mut deferred_unlink;
        q = dunlink_head;
        prev = 0 as *mut deferred_unlink;
        while !q.is_null() {
            if (*q).is_dir as i32 != 0
                && (*((*q).file_name).offset(0 as i32 as isize) as i32 == 0 as i32
                    || strcmp((*q).file_name, b".\0" as *const u8 as *const i8)
                        == 0 as i32) && (*q).dir_idx < (*p).dir_idx
            {
                break;
            }
            prev = q;
            q = (*q).next;
        }
        if !q.is_null() {
            dunlink_insert(prev, p);
        } else {
            dunlink_insert(dunlink_tail, p);
        }
    } else {
        dunlink_insert(dunlink_tail, p);
    };
}