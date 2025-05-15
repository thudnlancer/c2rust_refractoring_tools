use num_traits::ToPrimitive;
use std::env;
use std::ffi::{CStr, CString};
use std::ptr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrToLError {
    Ok,
    Overflow,
    InvalidSuffixChar,
    InvalidSuffixCharWithOverflow,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum HumanOption {
    RoundToNearest = 1,
    Floor = 2,
    Ceiling = 0,
    GroupDigits = 4,
    SuppressPointZero = 8,
    Autoscale = 16,
    Base1024 = 32,
    SpaceBeforeUnit = 64,
    SI = 128,
    B = 256,
}

static POWER_LETTER: [char; 9] = ['\0', 'K', 'M', 'G', 'T', 'P', 'E', 'Z', 'Y'];

fn adjust_value(inexact_style: HumanOption, value: f64) -> f64 {
    if inexact_style != HumanOption::RoundToNearest && value < u64::MAX as f64 {
        let u = value as u64;
        value = if inexact_style == HumanOption::Ceiling && (u as f64) != value {
            (u + 1) as f64
        } else {
            u as f64
        };
    }
    value
}

fn group_number(
    number: &mut [u8],
    grouping: &[u8],
    thousands_sep: &[u8],
) -> Vec<u8> {
    let mut result = Vec::new();
    let mut group_len = usize::MAX;
    let mut i = number.len();
    let mut buf = number.to_vec();

    let mut grouping_iter = grouping.iter();
    let mut current_group = grouping_iter.next();

    while i > 0 {
        if let Some(&g) = current_group {
            if g != 0 {
                group_len = if g < 127 { g as usize } else { i };
                current_group = grouping_iter.next();
            }
        }

        let actual_group_len = group_len.min(i);
        let start = i - actual_group_len;
        result.extend(&buf[start..i]);
        i = start;

        if i > 0 {
            result.extend(thousands_sep);
        }
    }

    result.reverse();
    result
}

pub fn human_readable(
    n: u64,
    opts: u32,
    from_block_size: u64,
    to_block_size: u64,
) -> String {
    let inexact_style = opts & (HumanOption::RoundToNearest as u32 
        | HumanOption::Floor as u32 
        | HumanOption::Ceiling as u32);
    let inexact_style = match inexact_style {
        0 => HumanOption::Ceiling,
        1 => HumanOption::RoundToNearest,
        2 => HumanOption::Floor,
        _ => HumanOption::RoundToNearest,
    };

    let base = if opts & HumanOption::Base1024 as u32 != 0 {
        1024
    } else {
        1000
    };

    let mut amt = 0;
    let mut tenths = 0;
    let mut exponent = -1;
    let exponent_max = POWER_LETTER.len() as i32 - 1;

    let mut result = String::new();

    let decimal_point = ".";
    let grouping = "";
    let thousands_sep = "";

    if to_block_size <= from_block_size {
        if from_block_size % to_block_size == 0 {
            let multiplier = from_block_size / to_block_size;
            amt = n.wrapping_mul(multiplier);
            if amt.wrapping_div(multiplier) == n {
                tenths = 0;
            } else {
                let d_to_block_size = to_block_size as f64;
                let d_amt = n as f64 * (from_block_size as f64 / d_to_block_size);
                return format_human(d_amt, inexact_style, base, exponent_max, opts);
            }
        } else {
            let d_to_block_size = to_block_size as f64;
            let d_amt = n as f64 * (from_block_size as f64 / d_to_block_size);
            return format_human(d_amt, inexact_style, base, exponent_max, opts);
        }
    } else if from_block_size != 0 && to_block_size % from_block_size == 0 {
        let divisor = to_block_size / from_block_size;
        let r10 = n % divisor * 10;
        let r2 = r10 % divisor * 2;
        amt = n / divisor;
        tenths = (r10 / divisor) as i32;
        let rounding = if r2 < divisor {
            if 0 < r2 { 1 } else { 0 }
        } else {
            2 + if divisor < r2 { 1 } else { 0 }
        };
    } else {
        let d_to_block_size = to_block_size as f64;
        let d_amt = n as f64 * (from_block_size as f64 / d_to_block_size);
        return format_human(d_amt, inexact_style, base, exponent_max, opts);
    }

    if opts & HumanOption::Autoscale as u32 != 0 {
        exponent = 0;
        if base as u64 <= amt {
            loop {
                let r10 = (amt % base as u64 * 10 + tenths as u64) as u32;
                let r2 = (r10 % base * 2 + (rounding >> 1) as u32) as u32;
                amt = amt / base as u64;
                tenths = (r10 / base) as i32;
                let rounding = if r2 < base {
                    (r2 + rounding as u32 != 0) as i32
                } else {
                    2 + (base < r2 + rounding as u32) as i32
                };
                exponent += 1;
                if !(base as u64 <= amt && exponent < exponent_max) {
                    break;
                }
            }

            if amt < 10 {
                let should_round = if inexact_style == HumanOption::RoundToNearest {
                    (2 < rounding + (tenths & 1)) as i32
                } else {
                    (inexact_style == HumanOption::Ceiling && 0 < rounding) as i32
                };
                if should_round != 0 {
                    tenths += 1;
                    if tenths == 10 {
                        amt += 1;
                        tenths = 0;
                    }
                }

                if amt < 10 && (tenths != 0 || opts & HumanOption::SuppressPointZero as u32 == 0) {
                    result.push_str(&tenths.to_string());
                    result.push_str(decimal_point);
                }
            }
        }
    }

    let should_round = if inexact_style == HumanOption::RoundToNearest {
        (5 < tenths + if 0 < (rounding as u64 + (amt & 1)) { 1 } else { 0 }) as i32
    } else {
        (inexact_style == HumanOption::Ceiling && 0 < tenths + rounding) as i32
    };

    if should_round != 0 {
        amt += 1;
        if opts & HumanOption::Autoscale as u32 != 0 
            && amt == base as u64 
            && exponent < exponent_max 
        {
            exponent += 1;
            if opts & HumanOption::SuppressPointZero as u32 == 0 {
                result.push('0');
                result.push_str(decimal_point);
            }
            amt = 1;
        }
    }

    let mut integer_part = amt.to_string();
    if opts & HumanOption::GroupDigits as u32 != 0 {
        integer_part = String::from_utf8(
            group_number(
                integer_part.as_bytes_mut(),
                grouping.as_bytes(),
                thousands_sep.as_bytes(),
            )
        ).unwrap();
    }

    result.insert_str(0, &integer_part);

    if opts & HumanOption::SI as u32 != 0 {
        if exponent < 0 {
            exponent = 0;
            let mut power = 1u64;
            while power < to_block_size {
                exponent += 1;
                if exponent == exponent_max {
                    break;
                }
                power = power.wrapping_mul(base as u64);
            }
        }

        if (exponent != 0 || opts & HumanOption::B as u32 != 0) 
            && opts & HumanOption::SpaceBeforeUnit as u32 != 0 
        {
            result.push(' ');
        }

        if exponent != 0 {
            let c = if opts & HumanOption::Base1024 as u32 == 0 && exponent == 1 {
                'k'
            } else {
                POWER_LETTER[exponent as usize]
            };
            result.push(c);
        }

        if opts & HumanOption::B as u32 != 0 {
            if opts & HumanOption::Base1024 as u32 != 0 && exponent != 0 {
                result.push('i');
            }
            result.push('B');
        }
    }

    result
}

fn format_human(
    value: f64,
    inexact_style: HumanOption,
    base: u32,
    exponent_max: i32,
    opts: u32,
) -> String {
    let decimal_point = ".";
    let mut exponent = 0;
    let mut adjusted_value = value;

    if opts & HumanOption::Autoscale as u32 == 0 {
        format!("{:.0}", adjust_value(inexact_style, adjusted_value))
    } else {
        let mut e = 1.0;
        loop {
            e *= base as f64;
            exponent += 1;
            if !(e * base as f64 <= adjusted_value && exponent < exponent_max) {
                break;
            }
        }
        adjusted_value /= e;
        let formatted = format!("{:.1}", adjust_value(inexact_style, adjusted_value));
        let non_integer_len = decimal_point.len() + 1;
        if 1 + non_integer_len + if opts & HumanOption::Base1024 as u32 == 0 { 1 } else { 0 } < formatted.len() 
            || opts & HumanOption::SuppressPointZero as u32 != 0 
                && formatted.ends_with('0')
        {
            format!(
                "{:.0}",
                adjust_value(inexact_style, adjusted_value * 10.0) / 10.0
            )
        } else {
            formatted
        }
    }
}

static BLOCK_SIZE_ARGS: [&str; 3] = ["human-readable", "si", ""];
static BLOCK_SIZE_OPTS: [u32; 2] = [
    HumanOption::Autoscale as u32 | HumanOption::SI as u32 | HumanOption::Base1024 as u32,
    HumanOption::Autoscale as u32 | HumanOption::SI as u32,
];

fn default_block_size() -> u64 {
    if env::var_os("POSIXLY_CORRECT").is_some() {
        512
    } else {
        1024
    }
}

pub fn human_options(spec: Option<&str>) -> Result<(u32, u64), StrToLError> {
    let spec = spec
        .or_else(|| env::var("BLOCK_SIZE").ok().as_deref())
        .or_else(|| env::var("BLOCKSIZE").ok().as_deref());

    if spec.is_none() {
        return Ok((0, default_block_size()));
    }

    let spec = spec.unwrap();
    let mut opts = 0;
    let mut block_size = 1;

    if spec.starts_with('\'') {
        opts |= HumanOption::GroupDigits as u32;
        let spec = &spec[1..];
    }

    if let Some(i) = BLOCK_SIZE_ARGS.iter().position(|&arg| arg == spec) {
        opts |= BLOCK_SIZE_OPTS[i];
        Ok((opts, block_size))
    } else {
        Err(StrToLError::Invalid)
    }
}