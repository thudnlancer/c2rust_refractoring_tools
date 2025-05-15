use libc::{c_char, c_int, c_void, size_t, off_t, pid_t, time_t};
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::ptr;
use std::fs::{File, OpenOptions, Metadata};
use std::os::unix::fs::{OpenOptionsExt, MetadataExt};
use std::io::{Error, ErrorKind, Result};
use std::mem;
use std::os::unix::io::{AsRawFd, RawFd};
use nix::fcntl::{fcntl, FcntlArg, OFlag};
use nix::sys::stat::{fstatat, stat, SFlag, Stat};
use nix::dir::{Dir, Type};
use nix::errno::Errno;
use nix::unistd::{close, fork, pipe, ForkResult};
use std::collections::VecDeque;
use std::sync::Once;

const DEFAULT_MXFAST: usize = 128;
const CHDIR_CACHE_SIZE: usize = 16;
const BILLION: i32 = 1_000_000_000;
const LOG10_BILLION: i32 = 9;

#[derive(Debug, Clone, Copy)]
pub enum BackupType {
    NoBackups,
    SimpleBackups,
    NumberedExistingBackups,
    NumberedBackups,
}

#[derive(Debug, Clone, Copy)]
pub enum RemoveOption {
    Ordinary,
    Recursive,
    WantDirectory,
}

struct Wd {
    name: CString,
    abspath: Option<PathBuf>,
    fd: Option<RawFd>,
}

struct NameBuf {
    buffer: Vec<u8>,
    dir_length: usize,
}

pub struct TarContext {
    wd: Vec<Wd>,
    wd_cache: VecDeque<usize>,
    current: usize,
    chdir_fd: Option<RawFd>,
}

impl TarContext {
    pub fn new() -> Self {
        TarContext {
            wd: vec![Wd {
                name: CString::new(".").unwrap(),
                abspath: None,
                fd: None,
            }],
            wd_cache: VecDeque::with_capacity(CHDIR_CACHE_SIZE),
            current: 0,
            chdir_fd: None,
        }
    }

    pub fn chdir_count(&self) -> usize {
        self.wd.len().saturating_sub(1)
    }

    pub fn chdir_arg(&mut self, dir: &Path) -> Result<usize> {
        let dir_cstr = CString::new(dir.as_os_str().as_bytes())?;
        
        if self.wd.len() == self.wd.capacity() {
            let new_cap = if self.wd.capacity() == 0 { 2 } else { self.wd.capacity() * 2 };
            self.wd.reserve(new_cap);
        }

        let idx = self.wd.len();
        self.wd.push(Wd {
            name: dir_cstr,
            abspath: None,
            fd: None,
        });

        Ok(idx)
    }

    pub fn chdir_do(&mut self, idx: usize) -> Result<()> {
        if self.current == idx {
            return Ok(());
        }

        let curr = &mut self.wd[idx];
        if curr.fd.is_none() {
            let flags = OFlag::O_RDONLY | OFlag::O_DIRECTORY;
            let fd = nix::fcntl::openat(
                self.chdir_fd.unwrap_or(nix::libc::AT_FDCWD),
                &curr.name,
                flags,
                nix::sys::stat::Mode::empty(),
            )?;

            curr.fd = Some(fd);

            if self.wd_cache.len() >= CHDIR_CACHE_SIZE {
                if let Some(stale_idx) = self.wd_cache.pop_back() {
                    if let Some(fd) = self.wd[stale_idx].fd.take() {
                        close(fd)?;
                    }
                }
            }
            self.wd_cache.push_front(idx);
        }

        self.current = idx;
        self.chdir_fd = curr.fd;
        Ok(())
    }

    pub fn tar_dirname(&self) -> &CString {
        &self.wd[self.current].name
    }

    fn get_cd_path(&mut self, idx: usize) -> Result<&Path> {
        if self.wd[idx].abspath.is_none() {
            let mut i = idx;
            while i > 0 && self.wd[i].abspath.is_none() {
                i -= 1;
            }

            for j in i+1..=idx {
                self.chdir_do(j)?;
                
                if j == 0 {
                    let cwd = std::env::current_dir()?;
                    self.wd[j].abspath = Some(cwd);
                } else if self.wd[j].name.as_bytes()[0] == b'/' {
                    self.wd[j].abspath = Some(PathBuf::from(self.wd[j].name.as_bytes()));
                } else {
                    let parent = self.wd[j-1].abspath.as_ref().unwrap();
                    let mut path = parent.join(self.wd[j].name.as_bytes());
                    if path.is_relative() {
                        path = parent.join(path);
                    }
                    self.wd[j].abspath = Some(path);
                }
            }
        }

        Ok(self.wd[idx].abspath.as_ref().unwrap())
    }
}

