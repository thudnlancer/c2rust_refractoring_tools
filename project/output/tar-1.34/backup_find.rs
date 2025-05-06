#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    fn backupfile_internal(_: i32, _: *const i8, _: backup_type, _: bool) -> *mut i8;
    fn __xargmatch_internal(
        context: *const i8,
        arg: *const i8,
        arglist: *const *const i8,
        vallist: *const libc::c_void,
        valsize: size_t,
        exit_fn: argmatch_exit_fn,
    ) -> ptrdiff_t;
    static mut argmatch_die: argmatch_exit_fn;
    fn xalloc_die();
    fn getenv(__name: *const i8) -> *mut i8;
}
pub type size_t = u64;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum backup_type {
    numbered_backups = 3,
    numbered_existing_backups = 2,
    simple_backups = 1,
    no_backups = 0,
}
impl backup_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            backup_type::numbered_backups => 3,
            backup_type::numbered_existing_backups => 2,
            backup_type::simple_backups => 1,
            backup_type::no_backups => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> backup_type {
        match value {
            3 => backup_type::numbered_backups,
            2 => backup_type::numbered_existing_backups,
            1 => backup_type::simple_backups,
            0 => backup_type::no_backups,
            _ => panic!("Invalid value for backup_type: {}", value),
        }
    }
}
impl AddAssign<u32> for backup_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = backup_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for backup_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = backup_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for backup_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = backup_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for backup_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = backup_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for backup_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = backup_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for backup_type {
    type Output = backup_type;
    fn add(self, rhs: u32) -> backup_type {
        backup_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for backup_type {
    type Output = backup_type;
    fn sub(self, rhs: u32) -> backup_type {
        backup_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for backup_type {
    type Output = backup_type;
    fn mul(self, rhs: u32) -> backup_type {
        backup_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for backup_type {
    type Output = backup_type;
    fn div(self, rhs: u32) -> backup_type {
        backup_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for backup_type {
    type Output = backup_type;
    fn rem(self, rhs: u32) -> backup_type {
        backup_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type ptrdiff_t = i64;
pub type argmatch_exit_fn = Option<unsafe extern "C" fn() -> ()>;
#[no_mangle]
pub unsafe extern "C" fn find_backup_file_name(
    mut dir_fd: i32,
    mut file: *const i8,
    mut backup_type: backup_type,
) -> *mut i8 {
    let mut result: *mut i8 = backupfile_internal(
        dir_fd,
        file,
        backup_type,
        0 as i32 != 0,
    );
    if result.is_null() {
        xalloc_die();
    }
    return result;
}
static mut backup_args: [*const i8; 9] = [
    b"none\0" as *const u8 as *const i8,
    b"off\0" as *const u8 as *const i8,
    b"simple\0" as *const u8 as *const i8,
    b"never\0" as *const u8 as *const i8,
    b"existing\0" as *const u8 as *const i8,
    b"nil\0" as *const u8 as *const i8,
    b"numbered\0" as *const u8 as *const i8,
    b"t\0" as *const u8 as *const i8,
    0 as *const i8,
];
static mut backup_types: [backup_type; 8] = [
    backup_type::no_backups,
    backup_type::no_backups,
    backup_type::simple_backups,
    backup_type::simple_backups,
    backup_type::numbered_existing_backups,
    backup_type::numbered_existing_backups,
    backup_type::numbered_backups,
    backup_type::numbered_backups,
];
#[no_mangle]
pub unsafe extern "C" fn get_version(
    mut context: *const i8,
    mut version: *const i8,
) -> backup_type {
    if version.is_null() || *version as i32 == 0 as i32 {
        return backup_type::numbered_existing_backups
    } else {
        return backup_types[__xargmatch_internal(
            context,
            version,
            backup_args.as_ptr(),
            backup_types.as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<backup_type>() as u64,
            argmatch_die,
        ) as usize]
    };
}
#[no_mangle]
pub unsafe extern "C" fn xget_version(
    mut context: *const i8,
    mut version: *const i8,
) -> backup_type {
    if !version.is_null() && *version as i32 != 0 {
        return get_version(context, version)
    } else {
        return get_version(
            b"$VERSION_CONTROL\0" as *const u8 as *const i8,
            getenv(b"VERSION_CONTROL\0" as *const u8 as *const i8),
        )
    };
}