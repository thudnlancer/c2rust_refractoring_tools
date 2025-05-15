use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
}
pub type __int32_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: i64,
    pub tm_zone: *const i8,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct partime {
    pub tm: tm,
    pub ymodulus: i32,
    pub yweek: i32,
    pub zone: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct name_val {
    pub name: [i8; 4],
    pub val: i32,
}
#[inline]
unsafe extern "C" fn tolower(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
static mut month_names: [name_val; 13] = unsafe {
    [
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"jan\0"),
                val: 0 as i32,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"feb\0"),
                val: 1 as i32,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"mar\0"),
                val: 2 as i32,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"apr\0"),
                val: 3 as i32,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"may\0"),
                val: 4 as i32,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"jun\0"),
                val: 5 as i32,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"jul\0"),
                val: 6 as i32,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"aug\0"),
                val: 7 as i32,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"sep\0"),
                val: 8 as i32,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"oct\0"),
                val: 9 as i32,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"nov\0"),
                val: 10 as i32,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"dec\0"),
                val: 11 as i32,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"\0\0\0\0"),
                val: -(1 as i32),
            };
            init
        },
    ]
};
static mut weekday_names: [name_val; 8] = unsafe {
    [
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"sun\0"),
                val: 0 as i32,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"mon\0"),
                val: 1 as i32,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"tue\0"),
                val: 2 as i32,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"wed\0"),
                val: 3 as i32,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"thu\0"),
                val: 4 as i32,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"fri\0"),
                val: 5 as i32,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"sat\0"),
                val: 6 as i32,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"\0\0\0\0"),
                val: -(1 as i32),
            };
            init
        },
    ]
};
static mut zone_names: [name_val; 36] = [name_val { name: [0; 4], val: 0 }; 36];
unsafe extern "C" fn lookup(mut s: *const i8, mut table: *const name_val) -> i32 {
    let mut j: i32 = 0;
    let mut buf: [i8; 4] = [0 as i32 as i8, 0, 0, 0];
    j = 0 as i32;
    while j < 4 as i32 {
        let fresh0 = s;
        s = s.offset(1);
        let mut c: u8 = *fresh0 as u8;
        buf[j as usize] = (if *(*__ctype_b_loc()).offset(c as i32 as isize) as i32
            & C2RustUnnamed::_ISupper as i32 as libc::c_ushort as i32 != 0
        {
            ({
                let mut __res: i32 = 0;
                if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                    if 0 != 0 {
                        let mut __c: i32 = c as i32;
                        __res = if __c < -(128 as i32) || __c > 255 as i32 {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = tolower(c as i32);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc()).offset(c as i32 as isize);
                }
                __res
            })
        } else {
            c as i32
        }) as i8;
        if *(*__ctype_b_loc()).offset(c as i32 as isize) as i32
            & C2RustUnnamed::_ISalpha as i32 as libc::c_ushort as i32 == 0
        {
            break;
        }
        j += 1;
        j;
    }
    's_30: while (*table.offset(0 as i32 as isize)).name[0 as i32 as usize] != 0 {
        j = 0 as i32;
        while buf[j as usize] as i32
            == (*table.offset(0 as i32 as isize)).name[j as usize] as i32
        {
            j += 1;
            if j == 4 as i32 || (*table.offset(0 as i32 as isize)).name[j as usize] == 0
            {
                break 's_30;
            }
        }
        table = table.offset(1);
        table;
    }
    return (*table.offset(0 as i32 as isize)).val;
}
unsafe extern "C" fn undefine(mut t: *mut partime) {
    (*t).yweek = -(1 as i32);
    (*t).ymodulus = (*t).yweek;
    (*t).tm.tm_yday = (*t).ymodulus;
    (*t).tm.tm_wday = (*t).tm.tm_yday;
    (*t).tm.tm_year = (*t).tm.tm_wday;
    (*t).tm.tm_mon = (*t).tm.tm_year;
    (*t).tm.tm_mday = (*t).tm.tm_mon;
    (*t).tm.tm_hour = (*t).tm.tm_mday;
    (*t).tm.tm_min = (*t).tm.tm_hour;
    (*t).tm.tm_sec = (*t).tm.tm_min;
    (*t).zone = -(24 as i32) as i64 * 60 as i32 as i64 * 60 as i32 as i64;
}
static mut patterns: [*const i8; 53] = [
    b"E_n_y\0" as *const u8 as *const i8,
    b"x\0" as *const u8 as *const i8,
    b"E_n\0" as *const u8 as *const i8,
    b"n_E\0" as *const u8 as *const i8,
    b"n\0" as *const u8 as *const i8,
    b"t:m:s_A\0" as *const u8 as *const i8,
    b"t:m_A\0" as *const u8 as *const i8,
    b"t_A\0" as *const u8 as *const i8,
    b"y/N/D$\0" as *const u8 as *const i8,
    b"y-N-D$\0" as *const u8 as *const i8,
    b"4ND$\0" as *const u8 as *const i8,
    b"Y-N$\0" as *const u8 as *const i8,
    b"RND$\0" as *const u8 as *const i8,
    b"-R=N$\0" as *const u8 as *const i8,
    b"-R$\0" as *const u8 as *const i8,
    b"--N=D$\0" as *const u8 as *const i8,
    b"N=DT\0" as *const u8 as *const i8,
    b"--N$\0" as *const u8 as *const i8,
    b"---D$\0" as *const u8 as *const i8,
    b"DT\0" as *const u8 as *const i8,
    b"Y-d$\0" as *const u8 as *const i8,
    b"4d$\0" as *const u8 as *const i8,
    b"R=d$\0" as *const u8 as *const i8,
    b"-d$\0" as *const u8 as *const i8,
    b"dT\0" as *const u8 as *const i8,
    b"y-W-X\0" as *const u8 as *const i8,
    b"yWX\0" as *const u8 as *const i8,
    b"y=W\0" as *const u8 as *const i8,
    b"-r-W-X\0" as *const u8 as *const i8,
    b"r-W-XT\0" as *const u8 as *const i8,
    b"-rWX\0" as *const u8 as *const i8,
    b"rWXT\0" as *const u8 as *const i8,
    b"-W=X\0" as *const u8 as *const i8,
    b"W=XT\0" as *const u8 as *const i8,
    b"-W\0" as *const u8 as *const i8,
    b"-w-X\0" as *const u8 as *const i8,
    b"w-XT\0" as *const u8 as *const i8,
    b"---X$\0" as *const u8 as *const i8,
    b"XT\0" as *const u8 as *const i8,
    b"4$\0" as *const u8 as *const i8,
    b"T\0" as *const u8 as *const i8,
    b"h:m:s$\0" as *const u8 as *const i8,
    b"hms$\0" as *const u8 as *const i8,
    b"h:m$\0" as *const u8 as *const i8,
    b"hm$\0" as *const u8 as *const i8,
    b"h$\0" as *const u8 as *const i8,
    b"-m:s$\0" as *const u8 as *const i8,
    b"-ms$\0" as *const u8 as *const i8,
    b"-m$\0" as *const u8 as *const i8,
    b"--s$\0" as *const u8 as *const i8,
    b"Y\0" as *const u8 as *const i8,
    b"Z\0" as *const u8 as *const i8,
    0 as *const i8,
];
unsafe extern "C" fn parse_fixed(
    mut s: *const i8,
    mut digits: i32,
    mut res: *mut i32,
) -> *const i8 {
    let mut n: i32 = 0 as i32;
    let mut lim: *const i8 = s.offset(digits as isize);
    while s < lim {
        let fresh1 = s;
        s = s.offset(1);
        let mut d: u32 = (*fresh1 as i32 - '0' as i32) as u32;
        if (9 as i32 as u32) < d {
            return 0 as *const i8;
        }
        n = ((10 as i32 * n) as u32).wrapping_add(d) as i32;
    }
    *res = n;
    return s;
}
unsafe extern "C" fn parse_ranged(
    mut s: *const i8,
    mut digits: i32,
    mut lo: i32,
    mut hi: i32,
    mut res: *mut i32,
) -> *const i8 {
    s = parse_fixed(s, digits, res);
    return if !s.is_null() && lo <= *res && *res <= hi { s } else { 0 as *const i8 };
}
unsafe extern "C" fn parse_decimal(
    mut s: *const i8,
    mut digits: i32,
    mut lo: i32,
    mut hi: i32,
    mut resolution: i32,
    mut res: *mut i32,
    mut fres: *mut i32,
) -> *const i8 {
    s = parse_fixed(s, digits, res);
    *fres = 0 as i32;
    if !s.is_null() && lo <= *res && *res <= hi {
        let mut f: i32 = 0 as i32;
        if (*s.offset(0 as i32 as isize) as i32 == ',' as i32
            || *s.offset(0 as i32 as isize) as i32 == '.' as i32)
            && *(*__ctype_b_loc())
                .offset(*s.offset(1 as i32 as isize) as u8 as i32 as isize) as i32
                & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 != 0
        {
            s = s.offset(1);
            let mut s1: *const i8 = s;
            let mut num10: i32 = 0 as i32;
            let mut denom10: i32 = 10 as i32;
            let mut product: i32 = 0;
            loop {
                s = s.offset(1);
                if !(*(*__ctype_b_loc()).offset(*s as u8 as i32 as isize) as i32
                    & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 != 0)
                {
                    break;
                }
                denom10 *= 10 as i32;
            }
            s = parse_fixed(s1, s.offset_from(s1) as i64 as i32, &mut num10);
            product = num10 * resolution;
            f = (product + (denom10 >> 1 as i32)) / denom10;
            f -= f & (product % denom10 == denom10 >> 1 as i32) as i32;
            if f < 0 as i32 || product / resolution != num10 {
                return 0 as *const i8;
            }
        }
        *fres = f;
        return s;
    }
    return 0 as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn parzone(mut s: *const i8, mut zone: *mut i64) -> *const i8 {
    let mut sign: i8 = 0;
    let mut hh: i32 = 0;
    let mut mm: i32 = 0;
    let mut ss: i32 = 0;
    let mut minutesEastOfUTC: i32 = 0;
    let mut offset: i64 = 0;
    let mut z: i64 = 0;
    let mut current_block_18: u64;
    match *s as i32 {
        45 | 43 => {
            z = 0 as i32 as i64;
        }
        _ => {
            minutesEastOfUTC = lookup(s, zone_names.as_ptr());
            if minutesEastOfUTC == -(1 as i32) {
                return 0 as *const i8;
            }
            while *(*__ctype_b_loc()).offset(*s as u8 as i32 as isize) as i32
                & C2RustUnnamed::_ISalpha as i32 as libc::c_ushort as i32 != 0
            {
                s = s.offset(1);
                s;
            }
            if minutesEastOfUTC == 1 as i32 {
                *zone = -(24 as i32) as i64 * 60 as i32 as i64 * 60 as i32 as i64
                    - 1 as i32 as i64;
                return s;
            }
            z = minutesEastOfUTC as i64 * 60 as i64;
            if (*s.offset(-(1 as i32) as isize) as i32 == 'T' as i32
                || *s.offset(-(1 as i32) as isize) as i32 == 't' as i32)
                && (*s.offset(-(2 as i32) as isize) as i32 == 'S' as i32
                    || *s.offset(-(2 as i32) as isize) as i32 == 's' as i32)
                && (*s.offset(-(3 as i32) as isize) as i32 == 'D' as i32
                    || *s.offset(-(3 as i32) as isize) as i32 == 't' as i32)
            {
                current_block_18 = 13819033241300642179;
            } else {
                while *(*__ctype_b_loc()).offset(*s as u8 as i32 as isize) as i32
                    & C2RustUnnamed::_ISspace as i32 as libc::c_ushort as i32 != 0
                {
                    s = s.offset(1);
                    s;
                }
                if (*s.offset(0 as i32 as isize) as i32 == 'D' as i32
                    || *s.offset(0 as i32 as isize) as i32 == 'd' as i32)
                    && (*s.offset(1 as i32 as isize) as i32 == 'S' as i32
                        || *s.offset(1 as i32 as isize) as i32 == 's' as i32)
                    && (*s.offset(2 as i32 as isize) as i32 == 'T' as i32
                        || *s.offset(2 as i32 as isize) as i32 == 't' as i32)
                {
                    s = s.offset(3 as i32 as isize);
                    current_block_18 = 13819033241300642179;
                } else {
                    match *s as i32 {
                        45 | 43 => {}
                        _ => {
                            *zone = z;
                            return s;
                        }
                    }
                    current_block_18 = 4495394744059808450;
                }
            }
            match current_block_18 {
                4495394744059808450 => {}
                _ => {
                    *zone = z + (60 as i32 * 60 as i32) as i64;
                    return s;
                }
            }
        }
    }
    let fresh2 = s;
    s = s.offset(1);
    sign = *fresh2;
    s = parse_ranged(s, 2 as i32, 0 as i32, 23 as i32, &mut hh);
    if s.is_null() {
        return 0 as *const i8;
    }
    ss = 0 as i32;
    mm = ss;
    if *s as i32 == ':' as i32 {
        s = s.offset(1);
        s;
    }
    if *(*__ctype_b_loc()).offset(*s as u8 as i32 as isize) as i32
        & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 != 0
    {
        s = parse_ranged(s, 2 as i32, 0 as i32, 59 as i32, &mut mm);
        if s.is_null() {
            return 0 as *const i8;
        }
        if *s as i32 == ':' as i32
            && *s.offset(-(3 as i32) as isize) as i32 == ':' as i32
            && *(*__ctype_b_loc())
                .offset(*s.offset(1 as i32 as isize) as u8 as i32 as isize) as i32
                & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 != 0
        {
            s = parse_ranged(
                s.offset(1 as i32 as isize),
                2 as i32,
                0 as i32,
                59 as i32,
                &mut ss,
            );
            if s.is_null() {
                return 0 as *const i8;
            }
        }
    }
    if *(*__ctype_b_loc()).offset(*s as u8 as i32 as isize) as i32
        & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 != 0
    {
        return 0 as *const i8;
    }
    offset = (hh * 60 as i32 + mm) as i64 * 60 as i64 + ss as i64;
    *zone = z + (if sign as i32 == '-' as i32 { -offset } else { offset });
    return s;
}
unsafe extern "C" fn parse_pattern_letter(
    mut s: *const i8,
    mut c: i32,
    mut t: *mut partime,
) -> *const i8 {
    let mut current_block_61: u64;
    match c {
        36 => {
            if *(*__ctype_b_loc()).offset(*s as u8 as i32 as isize) as i32
                & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 != 0
            {
                return 0 as *const i8;
            }
            current_block_61 = 919954187481050311;
        }
        45 | 47 | 58 => {
            let fresh3 = s;
            s = s.offset(1);
            if *fresh3 as i32 != c {
                return 0 as *const i8;
            }
            current_block_61 = 919954187481050311;
        }
        52 => {
            s = parse_fixed(s, 4 as i32, &mut (*t).tm.tm_year);
            current_block_61 = 919954187481050311;
        }
        61 => {
            s = s.offset((*s as i32 == '-' as i32) as i32 as isize);
            current_block_61 = 919954187481050311;
        }
        65 => {
            let fresh4 = s;
            s = s.offset(1);
            match *fresh4 as i32 {
                65 | 97 => {
                    if (*t).tm.tm_hour == 12 as i32 {
                        (*t).tm.tm_hour = 0 as i32;
                    }
                }
                80 | 112 => {
                    if (*t).tm.tm_hour != 12 as i32 {
                        (*t).tm.tm_hour += 12 as i32;
                    }
                }
                _ => return 0 as *const i8,
            }
            match *s as i32 {
                77 | 109 => {
                    s = s.offset(1);
                    s;
                }
                _ => {}
            }
            if *(*__ctype_b_loc()).offset(*s as i32 as isize) as i32
                & C2RustUnnamed::_ISalnum as i32 as libc::c_ushort as i32 != 0
            {
                return 0 as *const i8;
            }
            current_block_61 = 919954187481050311;
        }
        68 => {
            s = parse_ranged(s, 2 as i32, 1 as i32, 31 as i32, &mut (*t).tm.tm_mday);
            current_block_61 = 919954187481050311;
        }
        100 => {
            s = parse_ranged(s, 3 as i32, 1 as i32, 366 as i32, &mut (*t).tm.tm_yday);
            (*t).tm.tm_yday -= 1;
            (*t).tm.tm_yday;
            current_block_61 = 919954187481050311;
        }
        69 => {
            s = parse_ranged(
                s,
                (*(*__ctype_b_loc())
                    .offset(*s.offset(0 as i32 as isize) as u8 as i32 as isize) as i32
                    & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 != 0
                    && *(*__ctype_b_loc())
                        .offset(*s.offset(1 as i32 as isize) as u8 as i32 as isize)
                        as i32 & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32
                        != 0) as i32 + 1 as i32,
                1 as i32,
                31 as i32,
                &mut (*t).tm.tm_mday,
            );
            current_block_61 = 919954187481050311;
        }
        104 => {
            let mut frac: i32 = 0;
            s = parse_decimal(
                s,
                2 as i32,
                0 as i32,
                23 as i32,
                60 as i32 * 60 as i32,
                &mut (*t).tm.tm_hour,
                &mut frac,
            );
            (*t).tm.tm_min = frac / 60 as i32;
            (*t).tm.tm_sec = frac % 60 as i32;
            current_block_61 = 919954187481050311;
        }
        109 => {
            s = parse_decimal(
                s,
                2 as i32,
                0 as i32,
                59 as i32,
                60 as i32,
                &mut (*t).tm.tm_min,
                &mut (*t).tm.tm_sec,
            );
            current_block_61 = 919954187481050311;
        }
        110 => {
            (*t).tm.tm_mon = lookup(s, month_names.as_ptr());
            if !(0 as i32 <= (*t).tm.tm_mon) {
                return 0 as *const i8;
            }
            while *(*__ctype_b_loc()).offset(*s as u8 as i32 as isize) as i32
                & C2RustUnnamed::_ISalpha as i32 as libc::c_ushort as i32 != 0
            {
                s = s.offset(1);
                s;
            }
            current_block_61 = 919954187481050311;
        }
        78 => {
            s = parse_ranged(s, 2 as i32, 1 as i32, 12 as i32, &mut (*t).tm.tm_mon);
            (*t).tm.tm_mon -= 1;
            (*t).tm.tm_mon;
            current_block_61 = 919954187481050311;
        }
        114 => {
            s = parse_fixed(s, 1 as i32, &mut (*t).tm.tm_year);
            (*t).ymodulus = 10 as i32;
            current_block_61 = 919954187481050311;
        }
        82 => {
            current_block_61 = 11730582894990304640;
        }
        115 => {
            let mut frac_0: i32 = 0;
            s = parse_decimal(
                s,
                2 as i32,
                0 as i32,
                60 as i32,
                1 as i32,
                &mut (*t).tm.tm_sec,
                &mut frac_0,
            );
            (*t).tm.tm_sec += frac_0;
            current_block_61 = 919954187481050311;
        }
        84 => {
            let fresh5 = s;
            s = s.offset(1);
            match *fresh5 as i32 {
                84 | 116 => {}
                _ => return 0 as *const i8,
            }
            current_block_61 = 919954187481050311;
        }
        116 => {
            s = parse_ranged(
                s,
                (*(*__ctype_b_loc())
                    .offset(*s.offset(0 as i32 as isize) as u8 as i32 as isize) as i32
                    & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 != 0
                    && *(*__ctype_b_loc())
                        .offset(*s.offset(1 as i32 as isize) as u8 as i32 as isize)
                        as i32 & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32
                        != 0) as i32 + 1 as i32,
                1 as i32,
                12 as i32,
                &mut (*t).tm.tm_hour,
            );
            current_block_61 = 919954187481050311;
        }
        119 => {
            let fresh6 = s;
            s = s.offset(1);
            match *fresh6 as i32 {
                87 | 119 => {}
                _ => return 0 as *const i8,
            }
            current_block_61 = 919954187481050311;
        }
        87 => {
            let fresh7 = s;
            s = s.offset(1);
            match *fresh7 as i32 {
                87 | 119 => {}
                _ => return 0 as *const i8,
            }
            s = parse_ranged(s, 2 as i32, 0 as i32, 53 as i32, &mut (*t).yweek);
            current_block_61 = 919954187481050311;
        }
        88 => {
            s = parse_ranged(s, 1 as i32, 1 as i32, 7 as i32, &mut (*t).tm.tm_wday);
            (*t).tm.tm_wday %= 7 as i32;
            current_block_61 = 919954187481050311;
        }
        120 => {
            (*t).tm.tm_wday = lookup(s, weekday_names.as_ptr());
            if !(0 as i32 <= (*t).tm.tm_wday) {
                return 0 as *const i8;
            }
            while *(*__ctype_b_loc()).offset(*s as u8 as i32 as isize) as i32
                & C2RustUnnamed::_ISalpha as i32 as libc::c_ushort as i32 != 0
            {
                s = s.offset(1);
                s;
            }
            current_block_61 = 919954187481050311;
        }
        121 => {
            if *(*__ctype_b_loc())
                .offset(*s.offset(0 as i32 as isize) as u8 as i32 as isize) as i32
                & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 != 0
                && *(*__ctype_b_loc())
                    .offset(*s.offset(1 as i32 as isize) as u8 as i32 as isize) as i32
                    & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 != 0
                && *(*__ctype_b_loc())
                    .offset(*s.offset(2 as i32 as isize) as u8 as i32 as isize) as i32
                    & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 == 0
            {
                current_block_61 = 11730582894990304640;
            } else {
                current_block_61 = 14104132948982966916;
            }
        }
        89 => {
            current_block_61 = 14104132948982966916;
        }
        90 => {
            s = parzone(s, &mut (*t).zone);
            current_block_61 = 919954187481050311;
        }
        95 => {
            while *(*__ctype_b_loc()).offset(*s as i32 as isize) as i32
                & C2RustUnnamed::_ISalnum as i32 as libc::c_ushort as i32 == 0
                && *s as i32 != 0
            {
                s = s.offset(1);
                s;
            }
            current_block_61 = 919954187481050311;
        }
        _ => return 0 as *const i8,
    }
    match current_block_61 {
        11730582894990304640 => {
            s = parse_fixed(s, 2 as i32, &mut (*t).tm.tm_year);
            (*t).ymodulus = 100 as i32;
        }
        14104132948982966916 => {
            let mut len: i32 = 0 as i32;
            while *(*__ctype_b_loc())
                .offset(*s.offset(len as isize) as u8 as i32 as isize) as i32
                & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 != 0
            {
                len += 1;
                len;
            }
            if len < 4 as i32 {
                return 0 as *const i8;
            }
            s = parse_fixed(s, len, &mut (*t).tm.tm_year);
        }
        _ => {}
    }
    return s;
}
unsafe extern "C" fn parse_prefix(
    mut str: *const i8,
    mut t: *mut partime,
    mut pi: *mut i32,
) -> *const i8 {
    let mut i: i32 = *pi;
    let mut pat: *const i8 = 0 as *const i8;
    let mut c: u8 = 0;
    if i < 0 as i32 {
        return 0 as *const i8;
    }
    loop {
        c = *str as u8;
        if !(*(*__ctype_b_loc()).offset(c as i32 as isize) as i32
            & C2RustUnnamed::_ISalnum as i32 as libc::c_ushort as i32 == 0
            && c as i32 != '-' as i32 && c as i32 != '+' as i32)
        {
            break;
        }
        if c == 0 {
            undefine(t);
            *pi = -(1 as i32);
            return str;
        }
        str = str.offset(1);
        str;
    }
    loop {
        let fresh8 = i;
        i = i + 1;
        pat = patterns[fresh8 as usize];
        if pat.is_null() {
            break;
        }
        let mut s: *const i8 = str;
        undefine(t);
        loop {
            let fresh9 = pat;
            pat = pat.offset(1);
            c = *fresh9 as u8;
            if c == 0 {
                *pi = i;
                return s;
            }
            s = parse_pattern_letter(s, c as i32, t);
            if s.is_null() {
                break;
            }
        }
    }
    return 0 as *const i8;
}
unsafe extern "C" fn merge_partime(mut t: *mut partime, mut u: *const partime) -> i32 {
    if (*t).tm.tm_sec != (*u).tm.tm_sec && 0 as i32 <= (*t).tm.tm_sec
        && 0 as i32 <= (*u).tm.tm_sec
        || (*t).tm.tm_min != (*u).tm.tm_min && 0 as i32 <= (*t).tm.tm_min
            && 0 as i32 <= (*u).tm.tm_min
        || (*t).tm.tm_hour != (*u).tm.tm_hour && 0 as i32 <= (*t).tm.tm_hour
            && 0 as i32 <= (*u).tm.tm_hour
        || (*t).tm.tm_mday != (*u).tm.tm_mday && 0 as i32 <= (*t).tm.tm_mday
            && 0 as i32 <= (*u).tm.tm_mday
        || (*t).tm.tm_mon != (*u).tm.tm_mon && 0 as i32 <= (*t).tm.tm_mon
            && 0 as i32 <= (*u).tm.tm_mon
        || (*t).tm.tm_year != (*u).tm.tm_year && 0 as i32 <= (*t).tm.tm_year
            && 0 as i32 <= (*u).tm.tm_year
        || (*t).tm.tm_wday != (*u).tm.tm_wday && 0 as i32 <= (*t).tm.tm_wday
            && 0 as i32 <= (*u).tm.tm_wday
        || (*t).tm.tm_yday != (*u).tm.tm_yday && 0 as i32 <= (*t).tm.tm_yday
            && 0 as i32 <= (*u).tm.tm_yday
        || (*t).ymodulus != (*u).ymodulus && 0 as i32 <= (*t).ymodulus
            && 0 as i32 <= (*u).ymodulus
        || (*t).yweek != (*u).yweek && 0 as i32 <= (*t).yweek && 0 as i32 <= (*u).yweek
        || (*t).zone != (*u).zone
            && (*t).zone != -(24 as i32) as i64 * 60 as i32 as i64 * 60 as i32 as i64
            && (*u).zone != -(24 as i32) as i64 * 60 as i32 as i64 * 60 as i32 as i64
    {
        return -(1 as i32);
    }
    if 0 as i32 <= (*u).tm.tm_sec {
        (*t).tm.tm_sec = (*u).tm.tm_sec;
    }
    if 0 as i32 <= (*u).tm.tm_min {
        (*t).tm.tm_min = (*u).tm.tm_min;
    }
    if 0 as i32 <= (*u).tm.tm_hour {
        (*t).tm.tm_hour = (*u).tm.tm_hour;
    }
    if 0 as i32 <= (*u).tm.tm_mday {
        (*t).tm.tm_mday = (*u).tm.tm_mday;
    }
    if 0 as i32 <= (*u).tm.tm_mon {
        (*t).tm.tm_mon = (*u).tm.tm_mon;
    }
    if 0 as i32 <= (*u).tm.tm_year {
        (*t).tm.tm_year = (*u).tm.tm_year;
    }
    if 0 as i32 <= (*u).tm.tm_wday {
        (*t).tm.tm_wday = (*u).tm.tm_wday;
    }
    if 0 as i32 <= (*u).tm.tm_yday {
        (*t).tm.tm_yday = (*u).tm.tm_yday;
    }
    if 0 as i32 <= (*u).ymodulus {
        (*t).ymodulus = (*u).ymodulus;
    }
    if 0 as i32 <= (*u).yweek {
        (*t).yweek = (*u).yweek;
    }
    if (*u).zone != -(24 as i32) as i64 * 60 as i32 as i64 * 60 as i32 as i64 {
        (*t).zone = (*u).zone;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn partime(mut s: *const i8, mut t: *mut partime) -> *const i8 {
    let mut p: partime = partime {
        tm: tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            tm_gmtoff: 0,
            tm_zone: 0 as *const i8,
        },
        ymodulus: 0,
        yweek: 0,
        zone: 0,
    };
    undefine(t);
    while *s != 0 {
        let mut i: i32 = 0 as i32;
        let mut s1: *const i8 = 0 as *const i8;
        loop {
            s1 = parse_prefix(s, &mut p, &mut i);
            if s1.is_null() {
                return s;
            }
            if !(0 as i32 > merge_partime(t, &mut p)) {
                break;
            }
        }
        s = s1;
    }
    return s;
}
unsafe extern "C" fn run_static_initializers() {
    zone_names = [
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"hst\0"),
                val: if -(1000 as i32) < 0 as i32 {
                    -(--(1000 as i32) / 100 as i32 * 60 as i32
                        + --(1000 as i32) % 100 as i32)
                } else {
                    -(1000 as i32) / 100 as i32 * 60 as i32 + -(1000 as i32) % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"hast"),
                val: if -(1000 as i32) < 0 as i32 {
                    -(--(1000 as i32) / 100 as i32 * 60 as i32
                        + --(1000 as i32) % 100 as i32)
                } else {
                    -(1000 as i32) / 100 as i32 * 60 as i32 + -(1000 as i32) % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"hadt"),
                val: if (-(1000 as i32) + 100 as i32) < 0 as i32 {
                    -(-(-(1000 as i32) + 100 as i32) / 100 as i32 * 60 as i32
                        + -(-(1000 as i32) + 100 as i32) % 100 as i32)
                } else {
                    (-(1000 as i32) + 100 as i32) / 100 as i32 * 60 as i32
                        + (-(1000 as i32) + 100 as i32) % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"akst"),
                val: if -(900 as i32) < 0 as i32 {
                    -(--(900 as i32) / 100 as i32 * 60 as i32
                        + --(900 as i32) % 100 as i32)
                } else {
                    -(900 as i32) / 100 as i32 * 60 as i32 + -(900 as i32) % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"akdt"),
                val: if (-(900 as i32) + 100 as i32) < 0 as i32 {
                    -(-(-(900 as i32) + 100 as i32) / 100 as i32 * 60 as i32
                        + -(-(900 as i32) + 100 as i32) % 100 as i32)
                } else {
                    (-(900 as i32) + 100 as i32) / 100 as i32 * 60 as i32
                        + (-(900 as i32) + 100 as i32) % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"pst\0"),
                val: if -(800 as i32) < 0 as i32 {
                    -(--(800 as i32) / 100 as i32 * 60 as i32
                        + --(800 as i32) % 100 as i32)
                } else {
                    -(800 as i32) / 100 as i32 * 60 as i32 + -(800 as i32) % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"pdt\0"),
                val: if (-(800 as i32) + 100 as i32) < 0 as i32 {
                    -(-(-(800 as i32) + 100 as i32) / 100 as i32 * 60 as i32
                        + -(-(800 as i32) + 100 as i32) % 100 as i32)
                } else {
                    (-(800 as i32) + 100 as i32) / 100 as i32 * 60 as i32
                        + (-(800 as i32) + 100 as i32) % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"mst\0"),
                val: if -(700 as i32) < 0 as i32 {
                    -(--(700 as i32) / 100 as i32 * 60 as i32
                        + --(700 as i32) % 100 as i32)
                } else {
                    -(700 as i32) / 100 as i32 * 60 as i32 + -(700 as i32) % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"mdt\0"),
                val: if (-(700 as i32) + 100 as i32) < 0 as i32 {
                    -(-(-(700 as i32) + 100 as i32) / 100 as i32 * 60 as i32
                        + -(-(700 as i32) + 100 as i32) % 100 as i32)
                } else {
                    (-(700 as i32) + 100 as i32) / 100 as i32 * 60 as i32
                        + (-(700 as i32) + 100 as i32) % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"cst\0"),
                val: if -(600 as i32) < 0 as i32 {
                    -(--(600 as i32) / 100 as i32 * 60 as i32
                        + --(600 as i32) % 100 as i32)
                } else {
                    -(600 as i32) / 100 as i32 * 60 as i32 + -(600 as i32) % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"cdt\0"),
                val: if (-(600 as i32) + 100 as i32) < 0 as i32 {
                    -(-(-(600 as i32) + 100 as i32) / 100 as i32 * 60 as i32
                        + -(-(600 as i32) + 100 as i32) % 100 as i32)
                } else {
                    (-(600 as i32) + 100 as i32) / 100 as i32 * 60 as i32
                        + (-(600 as i32) + 100 as i32) % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"est\0"),
                val: if -(500 as i32) < 0 as i32 {
                    -(--(500 as i32) / 100 as i32 * 60 as i32
                        + --(500 as i32) % 100 as i32)
                } else {
                    -(500 as i32) / 100 as i32 * 60 as i32 + -(500 as i32) % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"edt\0"),
                val: if (-(500 as i32) + 100 as i32) < 0 as i32 {
                    -(-(-(500 as i32) + 100 as i32) / 100 as i32 * 60 as i32
                        + -(-(500 as i32) + 100 as i32) % 100 as i32)
                } else {
                    (-(500 as i32) + 100 as i32) / 100 as i32 * 60 as i32
                        + (-(500 as i32) + 100 as i32) % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"ast\0"),
                val: if -(400 as i32) < 0 as i32 {
                    -(--(400 as i32) / 100 as i32 * 60 as i32
                        + --(400 as i32) % 100 as i32)
                } else {
                    -(400 as i32) / 100 as i32 * 60 as i32 + -(400 as i32) % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"adt\0"),
                val: if (-(400 as i32) + 100 as i32) < 0 as i32 {
                    -(-(-(400 as i32) + 100 as i32) / 100 as i32 * 60 as i32
                        + -(-(400 as i32) + 100 as i32) % 100 as i32)
                } else {
                    (-(400 as i32) + 100 as i32) / 100 as i32 * 60 as i32
                        + (-(400 as i32) + 100 as i32) % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"nst\0"),
                val: if -(330 as i32) < 0 as i32 {
                    -(--(330 as i32) / 100 as i32 * 60 as i32
                        + --(330 as i32) % 100 as i32)
                } else {
                    -(330 as i32) / 100 as i32 * 60 as i32 + -(330 as i32) % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"ndt\0"),
                val: if (-(330 as i32) + 100 as i32) < 0 as i32 {
                    -(-(-(330 as i32) + 100 as i32) / 100 as i32 * 60 as i32
                        + -(-(330 as i32) + 100 as i32) % 100 as i32)
                } else {
                    (-(330 as i32) + 100 as i32) / 100 as i32 * 60 as i32
                        + (-(330 as i32) + 100 as i32) % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"utc\0"),
                val: if (0 as i32) < 0 as i32 {
                    -(-(0 as i32) / 100 as i32 * 60 as i32 + -(0 as i32) % 100 as i32)
                } else {
                    0 as i32 / 100 as i32 * 60 as i32 + 0 as i32 % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"cut\0"),
                val: if (0 as i32) < 0 as i32 {
                    -(-(0 as i32) / 100 as i32 * 60 as i32 + -(0 as i32) % 100 as i32)
                } else {
                    0 as i32 / 100 as i32 * 60 as i32 + 0 as i32 % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"ut\0\0"),
                val: if (0 as i32) < 0 as i32 {
                    -(-(0 as i32) / 100 as i32 * 60 as i32 + -(0 as i32) % 100 as i32)
                } else {
                    0 as i32 / 100 as i32 * 60 as i32 + 0 as i32 % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"z\0\0\0"),
                val: if (0 as i32) < 0 as i32 {
                    -(-(0 as i32) / 100 as i32 * 60 as i32 + -(0 as i32) % 100 as i32)
                } else {
                    0 as i32 / 100 as i32 * 60 as i32 + 0 as i32 % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"gmt\0"),
                val: if (0 as i32) < 0 as i32 {
                    -(-(0 as i32) / 100 as i32 * 60 as i32 + -(0 as i32) % 100 as i32)
                } else {
                    0 as i32 / 100 as i32 * 60 as i32 + 0 as i32 % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"bst\0"),
                val: if (0 as i32 + 100 as i32) < 0 as i32 {
                    -(-(0 as i32 + 100 as i32) / 100 as i32 * 60 as i32
                        + -(0 as i32 + 100 as i32) % 100 as i32)
                } else {
                    (0 as i32 + 100 as i32) / 100 as i32 * 60 as i32
                        + (0 as i32 + 100 as i32) % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"wet\0"),
                val: if (0 as i32) < 0 as i32 {
                    -(-(0 as i32) / 100 as i32 * 60 as i32 + -(0 as i32) % 100 as i32)
                } else {
                    0 as i32 / 100 as i32 * 60 as i32 + 0 as i32 % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"met\0"),
                val: if (100 as i32) < 0 as i32 {
                    -(-(100 as i32) / 100 as i32 * 60 as i32
                        + -(100 as i32) % 100 as i32)
                } else {
                    100 as i32 / 100 as i32 * 60 as i32 + 100 as i32 % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"cet\0"),
                val: if (100 as i32) < 0 as i32 {
                    -(-(100 as i32) / 100 as i32 * 60 as i32
                        + -(100 as i32) % 100 as i32)
                } else {
                    100 as i32 / 100 as i32 * 60 as i32 + 100 as i32 % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"eet\0"),
                val: if (200 as i32) < 0 as i32 {
                    -(-(200 as i32) / 100 as i32 * 60 as i32
                        + -(200 as i32) % 100 as i32)
                } else {
                    200 as i32 / 100 as i32 * 60 as i32 + 200 as i32 % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"ist\0"),
                val: if (530 as i32) < 0 as i32 {
                    -(-(530 as i32) / 100 as i32 * 60 as i32
                        + -(530 as i32) % 100 as i32)
                } else {
                    530 as i32 / 100 as i32 * 60 as i32 + 530 as i32 % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"jst\0"),
                val: if (900 as i32) < 0 as i32 {
                    -(-(900 as i32) / 100 as i32 * 60 as i32
                        + -(900 as i32) % 100 as i32)
                } else {
                    900 as i32 / 100 as i32 * 60 as i32 + 900 as i32 % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"jdt\0"),
                val: if (900 as i32 + 100 as i32) < 0 as i32 {
                    -(-(900 as i32 + 100 as i32) / 100 as i32 * 60 as i32
                        + -(900 as i32 + 100 as i32) % 100 as i32)
                } else {
                    (900 as i32 + 100 as i32) / 100 as i32 * 60 as i32
                        + (900 as i32 + 100 as i32) % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"kst\0"),
                val: if (900 as i32) < 0 as i32 {
                    -(-(900 as i32) / 100 as i32 * 60 as i32
                        + -(900 as i32) % 100 as i32)
                } else {
                    900 as i32 / 100 as i32 * 60 as i32 + 900 as i32 % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"kdt\0"),
                val: if (900 as i32 + 100 as i32) < 0 as i32 {
                    -(-(900 as i32 + 100 as i32) / 100 as i32 * 60 as i32
                        + -(900 as i32 + 100 as i32) % 100 as i32)
                } else {
                    (900 as i32 + 100 as i32) / 100 as i32 * 60 as i32
                        + (900 as i32 + 100 as i32) % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"nzst"),
                val: if (1200 as i32) < 0 as i32 {
                    -(-(1200 as i32) / 100 as i32 * 60 as i32
                        + -(1200 as i32) % 100 as i32)
                } else {
                    1200 as i32 / 100 as i32 * 60 as i32 + 1200 as i32 % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"nzdt"),
                val: if (1200 as i32 + 100 as i32) < 0 as i32 {
                    -(-(1200 as i32 + 100 as i32) / 100 as i32 * 60 as i32
                        + -(1200 as i32 + 100 as i32) % 100 as i32)
                } else {
                    (1200 as i32 + 100 as i32) / 100 as i32 * 60 as i32
                        + (1200 as i32 + 100 as i32) % 100 as i32
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"lt\0\0"),
                val: 1 as i32,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::core::mem::transmute::<&[u8; 4], &mut [i8; 4]>(b"\0\0\0\0"),
                val: -(1 as i32),
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];