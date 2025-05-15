use std::mem;

#[derive(Debug, Clone, Copy)]
struct CtData {
    code: u16,
    len: u16,
}

const L_CODES: usize = 286;
const D_CODES: usize = 30;
const BL_CODES: usize = 19;
const MAX_BITS: usize = 15;
const MAX_BL_BITS: usize = 7;
const LENGTH_CODES: usize = 29;
const DIST_CODE_LEN: usize = 512;
const MAX_MATCH: usize = 258;
const MIN_MATCH: usize = 3;
const END_BLOCK: usize = 256;
const REP_3_6: usize = 16;
const REPZ_3_10: usize = 17;
const REPZ_11_138: usize = 18;
const HEAP_SIZE: usize = (2 * L_CODES + 1);
const Buf_size: usize = 8 * 2 * mem::size_of::<u8>();

static STATIC_LTREE: [CtData; L_CODES + 2] = [
    CtData { code: 12, len: 8 }, CtData { code: 140, len: 8 }, CtData { code: 76, len: 8 },
    CtData { code: 204, len: 8 }, CtData { code: 44, len: 8 }, CtData { code: 172, len: 8 },
    CtData { code: 108, len: 8 }, CtData { code: 236, len: 8 }, CtData { code: 28, len: 8 },
    CtData { code: 156, len: 8 }, CtData { code: 92, len: 8 }, CtData { code: 220, len: 8 },
    CtData { code: 60, len: 8 }, CtData { code: 188, len: 8 }, CtData { code: 124, len: 8 },
    CtData { code: 252, len: 8 }, CtData { code: 2, len: 8 }, CtData { code: 130, len: 8 },
    CtData { code: 66, len: 8 }, CtData { code: 194, len: 8 }, CtData { code: 34, len: 8 },
    CtData { code: 162, len: 8 }, CtData { code: 98, len: 8 }, CtData { code: 226, len: 8 },
    CtData { code: 18, len: 8 }, CtData { code: 146, len: 8 }, CtData { code: 82, len: 8 },
    CtData { code: 210, len: 8 }, CtData { code: 50, len: 8 }, CtData { code: 178, len: 8 },
    CtData { code: 114, len: 8 }, CtData { code: 242, len: 8 }, CtData { code: 10, len: 8 },
    CtData { code: 138, len: 8 }, CtData { code: 74, len: 8 }, CtData { code: 202, len: 8 },
    CtData { code: 42, len: 8 }, CtData { code: 170, len: 8 }, CtData { code: 106, len: 8 },
    CtData { code: 234, len: 8 }, CtData { code: 26, len: 8 }, CtData { code: 154, len: 8 },
    CtData { code: 90, len: 8 }, CtData { code: 218, len: 8 }, CtData { code: 58, len: 8 },
    CtData { code: 186, len: 8 }, CtData { code: 122, len: 8 }, CtData { code: 250, len: 8 },
    CtData { code: 6, len: 8 }, CtData { code: 134, len: 8 }, CtData { code: 70, len: 8 },
    CtData { code: 198, len: 8 }, CtData { code: 38, len: 8 }, CtData { code: 166, len: 8 },
    CtData { code: 102, len: 8 }, CtData { code: 230, len: 8 }, CtData { code: 22, len: 8 },
    CtData { code: 150, len: 8 }, CtData { code: 86, len: 8 }, CtData { code: 214, len: 8 },
    CtData { code: 54, len: 8 }, CtData { code: 182, len: 8 }, CtData { code: 118, len: 8 },
    CtData { code: 246, len: 8 }, CtData { code: 14, len: 8 }, CtData { code: 142, len: 8 },
    CtData { code: 78, len: 8 }, CtData { code: 206, len: 8 }, CtData { code: 46, len: 8 },
    CtData { code: 174, len: 8 }, CtData { code: 110, len: 8 }, CtData { code: 238, len: 8 },
    CtData { code: 30, len: 8 }, CtData { code: 158, len: 8 }, CtData { code: 94, len: 8 },
    CtData { code: 222, len: 8 }, CtData { code: 62, len: 8 }, CtData { code: 190, len: 8 },
    CtData { code: 126, len: 8 }, CtData { code: 254, len: 8 }, CtData { code: 1, len: 8 },
    CtData { code: 129, len: 8 }, CtData { code: 65, len: 8 }, CtData { code: 193, len: 8 },
    CtData { code: 33, len: 8 }, CtData { code: 161, len: 8 }, CtData { code: 97, len: 8 },
    CtData { code: 225, len: 8 }, CtData { code: 17, len: 8 }, CtData { code: 145, len: 8 },
    CtData { code: 81, len: 8 }, CtData { code: 209, len: 8 }, CtData { code: 49, len: 8 },
    CtData { code: 177, len: 8 }, CtData { code: 113, len: 8 }, CtData { code: 241, len: 8 },
    CtData { code: 9, len: 8 }, CtData { code: 137, len: 8 }, CtData { code: 73, len: 8 },
    CtData { code: 201, len: 8 }, CtData { code: 41, len: 8 }, CtData { code: 169, len: 8 },
    CtData { code: 105, len: 8 }, CtData { code: 233, len: 8 }, CtData { code: 25, len: 8 },
    CtData { code: 153, len: 8 }, CtData { code: 89, len: 8 }, CtData { code: 217, len: 8 },
    CtData { code: 57, len: 8 }, CtData { code: 185, len: 8 }, CtData { code: 121, len: 8 },
    CtData { code: 249, len: 8 }, CtData { code: 5, len: 8 }, CtData { code: 133, len: 8 },
    CtData { code: 69, len: 8 }, CtData { code: 197, len: 8 }, CtData { code: 37, len: 8 },
    CtData { code: 165, len: 8 }, CtData { code: 101, len: 8 }, CtData { code: 229, len: 8 },
    CtData { code: 21, len: 8 }, CtData { code: 149, len: 8 }, CtData { code: 85, len: 8 },
    CtData { code: 213, len: 8 }, CtData { code: 53, len: 8 }, CtData { code: 181, len: 8 },
    CtData { code: 117, len: 8 }, CtData { code: 245, len: 8 }, CtData { code: 13, len: 8 },
    CtData { code: 141, len: 8 }, CtData { code: 77, len: 8 }, CtData { code: 205, len: 8 },
    CtData { code: 45, len: 8 }, CtData { code: 173, len: 8 }, CtData { code: 109, len: 8 },
    CtData { code: 237, len: 8 }, CtData { code: 29, len: 8 }, CtData { code: 157, len: 8 },
    CtData { code: 93, len: 8 }, CtData { code: 221, len: 8 }, CtData { code: 61, len: 8 },
    CtData { code: 189, len: 8 }, CtData { code: 125, len: 8 }, CtData { code: 253, len: 8 },
    CtData { code: 19, len: 9 }, CtData { code: 275, len: 9 }, CtData { code: 147, len: 9 },
    CtData { code: 403, len: 9 }, CtData { code: 83, len: 9 }, CtData { code: 339, len: 9 },
    CtData { code: 211, len: 9 }, CtData { code: 467, len: 9 }, CtData { code: 51, len: 9 },
    CtData { code: 307, len: 9 }, CtData { code: 179, len: 9 }, CtData { code: 435, len: 9 },
    CtData { code: 115, len: 9 }, CtData { code: 371, len: 9 }, CtData { code: 243, len: 9 },
    CtData { code: 499, len: 9 }, CtData { code: 11, len: 9 }, CtData { code: 267, len: 9 },
    CtData { code: 139, len: 9 }, CtData { code: 395, len: 9 }, CtData { code: 75, len: 9 },
    CtData { code: 331, len: 9 }, CtData { code: 203, len: 9 }, CtData { code: 459, len: 9 },
    CtData { code: 43, len: 9 }, CtData { code: 299, len: 9 }, CtData { code: 171, len: 9 },
    CtData { code: 427, len: 9 }, CtData { code: 107, len: 9 }, CtData { code: 363, len: 9 },
    CtData { code: 235, len: 9 }, CtData { code: 491, len: 9 }, CtData { code: 27, len: 9 },
    CtData { code: 283, len: 9 }, CtData { code: 155, len: 9 }, CtData { code: 411, len: 9 },
    CtData { code: 91, len: 9 }, CtData { code: 347, len: 9 }, CtData { code: 219, len: 9 },
    CtData { code: 475, len: 9 }, CtData { code: 59, len: 9 }, CtData { code: 315, len: 9 },
    CtData { code: 187, len: 9 }, CtData { code: 443, len: 9 }, CtData { code: 123, len: 9 },
    CtData { code: 379, len: 9 }, CtData { code: 251, len: 9 }, CtData { code: 507, len: 9 },
    CtData { code: 7, len: 9 }, CtData { code: 263, len: 9 }, CtData { code: 135, len: 9 },
    CtData { code: 391, len: 9 }, CtData { code: 71, len: 9 }, CtData { code: 327, len: 9 },
    CtData { code: 199, len: 9 }, CtData { code: 455, len: 9 }, CtData { code: 39, len: 9 },
    CtData { code: 295, len: 9 }, CtData { code: 167, len: 9 }, CtData { code: 423, len: 9 },
    CtData { code: 103, len: 9 }, CtData { code: 359, len: 9 }, CtData { code: 231, len: 9 },
    CtData { code: 487, len: 9 }, CtData { code: 23, len: 9 }, CtData { code: 279, len: 9 },
    CtData { code: 151, len: 9 }, CtData { code: 407, len: 9 }, CtData { code: 87, len: 9 },
    CtData { code: 343, len: 9 }, CtData { code: 215, len: 9 }, CtData { code: 471, len: 9 },
    CtData { code: 55, len: 9 }, CtData { code: 311, len: 9 }, CtData { code: 183, len: 9 },
    CtData { code: 439, len: 9 }, CtData { code: 119, len: 9 }, CtData { code: 375, len: 9 },
    CtData { code: 247, len: 9 }, CtData { code: 503, len: 9 }, CtData { code: 15, len: 9 },
    CtData { code: 271, len: 9 }, CtData { code: 143, len: 9 }, CtData { code: 399, len: 9 },
    CtData { code: 79, len: 9 }, CtData { code: 335, len: 9 }, CtData { code: 207, len: 9 },
    CtData { code: 463, len: 9 }, CtData { code: 47, len: 9 }, CtData { code: 303, len: 9 },
    CtData { code: 175, len: 9 }, CtData { code: 431, len: 9 }, CtData { code: 111, len: 9 },
    CtData { code: 367, len: 9 }, CtData { code: 239, len: 9 }, CtData { code: 495, len: 9 },
    CtData { code: 31, len: 9 }, CtData { code: 287, len: 9 }, CtData { code: 159, len: 9 },
    CtData { code: 415, len: 9 }, CtData { code: 95, len: 9 }, CtData { code: 351, len: 9 },
    CtData { code: 223, len: 9 }, CtData { code: 479, len: 9 }, CtData { code: 63, len: 9 },
    CtData { code: 319, len: 9 }, CtData { code: 191, len: 9 }, CtData { code: 447, len: 9 },
    CtData { code: 127, len: 9 }, CtData { code: 383, len: 9 }, CtData { code: 255, len: 9 },
    CtData { code: 511, len: 9 }, CtData { code: 0, len: 7 }, CtData { code: 64, len: 7 },
    CtData { code: 32, len: 7 }, CtData { code: 96, len: 7 }, CtData { code: 16, len: 7 },
    CtData { code: 80, len: 7 }, CtData { code: 48, len: 7 }, CtData { code: 112, len: 7 },
    CtData { code: 8, len: 7 }, CtData { code: 72, len: 7 }, CtData { code: 40, len: 7 },
    CtData { code: 104, len: 7 }, CtData { code: 24, len: 7 }, CtData { code: 88, len: 7 },
    CtData { code: 56, len: 7 }, CtData { code: 120, len: 7 }, CtData { code: 4, len: 7 },
    CtData { code: 68, len: 7 }, CtData { code: 36, len: 7 }, CtData { code: 100, len: 7 },
    CtData { code: 20, len: 7 }, CtData { code: 84, len: 7 }, CtData { code: 52, len: 7 },
    CtData { code: 116, len: 7 }, CtData { code: 3, len: 8 }, CtData { code: 131, len: 8 },
    CtData { code: 67, len: 8 }, CtData { code: 195, len: 8 }, CtData { code: 35, len: 8 },
    CtData { code: 163, len: 8 }, CtData { code: 99, len: 8 }, CtData { code: 227, len: 8 },
];

static STATIC_DTREE: [CtData; D_CODES] = [
    CtData