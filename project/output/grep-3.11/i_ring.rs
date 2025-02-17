#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn abort() -> !;
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    I_RING_SIZE = 4,
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            C2RustUnnamed::I_RING_SIZE => 4,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct I_ring {
    pub ir_data: [libc::c_int; 4],
    pub ir_default_val: libc::c_int,
    pub ir_front: libc::c_uint,
    pub ir_back: libc::c_uint,
    pub ir_empty: bool,
}
#[no_mangle]
pub unsafe extern "C" fn i_ring_init(mut ir: *mut I_ring, mut default_val: libc::c_int) {
    let mut i: libc::c_int = 0;
    (*ir).ir_empty = 1 as libc::c_int != 0;
    (*ir).ir_front = 0 as libc::c_int as libc::c_uint;
    (*ir).ir_back = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int;
    while i < I_RING_SIZE as libc::c_int {
        (*ir).ir_data[i as usize] = default_val;
        i += 1;
        i;
    }
    (*ir).ir_default_val = default_val;
}
#[no_mangle]
pub unsafe extern "C" fn i_ring_empty(mut ir: *const I_ring) -> bool {
    return (*ir).ir_empty;
}
#[no_mangle]
pub unsafe extern "C" fn i_ring_push(
    mut ir: *mut I_ring,
    mut val: libc::c_int,
) -> libc::c_int {
    let mut dest_idx: libc::c_uint = ((*ir).ir_front)
        .wrapping_add(!(*ir).ir_empty as libc::c_int as libc::c_uint)
        .wrapping_rem(I_RING_SIZE as libc::c_int as libc::c_uint);
    let mut old_val: libc::c_int = (*ir).ir_data[dest_idx as usize];
    (*ir).ir_data[dest_idx as usize] = val;
    (*ir).ir_front = dest_idx;
    if dest_idx == (*ir).ir_back {
        (*ir)
            .ir_back = ((*ir).ir_back)
            .wrapping_add(!(*ir).ir_empty as libc::c_int as libc::c_uint)
            .wrapping_rem(I_RING_SIZE as libc::c_int as libc::c_uint);
    }
    (*ir).ir_empty = 0 as libc::c_int != 0;
    return old_val;
}
#[no_mangle]
pub unsafe extern "C" fn i_ring_pop(mut ir: *mut I_ring) -> libc::c_int {
    let mut top_val: libc::c_int = 0;
    if i_ring_empty(ir) {
        abort();
    }
    top_val = (*ir).ir_data[(*ir).ir_front as usize];
    (*ir).ir_data[(*ir).ir_front as usize] = (*ir).ir_default_val;
    if (*ir).ir_front == (*ir).ir_back {
        (*ir).ir_empty = 1 as libc::c_int != 0;
    } else {
        (*ir)
            .ir_front = ((*ir).ir_front)
            .wrapping_add(I_RING_SIZE as libc::c_int as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_rem(I_RING_SIZE as libc::c_int as libc::c_uint);
    }
    return top_val;
}
