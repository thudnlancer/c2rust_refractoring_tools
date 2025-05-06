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
}
#[no_mangle]
pub unsafe extern "C" fn _glp_amd_control(mut Control: *mut libc::c_double) {
    let mut alpha: libc::c_double = 0.;
    let mut aggressive: i32 = 0;
    if !Control.is_null() {
        alpha = *Control.offset(0 as i32 as isize);
        aggressive = (*Control.offset(1 as i32 as isize) != 0 as i32 as libc::c_double)
            as i32;
    } else {
        alpha = 10.0f64;
        aggressive = 1 as i32;
    }
    glp_printf(
        b"\nAMD version %d.%d.%d, %s: approximate minimum degree ordering\n    dense row parameter: %g\n\0"
            as *const u8 as *const i8,
        2 as i32,
        2 as i32,
        0 as i32,
        b"May 31, 2007\0" as *const u8 as *const i8,
        alpha,
    );
    if alpha < 0 as i32 as libc::c_double {
        glp_printf(b"    no rows treated as dense\n\0" as *const u8 as *const i8);
    } else {
        glp_printf(
            b"    (rows with more than max (%g * sqrt (n), 16) entries are\n    considered \"dense\", and placed last in output permutation)\n\0"
                as *const u8 as *const i8,
            alpha,
        );
    }
    if aggressive != 0 {
        glp_printf(b"    aggressive absorption:  yes\n\0" as *const u8 as *const i8);
    } else {
        glp_printf(b"    aggressive absorption:  no\n\0" as *const u8 as *const i8);
    }
    glp_printf(
        b"    size of AMD integer: %d\n\n\0" as *const u8 as *const i8,
        ::core::mem::size_of::<i32>() as u64,
    );
}