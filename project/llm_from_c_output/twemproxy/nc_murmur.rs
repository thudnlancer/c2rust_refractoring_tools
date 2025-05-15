/*
 * "Murmur" hash provided by Austin, tanjent@gmail.com
 * http://murmurhash.googlepages.com/
 *
 * Note - This code makes a few assumptions about how your machine behaves -
 *
 * 1. We can read a 4-byte value from any address without crashing
 * 2. sizeof(int) == 4
 *
 * And it has a few limitations -
 * 1. It will not work incrementally.
 * 2. It will not produce the same results on little-endian and big-endian
 *  machines.
 *
 *  Updated to murmur2 hash - BP
 */

pub fn hash_murmur(key: &[u8]) -> u32 {
    /*
     * 'm' and 'r' are mixing constants generated offline.  They're not
     * really 'magic', they just happen to work well.
     */
    const M: u32 = 0x5bd1e995;
    let length = key.len() as u32;
    let seed = 0xdeadbeef_u32.wrapping_mul(length);
    const R: i32 = 24;

    /* Initialize the hash to a 'random' value */
    let mut h = seed ^ length;

    /* Mix 4 bytes at a time into the hash */
    let mut data = key;
    
    while data.len() >= 4 {
        let mut k = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);

        k = k.wrapping_mul(M);
        k ^= k >> R;
        k = k.wrapping_mul(M);

        h = h.wrapping_mul(M);
        h ^= k;

        data = &data[4..];
    }

    /* Handle the last few bytes of the input array */
    match data.len() {
        3 => {
            h ^= (data[2] as u32) << 16;
            h ^= (data[1] as u32) << 8;
            h ^= data[0] as u32;
            h = h.wrapping_mul(M);
        }
        2 => {
            h ^= (data[1] as u32) << 8;
            h ^= data[0] as u32;
            h = h.wrapping_mul(M);
        }
        1 => {
            h ^= data[0] as u32;
            h = h.wrapping_mul(M);
        }
        _ => {}
    }

    /*
     * Do a few final mixes of the hash to ensure the last few bytes are
     * well-incorporated.
     */
    h ^= h >> 13;
    h = h.wrapping_mul(M);
    h ^= h >> 15;

    h
}