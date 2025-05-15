use std::time::{SystemTime, UNIX_EPOCH};
use std::convert::TryFrom;
use std::str::FromStr;
use std::fmt;

const TM_UNDEFINED: i32 = -1;
const TM_UNDEFINED_ZONE: i64 = -24 * 60 * 60;
const TM_LOCAL_ZONE: i64 = TM_UNDEFINED_ZONE - 1;

#[derive(Debug, Clone, Default)]
pub struct Tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
}

#[derive(Debug, Clone, Default)]
pub struct Partime {
    pub tm: Tm,
    pub ymodulus: i32,
    pub yweek: i32,
    pub zone: i64,
}

impl Partime {
    fn tm_defined(x: i32) -> bool {
        x >= 0
    }

    fn undefine(&mut self) {
        self.tm.tm_sec = TM_UNDEFINED;
        self.tm.tm_min = TM_UNDEFINED;
        self.tm.tm_hour = TM_UNDEFINED;
        self.tm.tm_mday = TM_UNDEFINED;
        self.tm.tm_mon = TM_UNDEFINED;
        self.tm.tm_year = TM_UNDEFINED;
        self.tm.tm_wday = TM_UNDEFINED;
        self.tm.tm_yday = TM_UNDEFINED;
        self.ymodulus = TM_UNDEFINED;
        self.yweek = TM_UNDEFINED;
        self.zone = TM_UNDEFINED_ZONE;
    }
}

const NAME_LENGTH_MAXIMUM: usize = 4;

#[derive(Debug, Clone)]
struct NameVal {
    name: [u8; NAME_LENGTH_MAXIMUM],
    val: i32,
}

impl NameVal {
    fn new(name: &str, val: i32) -> Self {
        let mut n = [0; NAME_LENGTH_MAXIMUM];
        let bytes = name.as_bytes();
        let len = bytes.len().min(NAME_LENGTH_MAXIMUM);
        n[..len].copy_from_slice(&bytes[..len]);
        NameVal { name: n, val }
    }
}

static MONTH_NAMES: [NameVal; 13] = [
    NameVal::new("jan", 0),
    NameVal::new("feb", 1),
    NameVal::new("mar", 2),
    NameVal::new("apr", 3),
    NameVal::new("may", 4),
    NameVal::new("jun", 5),
    NameVal::new("jul", 6),
    NameVal::new("aug", 7),
    NameVal::new("sep", 8),
    NameVal::new("oct", 9),
    NameVal::new("nov", 10),
    NameVal::new("dec", 11),
    NameVal::new("", TM_UNDEFINED),
];

static WEEKDAY_NAMES: [NameVal; 8] = [
    NameVal::new("sun", 0),
    NameVal::new("mon", 1),
    NameVal::new("tue", 2),
    NameVal::new("wed", 3),
    NameVal::new("thu", 4),
    NameVal::new("fri", 5),
    NameVal::new("sat", 6),
    NameVal::new("", TM_UNDEFINED),
];

fn hr60_nonnegative(t: i32) -> i32 {
    (t / 100) * 60 + (t % 100)
}

fn hr60(t: i32) -> i32 {
    if t < 0 {
        -hr60_nonnegative(-t)
    } else {
        hr60_nonnegative(t)
    }
}

fn zs(t: i32, s: &str) -> NameVal {
    NameVal::new(s, hr60(t))
}

fn zd(t: i32, s: &str, d: &str) -> [NameVal; 2] {
    [zs(t, s), zs(t + 100, d)]
}

static ZONE_NAMES: [NameVal; 46] = [
    zs(-1000, "hst"),
    zs(-1000, "hast"),
    zs(-900, "hadt"),
    zs(-900, "akst"),
    zs(-800, "akdt"),
    zs(-800, "pst"),
    zs(-700, "pdt"),
    zs(-700, "mst"),
    zs(-600, "mdt"),
    zs(-600, "cst"),
    zs(-500, "cdt"),
    zs(-500, "est"),
    zs(-400, "edt"),
    zs(-400, "ast"),
    zs(-330, "adt"),
    zs(-330, "nst"),
    zs(0, "ndt"),
    zs(0, "utc"),
    zs(0, "cut"),
    zs(0, "ut"),
    zs(0, "z"),
    zs(0, "gmt"),
    zs(100, "bst"),
    zs(100, "wet"),
    zs(100, "met"),
    zs(100, "cet"),
    zs(200, "eet"),
    zs(530, "ist"),
    zs(900, "jst"),
    zs(900, "jdt"),
    zs(900, "kst"),
    zs(1200, "kdt"),
    zs(1200, "nzst"),
    zs(1200, "nzdt"),
    NameVal::new("lt", 1),
    NameVal::new("", -1),
];

