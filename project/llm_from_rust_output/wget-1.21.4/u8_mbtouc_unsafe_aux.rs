pub fn u8_mbtouc_unsafe_aux(puc: &mut u32, s: &[u8]) -> usize {
    if s.is_empty() {
        *puc = 0xFFFD;
        return 1;
    }

    let c = s[0];
    if c >= 0xC2 {
        if c < 0xE0 {
            if s.len() >= 2 {
                if s[1] & 0xC0 == 0x80 {
                    *puc = ((c as u32 & 0x1F) << 6) | (s[1] as u32 & 0x3F);
                    return 2;
                }
            }
            *puc = 0xFFFD;
            return 1;
        } else if c < 0xF0 {
            if s.len() >= 3 {
                if s[1] & 0xC0 == 0x80 && s[2] & 0xC0 == 0x80 {
                    if (c >= 0xE1 || s[1] >= 0xA0) && (c != 0xED || s[1] < 0xA0) {
                        *puc = ((c as u32 & 0x0F) << 12)
                            | ((s[1] as u32 & 0x3F) << 6)
                            | (s[2] as u32 & 0x3F);
                        return 3;
                    }
                    *puc = 0xFFFD;
                    return 3;
                }
                *puc = 0xFFFD;
                return if s[1] & 0xC0 != 0x80 { 1 } else { 2 };
            }
            *puc = 0xFFFD;
            return if s.len() == 1 || s[1] & 0xC0 != 0x80 {
                1
            } else {
                2
            };
        } else if c < 0xF8 {
            if s.len() >= 4 {
                if s[1] & 0xC0 == 0x80
                    && s[2] & 0xC0 == 0x80
                    && s[3] & 0xC0 == 0x80
                {
                    if (c >= 0xF1 || s[1] >= 0x90)
                        && (c < 0xF4 || (c == 0xF4 && s[1] < 0x90))
                    {
                        *puc = ((c as u32 & 0x07) << 18)
                            | ((s[1] as u32 & 0x3F) << 12)
                            | ((s[2] as u32 & 0x3F) << 6)
                            | (s[3] as u32 & 0x3F);
                        return 4;
                    }
                    *puc = 0xFFFD;
                    return 4;
                }
                *puc = 0xFFFD;
                return if s[1] & 0xC0 != 0x80 {
                    1
                } else if s[2] & 0xC0 != 0x80 {
                    2
                } else {
                    3
                };
            }
            *puc = 0xFFFD;
            if s.len() == 1 || s[1] & 0xC0 != 0x80 {
                return 1;
            } else if s.len() == 2 || s[2] & 0xC0 != 0x80 {
                return 2;
            } else {
                return 3;
            }
        }
    }
    *puc = 0xFFFD;
    1
}