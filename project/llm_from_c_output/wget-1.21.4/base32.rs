use std::convert::TryFrom;
use std::ptr;

const BASE32_ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

#[derive(Debug, Default)]
pub struct Base32DecodeContext {
    i: usize,
    buf: [u8; 8],
}

pub fn is_base32(ch: u8) -> bool {
    BASE32_DECODE[ch as usize] != -1
}

pub fn base32_encode(input: &[u8], output: &mut [u8]) -> usize {
    let mut in_pos = 0;
    let mut out_pos = 0;
    let in_len = input.len();
    let out_len = output.len();

    while in_pos < in_len && out_pos < out_len {
        output[out_pos] = BASE32_ALPHABET[(input[in_pos] >> 3) as usize & 0x1F];
        out_pos += 1;
        if out_pos >= out_len {
            break;
        }

        let byte1 = if in_pos + 1 < in_len { input[in_pos + 1] } else { 0 };
        output[out_pos] = BASE32_ALPHABET[((input[in_pos] << 2) | (byte1 >> 6)) as usize & 0x1F];
        out_pos += 1;
        if out_pos >= out_len {
            break;
        }

        if in_pos + 1 < in_len {
            output[out_pos] = BASE32_ALPHABET[(byte1 >> 1) as usize & 0x1F];
            out_pos += 1;
            if out_pos >= out_len {
                break;
            }

            let byte2 = if in_pos + 2 < in_len { input[in_pos + 2] } else { 0 };
            output[out_pos] = BASE32_ALPHABET[((byte1 << 4) | (byte2 >> 4)) as usize & 0x1F];
            out_pos += 1;
            if out_pos >= out_len {
                break;
            }

            if in_pos + 2 < in_len {
                let byte3 = if in_pos + 3 < in_len { input[in_pos + 3] } else { 0 };
                output[out_pos] = BASE32_ALPHABET[((byte2 << 1) | (byte3 >> 7)) as usize & 0x1F];
                out_pos += 1;
                if out_pos >= out_len {
                    break;
                }

                if in_pos + 3 < in_len {
                    output[out_pos] = BASE32_ALPHABET[(byte3 >> 2) as usize & 0x1F];
                    out_pos += 1;
                    if out_pos >= out_len {
                        break;
                    }

                    let byte4 = if in_pos + 4 < in_len { input[in_pos + 4] } else { 0 };
                    output[out_pos] = BASE32_ALPHABET[((byte3 << 3) | (byte4 >> 5)) as usize & 0x1F];
                    out_pos += 1;
                    if out_pos >= out_len {
                        break;
                    }

                    if in_pos + 4 < in_len {
                        output[out_pos] = BASE32_ALPHABET[(byte4 & 0x1F) as usize];
                        out_pos += 1;
                        in_pos += 5;
                    } else {
                        output[out_pos] = b'=';
                        out_pos += 1;
                        in_pos += 5;
                    }
                } else {
                    output[out_pos] = b'=';
                    out_pos += 1;
                    output[out_pos] = b'=';
                    out_pos += 1;
                    output[out_pos] = b'=';
                    out_pos += 1;
                    in_pos += 5;
                }
            } else {
                output[out_pos] = b'=';
                out_pos += 1;
                output[out_pos] = b'=';
                out_pos += 1;
                output[out_pos] = b'=';
                out_pos += 1;
                output[out_pos] = b'=';
                out_pos += 1;
                in_pos += 5;
            }
        } else {
            output[out_pos] = b'=';
            out_pos += 1;
            output[out_pos] = b'=';
            out_pos += 1;
            output[out_pos] = b'=';
            out_pos += 1;
            output[out_pos] = b'=';
            out_pos += 1;
            output[out_pos] = b'=';
            out_pos += 1;
            in_pos += 5;
        }
    }

    if out_pos < out_len {
        output[out_pos] = 0;
    }

    out_pos
}

pub fn base32_encode_alloc(input: &[u8]) -> Option<Vec<u8>> {
    let out_len = ((input.len() + 4) / 5) * 8 + 1;
    let mut output = vec![0; out_len];
    let encoded_len = base32_encode(input, &mut output);
    output.truncate(encoded_len);
    Some(output)
}

const BASE32_DECODE: [i8; 256] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, 26, 27, 28, 29, 30, 31, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14,
    15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, -1, -1, -1, -1, -1,
    -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14,
    15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
];

pub fn base32_decode_ctx_init(ctx: &mut Base32DecodeContext) {
    ctx.i = 0;
}

fn get_8(ctx: &mut Base32DecodeContext, input: &[u8], in_pos: &mut usize, in_len: usize) -> (usize, &[u8]) {
    if ctx.i == 8 {
        ctx.i = 0;
    }

    if ctx.i == 0 && *in_pos + 8 <= in_len {
        let non_nl = &input[*in_pos..*in_pos + 8];
        *in_pos += 8;
        return (8, non_nl);
    }

    let mut count = 0;
    while *in_pos < in_len && count < 8 {
        let c = input[*in_pos];
        *in_pos += 1;
        if c != b'\n' {
            ctx.buf[ctx.i] = c;
            ctx.i += 1;
            count += 1;
            if ctx.i == 8 {
                break;
            }
        }
    }

    (count, &ctx.buf[..ctx.i])
}

