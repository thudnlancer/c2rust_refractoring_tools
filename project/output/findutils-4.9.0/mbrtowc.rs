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
    fn mbrtowc(
        __pwc: *mut wchar_t,
        __s: *const i8,
        __n: size_t,
        __p: *mut mbstate_t,
    ) -> size_t;
    fn hard_locale(category: i32) -> bool;
}
pub type size_t = u64;
pub type wchar_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: i32,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: u32,
    pub __wchb: [i8; 4],
}
pub type mbstate_t = __mbstate_t;
#[no_mangle]
pub unsafe extern "C" fn rpl_mbrtowc(
    mut pwc: *mut wchar_t,
    mut s: *const i8,
    mut n: size_t,
    mut ps: *mut mbstate_t,
) -> size_t {
    let mut ret: size_t = 0;
    let mut wc: wchar_t = 0;
    if pwc.is_null() {
        pwc = &mut wc;
    }
    ret = mbrtowc(pwc, s, n, ps);
    if -(2 as i32) as size_t <= ret && n != 0 as i32 as u64 && !hard_locale(0 as i32) {
        let mut uc: u8 = *s as u8;
        *pwc = uc as wchar_t;
        return 1 as i32 as size_t;
    }
    return ret;
}