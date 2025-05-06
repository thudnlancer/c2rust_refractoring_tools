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
    fn mbsinit(__ps: *const mbstate_t) -> i32;
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const i8,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn panic(str: *const i8, _: ...);
    fn locale_charset() -> *const i8;
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
pub static mut mb_cur_max: i32 = 0;
#[no_mangle]
pub static mut is_utf8: bool = false;
#[no_mangle]
pub unsafe extern "C" fn is_mb_char(mut ch: i32, mut cur_stat: *mut mbstate_t) -> i32 {
    let c: i8 = ch as i8;
    let mb_pending: i32 = (mbsinit(cur_stat) == 0) as i32;
    let result: i32 = rpl_mbrtowc(0 as *mut wchar_t, &c, 1 as i32 as size_t, cur_stat)
        as i32;
    match result {
        -2 => return 1 as i32,
        -1 => {
            memset(
                cur_stat as *mut libc::c_void,
                0 as i32,
                ::core::mem::size_of::<mbstate_t>() as u64,
            );
            return 0 as i32;
        }
        1 => return mb_pending,
        0 => return 1 as i32,
        _ => {
            panic(
                b"is_mb_char: mbrtowc (0x%x) returned %d\0" as *const u8 as *const i8,
                ch as u32,
                result,
            );
        }
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn initialize_mbcs() {
    let mut codeset_name: *const i8 = 0 as *const i8;
    codeset_name = locale_charset();
    is_utf8 = strcmp(codeset_name, b"UTF-8\0" as *const u8 as *const i8) == 0 as i32;
    mb_cur_max = __ctype_get_mb_cur_max() as i32;
}