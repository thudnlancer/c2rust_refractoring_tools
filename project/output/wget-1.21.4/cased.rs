#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __uint32_t = u32;
pub type uint32_t = __uint32_t;
pub type ucs4_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub header: [i32; 1],
    pub level1: [i32; 2],
    pub level2: [libc::c_short; 256],
    pub level3: [u32; 400],
}
#[inline]
unsafe extern "C" fn bitmap_lookup(
    mut table: *const libc::c_void,
    mut uc: ucs4_t,
) -> i32 {
    let mut index1: u32 = uc >> 16 as i32;
    if index1 < *(table as *const i32).offset(0 as i32 as isize) as u32 {
        let mut lookup1: i32 = *(table as *const i32)
            .offset((1 as i32 as u32).wrapping_add(index1) as isize);
        if lookup1 >= 0 as i32 {
            let mut index2: u32 = uc >> 9 as i32 & 127 as i32 as u32;
            let mut lookup2: i32 = *(table as *const libc::c_short)
                .offset((lookup1 as u32).wrapping_add(index2) as isize) as i32;
            if lookup2 >= 0 as i32 {
                let mut index3: u32 = uc >> 5 as i32 & 15 as i32 as u32;
                let mut lookup3: u32 = *(table as *const u32)
                    .offset((lookup2 as u32).wrapping_add(index3) as isize);
                return (lookup3 >> (uc & 0x1f as i32 as u32) & 1 as i32 as u32) as i32;
            }
        }
    }
    return 0 as i32;
}
static mut u_casing_property_cased: C2RustUnnamed = C2RustUnnamed {
    header: [0; 1],
    level1: [0; 2],
    level2: [0; 256],
    level3: [0; 400],
};
#[no_mangle]
pub unsafe extern "C" fn uc_is_cased(mut uc: ucs4_t) -> bool {
    return bitmap_lookup(
        &u_casing_property_cased as *const C2RustUnnamed as *const libc::c_void,
        uc,
    ) != 0;
}
unsafe extern "C" fn run_static_initializers() {
    u_casing_property_cased = {
        let mut init = C2RustUnnamed {
            header: [2 as i32],
            level1: [
                (3 as i32 as u64)
                    .wrapping_mul(::core::mem::size_of::<i32>() as u64)
                    .wrapping_div(::core::mem::size_of::<libc::c_short>() as u64)
                    .wrapping_add(0 as i32 as u64) as i32,
                (3 as i32 as u64)
                    .wrapping_mul(::core::mem::size_of::<i32>() as u64)
                    .wrapping_div(::core::mem::size_of::<libc::c_short>() as u64)
                    .wrapping_add(128 as i32 as u64) as i32,
            ],
            level2: [
                (3 as i32 as u64)
                    .wrapping_add(
                        (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(0 as i32 as u64) as libc::c_short,
                (3 as i32 as u64)
                    .wrapping_add(
                        (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(16 as i32 as u64) as libc::c_short,
                (3 as i32 as u64)
                    .wrapping_add(
                        (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(32 as i32 as u64) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                (3 as i32 as u64)
                    .wrapping_add(
                        (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(48 as i32 as u64) as libc::c_short,
                (3 as i32 as u64)
                    .wrapping_add(
                        (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(64 as i32 as u64) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                (3 as i32 as u64)
                    .wrapping_add(
                        (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(80 as i32 as u64) as libc::c_short,
                (3 as i32 as u64)
                    .wrapping_add(
                        (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(96 as i32 as u64) as libc::c_short,
                (3 as i32 as u64)
                    .wrapping_add(
                        (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(112 as i32 as u64) as libc::c_short,
                -(1 as i32) as libc::c_short,
                (3 as i32 as u64)
                    .wrapping_add(
                        (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(128 as i32 as u64) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                (3 as i32 as u64)
                    .wrapping_add(
                        (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(144 as i32 as u64) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                (3 as i32 as u64)
                    .wrapping_add(
                        (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(160 as i32 as u64) as libc::c_short,
                -(1 as i32) as libc::c_short,
                (3 as i32 as u64)
                    .wrapping_add(
                        (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(176 as i32 as u64) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                (3 as i32 as u64)
                    .wrapping_add(
                        (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(192 as i32 as u64) as libc::c_short,
                -(1 as i32) as libc::c_short,
                (3 as i32 as u64)
                    .wrapping_add(
                        (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(208 as i32 as u64) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                (3 as i32 as u64)
                    .wrapping_add(
                        (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(224 as i32 as u64) as libc::c_short,
                (3 as i32 as u64)
                    .wrapping_add(
                        (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(240 as i32 as u64) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                (3 as i32 as u64)
                    .wrapping_add(
                        (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(256 as i32 as u64) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                (3 as i32 as u64)
                    .wrapping_add(
                        (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(272 as i32 as u64) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                (3 as i32 as u64)
                    .wrapping_add(
                        (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(288 as i32 as u64) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                (3 as i32 as u64)
                    .wrapping_add(
                        (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(304 as i32 as u64) as libc::c_short,
                (3 as i32 as u64)
                    .wrapping_add(
                        (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(320 as i32 as u64) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                (3 as i32 as u64)
                    .wrapping_add(
                        (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(336 as i32 as u64) as libc::c_short,
                (3 as i32 as u64)
                    .wrapping_add(
                        (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(352 as i32 as u64) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                (3 as i32 as u64)
                    .wrapping_add(
                        (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(368 as i32 as u64) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                (3 as i32 as u64)
                    .wrapping_add(
                        (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(384 as i32 as u64) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
                -(1 as i32) as libc::c_short,
            ],
            level3: [
                0 as u32,
                0 as u32,
                0x7fffffe as u32,
                0x7fffffe as u32,
                0 as u32,
                0x4200400 as u32,
                0xff7fffff as u32,
                0xff7fffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xf7ffffff as u32,
                0xfffffff0 as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffefffff as u32,
                0x1ffffff as u32,
                0x3 as u32,
                0x1f as u32,
                0 as u32,
                0 as u32,
                0x20 as u32,
                0xbccf0000 as u32,
                0xffffd740 as u32,
                0xfffffffb as u32,
                0xffffffff as u32,
                0xffbfffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xfffffc03 as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xfffeffff as u32,
                0x7fffff as u32,
                0xffffffff as u32,
                0x1ff as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0xffffffff as u32,
                0xffff20bf as u32,
                0xf7ffffff as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0x3f3fffff as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0xffff01ff as u32,
                0xe7ffffff as u32,
                0 as u32,
                0 as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0 as u32,
                0 as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0x3f3fffff as u32,
                0xffffffff as u32,
                0xaaff3f3f as u32,
                0x3fffffff as u32,
                0xffffffff as u32,
                0x5fdfffff as u32,
                0xfcf1fdc as u32,
                0x1fdc1fff as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0x80020000 as u32,
                0x1fff0000 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0x3e2ffc84 as u32,
                0xf21fbd50 as u32,
                0x43e0 as u32,
                0xffffffff as u32,
                0x18 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0xffc00000 as u32,
                0xffffffff as u32,
                0x3ff as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xc781f as u32,
                0xffffffff as u32,
                0x20bf as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0xffffffff as u32,
                0x3fff as u32,
                0x3fffffff as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0xfffffffc as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffff78ff as u32,
                0xffffffff as u32,
                0x3eb07ff as u32,
                0x77c0000 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0xffff0000 as u32,
                0xf7ffffff as u32,
                0xffff03ff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0xf8007f as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0x7fffffe as u32,
                0x7fffffe as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffff as u32,
                0 as u32,
                0 as u32,
                0xffff0000 as u32,
                0xff0fffff as u32,
                0xfffffff as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0xf7ff0000 as u32,
                0xffb7f7ff as u32,
                0x1bfbfffb as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0xffffffb9 as u32,
                0x7fdffff as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0xffffffff as u32,
                0x7ffff as u32,
                0xffffffff as u32,
                0x7ffff as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffdfffff as u32,
                0xffffffff as u32,
                0xdfffffff as u32,
                0xebffde64 as u32,
                0xffffffef as u32,
                0xffffffff as u32,
                0xdfdfe7bf as u32,
                0x7bffffff as u32,
                0xfffdfc5f as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xffffff3f as u32,
                0xf7fffffd as u32,
                0xf7ffffff as u32,
                0xffdfffff as u32,
                0xffdfffff as u32,
                0xffff7fff as u32,
                0xffff7fff as u32,
                0xfffffdff as u32,
                0xfffffdff as u32,
                0xff7 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0x7ffffbff as u32,
                0x7e0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0xffff0000 as u32,
                0xffffffff as u32,
                0x3fff as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0xffffffff as u32,
                0xffffffff as u32,
                0xf as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0xffff0000 as u32,
                0xffff03ff as u32,
                0xffff03ff as u32,
                0x3ff as u32,
                0 as u32,
                0 as u32,
                0 as u32,
            ],
        };
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];