/* nettle-openssl.rs

   Glue that's used only by the benchmark, and subject to change.

   Copyright (C) 2002, 2017 Niels Möller
   Copyright (C) 2017 Red Hat, Inc.

   This file is part of GNU Nettle.

   GNU Nettle is free software: you can redistribute it and/or
   modify it under the terms of either:

     * the GNU Lesser General Public License as published by the Free
       Software Foundation; either version 3 of the License, or (at your
       option) any later version.

   or

     * the GNU General Public License as published by the Free
       Software Foundation; either version 2 of the License, or (at your
       option) any later version.

   or both in parallel, as here.

   GNU Nettle is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   General Public License for more details.

   You should have received copies of the GNU General Public License and
   the GNU Lesser General Public License along with this program.  If
   not, see http://www.gnu.org/licenses/.
*/

#[cfg(feature = "openssl")]
mod openssl_glue {
    use openssl::symm::{Cipher, Crypter, Mode};
    use openssl::hash::{Hasher, MessageDigest};
    use std::ptr;

    pub struct OpensslCipherCtx {
        crypter: Crypter,
    }

    pub struct OpensslHashCtx {
        hasher: Hasher,
    }

    impl OpensslCipherCtx {
        pub fn new(cipher: Cipher, key: &[u8], mode: Mode) -> Result<Self, openssl::error::ErrorStack> {
            let mut crypter = Crypter::new(cipher, mode, key, None)?;
            crypter.pad(false);
            Ok(Self { crypter })
        }

        pub fn set_nonce(&mut self, nonce: &[u8]) -> Result<(), openssl::error::ErrorStack> {
            self.crypter.init(None, None, Some(nonce))?;
            Ok(())
        }

        pub fn encrypt(&mut self, data: &[u8], out: &mut [u8]) -> Result<usize, openssl::error::ErrorStack> {
            let mut len = 0;
            self.crypter.update(data, &mut out[len..])?;
            len += data.len();
            Ok(len)
        }

        pub fn decrypt(&mut self, data: &[u8], out: &mut [u8]) -> Result<usize, openssl::error::ErrorStack> {
            let mut len = 0;
            self.crypter.update(data, &mut out[len..])?;
            len += data.len();
            Ok(len)
        }

        pub fn gcm_digest(&mut self, tag: &mut [u8]) -> Result<(), openssl::error::ErrorStack> {
            self.crypter.get_tag(tag)?;
            Ok(())
        }
    }

    impl OpensslHashCtx {
        pub fn new(md: MessageDigest) -> Result<Self, openssl::error::ErrorStack> {
            let hasher = Hasher::new(md)?;
            Ok(Self { hasher })
        }

        pub fn update(&mut self, data: &[u8]) -> Result<(), openssl::error::ErrorStack> {
            self.hasher.update(data)?;
            Ok(())
        }

        pub fn digest(&mut self, out: &mut [u8]) -> Result<(), openssl::error::ErrorStack> {
            self.hasher.finish(out)?;
            Ok(())
        }
    }

    pub fn openssl_aes128_set_encrypt_key(ctx: &mut OpensslCipherCtx, key: &[u8]) -> Result<(), openssl::error::ErrorStack> {
        *ctx = OpensslCipherCtx::new(Cipher::aes_128_ecb(), key, Mode::Encrypt)?;
        Ok(())
    }

    pub fn openssl_aes128_set_decrypt_key(ctx: &mut OpensslCipherCtx, key: &[u8]) -> Result<(), openssl::error::ErrorStack> {
        *ctx = OpensslCipherCtx::new(Cipher::aes_128_ecb(), key, Mode::Decrypt)?;
        Ok(())
    }

    pub fn openssl_aes192_set_encrypt_key(ctx: &mut OpensslCipherCtx, key: &[u8]) -> Result<(), openssl::error::ErrorStack> {
        *ctx = OpensslCipherCtx::new(Cipher::aes_192_ecb(), key, Mode::Encrypt)?;
        Ok(())
    }

