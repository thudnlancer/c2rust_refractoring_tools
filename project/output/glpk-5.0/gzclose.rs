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
    fn _glp_zlib_gzclose_r(file: gzFile) -> i32;
    fn _glp_zlib_gzclose_w(file: gzFile) -> i32;
}
pub type Byte = u8;
pub type uInt = u32;
pub type uLong = u64;
pub type Bytef = Byte;
pub type voidpf = *mut libc::c_void;
pub type voidp = *mut libc::c_void;
pub type alloc_func = Option<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct internal_state {
    pub dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut i8,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: i32,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type gzFile = voidp;
pub type gz_statep = *mut gz_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gz_state {
    pub mode: i32,
    pub fd: i32,
    pub path: *mut i8,
    pub pos: i64,
    pub size: u32,
    pub want: u32,
    pub in_0: *mut u8,
    pub out: *mut u8,
    pub next: *mut u8,
    pub have: u32,
    pub eof: i32,
    pub start: i64,
    pub raw: i64,
    pub how: i32,
    pub direct: i32,
    pub level: i32,
    pub strategy: i32,
    pub skip: i64,
    pub seek: i32,
    pub err: i32,
    pub msg: *mut i8,
    pub strm: z_stream,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gzclose(mut file: gzFile) -> i32 {
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return -(2 as i32);
    }
    state = file as gz_statep;
    return if (*state).mode == 7247 as i32 {
        _glp_zlib_gzclose_r(file)
    } else {
        _glp_zlib_gzclose_w(file)
    };
}