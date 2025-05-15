use std::fmt;
use std::io::{self, Write};
use std::mem;
use std::os::raw::{c_char, c_int, c_long, c_void};
use std::ptr;

#[derive(Debug, Clone, Copy, PartialEq)]
enum FormatState {
    Default,
    Flags,
    Min,
    Dot,
    Max,
    Mod,
    Conv,
    Done,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum FormatConversion {
    Short,
    Long,
    LongDouble,
    LongLong,
}

bitflags! {
    struct FormatFlags: u32 {
        const MINUS     = 1 << 0;
        const PLUS      = 1 << 1;
        const SPACE     = 1 << 2;
        const NUM       = 1 << 3;
        const ZERO      = 1 << 4;
        const UP        = 1 << 5;
        const UNSIGNED  = 1 << 6;
    }
}

const NUL: char = '\0';

struct FormatContext<'a> {
    buffer: &'a mut [u8],
    currlen: usize,
    maxlen: usize,
}

impl<'a> FormatContext<'a> {
    fn new(buffer: &'a mut [u8], maxlen: usize) -> Self {
        FormatContext {
            buffer,
            currlen: 0,
            maxlen,
        }
    }

    fn outch(&mut self, c: char) {
        if self.currlen < self.maxlen {
            if !self.buffer.is_empty() {
                self.buffer[self.currlen] = c as u8;
            }
            self.currlen += 1;
        }
    }

    fn outstr(&mut self, s: &str, flags: FormatFlags, min: usize, max: usize) {
        let mut padlen = min.saturating_sub(s.len());
        if flags.contains(FormatFlags::MINUS) {
            padlen = 0;
        }

        if !flags.contains(FormatFlags::MINUS) {
            for _ in 0..padlen {
                self.outch(' ');
            }
        }

        for c in s.chars().take(max) {
            self.outch(c);
        }

        if flags.contains(FormatFlags::MINUS) {
            for _ in 0..padlen {
                self.outch(' ');
            }
        }
    }
}

fn fmtint(
    ctx: &mut FormatContext,
    value: i64,
    base: u32,
    min: usize,
    max: usize,
    flags: FormatFlags,
) {
    let mut signvalue = None;
    let mut uvalue = if !flags.contains(FormatFlags::UNSIGNED) {
        if value < 0 {
            signvalue = Some('-');
            (-value) as u64
        } else if flags.contains(FormatFlags::PLUS) {
            signvalue = Some('+');
            value as u64
        } else if flags.contains(FormatFlags::SPACE) {
            signvalue = Some(' ');
            value as u64
        } else {
            value as u64
        }
    } else {
        value as u64
    };

    let caps = flags.contains(FormatFlags::UP);
    let digits = if caps {
        "0123456789ABCDEF"
    } else {
        "0123456789abcdef"
    };

    let mut convert = [0u8; 20];
    let mut place = 0;

    if uvalue == 0 {
        convert[place] = b'0';
        place += 1;
    } else {
        while uvalue > 0 && place < 20 {
            convert[place] = digits.as_bytes()[(uvalue % base as u64) as usize];
            uvalue /= base as u64;
            place += 1;
        }
    }

    let zpadlen = max.saturating_sub(place);
    let spadlen = min.saturating_sub(max.max(place) + if signvalue.is_some() { 1 } else { 0 });

    let (zpadlen, spadlen) = if flags.contains(FormatFlags::ZERO) {
        (zpadlen.max(spadlen), 0)
    } else {
        (zpadlen, spadlen)
    };

    let spadlen = if flags.contains(FormatFlags::MINUS) {
        -(spadlen as isize)
    } else {
        spadlen as isize
    };

    if spadlen > 0 {
        for _ in 0..spadlen {
            ctx.outch(' ');
        }
    }

    if let Some(s) = signvalue {
        ctx.outch(s);
    }

    for _ in 0..zpadlen {
        ctx.outch('0');
    }

    for i in (0..place).rev() {
        ctx.outch(convert[i] as char);
    }

    if spadlen < 0 {
        for _ in 0..-spadlen {
            ctx.outch(' ');
        }
    }
}