fn lookup(s: &str, table: &[NameVal]) -> i32 {
    let mut buf = [0; NAME_LENGTH_MAXIMUM];
    let mut j = 0;
    
    for (i, c) in s.chars().enumerate().take(NAME_LENGTH_MAXIMUM) {
        if !c.is_alphabetic() {
            break;
        }
        buf[i] = c.to_ascii_lowercase() as u8;
        j = i + 1;
    }
    
    for entry in table {
        let mut k = 0;
        while k < j && buf[k] == entry.name[k] {
            k += 1;
        }
        if k == j && (entry.name[k] == 0 || j == NAME_LENGTH_MAXIMUM) {
            return entry.val;
        }
    }
    
    if !table.is_empty() {
        table.last().unwrap().val
    } else {
        -1
    }
}

static PATTERNS: &[&str] = &[
    "E_n_y", "x",
    "E_n", "n_E", "n", "t:m:s_A", "t:m_A", "t_A",
    "y/N/D$",
    "y-N-D$", "4ND$", "Y-N$",
    "RND$", "-R=N$", "-R$", "--N=D$", "N=DT",
    "--N$", "---D$", "DT",
    "Y-d$", "4d$", "R=d$", "-d$", "dT",
    "y-W-X", "yWX", "y=W",
    "-r-W-X", "r-W-XT", "-rWX", "rWXT", "-W=X", "W=XT", "-W",
    "-w-X", "w-XT", "---X$", "XT", "4$",
    "T",
    "h:m:s$", "hms$", "h:m$", "hm$", "h$", "-m:s$", "-ms$", "-m$", "--s$",
    "Y", "Z",
];

fn parse_fixed(s: &str, digits: usize) -> Option<(i32, &str)> {
    if s.len() < digits {
        return None;
    }
    
    let (num_str, rest) = s.split_at(digits);
    match num_str.parse::<i32>() {
        Ok(n) => Some((n, rest)),
        Err(_) => None,
    }
}

fn parse_ranged(s: &str, digits: usize, lo: i32, hi: i32) -> Option<(i32, &str)> {
    parse_fixed(s, digits).and_then(|(n, rest)| {
        if lo <= n && n <= hi {
            Some((n, rest))
        } else {
            None
        }
    })
}

fn parse_decimal(
    s: &str,
    digits: usize,
    lo: i32,
    hi: i32,
    resolution: i32,
) -> Option<(i32, i32, &str)> {
    parse_ranged(s, digits, lo, hi).and_then(|(n, rest)| {
        let mut frac = 0;
        let mut new_rest = rest;
        
        if rest.starts_with(',') || rest.starts_with('.') {
            let frac_part = rest[1..]
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>();
            if !frac_part.is_empty() {
                let frac_digits = frac_part.len();
                let num10: i32 = frac_part.parse().unwrap_or(0);
                let denom10 = 10i32.pow(frac_digits as u32);
                let product = num10 * resolution;
                frac = (product + (denom10 >> 1)) / denom10;
                if product % denom10 == denom10 >> 1 {
                    frac -= frac & 1; // round to even
                }
                new_rest = &rest[1 + frac_digits..];
            }
        }
        
        Some((n, frac, new_rest))
    })
}

pub fn parzone(s: &str) -> Option<(i64, &str)> {
    let s = s.trim_start();
    if s.is_empty() {
        return None;
    }

    let (sign, mut s) = match s.chars().next() {
        Some('+') => (1, &s[1..]),
        Some('-') => (-1, &s[1..]),
        _ => {
            let minutes_east = lookup(s, &ZONE_NAMES);
            if minutes_east == -1 {
                return None;
            }

            let mut end = 0;
            while end < s.len() && s[end..].chars().next().unwrap().is_alphabetic() {
                end += 1;
            }

            if minutes_east == 1 {
                return Some((TM_LOCAL_ZONE, &s[end..]));
            }

            let z = minutes_east as i64 * 60;
            let s = &s[end..];

            if s.len() >= 3 && s[..3].eq_ignore_ascii_case("dst") {
                return Some((z + 3600, &s[3..]));
            }

            let s = s.trim_start();
            if s.len() >= 3 && s[..3].eq_ignore_ascii_case("dst") {
                return Some((z + 3600, &s[3..]));
            }

            match s.chars().next() {
                Some('+') | Some('-') => (1, s),
                _ => return Some((z, s)),
            }
        }
    };

    let (hh, rest) = parse_ranged(s, 2, 0, 23)?;
    let (mm, rest) = if rest.starts_with(':') {
        parse_ranged(&rest[1..], 2, 0, 59)?
    } else {
        (0, rest)
    };
    let (ss, rest) = if rest.starts_with(':') && rest.len() > 3 && rest[1..3].chars().all(|c| c.is_ascii_digit()) {
        parse_ranged(&rest[1..], 2, 0, 59)?
    } else {
        (0, rest)
    };

    let offset = (hh * 60 + mm) * 60 + ss;
    let zone = sign * offset;
    Some((zone, rest))
}

