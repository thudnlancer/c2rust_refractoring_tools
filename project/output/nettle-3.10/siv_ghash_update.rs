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
    fn _nettle_ghash_update(
        ctx: *const gcm_key,
        state: *mut nettle_block16,
        blocks: size_t,
        data: *const uint8_t,
    ) -> *const uint8_t;
}
pub type size_t = u64;
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
#[no_mangle]
pub unsafe extern "C" fn _nettle_siv_ghash_update(
    mut ctx: *const gcm_key,
    mut state: *mut nettle_block16,
    mut blocks: size_t,
    mut data: *const uint8_t,
) -> *const uint8_t {
    loop {
        let fresh0 = blocks;
        blocks = blocks.wrapping_sub(1);
        if !(fresh0 > 0 as i32 as u64) {
            break;
        }
        let mut b: nettle_block16 = nettle_block16 { b: [0; 16] };
        b.u64_0[1 as i32 as usize] = (*data.offset(0 as i32 as isize) as uint64_t)
            << 56 as i32 | (*data.offset(1 as i32 as isize) as uint64_t) << 48 as i32
            | (*data.offset(2 as i32 as isize) as uint64_t) << 40 as i32
            | (*data.offset(3 as i32 as isize) as uint64_t) << 32 as i32
            | (*data.offset(4 as i32 as isize) as uint64_t) << 24 as i32
            | (*data.offset(5 as i32 as isize) as uint64_t) << 16 as i32
            | (*data.offset(6 as i32 as isize) as uint64_t) << 8 as i32
            | *data.offset(7 as i32 as isize) as uint64_t;
        b.u64_0[0 as i32 as usize] = (*data
            .offset(8 as i32 as isize)
            .offset(0 as i32 as isize) as uint64_t) << 56 as i32
            | (*data.offset(8 as i32 as isize).offset(1 as i32 as isize) as uint64_t)
                << 48 as i32
            | (*data.offset(8 as i32 as isize).offset(2 as i32 as isize) as uint64_t)
                << 40 as i32
            | (*data.offset(8 as i32 as isize).offset(3 as i32 as isize) as uint64_t)
                << 32 as i32
            | (*data.offset(8 as i32 as isize).offset(4 as i32 as isize) as uint64_t)
                << 24 as i32
            | (*data.offset(8 as i32 as isize).offset(5 as i32 as isize) as uint64_t)
                << 16 as i32
            | (*data.offset(8 as i32 as isize).offset(6 as i32 as isize) as uint64_t)
                << 8 as i32
            | *data.offset(8 as i32 as isize).offset(7 as i32 as isize) as uint64_t;
        _nettle_ghash_update(ctx, state, 1 as i32 as size_t, (b.b).as_mut_ptr());
        data = data.offset(16 as i32 as isize);
    }
    return data;
}