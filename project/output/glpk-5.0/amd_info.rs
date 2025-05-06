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
pub unsafe extern "C" fn _glp_amd_info(mut Info: *mut libc::c_double) {
    let mut n: libc::c_double = 0.;
    let mut ndiv: libc::c_double = 0.;
    let mut nmultsubs_ldl: libc::c_double = 0.;
    let mut nmultsubs_lu: libc::c_double = 0.;
    let mut lnz: libc::c_double = 0.;
    let mut lnzd: libc::c_double = 0.;
    glp_printf(
        b"\nAMD version %d.%d.%d, %s, results:\n\0" as *const u8 as *const i8,
        2 as i32,
        2 as i32,
        0 as i32,
        b"May 31, 2007\0" as *const u8 as *const i8,
    );
    if Info.is_null() {
        return;
    }
    n = *Info.offset(1 as i32 as isize);
    ndiv = *Info.offset(10 as i32 as isize);
    nmultsubs_ldl = *Info.offset(11 as i32 as isize);
    nmultsubs_lu = *Info.offset(12 as i32 as isize);
    lnz = *Info.offset(9 as i32 as isize);
    lnzd = if n >= 0 as i32 as libc::c_double && lnz >= 0 as i32 as libc::c_double {
        n + lnz
    } else {
        -(1 as i32) as libc::c_double
    };
    glp_printf(b"    status: \0" as *const u8 as *const i8);
    if *Info.offset(0 as i32 as isize) == 0 as i32 as libc::c_double {
        glp_printf(b"OK\n\0" as *const u8 as *const i8);
    } else if *Info.offset(0 as i32 as isize) == -(1 as i32) as libc::c_double {
        glp_printf(b"out of memory\n\0" as *const u8 as *const i8);
    } else if *Info.offset(0 as i32 as isize) == -(2 as i32) as libc::c_double {
        glp_printf(b"invalid matrix\n\0" as *const u8 as *const i8);
    } else if *Info.offset(0 as i32 as isize) == 1 as i32 as libc::c_double {
        glp_printf(b"OK, but jumbled\n\0" as *const u8 as *const i8);
    } else {
        glp_printf(b"unknown\n\0" as *const u8 as *const i8);
    }
    if n >= 0 as i32 as libc::c_double {
        glp_printf(
            b"    n, dimension of A:                                  %.20g\n\0"
                as *const u8 as *const i8,
            n,
        );
    }
    if *Info.offset(2 as i32 as isize) >= 0 as i32 as libc::c_double {
        glp_printf(
            b"    nz, number of nonzeros in A:                        %.20g\n\0"
                as *const u8 as *const i8,
            *Info.offset(2 as i32 as isize),
        );
    }
    if *Info.offset(3 as i32 as isize) >= 0 as i32 as libc::c_double {
        glp_printf(
            b"    symmetry of A:                                      %.4f\n\0"
                as *const u8 as *const i8,
            *Info.offset(3 as i32 as isize),
        );
    }
    if *Info.offset(4 as i32 as isize) >= 0 as i32 as libc::c_double {
        glp_printf(
            b"    number of nonzeros on diagonal:                     %.20g\n\0"
                as *const u8 as *const i8,
            *Info.offset(4 as i32 as isize),
        );
    }
    if *Info.offset(5 as i32 as isize) >= 0 as i32 as libc::c_double {
        glp_printf(
            b"    nonzeros in pattern of A+A' (excl. diagonal):       %.20g\n\0"
                as *const u8 as *const i8,
            *Info.offset(5 as i32 as isize),
        );
    }
    if *Info.offset(6 as i32 as isize) >= 0 as i32 as libc::c_double {
        glp_printf(
            b"    # dense rows/columns of A+A':                       %.20g\n\0"
                as *const u8 as *const i8,
            *Info.offset(6 as i32 as isize),
        );
    }
    if *Info.offset(7 as i32 as isize) >= 0 as i32 as libc::c_double {
        glp_printf(
            b"    memory used, in bytes:                              %.20g\n\0"
                as *const u8 as *const i8,
            *Info.offset(7 as i32 as isize),
        );
    }
    if *Info.offset(8 as i32 as isize) >= 0 as i32 as libc::c_double {
        glp_printf(
            b"    # of memory compactions:                            %.20g\n\0"
                as *const u8 as *const i8,
            *Info.offset(8 as i32 as isize),
        );
    }
    glp_printf(
        b"\n    The following approximate statistics are for a subsequent\n    factorization of A(P,P) + A(P,P)'.  They are slight upper\n    bounds if there are no dense rows/columns in A+A', and become\n    looser if dense rows/columns exist.\n\n\0"
            as *const u8 as *const i8,
    );
    if lnz >= 0 as i32 as libc::c_double {
        glp_printf(
            b"    nonzeros in L (excluding diagonal):                 %.20g\n\0"
                as *const u8 as *const i8,
            lnz,
        );
    }
    if lnzd >= 0 as i32 as libc::c_double {
        glp_printf(
            b"    nonzeros in L (including diagonal):                 %.20g\n\0"
                as *const u8 as *const i8,
            lnzd,
        );
    }
    if ndiv >= 0 as i32 as libc::c_double {
        glp_printf(
            b"    # divide operations for LDL' or LU:                 %.20g\n\0"
                as *const u8 as *const i8,
            ndiv,
        );
    }
    if nmultsubs_ldl >= 0 as i32 as libc::c_double {
        glp_printf(
            b"    # multiply-subtract operations for LDL':            %.20g\n\0"
                as *const u8 as *const i8,
            nmultsubs_ldl,
        );
    }
    if nmultsubs_lu >= 0 as i32 as libc::c_double {
        glp_printf(
            b"    # multiply-subtract operations for LU:              %.20g\n\0"
                as *const u8 as *const i8,
            nmultsubs_lu,
        );
    }
    if *Info.offset(13 as i32 as isize) >= 0 as i32 as libc::c_double {
        glp_printf(
            b"    max nz. in any column of L (incl. diagonal):        %.20g\n\0"
                as *const u8 as *const i8,
            *Info.offset(13 as i32 as isize),
        );
    }
    if n >= 0 as i32 as libc::c_double && ndiv >= 0 as i32 as libc::c_double
        && nmultsubs_ldl >= 0 as i32 as libc::c_double
        && nmultsubs_lu >= 0 as i32 as libc::c_double
    {
        glp_printf(
            b"\n    chol flop count for real A, sqrt counted as 1 flop: %.20g\n    LDL' flop count for real A:                         %.20g\n    LDL' flop count for complex A:                      %.20g\n    LU flop count for real A (with no pivoting):        %.20g\n    LU flop count for complex A (with no pivoting):     %.20g\n\n\0"
                as *const u8 as *const i8,
            n + ndiv + 2 as i32 as libc::c_double * nmultsubs_ldl,
            ndiv + 2 as i32 as libc::c_double * nmultsubs_ldl,
            9 as i32 as libc::c_double * ndiv
                + 8 as i32 as libc::c_double * nmultsubs_ldl,
            ndiv + 2 as i32 as libc::c_double * nmultsubs_lu,
            9 as i32 as libc::c_double * ndiv + 8 as i32 as libc::c_double * nmultsubs_lu,
        );
    }
}