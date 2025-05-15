use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_long, c_uchar, c_ushort};

#[derive(Debug, Clone, Copy)]
pub struct Tm {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
    pub tm_gmtoff: c_long,
    pub tm_zone: *const c_char,
}

#[derive(Debug, Clone, Copy)]
pub struct Partime {
    pub tm: Tm,
    pub ymodulus: c_int,
    pub yweek: c_int,
    pub zone: c_long,
}

#[derive(Debug, Clone, Copy)]
pub struct NameVal {
    pub name: [c_char; 4],
    pub val: c_int,
}

const _ISalnum: c_int = 8;
const _ISpunct: c_int = 4;
const _IScntrl: c_int = 2;
const _ISblank: c_int = 1;
const _ISgraph: c_int = 32768;
const _ISprint: c_int = 16384;
const _ISspace: c_int = 8192;
const _ISxdigit: c_int = 4096;
const _ISdigit: c_int = 2048;
const _ISalpha: c_int = 1024;
const _ISlower: c_int = 512;
const _ISupper: c_int = 256;

static MONTH_NAMES: [NameVal; 13] = [
    NameVal { name: [b'j' as c_char, b'a' as c_char, b'n' as c_char, 0], val: 0 },
    NameVal { name: [b'f' as c_char, b'e' as c_char, b'b' as c_char, 0], val: 1 },
    NameVal { name: [b'm' as c_char, b'a' as c_char, b'r' as c_char, 0], val: 2 },
    NameVal { name: [b'a' as c_char, b'p' as c_char, b'r' as c_char, 0], val: 3 },
    NameVal { name: [b'm' as c_char, b'a' as c_char, b'y' as c_char, 0], val: 4 },
    NameVal { name: [b'j' as c_char, b'u' as c_char, b'n' as c_char, 0], val: 5 },
    NameVal { name: [b'j' as c_char, b'u' as c_char, b'l' as c_char, 0], val: 6 },
    NameVal { name: [b'a' as c_char, b'u' as c_char, b'g' as c_char, 0], val: 7 },
    NameVal { name: [b's' as c_char, b'e' as c_char, b'p' as c_char, 0], val: 8 },
    NameVal { name: [b'o' as c_char, b'c' as c_char, b't' as c_char, 0], val: 9 },
    NameVal { name: [b'n' as c_char, b'o' as c_char, b'v' as c_char, 0], val: 10 },
    NameVal { name: [b'd' as c_char, b'e' as c_char, b'c' as c_char, 0], val: 11 },
    NameVal { name: [0, 0, 0, 0], val: -1 },
];

static WEEKDAY_NAMES: [NameVal; 8] = [
    NameVal { name: [b's' as c_char, b'u' as c_char, b'n' as c_char, 0], val: 0 },
    NameVal { name: [b'm' as c_char, b'o' as c_char, b'n' as c_char, 0], val: 1 },
    NameVal { name: [b't' as c_char, b'u' as c_char, b'e' as c_char, 0], val: 2 },
    NameVal { name: [b'w' as c_char, b'e' as c_char, b'd' as c_char, 0], val: 3 },
    NameVal { name: [b't' as c_char, b'h' as c_char, b'u' as c_char, 0], val: 4 },
    NameVal { name: [b'f' as c_char, b'r' as c_char, b'i' as c_char, 0], val: 5 },
    NameVal { name: [b's' as c_char, b'a' as c_char, b't' as c_char, 0], val: 6 },
    NameVal { name: [0, 0, 0, 0], val: -1 },
];

