/* inftrees.rs -- generate Huffman trees for efficient decoding
 * Translated from C to Rust with safety and idiomatic practices
 */

use std::cmp;

/// Operation type for decoding tables
#[derive(Debug, Clone, Copy)]
pub struct Code {
    pub op: u8,     // operation, extra bits, table bits
    pub bits: u8,   // bits in this part of the code
    pub val: u16,   // offset in table or code value
}

/// Type of code to build
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeType {
    Codes,
    Lens,
    Dists,
}

/// Maximum bits in a code
const MAXBITS: usize = 15;

/// Enough space for literal/length codes
const ENOUGH_LENS: usize = 852;
/// Enough space for distance codes
const ENOUGH_DISTS: usize = 592;
/// Total enough space
const ENOUGH: usize = ENOUGH_LENS + ENOUGH_DISTS;

/// Base values for length codes 257..285
const LBASE: [u16; 31] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31,
    35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258, 0, 0,
];

/// Extra bits for length codes 257..285
const LEXT: [u16; 31] = [
    16, 16, 16, 16, 16, 16, 16, 16, 17, 17, 17, 17, 18, 18, 18, 18,
    19, 19, 19, 19, 20, 20, 20, 20, 21, 21, 21, 21, 16, 73, 195,
];

/// Base values for distance codes 0..29
const DBASE: [u16; 32] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193,
    257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145,
    8193, 12289, 16385, 24577, 0, 0,
];

/// Extra bits for distance codes 0..29
const DEXT: [u16; 32] = [
    16, 16, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22,
    23, 23, 24, 24, 25, 25, 26, 26, 27, 27,
    28, 28, 29, 29, 64, 64,
];

