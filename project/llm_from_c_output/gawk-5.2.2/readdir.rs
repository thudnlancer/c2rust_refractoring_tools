use std::ffi::{CStr, CString};
use std::fs::{DirEntry, FileType, Metadata, ReadDir};
use std::io;
use std::os::unix::fs::FileTypeExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use gawkapi::{awk_bool_t, awk_ext_func_t, awk_ext_id_t, awk_fieldwidth_info_t, awk_input_buf_t, awk_input_parser_t, gawk_api_t, gawk_free, register_input_parser, update_ERRNO_int, warning};

static mut API: *const gawk_api_t = std::ptr::null();
static mut EXT_ID: awk_ext_id_t = 0;
static EXT_VERSION: &str = "readdir extension: version 2.0";

static mut INIT_FUNC: Option<fn() -> awk_bool_t> = Some(init_readdir);
static PLUGIN_IS_GPL_COMPATIBLE: i32 = 1;

struct OpenDirectory {
    read_dir: ReadDir,
    buf: String,
}

impl OpenDirectory {
    fn new(read_dir: ReadDir) -> Self {
        Self {
            read_dir,
            buf: String::with_capacity(1024),
        }
    }
}

fn ftype(entry: &DirEntry, dirname: &Path) -> &'static str {
    if let Ok(file_type) = entry.file_type() {
        if file_type.is_block_device() {
            return "b";
        }
        if file_type.is_char_device() {
            return "c";
        }
        if file_type.is_dir() {
            return "d";
        }
        if file_type.is_fifo() {
            return "p";
        }
        if file_type.is_file() {
            return "f";
        }
        if file_type.is_symlink() {
            return "l";
        }
        if file_type.is_socket() {
            return "s";
        }
    }

    // Fallback to stat if needed
    if let Ok(metadata) = entry.metadata() {
        if metadata.file_type().is_block_device() {
            return "b";
        }
        if metadata.file_type().is_char_device() {
            return "c";
        }
        if metadata.file_type().is_dir() {
            return "d";
        }
        if metadata.file_type().is_fifo() {
            return "p";
        }
        if metadata.file_type().is_file() {
            return "f";
        }
        if metadata.file_type().is_symlink() {
            return "l";
        }
        if metadata.file_type().is_socket() {
            return "s";
        }
    }

    "u"
}

fn get_inode(entry: &DirEntry, dirname: &Path) -> u64 {
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        if let Ok(metadata) = entry.metadata() {
            let inode_high = metadata.file_index_high();
            let inode_low = metadata.file_index_low();
            ((inode_high as u64) << 32) | (inode_low as u64)
        } else {
            0
        }
    }
    #[cfg(not(windows))]
    {
        use std::os::unix::fs::MetadataExt;
        if let Ok(metadata) = entry.metadata() {
            metadata.ino()
        } else {
            0
        }
    }
}

fn dir_get_record(
    out: *mut *mut libc::c_char,
    iobuf: *mut awk_input_buf_t,
    errcode: *mut libc::c_int,
    rt_start: *mut *mut libc::c_char,
    rt_len: *mut libc::size_t,
    _unused: *mut *const awk_fieldwidth_info_t,
) -> libc::c_int {
    unsafe {
        if out.is_null() || iobuf.is_null() || (*iobuf).opaque.is_null() {
            return libc::EOF;
        }

        let the_dir = &mut *((*iobuf).opaque as *mut OpenDirectory);
        let dirname = Path::new(CStr::from_ptr((*iobuf).name).to_str().unwrap());

        match the_dir.read_dir.next() {
            Some(Ok(entry)) => {
                let ino = get_inode(&entry, dirname);
                let ftstr = ftype(&entry, dirname);

                the_dir.buf.clear();
                the_dir.buf.push_str(&format!("{}/{}/{}", ino, entry.file_name().to_string_lossy(), ftstr));

                *out = the_dir.buf.as_ptr() as *mut libc::c_char;

                *rt_start = std::ptr::null_mut();
                *rt_len = 0;
                the_dir.buf.len() as libc::c_int
            }
            Some(Err(e)) => {
                *errcode = e.raw_os_error().unwrap_or(0);
                libc::EOF
            }
            None => {
                *errcode = 0;
                libc::EOF
            }
        }
    }
}

fn dir_close(iobuf: *mut awk_input_buf_t) {
    unsafe {
        if iobuf.is_null() || (*iobuf).opaque.is_null() {
            return;
        }

        let the_dir = Box::from_raw((*iobuf).opaque as *mut OpenDirectory);
        (*iobuf).fd = -1;
    }
}

fn dir_can_take_file(iobuf: *const awk_input_buf_t) -> awk_bool_t {
    unsafe {
        if iobuf.is_null() {
            return awk_bool_t::awk_false;
        }

        let sbuf = (*iobuf).sbuf;
        if (*iobuf).fd != -1 || (sbuf.st_mode & libc::S_IFMT) == libc::S_IFDIR {
            awk_bool_t::awk_true
        } else {
            awk_bool_t::awk_false
        }
    }
}

fn dir_take_control_of(iobuf: *mut awk_input_buf_t) -> awk_bool_t {
    unsafe {
        let dirname = CStr::from_ptr((*iobuf).name).to_str().unwrap();
        let path = Path::new(dirname);

        match std::fs::read_dir(path) {
            Ok(read_dir) => {
                let the_dir = Box::new(OpenDirectory::new(read_dir));
                (*iobuf).opaque = Box::into_raw(the_dir) as *mut libc::c_void;
                (*iobuf).get_record = Some(dir_get_record);
                (*iobuf).close_func = Some(dir_close);
                awk_bool_t::awk_true
            }
            Err(e) => {
                warning(
                    EXT_ID,
                    &format!("dir_take_control_of: opendir failed: {}", e),
                );
                update_ERRNO_int(e.raw_os_error().unwrap_or(0));
                awk_bool_t::awk_false
            }
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
        awk_bool_t::awk_true
    }
}

static FUNC_TABLE: [awk_ext_func_t; 1] = [awk_ext_func_t {
    name: std::ptr::null(),
    func: None,
    num_expected_args: 0,
    min_required_args: 0,
    suppress_lint: awk_bool_t::awk_false,
    next: std::ptr::null(),
}];

#[no_mangle]
pub extern "C" fn dl_load(
    api_ptr: *const gawk_api_t,
    ext_id: awk_ext_id_t,
) -> *const awk_ext_func_t {
    unsafe {
        API = api_ptr;
        EXT_ID = ext_id;
        if let Some(init) = INIT_FUNC {
            init();
        }
        FUNC_TABLE.as_ptr()
    }
}