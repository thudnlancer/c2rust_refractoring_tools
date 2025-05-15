pub type Uint32 = u32;
pub type Uint64 = u64;

pub fn camellia_absorb(nkeys: Uint32, dst: &mut [Uint64], subkey: &mut [Uint64]) {
    let mut kw2 = subkey[1];
    subkey[3] ^= kw2;
    subkey[5] ^= kw2;
    subkey[7] ^= kw2;

    let mut i = 8;
    while i < nkeys as usize {
        kw2 ^= (kw2 & !subkey[i + 1]) << 32;
        let dw = ((kw2 & subkey[i + 1]) >> 32) as Uint32;
        kw2 ^= (dw << 1 | dw >> 31) as Uint64;
        subkey[i + 3] ^= kw2;
        subkey[i + 5] ^= kw2;
        subkey[i + 7] ^= kw2;
        i += 8;
    }
    subkey[i] ^= kw2;

    let mut kw4 = subkey[nkeys as usize + 1];
    i = (nkeys - 8) as usize;
    while i > 0 {
        subkey[i + 6] ^= kw4;
        subkey[i + 4] ^= kw4;
        subkey[i + 2] ^= kw4;
        kw4 ^= (kw4 & !subkey[i]) << 32;
        let dw = ((kw4 & subkey[i]) >> 32) as Uint32;
        kw4 ^= (dw << 1 | dw >> 31) as Uint64;
        i -= 8;
    }
    subkey[6] ^= kw4;
    subkey[4] ^= kw4;
    subkey[2] ^= kw4;
    subkey[0] ^= kw4;

    dst[0] = subkey[0] ^ subkey[2];
    dst[1] = subkey[3];
    dst[2] = subkey[2] ^ subkey[4];
    dst[3] = subkey[3] ^ subkey[5];
    dst[4] = subkey[4] ^ subkey[6];
    dst[5] = subkey[5] ^ subkey[7];

    i = 8;
    while i < nkeys as usize {
        let tl = ((subkey[i + 2] >> 32) ^ (subkey[i + 2] & !subkey[i])) as Uint32;
        let dw = (tl as Uint64 & (subkey[i] >> 32)) as Uint32;
        let tr = (subkey[i + 2] ^ ((dw << 1 | dw >> 31) as Uint64)) as Uint32;
        dst[i - 2] = subkey[i - 2] ^ ((tl as Uint64) << 32 | tr as Uint64);
        dst[i - 1] = subkey[i];
        dst[i] = subkey[i + 1];

        let tl = ((subkey[i - 1] >> 32) ^ (subkey[i - 1] & !subkey[i + 1])) as Uint32;
        let dw = (tl as Uint64 & (subkey[i + 1] >> 32)) as Uint32;
        let tr = (subkey[i - 1] ^ ((dw << 1 | dw >> 31) as Uint64)) as Uint32;
        dst[i + 1] = subkey[i + 3] ^ ((tl as Uint64) << 32 | tr as Uint64);
        dst[i + 2] = subkey[i + 2] ^ subkey[i + 4];
        dst[i + 3] = subkey[i + 3] ^ subkey[i + 5];
        dst[i + 4] = subkey[i + 4] ^ subkey[i + 6];
        dst[i + 5] = subkey[i + 5] ^ subkey[i + 7];
        i += 8;
    }

    dst[i - 2] = subkey[i - 2];
    dst[i - 1] = subkey[i] ^ subkey[i - 1];
}