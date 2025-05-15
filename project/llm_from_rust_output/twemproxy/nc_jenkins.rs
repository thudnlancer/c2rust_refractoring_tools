use std::hash::{Hasher, Hash};

pub struct JenkinsHasher {
    a: u32,
    b: u32,
    c: u32,
}

impl JenkinsHasher {
    pub fn new() -> Self {
        JenkinsHasher { a: 0, b: 0, c: 0 }
    }

    fn mix(&mut self) {
        self.a = self.a.wrapping_sub(self.c);
        self.a ^= self.c.rotate_left(4) | self.c.rotate_right(32 - 4);
        self.c = self.c.wrapping_add(self.b);
        self.b = self.b.wrapping_sub(self.a);
        self.b ^= self.a.rotate_left(6) | self.a.rotate_right(32 - 6);
        self.a = self.a.wrapping_add(self.c);
        self.c = self.c.wrapping_sub(self.b);
        self.c ^= self.b.rotate_left(8) | self.b.rotate_right(32 - 8);
        self.b = self.b.wrapping_add(self.a);
        self.a = self.a.wrapping_sub(self.c);
        self.a ^= self.c.rotate_left(16) | self.c.rotate_right(32 - 16);
        self.c = self.c.wrapping_add(self.b);
        self.b = self.b.wrapping_sub(self.a);
        self.b ^= self.a.rotate_left(19) | self.a.rotate_right(32 - 19);
        self.a = self.a.wrapping_add(self.c);
        self.c = self.c.wrapping_sub(self.b);
        self.c ^= self.b.rotate_left(4) | self.b.rotate_right(32 - 4);
        self.b = self.b.wrapping_add(self.a);
    }

    fn finalize(&mut self) -> u32 {
        self.c ^= self.b;
        self.c = self.c.wrapping_sub(self.b.rotate_left(14) | self.b.rotate_right(32 - 14));
        self.a ^= self.c;
        self.a = self.a.wrapping_sub(self.c.rotate_left(11) | self.c.rotate_right(32 - 11));
        self.b ^= self.a;
        self.b = self.b.wrapping_sub(self.a.rotate_left(25) | self.a.rotate_right(32 - 25));
        self.c ^= self.b;
        self.c = self.c.wrapping_sub(self.b.rotate_left(16) | self.b.rotate_right(32 - 16));
        self.a ^= self.c;
        self.a = self.a.wrapping_sub(self.c.rotate_left(4) | self.c.rotate_right(32 - 4));
        self.b ^= self.a;
        self.b = self.b.wrapping_sub(self.a.rotate_left(14) | self.a.rotate_right(32 - 14));
        self.c ^= self.b;
        self.c.wrapping_sub(self.b.rotate_left(24) | self.b.rotate_right(32 - 24))
    }
}

