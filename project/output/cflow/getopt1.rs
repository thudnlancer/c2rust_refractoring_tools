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
    fn rpl_getopt_internal(
        ___argc: i32,
        ___argv: *mut *mut i8,
        __shortopts: *const i8,
        __longopts: *const rpl_option,
        __longind: *mut i32,
        __long_only: i32,
        __posixly_correct: i32,
    ) -> i32;
    fn _getopt_internal_r(
        ___argc: i32,
        ___argv: *mut *mut i8,
        __shortopts: *const i8,
        __longopts: *const rpl_option,
        __longind: *mut i32,
        __long_only: i32,
        __posixly_correct: i32,
        __data: *mut _getopt_data,
    ) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rpl_option {
    pub name: *const i8,
    pub has_arg: i32,
    pub flag: *mut i32,
    pub val: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _getopt_data {
    pub rpl_optind: i32,
    pub rpl_opterr: i32,
    pub rpl_optopt: i32,
    pub rpl_optarg: *mut i8,
    pub __initialized: i32,
    pub __nextchar: *mut i8,
    pub __ordering: C2RustUnnamed,
    pub __posixly_correct: i32,
    pub __first_nonopt: i32,
    pub __last_nonopt: i32,
}
pub type C2RustUnnamed = u32;
pub const RETURN_IN_ORDER: C2RustUnnamed = 2;
pub const PERMUTE: C2RustUnnamed = 1;
pub const REQUIRE_ORDER: C2RustUnnamed = 0;
#[no_mangle]
pub unsafe extern "C" fn rpl_getopt_long(
    mut argc: i32,
    mut argv: *mut *mut i8,
    mut options: *const i8,
    mut long_options: *const rpl_option,
    mut opt_index: *mut i32,
) -> i32 {
    return rpl_getopt_internal(
        argc,
        argv,
        options,
        long_options,
        opt_index,
        0 as i32,
        0 as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _getopt_long_r(
    mut argc: i32,
    mut argv: *mut *mut i8,
    mut options: *const i8,
    mut long_options: *const rpl_option,
    mut opt_index: *mut i32,
    mut d: *mut _getopt_data,
) -> i32 {
    return _getopt_internal_r(
        argc,
        argv,
        options,
        long_options,
        opt_index,
        0 as i32,
        0 as i32,
        d,
    );
}
#[no_mangle]
pub unsafe extern "C" fn rpl_getopt_long_only(
    mut argc: i32,
    mut argv: *mut *mut i8,
    mut options: *const i8,
    mut long_options: *const rpl_option,
    mut opt_index: *mut i32,
) -> i32 {
    return rpl_getopt_internal(
        argc,
        argv,
        options,
        long_options,
        opt_index,
        1 as i32,
        0 as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _getopt_long_only_r(
    mut argc: i32,
    mut argv: *mut *mut i8,
    mut options: *const i8,
    mut long_options: *const rpl_option,
    mut opt_index: *mut i32,
    mut d: *mut _getopt_data,
) -> i32 {
    return _getopt_internal_r(
        argc,
        argv,
        options,
        long_options,
        opt_index,
        1 as i32,
        0 as i32,
        d,
    );
}