#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(label_break_value)]
extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct twofish_ctx {
    pub keys: [uint32_t; 40],
    pub s_box: [[uint32_t; 256]; 4],
}
static mut q0: [uint8_t; 256] = [
    0xa9 as i32 as uint8_t,
    0x67 as i32 as uint8_t,
    0xb3 as i32 as uint8_t,
    0xe8 as i32 as uint8_t,
    0x4 as i32 as uint8_t,
    0xfd as i32 as uint8_t,
    0xa3 as i32 as uint8_t,
    0x76 as i32 as uint8_t,
    0x9a as i32 as uint8_t,
    0x92 as i32 as uint8_t,
    0x80 as i32 as uint8_t,
    0x78 as i32 as uint8_t,
    0xe4 as i32 as uint8_t,
    0xdd as i32 as uint8_t,
    0xd1 as i32 as uint8_t,
    0x38 as i32 as uint8_t,
    0xd as i32 as uint8_t,
    0xc6 as i32 as uint8_t,
    0x35 as i32 as uint8_t,
    0x98 as i32 as uint8_t,
    0x18 as i32 as uint8_t,
    0xf7 as i32 as uint8_t,
    0xec as i32 as uint8_t,
    0x6c as i32 as uint8_t,
    0x43 as i32 as uint8_t,
    0x75 as i32 as uint8_t,
    0x37 as i32 as uint8_t,
    0x26 as i32 as uint8_t,
    0xfa as i32 as uint8_t,
    0x13 as i32 as uint8_t,
    0x94 as i32 as uint8_t,
    0x48 as i32 as uint8_t,
    0xf2 as i32 as uint8_t,
    0xd0 as i32 as uint8_t,
    0x8b as i32 as uint8_t,
    0x30 as i32 as uint8_t,
    0x84 as i32 as uint8_t,
    0x54 as i32 as uint8_t,
    0xdf as i32 as uint8_t,
    0x23 as i32 as uint8_t,
    0x19 as i32 as uint8_t,
    0x5b as i32 as uint8_t,
    0x3d as i32 as uint8_t,
    0x59 as i32 as uint8_t,
    0xf3 as i32 as uint8_t,
    0xae as i32 as uint8_t,
    0xa2 as i32 as uint8_t,
    0x82 as i32 as uint8_t,
    0x63 as i32 as uint8_t,
    0x1 as i32 as uint8_t,
    0x83 as i32 as uint8_t,
    0x2e as i32 as uint8_t,
    0xd9 as i32 as uint8_t,
    0x51 as i32 as uint8_t,
    0x9b as i32 as uint8_t,
    0x7c as i32 as uint8_t,
    0xa6 as i32 as uint8_t,
    0xeb as i32 as uint8_t,
    0xa5 as i32 as uint8_t,
    0xbe as i32 as uint8_t,
    0x16 as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xe3 as i32 as uint8_t,
    0x61 as i32 as uint8_t,
    0xc0 as i32 as uint8_t,
    0x8c as i32 as uint8_t,
    0x3a as i32 as uint8_t,
    0xf5 as i32 as uint8_t,
    0x73 as i32 as uint8_t,
    0x2c as i32 as uint8_t,
    0x25 as i32 as uint8_t,
    0xb as i32 as uint8_t,
    0xbb as i32 as uint8_t,
    0x4e as i32 as uint8_t,
    0x89 as i32 as uint8_t,
    0x6b as i32 as uint8_t,
    0x53 as i32 as uint8_t,
    0x6a as i32 as uint8_t,
    0xb4 as i32 as uint8_t,
    0xf1 as i32 as uint8_t,
    0xe1 as i32 as uint8_t,
    0xe6 as i32 as uint8_t,
    0xbd as i32 as uint8_t,
    0x45 as i32 as uint8_t,
    0xe2 as i32 as uint8_t,
    0xf4 as i32 as uint8_t,
    0xb6 as i32 as uint8_t,
    0x66 as i32 as uint8_t,
    0xcc as i32 as uint8_t,
    0x95 as i32 as uint8_t,
    0x3 as i32 as uint8_t,
    0x56 as i32 as uint8_t,
    0xd4 as i32 as uint8_t,
    0x1c as i32 as uint8_t,
    0x1e as i32 as uint8_t,
    0xd7 as i32 as uint8_t,
    0xfb as i32 as uint8_t,
    0xc3 as i32 as uint8_t,
    0x8e as i32 as uint8_t,
    0xb5 as i32 as uint8_t,
    0xe9 as i32 as uint8_t,
    0xcf as i32 as uint8_t,
    0xbf as i32 as uint8_t,
    0xba as i32 as uint8_t,
    0xea as i32 as uint8_t,
    0x77 as i32 as uint8_t,
    0x39 as i32 as uint8_t,
    0xaf as i32 as uint8_t,
    0x33 as i32 as uint8_t,
    0xc9 as i32 as uint8_t,
    0x62 as i32 as uint8_t,
    0x71 as i32 as uint8_t,
    0x81 as i32 as uint8_t,
    0x79 as i32 as uint8_t,
    0x9 as i32 as uint8_t,
    0xad as i32 as uint8_t,
    0x24 as i32 as uint8_t,
    0xcd as i32 as uint8_t,
    0xf9 as i32 as uint8_t,
    0xd8 as i32 as uint8_t,
    0xe5 as i32 as uint8_t,
    0xc5 as i32 as uint8_t,
    0xb9 as i32 as uint8_t,
    0x4d as i32 as uint8_t,
    0x44 as i32 as uint8_t,
    0x8 as i32 as uint8_t,
    0x86 as i32 as uint8_t,
    0xe7 as i32 as uint8_t,
    0xa1 as i32 as uint8_t,
    0x1d as i32 as uint8_t,
    0xaa as i32 as uint8_t,
    0xed as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x70 as i32 as uint8_t,
    0xb2 as i32 as uint8_t,
    0xd2 as i32 as uint8_t,
    0x41 as i32 as uint8_t,
    0x7b as i32 as uint8_t,
    0xa0 as i32 as uint8_t,
    0x11 as i32 as uint8_t,
    0x31 as i32 as uint8_t,
    0xc2 as i32 as uint8_t,
    0x27 as i32 as uint8_t,
    0x90 as i32 as uint8_t,
    0x20 as i32 as uint8_t,
    0xf6 as i32 as uint8_t,
    0x60 as i32 as uint8_t,
    0xff as i32 as uint8_t,
    0x96 as i32 as uint8_t,
    0x5c as i32 as uint8_t,
    0xb1 as i32 as uint8_t,
    0xab as i32 as uint8_t,
    0x9e as i32 as uint8_t,
    0x9c as i32 as uint8_t,
    0x52 as i32 as uint8_t,
    0x1b as i32 as uint8_t,
    0x5f as i32 as uint8_t,
    0x93 as i32 as uint8_t,
    0xa as i32 as uint8_t,
    0xef as i32 as uint8_t,
    0x91 as i32 as uint8_t,
    0x85 as i32 as uint8_t,
    0x49 as i32 as uint8_t,
    0xee as i32 as uint8_t,
    0x2d as i32 as uint8_t,
    0x4f as i32 as uint8_t,
    0x8f as i32 as uint8_t,
    0x3b as i32 as uint8_t,
    0x47 as i32 as uint8_t,
    0x87 as i32 as uint8_t,
    0x6d as i32 as uint8_t,
    0x46 as i32 as uint8_t,
    0xd6 as i32 as uint8_t,
    0x3e as i32 as uint8_t,
    0x69 as i32 as uint8_t,
    0x64 as i32 as uint8_t,
    0x2a as i32 as uint8_t,
    0xce as i32 as uint8_t,
    0xcb as i32 as uint8_t,
    0x2f as i32 as uint8_t,
    0xfc as i32 as uint8_t,
    0x97 as i32 as uint8_t,
    0x5 as i32 as uint8_t,
    0x7a as i32 as uint8_t,
    0xac as i32 as uint8_t,
    0x7f as i32 as uint8_t,
    0xd5 as i32 as uint8_t,
    0x1a as i32 as uint8_t,
    0x4b as i32 as uint8_t,
    0xe as i32 as uint8_t,
    0xa7 as i32 as uint8_t,
    0x5a as i32 as uint8_t,
    0x28 as i32 as uint8_t,
    0x14 as i32 as uint8_t,
    0x3f as i32 as uint8_t,
    0x29 as i32 as uint8_t,
    0x88 as i32 as uint8_t,
    0x3c as i32 as uint8_t,
    0x4c as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0xb8 as i32 as uint8_t,
    0xda as i32 as uint8_t,
    0xb0 as i32 as uint8_t,
    0x17 as i32 as uint8_t,
    0x55 as i32 as uint8_t,
    0x1f as i32 as uint8_t,
    0x8a as i32 as uint8_t,
    0x7d as i32 as uint8_t,
    0x57 as i32 as uint8_t,
    0xc7 as i32 as uint8_t,
    0x8d as i32 as uint8_t,
    0x74 as i32 as uint8_t,
    0xb7 as i32 as uint8_t,
    0xc4 as i32 as uint8_t,
    0x9f as i32 as uint8_t,
    0x72 as i32 as uint8_t,
    0x7e as i32 as uint8_t,
    0x15 as i32 as uint8_t,
    0x22 as i32 as uint8_t,
    0x12 as i32 as uint8_t,
    0x58 as i32 as uint8_t,
    0x7 as i32 as uint8_t,
    0x99 as i32 as uint8_t,
    0x34 as i32 as uint8_t,
    0x6e as i32 as uint8_t,
    0x50 as i32 as uint8_t,
    0xde as i32 as uint8_t,
    0x68 as i32 as uint8_t,
    0x65 as i32 as uint8_t,
    0xbc as i32 as uint8_t,
    0xdb as i32 as uint8_t,
    0xf8 as i32 as uint8_t,
    0xc8 as i32 as uint8_t,
    0xa8 as i32 as uint8_t,
    0x2b as i32 as uint8_t,
    0x40 as i32 as uint8_t,
    0xdc as i32 as uint8_t,
    0xfe as i32 as uint8_t,
    0x32 as i32 as uint8_t,
    0xa4 as i32 as uint8_t,
    0xca as i32 as uint8_t,
    0x10 as i32 as uint8_t,
    0x21 as i32 as uint8_t,
    0xf0 as i32 as uint8_t,
    0xd3 as i32 as uint8_t,
    0x5d as i32 as uint8_t,
    0xf as i32 as uint8_t,
    0 as i32 as uint8_t,
    0x6f as i32 as uint8_t,
    0x9d as i32 as uint8_t,
    0x36 as i32 as uint8_t,
    0x42 as i32 as uint8_t,
    0x4a as i32 as uint8_t,
    0x5e as i32 as uint8_t,
    0xc1 as i32 as uint8_t,
    0xe0 as i32 as uint8_t,
];
static mut q1: [uint8_t; 256] = [
    0x75 as i32 as uint8_t,
    0xf3 as i32 as uint8_t,
    0xc6 as i32 as uint8_t,
    0xf4 as i32 as uint8_t,
    0xdb as i32 as uint8_t,
    0x7b as i32 as uint8_t,
    0xfb as i32 as uint8_t,
    0xc8 as i32 as uint8_t,
    0x4a as i32 as uint8_t,
    0xd3 as i32 as uint8_t,
    0xe6 as i32 as uint8_t,
    0x6b as i32 as uint8_t,
    0x45 as i32 as uint8_t,
    0x7d as i32 as uint8_t,
    0xe8 as i32 as uint8_t,
    0x4b as i32 as uint8_t,
    0xd6 as i32 as uint8_t,
    0x32 as i32 as uint8_t,
    0xd8 as i32 as uint8_t,
    0xfd as i32 as uint8_t,
    0x37 as i32 as uint8_t,
    0x71 as i32 as uint8_t,
    0xf1 as i32 as uint8_t,
    0xe1 as i32 as uint8_t,
    0x30 as i32 as uint8_t,
    0xf as i32 as uint8_t,
    0xf8 as i32 as uint8_t,
    0x1b as i32 as uint8_t,
    0x87 as i32 as uint8_t,
    0xfa as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x3f as i32 as uint8_t,
    0x5e as i32 as uint8_t,
    0xba as i32 as uint8_t,
    0xae as i32 as uint8_t,
    0x5b as i32 as uint8_t,
    0x8a as i32 as uint8_t,
    0 as i32 as uint8_t,
    0xbc as i32 as uint8_t,
    0x9d as i32 as uint8_t,
    0x6d as i32 as uint8_t,
    0xc1 as i32 as uint8_t,
    0xb1 as i32 as uint8_t,
    0xe as i32 as uint8_t,
    0x80 as i32 as uint8_t,
    0x5d as i32 as uint8_t,
    0xd2 as i32 as uint8_t,
    0xd5 as i32 as uint8_t,
    0xa0 as i32 as uint8_t,
    0x84 as i32 as uint8_t,
    0x7 as i32 as uint8_t,
    0x14 as i32 as uint8_t,
    0xb5 as i32 as uint8_t,
    0x90 as i32 as uint8_t,
    0x2c as i32 as uint8_t,
    0xa3 as i32 as uint8_t,
    0xb2 as i32 as uint8_t,
    0x73 as i32 as uint8_t,
    0x4c as i32 as uint8_t,
    0x54 as i32 as uint8_t,
    0x92 as i32 as uint8_t,
    0x74 as i32 as uint8_t,
    0x36 as i32 as uint8_t,
    0x51 as i32 as uint8_t,
    0x38 as i32 as uint8_t,
    0xb0 as i32 as uint8_t,
    0xbd as i32 as uint8_t,
    0x5a as i32 as uint8_t,
    0xfc as i32 as uint8_t,
    0x60 as i32 as uint8_t,
    0x62 as i32 as uint8_t,
    0x96 as i32 as uint8_t,
    0x6c as i32 as uint8_t,
    0x42 as i32 as uint8_t,
    0xf7 as i32 as uint8_t,
    0x10 as i32 as uint8_t,
    0x7c as i32 as uint8_t,
    0x28 as i32 as uint8_t,
    0x27 as i32 as uint8_t,
    0x8c as i32 as uint8_t,
    0x13 as i32 as uint8_t,
    0x95 as i32 as uint8_t,
    0x9c as i32 as uint8_t,
    0xc7 as i32 as uint8_t,
    0x24 as i32 as uint8_t,
    0x46 as i32 as uint8_t,
    0x3b as i32 as uint8_t,
    0x70 as i32 as uint8_t,
    0xca as i32 as uint8_t,
    0xe3 as i32 as uint8_t,
    0x85 as i32 as uint8_t,
    0xcb as i32 as uint8_t,
    0x11 as i32 as uint8_t,
    0xd0 as i32 as uint8_t,
    0x93 as i32 as uint8_t,
    0xb8 as i32 as uint8_t,
    0xa6 as i32 as uint8_t,
    0x83 as i32 as uint8_t,
    0x20 as i32 as uint8_t,
    0xff as i32 as uint8_t,
    0x9f as i32 as uint8_t,
    0x77 as i32 as uint8_t,
    0xc3 as i32 as uint8_t,
    0xcc as i32 as uint8_t,
    0x3 as i32 as uint8_t,
    0x6f as i32 as uint8_t,
    0x8 as i32 as uint8_t,
    0xbf as i32 as uint8_t,
    0x40 as i32 as uint8_t,
    0xe7 as i32 as uint8_t,
    0x2b as i32 as uint8_t,
    0xe2 as i32 as uint8_t,
    0x79 as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xaa as i32 as uint8_t,
    0x82 as i32 as uint8_t,
    0x41 as i32 as uint8_t,
    0x3a as i32 as uint8_t,
    0xea as i32 as uint8_t,
    0xb9 as i32 as uint8_t,
    0xe4 as i32 as uint8_t,
    0x9a as i32 as uint8_t,
    0xa4 as i32 as uint8_t,
    0x97 as i32 as uint8_t,
    0x7e as i32 as uint8_t,
    0xda as i32 as uint8_t,
    0x7a as i32 as uint8_t,
    0x17 as i32 as uint8_t,
    0x66 as i32 as uint8_t,
    0x94 as i32 as uint8_t,
    0xa1 as i32 as uint8_t,
    0x1d as i32 as uint8_t,
    0x3d as i32 as uint8_t,
    0xf0 as i32 as uint8_t,
    0xde as i32 as uint8_t,
    0xb3 as i32 as uint8_t,
    0xb as i32 as uint8_t,
    0x72 as i32 as uint8_t,
    0xa7 as i32 as uint8_t,
    0x1c as i32 as uint8_t,
    0xef as i32 as uint8_t,
    0xd1 as i32 as uint8_t,
    0x53 as i32 as uint8_t,
    0x3e as i32 as uint8_t,
    0x8f as i32 as uint8_t,
    0x33 as i32 as uint8_t,
    0x26 as i32 as uint8_t,
    0x5f as i32 as uint8_t,
    0xec as i32 as uint8_t,
    0x76 as i32 as uint8_t,
    0x2a as i32 as uint8_t,
    0x49 as i32 as uint8_t,
    0x81 as i32 as uint8_t,
    0x88 as i32 as uint8_t,
    0xee as i32 as uint8_t,
    0x21 as i32 as uint8_t,
    0xc4 as i32 as uint8_t,
    0x1a as i32 as uint8_t,
    0xeb as i32 as uint8_t,
    0xd9 as i32 as uint8_t,
    0xc5 as i32 as uint8_t,
    0x39 as i32 as uint8_t,
    0x99 as i32 as uint8_t,
    0xcd as i32 as uint8_t,
    0xad as i32 as uint8_t,
    0x31 as i32 as uint8_t,
    0x8b as i32 as uint8_t,
    0x1 as i32 as uint8_t,
    0x18 as i32 as uint8_t,
    0x23 as i32 as uint8_t,
    0xdd as i32 as uint8_t,
    0x1f as i32 as uint8_t,
    0x4e as i32 as uint8_t,
    0x2d as i32 as uint8_t,
    0xf9 as i32 as uint8_t,
    0x48 as i32 as uint8_t,
    0x4f as i32 as uint8_t,
    0xf2 as i32 as uint8_t,
    0x65 as i32 as uint8_t,
    0x8e as i32 as uint8_t,
    0x78 as i32 as uint8_t,
    0x5c as i32 as uint8_t,
    0x58 as i32 as uint8_t,
    0x19 as i32 as uint8_t,
    0x8d as i32 as uint8_t,
    0xe5 as i32 as uint8_t,
    0x98 as i32 as uint8_t,
    0x57 as i32 as uint8_t,
    0x67 as i32 as uint8_t,
    0x7f as i32 as uint8_t,
    0x5 as i32 as uint8_t,
    0x64 as i32 as uint8_t,
    0xaf as i32 as uint8_t,
    0x63 as i32 as uint8_t,
    0xb6 as i32 as uint8_t,
    0xfe as i32 as uint8_t,
    0xf5 as i32 as uint8_t,
    0xb7 as i32 as uint8_t,
    0x3c as i32 as uint8_t,
    0xa5 as i32 as uint8_t,
    0xce as i32 as uint8_t,
    0xe9 as i32 as uint8_t,
    0x68 as i32 as uint8_t,
    0x44 as i32 as uint8_t,
    0xe0 as i32 as uint8_t,
    0x4d as i32 as uint8_t,
    0x43 as i32 as uint8_t,
    0x69 as i32 as uint8_t,
    0x29 as i32 as uint8_t,
    0x2e as i32 as uint8_t,
    0xac as i32 as uint8_t,
    0x15 as i32 as uint8_t,
    0x59 as i32 as uint8_t,
    0xa8 as i32 as uint8_t,
    0xa as i32 as uint8_t,
    0x9e as i32 as uint8_t,
    0x6e as i32 as uint8_t,
    0x47 as i32 as uint8_t,
    0xdf as i32 as uint8_t,
    0x34 as i32 as uint8_t,
    0x35 as i32 as uint8_t,
    0x6a as i32 as uint8_t,
    0xcf as i32 as uint8_t,
    0xdc as i32 as uint8_t,
    0x22 as i32 as uint8_t,
    0xc9 as i32 as uint8_t,
    0xc0 as i32 as uint8_t,
    0x9b as i32 as uint8_t,
    0x89 as i32 as uint8_t,
    0xd4 as i32 as uint8_t,
    0xed as i32 as uint8_t,
    0xab as i32 as uint8_t,
    0x12 as i32 as uint8_t,
    0xa2 as i32 as uint8_t,
    0xd as i32 as uint8_t,
    0x52 as i32 as uint8_t,
    0xbb as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2f as i32 as uint8_t,
    0xa9 as i32 as uint8_t,
    0xd7 as i32 as uint8_t,
    0x61 as i32 as uint8_t,
    0x1e as i32 as uint8_t,
    0xb4 as i32 as uint8_t,
    0x50 as i32 as uint8_t,
    0x4 as i32 as uint8_t,
    0xf6 as i32 as uint8_t,
    0xc2 as i32 as uint8_t,
    0x16 as i32 as uint8_t,
    0x25 as i32 as uint8_t,
    0x86 as i32 as uint8_t,
    0x56 as i32 as uint8_t,
    0x55 as i32 as uint8_t,
    0x9 as i32 as uint8_t,
    0xbe as i32 as uint8_t,
    0x91 as i32 as uint8_t,
];
unsafe extern "C" fn gf_multiply(
    mut p: uint8_t,
    mut a: uint8_t,
    mut b: uint8_t,
) -> uint32_t {
    let mut shift: uint32_t = b as uint32_t;
    let mut result: uint8_t = 0 as i32 as uint8_t;
    while a != 0 {
        if a as i32 & 1 as i32 != 0 {
            result = (result as u32 ^ shift) as uint8_t;
        }
        a = (a as i32 >> 1 as i32) as uint8_t;
        shift = shift << 1 as i32;
        if shift & 0x100 as i32 as u32 != 0 {
            shift ^= p as u32;
        }
    }
    return result as uint32_t;
}
static mut rs_matrix: [[uint8_t; 8]; 4] = [
    [
        0x1 as i32 as uint8_t,
        0xa4 as i32 as uint8_t,
        0x55 as i32 as uint8_t,
        0x87 as i32 as uint8_t,
        0x5a as i32 as uint8_t,
        0x58 as i32 as uint8_t,
        0xdb as i32 as uint8_t,
        0x9e as i32 as uint8_t,
    ],
    [
        0xa4 as i32 as uint8_t,
        0x56 as i32 as uint8_t,
        0x82 as i32 as uint8_t,
        0xf3 as i32 as uint8_t,
        0x1e as i32 as uint8_t,
        0xc6 as i32 as uint8_t,
        0x68 as i32 as uint8_t,
        0xe5 as i32 as uint8_t,
    ],
    [
        0x2 as i32 as uint8_t,
        0xa1 as i32 as uint8_t,
        0xfc as i32 as uint8_t,
        0xc1 as i32 as uint8_t,
        0x47 as i32 as uint8_t,
        0xae as i32 as uint8_t,
        0x3d as i32 as uint8_t,
        0x19 as i32 as uint8_t,
    ],
    [
        0xa4 as i32 as uint8_t,
        0x55 as i32 as uint8_t,
        0x87 as i32 as uint8_t,
        0x5a as i32 as uint8_t,
        0x58 as i32 as uint8_t,
        0xdb as i32 as uint8_t,
        0x9e as i32 as uint8_t,
        0x3 as i32 as uint8_t,
    ],
];
unsafe extern "C" fn compute_s(mut m1: uint32_t, mut m2: uint32_t) -> uint32_t {
    let mut s: uint32_t = 0 as i32 as uint32_t;
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 4 as i32 {
        s
            |= (gf_multiply(
                0x4d as i32 as uint8_t,
                m1 as uint8_t,
                rs_matrix[i as usize][0 as i32 as usize],
            )
                ^ gf_multiply(
                    0x4d as i32 as uint8_t,
                    (m1 >> 8 as i32) as uint8_t,
                    rs_matrix[i as usize][1 as i32 as usize],
                )
                ^ gf_multiply(
                    0x4d as i32 as uint8_t,
                    (m1 >> 16 as i32) as uint8_t,
                    rs_matrix[i as usize][2 as i32 as usize],
                )
                ^ gf_multiply(
                    0x4d as i32 as uint8_t,
                    (m1 >> 24 as i32) as uint8_t,
                    rs_matrix[i as usize][3 as i32 as usize],
                )
                ^ gf_multiply(
                    0x4d as i32 as uint8_t,
                    m2 as uint8_t,
                    rs_matrix[i as usize][4 as i32 as usize],
                )
                ^ gf_multiply(
                    0x4d as i32 as uint8_t,
                    (m2 >> 8 as i32) as uint8_t,
                    rs_matrix[i as usize][5 as i32 as usize],
                )
                ^ gf_multiply(
                    0x4d as i32 as uint8_t,
                    (m2 >> 16 as i32) as uint8_t,
                    rs_matrix[i as usize][6 as i32 as usize],
                )
                ^ gf_multiply(
                    0x4d as i32 as uint8_t,
                    (m2 >> 24 as i32) as uint8_t,
                    rs_matrix[i as usize][7 as i32 as usize],
                )) << i * 8 as i32;
        i += 1;
        i;
    }
    return s;
}
static mut q_table: [[*const uint8_t; 5]; 4] = unsafe {
    [
        [q1.as_ptr(), q1.as_ptr(), q0.as_ptr(), q0.as_ptr(), q1.as_ptr()],
        [q0.as_ptr(), q1.as_ptr(), q1.as_ptr(), q0.as_ptr(), q0.as_ptr()],
        [q0.as_ptr(), q0.as_ptr(), q0.as_ptr(), q1.as_ptr(), q1.as_ptr()],
        [q1.as_ptr(), q0.as_ptr(), q1.as_ptr(), q1.as_ptr(), q0.as_ptr()],
    ]
};
static mut mds_matrix: [[uint8_t; 4]; 4] = [
    [
        0x1 as i32 as uint8_t,
        0xef as i32 as uint8_t,
        0x5b as i32 as uint8_t,
        0x5b as i32 as uint8_t,
    ],
    [
        0x5b as i32 as uint8_t,
        0xef as i32 as uint8_t,
        0xef as i32 as uint8_t,
        0x1 as i32 as uint8_t,
    ],
    [
        0xef as i32 as uint8_t,
        0x5b as i32 as uint8_t,
        0x1 as i32 as uint8_t,
        0xef as i32 as uint8_t,
    ],
    [
        0xef as i32 as uint8_t,
        0x1 as i32 as uint8_t,
        0xef as i32 as uint8_t,
        0x5b as i32 as uint8_t,
    ],
];
unsafe extern "C" fn h_byte(
    mut k: i32,
    mut i: i32,
    mut x: uint8_t,
    mut l0: uint8_t,
    mut l1: uint8_t,
    mut l2: uint8_t,
    mut l3: uint8_t,
) -> uint32_t {
    let mut y: uint8_t = *(q_table[i as usize][4 as i32 as usize])
        .offset(
            (l0 as i32
                ^ *(q_table[i as usize][3 as i32 as usize])
                    .offset(
                        (l1 as i32
                            ^ *(q_table[i as usize][2 as i32 as usize])
                                .offset(
                                    (if k == 2 as i32 {
                                        x as i32
                                    } else {
                                        l2 as i32
                                            ^ *(q_table[i as usize][1 as i32 as usize])
                                                .offset(
                                                    (if k == 3 as i32 {
                                                        x as i32
                                                    } else {
                                                        l3 as i32
                                                            ^ *(q_table[i as usize][0 as i32 as usize])
                                                                .offset(x as isize) as i32
                                                    }) as isize,
                                                ) as i32
                                    }) as isize,
                                ) as i32) as isize,
                    ) as i32) as isize,
        );
    return gf_multiply(
        0x69 as i32 as uint8_t,
        mds_matrix[0 as i32 as usize][i as usize],
        y,
    )
        | gf_multiply(
            0x69 as i32 as uint8_t,
            mds_matrix[1 as i32 as usize][i as usize],
            y,
        ) << 8 as i32
        | gf_multiply(
            0x69 as i32 as uint8_t,
            mds_matrix[2 as i32 as usize][i as usize],
            y,
        ) << 16 as i32
        | gf_multiply(
            0x69 as i32 as uint8_t,
            mds_matrix[3 as i32 as usize][i as usize],
            y,
        ) << 24 as i32;
}
unsafe extern "C" fn h(
    mut k: i32,
    mut x: uint8_t,
    mut l0: uint32_t,
    mut l1: uint32_t,
    mut l2: uint32_t,
    mut l3: uint32_t,
) -> uint32_t {
    return h_byte(
        k,
        0 as i32,
        x,
        l0 as uint8_t,
        l1 as uint8_t,
        l2 as uint8_t,
        l3 as uint8_t,
    )
        ^ h_byte(
            k,
            1 as i32,
            x,
            (l0 >> 8 as i32) as uint8_t,
            (l1 >> 8 as i32) as uint8_t,
            (l2 >> 8 as i32) as uint8_t,
            (l3 >> 8 as i32) as uint8_t,
        )
        ^ h_byte(
            k,
            2 as i32,
            x,
            (l0 >> 16 as i32) as uint8_t,
            (l1 >> 16 as i32) as uint8_t,
            (l2 >> 16 as i32) as uint8_t,
            (l3 >> 16 as i32) as uint8_t,
        )
        ^ h_byte(
            k,
            3 as i32,
            x,
            (l0 >> 24 as i32) as uint8_t,
            (l1 >> 24 as i32) as uint8_t,
            (l2 >> 24 as i32) as uint8_t,
            (l3 >> 24 as i32) as uint8_t,
        );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_twofish_set_key(
    mut context: *mut twofish_ctx,
    mut keysize: size_t,
    mut key: *const uint8_t,
) {
    let mut key_copy: [uint8_t; 32] = [0; 32];
    let mut m: [uint32_t; 8] = [0; 8];
    let mut s: [uint32_t; 4] = [0; 4];
    let mut t: uint32_t = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    if keysize <= 32 as i32 as u64 {} else {
        __assert_fail(
            b"keysize <= 32\0" as *const u8 as *const i8,
            b"twofish.c\0" as *const u8 as *const i8,
            291 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 75],
                &[i8; 75],
            >(
                b"void nettle_twofish_set_key(struct twofish_ctx *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2694: {
        if keysize <= 32 as i32 as u64 {} else {
            __assert_fail(
                b"keysize <= 32\0" as *const u8 as *const i8,
                b"twofish.c\0" as *const u8 as *const i8,
                291 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 75],
                    &[i8; 75],
                >(
                    b"void nettle_twofish_set_key(struct twofish_ctx *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    memset(key_copy.as_mut_ptr() as *mut libc::c_void, 0 as i32, 32 as i32 as u64);
    memcpy(
        key_copy.as_mut_ptr() as *mut libc::c_void,
        key as *const libc::c_void,
        keysize,
    );
    i = 0 as i32;
    while i < 8 as i32 {
        m[i as usize] = (*key_copy
            .as_mut_ptr()
            .offset((i * 4 as i32) as isize)
            .offset(3 as i32 as isize) as uint32_t) << 24 as i32
            | (*key_copy
                .as_mut_ptr()
                .offset((i * 4 as i32) as isize)
                .offset(2 as i32 as isize) as uint32_t) << 16 as i32
            | (*key_copy
                .as_mut_ptr()
                .offset((i * 4 as i32) as isize)
                .offset(1 as i32 as isize) as uint32_t) << 8 as i32
            | *key_copy
                .as_mut_ptr()
                .offset((i * 4 as i32) as isize)
                .offset(0 as i32 as isize) as uint32_t;
        i += 1;
        i;
    }
    if keysize <= 16 as i32 as u64 {
        k = 2 as i32;
    } else if keysize <= 24 as i32 as u64 {
        k = 3 as i32;
    } else {
        k = 4 as i32;
    }
    i = 0 as i32;
    while i < 20 as i32 {
        t = h(
            k,
            (2 as i32 * i + 1 as i32) as uint8_t,
            m[1 as i32 as usize],
            m[3 as i32 as usize],
            m[5 as i32 as usize],
            m[7 as i32 as usize],
        );
        t = t << 8 as i32 | (t & 0xff000000 as u32) >> 24 as i32;
        (*context).keys[(2 as i32 * i) as usize] = t
            .wrapping_add(
                h(
                    k,
                    (2 as i32 * i) as uint8_t,
                    m[0 as i32 as usize],
                    m[2 as i32 as usize],
                    m[4 as i32 as usize],
                    m[6 as i32 as usize],
                ),
            );
        t = (t as u32).wrapping_add((*context).keys[(2 as i32 * i) as usize]) as uint32_t
            as uint32_t;
        t = t << 9 as i32 | (t & 0xff800000 as u32) >> 23 as i32;
        (*context).keys[(2 as i32 * i + 1 as i32) as usize] = t;
        i += 1;
        i;
    }
    i = 0 as i32;
    while i < k {
        s[(k - 1 as i32 - i) as usize] = compute_s(
            m[(2 as i32 * i) as usize],
            m[(2 as i32 * i + 1 as i32) as usize],
        );
        i += 1;
        i;
    }
    i = 0 as i32;
    while i < 4 as i32 {
        j = 0 as i32;
        while j < 256 as i32 {
            (*context).s_box[i as usize][j as usize] = h_byte(
                k,
                i,
                j as uint8_t,
                (s[0 as i32 as usize] >> i * 8 as i32) as uint8_t,
                (s[1 as i32 as usize] >> i * 8 as i32) as uint8_t,
                (s[2 as i32 as usize] >> i * 8 as i32) as uint8_t,
                (s[3 as i32 as usize] >> i * 8 as i32) as uint8_t,
            );
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn nettle_twofish128_set_key(
    mut context: *mut twofish_ctx,
    mut key: *const uint8_t,
) {
    nettle_twofish_set_key(context, 16 as i32 as size_t, key);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_twofish192_set_key(
    mut context: *mut twofish_ctx,
    mut key: *const uint8_t,
) {
    nettle_twofish_set_key(context, 24 as i32 as size_t, key);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_twofish256_set_key(
    mut context: *mut twofish_ctx,
    mut key: *const uint8_t,
) {
    nettle_twofish_set_key(context, 32 as i32 as size_t, key);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_twofish_encrypt(
    mut context: *const twofish_ctx,
    mut length: size_t,
    mut ciphertext: *mut uint8_t,
    mut plaintext: *const uint8_t,
) {
    let mut keys: *const uint32_t = ((*context).keys).as_ptr();
    let mut s_box: *const [uint32_t; 256] = ((*context).s_box).as_ptr();
    if length.wrapping_rem(16 as i32 as u64) == 0 {} else {
        __assert_fail(
            b"!(length % TWOFISH_BLOCK_SIZE)\0" as *const u8 as *const i8,
            b"twofish.c\0" as *const u8 as *const i8,
            369 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 92],
                &[i8; 92],
            >(
                b"void nettle_twofish_encrypt(const struct twofish_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3543: {
        if length.wrapping_rem(16 as i32 as u64) == 0 {} else {
            __assert_fail(
                b"!(length % TWOFISH_BLOCK_SIZE)\0" as *const u8 as *const i8,
                b"twofish.c\0" as *const u8 as *const i8,
                369 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 92],
                    &[i8; 92],
                >(
                    b"void nettle_twofish_encrypt(const struct twofish_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while length != 0 {
        let mut words: [uint32_t; 4] = [0; 4];
        let mut r0: uint32_t = 0;
        let mut r1: uint32_t = 0;
        let mut r2: uint32_t = 0;
        let mut r3: uint32_t = 0;
        let mut t0: uint32_t = 0;
        let mut t1: uint32_t = 0;
        let mut i: i32 = 0;
        i = 0 as i32;
        while i < 4 as i32 {
            words[i as usize] = (*plaintext.offset(3 as i32 as isize) as uint32_t)
                << 24 as i32
                | (*plaintext.offset(2 as i32 as isize) as uint32_t) << 16 as i32
                | (*plaintext.offset(1 as i32 as isize) as uint32_t) << 8 as i32
                | *plaintext.offset(0 as i32 as isize) as uint32_t;
            i += 1;
            i;
            plaintext = plaintext.offset(4 as i32 as isize);
        }
        r0 = words[0 as i32 as usize] ^ *keys.offset(0 as i32 as isize);
        r1 = words[1 as i32 as usize] ^ *keys.offset(1 as i32 as isize);
        r2 = words[2 as i32 as usize] ^ *keys.offset(2 as i32 as isize);
        r3 = words[3 as i32 as usize] ^ *keys.offset(3 as i32 as isize);
        i = 0 as i32;
        while i < 8 as i32 {
            t1 = (*s_box.offset(1 as i32 as isize))[(r1 & 0xff as i32 as u32) as usize]
                ^ (*s_box
                    .offset(
                        2 as i32 as isize,
                    ))[(r1 >> 8 as i32 & 0xff as i32 as u32) as usize]
                ^ (*s_box
                    .offset(
                        3 as i32 as isize,
                    ))[(r1 >> 16 as i32 & 0xff as i32 as u32) as usize]
                ^ (*s_box
                    .offset(
                        0 as i32 as isize,
                    ))[(r1 >> 24 as i32 & 0xff as i32 as u32) as usize];
            t0 = ((*s_box.offset(0 as i32 as isize))[(r0 & 0xff as i32 as u32) as usize]
                ^ (*s_box
                    .offset(
                        1 as i32 as isize,
                    ))[(r0 >> 8 as i32 & 0xff as i32 as u32) as usize]
                ^ (*s_box
                    .offset(
                        2 as i32 as isize,
                    ))[(r0 >> 16 as i32 & 0xff as i32 as u32) as usize]
                ^ (*s_box
                    .offset(
                        3 as i32 as isize,
                    ))[(r0 >> 24 as i32 & 0xff as i32 as u32) as usize])
                .wrapping_add(t1);
            r3 = t1
                .wrapping_add(t0)
                .wrapping_add(*keys.offset((4 as i32 * i + 9 as i32) as isize))
                ^ (r3 << 1 as i32 | (r3 & 0x80000000 as u32) >> 31 as i32);
            r2 = t0.wrapping_add(*keys.offset((4 as i32 * i + 8 as i32) as isize)) ^ r2;
            r2 = r2 >> 1 as i32 | (r2 & 0x1 as i32 as u32) << 31 as i32;
            t1 = (*s_box.offset(1 as i32 as isize))[(r3 & 0xff as i32 as u32) as usize]
                ^ (*s_box
                    .offset(
                        2 as i32 as isize,
                    ))[(r3 >> 8 as i32 & 0xff as i32 as u32) as usize]
                ^ (*s_box
                    .offset(
                        3 as i32 as isize,
                    ))[(r3 >> 16 as i32 & 0xff as i32 as u32) as usize]
                ^ (*s_box
                    .offset(
                        0 as i32 as isize,
                    ))[(r3 >> 24 as i32 & 0xff as i32 as u32) as usize];
            t0 = ((*s_box.offset(0 as i32 as isize))[(r2 & 0xff as i32 as u32) as usize]
                ^ (*s_box
                    .offset(
                        1 as i32 as isize,
                    ))[(r2 >> 8 as i32 & 0xff as i32 as u32) as usize]
                ^ (*s_box
                    .offset(
                        2 as i32 as isize,
                    ))[(r2 >> 16 as i32 & 0xff as i32 as u32) as usize]
                ^ (*s_box
                    .offset(
                        3 as i32 as isize,
                    ))[(r2 >> 24 as i32 & 0xff as i32 as u32) as usize])
                .wrapping_add(t1);
            r1 = t1
                .wrapping_add(t0)
                .wrapping_add(*keys.offset((4 as i32 * i + 11 as i32) as isize))
                ^ (r1 << 1 as i32 | (r1 & 0x80000000 as u32) >> 31 as i32);
            r0 = t0.wrapping_add(*keys.offset((4 as i32 * i + 10 as i32) as isize)) ^ r0;
            r0 = r0 >> 1 as i32 | (r0 & 0x1 as i32 as u32) << 31 as i32;
            i += 1;
            i;
        }
        words[0 as i32 as usize] = r2 ^ *keys.offset(4 as i32 as isize);
        words[1 as i32 as usize] = r3 ^ *keys.offset(5 as i32 as isize);
        words[2 as i32 as usize] = r0 ^ *keys.offset(6 as i32 as isize);
        words[3 as i32 as usize] = r1 ^ *keys.offset(7 as i32 as isize);
        i = 0 as i32;
        while i < 4 as i32 {
            *ciphertext.offset(3 as i32 as isize) = (words[i as usize] >> 24 as i32
                & 0xff as i32 as u32) as uint8_t;
            *ciphertext.offset(2 as i32 as isize) = (words[i as usize] >> 16 as i32
                & 0xff as i32 as u32) as uint8_t;
            *ciphertext.offset(1 as i32 as isize) = (words[i as usize] >> 8 as i32
                & 0xff as i32 as u32) as uint8_t;
            *ciphertext.offset(0 as i32 as isize) = (words[i as usize]
                & 0xff as i32 as u32) as uint8_t;
            i += 1;
            i;
            ciphertext = ciphertext.offset(4 as i32 as isize);
        }
        length = (length as u64).wrapping_sub(16 as i32 as u64) as size_t as size_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn nettle_twofish_decrypt(
    mut context: *const twofish_ctx,
    mut length: size_t,
    mut plaintext: *mut uint8_t,
    mut ciphertext: *const uint8_t,
) {
    let mut keys: *const uint32_t = ((*context).keys).as_ptr();
    let mut s_box: *const [uint32_t; 256] = ((*context).s_box).as_ptr();
    if length.wrapping_rem(16 as i32 as u64) == 0 {} else {
        __assert_fail(
            b"!(length % TWOFISH_BLOCK_SIZE)\0" as *const u8 as *const i8,
            b"twofish.c\0" as *const u8 as *const i8,
            440 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 92],
                &[i8; 92],
            >(
                b"void nettle_twofish_decrypt(const struct twofish_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4335: {
        if length.wrapping_rem(16 as i32 as u64) == 0 {} else {
            __assert_fail(
                b"!(length % TWOFISH_BLOCK_SIZE)\0" as *const u8 as *const i8,
                b"twofish.c\0" as *const u8 as *const i8,
                440 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 92],
                    &[i8; 92],
                >(
                    b"void nettle_twofish_decrypt(const struct twofish_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while length != 0 {
        let mut words: [uint32_t; 4] = [0; 4];
        let mut r0: uint32_t = 0;
        let mut r1: uint32_t = 0;
        let mut r2: uint32_t = 0;
        let mut r3: uint32_t = 0;
        let mut t0: uint32_t = 0;
        let mut t1: uint32_t = 0;
        let mut i: i32 = 0;
        i = 0 as i32;
        while i < 4 as i32 {
            words[i as usize] = (*ciphertext.offset(3 as i32 as isize) as uint32_t)
                << 24 as i32
                | (*ciphertext.offset(2 as i32 as isize) as uint32_t) << 16 as i32
                | (*ciphertext.offset(1 as i32 as isize) as uint32_t) << 8 as i32
                | *ciphertext.offset(0 as i32 as isize) as uint32_t;
            i += 1;
            i;
            ciphertext = ciphertext.offset(4 as i32 as isize);
        }
        r0 = words[2 as i32 as usize] ^ *keys.offset(6 as i32 as isize);
        r1 = words[3 as i32 as usize] ^ *keys.offset(7 as i32 as isize);
        r2 = words[0 as i32 as usize] ^ *keys.offset(4 as i32 as isize);
        r3 = words[1 as i32 as usize] ^ *keys.offset(5 as i32 as isize);
        i = 0 as i32;
        while i < 8 as i32 {
            t1 = (*s_box.offset(1 as i32 as isize))[(r3 & 0xff as i32 as u32) as usize]
                ^ (*s_box
                    .offset(
                        2 as i32 as isize,
                    ))[(r3 >> 8 as i32 & 0xff as i32 as u32) as usize]
                ^ (*s_box
                    .offset(
                        3 as i32 as isize,
                    ))[(r3 >> 16 as i32 & 0xff as i32 as u32) as usize]
                ^ (*s_box
                    .offset(
                        0 as i32 as isize,
                    ))[(r3 >> 24 as i32 & 0xff as i32 as u32) as usize];
            t0 = ((*s_box.offset(0 as i32 as isize))[(r2 & 0xff as i32 as u32) as usize]
                ^ (*s_box
                    .offset(
                        1 as i32 as isize,
                    ))[(r2 >> 8 as i32 & 0xff as i32 as u32) as usize]
                ^ (*s_box
                    .offset(
                        2 as i32 as isize,
                    ))[(r2 >> 16 as i32 & 0xff as i32 as u32) as usize]
                ^ (*s_box
                    .offset(
                        3 as i32 as isize,
                    ))[(r2 >> 24 as i32 & 0xff as i32 as u32) as usize])
                .wrapping_add(t1);
            r1 = t1
                .wrapping_add(t0)
                .wrapping_add(*keys.offset((39 as i32 - 4 as i32 * i) as isize)) ^ r1;
            r1 = r1 >> 1 as i32 | (r1 & 0x1 as i32 as u32) << 31 as i32;
            r0 = t0.wrapping_add(*keys.offset((38 as i32 - 4 as i32 * i) as isize))
                ^ (r0 << 1 as i32 | (r0 & 0x80000000 as u32) >> 31 as i32);
            t1 = (*s_box.offset(1 as i32 as isize))[(r1 & 0xff as i32 as u32) as usize]
                ^ (*s_box
                    .offset(
                        2 as i32 as isize,
                    ))[(r1 >> 8 as i32 & 0xff as i32 as u32) as usize]
                ^ (*s_box
                    .offset(
                        3 as i32 as isize,
                    ))[(r1 >> 16 as i32 & 0xff as i32 as u32) as usize]
                ^ (*s_box
                    .offset(
                        0 as i32 as isize,
                    ))[(r1 >> 24 as i32 & 0xff as i32 as u32) as usize];
            t0 = ((*s_box.offset(0 as i32 as isize))[(r0 & 0xff as i32 as u32) as usize]
                ^ (*s_box
                    .offset(
                        1 as i32 as isize,
                    ))[(r0 >> 8 as i32 & 0xff as i32 as u32) as usize]
                ^ (*s_box
                    .offset(
                        2 as i32 as isize,
                    ))[(r0 >> 16 as i32 & 0xff as i32 as u32) as usize]
                ^ (*s_box
                    .offset(
                        3 as i32 as isize,
                    ))[(r0 >> 24 as i32 & 0xff as i32 as u32) as usize])
                .wrapping_add(t1);
            r3 = t1
                .wrapping_add(t0)
                .wrapping_add(*keys.offset((37 as i32 - 4 as i32 * i) as isize)) ^ r3;
            r3 = r3 >> 1 as i32 | (r3 & 0x1 as i32 as u32) << 31 as i32;
            r2 = t0.wrapping_add(*keys.offset((36 as i32 - 4 as i32 * i) as isize))
                ^ (r2 << 1 as i32 | (r2 & 0x80000000 as u32) >> 31 as i32);
            i += 1;
            i;
        }
        words[0 as i32 as usize] = r0 ^ *keys.offset(0 as i32 as isize);
        words[1 as i32 as usize] = r1 ^ *keys.offset(1 as i32 as isize);
        words[2 as i32 as usize] = r2 ^ *keys.offset(2 as i32 as isize);
        words[3 as i32 as usize] = r3 ^ *keys.offset(3 as i32 as isize);
        i = 0 as i32;
        while i < 4 as i32 {
            *plaintext.offset(3 as i32 as isize) = (words[i as usize] >> 24 as i32
                & 0xff as i32 as u32) as uint8_t;
            *plaintext.offset(2 as i32 as isize) = (words[i as usize] >> 16 as i32
                & 0xff as i32 as u32) as uint8_t;
            *plaintext.offset(1 as i32 as isize) = (words[i as usize] >> 8 as i32
                & 0xff as i32 as u32) as uint8_t;
            *plaintext.offset(0 as i32 as isize) = (words[i as usize]
                & 0xff as i32 as u32) as uint8_t;
            i += 1;
            i;
            plaintext = plaintext.offset(4 as i32 as isize);
        }
        length = (length as u64).wrapping_sub(16 as i32 as u64) as size_t as size_t;
    }
}