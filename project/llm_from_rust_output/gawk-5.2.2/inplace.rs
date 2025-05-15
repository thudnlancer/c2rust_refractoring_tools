use std::ffi::{CStr, CString};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::os::unix::fs::{PermissionsExt, MetadataExt, chown, chmod};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::path::{Path, PathBuf};
use std::ptr;
use libc::{self, c_int, c_char, c_void, size_t, mode_t, uid_t, gid_t};
use nix::unistd::{dup, dup2, close};
use nix::fcntl::{open, OFlag};
use nix::sys::stat::{self, FileStat};

static mut API: *const gawk_api_t = ptr::null();
static mut EXT_ID: awk_ext_id_t = ptr::null_mut();
static EXT_VERSION: &str = "inplace extension: version 1.0";

#[derive(Debug)]
struct State {
    temp_name: Option<PathBuf>,
    default_stdout: Option<File>,
    pos: Option<u64>,
}

impl State {
    fn new() -> Self {
        State {
            temp_name: None,
            default_stdout: None,
            pos: None,
        }
    }
}

static mut STATE: State = State::new();

fn at_exit(_data: *mut c_void, _exit_status: c_int) {
    unsafe {
        if let Some(ref temp_path) = STATE.temp_name {
            let _ = std::fs::remove_file(temp_path);
        }
        STATE.temp_name = None;
    }
}

fn invalid_filename(filename: &str) -> bool {
    filename.is_empty() || filename == "-"
}

fn do_inplace_begin(
    nargs: c_int,
    result: &mut awk_value_t,
    _unused: &mut awk_ext_func_t,
) -> Result<*mut awk_value_t, String> {
    if nargs != 2 {
        return Err(format!("inplace::begin: expects 2 arguments but called with {}", nargs));
    }

    let filename = unsafe {
        let mut filename_val = awk_value_t::default();
        if API.api_get_argument(EXT_ID, 0, AWK_STRING, &mut filename_val) == 0 {
            return Err("inplace::begin: cannot retrieve 1st argument as string".to_string());
        }
        CStr::from_ptr(filename_val.u.s.str_0).to_string_lossy().into_owned()
    };

    if invalid_filename(&filename) {
        unsafe {
            API.api_warning(
                EXT_ID,
                format!("inplace::begin: disabling in-place editing for invalid FILENAME '{}'", filename).as_ptr(),
            );
            API.api_unset_ERRNO(EXT_ID);
        }
        result.val_type = AWK_NUMBER;
        result.u.n.d = -1.0;
        result.u.n.type_0 = AWK_NUMBER_TYPE_DOUBLE;
        return Ok(result);
    }

    let metadata = match std::fs::metadata(&filename) {
        Ok(m) => m,
        Err(e) => {
            unsafe {
                API.api_warning(
                    EXT_ID,
                    format!("inplace::begin: Cannot stat '{}' ({})", filename, e).as_ptr(),
                );
                API.api_update_ERRNO_int(EXT_ID, e.raw_os_error().unwrap_or(0));
            }
            result.val_type = AWK_NUMBER;
            result.u.n.d = -1.0;
            result.u.n.type_0 = AWK_NUMBER_TYPE_DOUBLE;
            return Ok(result);
        }
    };

    if !metadata.is_file() {
        unsafe {
            API.api_warning(
                EXT_ID,
                format!("inplace::begin: '{}' is not a regular file", filename).as_ptr(),
            );
            API.api_unset_ERRNO(EXT_ID);
        }
        result.val_type = AWK_NUMBER;
        result.u.n.d = -1.0;
        result.u.n.type_0 = AWK_NUMBER_TYPE_DOUBLE;
        return Ok(result);
    }

    let temp_path = Path::new(&filename).with_extension("gawk.XXXXXX");
    let temp_file = match tempfile::NamedTempFile::new_in(temp_path.parent().unwrap()) {
        Ok(f) => f,
        Err(e) => {
            return Err(format!("inplace::begin: mkstemp failed: {}", e));
        }
    };

    if let Err(e) = chown(temp_file.path(), Some(metadata.uid() as uid_t), Some(metadata.gid() as gid_t)) {
        let _ = chown(temp_file.path(), None, Some(metadata.gid() as gid_t));
    }

    if let Err(e) = std::fs::set_permissions(temp_file.path(), std::fs::Permissions::from_mode(metadata.mode())) {
        return Err(format!("inplace::begin: chmod failed: {}", e));
    }

    unsafe {
        io::stdout().flush().unwrap();
        STATE.pos = Some(io::stdout().seek(SeekFrom::Current(0)).unwrap());
        STATE.default_stdout = Some(File::from_raw_fd(dup(1).unwrap()));
        
        dup2(temp_file.as_raw_fd(), 1).unwrap();
        close(temp_file.as_raw_fd()).unwrap();
        
        STATE.temp_name = Some(temp_file.path().to_path_buf());
        temp_file.keep().unwrap();
    }

    result.val_type = AWK_NUMBER;
    result.u.n.d = 0.0;
    result.u.n.type_0 = AWK_NUMBER_TYPE_DOUBLE;
    Ok(result)
}

// Similar safe implementations for do_inplace_end and other functions...

static FUNC_TABLE: [awk_ext_func_t; 2] = [
    awk_ext_func_t {
        name: b"begin\0".as_ptr() as *const c_char,
        function: Some(do_inplace_begin),
        max_expected_args: 2,
        min_required_args: 2,
        suppress_lint: awk_false,
        data: ptr::null_mut(),
    },
    awk_ext_func_t {
        name: b"end\0".as_ptr() as *const c_char,
        function: Some(do_inplace_end),
        max_expected_args: 2,
        min_required_args: 2,
        suppress_lint: awk_false,
        data: ptr::null_mut(),
    },
];

fn init_inplace() -> awk_bool_t {
    unsafe {
        API.api_awk_atexit(EXT_ID, Some(at_exit), ptr::null_mut());
    }
    awk_true
}

#[no_mangle]
pub extern "C" fn dl_load(api_p: *const gawk_api_t, id: awk_ext_id_t) -> c_int {
    unsafe {
        API = api_p;
        EXT_ID = id;
        
        if (*API).major_version != GAWK_API_MAJOR_VERSION as c_int || 
           (*API).minor_version < GAWK_API_MINOR_VERSION as c_int {
            eprintln!("inplace: version mismatch with gawk!");
            eprintln!("\tmy version (API {}.{}), gawk version (API {}.{})",
                GAWK_API_MAJOR_VERSION, GAWK_API_MINOR_VERSION,
                (*API).major_version, (*API).minor_version);
            return 0;
        }

        let mut errors = 0;
        
        for func in &FUNC_TABLE {
            if API.api_add_ext_func(EXT_ID, b"inplace\0".as_ptr() as *const c_char, func) == 0 {
                API.api_warning(EXT_ID, format!("inplace: could not add {}", 
                    CStr::from_ptr(func.name).to_string_lossy()).as_ptr());
                errors += 1;
            }
        }

        if init_inplace() == 0 {
            API.api_warning(EXT_ID, b"inplace: initialization function failed\0".as_ptr() as *const c_char);
            errors += 1;
        }

        API.api_register_ext_version(EXT_ID, EXT_VERSION.as_ptr() as *const c_char);
        
        (errors == 0) as c_int
    }
}