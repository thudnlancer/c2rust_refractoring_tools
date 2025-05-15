use std::cmp::Ordering;

type Ucs4 = u32;

const NONSPACING_TABLE_DATA: [u8; 2432] = [
    // ... (original table data remains the same)
];

const NONSPACING_TABLE_IND: [i8; 248] = [
    // ... (original table indices remain the same)
];

fn streq(s1: &str, s2: &str) -> bool {
    s1 == s2
}

fn is_cjk_encoding(encoding: &str) -> bool {
    streq(encoding, "EUC-JP") ||
    streq(encoding, "GB2312") ||
    streq(encoding, "GBK") ||
    streq(encoding, "EUC-TW") ||
    streq(encoding, "BIG5") ||
    streq(encoding, "EUC-KR") ||
    streq(encoding, "CP949") ||
    streq(encoding, "JOHAB")
}

fn is_nonspacing(uc: Ucs4) -> bool {
    if (uc >> 9) < 248 {
        let ind = NONSPACING_TABLE_IND[(uc >> 9) as usize];
        if ind >= 0 {
            let idx = (64 * ind as usize) + ((uc >> 3) & 63) as usize;
            return (NONSPACING_TABLE_DATA[idx] >> (uc & 7)) & 1 != 0;
        }
    } else if (uc >> 9) == (0xe0000 >> 9) {
        match uc {
            0xe0100..=0xe01ef => return true,
            _ => {
                if (0xe0020..=0xe007f).contains(&uc) || uc == 0xe0001 {
                    return true;
                }
            }
        }
    }
    false
}

pub fn uc_width(uc: Ucs4, encoding: &str) -> i32 {
    if is_nonspacing(uc) {
        if uc > 0 && uc < 0xa0 {
            return -1;
        } else {
            return 0;
        }
    }

    if (0x1100..0x1160).contains(&uc) ||
       (0x2329..0x232b).contains(&uc) ||
       ((0x2e80..0xa4d0).contains(&uc) && uc != 0x303f && !(0x4dc0..0x4e00).contains(&uc)) ||
       (0xac00..0xd7a4).contains(&uc) ||
       (0xf900..0xfb00).contains(&uc) ||
       (0xfe10..0xfe20).contains(&uc) ||
       (0xfe30..0xfe70).contains(&uc) ||
       (0xff00..0xff61).contains(&uc) ||
       (0xffe0..0xffe7).contains(&uc) ||
       (0x20000..=0x2ffff).contains(&uc) ||
       (0x30000..=0x3ffff).contains(&uc)
    {
        return 2;
    }

    if (0xa1..0xff61).contains(&uc) && uc != 0x20a9 && is_cjk_encoding(encoding) {
        return 2;
    }

    1
}