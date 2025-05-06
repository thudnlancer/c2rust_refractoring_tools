#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yarrow_key_event_ctx {
    pub index: u32,
    pub chars: [u32; 16],
    pub previous: u32,
}
#[no_mangle]
pub unsafe extern "C" fn nettle_yarrow_key_event_init(
    mut ctx: *mut yarrow_key_event_ctx,
) {
    let mut i: u32 = 0;
    (*ctx).index = 0 as i32 as u32;
    (*ctx).previous = 0 as i32 as u32;
    i = 0 as i32 as u32;
    while i < 16 as i32 as u32 {
        (*ctx).chars[i as usize] = 0 as i32 as u32;
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn nettle_yarrow_key_event_estimate(
    mut ctx: *mut yarrow_key_event_ctx,
    mut key: u32,
    mut time: u32,
) -> u32 {
    let mut entropy: u32 = 0 as i32 as u32;
    let mut i: u32 = 0;
    if (*ctx).previous != 0 && time > (*ctx).previous {
        if time.wrapping_sub((*ctx).previous) >= 256 as i32 as u32 {
            entropy = entropy.wrapping_add(1);
            entropy;
        }
    }
    (*ctx).previous = time;
    if key == 0 {
        return entropy;
    }
    i = 0 as i32 as u32;
    while i < 16 as i32 as u32 {
        if key == (*ctx).chars[i as usize] {
            return entropy;
        }
        i = i.wrapping_add(1);
        i;
    }
    if (*ctx).chars[(*ctx).index as usize] != 0 {
        entropy = entropy.wrapping_add(1);
        entropy;
    }
    (*ctx).chars[(*ctx).index as usize] = key;
    (*ctx).index = ((*ctx).index)
        .wrapping_add(1 as i32 as u32)
        .wrapping_rem(16 as i32 as u32);
    return entropy;
}