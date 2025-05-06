#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __uint8_t = u8;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union nettle_block16 {
    pub b: [uint8_t; 16],
    pub w: [u64; 2],
    pub u64_0: [uint64_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gcm_key {
    pub h: [nettle_block16; 256],
}
#[inline]
unsafe extern "C" fn block16_set(
    mut r: *mut nettle_block16,
    mut x: *const nettle_block16,
) {
    (*r).u64_0[0 as i32 as usize] = (*x).u64_0[0 as i32 as usize];
    (*r).u64_0[1 as i32 as usize] = (*x).u64_0[1 as i32 as usize];
}
#[inline]
unsafe extern "C" fn block16_mulx_ghash(
    mut r: *mut nettle_block16,
    mut x: *const nettle_block16,
) {
    let mut mask: uint64_t = 0;
    mask = ((*x).u64_0[1 as i32 as usize] >> 56 as i32 & 1 as i32 as u64).wrapping_neg();
    (*r).u64_0[1 as i32 as usize] = ((*x).u64_0[1 as i32 as usize]
        & 0xfefefefefefefefe as u64) >> 1 as i32
        | ((*x).u64_0[1 as i32 as usize] & 0x1010101010101 as u64) << 15 as i32
        | (*x).u64_0[0 as i32 as usize] >> 49 as i32 & 0x80 as i32 as u64;
    (*r).u64_0[0 as i32 as usize] = (((*x).u64_0[0 as i32 as usize]
        & 0xfefefefefefefefe as u64) >> 1 as i32
        | ((*x).u64_0[0 as i32 as usize] & 0x1010101010101 as u64) << 15 as i32)
        ^ mask & 0xe1 as u64;
}
#[no_mangle]
pub unsafe extern "C" fn _nettle_ghash_set_key_c(
    mut ctx: *mut gcm_key,
    mut key: *const nettle_block16,
) {
    let mut i: u32 = 0;
    block16_set(
        &mut *((*ctx).h).as_mut_ptr().offset((2 as i32 * 7 as i32) as isize),
        key,
    );
    i = 1 as i32 as u32;
    while i < 64 as i32 as u32 {
        block16_mulx_ghash(
            &mut *((*ctx).h)
                .as_mut_ptr()
                .offset((2 as i32 as u32).wrapping_mul(i ^ 7 as i32 as u32) as isize),
            &mut *((*ctx).h)
                .as_mut_ptr()
                .offset(
                    (2 as i32 as u32)
                        .wrapping_mul(i.wrapping_sub(1 as i32 as u32) ^ 7 as i32 as u32)
                        as isize,
                ),
        );
        i = i.wrapping_add(1);
        i;
    }
    block16_mulx_ghash(
        &mut *((*ctx).h).as_mut_ptr().offset((2 as i32 * 7 as i32 + 1 as i32) as isize),
        &mut *((*ctx).h)
            .as_mut_ptr()
            .offset((2 as i32 * (63 as i32 ^ 7 as i32)) as isize),
    );
    i = 1 as i32 as u32;
    while i < 64 as i32 as u32 {
        block16_mulx_ghash(
            &mut *((*ctx).h)
                .as_mut_ptr()
                .offset(
                    (2 as i32 as u32)
                        .wrapping_mul(i ^ 7 as i32 as u32)
                        .wrapping_add(1 as i32 as u32) as isize,
                ),
            &mut *((*ctx).h)
                .as_mut_ptr()
                .offset(
                    (2 as i32 as u32)
                        .wrapping_mul(i.wrapping_sub(1 as i32 as u32) ^ 7 as i32 as u32)
                        .wrapping_add(1 as i32 as u32) as isize,
                ),
        );
        i = i.wrapping_add(1);
        i;
    }
}