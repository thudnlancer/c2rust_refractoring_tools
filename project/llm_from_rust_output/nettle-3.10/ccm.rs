use std::mem;

const CCM_MIN_NONCE_SIZE: usize = 7;
const CCM_MAX_NONCE_SIZE: usize = 14;
const CCM_BLOCK_SIZE: usize = 16;

#[derive(Copy, Clone)]
#[repr(C)]
pub union NettleBlock16 {
    pub b: [u8; 16],
    pub w: [usize; 2],
    pub u64_0: [u64; 2],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CcmCtx {
    pub ctr: NettleBlock16,
    pub tag: NettleBlock16,
    pub blength: u32,
}

type NettleCipherFunc = fn(key: &[u8], length: usize, dst: &mut [u8], src: &[u8]);

impl CcmCtx {
    fn pad(&mut self, cipher: &[u8], f: NettleCipherFunc) {
        if self.blength != 0 {
            f(cipher, CCM_BLOCK_SIZE, &mut self.tag.b, &self.tag.b);
        }
        self.blength = 0;
    }

    fn build_iv(&mut self, noncelen: usize, nonce: &[u8], flags: u8, count: usize) {
        assert!(noncelen >= CCM_MIN_NONCE_SIZE && noncelen <= CCM_MAX_NONCE_SIZE);
        
        self.tag.b[0] = (flags | ((CCM_BLOCK_SIZE - 1) - noncelen - 1) as u8) & 0x7;
        self.tag.b[1..=noncelen].copy_from_slice(&nonce[..noncelen]);
        
        let mut i = (CCM_BLOCK_SIZE - 1) as u32;
        let mut remaining_count = count;
        while i >= (1 + noncelen) as u32 {
            self.tag.b[i as usize] = (remaining_count & 0xff) as u8;
            remaining_count >>= 8;
            i -= 1;
        }
        assert!(remaining_count == 0);
    }

    pub fn set_nonce(
        &mut self,
        cipher: &[u8],
        f: NettleCipherFunc,
        length: usize,
        nonce: &[u8],
        authlen: usize,
        msglen: usize,
        taglen: usize,
    ) {
        self.blength = 0;
        
        let flags = ((taglen - 2) << 2 & 0x38) as u8;
        self.build_iv(length, nonce, flags, msglen);
        
        self.build_iv(length, nonce, 0, 1);
        
        if authlen == 0 {
            f(cipher, CCM_BLOCK_SIZE, &mut self.tag.b, &self.tag.b);
            return;
        }
        
        self.tag.b[0] |= 0x40;
        f(cipher, CCM_BLOCK_SIZE, &mut self.tag.b, &self.tag.b);
        
        if authlen >= (1 << 32) {
            self.xor_auth_len(authlen, 0xff);
        } else if authlen >= ((1 << 16) - (1 << 8)) {
            self.xor_auth_len(authlen, 0xfe);
        }
        
        let fresh12 = self.blength;
        self.blength += 1;
        self.tag.b[fresh12 as usize] ^= (authlen >> 8 & 0xff) as u8;
        
        let fresh13 = self.blength;
        self.blength += 1;
        self.tag.b[fresh13 as usize] ^= (authlen & 0xff) as u8;
    }

    fn xor_auth_len(&mut self, authlen: usize, marker: u8) {
        let fresh0 = self.blength;
        self.blength += 1;
        self.tag.b[fresh0 as usize] ^= marker;
        
        let fresh1 = self.blength;
        self.blength += 1;
        self.tag.b[fresh1 as usize] ^= marker.wrapping_sub(1);
        
        for i in 0..6 {
            let fresh = self.blength;
            self.blength += 1;
            self.tag.b[fresh as usize] ^= (authlen >> (56 - 8 * i) & 0xff) as u8;
        }
    }

    pub fn update(&mut self, cipher: &[u8], f: NettleCipherFunc, length: usize, data: &[u8]) {
        if self.blength as usize + length < CCM_BLOCK_SIZE {
            xor_bytes(
                &mut self.tag.b[self.blength as usize..],
                data,
                length,
            );
            self.blength += length as u32;
            return;
        }
        
        if self.blength != 0 {
            let remaining = CCM_BLOCK_SIZE - self.blength as usize;
            xor_bytes(
                &mut self.tag.b[self.blength as usize..],
                &data[..remaining],
                remaining,
            );
            
            f(cipher, CCM_BLOCK_SIZE, &mut self.tag.b, &self.tag.b);
            self.blength = 0;
            
            self.update(cipher, f, length - remaining, &data[remaining..]);
            return;
        }
        
        let mut chunks = data.chunks_exact(CCM_BLOCK_SIZE);
        for chunk in chunks.by_ref() {
            xor_bytes(&mut self.tag.b, chunk, CCM_BLOCK_SIZE);
            f(cipher, CCM_BLOCK_SIZE, &mut self.tag.b, &self.tag.b);
        }
        
        let remainder = chunks.remainder();
        if !remainder.is_empty() {
            xor_bytes(&mut self.tag.b, remainder, remainder.len());
            self.blength = remainder.len() as u32;
        }
    }

