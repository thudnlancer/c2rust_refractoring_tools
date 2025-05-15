use std::ffi::{CStr, CString, OsStr, OsString};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom, ErrorKind};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::ptr;
use std::sync::Mutex;
use libc::{self, c_char, c_int, c_void, size_t, ssize_t, mode_t};
use nix::errno::Errno;
use nix::fcntl::OFlag;
use nix::sys::stat::Mode;
use nix::unistd;
use once_cell::sync::Lazy;

static OPEN_FILES: Lazy<Mutex<Vec<OpenFile>>> = Lazy::new(|| Mutex::new(Vec::new()));
static FILE_TO_UNLINK: Lazy<Mutex<Option<PathBuf>>> = Lazy::new(|| Mutex::new(None));

#[derive(Debug)]
struct OpenFile {
    file: File,
    name: PathBuf,
}

#[derive(Debug)]
pub struct Buffer {
    data: Vec<u8>,
}

#[derive(Debug, Clone, Copy)]
pub enum ExitCode {
    BadUsage = 1,
    BadInput = 2,
    Panic = 4,
}

pub fn panic(msg: &str) -> ! {
    eprintln!("{}: {}", program_name(), msg);
    std::process::exit(ExitCode::Panic as i32);
}

pub fn program_name() -> &'static str {
    unsafe {
        CStr::from_ptr(libc::program_name)
            .to_str()
            .unwrap_or("unknown")
    }
}

pub fn ck_fopen(name: &Path, mode: &str, fail: bool) -> io::Result<File> {
    let file = OpenOptions::new()
        .read(mode.contains('r'))
        .write(mode.contains('w'))
        .create(mode.contains('+'))
        .truncate(mode.contains('w'))
        .append(mode.contains('a'))
        .open(name)?;

    register_open_file(&file, name);
    Ok(file)
}

fn register_open_file(file: &File, name: &Path) {
    OPEN_FILES.lock().unwrap().push(OpenFile {
        file: file.try_clone().unwrap(),
        name: name.to_path_buf(),
    });
}

pub fn remove_cleanup_file() {
    if let Some(path) = FILE_TO_UNLINK.lock().unwrap().take() {
        let _ = std::fs::remove_file(path);
    }
}

pub fn cancel_cleanup() {
    FILE_TO_UNLINK.lock().unwrap().take();
}

pub fn ck_mkstemp(
    tmpdir: &Path,
    base: &str,
    mode: &str,
) -> io::Result<(File, PathBuf)> {
    let template = tmpdir.join(format!("{}XXXXXX", base));
    let template_c = CString::new(template.as_os_str().as_bytes()).unwrap();
    
    let mut template_bytes = template_c.into_bytes_with_nul();
    let fd = unsafe {
        libc::mkostemp(template_bytes.as_mut_ptr() as *mut c_char, 0)
    };
    
    if fd == -1 {
        return Err(io::Error::last_os_error());
    }

    let path = PathBuf::from(OsString::from_vec(template_bytes[..template_bytes.len()-1].to_vec()));
    *FILE_TO_UNLINK.lock().unwrap() = Some(path.clone());

    let file = unsafe { File::from_raw_fd(fd) };
    register_open_file(&file, &path);
    
    Ok((file, path))
}

pub fn ck_fwrite(data: &[u8], file: &mut File) -> io::Result<()> {
    file.write_all(data)?;
    Ok(())
}

pub fn ck_fread(buf: &mut [u8], file: &mut File) -> io::Result<usize> {
    file.read(buf)
}

pub fn ck_fflush(file: &mut File) -> io::Result<()> {
    file.flush()
}

pub fn ck_fclose(file: File) -> io::Result<()> {
    let mut open_files = OPEN_FILES.lock().unwrap();
    if let Some(pos) = open_files.iter().position(|f| f.file.as_raw_fd() == file.as_raw_fd()) {
        open_files.remove(pos);
    }
    drop(file); // File closes when dropped
    Ok(())
}

pub fn follow_symlink(path: &Path) -> io::Result<PathBuf> {
    let mut buf = Vec::new();
    let mut current_path = path.to_path_buf();
    let mut num_links = 0;
    let max_links = 40; // Same as __eloop_threshold()

    loop {
        buf.resize(1024, 0);
        let len = match unistd::readlink(&current_path, &mut buf) {
            Ok(len) => len,
            Err(Errno::EINVAL) => break, // Not a symlink
            Err(e) => return Err(e.into()),
        };

        if num_links >= max_links {
            return Err(io::Error::new(ErrorKind::Other, "Too many symbolic links"));
        }

        buf.truncate(len);
        let link_path = PathBuf::from(OsString::from_vec(buf));

        if link_path.is_absolute() {
            current_path = link_path;
        } else {
            current_path = current_path.parent().unwrap().join(link_path);
        }

        num_links += 1;
    }

    Ok(current_path)
}

pub fn ck_rename(from: &Path, to: &Path) -> io::Result<()> {
    std::fs::rename(from, to)
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