impl NameBuf {
    pub fn new(dir: &Path) -> Result<Self> {
        let dir_str = dir.as_os_str().as_bytes();
        let mut buffer = Vec::with_capacity(dir_str.len() + 2);
        buffer.extend_from_slice(dir_str);
        
        let mut dir_length = buffer.len();
        if *buffer.last().unwrap() != b'/' {
            buffer.push(b'/');
            dir_length += 1;
        }

        Ok(Self { buffer, dir_length })
    }

    pub fn name(&mut self, name: &[u8]) -> &[u8] {
        let required = self.dir_length + name.len() + 1;
        if required > self.buffer.capacity() {
            self.buffer.reserve(required - self.buffer.len());
        }

        self.buffer.truncate(self.dir_length);
        self.buffer.extend_from_slice(name);
        &self.buffer
    }

    pub fn add_dir(&mut self, name: &[u8]) {
        if *self.buffer.last().unwrap() != b'/' {
            self.name(b"/");
        }
        self.name(name);
        self.dir_length = self.buffer.len();
    }

    pub fn finish(mut self) -> Vec<u8> {
        if *self.buffer.last().unwrap() == b'/' {
            self.buffer.pop();
        }
        self.buffer
    }
}

pub fn unquote_string(s: &str) -> Result<String> {
    let mut result = Vec::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    let mut success = true;

    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(escaped) = chars.next() {
                match escaped {
                    '\\' => result.push(b'\\'),
                    'a' => result.push(0x07),
                    'b' => result.push(0x08),
                    'f' => result.push(0x0C),
                    'n' => result.push(b'\n'),
                    'r' => result.push(b'\r'),
                    't' => result.push(b'\t'),
                    'v' => result.push(0x0B),
                    '0'..='7' => {
                        let mut value = (escaped as u8 - b'0') as u32;
                        for _ in 0..2 {
                            if let Some(d) = chars.peek().filter(|&&c| c >= '0' && c <= '7') {
                                value = value * 8 + (*d as u8 - b'0') as u32;
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        result.push(value as u8);
                    }
                    _ => {
                        result.push(b'\\');
                        result.push(escaped as u8);
                        success = false;
                    }
                }
            } else {
                result.push(b'\\');
                success = false;
            }
        } else {
            result.push(c as u8);
        }
    }

    String::from_utf8(result).map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid UTF-8"))
        .and_then(|s| if success { Ok(s) } else {
            Err(Error::new(ErrorKind::InvalidData, "Invalid escape sequence"))
        })
}

pub fn zap_slashes(path: &Path) -> PathBuf {
    let mut s = path.as_os_str().as_bytes().to_vec();
    while s.last() == Some(&b'/') {
        s.pop();
    }
    PathBuf::from(OsStrExt::from_vec(s))
}

pub fn normalize_filename(path: &Path) -> PathBuf {
    let mut components = path.components();
    let mut normalized = PathBuf::new();

    for component in components {
        match component {
            std::path::Component::Prefix(_) => normalized.push(component),
            std::path::Component::RootDir => normalized.push("/"),
            std::path::Component::CurDir => {},
            std::path::Component::ParentDir => {
                if !normalized.pop() {
                    normalized.push("..");
                }
            },
            std::path::Component::Normal(c) => normalized.push(c),
        }
    }

    if normalized.as_os_str().is_empty() {
        normalized.push(".");
    }

    normalized
}

