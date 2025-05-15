pub type ucs4_t = u32;

pub fn u8_uctomb_aux(s: &mut [u8], uc: ucs4_t) -> Result<usize, i32> {
    let count = if uc < 0x80 {
        return Err(-2);
    } else if uc < 0x800 {
        2
    } else if uc < 0x10000 {
        if uc < 0xd800 || uc >= 0xe000 {
            3
        } else {
            return Err(-1);
        }
    } else if uc < 0x110000 {
        4
    } else {
        return Err(-1);
    };

    if s.len() < count {
        return Err(-2);
    }

    match count {
        4 => {
            s[3] = (0x80 | (uc & 0x3F)) as u8;
            let uc = (uc >> 6) | 0x10000;
            s[2] = (0x80 | (uc & 0x3F)) as u8;
            let uc = (uc >> 6) | 0x800;
            s[1] = (0x80 | (uc & 0x3F)) as u8;
            let uc = (uc >> 6) | 0xC0;
            s[0] = uc as u8;
        }
        3 => {
            s[2] = (0x80 | (uc & 0x3F)) as u8;
            let uc = (uc >> 6) | 0x800;
            s[1] = (0x80 | (uc & 0x3F)) as u8;
            let uc = (uc >> 6) | 0xC0;
            s[0] = uc as u8;
        }
        2 => {
            s[1] = (0x80 | (uc & 0x3F)) as u8;
            let uc = (uc >> 6) | 0xC0;
            s[0] = uc as u8;
        }
        _ => unreachable!(),
    }

    Ok(count)
}