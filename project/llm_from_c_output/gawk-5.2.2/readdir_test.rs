use std::ffi::{CStr, CString};
use std::fs::{File, Metadata};
use std::io;
use std::os::unix::fs::FileTypeExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use libc::{dirfd, DT_BLK, DT_CHR, DT_DIR, DT_FIFO, DT_LNK, DT_REG, DT_SOCK, DT_UNKNOWN};
use nix::dir::{Dir, Type};
use nix::errno::Errno;
use nix::fcntl::OFlag;
use nix::sys::stat::{fstat, FileStat, SFlag};

use gawkapi::{awk_bool_t, awk_ext_id_t, awk_fieldwidth_info_t, awk_input_buf_t, awk_input_parser_t};
use gawkapi::{emalloc, gawk_free, register_input_parser, update_ERRNO_int, warning};

static mut API: *const gawkapi::gawk_api_t = std::ptr::null();
static mut EXT_ID: awk_ext_id_t = 0;
static EXT_VERSION: &str = "readdir extension: version 3.0";

static mut PLUGIN_IS_GPL_COMPATIBLE: bool = true;

struct OpenDirectory {
    dir: Dir,
    buf: Vec<u8>,
    fw: awk_fieldwidth_info_t,
}

impl OpenDirectory {
    fn new(dir: Dir) -> Self {
        let size = std::mem::size_of::<libc::dirent>() + 21 + 2;
        let mut fw = awk_fieldwidth_info_t {
            use_chars: awk_bool_t::FALSE,
            nf: 3,
            fields: [
                gawkapi::awk_field_info_t { skip: 0, len: 0 },
                gawkapi::awk_field_info_t { skip: 1, len: 0 },
                gawkapi::awk_field_info_t { skip: 1, len: 0 },
            ],
        };
        
        OpenDirectory {
            dir,
            buf: vec![0; size],
            fw,
        }
    }
}

fn ftype(entry: &libc::dirent, dirname: &Path) -> &'static str {
    match entry.d_type {
        DT_BLK => "b",
        DT_CHR => "c",
        DT_DIR => "d",
        DT_FIFO => "p",
        DT_LNK => "l",
        DT_REG => "f",
        DT_SOCK => "s",
        DT_UNKNOWN => {
            let mut fname = dirname.to_path_buf();
            fname.push(unsafe { CStr::from_ptr(entry.d_name.as_ptr()).to_str().unwrap() });
            
            if let Ok(metadata) = std::fs::metadata(&fname) {
                if metadata.file_type().is_block_device() {
                    "b"
                } else if metadata.file_type().is_char_device() {
                    "c"
                } else if metadata.file_type().is_dir() {
                    "d"
                } else if metadata.file_type().is_fifo() {
                    "p"
                } else if metadata.file_type().is_file() {
                    "f"
                } else if metadata.file_type().is_symlink() {
                    "l"
                } else if metadata.file_type().is_socket() {
                    "s"
                } else {
                    "u"
                }
            } else {
                "u"
            }
        }
        _ => "u",
    }
}

fn get_inode(entry: &libc::dirent, _dirname: &Path) -> u64 {
    entry.d_ino as u64
}

