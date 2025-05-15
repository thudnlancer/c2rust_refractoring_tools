use ::libc;
extern "C" {
    fn rpl_free(ptr: *mut libc::c_void);
    fn rpmatch(__response: *const libc::c_char) -> libc::c_int;
    static mut stdin: *mut _IO_FILE;
    fn __getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
pub type ssize_t = __ssize_t;
pub type __ssize_t = libc::c_long;
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
pub type size_t = libc::c_ulong;
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut libc::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
#[no_mangle]
pub unsafe extern "C" fn yesno() -> bool {
    let mut yes: bool = false;
    let mut response: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut response_size: size_t = 0 as libc::c_int as size_t;
    let mut response_len: ssize_t = getline(&mut response, &mut response_size, stdin);
    if response_len <= 0 as libc::c_int as libc::c_long {
        yes = 0 as libc::c_int != 0;
    } else {
        if *response.offset((response_len - 1 as libc::c_int as libc::c_long) as isize)
            as libc::c_int == '\n' as i32
        {
            *response
                .offset(
                    (response_len - 1 as libc::c_int as libc::c_long) as isize,
                ) = '\0' as i32 as libc::c_char;
        }
        yes = (0 as libc::c_int) < rpmatch(response);
    }
    rpl_free(response as *mut libc::c_void);
    return yes;
}
