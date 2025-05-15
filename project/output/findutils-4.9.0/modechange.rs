use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn rpl_free(ptr: *mut libc::c_void);
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
}
pub type size_t = u64;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __syscall_slong_t = i64;
pub type mode_t = __mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mode_change {
    pub op: i8,
    pub flag: i8,
    pub affected: mode_t,
    pub value: mode_t,
    pub mentioned: mode_t,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    MODE_DONE,
    MODE_ORDINARY_CHANGE,
    MODE_X_IF_ANY_X,
    MODE_COPY_EXISTING,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::MODE_DONE => 0,
            C2RustUnnamed::MODE_ORDINARY_CHANGE => 1,
            C2RustUnnamed::MODE_X_IF_ANY_X => 2,
            C2RustUnnamed::MODE_COPY_EXISTING => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            0 => C2RustUnnamed::MODE_DONE,
            1 => C2RustUnnamed::MODE_ORDINARY_CHANGE,
            2 => C2RustUnnamed::MODE_X_IF_ANY_X,
            3 => C2RustUnnamed::MODE_COPY_EXISTING,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: i32,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
unsafe extern "C" fn octal_to_mode(mut octal: u32) -> mode_t {
    return if 0o4000 as i32 == 0o4000 as i32 && 0o2000 as i32 == 0o2000 as i32
        && 0o1000 as i32 == 0o1000 as i32 && 0o400 as i32 == 0o400 as i32
        && 0o200 as i32 == 0o200 as i32 && 0o100 as i32 == 0o100 as i32
        && 0o400 as i32 >> 3 as i32 == 0o40 as i32
        && 0o200 as i32 >> 3 as i32 == 0o20 as i32
        && 0o100 as i32 >> 3 as i32 == 0o10 as i32
        && 0o400 as i32 >> 3 as i32 >> 3 as i32 == 0o4 as i32
        && 0o200 as i32 >> 3 as i32 >> 3 as i32 == 0o2 as i32
        && 0o100 as i32 >> 3 as i32 >> 3 as i32 == 0o1 as i32
    {
        octal
    } else {
        ((if octal & 0o4000 as i32 as u32 != 0 { 0o4000 as i32 } else { 0 as i32 })
            | (if octal & 0o2000 as i32 as u32 != 0 { 0o2000 as i32 } else { 0 as i32 })
            | (if octal & 0o1000 as i32 as u32 != 0 { 0o1000 as i32 } else { 0 as i32 })
            | (if octal & 0o400 as i32 as u32 != 0 { 0o400 as i32 } else { 0 as i32 })
            | (if octal & 0o200 as i32 as u32 != 0 { 0o200 as i32 } else { 0 as i32 })
            | (if octal & 0o100 as i32 as u32 != 0 { 0o100 as i32 } else { 0 as i32 })
            | (if octal & 0o40 as i32 as u32 != 0 {
                0o400 as i32 >> 3 as i32
            } else {
                0 as i32
            })
            | (if octal & 0o20 as i32 as u32 != 0 {
                0o200 as i32 >> 3 as i32
            } else {
                0 as i32
            })
            | (if octal & 0o10 as i32 as u32 != 0 {
                0o100 as i32 >> 3 as i32
            } else {
                0 as i32
            })
            | (if octal & 0o4 as i32 as u32 != 0 {
                0o400 as i32 >> 3 as i32 >> 3 as i32
            } else {
                0 as i32
            })
            | (if octal & 0o2 as i32 as u32 != 0 {
                0o200 as i32 >> 3 as i32 >> 3 as i32
            } else {
                0 as i32
            })
            | (if octal & 0o1 as i32 as u32 != 0 {
                0o100 as i32 >> 3 as i32 >> 3 as i32
            } else {
                0 as i32
            })) as mode_t
    };
}
unsafe extern "C" fn make_node_op_equals(
    mut new_mode: mode_t,
    mut mentioned: mode_t,
) -> *mut mode_change {
    let mut p: *mut mode_change = xmalloc(
        (2 as i32 as u64).wrapping_mul(::core::mem::size_of::<mode_change>() as u64),
    ) as *mut mode_change;
    (*p).op = '=' as i32 as i8;
    (*p).flag = C2RustUnnamed::MODE_ORDINARY_CHANGE as i32 as i8;
    (*p).affected = (0o4000 as i32 | 0o2000 as i32 | 0o1000 as i32
        | (0o400 as i32 | 0o200 as i32 | 0o100 as i32)
        | (0o400 as i32 | 0o200 as i32 | 0o100 as i32) >> 3 as i32
        | (0o400 as i32 | 0o200 as i32 | 0o100 as i32) >> 3 as i32 >> 3 as i32)
        as mode_t;
    (*p).value = new_mode;
    (*p).mentioned = mentioned;
    (*p.offset(1 as i32 as isize)).flag = C2RustUnnamed::MODE_DONE as i32 as i8;
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn mode_compile(mut mode_string: *const i8) -> *mut mode_change {
    let mut current_block: u64;
    let mut mc: *mut mode_change = 0 as *mut mode_change;
    let mut used: size_t = 0 as i32 as size_t;
    let mut p: *const i8 = 0 as *const i8;
    if '0' as i32 <= *mode_string as i32 && (*mode_string as i32) < '8' as i32 {
        let mut octal_mode: u32 = 0 as i32 as u32;
        let mut mode: mode_t = 0;
        let mut mentioned: mode_t = 0;
        p = mode_string;
        loop {
            let fresh0 = p;
            p = p.offset(1);
            octal_mode = (8 as i32 as u32)
                .wrapping_mul(octal_mode)
                .wrapping_add(*fresh0 as u32)
                .wrapping_sub('0' as i32 as u32);
            if (0o7777 as i32 as u32) < octal_mode {
                return 0 as *mut mode_change;
            }
            if !('0' as i32 <= *p as i32 && (*p as i32) < '8' as i32) {
                break;
            }
        }
        if *p != 0 {
            return 0 as *mut mode_change;
        }
        mode = octal_to_mode(octal_mode);
        mentioned = if (p.offset_from(mode_string) as i64) < 5 as i32 as i64 {
            mode & (0o4000 as i32 | 0o2000 as i32) as u32 | 0o1000 as i32 as u32
                | (0o400 as i32 | 0o200 as i32 | 0o100 as i32
                    | (0o400 as i32 | 0o200 as i32 | 0o100 as i32) >> 3 as i32
                    | (0o400 as i32 | 0o200 as i32 | 0o100 as i32) >> 3 as i32
                        >> 3 as i32) as u32
        } else {
            (0o4000 as i32 | 0o2000 as i32 | 0o1000 as i32
                | (0o400 as i32 | 0o200 as i32 | 0o100 as i32)
                | (0o400 as i32 | 0o200 as i32 | 0o100 as i32) >> 3 as i32
                | (0o400 as i32 | 0o200 as i32 | 0o100 as i32) >> 3 as i32 >> 3 as i32)
                as u32
        };
        return make_node_op_equals(mode, mentioned);
    }
    let mut needed: size_t = 1 as i32 as size_t;
    p = mode_string;
    while *p != 0 {
        needed = (needed as u64)
            .wrapping_add(
                (*p as i32 == '=' as i32 || *p as i32 == '+' as i32
                    || *p as i32 == '-' as i32) as i32 as u64,
            ) as size_t as size_t;
        p = p.offset(1);
        p;
    }
    mc = xnmalloc(needed, ::core::mem::size_of::<mode_change>() as u64)
        as *mut mode_change;
    p = mode_string;
    's_90: loop {
        let mut affected: mode_t = 0 as i32 as mode_t;
        loop {
            match *p as i32 {
                117 => {
                    affected
                        |= (0o4000 as i32 | (0o400 as i32 | 0o200 as i32 | 0o100 as i32))
                            as u32;
                }
                103 => {
                    affected
                        |= (0o2000 as i32
                            | (0o400 as i32 | 0o200 as i32 | 0o100 as i32) >> 3 as i32)
                            as u32;
                }
                111 => {
                    affected
                        |= (0o1000 as i32
                            | (0o400 as i32 | 0o200 as i32 | 0o100 as i32) >> 3 as i32
                                >> 3 as i32) as u32;
                }
                97 => {
                    affected
                        |= (0o4000 as i32 | 0o2000 as i32 | 0o1000 as i32
                            | (0o400 as i32 | 0o200 as i32 | 0o100 as i32)
                            | (0o400 as i32 | 0o200 as i32 | 0o100 as i32) >> 3 as i32
                            | (0o400 as i32 | 0o200 as i32 | 0o100 as i32) >> 3 as i32
                                >> 3 as i32) as u32;
                }
                61 | 43 | 45 => {
                    break;
                }
                _ => {
                    current_block = 3929528004373468346;
                    break 's_90;
                }
            }
            p = p.offset(1);
            p;
        }
        loop {
            let fresh1 = p;
            p = p.offset(1);
            let mut op: i8 = *fresh1;
            let mut value: mode_t = 0;
            let mut mentioned_0: mode_t = 0 as i32 as mode_t;
            let mut flag: i8 = C2RustUnnamed::MODE_COPY_EXISTING as i32 as i8;
            let mut change: *mut mode_change = 0 as *mut mode_change;
            match *p as i32 {
                48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
                    let mut octal_mode_0: u32 = 0 as i32 as u32;
                    loop {
                        let fresh2 = p;
                        p = p.offset(1);
                        octal_mode_0 = (8 as i32 as u32)
                            .wrapping_mul(octal_mode_0)
                            .wrapping_add(*fresh2 as u32)
                            .wrapping_sub('0' as i32 as u32);
                        if (0o7777 as i32 as u32) < octal_mode_0 {
                            current_block = 3929528004373468346;
                            break 's_90;
                        }
                        if !('0' as i32 <= *p as i32 && (*p as i32) < '8' as i32) {
                            break;
                        }
                    }
                    if affected != 0 || *p as i32 != 0 && *p as i32 != ',' as i32 {
                        current_block = 3929528004373468346;
                        break 's_90;
                    }
                    mentioned_0 = (0o4000 as i32 | 0o2000 as i32 | 0o1000 as i32
                        | (0o400 as i32 | 0o200 as i32 | 0o100 as i32)
                        | (0o400 as i32 | 0o200 as i32 | 0o100 as i32) >> 3 as i32
                        | (0o400 as i32 | 0o200 as i32 | 0o100 as i32) >> 3 as i32
                            >> 3 as i32) as mode_t;
                    affected = mentioned_0;
                    value = octal_to_mode(octal_mode_0);
                    flag = C2RustUnnamed::MODE_ORDINARY_CHANGE as i32 as i8;
                }
                117 => {
                    value = (0o400 as i32 | 0o200 as i32 | 0o100 as i32) as mode_t;
                    p = p.offset(1);
                    p;
                }
                103 => {
                    value = ((0o400 as i32 | 0o200 as i32 | 0o100 as i32) >> 3 as i32)
                        as mode_t;
                    p = p.offset(1);
                    p;
                }
                111 => {
                    value = ((0o400 as i32 | 0o200 as i32 | 0o100 as i32) >> 3 as i32
                        >> 3 as i32) as mode_t;
                    p = p.offset(1);
                    p;
                }
                _ => {
                    value = 0 as i32 as mode_t;
                    flag = C2RustUnnamed::MODE_ORDINARY_CHANGE as i32 as i8;
                    loop {
                        match *p as i32 {
                            114 => {
                                value
                                    |= (0o400 as i32 | 0o400 as i32 >> 3 as i32
                                        | 0o400 as i32 >> 3 as i32 >> 3 as i32) as u32;
                            }
                            119 => {
                                value
                                    |= (0o200 as i32 | 0o200 as i32 >> 3 as i32
                                        | 0o200 as i32 >> 3 as i32 >> 3 as i32) as u32;
                            }
                            120 => {
                                value
                                    |= (0o100 as i32 | 0o100 as i32 >> 3 as i32
                                        | 0o100 as i32 >> 3 as i32 >> 3 as i32) as u32;
                            }
                            88 => {
                                flag = C2RustUnnamed::MODE_X_IF_ANY_X as i32 as i8;
                            }
                            115 => {
                                value |= (0o4000 as i32 | 0o2000 as i32) as u32;
                            }
                            116 => {
                                value |= 0o1000 as i32 as u32;
                            }
                            _ => {
                                break;
                            }
                        }
                        p = p.offset(1);
                        p;
                    }
                }
            }
            let fresh3 = used;
            used = used.wrapping_add(1);
            change = &mut *mc.offset(fresh3 as isize) as *mut mode_change;
            (*change).op = op;
            (*change).flag = flag;
            (*change).affected = affected;
            (*change).value = value;
            (*change).mentioned = if mentioned_0 != 0 {
                mentioned_0
            } else if affected != 0 {
                affected & value
            } else {
                value
            };
            if !(*p as i32 == '=' as i32 || *p as i32 == '+' as i32
                || *p as i32 == '-' as i32)
            {
                break;
            }
        }
        if *p as i32 != ',' as i32 {
            current_block = 16779030619667747692;
            break;
        }
        p = p.offset(1);
        p;
    }
    match current_block {
        16779030619667747692 => {
            if *p as i32 == 0 as i32 {
                (*mc.offset(used as isize)).flag = C2RustUnnamed::MODE_DONE as i32 as i8;
                return mc;
            }
        }
        _ => {}
    }
    rpl_free(mc as *mut libc::c_void);
    return 0 as *mut mode_change;
}
#[no_mangle]
pub unsafe extern "C" fn mode_create_from_ref(
    mut ref_file: *const i8,
) -> *mut mode_change {
    let mut ref_stats: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if stat(ref_file, &mut ref_stats) != 0 as i32 {
        return 0 as *mut mode_change;
    }
    return make_node_op_equals(
        ref_stats.st_mode,
        (0o4000 as i32 | 0o2000 as i32 | 0o1000 as i32
            | (0o400 as i32 | 0o200 as i32 | 0o100 as i32)
            | (0o400 as i32 | 0o200 as i32 | 0o100 as i32) >> 3 as i32
            | (0o400 as i32 | 0o200 as i32 | 0o100 as i32) >> 3 as i32 >> 3 as i32)
            as mode_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mode_adjust(
    mut oldmode: mode_t,
    mut dir: bool,
    mut umask_value: mode_t,
    mut changes: *const mode_change,
    mut pmode_bits: *mut mode_t,
) -> mode_t {
    let mut newmode: mode_t = oldmode
        & (0o4000 as i32 | 0o2000 as i32 | 0o1000 as i32
            | (0o400 as i32 | 0o200 as i32 | 0o100 as i32)
            | (0o400 as i32 | 0o200 as i32 | 0o100 as i32) >> 3 as i32
            | (0o400 as i32 | 0o200 as i32 | 0o100 as i32) >> 3 as i32 >> 3 as i32)
            as u32;
    let mut mode_bits: mode_t = 0 as i32 as mode_t;
    while (*changes).flag as i32 != C2RustUnnamed::MODE_DONE as i32 {
        let mut affected: mode_t = (*changes).affected;
        let mut omit_change: mode_t = (if dir as i32 != 0 {
            0o4000 as i32 | 0o2000 as i32
        } else {
            0 as i32
        }) as u32 & !(*changes).mentioned;
        let mut value: mode_t = (*changes).value;
        match (*changes).flag as i32 {
            3 => {
                value &= newmode;
                value
                    |= ((if value
                        & (0o400 as i32 | 0o400 as i32 >> 3 as i32
                            | 0o400 as i32 >> 3 as i32 >> 3 as i32) as u32 != 0
                    {
                        0o400 as i32 | 0o400 as i32 >> 3 as i32
                            | 0o400 as i32 >> 3 as i32 >> 3 as i32
                    } else {
                        0 as i32
                    })
                        | (if value
                            & (0o200 as i32 | 0o200 as i32 >> 3 as i32
                                | 0o200 as i32 >> 3 as i32 >> 3 as i32) as u32 != 0
                        {
                            0o200 as i32 | 0o200 as i32 >> 3 as i32
                                | 0o200 as i32 >> 3 as i32 >> 3 as i32
                        } else {
                            0 as i32
                        })
                        | (if value
                            & (0o100 as i32 | 0o100 as i32 >> 3 as i32
                                | 0o100 as i32 >> 3 as i32 >> 3 as i32) as u32 != 0
                        {
                            0o100 as i32 | 0o100 as i32 >> 3 as i32
                                | 0o100 as i32 >> 3 as i32 >> 3 as i32
                        } else {
                            0 as i32
                        })) as u32;
            }
            2 => {
                if newmode
                    & (0o100 as i32 | 0o100 as i32 >> 3 as i32
                        | 0o100 as i32 >> 3 as i32 >> 3 as i32) as u32 | dir as u32 != 0
                {
                    value
                        |= (0o100 as i32 | 0o100 as i32 >> 3 as i32
                            | 0o100 as i32 >> 3 as i32 >> 3 as i32) as u32;
                }
            }
            1 | _ => {}
        }
        value &= (if affected != 0 { affected } else { !umask_value }) & !omit_change;
        match (*changes).op as i32 {
            61 => {
                let mut preserved: mode_t = (if affected != 0 {
                    !affected
                } else {
                    0 as i32 as u32
                }) | omit_change;
                mode_bits
                    |= (0o4000 as i32 | 0o2000 as i32 | 0o1000 as i32
                        | (0o400 as i32 | 0o200 as i32 | 0o100 as i32)
                        | (0o400 as i32 | 0o200 as i32 | 0o100 as i32) >> 3 as i32
                        | (0o400 as i32 | 0o200 as i32 | 0o100 as i32) >> 3 as i32
                            >> 3 as i32) as u32 & !preserved;
                newmode = newmode & preserved | value;
            }
            43 => {
                mode_bits |= value;
                newmode |= value;
            }
            45 => {
                mode_bits |= value;
                newmode &= !value;
            }
            _ => {}
        }
        changes = changes.offset(1);
        changes;
    }
    if !pmode_bits.is_null() {
        *pmode_bits = mode_bits;
    }
    return newmode;
}