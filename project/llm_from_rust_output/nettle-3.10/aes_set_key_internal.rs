use std::convert::TryInto;

pub type Uint8 = u8;
pub type Uint32 = u32;

#[derive(Copy, Clone)]
pub struct AesTable {
    pub sbox: [Uint8; 256],
    pub table: [[Uint32; 256]; 4],
}

static RCON: [Uint8; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36];

pub fn aes_set_key(
    nr: Uint32,
    nk: Uint32,
    subkeys: &mut [Uint32],
    key: &[Uint8],
    encrypt_table: &AesTable,
) {
    assert!(nk != 0, "nk must not be zero");

    let lastkey = (16 / 4) * (nr + 1);
    let mut rcon_index = 0;

    for i in 0..nk {
        let pos = (i * 4) as usize;
        subkeys[i as usize] = Uint32::from(key[pos + 3]) << 24
            | Uint32::from(key[pos + 2]) << 16
            | Uint32::from(key[pos + 1]) << 8
            | Uint32::from(key[pos]);
    }

    for i in nk..lastkey {
        let mut t = subkeys[(i - 1) as usize];

        if i % nk == 0 {
            let rotated = t.rotate_left(8);
            t = (Uint32::from(encrypt_table.sbox[(rotated & 0xFF) as usize])
                | Uint32::from(encrypt_table.sbox[((rotated >> 8) & 0xFF) as usize]) << 8
                | Uint32::from(encrypt_table.sbox[((rotated >> 16) & 0xFF) as usize]) << 16
                | Uint32::from(encrypt_table.sbox[((rotated >> 24) & 0xFF) as usize]) << 24)
                ^ Uint32::from(RCON[rcon_index]);
            rcon_index += 1;
        } else if nk > 6 && i % nk == 4 {
            t = Uint32::from(encrypt_table.sbox[(t & 0xFF) as usize])
                | Uint32::from(encrypt_table.sbox[((t >> 8) & 0xFF) as usize]) << 8
                | Uint32::from(encrypt_table.sbox[((t >> 16) & 0xFF) as usize]) << 16
                | Uint32::from(encrypt_table.sbox[((t >> 24) & 0xFF) as usize]) << 24;
        }

        subkeys[i as usize] = subkeys[(i - nk) as usize] ^ t;
    }
}