use std::{
    ffi::{CStr, CString},
    fs::File,
    io::{Read, Write},
    mem,
    os::raw::{c_char, c_int, c_long, c_ulong, c_void},
    path::Path,
    ptr,
};

// Constants and types from original C code
const _ISalnum: c_uint = 8;
const _ISpunct: c_uint = 4;
const _IScntrl: c_uint = 2;
const _ISblank: c_uint = 1;
const _ISgraph: c_uint = 32768;
const _ISprint: c_uint = 16384;
const _ISspace: c_uint = 8192;
const _ISxdigit: c_uint = 4096;
const _ISdigit: c_uint = 2048;
const _ISalpha: c_uint = 1024;
const _ISlower: c_uint = 512;
const _ISupper: c_uint = 256;

type size_t = c_ulong;
type uint8_t = u8;
type uint16_t = u16;
type uint32_t = u32;
type uint64_t = u64;
type int32_t = i32;
type int64_t = i64;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct Header64 {
    e_type: uint16_t,
    e_machine: uint16_t,
    e_version: uint32_t,
    e_entry: uint64_t,
    e_phoff: uint64_t,
    e_shoff: uint64_t,
    e_flags: uint32_t,
    e_ehsize: uint16_t,
    e_phentsize: uint16_t,
    e_phnum: uint16_t,
    e_shentsize: uint16_t,
    e_shnum: uint16_t,
    e_shstrndx: uint16_t,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct Header32 {
    e_type: uint16_t,
    e_machine: uint16_t,
    e_version: uint32_t,
    e_entry: uint32_t,
    e_phoff: uint32_t,
    e_shoff: uint32_t,
    e_flags: uint32_t,
    e_ehsize: uint16_t,
    e_phentsize: uint16_t,
    e_phnum: uint16_t,
    e_shentsize: uint16_t,
    e_shnum: uint16_t,
    e_shstrndx: uint16_t,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct Prog64 {
    p_type: uint32_t,
    p_flags: uint32_t,
    p_offset: uint64_t,
    p_vaddr: uint64_t,
    p_paddr: uint64_t,
    p_filesz: uint64_t,
    p_memsz: uint64_t,
    p_align: uint64_t,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct Prog32 {
    p_type: uint32_t,
    p_offset: uint32_t,
    p_vaddr: uint32_t,
    p_paddr: uint32_t,
    p_filesz: uint32_t,
    p_memsz: uint32_t,
    p_flags: uint32_t,
    p_align: uint32_t,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct Dyn64 {
    d_tag: int64_t,
    d_val: uint64_t,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct Dyn32 {
    d_tag: int32_t,
    d_val: uint32_t,
}

#[derive(Debug, Clone, Copy)]
struct Compat {
    any: c_char,
    class: uint8_t,
    machine: uint16_t,
}

#[derive(Debug, Clone, Copy)]
struct Found {
    how: How,
    depth: size_t,
}

#[derive(Debug, Clone, Copy)]
enum How {
    Input = 0,
    Direct = 1,
    Rpath = 2,
    LdLibraryPath = 3,
    Runpath = 4,
    LdSoConf = 5,
    Default = 6,
}

#[derive(Debug)]
struct StringTable {
    arr: Vec<c_char>,
    n: size_t,
    capacity: size_t,
}

impl StringTable {
    fn new() -> Self {
        Self {
            arr: Vec::new(),
            n: 0,
            capacity: 1024,
        }
    }

    fn maybe_grow(&mut self, n: size_t) {
        if self.n + n <= self.capacity {
            return;
        }
        self.capacity = (2 * (self.n + n)).max(1024);
        self.arr.resize(self.capacity as usize, 0);
    }

    fn store(&mut self, s: &[c_char]) {
        let n = s.len() + 1;
        self.maybe_grow(n as size_t);
        unsafe {
            ptr::copy_nonoverlapping(
                s.as_ptr(),
                self.arr.as_mut_ptr().add(self.n as usize),
                n,
            );
        }
        self.n += n as size_t;
    }

    fn copy_from_file(&mut self, file: &mut File) {
        let mut buf = [0u8; 1];
        while file.read_exact(&mut buf).is_ok() {
            let c = buf[0] as c_char;
            if c == 0 || c == -1 {
                break;
            }
            self.maybe_grow(1);
            unsafe {
                *self.arr.as_mut_ptr().add(self.n as usize) = c;
            }
            self.n += 1;
        }
        self.maybe_grow(1);
        unsafe {
            *self.arr.as_mut_ptr().add(self.n as usize) = 0;
        }
        self.n += 1;
    }
}

#[derive(Debug)]
struct VisitedFile {
    st_dev: u64,
    st_ino: u64,
}

#[derive(Debug)]
struct VisitedFileArray {
    arr: Vec<VisitedFile>,
    n: size_t,
    capacity: size_t,
}

impl VisitedFileArray {
    fn new() -> Self {
        Self {
            arr: Vec::new(),
            n: 0,
            capacity: 256,
        }
    }

    fn contains(&self, needle: &VisitedFile) -> bool {
        self.arr[..self.n as usize]
            .iter()
            .any(|f| f.st_dev == needle.st_dev && f.st_ino == needle.st_ino)
    }

    fn append(&mut self, new: VisitedFile) {
        if self.n == self.capacity {
            self.capacity *= 2;
            self.arr.resize(self.capacity as usize, VisitedFile { st_dev: 0, st_ino: 0 });
        }
        self.arr[self.n as usize] = new;
        self.n += 1;
    }
}

#[derive(Debug)]
struct LibtreeState {
    verbosity: c_int,
    path: c_int,
    color: c_int,
    ld_conf_file: CString,
    max_depth: c_ulong,
    string_table: StringTable,
    visited: VisitedFileArray,
    platform: CString,
    lib: CString,
    osname: CString,
    osrel: CString,
    rpath_offsets: [size_t; 32],
    ld_library_path_offset: size_t,
    default_paths_offset: size_t,
    ld_so_conf_offset: size_t,
    found_all_needed: [c_char; 32],
}

impl LibtreeState {
    fn new() -> Self {
        let mut uname_val = libc::utsname {
            sysname: [0; 65],
            nodename: [0; 65],
            release: [0; 65],
            version: [0; 65],
            machine: [0; 65],
            __domainname: [0; 65],
        };
        unsafe {
            libc::uname(&mut uname_val);
        }

        let platform = unsafe { CStr::from_ptr(uname_val.machine.as_ptr()) };
        let osname = unsafe { CStr::from_ptr(uname_val.sysname.as_ptr()) };
        let osrel = unsafe { CStr::from_ptr(uname_val.release.as_ptr()) };

        let ld_conf_file = if osname.to_bytes() == b"FreeBSD" {
            CString::new("/etc/ld-elf.so.conf").unwrap()
        } else {
            CString::new("/etc/ld.so.conf").unwrap()
        };

        Self {
            verbosity: 0,
            path: 0,
            color: unsafe {
                (libc::getenv(CString::new("NO_COLOR").unwrap().as_ptr()).is_null()
                    && libc::isatty(1) != 0
            } as c_int,
            ld_conf_file,
            max_depth: 32,
            string_table: StringTable::new(),
            visited: VisitedFileArray::new(),
            platform: platform.to_owned(),
            lib: CString::new("lib").unwrap(),
            osname: osname.to_owned(),
            osrel: osrel.to_owned(),
            rpath_offsets: [0; 32],
            ld_library_path_offset: 0,
            default_paths_offset: 0,
            ld_so_conf_offset: 0,
            found_all_needed: [0; 32],
        }
    }
}

fn main() {
    let mut state = LibtreeState::new();
    // Rest of the implementation would go here...
}