use std::convert::TryInto;

pub type size_t = usize;
pub type uint8_t = u8;

#[derive(Copy, Clone)]
pub struct ArcfourCtx {
    pub S: [uint8_t; 256],
    pub i: uint8_t,
    pub j: uint8_t,
}

impl ArcfourCtx {
    pub fn set_key(&mut self, key: &[uint8_t]) {
        assert!(
            key.len() >= 1 && key.len() <= 256,
            "Key length must be between 1 and 256 bytes"
        );

        for (i, byte) in self.S.iter_mut().enumerate() {
            *byte = i as uint8_t;
        }

        let mut j = 0;
        let mut k = 0;
        for i in 0..256 {
            j = j.wrapping_add(self.S[i].wrapping_add(key[k]));
            self.S.swap(i, j as usize);
            k = (k + 1) % key.len();
        }

        self.i = 0;
        self.j = 0;
    }

    pub fn crypt(&mut self, src: &[uint8_t], dst: &mut [uint8_t]) {
        assert_eq!(src.len(), dst.len(), "Source and destination lengths must match");

        let mut i = self.i;
        let mut j = self.j;

        for (s, d) in src.iter().zip(dst.iter_mut()) {
            i = i.wrapping_add(1);
            let si = self.S[i as usize] as usize;
            j = j.wrapping_add(si as u8);
            let sj = self.S[j as usize] as usize;
            self.S.swap(i as usize, j as usize);
            *d = s ^ self.S[(si + sj) % 256];
        }

        self.i = i;
        self.j = j;
    }
}

pub fn arc_four128_set_key(ctx: &mut ArcfourCtx, key: &[uint8_t]) {
    assert_eq!(key.len(), 16, "ARC4-128 requires 16-byte key");
    ctx.set_key(key);
}