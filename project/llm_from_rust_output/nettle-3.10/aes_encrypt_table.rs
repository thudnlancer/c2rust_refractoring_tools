#[derive(Copy, Clone)]
pub struct AesTable {
    pub sbox: [u8; 256],
    pub table: [[u32; 256]; 4],
}

pub const NETTLE_AES_ENCRYPT_TABLE: AesTable = AesTable {
    sbox: [
        0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 
        0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
        0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 
        0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
        0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 
        0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
        0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 
        0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
        0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 
        0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
        0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 
        0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
        0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 
        0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
        0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 
        0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
        0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 
        0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
        0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 
        0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
        0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 
        0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
        0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 
        0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
        0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 
        0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
        0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 
        0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
        0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 
        0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
        0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 
        0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16
    ],
    table: [
        [
            0xa56363c6, 0x847c7cf8, 0x997777ee, 0x8d7b7bf6,
            0x0df2f2ff, 0xbd6b6bd6, 0xb16f6fde, 0x54c5c591,
            0x50303060, 0x03010102, 0xa96767ce, 0x7d2b2b56,
            0x19fefee7, 0x62d7d7b5, 0xe6abab4d, 0x9a7676ec,
            0x45caca8f, 0x9d82821f, 0x40c9c989, 0x877d7dfa,
            0x15fafaef, 0xeb5959b2, 0xc947478e, 0x0bf0f0fb,
            0xecadad41, 0x67d4d4b3, 0xfda2a25f, 0xeaafaf45,
            0xbf9c9c23, 0xf7a4a453, 0x967272e4, 0x5bc0c09b,
            0xc2b7b775, 0x1cfdfde1, 0xae93933d, 0x6a26264c,
            0x5a36366c, 0x413f3f7e, 0x02f7f7f5, 0x4fcccc83,
            0x5c343468, 0xf4a5a551, 0x34e5e5d1, 0x08f1f1f9,
            0x937171e2, 0x73d8d8ab, 0x53313162, 0x3f15152a,
            0x0c040408, 0x52c7c795, 0x65232346, 0x5ec3c39d,
            0x28181830, 0xa1969637, 0x0f05050a, 0xb59a9a2f,
            0x0907070e, 0x36121224, 0x9b80801b, 0x3de2e2df,
            0x26ebebcd, 0x6927274e, 0xcdb2b27f, 0x9f7575ea,
            0x1b090912, 0x9e83831d, 0x742c2c58, 0x2e1a1a34,
            0x2d1b1b36, 0xb26e6edc, 0xee5a5ab4, 0xfba0a05b,
            0xf65252a4, 0x4d3b3b76, 0x61d6d6b7, 0xceb3b37d,
            0x7b292952, 0x3ee3e3dd, 0x712f2f5e, 0x97848413,
            0xf55353a6, 0x68d1d1b9, 0x00000000, 0x2cededc1,
            0x60202040, 0x1ffcfce3, 0xc8b1b179, 0xed5b5bb6,
            0xbe6a6ad4, 0x46cbcb8d, 0xd9bebe67, 0x4b393972,
            0xde4a4a94, 0xd44c4c98, 0xe85858b0, 0x4acfcf85,
            0x6bd0d0bb, 0x2aefefc5, 0xe5aaaa4f, 0x16fbfbed,
            0xc5434386, 0xd74d4d9a, 0x55333366, 0x94858511,
            0xcf45458a, 0x10f9f9e9, 0x06020204, 0x817f7ffe,
            0xf05050a0, 0x443c3c78, 0xba9f9f25, 0xe3a8a84b,
            0xf35151a2, 0xfea3a35d, 0xc0404080, 0x8a8f8f05,
            0xad92923f, 0xbc9d9d21, 0x48383870, 0x04f5f5f1,
            0xdfbcbc63, 0xc1b6b677, 0x75dadaaf, 0x63212142,
            0x30101020, 0x1affffe5, 0x0ef3f3fd, 0x6dd2d2bf,
            0x4ccdcd81, 0x140c0c18, 0x35131326, 0x2fececc3,
            0xe15f5fbe, 0xa2979735, 0xcc444488, 0x3917172e,
            0x57c4c493, 0xf2a7a755, 0x827e7efc, 0x473d3d7a,
            0xac6464c8, 0xe75d5dba, 0x2b191932, 0x957373e6,
            0xa06060c0, 0x98818119, 0xd14f4f9e, 0x7fdcdca3,
            0x66222244, 0x7e2a2a54, 0xab90903b, 0x8388880b,
            0xca46468c, 0x29eeeec7, 0xd3b8b86b, 0x3c141428,
            0x79dedea7, 0xe25e5ebc, 0x1d0b0b16, 0x76dbdbad,
            0x3be0e0db, 0x56323264, 0x4e3a3a74, 0x1e0a0a14,
            0xdb494992, 0x0a06060c, 0x6c242448, 0xe45c5cb8,
            0x5dc2c29f, 0x6ed3d3bd, 0xefacac43, 0xa66262c4,
            0xa8919139, 0xa4959531, 0x37e4e4d3, 0x8b7979f2,
            0x32e7e7d5, 0x43c8c88b, 0x5937376e, 0xb76d6dda,
            0x8c8d8d01, 0x64d5d5b1, 0xd24e4e9c, 0xe0a9a949,
            0xb46c6cd8, 0xfa5656ac, 0x07f4f4f3, 0x25eaeacf,
            0xaf6565ca, 0x8e7a7af4, 0xe9aeae47, 0x18080810,
            0xd5baba6f, 0x887878f0, 0x6f25254a, 0x722e2e5c,
            0x241c1c38, 0xf1a6a657, 0xc7b4b473, 0x51c6c697,
            0x23e8e8cb, 0x7cdddda1, 0x9c7474e8, 0x211f1f3e,
            0xdd4b4b96, 0xdcbdbd61, 0x868b8b0d, 0x858a8a0f,
            0x907070e0, 0x423e3e7c, 0xc4b5b571, 0xaa6666cc,
            0xd8484890, 0x05030306, 0x01f6f6f7, 0x120e0e1c,
            0xa36161c2, 0x5f35356a, 0xf95757ae, 0xd0b9b969,
            0x91868617, 0x58c1c199, 0x271d1d3a, 0xb99e9e27,
            0x38e1e1d9, 0x13f8f8eb, 0xb398982b, 0x33111122,
            0xbb6969d2, 0x70d9d9a9, 0x898e8e07, 0xa7949433,
            0xb69b9b2d, 0x221e1e3c, 0x92878715, 0x20e9e9c9,
            0x49cece87, 0xff5555aa, 0x78282850, 0x7adfdfa5,
            0x8f8c8c03, 0xf8a1a159, 0x80898909, 0x170d0d1a,
            0xdabfbf65, 0x31e6e6d7, 0xc6424284, 0xb86868d0,
            0xc3414182, 0xb0999929, 0x772d2d5a, 0x110f0f1e,
            0xcbb0b07b, 0xfc5454a8, 0xd6bbbb6d, 0x3a16162c
        ],
        [
            0x6363c6a5, 0x7c7cf884, 0x7777ee99, 0x7b7bf68d,
            0xf2f2ff0d, 0x6b6bd6bd, 0x6f6fdeb1, 0xc5c59154,
            0x30306050, 0x01010203, 0x6767cea9, 0x2b2b567d,
            0xfefee719, 0xd7d7b562, 0xabab4de6, 0x7676ec9a,
            0xcaca8f45, 0x82821f9d, 0xc9c98940, 0x7d7dfa87,
            0xfafaef15, 0x5959b2eb, 0x47478ec9, 0xf0f0fb0b,
            0xadad41ec, 0xd4d4b367, 0xa2a25ffd, 0xafaf45ea,
            0x9c9c23bf, 0xa4a453f7, 0x7272e496, 0xc0c09b5b,
            0xb7b775c2, 0xfdfde11c, 0x93933dae, 0x26264c6a,
            0x36366c5a, 0x3f3f7e41, 0xf7f7f502, 0xcccc834f,
            0x3434685c, 0xa5a551f4, 0xe5e5d134, 0xf1f1f908