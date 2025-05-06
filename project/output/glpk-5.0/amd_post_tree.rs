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
pub unsafe extern "C" fn _glp_amd_post_tree(
    mut root: i32,
    mut k: i32,
    mut Child: *mut i32,
    mut Sibling: *const i32,
    mut Order: *mut i32,
    mut Stack: *mut i32,
) -> i32 {
    let mut f: i32 = 0;
    let mut head: i32 = 0;
    let mut h: i32 = 0;
    let mut i: i32 = 0;
    head = 0 as i32;
    *Stack.offset(0 as i32 as isize) = root;
    while head >= 0 as i32 {
        i = *Stack.offset(head as isize);
        if *Child.offset(i as isize) != -(1 as i32) {
            f = *Child.offset(i as isize);
            while f != -(1 as i32) {
                head += 1;
                head;
                f = *Sibling.offset(f as isize);
            }
            h = head;
            f = *Child.offset(i as isize);
            while f != -(1 as i32) {
                let fresh0 = h;
                h = h - 1;
                *Stack.offset(fresh0 as isize) = f;
                f = *Sibling.offset(f as isize);
            }
            *Child.offset(i as isize) = -(1 as i32);
        } else {
            head -= 1;
            head;
            let fresh1 = k;
            k = k + 1;
            *Order.offset(i as isize) = fresh1;
        }
    }
    return k;
}