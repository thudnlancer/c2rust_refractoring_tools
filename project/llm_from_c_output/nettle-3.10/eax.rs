use std::mem;
use std::ptr;
use std::slice;

const EAX_BLOCK_SIZE: usize = 16;
const EAX_DIGEST_SIZE: usize = 16;
const EAX_IV_SIZE: usize = 16;

#[derive(Clone, Copy)]
struct Block16 {
    b: [u8; EAX_BLOCK_SIZE],
}

impl Block16 {
    fn new() -> Self {
        Block16 { b: [0; EAX_BLOCK_SIZE] }
    }

    fn xor(&mut self, other: &Block16) {
        for (a, b) in self.b.iter_mut().zip(other.b.iter()) {
            *a ^= *b;
        }
    }

    fn mulx_be(&mut self) {
        let overflow = (self.b[0] & 0x80) != 0;
        for i in 0..EAX_BLOCK_SIZE - 1 {
            self.b[i] = (self.b[i] << 1) | (self.b[i + 1] >> 7);
        }
        self.b[EAX_BLOCK_SIZE - 1] <<= 1;
        if overflow {
            self.b[EAX_BLOCK_SIZE - 1] ^= 0x87;
        }
    }
}

struct EaxKey {
    pad_block: Block16,
    pad_partial: Block16,
}

struct EaxCtx {
    omac_nonce: Block16,
    omac_data: Block16,
    omac_message: Block16,
    ctr: Block16,
}

impl EaxKey {
    fn new() -> Self {
        EaxKey {
            pad_block: Block16::new(),
            pad_partial: Block16::new(),
        }
    }
}

impl EaxCtx {
    fn new() -> Self {
        EaxCtx {
            omac_nonce: Block16::new(),
            omac_data: Block16::new(),
            omac_message: Block16::new(),
            ctr: Block16::new(),
        }
    }
}

fn omac_init(state: &mut Block16, t: u8) {
    state.b = [0; EAX_BLOCK_SIZE];
    state.b[EAX_BLOCK_SIZE - 1] = t;
}

fn omac_update(
    state: &mut Block16,
    key: &EaxKey,
    cipher: &dyn Fn(&[u8], &mut [u8]),
    data: &[u8],
) {
    let mut chunks = data.chunks_exact(EAX_BLOCK_SIZE);
    for chunk in chunks.by_ref() {
        cipher(&state.b, &mut state.b);
        for (s, d) in state.b.iter_mut().zip(chunk) {
            *s ^= *d;
        }
    }

    let remainder = chunks.remainder();
    if !remainder.is_empty() {
        cipher(&state.b, &mut state.b);
        for (s, d) in state.b.iter_mut().zip(remainder) {
            *s ^= *d;
        }
        state.b[remainder.len()] ^= 0x80;
        state.xor(&key.pad_partial);
    }
}

fn omac_final(state: &mut Block16, key: &EaxKey, cipher: &dyn Fn(&[u8], &mut [u8])) {
    state.xor(&key.pad_block);
    cipher(&state.b, &mut state.b);
}

fn eax_set_key(key: &mut EaxKey, cipher: &dyn Fn(&[u8], &mut [u8])) {
    let zero_block = Block16::new();
    cipher(&zero_block.b, &mut key.pad_block.b);
    key.pad_block.mulx_be();
    key.pad_partial = key.pad_block;
    key.pad_partial.mulx_be();
    key.pad_partial.xor(&key.pad_block);
}

fn eax_set_nonce(
    eax: &mut EaxCtx,
    key: &EaxKey,
    cipher: &dyn Fn(&[u8], &mut [u8]),
    nonce: &[u8],
) {
    omac_init(&mut eax.omac_nonce, 0);
    omac_update(&mut eax.omac_nonce, key, cipher, nonce);
    omac_final(&mut eax.omac_nonce, key, cipher);
    eax.ctr = eax.omac_nonce;

    omac_init(&mut eax.omac_data, 1);
    omac_init(&mut eax.omac_message, 2);
}

fn eax_update(
    eax: &mut EaxCtx,
    key: &EaxKey,
    cipher: &dyn Fn(&[u8], &mut [u8]),
    data: &[u8],
) {
    omac_update(&mut eax.omac_data, key, cipher, data);
}

fn eax_encrypt(
    eax: &mut EaxCtx,
    key: &EaxKey,
    cipher: &dyn Fn(&[u8], &mut [u8]),
    dst: &mut [u8],
    src: &[u8],
) {
    ctr_crypt(&eax.ctr.b, cipher, dst, src);
    omac_update(&mut eax.omac_message, key, cipher, dst);
}

fn eax_decrypt(
    eax: &mut EaxCtx,
    key: &EaxKey,
    cipher: &dyn Fn(&[u8], &mut [u8]),
    dst: &mut [u8],
    src: &[u8],
) {
    omac_update(&mut eax.omac_message, key, cipher, src);
    ctr_crypt(&eax.ctr.b, cipher, dst, src);
}

fn eax_digest(
    eax: &mut EaxCtx,
    key: &EaxKey,
    cipher: &dyn Fn(&[u8], &mut [u8]),
    digest: &mut [u8],
) {
    assert!(!digest.is_empty());
    assert!(digest.len() <= EAX_BLOCK_SIZE);
    
    omac_final(&mut eax.omac_data, key, cipher);
    omac_final(&mut eax.omac_message, key, cipher);

    eax.omac_nonce.xor(&eax.omac_data);
    for (d, (n, m)) in digest.iter_mut().zip(
        eax.omac_nonce.b.iter().zip(eax.omac_message.b.iter())
    ) {
        *d = n ^ m;
    }
}

fn ctr_crypt(
    ctr: &[u8],
    cipher: &dyn Fn(&[u8], &mut [u8]),
    dst: &mut [u8],
    src: &[u8],
) {
    let mut counter = ctr.to_vec();
    let mut block = [0u8; EAX_BLOCK_SIZE];
    
    for (d, s) in dst.chunks_mut(EAX_BLOCK_SIZE).zip(src.chunks(EAX_BLOCK_SIZE)) {
        cipher(&counter, &mut block);
        for (i, (d, s)) in d.iter_mut().zip(s.iter()).enumerate() {
            *d = s ^ block[i];
        }
        
        // Increment counter
        for i in (0..EAX_BLOCK_SIZE).rev() {
            counter[i] = counter[i].wrapping_add(1);
            if counter[i] != 0 {
                break;
            }
        }
    }
}