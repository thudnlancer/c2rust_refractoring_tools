use std::ptr;
use std::mem;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uchar, c_ulong, c_void};
use std::slice;

const NTLMSTATE_NONE: u32 = 0;
const NTLMSTATE_TYPE1: u32 = 1;
const NTLMSTATE_TYPE2: u32 = 2;
const NTLMSTATE_TYPE3: u32 = 3;
const NTLMSTATE_LAST: u32 = 4;

struct NtlmData {
    state: u32,
    nonce: [u8; 8],
}

struct DesCtx {
    key: [u32; 32],
}

struct Md4Ctx {
    state: [u32; 4],
    count: u64,
    block: [u8; 64],
    index: u32,
}

fn c_isspace(c: i32) -> bool {
    matches!(c, 32 | 9 | 10 | 11 | 12 | 13)
}

fn c_toupper(c: i32) -> i32 {
    if c >= 'a' as i32 && c <= 'z' as i32 {
        c - 'a' as i32 + 'A' as i32
    } else {
        c
    }
}

fn ntlm_input(ntlm: &mut NtlmData, header: &[u8]) -> bool {
    if !header.starts_with(b"NTLM") {
        return false;
    }

    let mut header = &header[4..];
    while !header.is_empty() && c_isspace(header[0] as i32) {
        header = &header[1..];
    }

    if !header.is_empty() {
        let mut buffer = [0u8; 48];
        if opt.debug {
            debug_logprintf("Received a type-2 NTLM message.\n");
        }

        let size = wget_base64_decode(header, &mut buffer);
        if size < 0 {
            return false;
        }

        ntlm.state = NTLMSTATE_TYPE2;
        if size as usize >= buffer.len() {
            ntlm.nonce.copy_from_slice(&buffer[24..32]);
        }
    } else {
        match ntlm.state {
            NTLMSTATE_LAST => {
                if opt.debug {
                    debug_logprintf("NTLM auth restarted.\n");
                }
            }
            NTLMSTATE_TYPE3 => {
                if opt.debug {
                    debug_logprintf("NTLM handshake rejected.\n");
                }
                ntlm.state = NTLMSTATE_NONE;
                return false;
            }
            _ if ntlm.state >= NTLMSTATE_TYPE1 => {
                if opt.debug {
                    debug_logprintf("Unexpected empty NTLM message.\n");
                }
                return false;
            }
            _ => {}
        }

        if opt.debug {
            debug_logprintf("Empty NTLM message, (re)starting transaction.\n");
        }
        ntlm.state = NTLMSTATE_TYPE1;
    }

    true
}

fn setup_des_key(key_56: &[u8], des: &mut DesCtx) {
    let mut key = [0u8; 8];
    key[0] = key_56[0];
    key[1] = ((key_56[0] as i32) << 7 & 0xff | (key_56[1] as i32) >> 1) as u8;
    key[2] = ((key_56[1] as i32) << 6 & 0xff | (key_56[2] as i32) >> 2) as u8;
    key[3] = ((key_56[2] as i32) << 5 & 0xff | (key_56[3] as i32) >> 3) as u8;
    key[4] = ((key_56[3] as i32) << 4 & 0xff | (key_56[4] as i32) >> 4) as u8;
    key[5] = ((key_56[4] as i32) << 3 & 0xff | (key_56[5] as i32) >> 5) as u8;
    key[6] = ((key_56[5] as i32) << 2 & 0xff | (key_56[6] as i32) >> 6) as u8;
    key[7] = ((key_56[6] as i32) << 1 & 0xff) as u8;

    nettle_des_set_key(des, &key);
}

fn calc_resp(keys: &[u8], plaintext: &[u8], results: &mut [u8]) {
    let mut des = DesCtx { key: [0; 32] };
    setup_des_key(&keys[0..7], &mut des);
    nettle_des_encrypt(&des, 8, &mut results[0..8], plaintext);

    setup_des_key(&keys[7..14], &mut des);
    nettle_des_encrypt(&des, 8, &mut results[8..16], plaintext);

    setup_des_key(&keys[14..21], &mut des);
    nettle_des_encrypt(&des, 8, &mut results[16..24], plaintext);
}

fn mkhash(password: &str, nonce: &[u8], lmresp: &mut [u8], ntresp: &mut [u8]) {
    let mut lmbuffer = [0u8; 21];
    let mut ntbuffer = [0u8; 21];
    let mut pw = [0u8; 14];
    let magic = [0x4b, 0x47, 0x53, 0x21, 0x40, 0x23, 0x24, 0x25];

    let len = password.len().min(14);
    for (i, c) in password[..len].chars().enumerate() {
        pw[i] = c_toupper(c as i32) as u8;
    }

    let mut des = DesCtx { key: [0; 32] };
    setup_des_key(&pw[0..7], &mut des);
    nettle_des_encrypt(&des, 8, &mut lmbuffer[0..8], &magic);

    setup_des_key(&pw[7..14], &mut des);
    nettle_des_encrypt(&des, 8, &mut lmbuffer[8..16], &magic);

    lmbuffer[16..21].fill(0);
    calc_resp(&lmbuffer, nonce, lmresp);

    let mut md4 = Md4Ctx {
        state: [0; 4],
        count: 0,
        block: [0; 64],
        index: 0,
    };
    let mut pw4 = [0u8; 64];
    let len = password.len().min(32);
    for i in 0..len {
        pw4[i * 2] = password.as_bytes()[i];
        pw4[i * 2 + 1] = 0;
    }

    nettle_md4_init(&mut md4);
    nettle_md4_update(&mut md4, len * 2, &pw4);
    nettle_md4_digest(&mut md4, 16, &mut ntbuffer);

    ntbuffer[16..21].fill(0);
    calc_resp(&ntbuffer, nonce, ntresp);
}