static ZONE_NAMES: [NameVal; 36] = [
    NameVal { name: [b'h' as c_char, b's' as c_char, b't' as c_char, 0], val: -600 },
    NameVal { name: [b'h' as c_char, b'a' as c_char, b's' as c_char, b't'], val: -600 },
    NameVal { name: [b'h' as c_char, b'a' as c_char, b'd' as c_char, b't'], val: -500 },
    NameVal { name: [b'a' as c_char, b'k' as c_char, b's' as c_char, b't'], val: -540 },
    NameVal { name: [b'a' as c_char, b'k' as c_char, b'd' as c_char, b't'], val: -480 },
    NameVal { name: [b'p' as c_char, b's' as c_char, b't' as c_char, 0], val: -480 },
    NameVal { name: [b'p' as c_char, b'd' as c_char, b't' as c_char, 0], val: -420 },
    NameVal { name: [b'm' as c_char, b's' as c_char, b't' as c_char, 0], val: -420 },
    NameVal { name: [b'm' as c_char, b'd' as c_char, b't' as c_char, 0], val: -360 },
    NameVal { name: [b'c' as c_char, b's' as c_char, b't' as c_char, 0], val: -360 },
    NameVal { name: [b'c' as c_char, b'd' as c_char, b't' as c_char, 0], val: -300 },
    NameVal { name: [b'e' as c_char, b's' as c_char, b't' as c_char, 0], val: -300 },
    NameVal { name: [b'e' as c_char, b'd' as c_char, b't' as c_char, 0], val: -240 },
    NameVal { name: [b'a' as c_char, b's' as c_char, b't' as c_char, 0], val: -240 },
    NameVal { name: [b'a' as c_char, b'd' as c_char, b't' as c_char, 0], val: -180 },
    NameVal { name: [b'n' as c_char, b's' as c_char, b't' as c_char, 0], val: -210 },
    NameVal { name: [b'n' as c_char, b'd' as c_char, b't' as c_char, 0], val: -150 },
    NameVal { name: [b'u' as c_char, b't' as c_char, b'c' as c_char, 0], val: 0 },
    NameVal { name: [b'c' as c_char, b'u' as c_char, b't' as c_char, 0], val: 0 },
    NameVal { name: [b'u' as c_char, b't' as c_char, 0, 0], val: 0 },
    NameVal { name: [b'z' as c_char, 0, 0, 0], val: 0 },
    NameVal { name: [b'g' as c_char, b'm' as c_char, b't' as c_char, 0], val: 0 },
    NameVal { name: [b'b' as c_char, b's' as c_char, b't' as c_char, 0], val: 60 },
    NameVal { name: [b'w' as c_char, b'e' as c_char, b't' as c_char, 0], val: 0 },
    NameVal { name: [b'm' as c_char, b'e' as c_char, b't' as c_char, 0], val: 60 },
    NameVal { name: [b'c' as c_char, b'e' as c_char, b't' as c_char, 0], val: 60 },
    NameVal { name: [b'e' as c_char, b'e' as c_char, b't' as c_char, 0], val: 120 },
    NameVal { name: [b'i' as c_char, b's' as c_char, b't' as c_char, 0], val: 330 },
    NameVal { name: [b'j' as c_char, b's' as c_char, b't' as c_char, 0], val: 540 },
    NameVal { name: [b'j' as c_char, b'd' as c_char, b't' as c_char, 0], val: 600 },
    NameVal { name: [b'k' as c_char, b's' as c_char, b't' as c_char, 0], val: 540 },
    NameVal { name: [b'k' as c_char, b'd' as c_char, b't' as c_char, 0], val: 600 },
    NameVal { name: [b'n' as c_char, b'z' as c_char, b's' as c_char, b't'], val: 720 },
    NameVal { name: [b'n' as c_char, b'z' as c_char, b'd' as c_char, b't'], val: 780 },
    NameVal { name: [b'l' as c_char, b't' as c_char, 0, 0], val: 1 },
    NameVal { name: [0, 0, 0, 0], val: -1 },
];

static PATTERNS: [&[u8]; 53] = [
    b"E_n_y\0",
    b"x\0",
    b"E_n\0",
    b"n_E\0",
    b"n\0",
    b"t:m:s_A\0",
    b"t:m_A\0",
    b"t_A\0",
    b"y/N/D$\0",
    b"y-N-D$\0",
    b"4ND$\0",
    b"Y-N$\0",
    b"RND$\0",
    b"-R=N$\0",
    b"-R$\0",
    b"--N=D$\0",
    b"N=DT\0",
    b"--N$\0",
    b"---D$\0",
    b"DT\0",
    b"Y-d$\0",
    b"4d$\0",
    b"R=d$\0",
    b"-d$\0",
    b"dT\0",
    b"y-W-X\0",
    b"yWX\0",
    b"y=W\0",
    b"-r-W-X\0",
    b"r-W-XT\0",
    b"-rWX\0",
    b"rWXT\0",
    b"-W=X\0",
    b"W=XT\0",
    b"-W\0",
    b"-w-X\0",
    b"w-XT\0",
    b"---X$\0",
    b"XT\0",
    b"4$\0",
    b"T\0",
    b"h:m:s$\0",
    b"hms$\0",
    b"h:m$\0",
    b"hm$\0",
    b"h$\0",
    b"-m:s$\0",
    b"-ms$\0",
    b"-m$\0",
    b"--s$\0",
    b"Y\0",
    b"Z\0",
    &[],
];

