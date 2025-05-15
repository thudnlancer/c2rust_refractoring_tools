use std::cmp::Ordering;
use std::collections::HashMap;
use std::ffi::{CStr, CString, OsStr, OsString};
use std::fs::{File, Metadata, OpenOptions};
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{DirEntryExt, FileTypeExt, MetadataExt, OpenOptionsExt};
use std::path::{Path, PathBuf};
use std::ptr;

const DT_UNKNOWN: u8 = 0;
const DT_FIFO: u8 = 1;
const DT_CHR: u8 = 2;
const DT_DIR: u8 = 4;
const DT_BLK: u8 = 6;
const DT_REG: u8 = 8;
const DT_LNK: u8 = 10;
const DT_SOCK: u8 = 12;
const DT_WHT: u8 = 14;

const FTS_NO_STAT_REQUIRED: i32 = 1;
const FTS_STAT_REQUIRED: i32 = 2;
const MIN_DIR_NLINK: u64 = 2;
const _FTS_INODE_SORT_DIR_ENTRIES_THRESHOLD: usize = 10000;

#[derive(Debug)]
struct FTS {
    fts_cur: Option<FTSENT>,
    fts_child: Option<FTSENT>,
    fts_compar: Option<Box<dyn Fn(&FTSENT, &FTSENT) -> Ordering>>,
    fts_options: i32,
    fts_path: PathBuf,
    fts_pathlen: usize,
    fts_dev: u64,
    fts_rfd: i32,
    fts_cwd_fd: i32,
    fts_nitems: usize,
    fts_array: Vec<*mut FTSENT>,
    fts_leaf_optimization_works_ht: HashMap<u64, u64>,
    fts_fd_ring: Vec<i32>,
}

#[derive(Debug)]
struct FTSENT {
    fts_path: PathBuf,
    fts_accpath: PathBuf,
    fts_name: OsString,
    fts_namelen: usize,
    fts_level: i64,
    fts_info: u16,
    fts_errno: i32,
    fts_number: i64,
    fts_pointer: *mut libc::c_void,
    fts_parent: *mut FTSENT,
    fts_link: Option<Box<FTSENT>>,
    fts_fts: *mut FTS,
    fts_dirp: Option<*mut libc::DIR>,
    fts_statp: Metadata,
    fts_symfd: i32,
    fts_flags: u16,
    fts_instr: u16,
    fts_cycle: Option<Box<FTSENT>>,
}

