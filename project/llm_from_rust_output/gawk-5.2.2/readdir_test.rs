use std::ffi::{CStr, CString};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::fs::{read_dir, DirEntry, FileType};
use std::os::unix::fs::FileTypeExt;
use libc::{c_int, c_longlong, c_ulonglong};
use std::ptr;

#[repr(C)]
pub struct awk_input_buf_t {
    name: *const libc::c_char,
    fd: c_int,
    opaque: *mut libc::c_void,
    // ... other fields omitted for brevity
}

#[repr(C)]
pub struct awk_fieldwidth_info_t {
    use_chars: bool,
    nf: usize,
    fields: [awk_field_info; 1],
}

#[repr(C)]
pub struct awk_field_info {
    skip: usize,
    len: usize,
}

#[repr(C)]
pub struct open_directory {
    dp: *mut libc::DIR,
    buf: *mut libc::c_char,
    u: DirectoryUnion,
}

#[repr(C)]
pub union DirectoryUnion {
    fw: awk_fieldwidth_info_t,
    buf: [libc::c_char; 64],
}

pub struct DirectoryReader {
    dir: std::fs::ReadDir,
    path: PathBuf,
    buffer: Vec<u8>,
    field_info: awk_fieldwidth_info_t,
}

impl DirectoryReader {
    pub fn new(path: &Path) -> Result<Self, std::io::Error> {
        let dir = read_dir(path)?;
        Ok(Self {
            dir,
            path: path.to_path_buf(),
            buffer: Vec::with_capacity(4096),
            field_info: awk_fieldwidth_info_t {
                use_chars: false,
                nf: 3,
                fields: [awk_field_info { skip: 0, len: 0 }],
            },
        })
    }

    pub fn get_record(&mut self) -> Option<(&[u8], &awk_fieldwidth_info_t)> {
        let entry = self.dir.next()?.ok()?;
        let file_name = entry.file_name();
        let file_name_bytes = file_name.as_bytes();
        
        self.buffer.clear();
        
        // Get inode
        let ino = entry.ino().unwrap_or(0) as c_ulonglong;
        let ino_str = format!("{}", ino);
        self.buffer.extend_from_slice(ino_str.as_bytes());
        self.field_info.fields[0].len = ino_str.len();
        
        // Add filename
        self.buffer.push(b'/');
        self.buffer.extend_from_slice(file_name_bytes);
        self.field_info.fields[1].len = file_name_bytes.len();
        
        // Add file type
        let ft_str = self.get_file_type(&entry);
        self.buffer.push(b'/');
        self.buffer.extend_from_slice(ft_str.as_bytes());
        self.field_info.fields[2].len = ft_str.len();
        
        Some((&self.buffer, &self.field_info))
    }

    fn get_file_type(&self, entry: &DirEntry) -> &'static str {
        if let Ok(file_type) = entry.file_type() {
            if file_type.is_block_device() {
                "b"
            } else if file_type.is_char_device() {
                "c"
            } else if file_type.is_dir() {
                "d"
            } else if file_type.is_fifo() {
                "p"
            } else if file_type.is_file() {
                "f"
            } else if file_type.is_symlink() {
                "l"
            } else if file_type.is_socket() {
                "s"
            } else {
                "u"
            }
        } else {
            "u"
        }
    }
}

pub fn dir_can_take_file(iobuf: &awk_input_buf_t) -> bool {
    iobuf.fd != -1 || {
        let path = unsafe { CStr::from_ptr(iobuf.name) };
        if let Ok(metadata) = std::fs::metadata(Path::new(path.to_str().unwrap())) {
            metadata.is_dir()
        } else {
            false
        }
    }
}

pub fn dir_take_control_of(iobuf: &mut awk_input_buf_t) -> bool {
    let path = unsafe { CStr::from_ptr(iobuf.name) };
    let path_str = path.to_str().unwrap();
    
    match DirectoryReader::new(Path::new(path_str)) {
        Ok(reader) => {
            let boxed = Box::new(reader);
            iobuf.opaque = Box::into_raw(boxed) as *mut libc::c_void;
            true
        }
        Err(e) => {
            // Handle error
            false
        }
    }
}

pub fn dir_close(iobuf: &mut awk_input_buf_t) {
    if !iobuf.opaque.is_null() {
        unsafe {
            Box::from_raw(iobuf.opaque as *mut DirectoryReader);
        }
        iobuf.opaque = ptr::null_mut();
    }
    iobuf.fd = -1;
}

pub struct ReadDirPlugin {
    parser: awk_input_parser_t,
}

impl ReadDirPlugin {
    pub fn new() -> Self {
        Self {
            parser: awk_input_parser_t {
                name: CString::new("readdir").unwrap().into_raw(),
                can_take_file: Some(dir_can_take_file),
                take_control_of: Some(dir_take_control_of),
                next: ptr::null(),
            },
        }
    }
    
    pub fn init(&self) -> bool {
        // Register parser with gawk
        true
    }
}

#[no_mangle]
pub extern "C" fn dl_load(api_p: *const gawk_api_t, id: awk_ext_id_t) -> c_int {
    let plugin = ReadDirPlugin::new();
    if plugin.init() {
        1
    } else {
        0
    }
}