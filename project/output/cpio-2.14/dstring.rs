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
    fn _IO_getc(__fp: *mut _IO_FILE) -> i32;
    fn rpl_free(ptr: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn x2nrealloc(p: *mut libc::c_void, pn: *mut size_t, s: size_t) -> *mut libc::c_void;
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
pub struct dynamic_string {
    pub ds_size: size_t,
    pub ds_idx: size_t,
    pub ds_string: *mut i8,
}
#[no_mangle]
pub unsafe extern "C" fn ds_init(mut string: *mut dynamic_string) {
    memset(
        string as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<dynamic_string>() as u64,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ds_free(mut string: *mut dynamic_string) {
    rpl_free((*string).ds_string as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ds_resize(mut string: *mut dynamic_string, mut len: size_t) {
    while len.wrapping_add((*string).ds_idx) >= (*string).ds_size {
        (*string).ds_string = x2nrealloc(
            (*string).ds_string as *mut libc::c_void,
            &mut (*string).ds_size,
            1 as i32 as size_t,
        ) as *mut i8;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ds_reset(mut s: *mut dynamic_string, mut len: size_t) {
    ds_resize(s, len);
    (*s).ds_idx = len;
}
#[no_mangle]
pub unsafe extern "C" fn ds_fgetstr(
    mut f: *mut FILE,
    mut s: *mut dynamic_string,
    mut eos: i8,
) -> *mut i8 {
    let mut next_ch: i32 = 0;
    (*s).ds_idx = 0 as i32 as size_t;
    loop {
        next_ch = _IO_getc(f);
        if !(next_ch != eos as i32 && next_ch != -(1 as i32)) {
            break;
        }
        ds_resize(s, 0 as i32 as size_t);
        let fresh0 = (*s).ds_idx;
        (*s).ds_idx = ((*s).ds_idx).wrapping_add(1);
        *((*s).ds_string).offset(fresh0 as isize) = next_ch as i8;
    }
    ds_resize(s, 0 as i32 as size_t);
    *((*s).ds_string).offset((*s).ds_idx as isize) = '\0' as i32 as i8;
    if (*s).ds_idx == 0 as i32 as u64 && next_ch == -(1 as i32) {
        return 0 as *mut i8
    } else {
        return (*s).ds_string
    };
}
#[no_mangle]
pub unsafe extern "C" fn ds_append(mut s: *mut dynamic_string, mut c: i32) {
    ds_resize(s, 0 as i32 as size_t);
    *((*s).ds_string).offset((*s).ds_idx as isize) = c as i8;
    if c != 0 {
        (*s).ds_idx = ((*s).ds_idx).wrapping_add(1);
        (*s).ds_idx;
        ds_resize(s, 0 as i32 as size_t);
        *((*s).ds_string).offset((*s).ds_idx as isize) = 0 as i32 as i8;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ds_concat(mut s: *mut dynamic_string, mut str: *const i8) {
    let mut len: size_t = strlen(str);
    ds_resize(s, len);
    memcpy(
        ((*s).ds_string).offset((*s).ds_idx as isize) as *mut libc::c_void,
        str as *const libc::c_void,
        len,
    );
    (*s).ds_idx = ((*s).ds_idx as u64).wrapping_add(len) as size_t as size_t;
    *((*s).ds_string).offset((*s).ds_idx as isize) = 0 as i32 as i8;
}
#[no_mangle]
pub unsafe extern "C" fn ds_fgets(
    mut f: *mut FILE,
    mut s: *mut dynamic_string,
) -> *mut i8 {
    return ds_fgetstr(f, s, '\n' as i32 as i8);
}
#[no_mangle]
pub unsafe extern "C" fn ds_fgetname(
    mut f: *mut FILE,
    mut s: *mut dynamic_string,
) -> *mut i8 {
    return ds_fgetstr(f, s, '\0' as i32 as i8);
}
#[no_mangle]
pub unsafe extern "C" fn ds_endswith(mut s: *mut dynamic_string, mut c: i32) -> i32 {
    return ((*s).ds_idx > 0 as i32 as u64
        && *((*s).ds_string).offset(((*s).ds_idx).wrapping_sub(1 as i32 as u64) as isize)
            as i32 == c) as i32;
}