impl Hasher for JenkinsHasher {
    fn write(&mut self, bytes: &[u8]) {
        let mut length = bytes.len();
        self.c = 0xdeadbeefu32.wrapping_add(length as u32).wrapping_add(13);
        self.b = self.c;
        self.a = self.b;

        let mut chunks = bytes.chunks_exact(12);
        for chunk in chunks.by_ref() {
            let mut k = [0u32; 3];
            for (i, word) in chunk.chunks_exact(4).enumerate().take(3) {
                k[i] = u32::from_le_bytes([word[0], word[1], word[2], word[3]]);
            }
            self.a = self.a.wrapping_add(k[0]);
            self.b = self.b.wrapping_add(k[1]);
            self.c = self.c.wrapping_add(k[2]);
            self.mix();
            length -= 12;
        }

        let remainder = chunks.remainder();
        match remainder.len() {
            12 => {
                self.c = self.c.wrapping_add(u32::from_le_bytes([
                    remainder[8], remainder[9], remainder[10], remainder[11],
                ]));
                self.b = self.b.wrapping_add(u32::from_le_bytes([
                    remainder[4], remainder[5], remainder[6], remainder[7],
                ]));
                self.a = self.a.wrapping_add(u32::from_le_bytes([
                    remainder[0], remainder[1], remainder[2], remainder[3],
                ]));
            }
            11 => {
                self.c = self.c.wrapping_add(u32::from_le_bytes([
                    remainder[8], remainder[9], remainder[10], 0,
                ]));
                self.b = self.b.wrapping_add(u32::from_le_bytes([
                    remainder[4], remainder[5], remainder[6], remainder[7],
                ]));
                self.a = self.a.wrapping_add(u32::from_le_bytes([
                    remainder[0], remainder[1], remainder[2], remainder[3],
                ]));
            }
            10 => {
                self.c = self.c.wrapping_add(u32::from_le_bytes([
                    remainder[8], remainder[9], 0, 0,
                ]));
                self.b = self.b.wrapping_add(u32::from_le_bytes([
                    remainder[4], remainder[5], remainder[6], remainder[7],
                ]));
                self.a = self.a.wrapping_add(u32::from_le_bytes([
                    remainder[0], remainder[1], remainder[2], remainder[3],
                ]));
            }
            9 => {
                self.c = self.c.wrapping_add(u32::from_le_bytes([remainder[8], 0, 0, 0]));
                self.b = self.b.wrapping_add(u32::from_le_bytes([
                    remainder[4], remainder[5], remainder[6], remainder[7],
                ]));
                self.a = self.a.wrapping_add(u32::from_le_bytes([
                    remainder[0], remainder[1], remainder[2], remainder[3],
                ]));
            }
            8 => {
                self.b = self.b.wrapping_add(u32::from_le_bytes([
                    remainder[4], remainder[5], remainder[6], remainder[7],
                ]));
                self.a = self.a.wrapping_add(u32::from_le_bytes([
                    remainder[0], remainder[1], remainder[2], remainder[3],
                ]));
            }
            7 => {
                self.b = self.b.wrapping_add(u32::from_le_bytes([
                    remainder[4], remainder[5], remainder[6], 0,
                ]));
                self.a = self.a.wrapping_add(u32::from_le_bytes([
                    remainder[0], remainder[1], remainder[2], remainder[3],
                ]));
            }
            6 => {
                self.b = self.b.wrapping_add(u32::from_le_bytes([
                    remainder[4], remainder[5], 0, 0,
                ]));
                self.a = self.a.wrapping_add(u32::from_le_bytes([
                    remainder[0], remainder[1], remainder[2], remainder[3],
                ]));
            }
            5 => {
                self.b = self.b.wrapping_add(u32::from_le_bytes([remainder[4], 0, 0, 0]));
                self.a = self.a.wrapping_add(u32::from_le_bytes([
                    remainder[0], remainder[1], remainder[2], remainder[3],
                ]));
            }
            4 => {
                self.a = self.a.wrapping_add(u32::from_le_bytes([
                    remainder[0], remainder[1], remainder[2], remainder[3],
                ]));
            }
            3 => {
                self.a = self.a.wrapping_add(u32::from_le_bytes([
                    remainder[0], remainder[1], remainder[2], 0,
                ]));
            }
            2 => {
                self.a = self.a.wrapping_add(u32::from_le_bytes([remainder[0], remainder[1], 0, 0]));
            }
            1 => {
                self.a = self.a.wrapping_add(u32::from_le_bytes([remainder[0], 0, 0, 0]));
            }
            _ => {}
        }
    }

    fn finish(&self) -> u64 {
        let mut hasher = JenkinsHasher {
            a: self.a,
            b: self.b,
            c: self.c,
        };
        hasher.finalize() as u64
    }
}

pub fn hash_jenkins<T: Hash>(value: &T) -> u32 {
    let mut hasher = JenkinsHasher::new();
    value.hash(&mut hasher);
    hasher.finalize()
}