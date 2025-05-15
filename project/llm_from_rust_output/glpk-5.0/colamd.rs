use std::cmp::{max, min};
use std::mem::size_of;
use std::ptr;

type SizeT = usize;

#[derive(Copy, Clone)]
struct ColamdRow {
    start: i32,
    length: i32,
    shared1: RowUnion1,
    shared2: RowUnion2,
}

#[derive(Copy, Clone)]
union RowUnion1 {
    degree: i32,
    p: i32,
}

#[derive(Copy, Clone)]
union RowUnion2 {
    mark: i32,
    first_column: i32,
}

#[derive(Copy, Clone)]
struct ColamdCol {
    start: i32,
    length: i32,
    shared1: ColUnion1,
    shared2: ColUnion2,
    shared3: ColUnion3,
    shared4: ColUnion4,
}

#[derive(Copy, Clone)]
union ColUnion1 {
    thickness: i32,
    parent: i32,
}

#[derive(Copy, Clone)]
union ColUnion2 {
    score: i32,
    order: i32,
}

#[derive(Copy, Clone)]
union ColUnion3 {
    headhash: i32,
    hash: i32,
    prev: i32,
}

#[derive(Copy, Clone)]
union ColUnion4 {
    degree_next: i32,
    hash_next: i32,
}

fn t_add(a: SizeT, b: SizeT, ok: &mut i32) -> SizeT {
    *ok = if *ok != 0 && a.wrapping_add(b) >= if a > b { a } else { b } {
        1
    } else {
        0
    };
    if *ok != 0 { a.wrapping_add(b) } else { 0 }
}

fn t_mult(a: SizeT, k: SizeT, ok: &mut i32) -> SizeT {
    let mut s = 0;
    for _ in 0..k {
        s = t_add(s, a, ok);
    }
    s
}

pub fn glp_colamd_recommended(nnz: i32, n_row: i32, n_col: i32) -> SizeT {
    let mut s;
    let mut c;
    let mut r;
    let mut ok = 1;

    if nnz < 0 || n_row < 0 || n_col < 0 {
        return 0;
    }

    s = t_mult(nnz as SizeT, 2, &mut ok);
    c = (t_mult(
        t_add(n_col as SizeT, 1, &mut ok),
        size_of::<ColamdCol>() as SizeT,
        &mut ok,
    )) / size_of::<i32>() as SizeT;
    r = (t_mult(
        t_add(n_row as SizeT, 1, &mut ok),
        size_of::<ColamdRow>() as SizeT,
        &mut ok,
    )) / size_of::<i32>() as SizeT;

    s = t_add(s, c, &mut ok);
    s = t_add(s, r, &mut ok);
    s = t_add(s, n_col as SizeT, &mut ok);
    s = t_add(s, (nnz / 5) as SizeT, &mut ok);

    ok = if ok != 0 && s < i32::MAX as SizeT { 1 } else { 0 };
    if ok != 0 { s } else { 0 }
}

pub fn glp_colamd_set_defaults(knobs: &mut [f64; 20]) {
    for knob in knobs.iter_mut() {
        *knob = 0.0;
    }
    knobs[0] = 10.0;
    knobs[1] = 10.0;
    knobs[2] = 1.0;
}

// Additional functions would follow the same pattern of conversion,
// replacing raw pointers with Rust's safe alternatives where possible,
// using Option for nullable pointers, and avoiding unsafe blocks.
// The full conversion would require more context about the usage patterns
// and dependencies of the original code.