impl FTS {
    fn new(
        argv: &[&str],
        options: i32,
        compar: Option<Box<dyn Fn(&FTSENT, &FTSENT) -> Ordering>>,
    ) -> io::Result<Self> {
        if options & !0xfff != 0 || (options & 0x4 != 0 && options & 0x200 != 0) {
            return Err(io::Error::from_raw_os_error(libc::EINVAL));
        }

        let maxarglen = argv.iter().map(|s| s.len()).max().unwrap_or(0) + 1;
        let pathlen = std::cmp::max(maxarglen, 4096);

        let mut fts = FTS {
            fts_cur: None,
            fts_child: None,
            fts_compar: compar,
            fts_options: options,
            fts_path: PathBuf::with_capacity(pathlen),
            fts_pathlen: pathlen,
            fts_dev: 0,
            fts_rfd: -1,
            fts_cwd_fd: -1,
            fts_nitems: 0,
            fts_array: Vec::new(),
            fts_leaf_optimization_works_ht: HashMap::new(),
            fts_fd_ring: Vec::new(),
        };

        if fts.fts_options & 0x2 != 0 {
            fts.fts_options |= 0x4;
            fts.fts_options &= !0x200;
        }

        let mut root = None;
        let mut nitems = 0;
        let defer_stat = fts.fts_compar.is_none() || fts.fts_options & 0x400 != 0;

        for arg in argv {
            let mut name = arg.to_string();
            if fts.fts_options & 0x800 == 0 {
                while name.len() > 2 && name.ends_with('/') {
                    name.pop();
                }
            }

            let mut ent = FTSENT {
                fts_path: PathBuf::new(),
                fts_accpath: PathBuf::new(),
                fts_name: OsString::from(&name),
                fts_namelen: name.len(),
                fts_level: 0,
                fts_info: 0,
                fts_errno: 0,
                fts_number: 0,
                fts_pointer: ptr::null_mut(),
                fts_parent: ptr::null_mut(),
                fts_link: None,
                fts_fts: &mut fts as *mut FTS,
                fts_dirp: None,
                fts_statp: Metadata::from(std::fs::metadata(".")?),
                fts_symfd: -1,
                fts_flags: 0,
                fts_instr: 3,
                fts_cycle: None,
            };

            if defer_stat && root.is_some() {
                ent.fts_info = 11;
                // fts_set_stat_required(&mut ent, true);
            } else {
                ent.fts_info = fts_stat(&mut fts, &mut ent, false)? as u16;
            }

            if let Some(compar) = &fts.fts_compar {
                // TODO: Sort entries
            } else {
                // TODO: Link entries
            }

            nitems += 1;
        }

        if let Some(compar) = &fts.fts_compar {
            if nitems > 1 {
                // TODO: Sort root entries
            }
        }

        let mut cur = FTSENT {
            fts_path: PathBuf::new(),
            fts_accpath: PathBuf::new(),
            fts_name: OsString::new(),
            fts_namelen: 0,
            fts_level: 1,
            fts_info: 9,
            fts_errno: 0,
            fts_number: 0,
            fts_pointer: ptr::null_mut(),
            fts_parent: ptr::null_mut(),
            fts_link: root,
            fts_fts: &mut fts as *mut FTS,
            fts_dirp: None,
            fts_statp: Metadata::from(std::fs::metadata(".")?),
            fts_symfd: -1,
            fts_flags: 0,
            fts_instr: 3,
            fts_cycle: None,
        };

        if !setup_dir(&mut fts) {
            return Err(io::Error::from_raw_os_error(libc::ENOMEM));
        }

        if fts.fts_options & 0x4 == 0 && fts.fts_options & 0x200 == 0 {
            fts.fts_rfd = diropen(&fts, ".")?;
            if fts.fts_rfd < 0 {
                fts.fts_options |= 0x4;
            }
        }

        fts.fts_cur = Some(cur);
        Ok(fts)
    }

    fn close(&mut self) -> io::Result<()> {
        // TODO: Clean up resources
        Ok(())
    }

    fn read(&mut self) -> io::Result<Option<FTSENT>> {
        // TODO: Implement directory traversal
        Ok(None)
    }

    fn children(&mut self, instr: i32) -> io::Result<Option<FTSENT>> {
        // TODO: Implement getting children
        Ok(None)
    }

    fn set(&mut self, p: &mut FTSENT, instr: i32) -> io::Result<()> {
        if instr != 0 && instr != 1 && instr != 2 && instr != 3 && instr != 4 {
            return Err(io::Error::from_raw_os_error(libc::EINVAL));
        }
        p.fts_instr = instr as u16;
        Ok(())
    }
}

fn setup_dir(fts: &mut FTS) -> bool {
    // TODO: Initialize directory tracking structures
    true
}

fn diropen(fts: &FTS, dir: &str) -> io::Result<i32> {
    let mut open_flags = libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC;
    if fts.fts_options & 0x10 != 0 {
        open_flags |= libc::O_NOATIME;
    }

    if fts.fts_options & 0x200 != 0 {
        let fd = unsafe {
            libc::openat(
                fts.fts_cwd_fd,
                dir.as_ptr() as *const libc::c_char,
                open_flags,
            )
        };
        if fd >= 0 {
            Ok(fd)
        } else {
            Err(io::Error::last_os_error())
        }
    } else {
        let fd = unsafe { libc::open(dir.as_ptr() as *const libc::c_char, open_flags) };
        if fd >= 0 {
            Ok(fd)
        } else {
            Err(io::Error::last_os_error())
        }
    }
}