pub fn remove_any_file(path: &Path, option: RemoveOption) -> Result<bool> {
    match nix::unistd::unlinkat(
        None,
        path,
        nix::unistd::UnlinkatFlags::NoRemoveDir,
    ) {
        Ok(_) => return Ok(true),
        Err(Errno::EISDIR) => {},
        Err(e) if e != Errno::ENOENT && e != Errno::ENOTDIR => return Err(e.into()),
        _ => {},
    }

    if safer_rmdir(path)? {
        return Ok(true);
    }

    match Errno::last() {
        Errno::ENOTEMPTY => {
            if let RemoveOption::Recursive = option {
                let entries = std::fs::read_dir(path)?;
                for entry in entries {
                    let entry = entry?;
                    remove_any_file(&entry.path(), RemoveOption::Recursive)?;
                }
                return safer_rmdir(path);
            }
        },
        _ => {},
    }

    Ok(false)
}

fn safer_rmdir(path: &Path) -> Result<bool> {
    if must_be_dot_or_slash(path) {
        return Ok(false);
    }

    nix::unistd::unlinkat(
        None,
        path,
        nix::unistd::UnlinkatFlags::RemoveDir,
    ).map(|_| true)
    .or_else(|e| if e == Errno::ENOENT { Ok(false) } else { Err(e.into()) })
}

fn must_be_dot_or_slash(path: &Path) -> bool {
    let bytes = path.as_os_str().as_bytes();
    if bytes.is_empty() {
        return true;
    }

    if bytes[0] == b'/' {
        let mut i = 1;
        while i < bytes.len() {
            if bytes[i] == b'/' {
                i += 1;
            } else if bytes[i] == b'.' {
                let mut j = i + 1;
                if j < bytes.len() && bytes[j] == b'.' {
                    j += 1;
                }
                if j < bytes.len() && bytes[j] == b'/' {
                    i = j + 1;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    } else {
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'.' && i+1 < bytes.len() && bytes[i+1] == b'/' {
                i += 2;
                while i < bytes.len() && bytes[i] == b'/' {
                    i += 1;
                }
            } else {
                break;
            }
        }
        i >= bytes.len() || (bytes[i] == b'.' && i+1 >= bytes.len())
    }
}

pub fn blocking_read(fd: RawFd, buf: &mut [u8]) -> Result<usize> {
    loop {
        match nix::unistd::read(fd, buf) {
            Ok(n) => return Ok(n),
            Err(Errno::EAGAIN) => {
                let flags = fcntl(fd, FcntlArg::F_GETFL)?;
                let flags = OFlag::from_bits_truncate(flags);
                if flags.contains(OFlag::O_NONBLOCK) {
                    fcntl(fd, FcntlArg::F_SETFL(flags & !OFlag::O_NONBLOCK))?;
                } else {
                    return Err(Error::from(Errno::EAGAIN));
                }
            },
            Err(e) => return Err(e.into()),
        }
    }
}

pub fn blocking_write(fd: RawFd, buf: &[u8]) -> Result<usize> {
    let mut written = 0;
    while written < buf.len() {
        match nix::unistd::write(fd, &buf[written..]) {
            Ok(n) => written += n,
            Err(Errno::EAGAIN) => {
                let flags = fcntl(fd, FcntlArg::F_GETFL)?;
                let flags = OFlag::from_bits_truncate(flags);
                if flags.contains(OFlag::O_NONBLOCK) {
                    fcntl(fd, FcntlArg::F_SETFL(flags & !OFlag::O_NONBLOCK))?;
                } else {
                    return Err(Error::from(Errno::EAGAIN));
                }
            },
            Err(e) => return Err(e.into()),
        }
    }
    Ok(written)
}

pub fn xfork() -> Result<pid_t> {
    match fork() {
        Ok(ForkResult::Parent { child }) => Ok(child.as_raw()),
        Ok(ForkResult::Child) => Ok(0),
        Err(e) => Err(e.into()),
    }
}

pub fn xpipe() -> Result<(RawFd, RawFd)> {
    pipe().map_err(Into::into)
}

pub fn page_aligned_alloc(size: usize) -> Result<(*mut c_void, *mut c_void)> {
    let alignment = unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as usize;
    let total_size = size.checked_add(alignment).ok_or(Error::new(
        ErrorKind::Other,
        "Size overflow",
    ))?;

    let ptr = unsafe { libc::malloc(total_size) };
    if ptr.is_null() {
        return Err(Error::last_os_error());
    }

    let aligned_ptr = unsafe {
        let p = ptr as *mut u8;
        let offset = alignment - (p as usize) % alignment;
        p.add(offset) as *mut c_void
    };

    Ok((ptr, aligned_ptr))
}