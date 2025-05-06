#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#[no_mangle]
pub unsafe extern "C" fn _glp_amd_valid(
    mut n_row: i32,
    mut n_col: i32,
    mut Ap: *const i32,
    mut Ai: *const i32,
) -> i32 {
    let mut nz: i32 = 0;
    let mut j: i32 = 0;
    let mut p1: i32 = 0;
    let mut p2: i32 = 0;
    let mut ilast: i32 = 0;
    let mut i: i32 = 0;
    let mut p: i32 = 0;
    let mut result: i32 = 0 as i32;
    if n_row < 0 as i32 || n_col < 0 as i32 || Ap.is_null() || Ai.is_null() {
        return -(2 as i32);
    }
    nz = *Ap.offset(n_col as isize);
    if *Ap.offset(0 as i32 as isize) != 0 as i32 || nz < 0 as i32 {
        return -(2 as i32);
    }
    j = 0 as i32;
    while j < n_col {
        p1 = *Ap.offset(j as isize);
        p2 = *Ap.offset((j + 1 as i32) as isize);
        if p1 > p2 {
            return -(2 as i32);
        }
        ilast = -(1 as i32);
        p = p1;
        while p < p2 {
            i = *Ai.offset(p as isize);
            if i < 0 as i32 || i >= n_row {
                return -(2 as i32);
            }
            if i <= ilast {
                result = 1 as i32;
            }
            ilast = i;
            p += 1;
            p;
        }
        j += 1;
        j;
    }
    return result;
}