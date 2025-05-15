use std::ffi::{CStr, CString};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::FromRawFd;
use std::path::{Path, PathBuf};
use std::ptr;
use libc::{c_char, c_int, c_void, size_t, off_t, FILE};
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::close;

struct Exclude;
struct Wordsplit;
struct TarStatInfo;
struct Exclist;
struct Excfile;

static mut EXCFILE_HEAD: *mut Excfile = ptr::null_mut();
static mut EXCFILE_TAIL: *mut Excfile = ptr::null_mut();

fn excfile_add(name: &str, flags: i32) {
    unsafe {
        let p = Box::into_raw(Box::new(Excfile {
            next: ptr::null_mut(),
            flags,
            name: CString::new(name).unwrap().into_raw(),
        }));

        if !EXCFILE_TAIL.is_null() {
            (*EXCFILE_TAIL).next = p;
        } else {
            EXCFILE_HEAD = p;
        }
        EXCFILE_TAIL = p;
    }
}

fn info_attach_exclist(dir: &mut TarStatInfo) {
    unsafe {
        if !dir.exclude_list.is_null() {
            return;
        }

        let mut file = EXCFILE_HEAD;
        while !file.is_null() {
            let name = CStr::from_ptr((*file).name).to_str().unwrap();
            let path = Path::new(name);
            
            if path.exists() {
                let fd = match open(path, OFlag::O_RDONLY, Mode::empty()) {
                    Ok(fd) => fd,
                    Err(_) => {
                        open_error((*file).name);
                        continue;
                    }
                };

                let fp = File::from_raw_fd(fd);
                let reader = BufReader::new(fp);
                
                let mut ex = Box::new(Exclude::new());
                let vcsfile = get_vcs_ignore_file(name);

                if let Some(initfn) = vcsfile.initfn {
                    vcsfile.data = initfn(vcsfile.data);
                }

                for line in reader.lines() {
                    if let Ok(pattern) = line {
                        (vcsfile.addfn)(&mut ex, &pattern, 0, vcsfile.data);
                    }
                }

                let ent = Box::into_raw(Box::new(Exclist {
                    excluded: Box::into_raw(ex),
                    flags: if (*file).flags == 0 { (*file).flags } else { vcsfile.flags },
                    prev: ptr::null_mut(),
                    next: ptr::null_mut(),
                }));

                if !dir.exclude_list.is_null() {
                    (*dir.exclude_list).next = ent;
                } else {
                    dir.exclude_list = ent;
                }
            }
            file = (*file).next;
        }
    }
}

fn info_free_exclist(dir: &mut TarStatInfo) {
    unsafe {
        let mut ep = dir.exclude_list;
        while !ep.is_null() {
            let next = (*ep).next;
            Box::from_raw((*ep).excluded);
            Box::from_raw(ep);
            ep = next;
        }
        dir.exclude_list = ptr::null_mut();
    }
}

fn excluded_name(name: &str, st: Option<&TarStatInfo>) -> bool {
    unsafe {
        if excluded_file_name(EXCLUDED, name) {
            return true;
        }

        let mut st = st;
        let mut result = false;
        let mut nr = 0;

        while let Some(dir) = st {
            let mut ep = dir.exclude_list;
            while !ep.is_null() {
                if (*ep).flags & nr == 0 {
                    result = excluded_file_name((*ep).excluded, name);
                    if result {
                        break;
                    }
                    
                    // Handle relative and base names
                    let rname = name.trim_start_matches("./");
                    result = excluded_file_name((*ep).excluded, rname);
                    if result {
                        break;
                    }

                    let bname = Path::new(name).file_name().unwrap().to_str().unwrap();
                    result = excluded_file_name((*ep).excluded, bname);
                    if result {
                        break;
                    }
                }
                ep = (*ep).next;
            }
            st = dir.parent.as_ref();
            nr = 0x2;
        }
        result
    }
}

// Helper functions would need to be implemented
fn excluded_file_name(ex: *mut Exclude, name: &str) -> bool {
    // Implementation depends on exclude internals
    false
}

fn get_vcs_ignore_file(name: &str) -> &'static mut VcsIgnoreFile {
    unsafe {
        for file in VCS_IGNORE_FILES.iter_mut() {
            if file.filename.is_null() {
                continue;
            }
            let fname = CStr::from_ptr(file.filename).to_str().unwrap();
            if fname == name {
                return file;
            }
        }
        &mut VCS_IGNORE_FILES.last_mut().unwrap()
    }
}

static mut VCS_IGNORE_FILES: [VcsIgnoreFile; 5] = [
    VcsIgnoreFile {
        filename: b".cvsignore\0".as_ptr() as *const c_char,
        flags: 0x2,
        addfn: Some(cvs_addfn),
        initfn: None,
        data: ptr::null_mut(),
    },
    // Other VCS ignore files...
];

struct VcsIgnoreFile {
    filename: *const c_char,
    flags: c_int,
    addfn: Option<fn(*mut Exclude, *const c_char, c_int, *mut c_void)>,
    initfn: Option<fn(*mut c_void) -> *mut c_void>,
    data: *mut c_void,
}

fn exclude_vcs_ignores() {
    unsafe {
        for file in VCS_IGNORE_FILES.iter() {
            if !file.filename.is_null() {
                excfile_add(CStr::from_ptr(file.filename).to_str().unwrap(), 0);
            }
        }
    }
}