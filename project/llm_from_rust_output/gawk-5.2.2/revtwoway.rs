use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use std::sync::Once;

static mut API: *const gawk_api_t = ptr::null();
static mut EXT_ID: awk_ext_id_t = ptr::null_mut();
static EXT_VERSION: &[u8] = b"revtwoway extension: version 1.0\0";
static INIT_FUNC: Option<fn() -> awk_bool_t> = Some(init_revtwoway);
static mut MAX_FDS: size_t = 0;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct awk_field_info {
    pub skip: size_t,
    pub len: size_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct awk_fieldwidth_info_t {
    pub use_chars: awk_bool_t,
    pub nf: size_t,
    pub fields: [awk_field_info; 1],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct awk_input {
    pub name: *const c_char,
    pub fd: c_int,
    pub opaque: *mut c_void,
    pub get_record: Option<
        unsafe extern "C" fn(
            *mut *mut c_char,
            *mut awk_input,
            *mut c_int,
            *mut *mut c_char,
            *mut size_t,
            *mut *const awk_fieldwidth_info_t,
        ) -> c_int,
    >,
    pub read_func: Option<unsafe extern "C" fn(c_int, *mut c_void, size_t) -> ssize_t>,
    pub close_func: Option<unsafe extern "C" fn(*mut awk_input) -> ()>,
    pub sbuf: stat,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct awk_input_parser {
    pub name: *const c_char,
    pub can_take_file: Option<unsafe extern "C" fn(*const awk_input_buf_t) -> awk_bool_t>,
    pub take_control_of: Option<unsafe extern "C" fn(*mut awk_input_buf_t) -> awk_bool_t>,
    pub next: *const awk_input_parser,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct awk_output_buf {
    pub name: *const c_char,
    pub mode: *const c_char,
    pub fp: *mut FILE,
    pub redirected: awk_bool_t,
    pub opaque: *mut c_void,
    pub gawk_fwrite: Option<
        unsafe extern "C" fn(*const c_void, size_t, size_t, *mut FILE, *mut c_void) -> size_t,
    >,
    pub gawk_fflush: Option<unsafe extern "C" fn(*mut FILE, *mut c_void) -> c_int>,
    pub gawk_ferror: Option<unsafe extern "C" fn(*mut FILE, *mut c_void) -> c_int>,
    pub gawk_fclose: Option<unsafe extern "C" fn(*mut FILE, *mut c_void) -> c_int>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct awk_output_wrapper {
    pub name: *const c_char,
    pub can_take_file: Option<unsafe extern "C" fn(*const awk_output_buf_t) -> awk_bool_t>,
    pub take_control_of: Option<unsafe extern "C" fn(*mut awk_output_buf_t) -> awk_bool_t>,
    pub next: *const awk_output_wrapper,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct awk_two_way_processor {
    pub name: *const c_char,
    pub can_take_two_way: Option<unsafe extern "C" fn(*const c_char) -> awk_bool_t>,
    pub take_control_of: Option<
        unsafe extern "C" fn(*const c_char, *mut awk_input_buf_t, *mut awk_output_buf_t) -> awk_bool_t,
    >,
    pub next: *const awk_two_way_processor,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct two_way_proc_data {
    pub size: size_t,
    pub len: size_t,
    pub data: *mut c_char,
    pub in_use: size_t,
}

static mut TWO_WAY_PROCESSOR: awk_two_way_processor_t = awk_two_way_processor {
    name: b"revtwoway\0" as *const u8 as *const c_char,
    can_take_two_way: Some(revtwoway_can_take_two_way),
    take_control_of: Some(revtwoway_take_control_of),
    next: ptr::null(),
};

static mut FUNC_TABLE: [awk_ext_func_t; 1] = [awk_ext_func {
    name: ptr::null(),
    function: None,
    max_expected_args: 0,
    min_required_args: 0,
    suppress_lint: awk_false,
    data: ptr::null_mut(),
}];

fn close_two_proc_data(proc_data: *mut two_way_proc_data_t) {
    unsafe {
        if (*proc_data).in_use > 1 {
            (*proc_data).in_use -= 1;
            return;
        }
        ((*API).api_free.unwrap())((*proc_data).data as *mut c_void);
        ((*API).api_free.unwrap())(proc_data as *mut c_void);
    }
}

unsafe extern "C" fn rev2way_get_record(
    out: *mut *mut c_char,
    iobuf: *mut awk_input_buf_t,
    errcode: *mut c_int,
    rt_start: *mut *mut c_char,
    rt_len: *mut size_t,
    unused: *mut *const awk_fieldwidth_info_t,
) -> c_int {
    if out.is_null() || iobuf.is_null() || (*iobuf).opaque.is_null() {
        return -1;
    }

    let proc_data = (*iobuf).opaque as *mut two_way_proc_data_t;
    if (*proc_data).len == 0 {
        return 0;
    }

    *out = (*proc_data).data;
    let mut len = (*proc_data).len as c_int;
    (*proc_data).len = 0;
    *rt_len = 0;

    if *(*proc_data).data.offset((len - 1) as isize) as c_int == b'\n' as c_int {
        while *(*proc_data).data.offset((len - 1) as isize) as c_int == b'\n' as c_int {
            len -= 1;
            *rt_len += 1;
        }
        *rt_start = (*proc_data).data.offset(len as isize);
    }

    len
}

unsafe extern "C" fn rev2way_close(iobuf: *mut awk_input_buf_t) {
    if iobuf.is_null() || (*iobuf).opaque.is_null() {
        return;
    }
    let proc_data = (*iobuf).opaque as *mut two_way_proc_data_t;
    close_two_proc_data(proc_data);
    (*iobuf).fd = -1;
}

unsafe extern "C" fn rev2way_fwrite(
    buf: *const c_void,
    size: size_t,
    count: size_t,
    fp: *mut FILE,
    opaque: *mut c_void,
) -> size_t {
    if opaque.is_null() {
        return 0;
    }

    let proc_data = opaque as *mut two_way_proc_data_t;
    let amount = size * count;

    if amount > (*proc_data).size || (*proc_data).len > 0 {
        if (*proc_data).data.is_null() {
            (*proc_data).data = ((*API).api_malloc.unwrap())(amount) as *mut c_char;
            if (*proc_data).data.is_null() {
                ((*API).api_fatal.unwrap())(
                    EXT_ID,
                    b"%s: malloc of %d bytes failed\0" as *const u8 as *const c_char,
                    b"rev2way_fwrite\0" as *const u8 as *const c_char,
                    amount,
                );
            }
        } else {
            (*proc_data).data = ((*API).api_realloc.unwrap())(
                (*proc_data).data as *mut c_void,
                (*proc_data).size + amount,
            ) as *mut c_char;
            if (*proc_data).data.is_null() {
                ((*API).api_fatal.unwrap())(
                    EXT_ID,
                    b"%s: realloc of %d bytes failed\0" as *const u8 as *const c_char,
                    b"rev2way_fwrite\0" as *const u8 as *const c_char,
                    (*proc_data).size + amount,
                );
            }
        }
        (*proc_data).size += amount;
    }

    let mut src = (buf as *mut c_char).offset(amount as isize).offset(-1);
    let mut dest = (*proc_data).data.offset((*proc_data).len as isize);
    let mut char_count = amount;

    while char_count > 0 {
        *dest = *src;
        src = src.offset(-1);
        dest = dest.offset(1);
        char_count -= 1;
    }

    (*proc_data).len += amount;
    amount
}

unsafe extern "C" fn rev2way_fflush(fp: *mut FILE, opaque: *mut c_void) -> c_int {
    0
}

unsafe extern "C" fn rev2way_ferror(fp: *mut FILE, opaque: *mut c_void) -> c_int {
    0
}

unsafe extern "C" fn rev2way_fclose(fp: *mut FILE, opaque: *mut c_void) -> c_int {
    if opaque.is_null() {
        return -1;
    }
    let proc_data = opaque as *mut two_way_proc_data_t;
    close_two_proc_data(proc_data);
    0
}

unsafe extern "C" fn revtwoway_can_take_two_way(name: *const c_char) -> awk_bool_t {
    if name.is_null() {
        return awk_false;
    }
    let name_str = CStr::from_ptr(name).to_bytes();
    if name_str == b"/magic/mirror\0" {
        awk_true
    } else {
        awk_false
    }
}

unsafe extern "C" fn revtwoway_take_control_of(
    name: *const c_char,
    inbuf: *mut awk_input_buf_t,
    outbuf: *mut awk_output_buf_t,
) -> awk_bool_t {
    if inbuf.is_null() || outbuf.is_null() {
        return awk_false;
    }

    let proc_data = ((*API).api_malloc.unwrap())(std::mem::size_of::<two_way_proc_data_t>()) as *mut two_way_proc_data_t;
    if proc_data.is_null() {
        ((*API).api_fatal.unwrap())(
            EXT_ID,
            b"%s: malloc of %d bytes failed\0" as *const u8 as *const c_char,
            b"revtwoway_take_control_of\0" as *const u8 as *const c_char,
            std::mem::size_of::<two_way_proc_data_t>(),
        );
    }

    (*proc_data).in_use = 2;
    (*proc_data).size = 0;
    (*proc_data).len = 0;
    (*proc_data).data = ptr::null_mut();

    if MAX_FDS + 1 == 0 {
        MAX_FDS = unsafe { libc::getdtablesize() as size_t };
    }

    (*inbuf).get_record = Some(rev2way_get_record);
    (*inbuf).close_func = Some(rev2way_close);
    (*inbuf).fd = MAX_FDS as c_int;
    (*inbuf).opaque = proc_data as *mut c_void;

    (*outbuf).fp = MAX_FDS as *mut FILE;
    (*outbuf).opaque = proc_data as *mut c_void;
    (*outbuf).gawk_fwrite = Some(rev2way_fwrite);
    (*outbuf).gawk_fflush = Some(rev2way_fflush);
    (*outbuf).gawk_ferror = Some(rev2way_ferror);
    (*outbuf).gawk_fclose = Some(rev2way_fclose);
    (*outbuf).redirected = awk_true;

    awk_true
}

unsafe extern "C" fn init_revtwoway() -> awk_bool_t {
    ((*API).api_register_two_way_processor.unwrap())(EXT_ID, &mut TWO_WAY_PROCESSOR);
    MAX_FDS = libc::getdtablesize() as size_t;
    awk_true
}

#[no_mangle]
pub unsafe extern "C" fn dl_load(api_p: *const gawk_api_t, id: awk_ext_id_t) -> c_int {
    API = api_p;
    EXT_ID = id;

    if (*API).major_version != GAWK_API_MAJOR_VERSION as c_int
        || (*API).minor_version < GAWK_API_MINOR_VERSION as c_int
    {
        eprintln!("revtwoway: version mismatch with gawk!");
        eprintln!(
            "\tmy version (API {}.{}), gawk version (API {}.{})",
            GAWK_API_MAJOR_VERSION,
            GAWK_API_MINOR_VERSION,
            (*API).major_version,
            (*API).minor_version
        );
        std::process::exit(1);
    }

    let mut errors = 0;

    for func in FUNC_TABLE.iter_mut() {
        if func.name.is_null() {
            break;
        }
        if ((*API).api_add_ext_func.unwrap())(EXT_ID, b"\0" as *const u8 as *const c_char, func) == 0 {
            ((*API).api_warning.unwrap())(
                EXT_ID,
                b"revtwoway: could not add %s\0" as *const u8 as *const c_char,
                func.name,
            );
            errors += 1;
        }
    }

    if let Some(init) = INIT_FUNC {
        if init() == 0 {
            ((*API).api_warning.unwrap())(
                EXT_ID,
                b"revtwoway: initialization function failed\0" as *const u8 as *const c_char,
            );
            errors += 1;
        }
    }

    ((*API).api_register_ext_version.unwrap())(EXT_ID, EXT_VERSION.as_ptr() as *const c_char);

    if errors == 0 { 1 } else { 0 }
}