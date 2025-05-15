use std::ffi::{CStr, CString};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::ptr;
use std::mem;
use std::fs::{self, File, DirEntry, Metadata};
use std::os::unix::fs::{MetadataExt, DirEntryExt};
use std::collections::HashSet;
use std::os::unix::io::{AsRawFd, RawFd};
use std::sync::Arc;
use std::cell::RefCell;
use std::rc::Rc;

const FTS_COMFOLLOW: i32 = 0x0001;
const FTS_LOGICAL: i32 = 0x0002;
const FTS_NOCHDIR: i32 = 0x0004;
const FTS_NOSTAT: i32 = 0x0008;
const FTS_PHYSICAL: i32 = 0x0010;
const FTS_SEEDOT: i32 = 0x0020;
const FTS_XDEV: i32 = 0x0040;
const FTS_WHITEOUT: i32 = 0x0080;
const FTS_OPTIONMASK: i32 = 0x00ff;
const FTS_NAMEONLY: i32 = 0x0100;
const FTS_STOP: i32 = 0x2000;

const FTS_F: u16 = 1;
const FTS_D: u16 = 2;
const FTS_DNR: u16 = 3;
const FTS_DP: u16 = 4;
const FTS_NS: u16 = 5;
const FTS_SL: u16 = 6;
const FTS_SLNONE: u16 = 7;
const FTS_DEFAULT: u16 = 8;
const FTS_DOT: u16 = 9;
const FTS_DC: u16 = 10;
const FTS_NSOK: u16 = 11;
const FTS_W: u16 = 12;
const FTS_DONTCHDIR: u16 = 0x01;
const FTS_SYMFOLLOW: u16 = 0x02;

struct FTS {
    path: PathBuf,
    options: i32,
    compar: Option<Box<dyn FnMut(&DirEntry, &DirEntry) -> std::cmp::Ordering>>,
    entries: Vec<DirEntry>,
    current: usize,
    dev: u64,
    inodes: HashSet<(u64, u64)>,
    fd_ring: Vec<RawFd>,
    cwd_fd: Option<RawFd>,
}

struct FTSENT {
    fts_info: u16,
    fts_path: PathBuf,
    fts_name: String,
    fts_level: i32,
    fts_parent: Option<Rc<RefCell<FTSENT>>>,
    fts_link: Option<Rc<RefCell<FTSENT>>>,
    fts_stat: Option<Metadata>,
    fts_errno: i32,
    fts_flags: u16,
    fts_instr: u16,
}

impl FTS {
    fn new(
        paths: &[&Path],
        options: i32,
        compar: Option<Box<dyn FnMut(&DirEntry, &DirEntry) -> std::cmp::Ordering>>,
    ) -> Result<Self, std::io::Error> {
        let mut entries = Vec::new();
        for path in paths {
            let dir = fs::read_dir(path)?;
            entries.extend(dir.filter_map(|e| e.ok()));
        }

        Ok(FTS {
            path: PathBuf::new(),
            options,
            compar,
            entries,
            current: 0,
            dev: 0,
            inodes: HashSet::new(),
            fd_ring: Vec::new(),
            cwd_fd: None,
        })
    }

    fn read(&mut self) -> Option<Rc<RefCell<FTSENT>>> {
        if self.current >= self.entries.len() {
            return None;
        }

        let entry = &self.entries[self.current];
        self.current += 1;

        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        let metadata = entry.metadata().ok();

        let mut fts_info = FTS_DEFAULT;
        if let Some(ref md) = metadata {
            if md.is_dir() {
                fts_info = FTS_D;
            } else if md.file_type().is_symlink() {
                fts_info = FTS_SL;
            }
        }

        Some(Rc::new(RefCell::new(FTSENT {
            fts_info,
            fts_path: path,
            fts_name: name,
            fts_level: 0,
            fts_parent: None,
            fts_link: None,
            fts_stat: metadata,
            fts_errno: 0,
            fts_flags: 0,
            fts_instr: 0,
        })))
    }

    fn children(&mut self, instr: i32) -> Option<Rc<RefCell<FTSENT>>> {
        if self.current == 0 || self.entries.is_empty() {
            return None;
        }

        let parent = &self.entries[self.current - 1];
        if !parent.file_type().ok()?.is_dir() {
            return None;
        }

        let dir = fs::read_dir(parent.path()).ok()?;
        let mut entries: Vec<_> = dir.filter_map(|e| e.ok()).collect();

        if let Some(ref mut compar) = self.compar {
            entries.sort_by(|a, b| compar(a, b));
        }

        let mut head = None;
        let mut tail = None;

        for entry in entries {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().into_owned();
            let metadata = entry.metadata().ok();

            let mut fts_info = FTS_DEFAULT;
            if let Some(ref md) = metadata {
                if md.is_dir() {
                    fts_info = FTS_D;
                } else if md.file_type().is_symlink() {
                    fts_info = FTS_SL;
                }
            }

            let node = Rc::new(RefCell::new(FTSENT {
                fts_info,
                fts_path: path,
                fts_name: name,
                fts_level: 1,
                fts_parent: None,
                fts_link: None,
                fts_stat: metadata,
                fts_errno: 0,
                fts_flags: 0,
                fts_instr: 0,
            }));

            if head.is_none() {
                head = Some(node.clone());
                tail = Some(node);
            } else {
                tail.as_ref().unwrap().borrow_mut().fts_link = Some(node.clone());
                tail = Some(node);
            }
        }

        head
    }

    fn close(&mut self) -> Result<(), std::io::Error> {
        for fd in self.fd_ring.drain(..) {
            unsafe {
                libc::close(fd);
            }
        }
        Ok(())
    }
}

impl Drop for FTS {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

fn fts_open(
    paths: &[&Path],
    options: i32,
    compar: Option<Box<dyn FnMut(&DirEntry, &DirEntry) -> std::cmp::Ordering>>,
) -> Result<FTS, std::io::Error> {
    FTS::new(paths, options, compar)
}

fn fts_read(fts: &mut FTS) -> Option<Rc<RefCell<FTSENT>>> {
    fts.read()
}

fn fts_children(fts: &mut FTS, instr: i32) -> Option<Rc<RefCell<FTSENT>>> {
    fts.children(instr)
}

fn fts_close(fts: &mut FTS) -> Result<(), std::io::Error> {
    fts.close()
}

fn fts_set(ftsent: &mut FTSENT, instr: i32) -> Result<(), std::io::Error> {
    ftsent.fts_instr = instr as u16;
    Ok(())
}