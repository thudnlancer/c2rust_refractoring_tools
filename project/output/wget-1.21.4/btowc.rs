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
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
}
pub type size_t = u64;
pub type wchar_t = i32;
pub type wint_t = u32;
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
pub unsafe extern "C" fn rpl_btowc(mut c: i32) -> wint_t {
    if c != -(1 as i32) {
        let mut buf: [i8; 1] = [0; 1];
        let mut wc: wchar_t = 0;
        buf[0 as i32 as usize] = c as i8;
        let mut state: mbstate_t = mbstate_t {
            __count: 0,
            __value: C2RustUnnamed { __wch: 0 },
        };
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<mbstate_t>() as u64,
        );
        let mut ret: size_t = rpl_mbrtowc(
            &mut wc,
            buf.as_mut_ptr(),
            1 as i32 as size_t,
            &mut state,
        );
        if !(ret == -(1 as i32) as size_t || ret == -(2 as i32) as size_t) {
            return wc as wint_t;
        }
    }
    return 0xffffffff as u32;
}