fn tolower(c: c_int) -> c_int {
    if c >= -128 && c < 256 {
        unsafe { libc::tolower(c) }
    } else {
        c
    }
}

fn lookup(s: *const c_char, table: &[NameVal]) -> c_int {
    let mut buf = [0 as c_char; 4];
    let mut j = 0;
    
    unsafe {
        let mut s_ptr = s;
        while j < 4 {
            let c = *s_ptr as c_uchar;
            buf[j as usize] = if libc::isupper(c as c_int) != 0 {
                tolower(c as c_int) as c_char
            } else {
                c as c_char
            };
            
            if libc::isalpha(c as c_int) == 0 {
                break;
            }
            j += 1;
            s_ptr = s_ptr.add(1);
        }
    }

    for entry in table {
        if entry.name[0] == 0 {
            break;
        }
        
        let mut k = 0;
        while k < 4 && buf[k as usize] == entry.name[k as usize] {
            k += 1;
            if k == 4 || entry.name[k as usize] == 0 {
                return entry.val;
            }
        }
    }
    
    -1
}

fn undefine(t: &mut Partime) {
    t.yweek = -1;
    t.ymodulus = t.yweek;
    t.tm.tm_yday = t.ymodulus;
    t.tm.tm_wday = t.tm.tm_yday;
    t.tm.tm_year = t.tm.tm_wday;
    t.tm.tm_mon = t.tm.tm_year;
    t.tm.tm_mday = t.tm.tm_mon;
    t.tm.tm_hour = t.tm.tm_mday;
    t.tm.tm_min = t.tm.tm_hour;
    t.tm.tm_sec = t.tm.tm_min;
    t.zone = -24 * 60 * 60;
}

fn parse_fixed(s: *const c_char, digits: c_int, res: &mut c_int) -> *const c_char {
    let mut n = 0;
    let mut s_ptr = s;
    
    unsafe {
        for _ in 0..digits {
            let d = (*s_ptr as c_int - '0' as i32) as c_uint;
            if d > 9 {
                return std::ptr::null();
            }
            n = n * 10 + d as c_int;
            s_ptr = s_ptr.add(1);
        }
    }
    
    *res = n;
    s_ptr
}

fn parse_ranged(s: *const c_char, digits: c_int, lo: c_int, hi: c_int, res: &mut c_int) -> *const c_char {
    let s_ptr = parse_fixed(s, digits, res);
    if s_ptr.is_null() || *res < lo || *res > hi {
        std::ptr::null()
    } else {
        s_ptr
    }
}

fn parse_decimal(
    s: *const c_char,
    digits: c_int,
    lo: c_int,
    hi: c_int,
    resolution: c_int,
    res: &mut c_int,
    fres: &mut c_int,
) -> *const c_char {
    let s_ptr = parse_fixed(s, digits, res);
    *fres = 0;
    
    if s_ptr.is_null() || *res < lo || *res > hi {
        return std::ptr::null();
    }
    
    unsafe {
        if (*s_ptr == b',' as c_char || *s_ptr == b'.' as c_char)
            && libc::isdigit(*s_ptr.add(1) as c_int) != 0
        {
            let mut s1 = s_ptr.add(1);
            let mut num10 = 0;
            let mut denom10 = 10;
            let mut product;
            
            while libc::isdigit(*s1 as c_int) != 0 {
                s1 = s1.add(1);
                denom10 *= 10;
            }
            
            let s1_ptr = parse_fixed(s_ptr.add(1), (s1 as usize - s_ptr.add(1) as usize) as c_int, &mut num10);
            if s1_ptr.is_null() {
                return std::ptr::null();
            }
            
            product = num10 * resolution;
            let mut f = (product + (denom10 / 2)) / denom10;
            f -= (f & ((product % denom10 == denom10 / 2) as c_int));
            
            if f < 0 || product / resolution != num10 {
                return std::ptr::null();
            }
            
            *fres = f;
            s1
        } else {
            s_ptr
        }
    }
}

pub fn parzone(s: *const c_char, zone: &mut c_long) -> *const c_char {
    let mut sign = 0;
    let mut hh = 0;
    let mut mm = 0;
    let mut ss = 0;
    let mut minutes_east_of_utc = 0;
    let mut offset = 0;
    let mut z = 0;
    
    unsafe {
        match *s as c_int {
            b'-