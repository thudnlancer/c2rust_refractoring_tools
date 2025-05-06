#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(label_break_value)]
extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct md2_ctx {
    pub C: [uint8_t; 16],
    pub X: [uint8_t; 48],
    pub index: u32,
    pub block: [uint8_t; 16],
}
static mut S: [uint8_t; 256] = [
    41 as i32 as uint8_t,
    46 as i32 as uint8_t,
    67 as i32 as uint8_t,
    201 as i32 as uint8_t,
    162 as i32 as uint8_t,
    216 as i32 as uint8_t,
    124 as i32 as uint8_t,
    1 as i32 as uint8_t,
    61 as i32 as uint8_t,
    54 as i32 as uint8_t,
    84 as i32 as uint8_t,
    161 as i32 as uint8_t,
    236 as i32 as uint8_t,
    240 as i32 as uint8_t,
    6 as i32 as uint8_t,
    19 as i32 as uint8_t,
    98 as i32 as uint8_t,
    167 as i32 as uint8_t,
    5 as i32 as uint8_t,
    243 as i32 as uint8_t,
    192 as i32 as uint8_t,
    199 as i32 as uint8_t,
    115 as i32 as uint8_t,
    140 as i32 as uint8_t,
    152 as i32 as uint8_t,
    147 as i32 as uint8_t,
    43 as i32 as uint8_t,
    217 as i32 as uint8_t,
    188 as i32 as uint8_t,
    76 as i32 as uint8_t,
    130 as i32 as uint8_t,
    202 as i32 as uint8_t,
    30 as i32 as uint8_t,
    155 as i32 as uint8_t,
    87 as i32 as uint8_t,
    60 as i32 as uint8_t,
    253 as i32 as uint8_t,
    212 as i32 as uint8_t,
    224 as i32 as uint8_t,
    22 as i32 as uint8_t,
    103 as i32 as uint8_t,
    66 as i32 as uint8_t,
    111 as i32 as uint8_t,
    24 as i32 as uint8_t,
    138 as i32 as uint8_t,
    23 as i32 as uint8_t,
    229 as i32 as uint8_t,
    18 as i32 as uint8_t,
    190 as i32 as uint8_t,
    78 as i32 as uint8_t,
    196 as i32 as uint8_t,
    214 as i32 as uint8_t,
    218 as i32 as uint8_t,
    158 as i32 as uint8_t,
    222 as i32 as uint8_t,
    73 as i32 as uint8_t,
    160 as i32 as uint8_t,
    251 as i32 as uint8_t,
    245 as i32 as uint8_t,
    142 as i32 as uint8_t,
    187 as i32 as uint8_t,
    47 as i32 as uint8_t,
    238 as i32 as uint8_t,
    122 as i32 as uint8_t,
    169 as i32 as uint8_t,
    104 as i32 as uint8_t,
    121 as i32 as uint8_t,
    145 as i32 as uint8_t,
    21 as i32 as uint8_t,
    178 as i32 as uint8_t,
    7 as i32 as uint8_t,
    63 as i32 as uint8_t,
    148 as i32 as uint8_t,
    194 as i32 as uint8_t,
    16 as i32 as uint8_t,
    137 as i32 as uint8_t,
    11 as i32 as uint8_t,
    34 as i32 as uint8_t,
    95 as i32 as uint8_t,
    33 as i32 as uint8_t,
    128 as i32 as uint8_t,
    127 as i32 as uint8_t,
    93 as i32 as uint8_t,
    154 as i32 as uint8_t,
    90 as i32 as uint8_t,
    144 as i32 as uint8_t,
    50 as i32 as uint8_t,
    39 as i32 as uint8_t,
    53 as i32 as uint8_t,
    62 as i32 as uint8_t,
    204 as i32 as uint8_t,
    231 as i32 as uint8_t,
    191 as i32 as uint8_t,
    247 as i32 as uint8_t,
    151 as i32 as uint8_t,
    3 as i32 as uint8_t,
    255 as i32 as uint8_t,
    25 as i32 as uint8_t,
    48 as i32 as uint8_t,
    179 as i32 as uint8_t,
    72 as i32 as uint8_t,
    165 as i32 as uint8_t,
    181 as i32 as uint8_t,
    209 as i32 as uint8_t,
    215 as i32 as uint8_t,
    94 as i32 as uint8_t,
    146 as i32 as uint8_t,
    42 as i32 as uint8_t,
    172 as i32 as uint8_t,
    86 as i32 as uint8_t,
    170 as i32 as uint8_t,
    198 as i32 as uint8_t,
    79 as i32 as uint8_t,
    184 as i32 as uint8_t,
    56 as i32 as uint8_t,
    210 as i32 as uint8_t,
    150 as i32 as uint8_t,
    164 as i32 as uint8_t,
    125 as i32 as uint8_t,
    182 as i32 as uint8_t,
    118 as i32 as uint8_t,
    252 as i32 as uint8_t,
    107 as i32 as uint8_t,
    226 as i32 as uint8_t,
    156 as i32 as uint8_t,
    116 as i32 as uint8_t,
    4 as i32 as uint8_t,
    241 as i32 as uint8_t,
    69 as i32 as uint8_t,
    157 as i32 as uint8_t,
    112 as i32 as uint8_t,
    89 as i32 as uint8_t,
    100 as i32 as uint8_t,
    113 as i32 as uint8_t,
    135 as i32 as uint8_t,
    32 as i32 as uint8_t,
    134 as i32 as uint8_t,
    91 as i32 as uint8_t,
    207 as i32 as uint8_t,
    101 as i32 as uint8_t,
    230 as i32 as uint8_t,
    45 as i32 as uint8_t,
    168 as i32 as uint8_t,
    2 as i32 as uint8_t,
    27 as i32 as uint8_t,
    96 as i32 as uint8_t,
    37 as i32 as uint8_t,
    173 as i32 as uint8_t,
    174 as i32 as uint8_t,
    176 as i32 as uint8_t,
    185 as i32 as uint8_t,
    246 as i32 as uint8_t,
    28 as i32 as uint8_t,
    70 as i32 as uint8_t,
    97 as i32 as uint8_t,
    105 as i32 as uint8_t,
    52 as i32 as uint8_t,
    64 as i32 as uint8_t,
    126 as i32 as uint8_t,
    15 as i32 as uint8_t,
    85 as i32 as uint8_t,
    71 as i32 as uint8_t,
    163 as i32 as uint8_t,
    35 as i32 as uint8_t,
    221 as i32 as uint8_t,
    81 as i32 as uint8_t,
    175 as i32 as uint8_t,
    58 as i32 as uint8_t,
    195 as i32 as uint8_t,
    92 as i32 as uint8_t,
    249 as i32 as uint8_t,
    206 as i32 as uint8_t,
    186 as i32 as uint8_t,
    197 as i32 as uint8_t,
    234 as i32 as uint8_t,
    38 as i32 as uint8_t,
    44 as i32 as uint8_t,
    83 as i32 as uint8_t,
    13 as i32 as uint8_t,
    110 as i32 as uint8_t,
    133 as i32 as uint8_t,
    40 as i32 as uint8_t,
    132 as i32 as uint8_t,
    9 as i32 as uint8_t,
    211 as i32 as uint8_t,
    223 as i32 as uint8_t,
    205 as i32 as uint8_t,
    244 as i32 as uint8_t,
    65 as i32 as uint8_t,
    129 as i32 as uint8_t,
    77 as i32 as uint8_t,
    82 as i32 as uint8_t,
    106 as i32 as uint8_t,
    220 as i32 as uint8_t,
    55 as i32 as uint8_t,
    200 as i32 as uint8_t,
    108 as i32 as uint8_t,
    193 as i32 as uint8_t,
    171 as i32 as uint8_t,
    250 as i32 as uint8_t,
    36 as i32 as uint8_t,
    225 as i32 as uint8_t,
    123 as i32 as uint8_t,
    8 as i32 as uint8_t,
    12 as i32 as uint8_t,
    189 as i32 as uint8_t,
    177 as i32 as uint8_t,
    74 as i32 as uint8_t,
    120 as i32 as uint8_t,
    136 as i32 as uint8_t,
    149 as i32 as uint8_t,
    139 as i32 as uint8_t,
    227 as i32 as uint8_t,
    99 as i32 as uint8_t,
    232 as i32 as uint8_t,
    109 as i32 as uint8_t,
    233 as i32 as uint8_t,
    203 as i32 as uint8_t,
    213 as i32 as uint8_t,
    254 as i32 as uint8_t,
    59 as i32 as uint8_t,
    0 as i32 as uint8_t,
    29 as i32 as uint8_t,
    57 as i32 as uint8_t,
    242 as i32 as uint8_t,
    239 as i32 as uint8_t,
    183 as i32 as uint8_t,
    14 as i32 as uint8_t,
    102 as i32 as uint8_t,
    88 as i32 as uint8_t,
    208 as i32 as uint8_t,
    228 as i32 as uint8_t,
    166 as i32 as uint8_t,
    119 as i32 as uint8_t,
    114 as i32 as uint8_t,
    248 as i32 as uint8_t,
    235 as i32 as uint8_t,
    117 as i32 as uint8_t,
    75 as i32 as uint8_t,
    10 as i32 as uint8_t,
    49 as i32 as uint8_t,
    68 as i32 as uint8_t,
    80 as i32 as uint8_t,
    180 as i32 as uint8_t,
    143 as i32 as uint8_t,
    237 as i32 as uint8_t,
    31 as i32 as uint8_t,
    26 as i32 as uint8_t,
    219 as i32 as uint8_t,
    153 as i32 as uint8_t,
    141 as i32 as uint8_t,
    51 as i32 as uint8_t,
    159 as i32 as uint8_t,
    17 as i32 as uint8_t,
    131 as i32 as uint8_t,
    20 as i32 as uint8_t,
];
unsafe extern "C" fn md2_transform(mut ctx: *mut md2_ctx, mut data: *const uint8_t) {
    let mut i: u32 = 0;
    let mut t: uint8_t = 0;
    memcpy(
        ((*ctx).X).as_mut_ptr().offset(16 as i32 as isize) as *mut libc::c_void,
        data as *const libc::c_void,
        16 as i32 as u64,
    );
    i = 0 as i32 as u32;
    t = (*ctx).C[15 as i32 as usize];
    while i < 16 as i32 as u32 {
        (*ctx).X[((2 as i32 * 16 as i32) as u32).wrapping_add(i) as usize] = ((*ctx)
            .X[i as usize] as i32
            ^ (*ctx).X[(16 as i32 as u32).wrapping_add(i) as usize] as i32) as uint8_t;
        (*ctx).C[i as usize] = ((*ctx).C[i as usize] as i32
            ^ S[(*data.offset(i as isize) as i32 ^ t as i32) as usize] as i32)
            as uint8_t;
        t = (*ctx).C[i as usize];
        i = i.wrapping_add(1);
        i;
    }
    t = 0 as i32 as uint8_t;
    i = t as u32;
    while i < (16 as i32 + 2 as i32) as u32 {
        let mut j: u32 = 0;
        j = 0 as i32 as u32;
        while j < (3 as i32 * 16 as i32) as u32 {
            (*ctx).X[j as usize] = ((*ctx).X[j as usize] as i32 ^ S[t as usize] as i32)
                as uint8_t;
            t = (*ctx).X[j as usize];
            j = j.wrapping_add(1);
            j;
        }
        t = ((t as u32).wrapping_add(i) & 0xff as i32 as u32) as uint8_t;
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn nettle_md2_init(mut ctx: *mut md2_ctx) {
    memset(ctx as *mut libc::c_void, 0 as i32, ::core::mem::size_of::<md2_ctx>() as u64);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_md2_update(
    mut ctx: *mut md2_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    let mut current_block: u64;
    if !(length == 0) {
        if (*ctx).index != 0 {
            let mut __md_left: u32 = (::core::mem::size_of::<[uint8_t; 16]>() as u64)
                .wrapping_sub((*ctx).index as u64) as u32;
            if length < __md_left as u64 {
                memcpy(
                    ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                        as *mut libc::c_void,
                    data as *const libc::c_void,
                    length,
                );
                (*ctx).index = ((*ctx).index as u64).wrapping_add(length) as u32 as u32;
                current_block = 15652330335145281839;
            } else {
                memcpy(
                    ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                        as *mut libc::c_void,
                    data as *const libc::c_void,
                    __md_left as u64,
                );
                md2_transform(ctx, ((*ctx).block).as_mut_ptr());
                data = data.offset(__md_left as isize);
                length = (length as u64).wrapping_sub(__md_left as u64) as size_t
                    as size_t;
                current_block = 11812396948646013369;
            }
        } else {
            current_block = 11812396948646013369;
        }
        match current_block {
            15652330335145281839 => {}
            _ => {
                while length >= ::core::mem::size_of::<[uint8_t; 16]>() as u64 {
                    md2_transform(ctx, data);
                    data = data
                        .offset(::core::mem::size_of::<[uint8_t; 16]>() as u64 as isize);
                    length = (length as u64)
                        .wrapping_sub(::core::mem::size_of::<[uint8_t; 16]>() as u64)
                        as size_t as size_t;
                }
                memcpy(
                    ((*ctx).block).as_mut_ptr() as *mut libc::c_void,
                    data as *const libc::c_void,
                    length,
                );
                (*ctx).index = length as u32;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn nettle_md2_digest(
    mut ctx: *mut md2_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    let mut left: u32 = 0;
    if length <= 16 as i32 as u64 {} else {
        __assert_fail(
            b"length <= MD2_DIGEST_SIZE\0" as *const u8 as *const i8,
            b"md2.c\0" as *const u8 as *const i8,
            130 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 60],
                &[i8; 60],
            >(b"void nettle_md2_digest(struct md2_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_1492: {
        if length <= 16 as i32 as u64 {} else {
            __assert_fail(
                b"length <= MD2_DIGEST_SIZE\0" as *const u8 as *const i8,
                b"md2.c\0" as *const u8 as *const i8,
                130 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 60],
                    &[i8; 60],
                >(b"void nettle_md2_digest(struct md2_ctx *, size_t, uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    left = (16 as i32 as u32).wrapping_sub((*ctx).index);
    memset(
        ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize) as *mut libc::c_void,
        left as i32,
        left as u64,
    );
    md2_transform(ctx, ((*ctx).block).as_mut_ptr());
    md2_transform(ctx, ((*ctx).C).as_mut_ptr());
    memcpy(
        digest as *mut libc::c_void,
        ((*ctx).X).as_mut_ptr() as *const libc::c_void,
        length,
    );
    nettle_md2_init(ctx);
}