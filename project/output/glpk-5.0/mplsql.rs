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
    fn glp_printf(fmt: *const i8, _: ...);
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABDCA {
    pub id: i32,
    pub link: *mut libc::c_void,
    pub na: i32,
    pub arg: *mut *mut i8,
    pub nf: i32,
    pub name: *mut *mut i8,
    pub type_0: *mut i32,
    pub num: *mut libc::c_double,
    pub str_0: *mut *mut i8,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_db_iodbc_open(
    mut dca: *mut TABDCA,
    mut mode: i32,
) -> *mut libc::c_void {
    (dca == dca
        || {
            glp_assert_(
                b"dca == dca\0" as *const u8 as *const i8,
                b"mpl/mplsql.c\0" as *const u8 as *const i8,
                302 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (mode == mode
        || {
            glp_assert_(
                b"mode == mode\0" as *const u8 as *const i8,
                b"mpl/mplsql.c\0" as *const u8 as *const i8,
                303 as i32,
            );
            1 as i32 != 0
        }) as i32;
    glp_printf(b"iODBC table driver not supported\n\0" as *const u8 as *const i8);
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_db_iodbc_read(
    mut dca: *mut TABDCA,
    mut link: *mut libc::c_void,
) -> i32 {
    (dca != dca
        || {
            glp_assert_(
                b"dca != dca\0" as *const u8 as *const i8,
                b"mpl/mplsql.c\0" as *const u8 as *const i8,
                309 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (link != link
        || {
            glp_assert_(
                b"link != link\0" as *const u8 as *const i8,
                b"mpl/mplsql.c\0" as *const u8 as *const i8,
                310 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_db_iodbc_write(
    mut dca: *mut TABDCA,
    mut link: *mut libc::c_void,
) -> i32 {
    (dca != dca
        || {
            glp_assert_(
                b"dca != dca\0" as *const u8 as *const i8,
                b"mpl/mplsql.c\0" as *const u8 as *const i8,
                315 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (link != link
        || {
            glp_assert_(
                b"link != link\0" as *const u8 as *const i8,
                b"mpl/mplsql.c\0" as *const u8 as *const i8,
                316 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_db_iodbc_close(
    mut dca: *mut TABDCA,
    mut link: *mut libc::c_void,
) -> i32 {
    (dca != dca
        || {
            glp_assert_(
                b"dca != dca\0" as *const u8 as *const i8,
                b"mpl/mplsql.c\0" as *const u8 as *const i8,
                321 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (link != link
        || {
            glp_assert_(
                b"link != link\0" as *const u8 as *const i8,
                b"mpl/mplsql.c\0" as *const u8 as *const i8,
                322 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_db_mysql_open(
    mut dca: *mut TABDCA,
    mut mode: i32,
) -> *mut libc::c_void {
    (dca == dca
        || {
            glp_assert_(
                b"dca == dca\0" as *const u8 as *const i8,
                b"mpl/mplsql.c\0" as *const u8 as *const i8,
                1083 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (mode == mode
        || {
            glp_assert_(
                b"mode == mode\0" as *const u8 as *const i8,
                b"mpl/mplsql.c\0" as *const u8 as *const i8,
                1084 as i32,
            );
            1 as i32 != 0
        }) as i32;
    glp_printf(b"MySQL table driver not supported\n\0" as *const u8 as *const i8);
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_db_mysql_read(
    mut dca: *mut TABDCA,
    mut link: *mut libc::c_void,
) -> i32 {
    (dca != dca
        || {
            glp_assert_(
                b"dca != dca\0" as *const u8 as *const i8,
                b"mpl/mplsql.c\0" as *const u8 as *const i8,
                1090 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (link != link
        || {
            glp_assert_(
                b"link != link\0" as *const u8 as *const i8,
                b"mpl/mplsql.c\0" as *const u8 as *const i8,
                1091 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_db_mysql_write(
    mut dca: *mut TABDCA,
    mut link: *mut libc::c_void,
) -> i32 {
    (dca != dca
        || {
            glp_assert_(
                b"dca != dca\0" as *const u8 as *const i8,
                b"mpl/mplsql.c\0" as *const u8 as *const i8,
                1096 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (link != link
        || {
            glp_assert_(
                b"link != link\0" as *const u8 as *const i8,
                b"mpl/mplsql.c\0" as *const u8 as *const i8,
                1097 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_db_mysql_close(
    mut dca: *mut TABDCA,
    mut link: *mut libc::c_void,
) -> i32 {
    (dca != dca
        || {
            glp_assert_(
                b"dca != dca\0" as *const u8 as *const i8,
                b"mpl/mplsql.c\0" as *const u8 as *const i8,
                1102 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (link != link
        || {
            glp_assert_(
                b"link != link\0" as *const u8 as *const i8,
                b"mpl/mplsql.c\0" as *const u8 as *const i8,
                1103 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return 0 as i32;
}