/// Build Huffman decoding tables
pub fn inflate_table(
    typ: CodeType,
    lens: &[u16],
    codes: usize,
    table: &mut Vec<Code>,
    bits: &mut usize,
    work: &mut [u16],
) -> Result<(), &'static str> {
    let mut len;                   // a code's length in bits
    let mut sym;                   // index of code symbols
    let mut min;                   // minimum code length
    let mut max = *bits;           // maximum code length
    let mut root = *bits;          // number of index bits for root table
    let mut curr;                  // number of index bits for current table
    let mut drop;                  // code bits to drop for sub-table
    let mut left;                  // number of prefix codes available
    let mut used = 0;              // code entries in table used
    let mut huff = 0;              // Huffman code
    let mut incr;                  // for incrementing code, index
    let mut fill;                  // index for replicating entries
    let mut low = u32::MAX;        // low bits for current root entry
    let mut mask;                 // mask for low root bits
    let mut here: Code;            // table entry for duplication
    let mut count = [0; MAXBITS + 1]; // number of codes of each length
    let mut offs = [0; MAXBITS + 1];  // offsets in table for each length

    // Accumulate lengths for codes (assumes lens[] all in 0..MAXBITS)
    for &l in lens.iter().take(codes) {
        count[l as usize] += 1;
    }

    // Bound code lengths, force root to be within code lengths
    for max_pos in (1..=MAXBITS).rev() {
        if count[max_pos] != 0 {
            max = max_pos;
            break;
        }
    }
    if root > max {
        root = max;
    }
    if max == 0 {
        // no symbols to code at all
        here = Code {
            op: 64,    // invalid code marker
            bits: 1,
            val: 0,
        };
        table.push(here);
        table.push(here);
        *bits = 1;
        return Ok(()); // no symbols, but wait for decoding to report error
    }

    for min_pos in 1..max {
        if count[min_pos] != 0 {
            min = min_pos;
            break;
        }
    }
    if root < min {
        root = min;
    }

    // Check for an over-subscribed or incomplete set of lengths
    left = 1;
    for len in 1..=MAXBITS {
        left <<= 1;
        left -= count[len];
        if left < 0 {
            return Err("over-subscribed");
        }
    }
    if left > 0 && (typ == CodeType::Codes || max != 1) {
        return Err("incomplete set");
    }

    // Generate offsets into symbol table for each length for sorting
    offs[1] = 0;
    for len in 1..MAXBITS {
        offs[len + 1] = offs[len] + count[len];
    }

    // Sort symbols by length, by symbol order within each length
    for sym in 0..codes {
        let l = lens[sym] as usize;
        if l != 0 {
            work[offs[l]] = sym as u16;
            offs[l] += 1;
        }
    }

    // Set up for code type
    let (base, extra, end) = match typ {
        CodeType::Codes => (&work[..], &work[..], 19),
        CodeType::Lens => (&LBASE[..], &LEXT[..], 256),
        CodeType::Dists => (&DBASE[..], &DEXT[..], -1),
    };

    // Initialize state for loop
    huff = 0;                   // starting code
    sym = 0;                    // starting code symbol
    len = min;                  // starting code length
    curr = root;                // current table index bits
    drop = 0;                   // current bits to drop from code for index
    low = u32::MAX;             // trigger new sub-table when len > root
    used = 1 << root;           // use root table entries
    mask = used - 1;            // mask for comparing low

    // Check available table space
    if (typ == CodeType::Lens && used >= ENOUGH_LENS) ||
       (typ == CodeType::Dists && used >= ENOUGH_DISTS) {
        return Err("enough space not available");
    }

    // Process all codes and make table entries
    loop {
        // Create table entry
        here.bits = (len - drop) as u8;
        if (work[sym] as i32) < end {
            here.op = 0;
            here.val = work[sym];
        } else if (work[sym] as i32) > end {
            here.op = extra[work[sym] as usize] as u8;
            here.val = base[work[sym] as usize];
        } else {
            here.op = 32 + 64; // end of block
            here.val = 0;
        }

        // Replicate for those indices with low len bits equal to huff
        incr = 1 << (len - drop);
        fill = 1 << curr;
        let mut min_fill = fill; // save offset to next table
        while fill > 0 {
            fill -= incr;
            let index = (huff >> drop) + fill;
            if index >= table.len() {
                table.resize(index + 1, Code { op: 0, bits: 0, val: 0 });
            }
            table[index] = here;
        }

        // Backwards increment the len-bit code huff
        incr = 1 << (len - 1);
        while huff & incr != 0 {
            incr >>= 1;
        }
        if incr != 0 {
            huff &= incr - 1;
            huff += incr;
        } else {
            huff = 0;
        }

        // Go to next symbol, update count, len
        sym += 1;
        count[len] -= 1;
        if count[len] == 0 {
            if len == max {
                break;
            }
            len = lens[work[sym] as usize] as usize;
        }

        // Create new sub-table if needed
        if len > root && (huff & mask) != low {
            // If first time, transition to sub-tables
            if drop == 0 {
                drop = root;
            }

            // Determine length of next table
            curr = len - drop;
            left = 1 << curr;
            while curr + drop < max {
                left -= count[curr + drop];
                if left <= 0 {
                    break;
                }
                curr += 1;
                left <<= 1;
            }

            // Check for enough space
            used += 1 << curr;
            if (typ == CodeType::Lens && used >= ENOUGH_LENS) ||
               (typ == CodeType::Dists && used >= ENOUGH_DISTS) {
                return Err("enough space not available");
            }

            // Point entry in root table to sub-table
            low = huff & mask;
            table[low as usize].op = curr as u8;
            table[low as usize].bits = root as u8;
            table[low as usize].val = (table.len() - low as usize) as u16;
        }
    }

    // Fill in rest of table for incomplete codes
    here.op = 64;                // invalid code marker
    here.bits = (len - drop) as u8;
    here.val = 0;
    while huff != 0 {
        // When done with sub-table, drop back to root table
        if drop != 0 && (huff & mask) != low {
            drop = 0;
            len = root;
            curr = root;
            here.bits = len as u8;
        }

        // Put invalid code marker in table
        let index = huff >> drop;
        if index >= table.len() {
            table.resize(index + 1, Code { op: 0, bits: 0, val: 0 });
        }
        table[index] = here;

        // Backwards increment the len-bit code huff
        incr = 1 << (len - 1);
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