fn ntlm_output(
    ntlm: &mut NtlmData,
    user: &str,
    passwd: &str,
    ready: &mut bool,
) -> Option<CString> {
    let domain = "";
    let host = "";
    let mut domlen = domain.len();
    let hostlen = host.len();
    let mut output = None;
    *ready = false;

    match ntlm.state {
        NTLMSTATE_NONE | NTLMSTATE_TYPE1 | NTLMSTATE_LAST => {
            let hostoff = 32;
            let domoff = hostoff + hostlen;
            if opt.debug {
                debug_logprintf("Creating a type-1 NTLM message.\n");
            }

            let mut ntlmbuf = Vec::new();
            ntlmbuf.extend_from_slice(b"NTLMSSP\0\x01\0\0\0");
            ntlmbuf.extend_from_slice(&[
                ((1 << 1 | 1 << 9) & 0xff) as u8,
                ((1 << 1 | 1 << 9) >> 8) as u8,
                ((1 << 1 | 1 << 9) >> 16) as u8,
                ((1 << 1 | 1 << 9) >> 24) as u8,
                (domlen & 0xff) as u8,
                (domlen >> 8) as u8,
                (domlen & 0xff) as u8,
                (domlen >> 8) as u8,
                (domoff & 0xff) as u8,
                (domoff >> 8) as u8,
                0,
                0,
                (hostlen & 0xff) as u8,
                (hostlen >> 8) as u8,
                (hostlen & 0xff) as u8,
                (hostlen >> 8) as u8,
                (hostoff & 0xff) as u8,
                (hostoff >> 8) as u8,
                0,
                0,
            ]);
            ntlmbuf.extend_from_slice(host.as_bytes());
            ntlmbuf.extend_from_slice(domain.as_bytes());

            let size = 32 + hostlen + domlen;
            let mut encoded = vec![0u8; 5 + 4 * ((size + 2) / 3) + 1];
            encoded[..5].copy_from_slice(b"NTLM ");
            let encoded_len = wget_base64_encode(&ntlmbuf, &mut encoded[5..]);
            encoded.truncate(5 + encoded_len);
            output = Some(CString::new(encoded).unwrap());
        }
        NTLMSTATE_TYPE2 => {
            let mut lmresp = [0u8; 24];
            let mut ntresp = [0u8; 24];
            let mut usr = user;
            let mut userlen = usr.len();

            if opt.debug {
                debug_logprintf("Creating a type-3 NTLM message.\n");
            }

            if let Some(pos) = usr.find('\\').or_else(|| usr.find('/')) {
                domain = &usr[..pos];
                domlen = pos;
                usr = &usr[pos + 1..];
                userlen = usr.len();
            }

            mkhash(passwd, &ntlm.nonce, &mut lmresp, &mut ntresp);

            let domoff = 64;
            let useroff = domoff + domlen;
            let hostoff = useroff + userlen;
            let lmrespoff = hostoff + hostlen;
            let ntrespoff = lmrespoff + 24;

            let mut ntlmbuf = Vec::new();
            ntlmbuf.extend_from_slice(b"NTLMSSP\0\x03\0\0\0");
            ntlmbuf.extend_from_slice(&[
                24, 0, 24, 0, (lmrespoff & 0xff) as u8, (lmrespoff >> 8) as u8, 0, 0,
                24, 0, 24, 0, (ntrespoff & 0xff) as u8, (ntrespoff >> 8) as u8, 0, 0,
                (domlen & 0xff) as u8, (domlen >> 8) as u8, (domlen & 0xff) as u8, (domlen >> 8) as u8,
                (domoff & 0xff) as u8, (domoff >> 8) as u8, 0, 0,
                (userlen & 0xff) as u8, (userlen >> 8) as u8, (userlen & 0xff) as u8, (userlen >> 8) as u8,
                (useroff & 0xff) as u8, (useroff >> 8) as u8, 0, 0,
                (hostlen & 0xff) as u8, (hostlen >> 8) as u8, (hostlen & 0xff) as u8, (hostlen >> 8) as u8,
                (hostoff & 0xff) as u8, (hostoff >> 8) as u8, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, 0x01, 0x82, 0, 0,
            ]);

            let mut size = 64;
            ntlmbuf.extend_from_slice(domain.as_bytes());
            size += domlen;
            ntlmbuf.extend_from_slice(usr.as_bytes());
            size += userlen;

            if size + 24 <= 256 {
                ntlmbuf.extend_from_slice(&lmresp);
                size += 24;
            }

            if size + 24 <= 256 {
                ntlmbuf.extend_from_slice(&ntresp);
                size += 24;
            }

            ntlmbuf[56] = (size & 0xff) as u8;
            ntlmbuf[57] = (size >> 8) as u8;

            let mut encoded = vec![0u8; 5 + 4 * ((size + 2) / 3) + 1];
            encoded[..5].copy_from_slice(b"NTLM ");
            let encoded_len = wget_base64_encode(&ntlmbuf, &mut encoded[5..]);
            encoded.truncate(5 + encoded_len);
            output = Some(CString::new(encoded).unwrap());

            ntlm.state = NTLMSTATE_TYPE3;
            *ready = true;
        }
        NTLMSTATE_TYPE3 => {
            *ready = true;
        }
        _ => {}
    }

    output
}