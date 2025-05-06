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
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const i8,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
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
static mut internal_state: mbstate_t = mbstate_t {
    __count: 0,
    __value: C2RustUnnamed { __wch: 0 },
};
#[no_mangle]
pub unsafe extern "C" fn rpl_mbrlen(
    mut s: *const i8,
    mut n: size_t,
    mut ps: *mut mbstate_t,
) -> size_t {
    if ps.is_null() {
        ps = &mut internal_state;
    }
    return rpl_mbrtowc(0 as *mut wchar_t, s, n, ps);
}