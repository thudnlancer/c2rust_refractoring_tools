use std::fmt;
use std::mem;
use std::ptr;
use std::slice;
use std::str;

use base64;
use md4::{Digest, Md4};
use des::Des;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;

type DesCbc = Cbc<Des, Pkcs7>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NtlmState {
    None,
    Type1,
    Type2,
    Type3,
    Last,
}

#[derive(Debug, Clone)]
pub struct NtlmData {
    pub state: NtlmState,
    pub nonce: [u8; 8],
}

impl NtlmData {
    pub fn new() -> Self {
        NtlmData {
            state: NtlmState::None,
            nonce: [0; 8],
        }
    }
}

const NTLMFLAG_NEGOTIATE_OEM: u32 = 1 << 1;
const NTLMFLAG_NEGOTIATE_NTLM_KEY: u32 = 1 << 9;

pub fn ntlm_input(ntlm: &mut NtlmData, header: &str) -> bool {
    if !header.starts_with("NTLM") {
        return false;
    }

    let header = &header[4..];
    let header = header.trim_start();

    if !header.is_empty() {
        let mut buffer = [0u8; 48];
        let size = match base64::decode_config_slice(header, base64::STANDARD, &mut buffer) {
            Ok(size) => size,
            Err(_) => return false,
        };

        ntlm.state = NtlmState::Type2;

        if size >= buffer.len() {
            ntlm.nonce.copy_from_slice(&buffer[24..32]);
        }
    } else {
        match ntlm.state {
            NtlmState::Last => {}
            NtlmState::Type3 => {
                ntlm.state = NtlmState::None;
                return false;
            }
            _ if ntlm.state >= NtlmState::Type1 => {
                return false;
            }
            _ => {}
        }

        ntlm.state = NtlmState::Type1;
    }

    true
}

