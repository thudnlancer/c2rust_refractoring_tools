use std::cmp;
use std::env;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::c_char;
use std::ptr;

const LONGEST_HUMAN_READABLE: usize = ((2 * mem::size_of::<u64>() * 146 / 485 + 1) * (4 + 1) - 4 + 1 + 3);

pub const HUMAN_CEILING: i32 = 0;
pub const HUMAN_ROUND_TO_NEAREST: i32 = 1;
pub const HUMAN_FLOOR: i32 = 2;
pub const HUMAN_GROUP_DIGITS: i32 = 4;
pub const HUMAN_SUPPRESS_POINT_ZERO: i32 = 8;
pub const HUMAN_AUTOSCALE: i32 = 16;
pub const HUMAN_BASE_1024: i32 = 32;
pub const HUMAN_SPACE_BEFORE_UNIT: i32 = 64;
pub const HUMAN_SI: i32 = 128;
pub const HUMAN_B: i32 = 256;

const HUMAN_READABLE_SUFFIX_LENGTH_MAX: usize = 3;

const POWER_LETTER: [char; 9] = [
    '\0', // not used
    'K',  // kibi ('k' for kilo is a special case)
    'M',  // mega or mebi
    'G',  // giga or gibi
    'T',  // tera or tebi
    'P',  // peta or pebi
    'E',  // exa or exbi
    'Z',  // zetta or 2**70
    'Y',  // yotta or 2**80
];

fn adjust_value(inexact_style: i32, value: f64) -> f64 {
    if inexact_style != HUMAN_ROUND_TO_NEAREST && value < u64::MAX as f64 {
        let u = value as u64;
        if inexact_style == HUMAN_CEILING && u as f64 != value {
            return (u + 1) as f64;
        }
        return u as f64;
    }
    value
}

fn group_number(
    number: &mut [u8],
    numberlen: usize,
    grouping: &str,
    thousands_sep: &str,
) -> &mut [u8] {
    let mut buf = [0u8; 2 * 20 + 1]; // Enough for u64::MAX squared
    buf[..numberlen].copy_from_slice(&number[..numberlen]);
    
    let mut d = numberlen;
    let mut grouplen = usize::MAX;
    let thousands_seplen = thousands_sep.len();
    let mut i = numberlen;
    let mut grouping_iter = grouping.chars();

    loop {
        let g = grouping_iter.next().unwrap_or('\0');
        if g != '\0' {
            grouplen = if g as u8 != u8::MAX {
                g as usize
            } else {
                i
            };
        }

        if i < grouplen {
            grouplen = i;
        }

        d -= grouplen;
        i -= grouplen;
        number[d..d + grouplen].copy_from_slice(&buf[i..i + grouplen]);
        if i == 0 {
            return &mut number[..d + grouplen];
        }

        d -= thousands_seplen;
        number[d..d + thousands_seplen].copy_from_slice(thousands_sep.as_bytes());
    }
}

