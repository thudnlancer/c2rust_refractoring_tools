use std::fs::{File, OpenOptions};
use std::io::{Read, Error, ErrorKind};
use std::path::Path;
use std::os::unix::fs::FileTypeExt;
use std::ffi::CStr;
use libc::{stat, S_IFREG, O_RDONLY, EINVAL};
use gawkapi::{GawkApi, AwkExtId, AwkBool, AwkValue, AwkInputBuf, AwkInputParser, AwkExtFunc};

const EXT_VERSION: &str = "readfile extension: version 2.0";

static mut API: Option<&'static GawkApi> = None;
static mut EXT_ID: Option<AwkExtId> = None;

struct ReadFileParser;

impl ReadFileParser {
    fn can_take_file(&self, iobuf: &AwkInputBuf) -> AwkBool {
        if iobuf.is_null() {
            return AwkBool::False;
        }

        let array = match unsafe { API.unwrap().sym_lookup("PROCINFO") } {
            Some(val) => val,
            None => return AwkBool::False,
        };

        let index = unsafe { API.unwrap().make_const_string("readfile") };
        match unsafe { API.unwrap().get_array_element(array.array_cookie, &index) } {
            Some(_) => AwkBool::True,
            None => AwkBool::False,
        }
    }

    fn take_control_of(&self, iobuf: &mut AwkInputBuf) -> AwkBool {
        if iobuf.is_null() {
            return AwkBool::False;
        }

        iobuf.get_record = Some(Self::get_record);
        AwkBool::True
    }

    fn get_record(
        out: &mut *mut u8,
        iobuf: &mut AwkInputBuf,
        errcode: &mut i32,
        rt_start: &mut *mut u8,
        rt_len: &mut usize,
        unused: *const (),
    ) -> i32 {
        if out.is_null() || iobuf.is_null() {
            return -1;
        }

        if !iobuf.opaque.is_null() {
            unsafe { API.unwrap().gawk_free(iobuf.opaque) };
            iobuf.opaque = std::ptr::null_mut();
            return -1;
        }

        let text = match Self::read_file_to_buffer(iobuf.fd, &iobuf.sbuf) {
            Ok(t) => t,
            Err(_) => return -1,
        };

        iobuf.opaque = text.as_ptr() as *mut _;
        *rt_start = std::ptr::null_mut();
        *rt_len = 0;
        *out = text.as_ptr() as *mut _;

        iobuf.sbuf.st_size as i32
    }

    fn read_file_to_buffer(fd: i32, sbuf: &stat) -> Result<Vec<u8>, Error> {
        if (sbuf.st_mode & S_IFREG) != S_IFREG {
            return Err(Error::new(ErrorKind::InvalidInput, "Not a regular file"));
        }

        let mut file = unsafe { File::from_raw_fd(fd) };
        let mut buffer = vec![0; sbuf.st_size as usize];
        file.read_exact(&mut buffer)?;
        Ok(buffer)
    }
}

fn do_readfile(nargs: i32, result: &mut AwkValue, _unused: *const ()) -> *mut AwkValue {
    assert!(!result.is_null());
    unsafe { API.unwrap().make_null_string(result) };

    unsafe { API.unwrap().unset_ERRNO() };

    let filename = match unsafe { API.unwrap().get_argument(0) } {
        Some(val) => val,
        None => {
            if unsafe { API.unwrap().do_lint() } {
                unsafe { API.unwrap().lintwarn(EXT_ID.unwrap(), "readfile: called with wrong kind of argument") };
            }
            return result;
        }
    };

    let path = unsafe { CStr::from_ptr(filename.str_value.str) }.to_str().unwrap();
    let sbuf = match std::fs::metadata(path) {
        Ok(m) => m,
        Err(e) => {
            unsafe { API.unwrap().update_ERRNO_int(e.raw_os_error().unwrap()) };
            return result;
        }
    };

    let file = match OpenOptions::new().read(true).open(path) {
        Ok(f) => f,
        Err(e) => {
            unsafe { API.unwrap().update_ERRNO_int(e.raw_os_error().unwrap()) };
            return result;
        }
    };

    let text = match ReadFileParser::read_file_to_buffer(file.as_raw_fd(), &sbuf) {
        Ok(t) => t,
        Err(e) => {
            unsafe { API.unwrap().update_ERRNO_int(e.raw_os_error().unwrap()) };
            return result;
        }
    };

    unsafe { API.unwrap().make_malloced_string(text.as_ptr() as *mut _, text.len(), result) };
    result
}

fn init_readfile() -> AwkBool {
    let parser = AwkInputParser {
        name: "readfile",
        can_take_file: ReadFileParser::can_take_file,
        take_control_of: ReadFileParser::take_control_of,
        next: std::ptr::null_mut(),
    };

    unsafe { API.unwrap().register_input_parser(&parser) };
    AwkBool::True
}

#[no_mangle]
pub extern "C" fn dl_load(api: &'static GawkApi, ext_id: AwkExtId) -> i32 {
    unsafe {
        API = Some(api);
        EXT_ID = Some(ext_id);
    }

    let func_table = [
        AwkExtFunc {
            name: "readfile",
            func: do_readfile,
            num_expected_args: 1,
            min_required_args: 1,
            suppress_lint: AwkBool::False,
            next: std::ptr::null_mut(),
        },
    ];

    unsafe { api.register_extension(func_table.as_ptr(), EXT_VERSION.as_ptr()) };
    0
}