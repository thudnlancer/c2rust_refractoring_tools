/*
 * By Bob Jenkins, 2006.  bob_jenkins@burtleburtle.net.  You may use this
 * code any way you wish, private, educational, or commercial.  It's free.
 * Use for hash table lookup, or anything where one collision in 2^^32 is
 * acceptable.  Do NOT use for cryptographic purposes.
 * http://burtleburtle.net/bob/hash/index.html
 *
 * Modified by Brian Pontz for libmemcached
 * TODO:
 * Add big endian support
 */

const JENKINS_INITVAL: u32 = 13;

fn rot(x: u32, k: u32) -> u32 {
    (x << k) | (x >> (32 - k))
}

fn mix(a: &mut u32, b: &mut u32, c: &mut u32) {
    *a -= *c;  *a ^= rot(*c, 4);  *c += *b;
    *b -= *a;  *b ^= rot(*a, 6);  *a += *c;
    *c -= *b;  *c ^= rot(*b, 8);  *b += *a;
    *a -= *c;  *a ^= rot(*c,16);  *c += *b;
    *b -= *a;  *b ^= rot(*a,19);  *a += *c;
    *c -= *b;  *c ^= rot(*b, 4);  *b += *a;
}

fn final_mix(a: &mut u32, b: &mut u32, c: &mut u32) {
    *c ^= *b; *c -= rot(*b,14);
    *a ^= *c; *a -= rot(*c,11);
    *b ^= *a; *b -= rot(*a,25);
    *c ^= *b; *c -= rot(*b,16);
    *a ^= *c; *a -= rot(*c,4);
    *b ^= *a; *b -= rot(*a,14);
    *c ^= *b; *c -= rot(*b,24);
}

/*
 * jenkins_hash() -- hash a variable-length key into a 32-bit value
 *  key     : the key (the unaligned variable-length array of bytes)
 *  length  : the length of the key, counting by bytes
 * Returns a 32-bit value.  Every bit of the key affects every bit of
 * the return value.  Two keys differing by one or two bits will have
 * totally different hash values.
 *
 * The best hash table sizes are powers of 2.  There is no need to do
 * mod a prime (mod is sooo slow!).  If you need less than 32 bits,
 * use a bitmask.  For example, if you need only 10 bits, do
 *   h = (h & hashmask(10));
 * In which case, the hash table should have hashsize(10) elements.
 */
pub fn hash_jenkins(key: &[u8]) -> u32 {
    let length = key.len();
    let mut a = 0xdeadbeef + (length as u32) + JENKINS_INITVAL;
    let mut b = a;
    let mut c = a;

    let mut chunks = key.chunks_exact(4);
    for chunk in chunks.by_ref() {
        if chunk.len() == 4 {
            let k = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            a += k;
            b += chunks.next().map_or(0, |ch| {
                if ch.len() == 4 {
                    u32::from_le_bytes([ch[0], ch[1], ch[2], ch[3]])
                } else { 0 }
            });
            c += chunks.next().map_or(0, |ch| {
                if ch.len() == 4 {
                    u32::from_le_bytes([ch[0], ch[1], ch[2], ch[3]])
                } else { 0 }
            });
            mix(&mut a, &mut b, &mut c);
        }
    }

    let remainder = chunks.remainder();
    match remainder.len() {
        12 => {
            c += u32::from_le_bytes([remainder[8], remainder[9], remainder[10], remainder[11]]);
            b += u32::from_le_bytes([remainder[4], remainder[5], remainder[6], remainder[7]]);
            a += u32::from_le_bytes([remainder[0], remainder[1], remainder[2], remainder[3]]);
        }
        11 => {
            c += (remainder[10] as u32) << 16;
            c += (remainder[9] as u32) << 8;
            c += remainder[8] as u32;
            b += u32::from_le_bytes([remainder[4], remainder[5], remainder[6], remainder[7]]);
            a += u32::from_le_bytes([remainder[0], remainder[1], remainder[2], remainder[3]]);
        }
        10 => {
            c += (remainder[9] as u32) << 8;
            c += remainder[8] as u32;
            b += u32::from_le_bytes([remainder[4], remainder[5], remainder[6], remainder[7]]);
            a += u32::from_le_bytes([remainder[0], remainder[1], remainder[2], remainder[3]]);
        }
        9 => {
            c += remainder[8] as u32;
            b += u32::from_le_bytes([remainder[4], remainder[5], remainder[6], remainder[7]]);
            a += u32::from_le_bytes([remainder[0], remainder[1], remainder[2], remainder[3]]);
        }
        8 => {
            b += u32::from_le_bytes([remainder[4], remainder[5], remainder[6], remainder[7]]);
            a += u32::from_le_bytes([remainder[0], remainder[1], remainder[2], remainder[3]]);
        }
        7 => {
            b += (remainder[6] as u32) << 16;
            b += (remainder[5] as u32) << 8;
            b += remainder[4] as u32;
            a += u32::from_le_bytes([remainder[0], remainder[1], remainder[2], remainder[3]]);
        }
        6 => {
            b += (remainder[5] as u32) << 8;
            b += remainder[4] as u32;
            a += u32::from_le_bytes([remainder[0], remainder[1], remainder[2], remainder[3]]);
        }
        5 => {
            b += remainder[4] as u32;
            a += u32::from_le_bytes([remainder[0], remainder[1], remainder[2], remainder[3]]);
        }
        4 => {
            a += u32::from_le_bytes([remainder[0], remainder[1], remainder[2], remainder[3]]);
        }
        3 => {
            a += (remainder[2] as u32) << 16;
            a += (remainder[1] as u32) << 8;
            a += remainder[0] as u32;
        }
        2 => {
            a += (remainder[1] as u32) << 8;
            a += remainder[0] as u32;
        }
        1 => {
            a += remainder[0] as u32;
        }
        0 => return c,
        _ => return c,
    }

    final_mix(&mut a, &mut b, &mut c);
    c
}