fn setup_des_key(key_56: &[u8]) -> [u8; 8] {
    let mut key = [0u8; 8];
    key[0] = key_56[0];
    key[1] = ((key_56[0] << 7) & 0xFF | (key_56[1] >> 1);
    key[2] = ((key_56[1] << 6) & 0xFF | (key_56[2] >> 2);
    key[3] = ((key_56[2] << 5) & 0xFF | (key_56[3] >> 3);
    key[4] = ((key_56[3] << 4) & 0xFF | (key_56[4] >> 4);
    key[5] = ((key_56[4] << 3) & 0xFF | (key_56[5] >> 5);
    key[6] = ((key_56[5] << 2) & 0xFF | (key_56[6] >> 6);
    key[7] = (key_56[6] << 1) & 0xFF;
    key
}

fn calc_resp(keys: &[u8], plaintext: &[u8], results: &mut [u8]) {
    let key1 = setup_des_key(&keys[0..7]);
    let cipher1 = Des::new_from_slice(&key1).unwrap();
    let mut block1 = plaintext.to_vec();
    cipher1.encrypt_block(&mut block1);
    results[0..8].copy_from_slice(&block1);

    let key2 = setup_des_key(&keys[7..14]);
    let cipher2 = Des::new_from_slice(&key2).unwrap();
    let mut block2 = plaintext.to_vec();
    cipher2.encrypt_block(&mut block2);
    results[8..16].copy_from_slice(&block2);

    let key3 = setup_des_key(&keys[14..21]);
    let cipher3 = Des::new_from_slice(&key3).unwrap();
    let mut block3 = plaintext.to_vec();
    cipher3.encrypt_block(&mut block3);
    results[16..24].copy_from_slice(&block3);
}

fn mkhash(password: &str, nonce: &[u8], lmresp: &mut [u8], ntresp: &mut [u8]) {
    let magic = [0x4B, 0x47, 0x53, 0x21, 0x40, 0x23, 0x24, 0x25];
    let mut pw = [0u8; 14];
    let len = password.len().min(14);
    pw[..len].copy_from_slice(&password.as_bytes()[..len].to_ascii_uppercase());

    let mut lmbuffer = [0u8; 21];
    let key1 = setup_des_key(&pw[0..7]);
    let cipher1 = Des::new_from_slice(&key1).unwrap();
    let mut block1 = magic.to_vec();
    cipher1.encrypt_block(&mut block1);
    lmbuffer[0..8].copy_from_slice(&block1);

    let key2 = setup_des_key(&pw[7..14]);
    let cipher2 = Des::new_from_slice(&key2).unwrap();
    let mut block2 = magic.to_vec();
    cipher2.encrypt_block(&mut block2);
    lmbuffer[8..16].copy_from_slice(&block2);

    calc_resp(&lmbuffer, nonce, lmresp);

    let mut pw4 = [0u8; 64];
    let len = password.len().min(32);
    for i in 0..len {
        pw4[2 * i] = password.as_bytes()[i];
    }

    let mut hasher = Md4::new();
    hasher.update(&pw4[..2 * len]);
    let ntbuffer = hasher.finalize();

    let mut ntbuffer_full = [0u8; 21];
    ntbuffer_full[..16].copy_from_slice(&ntbuffer);
    calc_resp(&ntbuffer_full, nonce, ntresp);
}

pub fn ntlm_output(ntlm: &mut NtlmData, user: &str, passwd: &str) -> Option<String> {
    let domain = "";
    let host = "";
    let domlen = domain.len();
    let hostlen = host.len();

    let user = user.unwrap_or("");
    let passwd = passwd.unwrap_or("");

    match ntlm.state {
        NtlState::Type1 | NtlState::None | NtlState::Last => {
            let hostoff = 32;
            let domoff = hostoff + hostlen;

            let flags = NTLMFLAG_NEGOTIATE_OEM | NTLMFLAG_NEGOTIATE_NTLM_KEY;
            let mut ntlmbuf = Vec::with_capacity(256);

            ntlmbuf.extend_from_slice(b"NTLMSSP\0");
            ntlmbuf.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]);
            ntlmbuf.extend_from_slice(&[
                (flags & 0xFF) as u8,
                ((flags >> 8) & 0xFF) as u8,
                ((flags >> 16) & 0xFF) as u8,
                ((flags >> 24) & 0xFF) as u8,
            ]);
            ntlmbuf.extend_from_slice(&[
                (domlen & 0xFF) as u8,
                ((domlen >> 8) & 0xFF) as u8,
                (domlen & 0xFF) as u8,
                ((domlen >> 8) & 0xFF) as u8,
                (domoff & 0xFF) as u8,
                ((domoff >> 8) & 0xFF) as u8,
                0, 0,
            ]);
            ntlmbuf.extend_from_slice(&[
                (hostlen & 0xFF) as u8,
                ((hostlen >> 8) & 0xFF) as u8,
                (hostlen & 0xFF) as u8,
                ((hostlen >> 8) & 0xFF) as u8,
                (hostoff & 0xFF) as u8,
                ((hostoff >> 8) & 0xFF) as u8,
                0, 0,
            ]);
            ntlmbuf.extend_from_slice(host.as_bytes());
            ntlmbuf.extend_from_slice(domain.as_bytes());

            let size = 32 + hostlen + domlen;
            let mut output = Vec::with_capacity(5 + base64::encoded_len(size));
            output.extend_from_slice(b"NTLM ");
            base64::encode_config_slice(&ntlmbuf[..size], base64::STANDARD, &mut output[5..]);
            Some(unsafe { String::from_utf8_unchecked(output) })
        }
        NtlState::Type2 => {
            let mut lmresp = [0u8; 0x18];
            let mut ntresp = [0u8; 0x18];

            let (domain, user) = if let Some(pos) = user.find(|c| c == '\\' || c == '/') {
                (&user[..pos], &user[pos + 1..])
            } else {
                ("", user)
            };
            let domlen = domain.len();
            let userlen = user.len();

            mkhash(passwd, &ntlm.nonce, &mut lmresp, &mut ntresp);

            let domoff = 64;
            let useroff = domoff + domlen;
            let hostoff = useroff + userlen;
            let lmrespoff = hostoff + hostlen;
            let ntrespoff = lmrespoff + 0x18;

            let mut ntlmbuf = Vec::with_capacity(256);
            ntlmbuf.extend_from_slice(b"NTLMSSP\0");
            ntlmbuf.extend_from_slice(&[0x03, 0x00, 0x00, 0x00]);
            ntlmbuf.extend_from_slice(&[0x18, 0x00, 0x18, 0x00]);
            ntlmbuf.extend_from_slice(&[
                (lmrespoff & 0xFF) as u8,
                ((lmrespoff >> 8) & 0xFF) as u8,
                0, 0,
            ]);
            ntlmbuf.extend_from_slice(&[0x18, 0x00, 0x18, 0x00]);
            ntlmbuf.extend_from_slice(&[
                (ntrespoff & 0xFF) as u8,
                ((ntrespoff >> 8) & 0xFF) as u8,
                0, 0,
            ]);
            ntlmbuf.extend_from_slice(&[
                (domlen & 0xFF) as u8,
                ((domlen >> 8) & 0xFF) as u8,
                (domlen & 0xFF) as u8,
                ((domlen >> 8) & 0xFF) as u8,
                (domoff & 0xFF) as u8,
                ((domoff >> 8) & 0xFF) as u8,
                0, 0,
            ]);
            ntlmbuf.extend_from_slice(&[
                (userlen & 0xFF) as u8,
                ((userlen >> 8) & 0xFF) as u8,
                (userlen & 0xFF) as u8,
                ((userlen >> 8) & 0xFF) as u8,
                (useroff & 0xFF) as u8,
                ((useroff >> 8) & 0xFF) as u8,
                0, 0,
            ]);
            ntlmbuf.extend_from_slice(&[
                (hostlen & 0xFF) as u8,
                ((hostlen >> 8) & 0xFF) as u8,
                (hostlen & 0xFF) as u8,
                ((hostlen >> 8) & 0xFF) as u8,
                (hostoff & 0xFF) as u8,
                ((hostoff >> 8) & 0xFF) as u8,
                0, 0, 0, 0, 0, 0,
            ]);
            ntlmbuf.extend_from_slice(&[0xFF, 0xFF, 0x00, 0x00, 0x01, 0x82, 0x00, 0x00]);

            let mut size = 64;
            ntlmbuf.extend_from_slice(domain.as_bytes());
            size += domlen;
            ntlmbuf.extend_from_slice(user.as_bytes());
            size += userlen;
            ntlmbuf.extend_from_slice(&lmresp);
            size += 0x18;
            ntlmbuf.extend_from_slice(&ntresp);
            size += 0x18;

            ntlmbuf[56] = (size & 0xFF) as u8;
            ntlmbuf[57] = ((size >> 8) & 0xFF) as u8;

            let mut output = Vec::with_capacity(5 + base64::encoded_len(size));
            output.extend_from_slice(b"NTLM ");
            base64::encode_config_slice(&ntlmbuf[..size], base64::STANDARD, &mut output[5..]);
            ntlm.state = NtlState::Type3;
            Some(unsafe { String::from_utf8_unchecked(output) })
        }
        NtlState::Type3 => None,
    }
}