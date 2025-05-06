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
    fn setlocale_null_r(category: i32, buf: *mut i8, bufsize: size_t) -> i32;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
}
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn hard_locale(mut category: i32) -> bool {
    let mut locale: [i8; 257] = [0; 257];
    if setlocale_null_r(
        category,
        locale.as_mut_ptr(),
        ::core::mem::size_of::<[i8; 257]>() as u64,
    ) != 0
    {
        return 0 as i32 != 0;
    }
    return !(strcmp(locale.as_mut_ptr(), b"C\0" as *const u8 as *const i8) == 0 as i32
        || strcmp(locale.as_mut_ptr(), b"POSIX\0" as *const u8 as *const i8)
            == 0 as i32);
}