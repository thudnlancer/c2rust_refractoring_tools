// base64-encode.rs

const BASE64_ENCODE_TABLE: [u8; 64] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                        abcdefghijklmnopqrstuvwxyz\
                                        0123456789+/";

fn encode_raw(alphabet: &[u8; 64], dst: &mut [u8], src: &[u8]) {
    let length = src.len();
    let mut in_ptr = length;
    let mut out_ptr = BASE64_ENCODE_RAW_LENGTH(length);

    let left_over = length % 3;

    if left_over != 0 {
        in_ptr -= left_over;
        out_ptr -= 1;
        dst[out_ptr] = b'=';

        match left_over {
            1 => {
                out_ptr -= 1;
                dst[out_ptr] = b'=';
                out_ptr -= 1;
                dst[out_ptr] = alphabet[(src[in_ptr] << 4) as usize & 0x3F];
            }
            2 => {
                out_ptr -= 1;
                dst[out_ptr] = alphabet[(src[in_ptr + 1] << 2) as usize & 0x3F];
                out_ptr -= 1;
                dst[out_ptr] = alphabet[((src[in_ptr] << 4 | src[in_ptr + 1] >> 4) as usize & 0x3F];
            }
            _ => unreachable!(),
        }
        out_ptr -= 1;
        dst[out_ptr] = alphabet[(src[in_ptr] >> 2) as usize & 0x3F];
    }

    while in_ptr > 0 {
        in_ptr -= 3;
        out_ptr -= 1;
        dst[out_ptr] = alphabet[src[in_ptr + 2] as usize & 0x3F];
        out_ptr -= 1;
        dst[out_ptr] = alphabet[((src[in_ptr + 1] << 2 | src[in_ptr + 2] >> 6) as usize) & 0x3F];
        out_ptr -= 1;
        dst[out_ptr] = alphabet[((src[in_ptr] << 4 | src[in_ptr + 1] >> 4) as usize) & 0x3F];
        out_ptr -= 1;
        dst[out_ptr] = alphabet[(src[in_ptr] >> 2) as usize & 0x3F];
    }
    assert_eq!(in_ptr, 0);
    assert_eq!(out_ptr, 0);
}

pub fn base64_encode_raw(dst: &mut [u8], src: &[u8]) {
    encode_raw(&BASE64_ENCODE_TABLE, dst, src);
}

pub fn base64_encode_group(dst: &mut [u8], group: u32) {
    dst[0] = BASE64_ENCODE_TABLE[((group >> 18) & 0x3F) as usize];
    dst[1] = BASE64_ENCODE_TABLE[((group >> 12) & 0x3F) as usize];
    dst[2] = BASE64_ENCODE_TABLE[((group >> 6) & 0x3F) as usize];
    dst[3] = BASE64_ENCODE_TABLE[(group & 0x3F) as usize];
}

pub struct Base64EncodeCtx {
    word: u32,
    bits: u8,
    alphabet: [u8; 64],
}

impl Base64EncodeCtx {
    pub fn new() -> Self {
        Self {
            word: 0,
            bits: 0,
            alphabet: BASE64_ENCODE_TABLE,
        }
    }

    pub fn encode_single(&mut self, dst: &mut [u8], src: u8) -> usize {
        let mut done = 0;
        let mut word = (self.word << 8) | src as u32;
        let mut bits = self.bits + 8;

        while bits >= 6 {
            bits -= 6;
            dst[done] = self.alphabet[((word >> bits) & 0x3F) as usize];
            done += 1;
        }

        self.bits = bits;
        self.word = word;

        assert!(done <= 2);
        done
    }

    pub fn encode_update(&mut self, dst: &mut [u8], src: &[u8]) -> usize {
        let mut done = 0;
        let mut left = src.len();
        let mut src_ptr = 0;

        while self.bits > 0 && left > 0 {
            left -= 1;
            done += self.encode_single(&mut dst[done..], src[src_ptr]);
            src_ptr += 1;
        }

        let left_over = left % 3;
        let bulk = left - left_over;

        if bulk > 0 {
            assert!(bulk % 3 == 0);
            encode_raw(&self.alphabet, &mut dst[done..], &src[src_ptr..src_ptr + bulk]);
            done += BASE64_ENCODE_RAW_LENGTH(bulk);
            src_ptr += bulk;
            left = left_over;
        }

        while left > 0 {
            left -= 1;
            done += self.encode_single(&mut dst[done..], src[src_ptr]);
            src_ptr += 1;
        }

        assert!(done <= BASE64_ENCODE_LENGTH(src.len()));
        done
    }

    pub fn encode_final(&mut self, dst: &mut [u8]) -> usize {
        let mut done = 0;
        let bits = self.bits;

        if bits > 0 {
            dst[done] = self.alphabet[((self.word << (6 - bits)) & 0x3F) as usize];
            done += 1;
            for _ in bits..6 {
                dst[done] = b'=';
                done += 1;
            }
            self.bits = 0;
        }

        assert!(done <= BASE64_ENCODE_FINAL_LENGTH);
        done
    }
}

const fn BASE64_ENCODE_RAW_LENGTH(length: usize) -> usize {
    ((length + 2) / 3) * 4
}

const fn BASE64_ENCODE_LENGTH(length: usize) -> usize {
    ((length * 4) + 2) / 3
}

const fn BASE64_ENCODE_FINAL_LENGTH() -> usize {
    4
}