pub fn human_readable(
    n: u64,
    buf: &mut [u8],
    opts: i32,
    from_block_size: u64,
    to_block_size: u64,
) -> &mut [u8] {
    let inexact_style = opts & (HUMAN_ROUND_TO_NEAREST | HUMAN_FLOOR | HUMAN_CEILING);
    let base = if opts & HUMAN_BASE_1024 != 0 { 1024 } else { 1000 };
    let mut amt;
    let mut tenths = 0;
    let mut rounding = 0;
    let mut exponent = -1;
    let exponent_max = POWER_LETTER.len() - 1;

    let decimal_point = ".";
    let decimal_pointlen = decimal_point.len();
    let thousands_sep = "";
    let grouping = "";

    let psuffix_pos = buf.len() - 1 - HUMAN_READABLE_SUFFIX_LENGTH_MAX;
    let (psuffix, buf_rest) = buf.split_at_mut(psuffix_pos);
    let mut p = psuffix.len();

    if to_block_size <= from_block_size {
        if from_block_size % to_block_size == 0 {
            let multiplier = from_block_size / to_block_size;
            amt = n.checked_mul(multiplier);
            if let Some(a) = amt {
                if a / multiplier == n {
                    amt = a;
                    tenths = 0;
                    rounding = 0;
                    // goto use_integer_arithmetic
                }
            }
        }
    } else if from_block_size != 0 && to_block_size % from_block_size == 0 {
        let divisor = to_block_size / from_block_size;
        let r10 = (n % divisor) * 10;
        let r2 = (r10 % divisor) * 2;
        amt = n / divisor;
        tenths = r10 / divisor;
        rounding = if r2 < divisor {
            if 0 < r2 { 1 } else { 0 }
        } else {
            2 + if divisor < r2 { 1 } else { 0 }
        };
        // goto use_integer_arithmetic
    } else {
        // Floating point fallback
        let dto_block_size = to_block_size as f64;
        let damt = n as f64 * (from_block_size as f64 / dto_block_size);
        
        if opts & HUMAN_AUTOSCALE == 0 {
            let s = format!("{:.0}", adjust_value(inexact_style, damt));
            let buflen = s.len();
            p -= buflen;
            buf_rest[p..p + buflen].copy_from_slice(s.as_bytes());
            // goto do_grouping
        } else {
            let mut e = 1.0;
            exponent = 0;
            
            while e * base as f64 <= damt && exponent < exponent_max {
                e *= base as f64;
                exponent += 1;
            }
            
            let damt = damt / e;
            let s = format!("{:.1}", adjust_value(inexact_style, damt));
            let buflen = s.len();
            let nonintegerlen = decimal_pointlen + 1;
            
            if 1 + nonintegerlen + if opts & HUMAN_BASE_1024 == 0 { 1 } else { 0 } < buflen
                || ((opts & HUMAN_SUPPRESS_POINT_ZERO) != 0 && s.ends_with('0'))
            {
                let s = format!("{:.0}", adjust_value(inexact_style, damt * 10.0) / 10.0);
                p -= s.len();
                buf_rest[p..p + s.len()].copy_from_slice(s.as_bytes());
                // goto do_grouping
            } else {
                p -= buflen;
                buf_rest[p..p + buflen].copy_from_slice(s.as_bytes());
                // goto do_grouping
            }
        }
    }

    // use_integer_arithmetic:
    if opts & HUMAN_AUTOSCALE != 0 {
        exponent = 0;
        if base <= amt {
            loop {
                let r10 = (amt % base) * 10 + tenths;
                let r2 = (r10 % base) * 2 + (rounding >> 1);
                amt /= base;
                tenths = r10 / base;
                rounding = if r2 < base {
                    if (r2 + rounding) != 0 { 1 } else { 0 }
                } else {
                    2 + if base < r2 + rounding { 1 } else { 0 }
                };
                exponent += 1;
                if !(base <= amt && exponent < exponent_max) {
                    break;
                }
            }
            
            if amt < 10 {
                if (inexact_style == HUMAN_ROUND_TO_NEAREST && 2 < rounding + (tenths & 1))
                    || (inexact_style == HUMAN_CEILING && 0 < rounding)
                {
                    tenths += 1;
                    rounding = 0;
                    if tenths == 10 {
                        amt += 1;
                        tenths = 0;
                    }
                }
                
                if amt < 10 && (tenths != 0 || opts & HUMAN_SUPPRESS_POINT_ZERO == 0) {
                    p -= 1;
                    buf_rest[p] = b'0' + tenths as u8;
                    p -= decimal_pointlen;
                    buf_rest[p..p + decimal_pointlen].copy_from_slice(decimal_point.as_bytes());
                    tenths = 0;
                    rounding = 0;
                }
            }
        }
    }

    if (inexact_style == HUMAN_ROUND_TO_NEAREST && 5 < tenths + if 0 < rounding + (amt & 1) { 1 } else { 0 })
        || (inexact_style == HUMAN_CEILING && 0 < tenths + rounding)
    {
        amt += 1;
        if (opts & HUMAN_AUTOSCALE) != 0 && amt == base as u64 && exponent < exponent_max {
            exponent += 1;
            if opts & HUMAN_SUPPRESS_POINT_ZERO == 0 {
                p -= 1;
                buf_rest[p] = b'0';
                p -= decimal_pointlen;
                buf_rest[p..p + decimal_pointlen].copy_from_slice(decimal_point.as_bytes());
            }
            amt = 1;
        }
    }

    let integerlim = p;
    let mut tmp = amt;
    loop {
        let digit = (tmp % 10) as u8;
        p -= 1;
        buf_rest[p] = b'0' + digit;
        tmp /= 10;
        if tmp == 0 {
            break;
        }
    }

    // do_grouping:
    if opts & HUMAN_GROUP_DIGITS != 0 {
        let grouped = group_number(
            &mut buf_rest[p..integerlim],
            integerlim - p,
            grouping,
            thousands_sep,
        );
        p = integerlim - grouped.len();
    }

    if opts & HUMAN_SI != 0 {
        if exponent < 0 {
            let mut power = 1;
            exponent = 0;
            while power < to_block_size {
                power *= base;
                exponent += 1;
                if exponent == exponent_max {
                    break;
                }
            }
        }

        let mut psuffix_pos = psuffix_pos;
        if (exponent != 0 || opts & HUMAN_B != 0) && opts & HUMAN_SPACE_BEFORE_UNIT != 0 {
            buf[psuffix_pos] = b' ';
            psuffix_pos += 1;
        }

        if exponent != 0 {
            let c = if opts & HUMAN_BASE_1024 == 0 && exponent == 1 {
                'k'
            } else {
                POWER_LETTER[exponent]
            };
            buf[psuffix_pos] = c as u8;
            psuffix_pos += 1;
        }

        if opts & HUMAN_B != 0 {
            if opts & HUMAN_BASE_1024 != 0 && exponent != 0 {
                buf[psuffix_pos] = b'i';
                psuffix_pos += 1;
            }
            buf[psuffix_pos] = b'B';
            psuffix_pos += 1;
        }
        buf[psuffix_pos] = 0;
    }

    &mut buf[p..]
}

