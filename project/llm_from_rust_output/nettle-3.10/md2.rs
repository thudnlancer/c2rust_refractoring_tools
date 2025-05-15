use std::mem;

const MD2_BLOCK_SIZE: usize = 16;
const MD2_DIGEST_SIZE: usize = 16;

static S: [u8; 256] = [
    41, 46, 67, 201, 162, 216, 124, 1, 61, 54, 84, 161, 236, 240, 6, 19,
    98, 167, 5, 243, 192, 199, 115, 140, 152, 147, 43, 217, 188, 76, 130, 202,
    30, 155, 87, 60, 253, 212, 224, 22, 103, 66, 111, 24, 138, 23, 229, 18,
    190, 78, 196, 214, 218, 158, 222, 73, 160, 251, 245, 142, 187, 47, 238, 122,
    169, 104, 121, 145, 21, 178, 7, 63, 148, 194, 16, 137, 11, 34, 95, 33,
    128, 127, 93, 154, 90, 144, 50, 39, 53, 62, 204, 231, 191, 247, 151, 3,
    255, 25, 48, 179, 72, 165, 181, 209, 215, 94, 146, 42, 172, 86, 170, 198,
    79, 184, 56, 210, 150, 164, 125, 182, 118, 252, 107, 226, 156, 116, 4, 241,
    69, 157, 112, 89, 100, 113, 135, 32, 134, 91, 207, 101, 230, 45, 168, 2,
    27, 96, 37, 173, 174, 176, 185, 246, 28, 70, 97, 105, 52, 64, 126, 15,
    85, 71, 163, 35, 221, 81, 175, 58, 195, 92, 249, 206, 186, 197, 234, 38,
    44, 83, 13, 110, 133, 40, 132, 9, 211, 223, 205, 244, 65, 129, 77, 82,
    106, 220, 55, 200, 108, 193, 171, 250, 36, 225, 123, 8, 12, 189, 177, 74,
    120, 136, 149, 139, 227, 99, 232, 109, 233, 203, 213, 254, 59, 0, 29, 57,
    242, 239, 183, 14, 102, 88, 208, 228, 166, 119, 114, 248, 235, 117, 75, 10,
    49, 68, 80, 180, 143, 237, 31, 26, 219, 153, 141, 51, 159, 17, 131, 20,
];

#[derive(Clone)]
pub struct Md2Ctx {
    c: [u8; MD2_BLOCK_SIZE],
    x: [u8; MD2_BLOCK_SIZE * 3],
    index: usize,
    block: [u8; MD2_BLOCK_SIZE],
}

impl Md2Ctx {
    pub fn new() -> Self {
        Md2Ctx {
            c: [0; MD2_BLOCK_SIZE],
            x: [0; MD2_BLOCK_SIZE * 3],
            index: 0,
            block: [0; MD2_BLOCK_SIZE],
        }
    }

    fn transform(&mut self, data: &[u8; MD2_BLOCK_SIZE]) {
        // Copy data to X[16..32]
        self.x[16..32].copy_from_slice(data);

        // First pass
        let mut t = self.c[15];
        for i in 0..MD2_BLOCK_SIZE {
            self.x[32 + i] = self.x[i] ^ self.x[16 + i];
            self.c[i] ^= S[(data[i] ^ t) as usize];
            t = self.c[i];
        }

        // Second pass
        t = 0;
        for i in 0..18 {
            for j in 0..48 {
                self.x[j] ^= S[t as usize];
                t = self.x[j];
            }
            t = t.wrapping_add(i as u8);
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        if data.is_empty() {
            return;
        }

        let mut offset = 0;
        if self.index != 0 {
            let remaining = MD2_BLOCK_SIZE - self.index;
            if data.len() < remaining {
                self.block[self.index..self.index + data.len()].copy_from_slice(data);
                self.index += data.len();
                return;
            }

            self.block[self.index..].copy_from_slice(&data[..remaining]);
            self.transform(&self.block);
            offset = remaining;
        }

        while offset + MD2_BLOCK_SIZE <= data.len() {
            let block = &data[offset..offset + MD2_BLOCK_SIZE];
            self.transform(unsafe { mem::transmute(block.as_ptr()) });
            offset += MD2_BLOCK_SIZE;
        }

        self.block[..data.len() - offset].copy_from_slice(&data[offset..]);
        self.index = data.len() - offset;
    }

    pub fn digest(&mut self, output: &mut [u8]) {
        assert!(output.len() <= MD2_DIGEST_SIZE);

        let padding = (MD2_BLOCK_SIZE - self.index) as u8;
        self.block[self.index..].fill(padding);
        self.transform(&self.block);
        self.transform(&self.c);

        output.copy_from_slice(&self.x[..output.len()]);
        *self = Self::new();
    }
}

pub fn md2_init() -> Md2Ctx {
    Md2Ctx::new()
}

pub fn md2_update(ctx: &mut Md2Ctx, data: &[u8]) {
    ctx.update(data);
}

pub fn md2_digest(ctx: &mut Md2Ctx, output: &mut [u8]) {
    ctx.digest(output);
}