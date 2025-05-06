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
    fn _glp_amd_post_tree(
        root: i32,
        k: i32,
        Child: *mut i32,
        Sibling: *const i32,
        Order: *mut i32,
        Stack: *mut i32,
    ) -> i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_amd_postorder(
    mut nn: i32,
    mut Parent: *mut i32,
    mut Nv: *mut i32,
    mut Fsize: *mut i32,
    mut Order: *mut i32,
    mut Child: *mut i32,
    mut Sibling: *mut i32,
    mut Stack: *mut i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut parent: i32 = 0;
    let mut frsize: i32 = 0;
    let mut f: i32 = 0;
    let mut fprev: i32 = 0;
    let mut maxfrsize: i32 = 0;
    let mut bigfprev: i32 = 0;
    let mut bigf: i32 = 0;
    let mut fnext: i32 = 0;
    j = 0 as i32;
    while j < nn {
        *Child.offset(j as isize) = -(1 as i32);
        *Sibling.offset(j as isize) = -(1 as i32);
        j += 1;
        j;
    }
    j = nn - 1 as i32;
    while j >= 0 as i32 {
        if *Nv.offset(j as isize) > 0 as i32 {
            parent = *Parent.offset(j as isize);
            if parent != -(1 as i32) {
                *Sibling.offset(j as isize) = *Child.offset(parent as isize);
                *Child.offset(parent as isize) = j;
            }
        }
        j -= 1;
        j;
    }
    i = 0 as i32;
    while i < nn {
        if *Nv.offset(i as isize) > 0 as i32 && *Child.offset(i as isize) != -(1 as i32)
        {
            fprev = -(1 as i32);
            maxfrsize = -(1 as i32);
            bigfprev = -(1 as i32);
            bigf = -(1 as i32);
            f = *Child.offset(i as isize);
            while f != -(1 as i32) {
                frsize = *Fsize.offset(f as isize);
                if frsize >= maxfrsize {
                    maxfrsize = frsize;
                    bigfprev = fprev;
                    bigf = f;
                }
                fprev = f;
                f = *Sibling.offset(f as isize);
            }
            fnext = *Sibling.offset(bigf as isize);
            if fnext != -(1 as i32) {
                if bigfprev == -(1 as i32) {
                    *Child.offset(i as isize) = fnext;
                } else {
                    *Sibling.offset(bigfprev as isize) = fnext;
                }
                *Sibling.offset(bigf as isize) = -(1 as i32);
                *Sibling.offset(fprev as isize) = bigf;
            }
        }
        i += 1;
        i;
    }
    i = 0 as i32;
    while i < nn {
        *Order.offset(i as isize) = -(1 as i32);
        i += 1;
        i;
    }
    k = 0 as i32;
    i = 0 as i32;
    while i < nn {
        if *Parent.offset(i as isize) == -(1 as i32) && *Nv.offset(i as isize) > 0 as i32
        {
            k = _glp_amd_post_tree(i, k, Child, Sibling as *const i32, Order, Stack);
        }
        i += 1;
        i;
    }
}