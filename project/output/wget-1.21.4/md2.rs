#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
}
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct md2_ctx {
    pub chksum: [u8; 16],
    pub X: [u8; 48],
    pub buf: [u8; 16],
    pub curlen: size_t,
}
#[no_mangle]
pub unsafe extern "C" fn md2_init_ctx(mut ctx: *mut md2_ctx) {
    memset(
        ((*ctx).X).as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<[u8; 48]>() as u64,
    );
    memset(
        ((*ctx).chksum).as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<[u8; 16]>() as u64,
    );
    memset(
        ((*ctx).buf).as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<[u8; 16]>() as u64,
    );
    (*ctx).curlen = 0 as i32 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn md2_read_ctx(
    mut ctx: *const md2_ctx,
    mut resbuf: *mut libc::c_void,
) -> *mut libc::c_void {
    memcpy(resbuf, ((*ctx).X).as_ptr() as *const libc::c_void, 16 as i32 as u64);
    return resbuf;
}
#[no_mangle]
pub unsafe extern "C" fn md2_finish_ctx(
    mut ctx: *mut md2_ctx,
    mut resbuf: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut i: u64 = 0;
    let mut k: u64 = 0;
    k = (16 as i32 as u64).wrapping_sub((*ctx).curlen);
    i = (*ctx).curlen;
    while i < 16 as i32 as u64 {
        (*ctx).buf[i as usize] = k as u8;
        i = i.wrapping_add(1);
        i;
    }
    md2_compress(ctx);
    md2_update_chksum(ctx);
    memcpy(
        ((*ctx).buf).as_mut_ptr() as *mut libc::c_void,
        ((*ctx).chksum).as_mut_ptr() as *const libc::c_void,
        16 as i32 as u64,
    );
    md2_compress(ctx);
    return md2_read_ctx(ctx, resbuf);
}
#[no_mangle]
pub unsafe extern "C" fn md2_buffer(
    mut buffer: *const i8,
    mut len: size_t,
    mut resblock: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut ctx: md2_ctx = md2_ctx {
        chksum: [0; 16],
        X: [0; 48],
        buf: [0; 16],
        curlen: 0,
    };
    md2_init_ctx(&mut ctx);
    md2_process_block(buffer as *const libc::c_void, len, &mut ctx);
    return md2_finish_ctx(&mut ctx, resblock);
}
#[no_mangle]
pub unsafe extern "C" fn md2_process_bytes(
    mut buffer: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut md2_ctx,
) {
    let mut in_0: *const i8 = buffer as *const i8;
    let mut n: u64 = 0;
    while len > 0 as i32 as u64 {
        n = if len < (16 as i32 as u64).wrapping_sub((*ctx).curlen) {
            len
        } else {
            (16 as i32 as u64).wrapping_sub((*ctx).curlen)
        };
        memcpy(
            ((*ctx).buf).as_mut_ptr().offset((*ctx).curlen as isize)
                as *mut libc::c_void,
            in_0 as *const libc::c_void,
            n,
        );
        (*ctx).curlen = ((*ctx).curlen as u64).wrapping_add(n) as size_t as size_t;
        in_0 = in_0.offset(n as isize);
        len = (len as u64).wrapping_sub(n) as size_t as size_t;
        if (*ctx).curlen == 16 as i32 as u64 {
            md2_compress(ctx);
            md2_update_chksum(ctx);
            (*ctx).curlen = 0 as i32 as size_t;
        }
    }
}
static mut PI_SUBST: [u8; 256] = [
    41 as i32 as u8,
    46 as i32 as u8,
    67 as i32 as u8,
    201 as i32 as u8,
    162 as i32 as u8,
    216 as i32 as u8,
    124 as i32 as u8,
    1 as i32 as u8,
    61 as i32 as u8,
    54 as i32 as u8,
    84 as i32 as u8,
    161 as i32 as u8,
    236 as i32 as u8,
    240 as i32 as u8,
    6 as i32 as u8,
    19 as i32 as u8,
    98 as i32 as u8,
    167 as i32 as u8,
    5 as i32 as u8,
    243 as i32 as u8,
    192 as i32 as u8,
    199 as i32 as u8,
    115 as i32 as u8,
    140 as i32 as u8,
    152 as i32 as u8,
    147 as i32 as u8,
    43 as i32 as u8,
    217 as i32 as u8,
    188 as i32 as u8,
    76 as i32 as u8,
    130 as i32 as u8,
    202 as i32 as u8,
    30 as i32 as u8,
    155 as i32 as u8,
    87 as i32 as u8,
    60 as i32 as u8,
    253 as i32 as u8,
    212 as i32 as u8,
    224 as i32 as u8,
    22 as i32 as u8,
    103 as i32 as u8,
    66 as i32 as u8,
    111 as i32 as u8,
    24 as i32 as u8,
    138 as i32 as u8,
    23 as i32 as u8,
    229 as i32 as u8,
    18 as i32 as u8,
    190 as i32 as u8,
    78 as i32 as u8,
    196 as i32 as u8,
    214 as i32 as u8,
    218 as i32 as u8,
    158 as i32 as u8,
    222 as i32 as u8,
    73 as i32 as u8,
    160 as i32 as u8,
    251 as i32 as u8,
    245 as i32 as u8,
    142 as i32 as u8,
    187 as i32 as u8,
    47 as i32 as u8,
    238 as i32 as u8,
    122 as i32 as u8,
    169 as i32 as u8,
    104 as i32 as u8,
    121 as i32 as u8,
    145 as i32 as u8,
    21 as i32 as u8,
    178 as i32 as u8,
    7 as i32 as u8,
    63 as i32 as u8,
    148 as i32 as u8,
    194 as i32 as u8,
    16 as i32 as u8,
    137 as i32 as u8,
    11 as i32 as u8,
    34 as i32 as u8,
    95 as i32 as u8,
    33 as i32 as u8,
    128 as i32 as u8,
    127 as i32 as u8,
    93 as i32 as u8,
    154 as i32 as u8,
    90 as i32 as u8,
    144 as i32 as u8,
    50 as i32 as u8,
    39 as i32 as u8,
    53 as i32 as u8,
    62 as i32 as u8,
    204 as i32 as u8,
    231 as i32 as u8,
    191 as i32 as u8,
    247 as i32 as u8,
    151 as i32 as u8,
    3 as i32 as u8,
    255 as i32 as u8,
    25 as i32 as u8,
    48 as i32 as u8,
    179 as i32 as u8,
    72 as i32 as u8,
    165 as i32 as u8,
    181 as i32 as u8,
    209 as i32 as u8,
    215 as i32 as u8,
    94 as i32 as u8,
    146 as i32 as u8,
    42 as i32 as u8,
    172 as i32 as u8,
    86 as i32 as u8,
    170 as i32 as u8,
    198 as i32 as u8,
    79 as i32 as u8,
    184 as i32 as u8,
    56 as i32 as u8,
    210 as i32 as u8,
    150 as i32 as u8,
    164 as i32 as u8,
    125 as i32 as u8,
    182 as i32 as u8,
    118 as i32 as u8,
    252 as i32 as u8,
    107 as i32 as u8,
    226 as i32 as u8,
    156 as i32 as u8,
    116 as i32 as u8,
    4 as i32 as u8,
    241 as i32 as u8,
    69 as i32 as u8,
    157 as i32 as u8,
    112 as i32 as u8,
    89 as i32 as u8,
    100 as i32 as u8,
    113 as i32 as u8,
    135 as i32 as u8,
    32 as i32 as u8,
    134 as i32 as u8,
    91 as i32 as u8,
    207 as i32 as u8,
    101 as i32 as u8,
    230 as i32 as u8,
    45 as i32 as u8,
    168 as i32 as u8,
    2 as i32 as u8,
    27 as i32 as u8,
    96 as i32 as u8,
    37 as i32 as u8,
    173 as i32 as u8,
    174 as i32 as u8,
    176 as i32 as u8,
    185 as i32 as u8,
    246 as i32 as u8,
    28 as i32 as u8,
    70 as i32 as u8,
    97 as i32 as u8,
    105 as i32 as u8,
    52 as i32 as u8,
    64 as i32 as u8,
    126 as i32 as u8,
    15 as i32 as u8,
    85 as i32 as u8,
    71 as i32 as u8,
    163 as i32 as u8,
    35 as i32 as u8,
    221 as i32 as u8,
    81 as i32 as u8,
    175 as i32 as u8,
    58 as i32 as u8,
    195 as i32 as u8,
    92 as i32 as u8,
    249 as i32 as u8,
    206 as i32 as u8,
    186 as i32 as u8,
    197 as i32 as u8,
    234 as i32 as u8,
    38 as i32 as u8,
    44 as i32 as u8,
    83 as i32 as u8,
    13 as i32 as u8,
    110 as i32 as u8,
    133 as i32 as u8,
    40 as i32 as u8,
    132 as i32 as u8,
    9 as i32 as u8,
    211 as i32 as u8,
    223 as i32 as u8,
    205 as i32 as u8,
    244 as i32 as u8,
    65 as i32 as u8,
    129 as i32 as u8,
    77 as i32 as u8,
    82 as i32 as u8,
    106 as i32 as u8,
    220 as i32 as u8,
    55 as i32 as u8,
    200 as i32 as u8,
    108 as i32 as u8,
    193 as i32 as u8,
    171 as i32 as u8,
    250 as i32 as u8,
    36 as i32 as u8,
    225 as i32 as u8,
    123 as i32 as u8,
    8 as i32 as u8,
    12 as i32 as u8,
    189 as i32 as u8,
    177 as i32 as u8,
    74 as i32 as u8,
    120 as i32 as u8,
    136 as i32 as u8,
    149 as i32 as u8,
    139 as i32 as u8,
    227 as i32 as u8,
    99 as i32 as u8,
    232 as i32 as u8,
    109 as i32 as u8,
    233 as i32 as u8,
    203 as i32 as u8,
    213 as i32 as u8,
    254 as i32 as u8,
    59 as i32 as u8,
    0 as i32 as u8,
    29 as i32 as u8,
    57 as i32 as u8,
    242 as i32 as u8,
    239 as i32 as u8,
    183 as i32 as u8,
    14 as i32 as u8,
    102 as i32 as u8,
    88 as i32 as u8,
    208 as i32 as u8,
    228 as i32 as u8,
    166 as i32 as u8,
    119 as i32 as u8,
    114 as i32 as u8,
    248 as i32 as u8,
    235 as i32 as u8,
    117 as i32 as u8,
    75 as i32 as u8,
    10 as i32 as u8,
    49 as i32 as u8,
    68 as i32 as u8,
    80 as i32 as u8,
    180 as i32 as u8,
    143 as i32 as u8,
    237 as i32 as u8,
    31 as i32 as u8,
    26 as i32 as u8,
    219 as i32 as u8,
    153 as i32 as u8,
    141 as i32 as u8,
    51 as i32 as u8,
    159 as i32 as u8,
    17 as i32 as u8,
    131 as i32 as u8,
    20 as i32 as u8,
];
unsafe extern "C" fn md2_update_chksum(mut ctx: *mut md2_ctx) {
    let mut j: i32 = 0;
    let mut L: u8 = 0;
    L = (*ctx).chksum[15 as i32 as usize];
    j = 0 as i32;
    while j < 16 as i32 {
        (*ctx).chksum[j as usize] = ((*ctx).chksum[j as usize] as i32
            ^ PI_SUBST[((*ctx).buf[j as usize] as i32 ^ L as i32) as usize] as i32
                & 255 as i32) as u8;
        L = (*ctx).chksum[j as usize];
        j += 1;
        j;
    }
}
unsafe extern "C" fn md2_compress(mut ctx: *mut md2_ctx) {
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut t: u8 = 0;
    j = 0 as i32 as size_t;
    while j < 16 as i32 as u64 {
        (*ctx).X[(16 as i32 as u64).wrapping_add(j) as usize] = (*ctx).buf[j as usize];
        (*ctx).X[(32 as i32 as u64).wrapping_add(j) as usize] = ((*ctx).X[j as usize]
            as i32 ^ (*ctx).X[(16 as i32 as u64).wrapping_add(j) as usize] as i32) as u8;
        j = j.wrapping_add(1);
        j;
    }
    t = 0 as i32 as u8;
    j = 0 as i32 as size_t;
    while j < 18 as i32 as u64 {
        k = 0 as i32 as size_t;
        while k < 48 as i32 as u64 {
            (*ctx).X[k as usize] = ((*ctx).X[k as usize] as i32
                ^ PI_SUBST[(t as i32 & 255 as i32) as usize] as i32) as u8;
            t = (*ctx).X[k as usize];
            k = k.wrapping_add(1);
            k;
        }
        t = (t as i32 + j as u8 as i32 & 255 as i32) as u8;
        j = j.wrapping_add(1);
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn md2_process_block(
    mut buffer: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut md2_ctx,
) {
    md2_process_bytes(buffer, len, ctx);
}