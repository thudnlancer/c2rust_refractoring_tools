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
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_fp2rat(
    mut x: libc::c_double,
    mut eps: libc::c_double,
    mut p: *mut libc::c_double,
    mut q: *mut libc::c_double,
) -> i32 {
    let mut k: i32 = 0;
    let mut xk: libc::c_double = 0.;
    let mut Akm1: libc::c_double = 0.;
    let mut Ak: libc::c_double = 0.;
    let mut Bkm1: libc::c_double = 0.;
    let mut Bk: libc::c_double = 0.;
    let mut ak: libc::c_double = 0.;
    let mut bk: libc::c_double = 0.;
    let mut fk: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    (0.0f64 <= x && x < 1.0f64
        || {
            glp_assert_(
                b"0.0 <= x && x < 1.0\0" as *const u8 as *const i8,
                b"misc/fp2rat.c\0" as *const u8 as *const i8,
                117 as i32,
            );
            1 as i32 != 0
        }) as i32;
    k = 0 as i32;
    loop {
        (k <= 100 as i32
            || {
                glp_assert_(
                    b"k <= 100\0" as *const u8 as *const i8,
                    b"misc/fp2rat.c\0" as *const u8 as *const i8,
                    119 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if k == 0 as i32 {
            xk = x;
            Akm1 = 1.0f64;
            Ak = 0.0f64;
            Bkm1 = 0.0f64;
            Bk = 1.0f64;
        } else {
            temp = xk - floor(xk);
            (temp != 0.0f64
                || {
                    glp_assert_(
                        b"temp != 0.0\0" as *const u8 as *const i8,
                        b"misc/fp2rat.c\0" as *const u8 as *const i8,
                        135 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            xk = 1.0f64 / temp;
            ak = 1.0f64;
            bk = floor(xk);
            temp = bk * Ak + ak * Akm1;
            Akm1 = Ak;
            Ak = temp;
            temp = bk * Bk + ak * Bkm1;
            Bkm1 = Bk;
            Bk = temp;
        }
        fk = Ak / Bk;
        if fabs(x - fk) <= eps {
            break;
        }
        k += 1;
        k;
    }
    *p = Ak;
    *q = Bk;
    return k;
}