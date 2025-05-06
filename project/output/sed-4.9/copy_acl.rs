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
    fn qcopy_acl(_: *const i8, _: i32, _: *const i8, _: i32, _: mode_t) -> i32;
    fn __errno_location() -> *mut i32;
    fn quote(arg: *const i8) -> *const i8;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
}
pub type __mode_t = u32;
pub type mode_t = __mode_t;
#[no_mangle]
pub unsafe extern "C" fn copy_acl(
    mut src_name: *const i8,
    mut source_desc: i32,
    mut dst_name: *const i8,
    mut dest_desc: i32,
    mut mode: mode_t,
) -> i32 {
    let mut ret: i32 = qcopy_acl(src_name, source_desc, dst_name, dest_desc, mode);
    match ret {
        -2 => {
            error(
                0 as i32,
                *__errno_location(),
                b"%s\0" as *const u8 as *const i8,
                quote(src_name),
            );
        }
        -1 => {
            error(
                0 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"preserving permissions for %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                quote(dst_name),
            );
        }
        _ => {}
    }
    return ret;
}