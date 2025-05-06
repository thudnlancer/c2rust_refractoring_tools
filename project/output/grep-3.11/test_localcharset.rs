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
    fn locale_charset() -> *const i8;
    fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
    fn printf(_: *const i8, _: ...) -> i32;
}
unsafe fn main_0() -> i32 {
    setlocale(6 as i32, b"\0" as *const u8 as *const i8);
    printf(b"%s\n\0" as *const u8 as *const i8, locale_charset());
    return 0 as i32;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}