fn decode_8(input: &[u8], output: &mut [u8], out_pos: &mut usize) -> bool {
    if input.len() < 8 {
        return false;
    }

    if !is_base32(input[0]) || !is_base32(input[1]) {
        return false;
    }

    if *out_pos < output.len() {
        output[*out_pos] = ((BASE32_DECODE[input[0] as usize] << 3) | (BASE32_DECODE[input[1] as usize] >> 2)) as u8;
        *out_pos += 1;
    }

    if input[2] == b'=' {
        if input[3] != b'=' || input[4] != b'=' || input[5] != b'=' || input[6] != b'=' || input[7] != b'=' {
            return false;
        }
    } else {
        if !is_base32(input[2]) || !is_base32(input[3]) {
            return false;
        }

        if *out_pos < output.len() {
            output[*out_pos] = ((BASE32_DECODE[input[1] as usize] << 6) 
                | (BASE32_DECODE[input[2] as usize] << 1) 
                | (BASE32_DECODE[input[3] as usize] >> 4)) as u8;
            *out_pos += 1;
        }

        if input[4] == b'=' {
            if input[5] != b'=' || input[6] != b'=' || input[7] != b'=' {
                return false;
            }
        } else {
            if !is_base32(input[4]) {
                return false;
            }

            if *out_pos < output.len() {
                output[*out_pos] = ((BASE32_DECODE[input[3] as usize] << 4) 
                    | (BASE32_DECODE[input[4] as usize] >> 1)) as u8;
                *out_pos += 1;
            }

            if input[5] == b'=' {
                if input[6] != b'=' || input[7] != b'=' {
                    return false;
                }
            } else {
                if !is_base32(input[5]) || !is_base32(input[6]) {
                    return false;
                }

                if *out_pos < output.len() {
                    output[*out_pos] = ((BASE32_DECODE[input[4] as usize] << 7) 
                        | (BASE32_DECODE[input[5] as usize] << 2) 
                        | (BASE32_DECODE[input[6] as usize] >> 3)) as u8;
                    *out_pos += 1;
                }

                if input[7] != b'=' {
                    if !is_base32(input[7]) {
                        return false;
                    }

                    if *out_pos < output.len() {
                        output[*out_pos] = ((BASE32_DECODE[input[6] as usize] << 5) 
                            | BASE32_DECODE[input[7] as usize]) as u8;
                        *out_pos += 1;
                    }
                }
            }
        }
    }

    true
}

pub fn base32_decode_ctx(ctx: Option<&mut Base32DecodeContext>, input: &[u8], output: &mut [u8]) -> Result<usize, ()> {
    let mut out_pos = 0;
    let mut in_pos = 0;
    let in_len = input.len();
    let mut ctx_i = 0;
    let mut flush_ctx = false;

    let mut local_ctx = Base32DecodeContext::default();
    let ctx = match ctx {
        Some(c) => c,
        None => &mut local_ctx,
    };

    if in_len == 0 {
        flush_ctx = true;
    } else {
        ctx_i = ctx.i;
    }

    loop {
        let outleft_save = output.len() - out_pos;
        if ctx_i == 0 && !flush_ctx {
            while in_pos + 8 <= in_len {
                if !decode_8(&input[in_pos..in_pos+8], output, &mut out_pos) {
                    break;
                }
                in_pos += 8;
            }
        }

        if in_pos >= in_len && !flush_ctx {
            break;
        }

        if in_pos < in_len && input[in_pos] == b'\n' && ctx.is_some() {
            in_pos += 1;
            continue;
        }

        let (non_nl_len, non_nl) = get_8(ctx, input, &mut in_pos, in_len);
        if non_nl_len == 0 || (non_nl_len < 8 && !flush_ctx && ctx.is_some()) {
            break;
        }

        if !decode_8(non_nl, output, &mut out_pos) {
            return Err(());
        }
    }

    Ok(out_pos)
}

pub fn base32_decode_alloc_ctx(ctx: Option<&mut Base32DecodeContext>, input: &[u8]) -> Result<Vec<u8>, ()> {
    let need_len = 5 * ((input.len() >> 3) + 1);
    let mut output = vec![0; need_len];
    let decoded_len = base32_decode_ctx(ctx, input, &mut output)?;
    output.truncate(decoded_len);
    Ok(output)
}

pub fn base32_decode(input: &[u8], output: &mut [u8]) -> Result<usize, ()> {
    base32_decode_ctx(None, input, output)
}

pub fn base32_decode_alloc(input: &[u8]) -> Result<Vec<u8>, ()> {
    base32_decode_alloc_ctx(None, input)
}