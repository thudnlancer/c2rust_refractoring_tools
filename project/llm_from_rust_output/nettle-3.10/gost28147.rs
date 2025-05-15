use std::convert::TryInto;

pub type Uint32 = u32;

#[derive(Copy, Clone)]
pub struct Gost28147Param {
    pub sbox: [[Uint32; 256]; 4],
}

pub const GOST28147_PARAM_TEST_3411: Gost28147Param = {
    let sbox = [
        [
            0x72000, 0x75000, 0x74800, 0x71000, 0x76800, 0x74000, 0x70000, 0x77000,
            0x73000, 0x75800, 0x70800, 0x76000, 0x73800, 0x77800, 0x72800, 0x71800,
            // ... (完整数组内容)
        ],
        // ... (其他3个sbox数组)
    ];
    Gost28147Param { sbox }
};

pub const GOST28147_PARAM_CRYPTOPRO_3411: Gost28147Param = {
    let sbox = [
        [
            0x2d000, 0x2a000, 0x2a800, 0x2b000, 0x2c000, 0x28800, 0x29800, 0x2b800,
            0x2e800, 0x2e000, 0x2f000, 0x28000, 0x2c800, 0x29000, 0x2d800, 0x2f800,
            // ... (完整数组内容)
        ],
        // ... (其他3个sbox数组)
    ];
    Gost28147Param { sbox }
};

pub fn gost28147_encrypt_block(
    key: &[Uint32; 8],
    sbox: &Gost28147Param,
    input: &[Uint32; 2],
    output: &mut [Uint32; 2],
) {
    let mut l = input[1];
    let mut r = input[0];

    macro_rules! round {
        ($key_idx:expr) => {
            let tmp = key[$key_idx].wrapping_add(r);
            l ^= sbox.sbox[0][(tmp & 0xff) as usize]
                ^ sbox.sbox[1][(tmp >> 8 & 0xff) as usize]
                ^ sbox.sbox[2][(tmp >> 16 & 0xff) as usize]
                ^ sbox.sbox[3][(tmp >> 24) as usize];
            
            let tmp = key[$key_idx + 1].wrapping_add(l);
            r ^= sbox.sbox[0][(tmp & 0xff) as usize]
                ^ sbox.sbox[1][(tmp >> 8 & 0xff) as usize]
                ^ sbox.sbox[2][(tmp >> 16 & 0xff) as usize]
                ^ sbox.sbox[3][(tmp >> 24) as usize];
        };
    }

    // 前8轮
    round!(0);
    round!(2);
    round!(4);
    round!(6);
    
    // 中间8轮
    round!(0);
    round!(2);
    round!(4);
    round!(6);
    
    // 后8轮
    round!(0);
    round!(2);
    round!(4);
    round!(6);
    
    // 最后8轮(反向)
    let tmp = key[7].wrapping_add(r);
    l ^= sbox.sbox[0][(tmp & 0xff) as usize]
        ^ sbox.sbox[1][(tmp >> 8 & 0xff) as usize]
        ^ sbox.sbox[2][(tmp >> 16 & 0xff) as usize]
        ^ sbox.sbox[3][(tmp >> 24) as usize];
    
    let tmp = key[6].wrapping_add(l);
    r ^= sbox.sbox[0][(tmp & 0xff) as usize]
        ^ sbox.sbox[1][(tmp >> 8 & 0xff) as usize]
        ^ sbox.sbox[2][(tmp >> 16 & 0xff) as usize]
        ^ sbox.sbox[3][(tmp >> 24) as usize];
    
    // ... (剩余6轮反向操作)

    output[0] = r;
    output[1] = l;
}