fn fmtfp(
    ctx: &mut FormatContext,
    fvalue: f64,
    min: usize,
    max: usize,
    flags: FormatFlags,
) {
    let max = if max < 0 { 6 } else { max.min(9) };
    let ufvalue = fvalue.abs();
    let signvalue = if fvalue < 0.0 {
        Some('-')
    } else if flags.contains(FormatFlags::PLUS) {
        Some('+')
    } else if flags.contains(FormatFlags::SPACE) {
        Some(' ')
    } else {
        None
    };

    let intpart = ufvalue.trunc() as i64;
    let fracpart = ((ufvalue.fract() * 10f64.powi(max as i32)).round() as i64) % 10i64.pow(max as u32);

    let mut iconvert = [0u8; 20];
    let mut iplace = 0;

    let mut tmp = intpart;
    if tmp == 0 {
        iconvert[iplace] = b'0';
        iplace += 1;
    } else {
        while tmp > 0 && iplace < 20 {
            iconvert[iplace] = b'0' + (tmp % 10) as u8;
            tmp /= 10;
            iplace += 1;
        }
    }

    let mut fconvert = [0u8; 20];
    let mut fplace = 0;

    let mut tmp = fracpart;
    if tmp == 0 {
        fconvert[fplace] = b'0';
        fplace += 1;
    } else {
        while tmp > 0 && fplace < 20 {
            fconvert[fplace] = b'0' + (tmp % 10) as u8;
            tmp /= 10;
            fplace += 1;
        }
    }

    let padlen = min
        .saturating_sub(iplace + max + 1 + if signvalue.is_some() { 1 } else { 0 });
    let zpadlen = max.saturating_sub(fplace);

    let padlen = if flags.contains(FormatFlags::MINUS) {
        -(padlen as isize)
    } else {
        padlen as isize
    };

    if flags.contains(FormatFlags::ZERO) && padlen > 0 {
        if let Some(s) = signvalue {
            ctx.outch(s);
        }
        for _ in 0..padlen {
            ctx.outch('0');
        }
    } else {
        if padlen > 0 {
            for _ in 0..padlen {
                ctx.outch(' ');
            }
        }
        if let Some(s) = signvalue {
            ctx.outch(s);
        }
    }

    for i in (0..iplace).rev() {
        ctx.outch(iconvert[i] as char);
    }

    if max > 0 {
        ctx.outch('.');
        for i in (0..fplace).rev() {
            ctx.outch(fconvert[i] as char);
        }
        for _ in 0..zpadlen {
            ctx.outch('0');
        }
    }

    if padlen < 0 {
        for _ in 0..-padlen {
            ctx.outch(' ');
        }
    }
}

