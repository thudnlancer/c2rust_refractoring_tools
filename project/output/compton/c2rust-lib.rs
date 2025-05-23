#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(asm)]
#![feature(extern_types)]
#[macro_use]
extern crate c2rust_bitfields;
#[macro_use]
extern crate c2rust_asm_casts;
extern crate libc;
pub mod src {
    pub mod c2;
    pub mod compton;
    pub mod dbus;
    pub mod opengl;
}