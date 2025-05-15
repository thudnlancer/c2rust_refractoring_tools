const HEX_DIGITS: [u8; 16] = *b"0123456789abcdef";

pub fn nettle_base16_encode_single(dst: &mut [u8; 2], src: u8) {
    dst[0] = HEX_DIGITS[(src >> 4) as usize];
    dst[1] = HEX_DIGITS[(src & 0xf) as usize];
}

pub fn nettle_base16_encode_update(dst: &mut [u8], src: &[u8]) {
    assert!(dst.len() >= src.len() * 2);
    
    for (i, &byte) in src.iter().enumerate() {
        let offset = i * 2;
        nettle_base16_encode_single(
            (&mut dst[offset..offset + 2]).try_into().unwrap(),
            byte
        );
    }
}