use ::libc;
extern "C" {
    fn fwrite(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;
    fn __errno_location() -> *mut libc::c_int;
    fn rpl_free(ptr: *mut libc::c_void);
    fn fseterr(fp: *mut FILE);
    fn vasnprintf(
        resultbuf: *mut libc::c_char,
        lengthp: *mut size_t,
        format: *const libc::c_char,
        args: ::core::ffi::VaList,
    ) -> *mut libc::c_char;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
#[no_mangle]
pub unsafe extern "C" fn rpl_vfprintf(
    mut fp: *mut FILE,
    mut format: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> libc::c_int {
    let mut buf: [libc::c_char; 2000] = [0; 2000];
    let mut output: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut lenbuf: size_t = ::core::mem::size_of::<[libc::c_char; 2000]>()
        as libc::c_ulong;
    output = vasnprintf(buf.as_mut_ptr(), &mut lenbuf, format, args.as_va_list());
    len = lenbuf;
    if output.is_null() {
        fseterr(fp);
        return -(1 as libc::c_int);
    }
    if fwrite(output as *const libc::c_void, 1 as libc::c_int as size_t, len, fp) < len {
        if output != buf.as_mut_ptr() {
            rpl_free(output as *mut libc::c_void);
        }
        return -(1 as libc::c_int);
    }
    if output != buf.as_mut_ptr() {
        rpl_free(output as *mut libc::c_void);
    }
    if len > 2147483647 as libc::c_int as libc::c_ulong {
        *__errno_location() = 75 as libc::c_int;
        fseterr(fp);
        return -(1 as libc::c_int);
    }
    return len as libc::c_int;
}
