#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    fn abort() -> !;
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    I_RING_SIZE = 4,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::I_RING_SIZE => 4,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            4 => C2RustUnnamed::I_RING_SIZE,
            _ => panic!("Invalid value for C2RustUnnamed: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn add(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn sub(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn mul(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn div(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn rem(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct I_ring {
    pub ir_data: [i32; 4],
    pub ir_default_val: i32,
    pub ir_front: u32,
    pub ir_back: u32,
    pub ir_empty: bool,
}
#[no_mangle]
pub unsafe extern "C" fn i_ring_init(mut ir: *mut I_ring, mut default_val: i32) {
    let mut i: i32 = 0;
    (*ir).ir_empty = 1 as i32 != 0;
    (*ir).ir_front = 0 as i32 as u32;
    (*ir).ir_back = 0 as i32 as u32;
    i = 0 as i32;
    while i < C2RustUnnamed::I_RING_SIZE as i32 {
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
pub unsafe extern "C" fn i_ring_push(mut ir: *mut I_ring, mut val: i32) -> i32 {
    let mut dest_idx: u32 = ((*ir).ir_front)
        .wrapping_add(!(*ir).ir_empty as i32 as u32)
        .wrapping_rem(C2RustUnnamed::I_RING_SIZE as i32 as u32);
    let mut old_val: i32 = (*ir).ir_data[dest_idx as usize];
    (*ir).ir_data[dest_idx as usize] = val;
    (*ir).ir_front = dest_idx;
    if dest_idx == (*ir).ir_back {
        (*ir).ir_back = ((*ir).ir_back)
            .wrapping_add(!(*ir).ir_empty as i32 as u32)
            .wrapping_rem(C2RustUnnamed::I_RING_SIZE as i32 as u32);
    }
    (*ir).ir_empty = 0 as i32 != 0;
    return old_val;
}
#[no_mangle]
pub unsafe extern "C" fn i_ring_pop(mut ir: *mut I_ring) -> i32 {
    let mut top_val: i32 = 0;
    if i_ring_empty(ir) {
        abort();
    }
    top_val = (*ir).ir_data[(*ir).ir_front as usize];
    (*ir).ir_data[(*ir).ir_front as usize] = (*ir).ir_default_val;
    if (*ir).ir_front == (*ir).ir_back {
        (*ir).ir_empty = 1 as i32 != 0;
    } else {
        (*ir).ir_front = ((*ir).ir_front)
            .wrapping_add(C2RustUnnamed::I_RING_SIZE as i32 as u32)
            .wrapping_sub(1 as i32 as u32)
            .wrapping_rem(C2RustUnnamed::I_RING_SIZE as i32 as u32);
    }
    return top_val;
}