fn parse_pattern_letter(s: &str, c: char, t: &mut Partime) -> Option<&str> {
    match c {
        '$' => {
            if s.chars().next()?.is_ascii_digit() {
                None
            } else {
                Some(s)
            }
        }
        '-' | '/' | ':' => {
            if s.starts_with(c) {
                Some(&s[1..])
            } else {
                None
            }
        }
        '4' => {
            let (year, rest) = parse_fixed(s, 4)?;
            t.tm.tm_year = year;
            Some(rest)
        }
        '=' => {
            if s.starts_with('-') {
                Some(&s[1..])
            } else {
                Some(s)
            }
        }
        'A' => {
            let mut chars = s.chars();
            let first = chars.next()?;
            let hour = match first.to_ascii_uppercase() {
                'A' => {
                    if t.tm.tm_hour == 12 {
                        0
                    } else {
                        t.tm.tm_hour
                    }
                }
                'P' => {
                    if t.tm.tm_hour != 12 {
                        t.tm.tm_hour + 12
                    } else {
                        t.tm.tm_hour
                    }
                }
                _ => return None,
            };
            t.tm.tm_hour = hour;
            let rest = chars.as_str();
            if rest.starts_with('M') || rest.starts_with('m') {
                Some(&rest[1..])
            } else {
                Some(rest)
            }
        }
        'D' => {
            let (mday, rest) = parse_ranged(s, 2, 1, 31)?;
            t.tm.tm_mday = mday;
            Some(rest)
        }
        'd' => {
            let (yday, rest) = parse_ranged(s, 3, 1, 366)?;
            t.tm.tm_yday = yday - 1;
            Some(rest)
        }
        'E' => {
            let digits = if s.len() >= 2 && s[..2].chars().all(|c| c.is_ascii_digit()) {
                2
            } else {
                1
            };
            let (mday, rest) = parse_ranged(s, digits, 1, 31)?;
            t.tm.tm_mday = mday;
            Some(rest)
        }
        'h' => {
            let (hour, frac, rest) = parse_decimal(s, 2, 0, 23, 60 * 60)?;
            t.tm.tm_hour = hour;
            t.tm.tm_min = frac / 60;
            t.tm.tm_sec = frac % 60;
            Some(rest)
        }
        'm' => {
            let (min, sec, rest) = parse_decimal(s, 2, 0, 59, 60)?;
            t.tm.tm_min = min;
            t.tm.tm_sec = sec;
            Some(rest)
        }
        'n' => {
            let mon = lookup(s, &MONTH_NAMES);
            if !Partime::tm_defined(mon) {
                return None;
            }
            t.tm.tm_mon = mon;
            let mut end = 0;
            while end < s.len() && s[end..].chars().next().unwrap().is_alphabetic() {
                end += 1;
            }
            Some(&s[end..])
        }
        'N' => {
            let (mon, rest) = parse_ranged(s, 2, 1, 12)?;
            t.tm.tm_mon = mon - 1;
            Some(rest)
        }
        'r' => {
            let (year, rest) = parse_fixed(s, 1)?;
            t.tm.tm_year = year;
            t.ymodulus = 10;
            Some(rest)
        }
        'R' => {
            let (year, rest) = parse_fixed(s, 2)?;
            t.tm.tm_year = year;
            t.ymodulus = 100;
            Some(rest)
        }
        's' => {
            let (sec, frac, rest) = parse_decimal(s, 2, 0, 60, 1)?;
            t.tm.tm_sec = sec + frac;
            Some(rest)
        }
        'T' => {
            if s.starts_with('T') || s.starts_with('t') {
                Some(&s[1..])
            } else {
                None
            }
        }
        't' => {
            let digits = if s.len() >= 2 && s[..2].chars().all(|c| c.is_ascii_digit()) {
                2
            } else {
                1
            };
            let (hour, rest) = parse_ranged(s, digits, 1, 12)?;
            t.tm.tm_hour = hour;
            Some(rest)
        }
        'w' => {
            if s.starts_with('W') || s.starts_with('w') {
                Some(&s[1..])
            } else {
                None
            }
        }
        'W' => {
            let rest = if s.starts_with('W') || s.starts_with('w') {
                &s[1..]
            } else {
                return None;
            };
            let (week, rest) = parse_ranged(rest, 2, 0, 53)?;
            t.yweek = week;
            Some(rest)
        }
        'X' => {
            let (wday, rest) = parse_ranged(s, 1, 1,