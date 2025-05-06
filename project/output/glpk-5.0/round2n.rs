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
    fn frexp(_: libc::c_double, _: *mut i32) -> libc::c_double;
    fn ldexp(_: libc::c_double, _: i32) -> libc::c_double;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_round2n(mut x: libc::c_double) -> libc::c_double {
    let mut e: i32 = 0;
    let mut f: libc::c_double = 0.;
    (x > 0.0f64
        || {
            glp_assert_(
                b"x > 0.0\0" as *const u8 as *const i8,
                b"misc/round2n.c\0" as *const u8 as *const i8,
                57 as i32,
            );
            1 as i32 != 0
        }) as i32;
    f = frexp(x, &mut e);
    return ldexp(1.0f64, if f <= 0.75f64 { e - 1 as i32 } else { e });
}