    pub fn openssl_aes192_set_decrypt_key(ctx: &mut OpensslCipherCtx, key: &[u8]) -> Result<(), openssl::error::ErrorStack> {
        *ctx = OpensslCipherCtx::new(Cipher::aes_192_ecb(), key, Mode::Decrypt)?;
        Ok(())
    }

    pub fn openssl_aes256_set_encrypt_key(ctx: &mut OpensslCipherCtx, key: &[u8]) -> Result<(), openssl::error::ErrorStack> {
        *ctx = OpensslCipherCtx::new(Cipher::aes_256_ecb(), key, Mode::Encrypt)?;
        Ok(())
    }

    pub fn openssl_aes256_set_decrypt_key(ctx: &mut OpensslCipherCtx, key: &[u8]) -> Result<(), openssl::error::ErrorStack> {
        *ctx = OpensslCipherCtx::new(Cipher::aes_256_ecb(), key, Mode::Decrypt)?;
        Ok(())
    }

    pub fn openssl_gcm_aes128_set_encrypt_key(ctx: &mut OpensslCipherCtx, key: &[u8]) -> Result<(), openssl::error::ErrorStack> {
        *ctx = OpensslCipherCtx::new(Cipher::aes_128_gcm(), key, Mode::Encrypt)?;
        Ok(())
    }

    pub fn openssl_gcm_aes128_set_decrypt_key(ctx: &mut OpensslCipherCtx, key: &[u8]) -> Result<(), openssl::error::ErrorStack> {
        *ctx = OpensslCipherCtx::new(Cipher::aes_128_gcm(), key, Mode::Decrypt)?;
        Ok(())
    }

    pub fn openssl_gcm_aes192_set_encrypt_key(ctx: &mut OpensslCipherCtx, key: &[u8]) -> Result<(), openssl::error::ErrorStack> {
        *ctx = OpensslCipherCtx::new(Cipher::aes_192_gcm(), key, Mode::Encrypt)?;
        Ok(())
    }

    pub fn openssl_gcm_aes192_set_decrypt_key(ctx: &mut OpensslCipherCtx, key: &[u8]) -> Result<(), openssl::error::ErrorStack> {
        *ctx = OpensslCipherCtx::new(Cipher::aes_192_gcm(), key, Mode::Decrypt)?;
        Ok(())
    }

    pub fn openssl_gcm_aes256_set_encrypt_key(ctx: &mut OpensslCipherCtx, key: &[u8]) -> Result<(), openssl::error::ErrorStack> {
        *ctx = OpensslCipherCtx::new(Cipher::aes_256_gcm(), key, Mode::Encrypt)?;
        Ok(())
    }

    pub fn openssl_gcm_aes256_set_decrypt_key(ctx: &mut OpensslCipherCtx, key: &[u8]) -> Result<(), openssl::error::ErrorStack> {
        *ctx = OpensslCipherCtx::new(Cipher::aes_256_gcm(), key, Mode::Decrypt)?;
        Ok(())
    }

    pub fn openssl_md5_init(ctx: &mut OpensslHashCtx) -> Result<(), openssl::error::ErrorStack> {
        *ctx = OpensslHashCtx::new(MessageDigest::md5())?;
        Ok(())
    }

    pub fn openssl_md5_digest(ctx: &mut OpensslHashCtx, out: &mut [u8]) -> Result<(), openssl::error::ErrorStack> {
        ctx.digest(out)
    }

    pub fn openssl_sha1_init(ctx: &mut OpensslHashCtx) -> Result<(), openssl::error::ErrorStack> {
        *ctx = OpensslHashCtx::new(MessageDigest::sha1())?;
        Ok(())
    }

    pub fn openssl_sha1_digest(ctx: &mut OpensslHashCtx, out: &mut [u8]) -> Result<(), openssl::error::ErrorStack> {
        ctx.digest(out)
    }
}