fn fts_stat(sp: &mut FTS, p: &mut FTSENT, follow: bool) -> io::Result<u16> {
    let follow_flag = if follow { 0 } else { libc::AT_SYMLINK_NOFOLLOW };
    
    let mut stat_buf = libc::stat {
        st_dev: 0,
        st_ino: 0,
        st_mode: 0,
        st_nlink: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atime: 0,
        st_atime_nsec: 0,
        st_mtime: 0,
        st_mtime_nsec: 0,
        st_ctime: 0,
        st_ctime_nsec: 0,
        __unused: [0; 2],
    };

    let res = unsafe {
        libc::fstatat(
            sp.fts_cwd_fd,
            p.fts_accpath.as_os_str().as_bytes().as_ptr() as *const libc::c_char,
            &mut stat_buf,
            follow_flag,
        )
    };

    if res < 0 {
        let err = io::Error::last_os_error();
        p.fts_errno = err.raw_os_error().unwrap_or(0);
        Ok(10) // FTS_NS
    } else {
        let mode = stat_buf.st_mode & libc::S_IFMT;
        match mode {
            libc::S_IFDIR => {
                if p.fts_name.as_bytes().starts_with(b".")
                    && (p.fts_name.len() == 1
                        || (p.fts_name.len() == 2 && p.fts_name.as_bytes()[1] == b'.'))
                {
                    Ok(if p.fts_level == 0 { 1 } else { 5 }) // FTS_DOT or FTS_DOTDOT
                } else {
                    Ok(1) // FTS_D
                }
            }
            libc::S_IFLNK => Ok(12), // FTS_SL
            libc::S_IFREG => Ok(8),  // FTS_F
            _ => Ok(3),              // FTS_NSOK
        }
    }
}

fn fts_compare_ino(a: &&FTSENT, b: &&FTSENT) -> Ordering {
    a.fts_statp.ino().cmp(&b.fts_statp.ino())
}

fn fts_sort(sp: &mut FTS, head: Option<FTSENT>, nitems: usize) -> Option<FTSENT> {
    // TODO: Implement sorting
    head
}

fn fts_alloc(sp: &mut FTS, name: &str, namelen: usize) -> io::Result<FTSENT> {
    Ok(FTSENT {
        fts_path: sp.fts_path.clone(),
        fts_accpath: PathBuf::from(name),
        fts_name: OsString::from(name),
        fts_namelen: namelen,
        fts_level: 0,
        fts_info: 0,
        fts_errno: 0,
        fts_number: 0,
        fts_pointer: ptr::null_mut(),
        fts_parent: ptr::null_mut(),
        fts_link: None,
        fts_fts: sp,
        fts_dirp: None,
        fts_statp: Metadata::from(std::fs::metadata(".")?),
        fts_symfd: -1,
        fts_flags: 0,
        fts_instr: 3,
        fts_cycle: None,
    })
}

fn fts_lfree(head: Option<FTSENT>) {
    // TODO: Free linked list
}

fn fts_palloc(sp: &mut FTS, more: usize) -> bool {
    let new_len = sp.fts_pathlen + more + 256;
    if new_len < sp.fts_pathlen {
        return false;
    }
    sp.fts_pathlen = new_len;
    sp.fts_path.reserve(new_len);
    true
}

fn fts_padjust(sp: &mut FTS, head: Option<FTSENT>) {
    // TODO: Adjust paths
}

fn fts_safe_changedir(sp: &mut FTS, p: &mut FTSENT, fd: i32, dir: &str) -> io::Result<()> {
    // TODO: Implement safe directory change
    Ok(())
}

fn filesystem_type(p: &FTSENT, fd: i32) -> u64 {
    // TODO: Implement filesystem type detection
    0
}

fn leaf_optimization(p: &FTSENT, dir_fd: i32) -> bool {
    // TODO: Implement leaf optimization
    false
}