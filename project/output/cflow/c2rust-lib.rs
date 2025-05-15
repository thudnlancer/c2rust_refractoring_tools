#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]
#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
pub mod gnu {
    pub mod argp_ba;
    pub mod argp_eexst;
    pub mod argp_fmtstream;
    pub mod argp_fs_xinl;
    pub mod argp_help;
    pub mod argp_parse;
    pub mod argp_pin;
    pub mod argp_pv;
    pub mod argp_pvh;
    pub mod argp_version_etc;
    pub mod argp_xinl;
    pub mod asnprintf;
    pub mod basename_lgpl;
    pub mod dirname_lgpl;
    pub mod exitfail;
    pub mod getopt;
    pub mod getopt1;
    pub mod hash;
    pub mod printf_args;
    pub mod printf_parse;
    pub mod progname;
    pub mod stripslash;
    pub mod vasnprintf;
    pub mod version_etc;
    pub mod xalloc_die;
    pub mod xmalloc;
}
pub mod src {
    pub mod argcv;
    pub mod c;
    pub mod depmap;
    pub mod gnu;
    pub mod linked_list;
    pub mod main;
    pub mod output;
    pub mod parser;
    pub mod posix;
    pub mod rc;
    pub mod symbol;
}