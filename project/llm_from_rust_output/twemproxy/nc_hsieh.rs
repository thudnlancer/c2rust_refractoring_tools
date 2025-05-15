pub fn hash_hsieh(key: &[u8]) -> u32 {
    let mut hash: u32 = 0;
    let key_length = key.len();

    if key_length == 0 {
        return 0;
    }

    let rem = key_length % 4;
    let chunks = key_length / 4;

    let mut pos = 0;
    for _ in 0..chunks {
        if pos + 4 > key_length {
            break;
        }

        let chunk = &key[pos..pos+4];
        let first = u16::from_le_bytes([chunk[0], chunk[1]]);
        let second = u16::from_le_bytes([chunk[2], chunk[3]]);

        hash = hash.wrapping_add(first as u32);
        let tmp = ((second as u32) << 11) ^ hash;
        hash = (hash << 16) ^ tmp;
        hash = hash.wrapping_add(hash >> 11);
        pos += 4;
    }

    match rem {
        3 => {
            let chunk = &key[pos..pos+3];
            let first = u16::from_le_bytes([chunk[0], chunk[1]]);
            hash = hash.wrapping_add(first as u32);
            hash ^= hash << 16;
            hash ^= (chunk[2] as u32) << 18;
            hash = hash.wrapping_add(hash >> 11);
        }
        2 => {
            let chunk = &key[pos..pos+2];
            let first = u16::from_le_bytes([chunk[0], chunk[1]]);
            hash = hash.wrapping_add(first as u32);
            hash ^= hash << 11;
            hash = hash.wrapping_add(hash >> 17);
        }
        1 => {
            hash = hash.wrapping_add(key[pos] as u32);
            hash ^= hash << 10;
            hash = hash.wrapping_add(hash >> 1);
        }
        _ => {}
    }

    hash ^= hash << 3;
    hash = hash.wrapping_add(hash >> 5);
    hash ^= hash << 4;
    hash = hash.wrapping_add(hash >> 17);
    hash ^= hash << 25;
    hash = hash.wrapping_add(hash >> 6);

    hash
}