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
pub struct uc_property_t {
    pub test_fn: Option<unsafe extern "C" fn(ucs4_t) -> bool>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub header: [i32; 1],
    pub level1: [i32; 2],
    pub level2: [libc::c_short; 256],
    pub level3: [u32; 176],
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
static mut u_property_soft_dotted: C2RustUnnamed = C2RustUnnamed {
    header: [0; 1],
    level1: [0; 2],
    level2: [0; 256],
    level3: [0; 176],
};
#[no_mangle]
pub unsafe extern "C" fn uc_is_property_soft_dotted(mut uc: ucs4_t) -> bool {
    return bitmap_lookup(
        &u_property_soft_dotted as *const C2RustUnnamed as *const libc::c_void,
        uc,
    ) != 0;
}
#[no_mangle]
pub static mut UC_PROPERTY_SOFT_DOTTED: uc_property_t = {
    let mut init = uc_property_t {
        test_fn: Some(uc_is_property_soft_dotted as unsafe extern "C" fn(ucs4_t) -> bool),
    };
    init
};
unsafe extern "C" fn run_static_initializers() {
    u_property_soft_dotted = {
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
                    .wrapping_add(48 as i32 as u64) as libc::c_short,
                (3 as i32 as u64)
                    .wrapping_add(
                        (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(64 as i32 as u64) as libc::c_short,
                (3 as i32 as u64)
                    .wrapping_add(
                        (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(80 as i32 as u64) as libc::c_short,
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
                    .wrapping_add(96 as i32 as u64) as libc::c_short,
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
                    .wrapping_add(112 as i32 as u64) as libc::c_short,
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
                (3 as i32 as u64)
                    .wrapping_add(
                        (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as u64)
                            .wrapping_div(::core::mem::size_of::<i32>() as u64),
                    )
                    .wrapping_add(160 as i32 as u64) as libc::c_short,
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
            ],
            level3: [
                0 as u32,
                0 as u32,
                0 as u32,
                0x600 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0x8000 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0x200 as u32,
                0x100 as u32,
                0x20000000 as u32,
                0x40000 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0x80000 as u32,
                0 as u32,
                0 as u32,
                0x1400000 as u32,
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
                0 as u32,
                0 as u32,
                0 as u32,
                0x4 as u32,
                0x400000 as u32,
                0x110 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0x2000 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0x800 as u32,
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
                0x20000 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0x300 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0x10000000 as u32,
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
                0xc as u32,
                0xc00000 as u32,
                0 as u32,
                0xc00 as u32,
                0xc0000000 as u32,
                0 as u32,
                0xc0000 as u32,
                0 as u32,
                0xc0 as u32,
                0xc000000 as u32,
                0 as u32,
                0xc000 as u32,
                0 as u32,
                0xc as u32,
                0xc00000 as u32,
                0 as u32,
                0xc00 as u32,
                0xc0000000 as u32,
                0 as u32,
                0xc0000 as u32,
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
                0x4000000 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0 as u32,
                0x3000 as u32,
                0x100 as u32,
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