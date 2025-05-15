pub fn u8_mbtoucr(s: &[u8]) -> Result<(u32, usize), (u32, isize)> {
    if s.is_empty() {
        return Err((0xFFFD, -1));
    }

    let c = s[0];
    if c < 0x80 {
        return Ok((c as u32, 1));
    } else if c >= 0xC2 {
        if c < 0xE0 {
            if s.len() >= 2 {
                let c2 = s[1];
                if (c2 ^ 0x80) < 0x40 {
                    let uc = ((c as u32 & 0x1F) << 6) | (c2 as u32 ^ 0x80);
                    return Ok((uc, 2));
                }
            }
            return Err((0xFFFD, -2));
        } else if c < 0xF0 {
            if s.len() >= 2 {
                let c2 = s[1];
                if (c2 ^ 0x80) < 0x40 
                    && (c >= 0xE1 || c2 >= 0xA0)
                    && (c != 0xED || c2 < 0xA0)
                {
                    if s.len() >= 3 {
                        let c3 = s[2];
                        if (c3 ^ 0x80) < 0x40 {
                            let uc = ((c as u32 & 0x0F) << 12)
                                | ((c2 as u32 ^ 0x80) << 6)
                                | (c3 as u32 ^ 0x80);
                            return Ok((uc, 3));
                        }
                    }
                    return Err((0xFFFD, -2));
                }
            }
            return Err((0xFFFD, -2));
        } else if c < 0xF8 {
            if s.len() >= 2 {
                let c2 = s[1];
                if (c2 ^ 0x80) < 0x40
                    && (c >= 0xF1 || c2 >= 0x90)
                    && (c < 0xF4 || (c == 0xF4 && c2 < 0x90))
                {
                    if s.len() >= 3 {
                        let c3 = s[2];
                        if (c3 ^ 0x80) < 0x40 {
                            if s.len() >= 4 {
                                let c4 = s[3];
                                if (c4 ^ 0x80) < 0x40 {
                                    let uc = ((c as u32 & 0x07) << 18)
                                        | ((c2 as u32 ^ 0x80) << 12)
                                        | ((c3 as u32 ^ 0x80) << 6)
                                        | (c4 as u32 ^ 0x80);
                                    return Ok((uc, 4));
                                }
                            }
                            return Err((0xFFFD, -2));
                        }
                    }
                    return Err((0xFFFD, -2));
                }
            }
            return Err((0xFFFD, -2));
        }
    }
    Err((0xFFFD, -1))
}