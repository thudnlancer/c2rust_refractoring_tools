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
    fn nettle_base64_encode_init(ctx: *mut base64_encode_ctx);
    fn nettle_base64_encode_update(
        ctx: *mut base64_encode_ctx,
        dst: *mut i8,
        length: size_t,
        src: *const uint8_t,
    ) -> size_t;
    fn nettle_base64_encode_final(ctx: *mut base64_encode_ctx, dst: *mut i8) -> size_t;
    fn nettle_base64_decode_init(ctx: *mut base64_decode_ctx);
    fn nettle_base64_decode_update(
        ctx: *mut base64_decode_ctx,
        dst_length: *mut size_t,
        dst: *mut uint8_t,
        src_length: size_t,
        src: *const i8,
    ) -> i32;
    fn nettle_base64_decode_final(ctx: *mut base64_decode_ctx) -> i32;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
pub type nettle_armor_length_func = unsafe extern "C" fn(size_t) -> size_t;
pub type nettle_armor_init_func = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type nettle_armor_encode_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut i8,
    size_t,
    *const uint8_t,
) -> size_t;
pub type nettle_armor_encode_final_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut i8,
) -> size_t;
pub type nettle_armor_decode_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut size_t,
    *mut uint8_t,
    size_t,
    *const i8,
) -> i32;
pub type nettle_armor_decode_final_func = unsafe extern "C" fn(*mut libc::c_void) -> i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_armor {
    pub name: *const i8,
    pub encode_context_size: u32,
    pub decode_context_size: u32,
    pub encode_final_length: u32,
    pub encode_init: Option<nettle_armor_init_func>,
    pub encode_length: Option<nettle_armor_length_func>,
    pub encode_update: Option<nettle_armor_encode_update_func>,
    pub encode_final: Option<nettle_armor_encode_final_func>,
    pub decode_init: Option<nettle_armor_init_func>,
    pub decode_length: Option<nettle_armor_length_func>,
    pub decode_update: Option<nettle_armor_decode_update_func>,
    pub decode_final: Option<nettle_armor_decode_final_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base64_decode_ctx {
    pub table: *const libc::c_schar,
    pub word: libc::c_ushort,
    pub bits: u8,
    pub padding: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base64_encode_ctx {
    pub alphabet: *const i8,
    pub word: libc::c_ushort,
    pub bits: u8,
}
unsafe extern "C" fn base64_encode_length(mut length: size_t) -> size_t {
    return length
        .wrapping_mul(8 as i32 as u64)
        .wrapping_add(4 as i32 as u64)
        .wrapping_div(6 as i32 as u64);
}
unsafe extern "C" fn base64_decode_length(mut length: size_t) -> size_t {
    return length
        .wrapping_add(1 as i32 as u64)
        .wrapping_mul(6 as i32 as u64)
        .wrapping_div(8 as i32 as u64);
}
#[no_mangle]
pub static mut nettle_base64: nettle_armor = unsafe {
    {
        let mut init = nettle_armor {
            name: b"base64\0" as *const u8 as *const i8,
            encode_context_size: ::core::mem::size_of::<base64_encode_ctx>() as u64
                as u32,
            decode_context_size: ::core::mem::size_of::<base64_decode_ctx>() as u64
                as u32,
            encode_final_length: 3 as i32 as u32,
            encode_init: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut base64_encode_ctx) -> ()>,
                Option<nettle_armor_init_func>,
            >(
                Some(
                    nettle_base64_encode_init
                        as unsafe extern "C" fn(*mut base64_encode_ctx) -> (),
                ),
            ),
            encode_length: Some(base64_encode_length as nettle_armor_length_func),
            encode_update: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut base64_encode_ctx,
                        *mut i8,
                        size_t,
                        *const uint8_t,
                    ) -> size_t,
                >,
                Option<nettle_armor_encode_update_func>,
            >(
                Some(
                    nettle_base64_encode_update
                        as unsafe extern "C" fn(
                            *mut base64_encode_ctx,
                            *mut i8,
                            size_t,
                            *const uint8_t,
                        ) -> size_t,
                ),
            ),
            encode_final: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut base64_encode_ctx, *mut i8) -> size_t>,
                Option<nettle_armor_encode_final_func>,
            >(
                Some(
                    nettle_base64_encode_final
                        as unsafe extern "C" fn(
                            *mut base64_encode_ctx,
                            *mut i8,
                        ) -> size_t,
                ),
            ),
            decode_init: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut base64_decode_ctx) -> ()>,
                Option<nettle_armor_init_func>,
            >(
                Some(
                    nettle_base64_decode_init
                        as unsafe extern "C" fn(*mut base64_decode_ctx) -> (),
                ),
            ),
            decode_length: Some(base64_decode_length as nettle_armor_length_func),
            decode_update: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut base64_decode_ctx,
                        *mut size_t,
                        *mut uint8_t,
                        size_t,
                        *const i8,
                    ) -> i32,
                >,
                Option<nettle_armor_decode_update_func>,
            >(
                Some(
                    nettle_base64_decode_update
                        as unsafe extern "C" fn(
                            *mut base64_decode_ctx,
                            *mut size_t,
                            *mut uint8_t,
                            size_t,
                            *const i8,
                        ) -> i32,
                ),
            ),
            decode_final: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut base64_decode_ctx) -> i32>,
                Option<nettle_armor_decode_final_func>,
            >(
                Some(
                    nettle_base64_decode_final
                        as unsafe extern "C" fn(*mut base64_decode_ctx) -> i32,
                ),
            ),
        };
        init
    }
};