use std::mem;

#[derive(Copy, Clone, Debug)]
pub struct Code {
    pub op: u8,
    pub bits: u8,
    pub val: u16,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CodeType {
    Codes,
    Lens,
    Dists,
}

pub const GLP_ZLIB_INFLATE_COPYRIGHT: &[u8; 47] = b" inflate 1.2.5 Copyright 1995-2010 Mark Adler \0";

pub fn inflate_table(
    type_: CodeType,
    lens: &[u16],
    codes: u32,
    table: &mut Vec<Code>,
    bits: &mut u32,
    work: &mut [u16],
) -> Result<(), &'static str> {
    let mut count = [0u16; 16];
    let mut offs = [0u16; 16];

    // Count the number of codes of each length
    for &len in lens.iter().take(codes as usize) {
        if len as usize >= count.len() {
            return Err("Invalid length");
        }
        count[len as usize] = count[len as usize].wrapping_add(1);
    }

    let mut root = *bits;
    let mut max = 15;
    while max >= 1 {
        if count[max] != 0 {
            break;
        }
        max -= 1;
    }

    if root > max as u32 {
        root = max as u32;
    }

    if max == 0 {
        table.push(Code {
            op: 64,
            bits: 1,
            val: 0,
        });
        table.push(Code {
            op: 64,
            bits: 1,
            val: 0,
        });
        *bits = 1;
        return Ok(());
    }

    let mut min = 1;
    while min < max {
        if count[min] != 0 {
            break;
        }
        min += 1;
    }

    if (root as usize) < min {
        root = min as u32;
    }

    let mut left = 1i32;
    for len in 1..=15 {
        left <<= 1;
        left -= count[len] as i32;
        if left < 0 {
            return Err("Over-subscribed");
        }
    }

    if left > 0 && (type_ == CodeType::Codes || max != 1) {
        return Err("Incomplete set");
    }

    offs[1] = 0;
    for len in 1..15 {
        offs[len + 1] = offs[len] + count[len];
    }

    for sym in 0..codes {
        let len = lens[sym as usize];
        if len != 0 {
            let offset = &mut offs[len as usize];
            work[*offset as usize] = sym as u16;
            *offset = offset.wrapping_add(1);
        }
    }

    let (base, extra, end) = match type_ {
        CodeType::Codes => (&work[..], &work[..], 19),
        CodeType::Lens => {
            const LBASE: [u16; 31] = [
                3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83,
                99, 115, 131, 163, 195, 227, 258, 0, 0,
            ];
            const LEXT: [u16; 31] = [
                16, 16, 16, 16, 16, 16, 16, 16, 17, 17, 17, 17, 18, 18, 18, 18, 19, 19, 19, 19, 20,
                20, 20, 20, 21, 21, 21, 21, 16, 73, 195,
            ];
            (&LBASE[..], &LEXT[..], 256)
        }
        CodeType::Dists => {
            const DBASE: [u16; 32] = [
                1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769,
                1025, 1537, 2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577, 0, 0,
            ];
            const DEXT: [u16; 32] = [
                16, 16, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22, 23, 23, 24, 24, 25,
                25, 26, 26, 27, 27, 28, 28, 29, 29, 64, 64,
            ];
            (&DBASE[..], &DEXT[..], -1)
        }
    };

    let mut huff = 0u32;
    let mut sym = 0u32;
    let mut len = min;
    let mut curr = root;
    let mut drop = 0u32;
    let mut low = !0u32;
    let mut used = 1u32 << root;
    let mask = used - 1;

    if (type_ == CodeType::Lens && used >= 852) || (type_ == CodeType::Dists && used >= 592) {
        return Err("Too many codes");
    }

    let mut next_index = table.len();
    table.resize(table.len() + (1 << max) as usize, Code { op: 0, bits: 0, val: 0 });

    while sym < codes {
        let mut here = Code {
            bits: (len - drop) as u8,
            op: 0,
            val: 0,
        };

        let work_sym = work[sym as usize];
        if (work_sym as i32) < end {
            here.op = 0;
            here.val = work_sym;
        } else if (work_sym as i32) > end {
            here.op = extra[work_sym as usize] as u8;
            here.val = base[work_sym as usize];
        } else {
            here.op = 32 + 64;
            here.val = 0;
        }

        let incr = 1u32 << (len - drop);
        let fill = 1u32 << curr;
        let mut min_fill = fill;
        while min_fill > 0 {
            min_fill -= incr;
            let index = next_index + ((huff >> drop) + min_fill) as usize;
            if index >= table.len() {
                return Err("Table index out of bounds");
            }
            table[index] = here;
        }

        let mut incr = 1u32 << (len - 1);
        while huff & incr != 0 {
            incr >>= 1;
        }
        if incr != 0 {
            huff &= incr - 1;
            huff += incr;
        } else {
            huff = 0;
        }

        sym += 1;
        count[len as usize] -= 1;

        if count[len as usize] == 0 {
            if len == max as u32 {
                break;
            }
            len = lens[work[sym as usize] as usize] as u32;
        }

        if len > root && (huff & mask) != low {
            if drop == 0 {
                drop = root;
            }
            next_index += (1 << curr) as usize;
            curr = len - drop;
            let mut left = 1i32 << curr;
            while curr + drop < max as u32 {
                left -= count[(curr + drop) as usize] as i32;
                if left <= 0 {
                    break;
                }
                curr += 1;
                left <<= 1;
            }
            used += 1u32 << curr;
            if (type_ == CodeType::Lens && used >= 852) || (type_ == CodeType::Dists && used >= 592) {
                return Err("Too many codes");
            }
            low = huff & mask;
            table[next_index + low as usize] = Code {
                op: curr as u8,
                bits: root as u8,
                val: (next_index - table.len()) as u16,
            };
        }
    }

    let mut here = Code {
        op: 64,
        bits: (len - drop) as u8,
        val: 0,
    };

    while huff != 0 {
        if drop != 0 && (huff & mask) != low {
            drop = 0;
            len = root;
            next_index = table.len();
            here.bits = len as u8;
        }
        let index = next_index + (huff >> drop) as usize;
        if index >= table.len() {
            return Err("Table index out of bounds");
        }
        table[index] = here;
        let mut incr = 1u32 << (len - 1);
        while huff & incr != 0 {
            incr >>= 1;
        }
        if incr != 0 {
            huff &= incr - 1;
            huff += incr;
        } else {
            huff = 0;
        }
    }

    *bits = root;
    Ok(())
}