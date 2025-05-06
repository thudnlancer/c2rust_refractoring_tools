#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(linkage)]
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn wcwidth(__c: wchar_t) -> i32;
    fn iswcntrl(__wc: wint_t) -> i32;
}
pub type size_t = u64;
pub type wchar_t = i32;
pub type wint_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbchar {
    pub ptr: *const i8,
    pub bytes: size_t,
    pub wc_valid: bool,
    pub wc: wchar_t,
    pub buf: [i8; 24],
}
pub type mbchar_t = mbchar;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn is_basic(mut c: i8) -> bool {
    return *is_basic_table.as_ptr().offset((c as u8 as i32 >> 5 as i32) as isize)
        >> (c as u8 as i32 & 31 as i32) & 1 as i32 as u32 != 0;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mb_copy(
    mut new_mbc: *mut mbchar_t,
    mut old_mbc: *const mbchar_t,
) {
    if (*old_mbc).ptr
        == &*((*old_mbc).buf).as_ptr().offset(0 as i32 as isize) as *const i8
    {
        memcpy(
            &mut *((*new_mbc).buf).as_mut_ptr().offset(0 as i32 as isize) as *mut i8
                as *mut libc::c_void,
            &*((*old_mbc).buf).as_ptr().offset(0 as i32 as isize) as *const i8
                as *const libc::c_void,
            (*old_mbc).bytes,
        );
        (*new_mbc).ptr = &mut *((*new_mbc).buf).as_mut_ptr().offset(0 as i32 as isize)
            as *mut i8;
    } else {
        (*new_mbc).ptr = (*old_mbc).ptr;
    }
    (*new_mbc).bytes = (*old_mbc).bytes;
    (*new_mbc).wc_valid = (*old_mbc).wc_valid;
    if (*new_mbc).wc_valid {
        (*new_mbc).wc = (*old_mbc).wc;
    }
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mb_width_aux(mut wc: wint_t) -> i32 {
    let mut w: i32 = wcwidth(wc as wchar_t);
    return if w >= 0 as i32 {
        w
    } else if iswcntrl(wc) != 0 {
        0 as i32
    } else {
        1 as i32
    };
}
#[no_mangle]
pub static mut is_basic_table: [u32; 8] = [
    0x1a00 as i32 as u32,
    0xffffffef as u32,
    0xfffffffe as u32,
    0x7ffffffe as i32 as u32,
    0,
    0,
    0,
    0,
];