use aes::Aes256;
use block_modes::{BlockMode, Ctr};
use block_modes::block_padding::NoPadding;
use block_modes::BlockModeError;
use generic_array::GenericArray;
use std::convert::TryInto;

type Aes256Ctr = Ctr<Aes256, NoPadding>;

const AES_BLOCK_SIZE: usize = 16;
const DRBG_CTR_AES256_SEED_SIZE: usize = 48;
const AES256_KEY_SIZE: usize = 32;

#[derive(Clone)]
pub struct DrbgCtrAes256Ctx {
    key: Aes256,
    v: [u8; AES_BLOCK_SIZE],
}

impl DrbgCtrAes256Ctx {
    fn output(&mut self, n: usize, dst: &mut [u8]) -> Result<(), BlockModeError> {
        let mut remaining = n;
        let mut offset = 0;
        
        while remaining >= AES_BLOCK_SIZE {
            self.increment_v();
            let cipher = Aes256Ctr::new_from_slices(&self.key, &self.v)?;
            let block = cipher.encrypt_vec(&self.v);
            dst[offset..offset + AES_BLOCK_SIZE].copy_from_slice(&block[..AES_BLOCK_SIZE]);
            remaining -= AES_BLOCK_SIZE;
            offset += AES_BLOCK_SIZE;
        }

        if remaining > 0 {
            self.increment_v();
            let cipher = Aes256Ctr::new_from_slices(&self.key, &self.v)?;
            let block = cipher.encrypt_vec(&self.v);
            dst[offset..offset + remaining].copy_from_slice(&block[..remaining]);
        }

        Ok(())
    }

    fn increment_v(&mut self) {
        for i in (0..AES_BLOCK_SIZE).rev() {
            self.v[i] = self.v[i].wrapping_add(1);
            if self.v[i] != 0 {
                break;
            }
        }
    }

    fn update(&mut self, provided_data: Option<&[u8]>) -> Result<(), BlockModeError> {
        let mut tmp = [[0u8; AES_BLOCK_SIZE]; 3];
        self.output(DRBG_CTR_AES256_SEED_SIZE, &mut tmp[0])?;

        if let Some(data) = provided_data {
            for (t, d) in tmp[0].iter_mut().zip(data.iter()) {
                *t ^= *d;
            }
        }

        self.key = Aes256::new_from_slice(&tmp[0][..AES256_KEY_SIZE]).unwrap();
        self.v.copy_from_slice(&tmp[2]);

        Ok(())
    }

    pub fn init(seed_material: Option<&[u8]>) -> Result<Self, BlockModeError> {
        let zero_key = [0u8; AES256_KEY_SIZE];
        let key = Aes256::new_from_slice(&zero_key).unwrap();
        let mut ctx = Self {
            key,
            v: [0u8; AES_BLOCK_SIZE],
        };

        ctx.update(seed_material)?;
        Ok(ctx)
    }

    pub fn random(&mut self, n: usize, dst: &mut [u8]) -> Result<(), BlockModeError> {
        self.output(n, dst)?;
        self.update(None)?;
        Ok(())
    }
}