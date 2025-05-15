use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strdup(_: *const i8) -> *mut i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn rpl_free(_: *mut libc::c_void);
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn getpwnam(__name: *const i8) -> *mut passwd;
    fn getgrgid(__gid: __gid_t) -> *mut group;
    fn getgrnam(__name: *const i8) -> *mut group;
    fn umaxtostr(_: uintmax_t, _: *mut i8) -> *mut i8;
}
pub type __uintmax_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    _ISalnum = 8,
    _ISpunct = 4,
    _IScntrl = 2,
    _ISblank = 1,
    _ISgraph = 32768,
    _ISprint = 16384,
    _ISspace = 8192,
    _ISxdigit = 4096,
    _ISdigit = 2048,
    _ISalpha = 1024,
    _ISlower = 512,
    _ISupper = 256,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::_ISalnum => 8,
            C2RustUnnamed::_ISpunct => 4,
            C2RustUnnamed::_IScntrl => 2,
            C2RustUnnamed::_ISblank => 1,
            C2RustUnnamed::_ISgraph => 32768,
            C2RustUnnamed::_ISprint => 16384,
            C2RustUnnamed::_ISspace => 8192,
            C2RustUnnamed::_ISxdigit => 4096,
            C2RustUnnamed::_ISdigit => 2048,
            C2RustUnnamed::_ISalpha => 1024,
            C2RustUnnamed::_ISlower => 512,
            C2RustUnnamed::_ISupper => 256,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            8 => C2RustUnnamed::_ISalnum,
            4 => C2RustUnnamed::_ISpunct,
            2 => C2RustUnnamed::_IScntrl,
            1 => C2RustUnnamed::_ISblank,
            32768 => C2RustUnnamed::_ISgraph,
            16384 => C2RustUnnamed::_ISprint,
            8192 => C2RustUnnamed::_ISspace,
            4096 => C2RustUnnamed::_ISxdigit,
            2048 => C2RustUnnamed::_ISdigit,
            1024 => C2RustUnnamed::_ISalpha,
            512 => C2RustUnnamed::_ISlower,
            256 => C2RustUnnamed::_ISupper,
            _ => panic!("Invalid value for C2RustUnnamed: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn add(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn sub(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn mul(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn div(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn rem(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut i8,
    pub pw_passwd: *mut i8,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut i8,
    pub pw_dir: *mut i8,
    pub pw_shell: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut i8,
    pub gr_passwd: *mut i8,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut i8,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut i8, 10 as i32) as i32;
}
unsafe extern "C" fn isnumber_p(mut str: *const i8) -> i32 {
    while *str != 0 {
        if *(*__ctype_b_loc()).offset(*str as i32 as isize) as i32
            & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 == 0
        {
            return 0 as i32;
        }
        str = str.offset(1);
        str;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn parse_user_spec(
    mut spec_arg: *const i8,
    mut uid: *mut uid_t,
    mut gid: *mut gid_t,
    mut username_arg: *mut *mut i8,
    mut groupname_arg: *mut *mut i8,
) -> *const i8 {
    static mut tired: *const i8 = b"virtual memory exhausted\0" as *const u8
        as *const i8;
    let mut error_msg: *const i8 = 0 as *const i8;
    let mut spec: *mut i8 = 0 as *mut i8;
    let mut pwd: *mut passwd = 0 as *mut passwd;
    let mut grp: *mut group = 0 as *mut group;
    let mut g: *mut i8 = 0 as *mut i8;
    let mut u: *mut i8 = 0 as *mut i8;
    let mut separator: *mut i8 = 0 as *mut i8;
    let mut groupname: *mut i8 = 0 as *mut i8;
    error_msg = 0 as *const i8;
    *groupname_arg = 0 as *mut i8;
    *username_arg = *groupname_arg;
    groupname = 0 as *mut i8;
    let mut _len: i32 = strlen(spec_arg) as i32;
    let mut fresh0 = ::std::vec::from_elem(0, (_len + 1 as i32) as u64 as usize);
    spec = fresh0.as_mut_ptr() as *mut i8;
    strcpy(spec, spec_arg);
    separator = strchr(spec, ':' as i32);
    if separator.is_null() {
        separator = strchr(spec, '.' as i32);
    }
    if !separator.is_null() {
        *separator = '\0' as i32 as i8;
    }
    u = if *spec as i32 == '\0' as i32 { 0 as *mut i8 } else { spec };
    g = if separator.is_null()
        || *separator.offset(1 as i32 as isize) as i32 == '\0' as i32
    {
        0 as *mut i8
    } else {
        separator.offset(1 as i32 as isize)
    };
    if u.is_null() && g.is_null() {
        return b"can not omit both user and group\0" as *const u8 as *const i8;
    }
    if !u.is_null() {
        if *u as i32 == '+' as i32 {
            pwd = 0 as *mut passwd;
            u = u.offset(1);
            u;
        } else {
            pwd = getpwnam(u);
        }
        if pwd.is_null() {
            if isnumber_p(u) == 0 {
                error_msg = dcgettext(
                    0 as *const i8,
                    b"invalid user\0" as *const u8 as *const i8,
                    5 as i32,
                );
            } else {
                let mut use_login_group: i32 = 0;
                use_login_group = (!separator.is_null() && g.is_null()) as i32;
                if use_login_group != 0 {
                    error_msg = dcgettext(
                        0 as *const i8,
                        b"cannot get the login group of a numeric UID\0" as *const u8
                            as *const i8,
                        5 as i32,
                    );
                } else {
                    *uid = atoi(u) as uid_t;
                }
            }
        } else {
            *uid = (*pwd).pw_uid;
            if g.is_null() && !separator.is_null() {
                *gid = (*pwd).pw_gid;
                grp = getgrgid((*pwd).pw_gid);
                if grp.is_null() {
                    let mut nbuf: [i8; 21] = [0; 21];
                    let mut _len_0: i32 = strlen(
                        umaxtostr((*pwd).pw_gid as uintmax_t, nbuf.as_mut_ptr()),
                    ) as i32;
                    let mut fresh1 = ::std::vec::from_elem(
                        0,
                        (_len_0 + 1 as i32) as u64 as usize,
                    );
                    groupname = fresh1.as_mut_ptr() as *mut i8;
                    strcpy(
                        groupname,
                        umaxtostr((*pwd).pw_gid as uintmax_t, nbuf.as_mut_ptr()),
                    );
                } else {
                    let mut _len_1: i32 = strlen((*grp).gr_name) as i32;
                    let mut fresh2 = ::std::vec::from_elem(
                        0,
                        (_len_1 + 1 as i32) as u64 as usize,
                    );
                    groupname = fresh2.as_mut_ptr() as *mut i8;
                    strcpy(groupname, (*grp).gr_name);
                }
            }
        }
    }
    if !g.is_null() && error_msg.is_null() {
        if *g as i32 == '+' as i32 {
            grp = 0 as *mut group;
            g = g.offset(1);
            g;
        } else {
            grp = getgrnam(g);
        }
        if grp.is_null() {
            if isnumber_p(g) == 0 {
                error_msg = dcgettext(
                    0 as *const i8,
                    b"invalid group\0" as *const u8 as *const i8,
                    5 as i32,
                );
            } else {
                *gid = atoi(g) as gid_t;
            }
        } else {
            *gid = (*grp).gr_gid;
        }
        if error_msg.is_null() {
            let mut _len_2: i32 = strlen(g) as i32;
            let mut fresh3 = ::std::vec::from_elem(
                0,
                (_len_2 + 1 as i32) as u64 as usize,
            );
            groupname = fresh3.as_mut_ptr() as *mut i8;
            strcpy(groupname, g);
        }
    }
    if error_msg.is_null() {
        if !u.is_null() {
            *username_arg = strdup(u);
            if (*username_arg).is_null() {
                error_msg = tired;
            }
        }
        if !groupname.is_null() && error_msg.is_null() {
            *groupname_arg = strdup(groupname);
            if (*groupname_arg).is_null() {
                if !(*username_arg).is_null() {
                    rpl_free(*username_arg as *mut libc::c_void);
                    *username_arg = 0 as *mut i8;
                }
                error_msg = tired;
            }
        }
    }
    return error_msg;
}