#[derive(Default, Clone, Copy)]
pub struct Base64EncodeCtx {
    alphabet: &'static [u8; 64],
    word: u16,
    bits: u8,
}

pub fn nettle_base64url_encode_init(ctx: &mut Base64EncodeCtx) {
    static BASE64URL_ENCODE_TABLE: [u8; 64] = 
        *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    
    ctx.bits = 0;
    ctx.word = ctx.bits as u16;
    ctx.alphabet = &BASE64URL_ENCODE_TABLE;
}