const DEFAULT_BLOCK_SIZE: u64 = 1024;

fn default_block_size() -> u64 {
    match env::var("POSIXLY_CORRECT") {
        Ok(_) => 512,
        Err(_) => DEFAULT_BLOCK_SIZE,
    }
}

pub enum StrToLError {
    Ok,
    Invalid,
    OutOfRange,
}

pub fn human_options(spec: Option<&str>, opts: &mut i32, block_size: &mut u64) -> StrToLError {
    let e = humblock(spec, block_size, opts);
    if *block_size == 0 {
        *block_size = default_block_size();
        StrToLError::Invalid
    } else {
        e
    }
}

fn humblock(spec: Option<&str>, block_size: &mut u64, options: &mut i32) -> StrToLError {
    let mut opts = 0;
    let spec = match spec {
        Some(s) => s,
        None => {
            *block_size = default_block_size();
            *options = opts;
            return StrToLError::Ok;
        }
    };

    let mut spec = spec;
    if spec.starts_with('\'') {
        opts |= HUMAN_GROUP_DIGITS;
        spec = &spec[1..];
    }

    // Simplified version - actual implementation would need to handle more cases
    *block_size = 1;
    if spec == "human-readable" {
        opts |= HUMAN_AUTOSCALE | HUMAN_SI | HUMAN_BASE_1024;
    } else if spec == "si" {
        opts |= HUMAN_AUTOSCALE | HUMAN_SI;
    } else {
        // Parse numeric value
        match spec.parse::<u64>() {
            Ok(n) => {
                *block_size = n;
                // Check for SI suffixes
                if !spec.chars().next().unwrap().is_ascii_digit() {
                    opts |= HUMAN_SI;
                    if spec.ends_with('B') {
                        opts |= HUMAN_B;
                        if spec.len() > 1 && spec.chars().nth(spec.len() - 2).unwrap() == 'i' {
                            opts |= HUMAN_BASE_1024;
                        }
                    }
                }
            }
            Err(_) => {
                *options = 0;
                return StrToLError::Invalid;
            }
        }
    }

    *options = opts;
    StrToLError::Ok
}