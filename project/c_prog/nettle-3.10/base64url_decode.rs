use ::libc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base64_decode_ctx {
    pub table: *const libc::c_schar,
    pub word: libc::c_ushort,
    pub bits: libc::c_uchar,
    pub padding: libc::c_uchar,
}
#[no_mangle]
pub unsafe extern "C" fn nettle_base64url_decode_init(mut ctx: *mut base64_decode_ctx) {
    static mut base64url_decode_table: [libc::c_schar; 256] = [
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(2 as libc::c_int) as libc::c_schar,
        -(2 as libc::c_int) as libc::c_schar,
        -(2 as libc::c_int) as libc::c_schar,
        -(2 as libc::c_int) as libc::c_schar,
        -(2 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(2 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        62 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        52 as libc::c_int as libc::c_schar,
        53 as libc::c_int as libc::c_schar,
        54 as libc::c_int as libc::c_schar,
        55 as libc::c_int as libc::c_schar,
        56 as libc::c_int as libc::c_schar,
        57 as libc::c_int as libc::c_schar,
        58 as libc::c_int as libc::c_schar,
        59 as libc::c_int as libc::c_schar,
        60 as libc::c_int as libc::c_schar,
        61 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(3 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        1 as libc::c_int as libc::c_schar,
        2 as libc::c_int as libc::c_schar,
        3 as libc::c_int as libc::c_schar,
        4 as libc::c_int as libc::c_schar,
        5 as libc::c_int as libc::c_schar,
        6 as libc::c_int as libc::c_schar,
        7 as libc::c_int as libc::c_schar,
        8 as libc::c_int as libc::c_schar,
        9 as libc::c_int as libc::c_schar,
        10 as libc::c_int as libc::c_schar,
        11 as libc::c_int as libc::c_schar,
        12 as libc::c_int as libc::c_schar,
        13 as libc::c_int as libc::c_schar,
        14 as libc::c_int as libc::c_schar,
        15 as libc::c_int as libc::c_schar,
        16 as libc::c_int as libc::c_schar,
        17 as libc::c_int as libc::c_schar,
        18 as libc::c_int as libc::c_schar,
        19 as libc::c_int as libc::c_schar,
        20 as libc::c_int as libc::c_schar,
        21 as libc::c_int as libc::c_schar,
        22 as libc::c_int as libc::c_schar,
        23 as libc::c_int as libc::c_schar,
        24 as libc::c_int as libc::c_schar,
        25 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        63 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        26 as libc::c_int as libc::c_schar,
        27 as libc::c_int as libc::c_schar,
        28 as libc::c_int as libc::c_schar,
        29 as libc::c_int as libc::c_schar,
        30 as libc::c_int as libc::c_schar,
        31 as libc::c_int as libc::c_schar,
        32 as libc::c_int as libc::c_schar,
        33 as libc::c_int as libc::c_schar,
        34 as libc::c_int as libc::c_schar,
        35 as libc::c_int as libc::c_schar,
        36 as libc::c_int as libc::c_schar,
        37 as libc::c_int as libc::c_schar,
        38 as libc::c_int as libc::c_schar,
        39 as libc::c_int as libc::c_schar,
        40 as libc::c_int as libc::c_schar,
        41 as libc::c_int as libc::c_schar,
        42 as libc::c_int as libc::c_schar,
        43 as libc::c_int as libc::c_schar,
        44 as libc::c_int as libc::c_schar,
        45 as libc::c_int as libc::c_schar,
        46 as libc::c_int as libc::c_schar,
        47 as libc::c_int as libc::c_schar,
        48 as libc::c_int as libc::c_schar,
        49 as libc::c_int as libc::c_schar,
        50 as libc::c_int as libc::c_schar,
        51 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
    ];
    (*ctx).padding = 0 as libc::c_int as libc::c_uchar;
    (*ctx).bits = (*ctx).padding;
    (*ctx).word = (*ctx).bits as libc::c_ushort;
    (*ctx).table = base64url_decode_table.as_ptr();
}
