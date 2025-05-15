/*  Functions from hack's utils library.
    Copyright (C) 1989-2022 Free Software Foundation, Inc.

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 3, or (at your option)
    any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; If not, see <https://www.gnu.org/licenses/>. */

use std::{
    ffi::CString,
    fs::{File, OpenOptions},
    io::{self, Read, Write, Seek, SeekFrom, ErrorKind},
    path::{Path, PathBuf},
    os::unix::ffi::OsStrExt,
    os::fd::{AsRawFd, FromRawFd},
    sync::Mutex,
    collections::LinkedList,
    fmt,
    error::Error,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCode {
    Success = 0,
    BadUsage = 1,
    BadInput = 2,
    Panic = 4,
}

#[derive(Debug)]
pub struct UtilsError {
    message: String,
}

impl fmt::Display for UtilsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for UtilsError {}

impl UtilsError {
    pub fn new(msg: &str) -> Self {
        UtilsError {
            message: msg.to_string(),
        }
    }
}

pub type Result<T> = std::result::Result<T, UtilsError>;

lazy_static::lazy_static! {
    static ref OPEN_FILES: Mutex<LinkedList<OpenFile>> = Mutex::new(LinkedList::new());
    static ref FILE_TO_UNLINK: Mutex<Option<PathBuf>> = Mutex::new(None);
}

struct OpenFile {
    file: File,
    name: String,
}

pub fn panic(msg: &str) -> ! {
    eprintln!("{}: {}", std::env::args().next().unwrap_or_default(), msg);
    std::process::exit(ExitCode::Panic as i32)
}

fn register_open_file(file: File, name: &str) {
    let mut open_files = OPEN_FILES.lock().unwrap();
    open_files.push_front(OpenFile {
        file,
        name: name.to_string(),
    });
}

fn get_file_name(file: &File) -> &str {
    let open_files = OPEN_FILES.lock().unwrap();
    for open_file in open_files.iter() {
        if &open_file.file as *const _ == file as *const _ {
            return &open_file.name;
        }
    }

    if file as *const _ == io::stdin() as *const _ {
        "stdin"
    } else if file as *const _ == io::stdout() as *const _ {
        "stdout"
    } else if file as *const _ == io::stderr() as *const _ {
        "stderr"
    } else {
        "<unknown>"
    }
}

pub fn ck_fopen(name: &str, mode: &str, fail: bool) -> Result<File> {
    let file = OpenOptions::new()
        .read(mode.contains('r'))
        .write(mode.contains('w') || mode.contains('a'))
        .append(mode.contains('a'))
        .truncate(mode.contains('w'))
        .create(mode.contains('w') || mode.contains('a'))
        .open(name)
        .map_err(|e| {
            if fail {
                panic(&format!("couldn't open file {}: {}", name, e));
            }
            UtilsError::new(&format!("couldn't open file {}: {}", name, e))
        })?;

    register_open_file(file.try_clone()?, name);
    Ok(file)
}

pub fn ck_fdopen(fd: i32, name: &str, mode: &str, fail: bool) -> Result<File> {
    unsafe {
        let file = File::from_raw_fd(fd);
        let file = OpenOptions::new()
            .read(mode.contains('r'))
            .write(mode.contains('w') || mode.contains('a'))
            .append(mode.contains('a'))
            .open(name)
            .map_err(|e| {
                if fail {
                    panic(&format!("couldn't attach to {}: {}", name, e));
                }
                UtilsError::new(&format!("couldn't attach to {}: {}", name, e))
            })?;

        register_open_file(file.try_clone()?, name);
        Ok(file)
    }
}

pub fn remove_cleanup_file() -> Result<()> {
    let mut file_to_unlink = FILE_TO_UNLINK.lock().unwrap();
    if let Some(path) = file_to_unlink.take() {
        std::fs::remove_file(path).map_err(|e| {
            UtilsError::new(&format!("couldn't remove cleanup file: {}", e))
        })?;
    }
    Ok(())
}

fn register_cleanup_file(file: &str) {
    let mut file_to_unlink = FILE_TO_UNLINK.lock().unwrap();
    *file_to_unlink = Some(PathBuf::from(file));
}

pub fn cancel_cleanup() {
    let mut file_to_unlink = FILE_TO_UNLINK.lock().unwrap();
    *file_to_unlink = None;
}

pub fn ck_mkstemp(
    p_filename: &mut String,
    tmpdir: &str,
    base: &str,
    mode: &str,
) -> Result<File> {
    let template = format!("{}/{}.XXXXXX", tmpdir, base);
    let template_c = CString::new(template.clone()).unwrap();
    let mut template_bytes = template_c.into_bytes_with_nul();
    let fd = unsafe {
        libc::mkostemp(template_bytes.as_mut_ptr() as *mut libc::c_char, 0)
    };

    if fd == -1 {
        panic(&format!(
            "couldn't open temporary file {}: {}",
            template,
            io::Error::last_os_error()
        ));
    }

    *p_filename = template.clone();
    register_cleanup_file(&template);

    let file = unsafe { File::from_raw_fd(fd) };
    let file = OpenOptions::new()
        .read(mode.contains('r'))
        .write(mode.contains('w') || mode.contains('a'))
        .append(mode.contains('a'))
        .open(&template)
        .map_err(|e| {
            UtilsError::new(&format!(
                "couldn't open temporary file {}: {}",
                template, e
            ))
        })?;

    register_open_file(file.try_clone()?, &template);
    Ok(file)
}

pub fn ck_fwrite(ptr: &[u8], file: &mut File) -> Result<()> {
    file.write_all(ptr).map_err(|e| {
        UtilsError::new(&format!(
            "couldn't write to {}: {}",
            get_file_name(file),
            e
        ))
    })
}

pub fn ck_fread(ptr: &mut [u8], file: &mut File) -> Result<usize> {
    file.read(ptr).map_err(|e| {
        UtilsError::new(&format!(
            "read error on {}: {}",
            get_file_name(file),
            e
        ))
    })
}

pub fn ck_fflush(file: &mut File) -> Result<()> {
    file.flush().map_err(|e| {
        UtilsError::new(&format!(
            "couldn't flush {}: {}",
            get_file_name(file),
            e
        ))
    })
}

pub fn ck_fclose(file: Option<&mut File>) -> Result<()> {
    let mut open_files = OPEN_FILES.lock().unwrap();
    let mut prev = &mut *open_files;

    while !prev.is_empty() {
        if file.is_none() || prev.front().unwrap().file.as_raw_fd() == file.unwrap().as_raw_fd() {
            let open_file = prev.pop_front().unwrap();
            do_ck_fclose(open_file.file, &open_file.name)?;
        } else {
            prev = &mut prev.make_contiguous()[1..].to_vec().into_iter().collect();
        }
    }

    if file.is_none() {
        do_ck_fclose(io::stdout(), "stdout")?;
    }

    Ok(())
}

fn do_ck_fclose(mut file: File, name: &str) -> Result<()> {
    ck_fflush(&mut file)?;
    drop(file);
    Ok(())
}

pub fn follow_symlink(fname: &str) -> Result<String> {
    let mut path = PathBuf::from(fname);
    let mut num_links = 0;

    while let Ok(link) = std::fs::read_link(&path) {
        if num_links >= 40 {
            return Err(UtilsError::new(&format!(
                "couldn't follow symlink {}: too many levels of symbolic links",
                fname
            )));
        }

        if link.is_absolute() {
            path = link;
        } else {
            path = path.parent().unwrap().join(link);
        }

        num_links += 1;
    }

    Ok(path.to_string_lossy().into_owned())
}

pub fn ck_rename(from: &str, to: &str) -> Result<()> {
    std::fs::rename(from, to).map_err(|e| {
        UtilsError::new(&format!("cannot rename {}: {}", from, e))
    })
}

pub struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            data: Vec::with_capacity(50),
        }
    }

    pub fn get(&self) -> &[u8] {
        &self.data
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn add(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    pub fn add1(&mut self, byte: u8) {
        self.data.push(byte);
    }
}

pub fn init_buffer() -> Buffer {
    Buffer::new()
}

pub fn get_buffer(buffer: &Buffer) -> &[u8] {
    buffer.get()
}

pub fn size_buffer(buffer: &Buffer) -> usize {
    buffer.size()
}

pub fn add_buffer(buffer: &mut Buffer, bytes: &[u8]) {
    buffer.add(bytes);
}

pub fn add1_buffer(buffer: &mut Buffer, byte: u8) {
    buffer.add1(byte);
}

pub fn free_buffer(_buffer: Buffer) {
    // Automatically dropped when out of scope
}