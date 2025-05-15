use std::env;
use std::ffi::OsStr;
use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;
use std::ops::{Div, Mul, Rem};
use std::cmp::Ordering;
use std::mem;
use std::ptr;
use std::slice;
use std::borrow::Cow;

const LONGEST_HUMAN_READABLE: usize = {
    // Calculate based on the same formula as C version
    ((2 * mem::size_of::<u64>() * 8 * 146 / 485 + 1) * (4 + 1) - 4 + 1 + 3)
};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HumanOption {
    Ceiling = 0,
    RoundToNearest = 1,
    Floor = 2,
    GroupDigits = 4,
    SuppressPointZero = 8,
    Autoscale = 16,
    Base1024 = 32,
    SpaceBeforeUnit = 64,
    SI = 128,
    B = 256,
}

impl HumanOption {
    pub fn from_bits(bits: i32) -> Vec<Self> {
        let mut options = Vec::new();
        if bits & Self::Ceiling as i32 != 0 {
            options.push(Self::Ceiling);
        }
        if bits & Self::RoundToNearest as i32 != 0 {
            options.push(Self::RoundToNearest);
        }
        if bits & Self::Floor as i32 != 0 {
            options.push(Self::Floor);
        }
        if bits & Self::GroupDigits as i32 != 0 {
            options.push(Self::GroupDigits);
        }
        if bits & Self::SuppressPointZero as i32 != 0 {
            options.push(Self::SuppressPointZero);
        }
        if bits & Self::Autoscale as i32 != 0 {
            options.push(Self::Autoscale);
        }
        if bits & Self::Base1024 as i32 != 0 {
            options.push(Self::Base1024);
        }
        if bits & Self::SpaceBeforeUnit as i32 != 0 {
            options.push(Self::SpaceBeforeUnit);
        }
        if bits & Self::SI as i32 != 0 {
            options.push(Self::SI);
        }
        if bits & Self::B as i32 != 0 {
            options.push(Self::B);
        }
        options
    }

    pub fn to_bits(options: &[Self]) -> i32 {
        let mut bits = 0;
        for opt in options {
            bits |= *opt as i32;
        }
        bits
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HumanError {
    Invalid,
    Overflow,
    Underflow,
    Incomplete,
}

impl fmt::Display for HumanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Invalid => write!(f, "invalid number"),
            Self::Overflow => write!(f, "number too large"),
            Self::Underflow => write!(f, "number too small"),
            Self::Incomplete => write!(f, "incomplete number"),
        }
    }
}

impl std::error::Error for HumanError {}

fn adjust_value(inexact_style: HumanOption, value: f64) -> f64 {
    if inexact_style != HumanOption::RoundToNearest && value < u64::MAX as f64 {
        let u = value as u64;
        if inexact_style == HumanOption::Ceiling && u as f64 != value {
            (u + 1) as f64
        } else {
            u as f64
        }
    } else {
        value
    }
}

fn group_number(
    number: &str,
    grouping: &str,
    thousands_sep: &str,
) -> String {
    let mut result = String::new();
    let mut chars = number.chars().rev().peekable();
    let mut group_size = 0;
    let mut group_spec = grouping.chars();

    while let Some(c) = group_spec.next() {
        if c == '\0' {
            break;
        }
        group_size = c as usize;
        
        for _ in 0..group_size {
            if let Some(digit) = chars.next() {
                result.push(digit);
            } else {
                break;
            }
        }
        
        if chars.peek().is_some() {
            result.extend(thousands_sep.chars().rev());
        }
    }

    // Handle remaining digits with last group size
    while chars.peek().is_some() {
        for _ in 0..group_size {
            if let Some(digit) = chars.next() {
                result.push(digit);
            } else {
                break;
            }
        }
        
        if chars.peek().is_some() {
            result.extend(thousands_sep.chars().rev());
        }
    }

    result.chars().rev().collect()
}