fn dir_get_record(
    out: &mut *mut libc::c_char,
    iobuf: &mut awk_input_buf_t,
    errcode: &mut libc::c_int,
    rt_start: &mut *mut libc::c_char,
    rt_len: &mut libc::size_t,
    field_width: &mut *const awk_fieldwidth_info_t,
) -> libc::c_int {
    if out.is_null() || iobuf.opaque.is_null() {
        return libc::EOF;
    }

    let the_dir = unsafe { &mut *(iobuf.opaque as *mut OpenDirectory) };
    
    match the_dir.dir.iter().next() {
        Some(Ok(entry)) => {
            let ino = get_inode(&entry, Path::new(unsafe { CStr::from_ptr(iobuf.name).to_str().unwrap() }));
            let name = unsafe { CStr::from_ptr(entry.d_name.as_ptr()).to_str().unwrap() };
            let ftstr = ftype(&entry, Path::new(unsafe { CStr::from_ptr(iobuf.name).to_str().unwrap() }));
            
            let len = unsafe {
                libc::sprintf(
                    the_dir.buf.as_mut_ptr() as *mut libc::c_char,
                    b"%llu/%s/%s\0".as_ptr() as *const libc::c_char,
                    ino,
                    name,
                    ftstr,
                )
            };
            
            the_dir.fw.fields[0].len = len as usize;
            the_dir.fw.fields[1].len = name.len();
            the_dir.fw.fields[2].len = ftstr.len();
            
            *out = the_dir.buf.as_mut_ptr() as *mut libc::c_char;
            *rt_start = std::ptr::null_mut();
            *rt_len = 0;
            
            if !field_width.is_null() {
                *field_width = &the_dir.fw;
            }
            
            len
        }
        Some(Err(e)) => {
            *errcode = e as libc::c_int;
            libc::EOF
        }
        None => {
            *errcode = 0;
            libc::EOF
        }
    }
}

fn dir_close(iobuf: &mut awk_input_buf_t) {
    if iobuf.opaque.is_null() {
        return;
    }

    let the_dir = unsafe { Box::from_raw(iobuf.opaque as *mut OpenDirectory) };
    iobuf.opaque = std::ptr::null_mut();
    iobuf.fd = -1;
}

fn dir_can_take_file(iobuf: &awk_input_buf_t) -> awk_bool_t {
    if iobuf.fd != -1 || (iobuf.sbuf.st_mode & libc::S_IFMT) == libc::S_IFDIR {
        awk_bool_t::TRUE
    } else {
        awk_bool_t::FALSE
    }
}

fn dir_take_control_of(iobuf: &mut awk_input_buf_t) -> awk_bool_t {
    let dp = unsafe {
        if iobuf.fd != -1 {
            Dir::from_fd(iobuf.fd).ok()
        } else {
            Dir::open(Path::new(unsafe { CStr::from_ptr(iobuf.name).to_str().unwrap() }), OFlag::O_RDONLY, nix::sys::stat::Mode::empty()).ok()
        }
    };
    
    match dp {
        Some(dir) => {
            let the_dir = Box::new(OpenDirectory::new(dir));
            iobuf.opaque = Box::into_raw(the_dir) as *mut libc::c_void;
            iobuf.get_record = Some(dir_get_record);
            iobuf.close_func = Some(dir_close);
            awk_bool_t::TRUE
        }
        None => {
            let err = Errno::last();
            warning(unsafe { EXT_ID }, &format!("dir_take_control_of: opendir/fdopendir failed: {}", err));
            update_ERRNO_int(err as i32);
            awk_bool_t::FALSE
        }
    }
}

static READDIR_PARSER: awk_input_parser_t = awk_input_parser_t {
    name: b"readdir\0".as_ptr() as *const libc::c_char,
    can_take_file: Some(dir_can_take_file),
    take_control_of: Some(dir_take_control_of),
    parser_flags: 0,
};

fn init_readdir() -> awk_bool_t {
    unsafe {
        register_input_parser(&READDIR_PARSER);
    }
    awk_bool_t::TRUE
}

#[no_mangle]
pub extern "C" fn dl_load(api: *const gawkapi::gawk_api_t, ext_id: awk_ext_id_t) -> *mut gawkapi::awk_ext_func_t {
    unsafe {
        API = api;
        EXT_ID = ext_id;
        
        if init_readdir() == awk_bool_t::FALSE {
            return std::ptr::null_mut();
        }
        
        static FUNC_TABLE: [awk_ext_func_t; 1] = [awk_ext_func_t {
            name: std::ptr::null(),
            func: None,
            num_expected_args: 0,
            min_required_args: 0,
            suppress_lint: awk_bool_t::FALSE,
            next: std::ptr::null(),
        }];
        
        FUNC_TABLE.as_ptr() as *mut awk_ext_func_t
    }
}