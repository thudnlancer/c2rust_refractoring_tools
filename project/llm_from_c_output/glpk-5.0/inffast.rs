// inffast.rs -- fast decoding
// Copyright (C) 1995-2008, 2010 Mark Adler
// Translated to Rust with safety and efficiency in mind

use crate::zlib::{InflateState, ZStream, ZLibResult, ZLibError};
use crate::inftrees::Code;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
enum InflateMode {
    Len,
    Type,
    Bad,
}

pub fn inflate_fast(strm: &mut ZStream, start: u32) -> ZLibResult<()> {
    let state = &mut strm.state;
    let mut in_buf = strm.next_in;
    let last = in_buf.checked_add(strm.avail_in - 5).ok_or(ZLibError::BufError)?;
    let mut out_buf = strm.next_out;
    let beg = out_buf.checked_sub(start - strm.avail_out).ok_or(ZLibError::BufError)?;
    let end = out_buf.checked_add(strm.avail_out - 257).ok_or(ZLibError::BufError)?;

    let wsize = state.wsize;
    let whave = state.whave;
    let wnext = state.wnext;
    let window = &state.window;
    let mut hold = state.hold;
    let mut bits = state.bits;
    let lcode = &state.lencode;
    let dcode = &state.distcode;
    let lmask = (1 << state.lenbits) - 1;
    let dmask = (1 << state.distbits) - 1;

    let mut mode = InflateMode::Len;

    while in_buf < last && out_buf < end {
        if bits < 15 {
            hold += (u32::from(unsafe { *in_buf }) << bits;
            in_buf = in_buf.checked_add(1).ok_or(ZLibError::BufError)?;
            bits += 8;
            hold += (u32::from(unsafe { *in_buf }) << bits;
            in_buf = in_buf.checked_add(1).ok_or(ZLibError::BufError)?;
            bits += 8;
        }

        let mut here = lcode[(hold & lmask) as usize];
        loop {
            let op = here.bits;
            hold >>= op;
            bits -= op;
            let op = here.op;

            if op == 0 {
                // literal
                unsafe { *out_buf = here.val as u8; }
                out_buf = out_buf.checked_add(1).ok_or(ZLibError::BufError)?;
                break;
            } else if op & 16 != 0 {
                // length base
                let mut len = here.val as u32;
                let op = op & 15;
                if op != 0 {
                    if bits < op {
                        hold += u32::from(unsafe { *in_buf }) << bits;
                        in_buf = in_buf.checked_add(1).ok_or(ZLibError::BufError)?;
                        bits += 8;
                    }
                    len += hold & ((1 << op) - 1);
                    hold >>= op;
                    bits -= op;
                }

                if bits < 15 {
                    hold += u32::from(unsafe { *in_buf }) << bits;
                    in_buf = in_buf.checked_add(1).ok_or(ZLibError::BufError)?;
                    bits += 8;
                    hold += u32::from(unsafe { *in_buf }) << bits;
                    in_buf = in_buf.checked_add(1).ok_or(ZLibError::BufError)?;
                    bits += 8;
                }

                here = dcode[(hold & dmask) as usize];
                loop {
                    let op = here.bits;
                    hold >>= op;
                    bits -= op;
                    let op = here.op;

                    if op & 16 != 0 {
                        // distance base
                        let mut dist = here.val as u32;
                        let op = op & 15;
                        if bits < op {
                            hold += u32::from(unsafe { *in_buf }) << bits;
                            in_buf = in_buf.checked_add(1).ok_or(ZLibError::BufError)?;
                            bits += 8;
                            if bits < op {
                                hold += u32::from(unsafe { *in_buf }) << bits;
                                in_buf = in_buf.checked_add(1).ok_or(ZLibError::BufError)?;
                                bits += 8;
                            }
                        }
                        dist += hold & ((1 << op) - 1);
                        hold >>= op;
                        bits -= op;

                        let op = (out_buf as usize - beg as usize) as u32;
                        if dist > op {
                            let op = dist - op;
                            if op > whave {
                                if state.sane {
                                    return Err(ZLibError::DataError("invalid distance too far back"));
                                }
                                // Handle invalid distance cases
                            }
                            let mut from = window.as_ptr();
                            if wnext == 0 {
                                from = from.wrapping_add(wsize - op as usize);
                                if op < len {
                                    len -= op;
                                    for _ in 0..op {
                                        unsafe { *out_buf = *from; }
                                        out_buf = out_buf.checked_add(1).ok_or(ZLibError::BufError)?;
                                        from = from.checked_add(1).ok_or(ZLibError::BufError)?;
                                    }
                                    from = out_buf.wrapping_sub(dist as usize);
                                }
                            } else if wnext < op as usize {
                                from = from.wrapping_add(wsize + wnext - op as usize);
                                let mut op = op as usize - wnext;
                                if op < len as usize {
                                    len -= op as u32;
                                    for _ in 0..op {
                                        unsafe { *out_buf = *from; }
                                        out_buf = out_buf.checked_add(1).ok_or(ZLibError::BufError)?;
                                        from = from.checked_add(1).ok_or(ZLibError::BufError)?;
                                    }
                                    from = window.as_ptr();
                                    if wnext < len as usize {
                                        op = wnext;
                                        len -= op as u32;
                                        for _ in 0..op {
                                            unsafe { *out_buf = *from; }
                                            out_buf = out_buf.checked_add(1).ok_or(ZLibError::BufError)?;
                                            from = from.checked_add(1).ok_or(ZLibError::BufError)?;
                                        }
                                        from = out_buf.wrapping_sub(dist as usize);
                                    }
                                }
                            } else {
                                from = from.wrapping_add(wnext - op as usize);
                                if op < len {
                                    len -= op;
                                    for _ in 0..op {
                                        unsafe { *out_buf = *from; }
                                        out_buf = out_buf.checked_add(1).ok_or(ZLibError::BufError)?;
                                        from = from.checked_add(1).ok_or(ZLibError::BufError)?;
                                    }
                                    from = out_buf.wrapping_sub(dist as usize);
                                }
                            }
                            while len > 2 {
                                unsafe {
                                    *out_buf = *from;
                                    out_buf = out_buf.checked_add(1).ok_or(ZLibError::BufError)?;
                                    *out_buf = *from;
                                    out_buf = out_buf.checked_add(1).ok_or(ZLibError::BufError)?;
                                    *out_buf = *from;
                                    out_buf = out_buf.checked_add(1).ok_or(ZLibError::BufError)?;
                                }
                                from = from.checked_add(3).ok_or(ZLibError::BufError)?;
                                len -= 3;
                            }
                            if len != 0 {
                                unsafe { *out_buf = *from; }
                                out_buf = out_buf.checked_add(1).ok_or(ZLibError::BufError)?;
                                if len > 1 {
                                    unsafe { *out_buf = *from.add(1); }
                                    out_buf = out_buf.checked_add(1).ok_or(ZLibError::BufError)?;
                                }
                            }
                        } else {
                            let mut from = out_buf.wrapping_sub(dist as usize);
                            while len > 2 {
                                unsafe {
                                    *out_buf = *from;
                                    out_buf = out_buf.checked_add(1).ok_or(ZLibError::BufError)?;
                                    *out_buf = *from.add(1);
                                    out_buf = out_buf.checked_add(1).ok_or(ZLibError::BufError)?;
                                    *out_buf = *from.add(2);
                                    out_buf = out_buf.checked_add(1).ok_or(ZLibError::BufError)?;
                                }
                                from = from.checked_add(3).ok_or(ZLibError::BufError)?;
                                len -= 3;
                            }
                            if len != 0 {
                                unsafe { *out_buf = *from; }
                                out_buf = out_buf.checked_add(1).ok_or(ZLibError::BufError)?;
                                if len > 1 {
                                    unsafe { *out_buf = *from.add(1); }
                                    out_buf = out_buf.checked_add(1).ok_or(ZLibError::BufError)?;
                                }
                            }
                        }
                        break;
                    } else if op & 64 == 0 {
                        // 2nd level distance code
                        here = dcode[(here.val as usize + (hold & ((1 << op) - 1)) as usize];
                        continue;
                    } else {
                        return Err(ZLibError::DataError("invalid distance code"));
                    }
                }
            } else if op & 64 == 0 {
                // 2nd level length code
                here = lcode[(here.val as usize + (hold & ((1 << op) - 1)) as usize];
                continue;
            } else if op & 32 != 0 {
                // end-of-block
                mode = InflateMode::Type;
                break;
            } else {
                return Err(ZLibError::DataError("invalid literal/length code"));
            }
        }
        if mode == InflateMode::Type {
            break;
        }
    }

    // Update state
    let len = bits >> 3;
    in_buf = in_buf.wrapping_sub(len as usize);
    bits -= len << 3;
    hold &= (1 << bits) - 1;

    strm.next_in = in_buf;
    strm.next_out = out_buf;
    strm.avail_in = if in_buf < last {
        5 + (last as usize - in_buf as usize)
    } else {
        5 - (in_buf as usize - last as usize)
    };
    strm.avail_out = if out_buf < end {
        257 + (end as usize - out_buf as usize)
    } else {
        257 - (out_buf as usize - end as usize)
    };
    state.hold = hold;
    state.bits = bits;
    state.mode = match mode {
        InflateMode::Len => crate::inflate::InflateMode::Len,
        InflateMode::Type => crate::inflate::InflateMode::Type,
        InflateMode::Bad => crate::inflate::InflateMode::Bad,
    };

    Ok(())
}