    pub fn encrypt(&mut self, cipher: &[u8], f: NettleCipherFunc, length: usize, dst: &mut [u8], src: &[u8]) {
        self.pad(cipher, f);
        self.update(cipher, f, length, src);
        ctr_crypt(cipher, f, &mut self.ctr.b, length, dst, src);
    }

    pub fn decrypt(&mut self, cipher: &[u8], f: NettleCipherFunc, length: usize, dst: &mut [u8], src: &[u8]) {
        ctr_crypt(cipher, f, &mut self.ctr.b, length, dst, src);
        self.pad(cipher, f);
        self.update(cipher, f, length, dst);
    }

    pub fn digest(&mut self, cipher: &[u8], f: NettleCipherFunc, length: usize, digest: &mut [u8]) {
        assert!(length <= CCM_BLOCK_SIZE);
        
        let i = CCM_BLOCK_SIZE - ((self.ctr.b[0] & 0x7) + 1) as usize;
        self.ctr.b[i..CCM_BLOCK_SIZE].fill(0);
        
        self.pad(cipher, f);
        ctr_crypt(cipher, f, &mut self.ctr.b, length, digest, &self.tag.b);
    }
}

fn xor_bytes(dst: &mut [u8], src: &[u8], len: usize) {
    for (d, s) in dst.iter_mut().zip(src.iter()).take(len) {
        *d ^= *s;
    }
}

fn ctr_crypt(
    cipher: &[u8],
    f: NettleCipherFunc,
    ctr: &mut [u8],
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    let mut counter = *unsafe { mem::transmute::<_, &[u8; CCM_BLOCK_SIZE]>(ctr) };
    let mut keystream = [0u8; CCM_BLOCK_SIZE];
    
    for (i, chunk) in src.chunks(CCM_BLOCK_SIZE).enumerate() {
        f(cipher, CCM_BLOCK_SIZE, &mut keystream, &counter);
        
        for j in 0..chunk.len() {
            dst[i * CCM_BLOCK_SIZE + j] = chunk[j] ^ keystream[j];
        }
        
        increment_counter(&mut counter);
    }
    
    ctr.copy_from_slice(&counter);
}

fn increment_counter(counter: &mut [u8; CCM_BLOCK_SIZE]) {
    for i in (0..CCM_BLOCK_SIZE).rev() {
        counter[i] = counter[i].wrapping_add(1);
        if counter[i] != 0 {
            break;
        }
    }
}

pub fn encrypt_message(
    cipher: &[u8],
    f: NettleCipherFunc,
    nlength: usize,
    nonce: &[u8],
    alength: usize,
    adata: &[u8],
    tlength: usize,
    clength: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    assert!(clength >= tlength);
    
    let mut ctx = CcmCtx {
        ctr: NettleBlock16 { b: [0; 16] },
        tag: NettleBlock16 { b: [0; 16] },
        blength: 0,
    };
    
    let (message, tag) = dst.split_at_mut(clength - tlength);
    
    ctx.set_nonce(cipher, f, nlength, nonce, alength, clength - tlength, tlength);
    ctx.update(cipher, f, alength, adata);
    ctx.encrypt(cipher, f, clength - tlength, message, src);
    ctx.digest(cipher, f, tlength, tag);
}

pub fn decrypt_message(
    cipher: &[u8],
    f: NettleCipherFunc,
    nlength: usize,
    nonce: &[u8],
    alength: usize,
    adata: &[u8],
    tlength: usize,
    mlength: usize,
    dst: &mut [u8],
    src: &[u8],
) -> bool {
    let mut ctx = CcmCtx {
        ctr: NettleBlock16 { b: [0; 16] },
        tag: NettleBlock16 { b: [0; 16] },
        blength: 0,
    };
    
    let mut tag = [0u8; 16];
    
    ctx.set_nonce(cipher, f, nlength, nonce, alength, mlength, tlength);
    ctx.update(cipher, f, alength, adata);
    ctx.decrypt(cipher, f, mlength, dst, src);
    ctx.digest(cipher, f, tlength, &mut tag);
    
    tag[..tlength] == src[mlength..mlength + tlength]
}