fn dopr(
    buffer: &mut [u8],
    maxlen: usize,
    retlen: &mut usize,
    format: &str,
    args: &mut fmt::Arguments,
) {
    let mut ctx = FormatContext::new(buffer, maxlen);
    let mut state = FormatState::Default;
    let mut flags = FormatFlags::empty();
    let mut cflags = None;
    let mut min = 0;
    let mut max = -1;

    for ch in format.chars() {
        if state == FormatState::Done {
            break;
        }

        match state {
            FormatState::Default => {
                if ch == '%' {
                    state = FormatState::Flags;
                } else {
                    ctx.outch(ch);
                }
            }
            FormatState::Flags => {
                match ch {
                    '-' => flags.insert(FormatFlags::MINUS),
                    '+' => flags.insert(FormatFlags::PLUS),
                    ' ' => flags.insert(FormatFlags::SPACE),
                    '#' => flags.insert(FormatFlags::NUM),
                    '0' => flags.insert(FormatFlags::ZERO),
                    _ => {
                        state = FormatState::Min;
                        continue;
                    }
                }
            }
            FormatState::Min => {
                if ch.is_digit(10) {
                    min = min * 10 + ch.to_digit(10).unwrap() as usize;
                } else if ch == '*' {
                    // Handle * width specifier
                    state = FormatState::Dot;
                } else {
                    state = FormatState::Dot;
                }
            }
            FormatState::Dot => {
                if ch == '.' {
                    state = FormatState::Max;
                } else {
                    state = FormatState::Mod;
                }
            }
            FormatState::Max => {
                if ch.is_digit(10) {
                    if max < 0 {
                        max = 0;
                    }
                    max = max * 10 + ch.to_digit(10).unwrap() as i32;
                } else if ch == '*' {
                    // Handle * precision specifier
                    state = FormatState::Mod;
                } else {
                    state = FormatState::Mod;
                }
            }
            FormatState::Mod => {
                cflags = match ch {
                    'h' => Some(FormatConversion::Short),
                    'l' => Some(FormatConversion::Long),
                    'L' => Some(FormatConversion::LongDouble),
                    'q' => Some(FormatConversion::LongLong),
                    _ => None,
                };
                state = FormatState::Conv;
            }
            FormatState::Conv => {
                match ch {
                    'd' | 'i' => {
                        let value = match cflags {
                            Some(FormatConversion::Short) => args.next().unwrap().as_i64().unwrap() as i16 as i64,
                            Some(FormatConversion::Long) => args.next().unwrap().as_i64().unwrap(),
                            Some(FormatConversion::LongLong) => args.next().unwrap().as_i64().unwrap(),
                            _ => args.next().unwrap().as_i64().unwrap(),
                        };
                        fmtint(&mut ctx, value, 10, min, max.max(0) as usize, flags);
                    }
                    'x' | 'X' | 'o' | 'u' => {
                        let mut local_flags = flags;
                        if ch == 'X' {
                            local_flags.insert(FormatFlags::UP);
                        }
                        local_flags.insert(FormatFlags::UNSIGNED);
                        
                        let value = match cflags {
                            Some(FormatConversion::Short) => args.next().unwrap().as_u64().unwrap() as u16 as u64,
                            Some(FormatConversion::Long) => args.next().unwrap().as_u64().unwrap(),
                            Some(FormatConversion::LongLong) => args.next().unwrap().as_u64().unwrap(),
                            _ => args.next().unwrap().as_u64().unwrap(),
                        };
                        
                        let base = match ch {
                            'o' => 8,
                            'u' => 10,
                            _ => 16,
                        };
                        
                        fmtint(&mut ctx, value as i64, base, min, max.max(0) as usize, local_flags);
                    }
                    'f' => {
                        let fvalue = match cflags {
                            Some(FormatConversion::LongDouble) => args.next().unwrap().as_f64().unwrap(),
                            _ => args.next().unwrap().as_f64().unwrap(),
                        };
                        fmtfp(&mut ctx, fvalue, min, max.max(0) as usize, flags);
                    }
                    'c' => {
                        let c = args.next().unwrap().as_char().unwrap();
                        ctx.outch(c);
                    }
                    's' => {
                        let s = args.next().unwrap().as_str().unwrap();
                        ctx.outstr(s, flags, min, max.max(0) as usize);
                    }
                    'p' => {
                        let p = args.next().unwrap().as_pointer().unwrap();
                        fmtint(&mut ctx, p as i64, 16, min, max.max(0) as usize, flags);
                    }
                    '%' => ctx.outch('%'),
                    _ => {}
                }
                state = FormatState::Default;
                flags = FormatFlags::empty();
                cflags = None;
                min = 0;
                max = -1;
            }
            _ => {}
        }
    }

    *retlen = ctx.currlen;
    if !buffer.is_empty() && ctx.currlen < buffer.len() {
        buffer[ctx.currlen] = b'\0';
    }
}

pub fn pth_vsnprintf(
    str: &mut [u8],
    count: usize,
    fmt: &str,
    args: &mut fmt::Arguments,
) -> usize {
    let mut retlen = 0;
    dopr(str, count, &mut retlen, fmt, args);
    retlen
}

pub fn pth_snprintf(
    str: &mut [u8],
    count: usize,
    fmt: &str,
    args: fmt::Arguments,
) -> usize {
    let mut retlen = 0;
    dopr(str, count, &mut retlen, fmt, &mut args);
    retlen
}

pub fn pth_vasprintf(
    fmt: &str,
    args: &mut fmt::Arguments,
) -> Option<Vec<u8>> {
    let mut size = 0;
    dopr(&mut [], usize::MAX, &mut size, fmt, args);
    
    let mut buffer = vec![0u8; size + 1];
    let mut retlen = 0;
    dopr(&mut buffer, size + 1, &mut retlen, fmt, args);
    
    Some(buffer)
}

pub fn pth_asprintf(
    fmt: &str,
    args: fmt::Arguments,
) -> Option<Vec<u8>> {
    pth_vasprintf(fmt, &mut args)
}