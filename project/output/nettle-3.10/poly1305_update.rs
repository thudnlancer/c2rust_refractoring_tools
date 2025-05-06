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
    fn _nettle_poly1305_block(ctx: *mut poly1305_ctx, m: *const uint8_t, high: u32);
    fn _nettle_poly1305_blocks(
        ctx: *mut poly1305_ctx,
        blocks: size_t,
        m: *const uint8_t,
    ) -> *const uint8_t;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct poly1305_ctx {
    pub r: C2RustUnnamed_0,
    pub s32: [uint32_t; 3],
    pub hh: uint32_t,
    pub h: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub h32: [uint32_t; 4],
    pub h64: [uint64_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub r32: [uint32_t; 6],
    pub r64: [uint64_t; 3],
}
#[no_mangle]
pub unsafe extern "C" fn _nettle_poly1305_update(
    mut ctx: *mut poly1305_ctx,
    mut block: *mut uint8_t,
    mut index: u32,
    mut length: size_t,
    mut m: *const uint8_t,
) -> u32 {
    if length == 0 {
        return index;
    }
    if index > 0 as i32 as u32 {
        let mut __md_left: u32 = (16 as i32 as u32).wrapping_sub(index);
        if length < __md_left as u64 {
            memcpy(
                block.offset(index as isize) as *mut libc::c_void,
                m as *const libc::c_void,
                length,
            );
            return (index as u64).wrapping_add(length) as u32;
        }
        memcpy(
            block.offset(index as isize) as *mut libc::c_void,
            m as *const libc::c_void,
            __md_left as u64,
        );
        m = m.offset(__md_left as isize);
        length = (length as u64).wrapping_sub(__md_left as u64) as size_t as size_t;
        _nettle_poly1305_block(ctx, block, 1 as i32 as u32);
    }
    m = _nettle_poly1305_blocks(ctx, length >> 4 as i32, m);
    length &= 15 as i32 as u64;
    memcpy(block as *mut libc::c_void, m as *const libc::c_void, length);
    return length as u32;
}