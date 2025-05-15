use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn printf(_: *const i8, _: ...) -> i32;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn exit(_: i32) -> !;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn getopt_long(
        ___argc: i32,
        ___argv: *const *mut i8,
        __shortopts: *const i8,
        __longopts: *const option,
        __longind: *mut i32,
    ) -> i32;
    static mut opterr: i32;
    static mut optind: i32;
    fn generic_warn(who: *const i8, fmt: *const i8, _: ...);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct program {
    pub invoke: *const i8,
    pub name: *const i8,
    pub desc: *const i8,
    pub help: *const i8,
    pub tyag: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const i8,
    pub has_arg: i32,
    pub flag: *mut i32,
    pub val: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum hv_option_values {
    hv_version = 1,
    hv_help = 0,
}
impl hv_option_values {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            hv_option_values::hv_version => 1,
            hv_option_values::hv_help => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> hv_option_values {
        match value {
            1 => hv_option_values::hv_version,
            0 => hv_option_values::hv_help,
            _ => panic!("Invalid value for hv_option_values: {}", value),
        }
    }
}
impl AddAssign<u32> for hv_option_values {
    fn add_assign(&mut self, rhs: u32) {
        *self = hv_option_values::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for hv_option_values {
    fn sub_assign(&mut self, rhs: u32) {
        *self = hv_option_values::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for hv_option_values {
    fn mul_assign(&mut self, rhs: u32) {
        *self = hv_option_values::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for hv_option_values {
    fn div_assign(&mut self, rhs: u32) {
        *self = hv_option_values::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for hv_option_values {
    fn rem_assign(&mut self, rhs: u32) {
        *self = hv_option_values::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for hv_option_values {
    type Output = hv_option_values;
    fn add(self, rhs: u32) -> hv_option_values {
        hv_option_values::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for hv_option_values {
    type Output = hv_option_values;
    fn sub(self, rhs: u32) -> hv_option_values {
        hv_option_values::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for hv_option_values {
    type Output = hv_option_values;
    fn mul(self, rhs: u32) -> hv_option_values {
        hv_option_values::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for hv_option_values {
    type Output = hv_option_values;
    fn div(self, rhs: u32) -> hv_option_values {
        hv_option_values::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for hv_option_values {
    type Output = hv_option_values;
    fn rem(self, rhs: u32) -> hv_option_values {
        hv_option_values::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[no_mangle]
pub unsafe extern "C" fn nice_getopt(
    mut argc: i32,
    mut argv: *mut *mut i8,
    mut longopts: *const option,
) -> i32 {
    optind = 0 as i32;
    opterr = 0 as i32;
    return getopt_long(
        argc,
        argv,
        b"+\0" as *const u8 as *const i8,
        longopts,
        0 as *mut i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn display_version(mut prog: *const program, mut flags: i32) {
    if 1 as i32 & flags != 0 {
        generic_warn(
            0 as *const i8,
            b"-V is obsolete; instead, use --version\0" as *const u8 as *const i8,
        );
    }
    printf(
        b"%s%s\0" as *const u8 as *const i8,
        (*prog).name,
        b" (GNU RCS) 5.10.0\nCopyright (C) 2010-2020 Thien-Thi Nguyen\nCopyright (C) 1990-1995 Paul Eggert\nCopyright (C) 1982,1988,1989 Walter F. Tichy, Purdue CS\nLicense GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\n\0"
            as *const u8 as *const i8,
    );
    if 2 as i32 & flags != 0 {
        exit(0 as i32);
    }
}
static mut ok: [option; 3] = [
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: hv_option_values::hv_help as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: hv_option_values::hv_version as i32,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 0 as i32,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn check_hv(
    mut argc: i32,
    mut argv: *mut *mut i8,
    mut prog: *const program,
) {
    if 1 as i32 >= argc {
        return;
    }
    match nice_getopt(argc, argv, ok.as_ptr()) {
        0 => {
            let mut usage: [i8; 128] = [0; 128];
            let mut nl: i32 = 0;
            snprintf(
                usage.as_mut_ptr(),
                128 as i32 as u64,
                b"%s\0" as *const u8 as *const i8,
                (*prog).help,
            );
            nl = (strchr(usage.as_mut_ptr(), '\n' as i32))
                .offset_from(usage.as_mut_ptr()) as i64 as i32;
            usage[nl as usize] = '\0' as i32 as i8;
            printf(
                b"Usage: %s %s\n\n%s\n%s%s\0" as *const u8 as *const i8,
                (*prog).name,
                usage.as_mut_ptr(),
                (*prog).desc,
                ((*prog).help).offset(nl as isize),
                b"\nReport bugs to: <bug-rcs@gnu.org>\nRCS home page: <http://www.gnu.org/software/rcs/>\nGeneral help using GNU software: <http://www.gnu.org/gethelp/>\n\0"
                    as *const u8 as *const i8,
            );
            exit(0 as i32);
        }
        1 => {
            display_version(prog, 2 as i32);
        }
        _ => {}
    };
}