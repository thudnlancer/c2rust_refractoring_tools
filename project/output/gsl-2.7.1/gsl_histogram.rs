#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    static mut stdin: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    fn printf(_: *const i8, _: ...) -> i32;
    fn fscanf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn strtod(__nptr: *const i8, __endptr: *mut *mut i8) -> libc::c_double;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn exit(_: i32) -> !;
    fn gsl_histogram_alloc(n: size_t) -> *mut gsl_histogram;
    fn gsl_histogram_free(h: *mut gsl_histogram);
    fn gsl_histogram_increment(h: *mut gsl_histogram, x: libc::c_double) -> i32;
    fn gsl_histogram_set_ranges_uniform(
        h: *mut gsl_histogram,
        xmin: libc::c_double,
        xmax: libc::c_double,
    ) -> i32;
    fn gsl_histogram_fprintf(
        stream: *mut FILE,
        h: *const gsl_histogram,
        range_format: *const i8,
        bin_format: *const i8,
    ) -> i32;
}
pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_histogram {
    pub n: size_t,
    pub range: *mut libc::c_double,
    pub bin: *mut libc::c_double,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut i8, 10 as i32) as i32;
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const i8) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut i8);
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut a: libc::c_double = 0.0f64;
    let mut b: libc::c_double = 1.0f64;
    let mut n: size_t = 10 as i32 as size_t;
    if argc != 3 as i32 && argc != 4 as i32 {
        printf(b"Usage: gsl-histogram xmin xmax [n]\n\0" as *const u8 as *const i8);
        printf(
            b"Computes a histogram of the data on stdin using n bins from xmin to xmax.\nIf n is unspecified then bins of integer width are used.\n\0"
                as *const u8 as *const i8,
        );
        exit(0 as i32);
    }
    a = atof(*argv.offset(1 as i32 as isize));
    b = atof(*argv.offset(2 as i32 as isize));
    if argc == 4 as i32 {
        n = atoi(*argv.offset(3 as i32 as isize)) as size_t;
    } else {
        n = (b - a) as i32 as size_t;
    }
    let mut x: libc::c_double = 0.;
    let mut h: *mut gsl_histogram = gsl_histogram_alloc(n);
    gsl_histogram_set_ranges_uniform(h, a, b);
    while fscanf(
        stdin,
        b"%lg\0" as *const u8 as *const i8,
        &mut x as *mut libc::c_double,
    ) == 1 as i32
    {
        gsl_histogram_increment(h, x);
    }
    gsl_histogram_fprintf(
        stdout,
        h,
        b"%g\0" as *const u8 as *const i8,
        b"%g\0" as *const u8 as *const i8,
    );
    gsl_histogram_free(h);
    return 0 as i32;
}
pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32,
        )
    }
}