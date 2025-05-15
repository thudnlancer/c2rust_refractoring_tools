use std::mem::size_of;

type SizeT = usize;
type UInt8T = u8;

pub struct Base64DecodeCtx {
    table: *const i8,
    word: u16,
    bits: u8,
    padding: u8,
}

pub struct Base64EncodeCtx {
    alphabet: *const i8,
    word: u16,
    bits: u8,
}

pub struct NettleArmor {
    pub name: &'static str,
    pub encode_context_size: u32,
    pub decode_context_size: u32,
    pub encode_final_length: u32,
    pub encode_init: fn(&mut Base64EncodeCtx),
    pub encode_length: fn(SizeT) -> SizeT,
    pub encode_update: fn(&mut Base64EncodeCtx, &mut [i8], &[UInt8T]) -> SizeT,
    pub encode_final: fn(&mut Base64EncodeCtx, &mut [i8]) -> SizeT,
    pub decode_init: fn(&mut Base64DecodeCtx),
    pub decode_length: fn(SizeT) -> SizeT,
    pub decode_update: fn(&mut Base64DecodeCtx, &mut SizeT, &mut [UInt8T], &[i8]) -> i32,
    pub decode_final: fn(&mut Base64DecodeCtx) -> i32,
}

fn base64_encode_length(length: SizeT) -> SizeT {
    (length * 8 + 4) / 6
}

fn base64_decode_length(length: SizeT) -> SizeT {
    (length + 1) * 6 / 8
}

pub static NETTLE_BASE64: NettleArmor = NettleArmor {
    name: "base64",
    encode_context_size: size_of::<Base64EncodeCtx>() as u32,
    decode_context_size: size_of::<Base64DecodeCtx>() as u32,
    encode_final_length: 3,
    encode_init: |_| {},
    encode_length: base64_encode_length,
    encode_update: |_, _, _| 0,
    encode_final: |_, _| 0,
    decode_init: |_| {},
    decode_length: base64_decode_length,
    decode_update: |_, _, _, _| 0,
    decode_final: |_| 0,
};