pub fn human_readable(
    n: u64,
    opts: i32,
    from_block_size: u64,
    to_block_size: u64,
) -> String {
    let options = HumanOption::from_bits(opts);
    let inexact_style = if options.contains(&HumanOption::RoundToNearest) {
        HumanOption::RoundToNearest
    } else if options.contains(&HumanOption::Floor) {
        HumanOption::Floor
    } else {
        HumanOption::Ceiling
    };
    
    let base = if options.contains(&HumanOption::Base1024) {
        1024
    } else {
        1000
    };

    let mut amt;
    let mut tenths = 0;
    let mut rounding = 0;
    let mut exponent = -1;
    let exponent_max = POWER_LETTER.len() - 1;

    let decimal_point = ".";
    let grouping = "";
    let thousands_sep = "";

    // Adjust AMT out of FROM_BLOCK_SIZE units and into TO_BLOCK_SIZE units
    if to_block_size <= from_block_size {
        if from_block_size % to_block_size == 0 {
            let multiplier = from_block_size / to_block_size;
            amt = n.checked_mul(multiplier);
            if let Some(a) = amt {
                if a / multiplier == n {
                    amt = Some(a);
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
        amt = Some(n / divisor);
        tenths = r10 / divisor;
        rounding = match (r2 < divisor, divisor < r2) {
            (true, _) => if r2 > 0 { 1 } else { 0 },
            (false, true) => 3,
            (false, false) => 2,
        };
        // goto use_integer_arithmetic
    } else {
        amt = None;
    }

    let mut result = String::with_capacity(LONGEST_HUMAN_READABLE);
    let mut psuffix_pos = LONGEST_HUMAN_READABLE - 1 - HUMAN_READABLE_SUFFIX_LENGTH_MAX;

    if let Some(mut a) = amt {
        // Integer arithmetic path
        if options.contains(&HumanOption::Autoscale) {
            exponent = 0;

            if base <= a {
                loop {
                    let r10 = (a % base) * 10 + tenths;
                    let r2 = (r10 % base) * 2 + (rounding >> 1);
                    a /= base;
                    tenths = r10 / base;
                    rounding = match (r2 < base, base < r2 + rounding) {
                        (true, _) => if r2 + rounding > 0 { 1 } else { 0 },
                        (false, true) => 3,
                        (false, false) => 2,
                    };
                    exponent += 1;
                    
                    if !(base <= a && exponent < exponent_max) {
                        break;
                    }
                }

                if a < 10 {
                    let should_round = match inexact_style {
                        HumanOption::RoundToNearest => 2 < rounding + (tenths & 1),
                        HumanOption::Ceiling => 0 < rounding,
                        _ => false,
                    };

                    if should_round {
                        tenths += 1;
                        rounding = 0;

                        if tenths == 10 {
                            a += 1;
                            tenths = 0;
                        }
                    }

                    if a < 10 && (tenths > 0 || !options.contains(&HumanOption::SuppressPointZero)) {
                        result.push_str(&format!("{}{}", decimal_point, tenths));
                        tenths = 0;
                        rounding = 0;
                    }
                }
            }
        }

        let should_increment = match inexact_style {
            HumanOption::RoundToNearest => 5 < tenths + if rounding > 0 || (a & 1) != 0 { 1 } else { 0 },
            HumanOption::Ceiling => 0 < tenths + rounding,
            _ => false,
        };

        if should_increment {
            a += 1;

            if options.contains(&HumanOption::Autoscale) && a == base && exponent < exponent_max {
                exponent += 1;
                if !options.contains(&HumanOption::SuppressPointZero) {
                    result.push_str(&format!("{}{}", decimal_point, 0));
                }
                a = 1;
            }
        }

        // Convert amount to string digits
        let mut digits = Vec::new();
        let mut amount = a;
        while amount > 0 {
            digits.push((amount % 10) as u8 + b'0');
            amount /= 10;
        }
        digits.reverse();
        
        let number_str = unsafe { String::from_utf8_unchecked(digits) };
        
        if options.contains(&HumanOption::GroupDigits) {
            result = group_number(&number_str, grouping, thousands_sep);
        } else {
            result = number_str;
        }
    } else {
        // Floating point path
        let dto_block_size = to_block_size as f64;
        let damt = n as f64 * (from_block_size as f64 / dto_block_size);
        
        if !options.contains(&HumanOption::Autoscale) {
            result = format!("{:.0}", adjust_value(inexact_style, damt));
        } else {
            let mut e = 1.0;
            exponent = 0;

            while e * (base as f64) <= damt && exponent < exponent_max {
                e *= base as f64;
                exponent += 1;
            }

            let scaled_damt = damt / e;
            result = format!("{:.1}", adjust_value(inexact_style, scaled_damt));

            let noninteger_len = decimal_point.len() + 1;
            if 1 + noninteger_len + if !options.contains(&HumanOption::Base1024) { 1 } else { 0 } < result.len() ||
                (options.contains(&HumanOption::SuppressPointZero) && result.ends_with('0')) 
            {
                result = format!("{:.0}", adjust_value(inexact_style, scaled_damt * 10.0) / 10.0);
            }
        }
    }

    // Handle suffix
    if options.contains(&HumanOption::SI) {
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

        if (exponent > 0 || options.contains(&HumanOption::B)) && 
           options.contains(&HumanOption::SpaceBeforeUnit) 
        {
            result.push(' ');
        }

        if exponent > 0 {
            let c = if !options.contains(&HumanOption::Base1024) && exponent == 1 {
                'k'
            } else {
                POWER_LETTER[exponent]
            };
            result.push(c);
        }

        if options.contains(&HumanOption::B) {
            if options.contains(&HumanOption::Base1024) && exponent > 0 {
                result.push('i');
            }
            result.push('B');
        }
    }

    result
}

const DEFAULT_BLOCK_SIZE: u64 = 1024;

fn default_block_size() -> u64 {
    if env::var_os("POSIXLY_CORRECT").is_some() {
        512
    } else {
        DEFAULT_BLOCK_SIZE
    }
}

pub fn human_options(spec: Option<&str>) -> Result<(i32, u64), HumanError> {
    let block_size_args = ["human-readable", "si"];
    let block_size_opts = [
        HumanOption::to_bits(&[HumanOption::Autoscale, HumanOption::SI, HumanOption::Base1024]),
        HumanOption::to_bits(&[HumanOption::Autoscale, HumanOption::SI]),
    ];

    let (spec, mut opts) = if let Some(s) = spec {
        if s.starts_with('\'') {
            (Some(&s[1..]), HumanOption::GroupDigits as i32)
        } else {
            (Some(s), 0)
        }
    } else {
        let env_var = env::var_os("BLOCK_SIZE")
            .or_else(|| env::var_os("BLOCKSIZE"));
        
        if let Some(s) = env_var {
            let s = s.to_str().ok_or(HumanError::Invalid)?;
            if s.starts_with('\'') {
                (Some(&s[1..]), HumanOption::GroupDigits as i32)
            } else {
                (Some(s), 0)
            }
        } else {
            (None, 0)
        }
    };

    let block_size = if let Some(spec) = spec {
        if let Some(i) = block_size_args.iter().position(|&x| x == spec) {
            opts |= block_size_opts[i];
            1
        } else {
            let mut s = spec;
            let mut has_suffix = false;
            let mut base1024 = false;
            let mut has_b = false;
            
            // Parse suffix characters
            while !s.is_empty() && !s.chars().next().unwrap().is_ascii_digit() {
                let c = s.chars().next().unwrap();
                if c == 'B' {
                    has_b = true;
                } else if c == 'i' {
                    base1024 = true;
                }
                s = &s[1..];
                has_suffix = true;
            }
            
            if has_suffix {
                opts |= HumanOption::SI as i32;
                if has_b {
                    opts |= HumanOption::B as i32;
                }
                if base1024 || !has_b {
                    opts |= HumanOption::Base1024 as i32;
                }
            }
            
            s.parse::<u64>().map_err(|_| HumanError::Invalid)?
        }
    } else {
        default_block_size()
    };

    if block_size == 0 {
        Err(HumanError::Invalid)
    } else {
        Ok((opts, block_size))
    }
}