use std::char;

type WChar = i32;
type WInt = u32;

fn casecmp(a: WChar, b: WChar) -> i32 {
    (char::from_u32(a as WInt).map_or(0, |a| 
        char::from_u32(b as WInt).map_or(0, |b| 
            (a.to_uppercase().eq(b.to_uppercase())) as i32
        )
    )
}

fn exactcmp(a: WChar, b: WChar) -> i32 {
    (a == b) as i32
}

fn is_in_range(ch: WChar, p: &mut &[WChar], reverse: &mut i32) -> i32 {
    let mut found = 0;
    
    if !p.is_empty() && p[0] == '^' as i32 {
        *reverse = 1;
        *p = &p[1..];
    } else {
        *reverse = 0;
    }

    while !p.is_empty() && p[0] != ']' as i32 {
        let first = p[0];
        *p = &p[1..];
        
        if p.is_empty() {
            return 0;
        }

        if !p.is_empty() && p[0] == '-' as i32 {
            *p = &p[1..];
            if p.is_empty() {
                return 0;
            }
            
            let last = p[0];
            if last == ']' as i32 {
                if ch == first || ch == '-' as i32 {
                    found = 1;
                }
                break;
            } else {
                *p = &p[1..];
                if ch >= first && ch <= last {
                    found = 1;
                }
            }
        } else if ch == first {
            found = 1;
        }
    }
    
    found ^ *reverse
}

fn parse_range(
    p: &mut &[WChar],
    s: WChar,
    out: &mut Option<&mut WChar>,
    compfn: fn(WChar, WChar) -> i32,
) -> i32 {
    let mut reverse = 0;
    let mut p0 = *p;
    let mut p1 = *p;

    if let Some(out_ref) = out {
        **out_ref = s;
    }

    if is_in_range(s, p, &mut reverse) != 0 {
        return 1 ^ reverse;
    }

    if compfn == exactcmp {
        return reverse;
    }

    if let Some(lower) = char::from_u32(s as WInt).map(|c| c.to_lowercase().next().unwrap_or(c)) {
        if is_in_range(lower as WChar, &mut p0, &mut reverse) != 0 {
            if let Some(out_ref) = out {
                **out_ref = lower as WChar;
            }
            return 1 ^ reverse;
        }
    }

    if let Some(upper) = char::from_u32(s as WInt).map(|c| c.to_uppercase().next().unwrap_or(c)) {
        if is_in_range(upper as WChar, &mut p1, &mut reverse) != 0 {
            if let Some(out_ref) = out {
                **out_ref = upper as WChar;
            }
            return 1 ^ reverse;
        }
    }

    reverse
}

fn mt_match(
    s: &[WChar],
    p: &[WChar],
    out: &mut Option<&mut [WChar]>,
    case: i32,
    mut length: i32,
    compfn: fn(WChar, WChar) -> i32,
) -> i32 {
    let mut s_idx = 0;
    let mut p_idx = 0;
    let mut out_idx = 0;

    while p_idx < p.len() && length != 0 {
        match p[p_idx] {
            63 => { // '?'
                if s_idx >= s.len() {
                    return 0;
                }
                if let Some(out_slice) = out {
                    if out_idx < out_slice.len() {
                        out_slice[out_idx] = s[s_idx];
                        out_idx += 1;
                    }
                }
                s_idx += 1;
                p_idx += 1;
                length -= 1;
            }
            42 => { // '*'
                while p_idx < p.len() && p[p_idx] == '*' as i32 && length != 0 {
                    p_idx += 1;
                    length -= 1;
                }

                let mut remaining_s = &s[s_idx..];
                while !remaining_s.is_empty() {
                    if mt_match(remaining_s, &p[p_idx..], out, case, length, compfn) != 0 {
                        return 1;
                    }
                    if let Some(out_slice) = out {
                        if out_idx < out_slice.len() {
                            out_slice[out_idx] = remaining_s[0];
                            out_idx += 1;
                        }
                    }
                    remaining_s = &remaining_s[1..];
                }
                continue;
            }
            91 => { // '['
                p_idx += 1;
                length -= 1;
                if p_idx >= p.len() {
                    return 0;
                }

                let mut range_p = &p[p_idx..];
                if let Some(out_slice) = out {
                    if out_idx < out_slice.len() {
                        let result = parse_range(
                            &mut range_p,
                            s[s_idx],
                            &mut Some(&mut out_slice[out_idx..=out_idx]),
                            compfn,
                        );
                        if result == 0 {
                            return 0;
                        }
                        out_idx += 1;
                    }
                } else {
                    if parse_range(&mut range_p, s[s_idx], &mut None, compfn) == 0 {
                        return 0;
                    }
                }
                p_idx += p.len() - range_p.len();
            }
            92 => { // '\\'
                p_idx += 1;
                length -= 1;
                if p_idx >= p.len() {
                    return 0;
                }
                if compfn(s[s_idx], p[p_idx]) == 0 {
                    return 0;
                }
                if let Some(out_slice) = out {
                    if out_idx < out_slice.len() {
                        out_slice[out_idx] = p[p_idx];
                        out_idx += 1;
                    }
                }
            }
            _ => {
                if compfn(s[s_idx], p[p_idx]) == 0 {
                    return 0;
                }
                if let Some(out_slice) = out {
                    if out_idx < out_slice.len() {
                        out_slice[out_idx] = p[p_idx];
                        out_idx += 1;
                    }
                }
            }
        }
        s_idx += 1;
        p_idx += 1;
        length -= 1;
    }

    if let Some(out_slice) = out {
        if out_idx < out_slice.len() {
            out_slice[out_idx] = 0;
        }
    }

    if s_idx < s.len() { 0 } else { 1 }
}

#[no_mangle]
pub extern "C" fn r#match(
    s: *const WChar,
    p: *const WChar,
    out: *mut WChar,
    case: i32,
    length: i32,
) -> i32 {
    let s_slice = unsafe { std::slice::from_raw_parts(s, length as usize) };
    let p_slice = unsafe { std::slice::from_raw_parts(p, length as usize) };
    let out_slice = if !out.is_null() {
        Some(unsafe { std::slice::from_raw_parts_mut(out, length as usize) })
    } else {
        None
    };

    let compfn = if case != 0 { casecmp } else { exactcmp };

    mt_match(s_slice, p_slice, &mut out_slice, case, length, compfn)
}