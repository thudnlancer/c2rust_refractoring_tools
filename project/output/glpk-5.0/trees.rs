#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type Byte = u8;
pub type uInt = u32;
pub type uLong = u64;
pub type Bytef = Byte;
pub type charf = i8;
pub type intf = i32;
pub type voidpf = *mut libc::c_void;
pub type alloc_func = Option<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct internal_state {
    pub strm: z_streamp,
    pub status: i32,
    pub pending_buf: *mut Bytef,
    pub pending_buf_size: ulg,
    pub pending_out: *mut Bytef,
    pub pending: uInt,
    pub wrap: i32,
    pub gzhead: gz_headerp,
    pub gzindex: uInt,
    pub method: Byte,
    pub last_flush: i32,
    pub w_size: uInt,
    pub w_bits: uInt,
    pub w_mask: uInt,
    pub window: *mut Bytef,
    pub window_size: ulg,
    pub prev: *mut Posf,
    pub head: *mut Posf,
    pub ins_h: uInt,
    pub hash_size: uInt,
    pub hash_bits: uInt,
    pub hash_mask: uInt,
    pub hash_shift: uInt,
    pub block_start: i64,
    pub match_length: uInt,
    pub prev_match: IPos,
    pub match_available: i32,
    pub strstart: uInt,
    pub match_start: uInt,
    pub lookahead: uInt,
    pub prev_length: uInt,
    pub max_chain_length: uInt,
    pub max_lazy_match: uInt,
    pub level: i32,
    pub strategy: i32,
    pub good_match: uInt,
    pub nice_match: i32,
    pub dyn_ltree: [ct_data_s; 573],
    pub dyn_dtree: [ct_data_s; 61],
    pub bl_tree: [ct_data_s; 39],
    pub l_desc: tree_desc_s,
    pub d_desc: tree_desc_s,
    pub bl_desc: tree_desc_s,
    pub bl_count: [ush; 16],
    pub heap: [i32; 573],
    pub heap_len: i32,
    pub heap_max: i32,
    pub depth: [uch; 573],
    pub l_buf: *mut uchf,
    pub lit_bufsize: uInt,
    pub last_lit: uInt,
    pub d_buf: *mut ushf,
    pub opt_len: ulg,
    pub static_len: ulg,
    pub matches: uInt,
    pub last_eob_len: i32,
    pub bi_buf: ush,
    pub bi_valid: i32,
    pub high_water: ulg,
}
pub type ulg = u64;
pub type ush = libc::c_ushort;
pub type ushf = ush;
pub type uchf = uch;
pub type uch = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tree_desc_s {
    pub dyn_tree: *mut ct_data,
    pub max_code: i32,
    pub stat_desc: *mut static_tree_desc,
}
pub type static_tree_desc = static_tree_desc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct static_tree_desc_s {
    pub static_tree: *const ct_data,
    pub extra_bits: *const intf,
    pub extra_base: i32,
    pub elems: i32,
    pub max_length: i32,
}
pub type ct_data = ct_data_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ct_data_s {
    pub fc: C2RustUnnamed_0,
    pub dl: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub dad: ush,
    pub len: ush,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub freq: ush,
    pub code: ush,
}
pub type IPos = u32;
pub type Posf = Pos;
pub type Pos = ush;
pub type gz_headerp = *mut gz_header;
pub type gz_header = gz_header_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gz_header_s {
    pub text: i32,
    pub time: uLong,
    pub xflags: i32,
    pub os: i32,
    pub extra: *mut Bytef,
    pub extra_len: uInt,
    pub extra_max: uInt,
    pub name: *mut Bytef,
    pub name_max: uInt,
    pub comment: *mut Bytef,
    pub comm_max: uInt,
    pub hcrc: i32,
    pub done: i32,
}
pub type z_streamp = *mut z_stream;
pub type z_stream = z_stream_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut i8,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: i32,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type tree_desc = tree_desc_s;
pub type deflate_state = internal_state;
static mut extra_lbits: [i32; 29] = [
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    2 as i32,
    2 as i32,
    2 as i32,
    2 as i32,
    3 as i32,
    3 as i32,
    3 as i32,
    3 as i32,
    4 as i32,
    4 as i32,
    4 as i32,
    4 as i32,
    5 as i32,
    5 as i32,
    5 as i32,
    5 as i32,
    0 as i32,
];
static mut extra_dbits: [i32; 30] = [
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    1 as i32,
    1 as i32,
    2 as i32,
    2 as i32,
    3 as i32,
    3 as i32,
    4 as i32,
    4 as i32,
    5 as i32,
    5 as i32,
    6 as i32,
    6 as i32,
    7 as i32,
    7 as i32,
    8 as i32,
    8 as i32,
    9 as i32,
    9 as i32,
    10 as i32,
    10 as i32,
    11 as i32,
    11 as i32,
    12 as i32,
    12 as i32,
    13 as i32,
    13 as i32,
];
static mut extra_blbits: [i32; 19] = [
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    0 as i32,
    2 as i32,
    3 as i32,
    7 as i32,
];
static mut bl_order: [uch; 19] = [
    16 as i32 as uch,
    17 as i32 as uch,
    18 as i32 as uch,
    0 as i32 as uch,
    8 as i32 as uch,
    7 as i32 as uch,
    9 as i32 as uch,
    6 as i32 as uch,
    10 as i32 as uch,
    5 as i32 as uch,
    11 as i32 as uch,
    4 as i32 as uch,
    12 as i32 as uch,
    3 as i32 as uch,
    13 as i32 as uch,
    2 as i32 as uch,
    14 as i32 as uch,
    1 as i32 as uch,
    15 as i32 as uch,
];
static mut static_ltree: [ct_data; 288] = [
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 12 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 140 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 76 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 204 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 44 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 172 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 108 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 236 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 28 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 156 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 92 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 220 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 60 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 188 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 124 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 252 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 2 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 130 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 66 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 194 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 34 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 162 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 98 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 226 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 18 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 146 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 82 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 210 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 50 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 178 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 114 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 242 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 10 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 138 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 74 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 202 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 42 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 170 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 106 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 234 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 26 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 154 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 90 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 218 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 58 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 186 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 122 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 250 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 6 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 134 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 70 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 198 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 38 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 166 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 102 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 230 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 22 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 150 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 86 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 214 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 54 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 182 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 118 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 246 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 14 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 142 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 78 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 206 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 46 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 174 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 110 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 238 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 30 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 158 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 94 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 222 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 62 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 190 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 126 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 254 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 1 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 129 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 65 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 193 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 33 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 161 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 97 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 225 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 17 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 145 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 81 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 209 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 49 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 177 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 113 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 241 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 9 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 137 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 73 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 201 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 41 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 169 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 105 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 233 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 25 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 153 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 89 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 217 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 57 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 185 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 121 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 249 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 5 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 133 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 69 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 197 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 37 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 165 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 101 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 229 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 21 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 149 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 85 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 213 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 53 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 181 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 117 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 245 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 13 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 141 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 77 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 205 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 45 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 173 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 109 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 237 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 29 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 157 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 93 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 221 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 61 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 189 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 125 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 253 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 19 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 275 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 147 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 403 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 83 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 339 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 211 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 467 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 51 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 307 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 179 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 435 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 115 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 371 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 243 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 499 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 11 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 267 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 139 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 395 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 75 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 331 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 203 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 459 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 43 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 299 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 171 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 427 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 107 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 363 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 235 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 491 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 27 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 283 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 155 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 411 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 91 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 347 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 219 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 475 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 59 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 315 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 187 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 443 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 123 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 379 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 251 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 507 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 7 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 263 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 135 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 391 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 71 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 327 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 199 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 455 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 39 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 295 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 167 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 423 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 103 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 359 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 231 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 487 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 23 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 279 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 151 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 407 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 87 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 343 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 215 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 471 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 55 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 311 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 183 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 439 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 119 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 375 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 247 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 503 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 15 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 271 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 143 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 399 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 79 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 335 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 207 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 463 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 47 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 303 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 175 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 431 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 111 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 367 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 239 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 495 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 31 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 287 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 159 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 415 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 95 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 351 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 223 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 479 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 63 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 319 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 191 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 447 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 127 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 383 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 255 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 511 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 0 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 64 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 32 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 96 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 16 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 80 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 48 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 112 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 8 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 72 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 40 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 104 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 24 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 88 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 56 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 120 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 4 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 68 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 36 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 100 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 20 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 84 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 52 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 116 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 3 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 131 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 67 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 195 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 35 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 163 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 99 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 227 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as i32 as ush,
            },
        };
        init
    },
];
static mut static_dtree: [ct_data; 30] = [
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 0 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 16 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 8 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 24 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 4 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 20 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 12 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 28 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 2 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 18 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 10 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 26 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 6 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 22 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 14 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 30 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 1 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 17 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 9 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 25 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 5 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 21 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 13 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 29 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 3 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 19 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 11 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 27 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 7 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 23 as i32 as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as i32 as ush,
            },
        };
        init
    },
];
#[no_mangle]
pub static mut _glp_zlib_dist_code: [uch; 512] = [
    0 as i32 as uch,
    1 as i32 as uch,
    2 as i32 as uch,
    3 as i32 as uch,
    4 as i32 as uch,
    4 as i32 as uch,
    5 as i32 as uch,
    5 as i32 as uch,
    6 as i32 as uch,
    6 as i32 as uch,
    6 as i32 as uch,
    6 as i32 as uch,
    7 as i32 as uch,
    7 as i32 as uch,
    7 as i32 as uch,
    7 as i32 as uch,
    8 as i32 as uch,
    8 as i32 as uch,
    8 as i32 as uch,
    8 as i32 as uch,
    8 as i32 as uch,
    8 as i32 as uch,
    8 as i32 as uch,
    8 as i32 as uch,
    9 as i32 as uch,
    9 as i32 as uch,
    9 as i32 as uch,
    9 as i32 as uch,
    9 as i32 as uch,
    9 as i32 as uch,
    9 as i32 as uch,
    9 as i32 as uch,
    10 as i32 as uch,
    10 as i32 as uch,
    10 as i32 as uch,
    10 as i32 as uch,
    10 as i32 as uch,
    10 as i32 as uch,
    10 as i32 as uch,
    10 as i32 as uch,
    10 as i32 as uch,
    10 as i32 as uch,
    10 as i32 as uch,
    10 as i32 as uch,
    10 as i32 as uch,
    10 as i32 as uch,
    10 as i32 as uch,
    10 as i32 as uch,
    11 as i32 as uch,
    11 as i32 as uch,
    11 as i32 as uch,
    11 as i32 as uch,
    11 as i32 as uch,
    11 as i32 as uch,
    11 as i32 as uch,
    11 as i32 as uch,
    11 as i32 as uch,
    11 as i32 as uch,
    11 as i32 as uch,
    11 as i32 as uch,
    11 as i32 as uch,
    11 as i32 as uch,
    11 as i32 as uch,
    11 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    0 as i32 as uch,
    0 as i32 as uch,
    16 as i32 as uch,
    17 as i32 as uch,
    18 as i32 as uch,
    18 as i32 as uch,
    19 as i32 as uch,
    19 as i32 as uch,
    20 as i32 as uch,
    20 as i32 as uch,
    20 as i32 as uch,
    20 as i32 as uch,
    21 as i32 as uch,
    21 as i32 as uch,
    21 as i32 as uch,
    21 as i32 as uch,
    22 as i32 as uch,
    22 as i32 as uch,
    22 as i32 as uch,
    22 as i32 as uch,
    22 as i32 as uch,
    22 as i32 as uch,
    22 as i32 as uch,
    22 as i32 as uch,
    23 as i32 as uch,
    23 as i32 as uch,
    23 as i32 as uch,
    23 as i32 as uch,
    23 as i32 as uch,
    23 as i32 as uch,
    23 as i32 as uch,
    23 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    28 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
    29 as i32 as uch,
];
#[no_mangle]
pub static mut _glp_zlib_length_code: [uch; 256] = [
    0 as i32 as uch,
    1 as i32 as uch,
    2 as i32 as uch,
    3 as i32 as uch,
    4 as i32 as uch,
    5 as i32 as uch,
    6 as i32 as uch,
    7 as i32 as uch,
    8 as i32 as uch,
    8 as i32 as uch,
    9 as i32 as uch,
    9 as i32 as uch,
    10 as i32 as uch,
    10 as i32 as uch,
    11 as i32 as uch,
    11 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    12 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    13 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    14 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    15 as i32 as uch,
    16 as i32 as uch,
    16 as i32 as uch,
    16 as i32 as uch,
    16 as i32 as uch,
    16 as i32 as uch,
    16 as i32 as uch,
    16 as i32 as uch,
    16 as i32 as uch,
    17 as i32 as uch,
    17 as i32 as uch,
    17 as i32 as uch,
    17 as i32 as uch,
    17 as i32 as uch,
    17 as i32 as uch,
    17 as i32 as uch,
    17 as i32 as uch,
    18 as i32 as uch,
    18 as i32 as uch,
    18 as i32 as uch,
    18 as i32 as uch,
    18 as i32 as uch,
    18 as i32 as uch,
    18 as i32 as uch,
    18 as i32 as uch,
    19 as i32 as uch,
    19 as i32 as uch,
    19 as i32 as uch,
    19 as i32 as uch,
    19 as i32 as uch,
    19 as i32 as uch,
    19 as i32 as uch,
    19 as i32 as uch,
    20 as i32 as uch,
    20 as i32 as uch,
    20 as i32 as uch,
    20 as i32 as uch,
    20 as i32 as uch,
    20 as i32 as uch,
    20 as i32 as uch,
    20 as i32 as uch,
    20 as i32 as uch,
    20 as i32 as uch,
    20 as i32 as uch,
    20 as i32 as uch,
    20 as i32 as uch,
    20 as i32 as uch,
    20 as i32 as uch,
    20 as i32 as uch,
    21 as i32 as uch,
    21 as i32 as uch,
    21 as i32 as uch,
    21 as i32 as uch,
    21 as i32 as uch,
    21 as i32 as uch,
    21 as i32 as uch,
    21 as i32 as uch,
    21 as i32 as uch,
    21 as i32 as uch,
    21 as i32 as uch,
    21 as i32 as uch,
    21 as i32 as uch,
    21 as i32 as uch,
    21 as i32 as uch,
    21 as i32 as uch,
    22 as i32 as uch,
    22 as i32 as uch,
    22 as i32 as uch,
    22 as i32 as uch,
    22 as i32 as uch,
    22 as i32 as uch,
    22 as i32 as uch,
    22 as i32 as uch,
    22 as i32 as uch,
    22 as i32 as uch,
    22 as i32 as uch,
    22 as i32 as uch,
    22 as i32 as uch,
    22 as i32 as uch,
    22 as i32 as uch,
    22 as i32 as uch,
    23 as i32 as uch,
    23 as i32 as uch,
    23 as i32 as uch,
    23 as i32 as uch,
    23 as i32 as uch,
    23 as i32 as uch,
    23 as i32 as uch,
    23 as i32 as uch,
    23 as i32 as uch,
    23 as i32 as uch,
    23 as i32 as uch,
    23 as i32 as uch,
    23 as i32 as uch,
    23 as i32 as uch,
    23 as i32 as uch,
    23 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    24 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    25 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    26 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    27 as i32 as uch,
    28 as i32 as uch,
];
static mut base_length: [i32; 29] = [
    0 as i32,
    1 as i32,
    2 as i32,
    3 as i32,
    4 as i32,
    5 as i32,
    6 as i32,
    7 as i32,
    8 as i32,
    10 as i32,
    12 as i32,
    14 as i32,
    16 as i32,
    20 as i32,
    24 as i32,
    28 as i32,
    32 as i32,
    40 as i32,
    48 as i32,
    56 as i32,
    64 as i32,
    80 as i32,
    96 as i32,
    112 as i32,
    128 as i32,
    160 as i32,
    192 as i32,
    224 as i32,
    0 as i32,
];
static mut base_dist: [i32; 30] = [
    0 as i32,
    1 as i32,
    2 as i32,
    3 as i32,
    4 as i32,
    6 as i32,
    8 as i32,
    12 as i32,
    16 as i32,
    24 as i32,
    32 as i32,
    48 as i32,
    64 as i32,
    96 as i32,
    128 as i32,
    192 as i32,
    256 as i32,
    384 as i32,
    512 as i32,
    768 as i32,
    1024 as i32,
    1536 as i32,
    2048 as i32,
    3072 as i32,
    4096 as i32,
    6144 as i32,
    8192 as i32,
    12288 as i32,
    16384 as i32,
    24576 as i32,
];
static mut static_l_desc: static_tree_desc = unsafe {
    {
        let mut init = static_tree_desc_s {
            static_tree: static_ltree.as_ptr(),
            extra_bits: extra_lbits.as_ptr(),
            extra_base: 256 as i32 + 1 as i32,
            elems: 256 as i32 + 1 as i32 + 29 as i32,
            max_length: 15 as i32,
        };
        init
    }
};
static mut static_d_desc: static_tree_desc = unsafe {
    {
        let mut init = static_tree_desc_s {
            static_tree: static_dtree.as_ptr(),
            extra_bits: extra_dbits.as_ptr(),
            extra_base: 0 as i32,
            elems: 30 as i32,
            max_length: 15 as i32,
        };
        init
    }
};
static mut static_bl_desc: static_tree_desc = unsafe {
    {
        let mut init = static_tree_desc_s {
            static_tree: 0 as *const ct_data,
            extra_bits: extra_blbits.as_ptr(),
            extra_base: 0 as i32,
            elems: 19 as i32,
            max_length: 7 as i32,
        };
        init
    }
};
unsafe extern "C" fn tr_static_init() {}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_tr_init(mut s: *mut deflate_state) {
    tr_static_init();
    (*s).l_desc.dyn_tree = ((*s).dyn_ltree).as_mut_ptr();
    (*s).l_desc.stat_desc = &mut static_l_desc;
    (*s).d_desc.dyn_tree = ((*s).dyn_dtree).as_mut_ptr();
    (*s).d_desc.stat_desc = &mut static_d_desc;
    (*s).bl_desc.dyn_tree = ((*s).bl_tree).as_mut_ptr();
    (*s).bl_desc.stat_desc = &mut static_bl_desc;
    (*s).bi_buf = 0 as i32 as ush;
    (*s).bi_valid = 0 as i32;
    (*s).last_eob_len = 8 as i32;
    init_block(s);
}
unsafe extern "C" fn init_block(mut s: *mut deflate_state) {
    let mut n: i32 = 0;
    n = 0 as i32;
    while n < 256 as i32 + 1 as i32 + 29 as i32 {
        (*s).dyn_ltree[n as usize].fc.freq = 0 as i32 as ush;
        n += 1;
        n;
    }
    n = 0 as i32;
    while n < 30 as i32 {
        (*s).dyn_dtree[n as usize].fc.freq = 0 as i32 as ush;
        n += 1;
        n;
    }
    n = 0 as i32;
    while n < 19 as i32 {
        (*s).bl_tree[n as usize].fc.freq = 0 as i32 as ush;
        n += 1;
        n;
    }
    (*s).dyn_ltree[256 as i32 as usize].fc.freq = 1 as i32 as ush;
    (*s).static_len = 0 as i64 as ulg;
    (*s).opt_len = (*s).static_len;
    (*s).matches = 0 as i32 as uInt;
    (*s).last_lit = (*s).matches;
}
unsafe extern "C" fn pqdownheap(
    mut s: *mut deflate_state,
    mut tree: *mut ct_data,
    mut k: i32,
) {
    let mut v: i32 = (*s).heap[k as usize];
    let mut j: i32 = k << 1 as i32;
    while j <= (*s).heap_len {
        if j < (*s).heap_len
            && (((*tree.offset((*s).heap[(j + 1 as i32) as usize] as isize)).fc.freq
                as i32) < (*tree.offset((*s).heap[j as usize] as isize)).fc.freq as i32
                || (*tree.offset((*s).heap[(j + 1 as i32) as usize] as isize)).fc.freq
                    as i32
                    == (*tree.offset((*s).heap[j as usize] as isize)).fc.freq as i32
                    && (*s).depth[(*s).heap[(j + 1 as i32) as usize] as usize] as i32
                        <= (*s).depth[(*s).heap[j as usize] as usize] as i32)
        {
            j += 1;
            j;
        }
        if ((*tree.offset(v as isize)).fc.freq as i32)
            < (*tree.offset((*s).heap[j as usize] as isize)).fc.freq as i32
            || (*tree.offset(v as isize)).fc.freq as i32
                == (*tree.offset((*s).heap[j as usize] as isize)).fc.freq as i32
                && (*s).depth[v as usize] as i32
                    <= (*s).depth[(*s).heap[j as usize] as usize] as i32
        {
            break;
        }
        (*s).heap[k as usize] = (*s).heap[j as usize];
        k = j;
        j <<= 1 as i32;
    }
    (*s).heap[k as usize] = v;
}
unsafe extern "C" fn gen_bitlen(mut s: *mut deflate_state, mut desc: *mut tree_desc) {
    let mut tree: *mut ct_data = (*desc).dyn_tree;
    let mut max_code: i32 = (*desc).max_code;
    let mut stree: *const ct_data = (*(*desc).stat_desc).static_tree;
    let mut extra: *const intf = (*(*desc).stat_desc).extra_bits;
    let mut base: i32 = (*(*desc).stat_desc).extra_base;
    let mut max_length: i32 = (*(*desc).stat_desc).max_length;
    let mut h: i32 = 0;
    let mut n: i32 = 0;
    let mut m: i32 = 0;
    let mut bits: i32 = 0;
    let mut xbits: i32 = 0;
    let mut f: ush = 0;
    let mut overflow: i32 = 0 as i32;
    bits = 0 as i32;
    while bits <= 15 as i32 {
        (*s).bl_count[bits as usize] = 0 as i32 as ush;
        bits += 1;
        bits;
    }
    (*tree.offset((*s).heap[(*s).heap_max as usize] as isize)).dl.len = 0 as i32 as ush;
    h = (*s).heap_max + 1 as i32;
    while h < 2 as i32 * (256 as i32 + 1 as i32 + 29 as i32) + 1 as i32 {
        n = (*s).heap[h as usize];
        bits = (*tree.offset((*tree.offset(n as isize)).dl.dad as isize)).dl.len as i32
            + 1 as i32;
        if bits > max_length {
            bits = max_length;
            overflow += 1;
            overflow;
        }
        (*tree.offset(n as isize)).dl.len = bits as ush;
        if !(n > max_code) {
            (*s).bl_count[bits as usize] = ((*s).bl_count[bits as usize])
                .wrapping_add(1);
            (*s).bl_count[bits as usize];
            xbits = 0 as i32;
            if n >= base {
                xbits = *extra.offset((n - base) as isize);
            }
            f = (*tree.offset(n as isize)).fc.freq;
            (*s).opt_len = ((*s).opt_len as u64)
                .wrapping_add((f as ulg).wrapping_mul((bits + xbits) as u64)) as ulg
                as ulg;
            if !stree.is_null() {
                (*s).static_len = ((*s).static_len as u64)
                    .wrapping_add(
                        (f as ulg)
                            .wrapping_mul(
                                ((*stree.offset(n as isize)).dl.len as i32 + xbits) as u64,
                            ),
                    ) as ulg as ulg;
            }
        }
        h += 1;
        h;
    }
    if overflow == 0 as i32 {
        return;
    }
    loop {
        bits = max_length - 1 as i32;
        while (*s).bl_count[bits as usize] as i32 == 0 as i32 {
            bits -= 1;
            bits;
        }
        (*s).bl_count[bits as usize] = ((*s).bl_count[bits as usize]).wrapping_sub(1);
        (*s).bl_count[bits as usize];
        (*s).bl_count[(bits + 1 as i32) as usize] = ((*s)
            .bl_count[(bits + 1 as i32) as usize] as i32 + 2 as i32) as ush;
        (*s).bl_count[max_length as usize] = ((*s).bl_count[max_length as usize])
            .wrapping_sub(1);
        (*s).bl_count[max_length as usize];
        overflow -= 2 as i32;
        if !(overflow > 0 as i32) {
            break;
        }
    }
    bits = max_length;
    while bits != 0 as i32 {
        n = (*s).bl_count[bits as usize] as i32;
        while n != 0 as i32 {
            h -= 1;
            m = (*s).heap[h as usize];
            if m > max_code {
                continue;
            }
            if (*tree.offset(m as isize)).dl.len as u32 != bits as u32 {
                (*s).opt_len = ((*s).opt_len as u64)
                    .wrapping_add(
                        ((bits as i64 - (*tree.offset(m as isize)).dl.len as i64)
                            * (*tree.offset(m as isize)).fc.freq as i64) as u64,
                    ) as ulg as ulg;
                (*tree.offset(m as isize)).dl.len = bits as ush;
            }
            n -= 1;
            n;
        }
        bits -= 1;
        bits;
    }
}
unsafe extern "C" fn gen_codes(
    mut tree: *mut ct_data,
    mut max_code: i32,
    mut bl_count: *mut ushf,
) {
    let mut next_code: [ush; 16] = [0; 16];
    let mut code: ush = 0 as i32 as ush;
    let mut bits: i32 = 0;
    let mut n: i32 = 0;
    bits = 1 as i32;
    while bits <= 15 as i32 {
        code = ((code as i32 + *bl_count.offset((bits - 1 as i32) as isize) as i32)
            << 1 as i32) as ush;
        next_code[bits as usize] = code;
        bits += 1;
        bits;
    }
    n = 0 as i32;
    while n <= max_code {
        let mut len: i32 = (*tree.offset(n as isize)).dl.len as i32;
        if !(len == 0 as i32) {
            let fresh0 = next_code[len as usize];
            next_code[len as usize] = (next_code[len as usize]).wrapping_add(1);
            (*tree.offset(n as isize)).fc.code = bi_reverse(fresh0 as u32, len) as ush;
        }
        n += 1;
        n;
    }
}
unsafe extern "C" fn build_tree(mut s: *mut deflate_state, mut desc: *mut tree_desc) {
    let mut tree: *mut ct_data = (*desc).dyn_tree;
    let mut stree: *const ct_data = (*(*desc).stat_desc).static_tree;
    let mut elems: i32 = (*(*desc).stat_desc).elems;
    let mut n: i32 = 0;
    let mut m: i32 = 0;
    let mut max_code: i32 = -(1 as i32);
    let mut node: i32 = 0;
    (*s).heap_len = 0 as i32;
    (*s).heap_max = 2 as i32 * (256 as i32 + 1 as i32 + 29 as i32) + 1 as i32;
    n = 0 as i32;
    while n < elems {
        if (*tree.offset(n as isize)).fc.freq as i32 != 0 as i32 {
            max_code = n;
            (*s).heap_len += 1;
            (*s).heap[(*s).heap_len as usize] = max_code;
            (*s).depth[n as usize] = 0 as i32 as uch;
        } else {
            (*tree.offset(n as isize)).dl.len = 0 as i32 as ush;
        }
        n += 1;
        n;
    }
    while (*s).heap_len < 2 as i32 {
        (*s).heap_len += 1;
        (*s).heap[(*s).heap_len as usize] = if max_code < 2 as i32 {
            max_code += 1;
            max_code
        } else {
            0 as i32
        };
        node = (*s).heap[(*s).heap_len as usize];
        (*tree.offset(node as isize)).fc.freq = 1 as i32 as ush;
        (*s).depth[node as usize] = 0 as i32 as uch;
        (*s).opt_len = ((*s).opt_len).wrapping_sub(1);
        (*s).opt_len;
        if !stree.is_null() {
            (*s).static_len = ((*s).static_len as u64)
                .wrapping_sub((*stree.offset(node as isize)).dl.len as u64) as ulg
                as ulg;
        }
    }
    (*desc).max_code = max_code;
    n = (*s).heap_len / 2 as i32;
    while n >= 1 as i32 {
        pqdownheap(s, tree, n);
        n -= 1;
        n;
    }
    node = elems;
    loop {
        n = (*s).heap[1 as i32 as usize];
        let fresh1 = (*s).heap_len;
        (*s).heap_len = (*s).heap_len - 1;
        (*s).heap[1 as i32 as usize] = (*s).heap[fresh1 as usize];
        pqdownheap(s, tree, 1 as i32);
        m = (*s).heap[1 as i32 as usize];
        (*s).heap_max -= 1;
        (*s).heap[(*s).heap_max as usize] = n;
        (*s).heap_max -= 1;
        (*s).heap[(*s).heap_max as usize] = m;
        (*tree.offset(node as isize)).fc.freq = ((*tree.offset(n as isize)).fc.freq
            as i32 + (*tree.offset(m as isize)).fc.freq as i32) as ush;
        (*s).depth[node as usize] = ((if (*s).depth[n as usize] as i32
            >= (*s).depth[m as usize] as i32
        {
            (*s).depth[n as usize] as i32
        } else {
            (*s).depth[m as usize] as i32
        }) + 1 as i32) as uch;
        let ref mut fresh2 = (*tree.offset(m as isize)).dl.dad;
        *fresh2 = node as ush;
        (*tree.offset(n as isize)).dl.dad = *fresh2;
        let fresh3 = node;
        node = node + 1;
        (*s).heap[1 as i32 as usize] = fresh3;
        pqdownheap(s, tree, 1 as i32);
        if !((*s).heap_len >= 2 as i32) {
            break;
        }
    }
    (*s).heap_max -= 1;
    (*s).heap[(*s).heap_max as usize] = (*s).heap[1 as i32 as usize];
    gen_bitlen(s, desc);
    gen_codes(tree, max_code, ((*s).bl_count).as_mut_ptr());
}
unsafe extern "C" fn scan_tree(
    mut s: *mut deflate_state,
    mut tree: *mut ct_data,
    mut max_code: i32,
) {
    let mut n: i32 = 0;
    let mut prevlen: i32 = -(1 as i32);
    let mut curlen: i32 = 0;
    let mut nextlen: i32 = (*tree.offset(0 as i32 as isize)).dl.len as i32;
    let mut count: i32 = 0 as i32;
    let mut max_count: i32 = 7 as i32;
    let mut min_count: i32 = 4 as i32;
    if nextlen == 0 as i32 {
        max_count = 138 as i32;
        min_count = 3 as i32;
    }
    (*tree.offset((max_code + 1 as i32) as isize)).dl.len = 0xffff as i32 as ush;
    n = 0 as i32;
    while n <= max_code {
        curlen = nextlen;
        nextlen = (*tree.offset((n + 1 as i32) as isize)).dl.len as i32;
        count += 1;
        if !(count < max_count && curlen == nextlen) {
            if count < min_count {
                (*s).bl_tree[curlen as usize].fc.freq = ((*s)
                    .bl_tree[curlen as usize]
                    .fc
                    .freq as i32 + count) as ush;
            } else if curlen != 0 as i32 {
                if curlen != prevlen {
                    (*s).bl_tree[curlen as usize].fc.freq = ((*s)
                        .bl_tree[curlen as usize]
                        .fc
                        .freq)
                        .wrapping_add(1);
                    (*s).bl_tree[curlen as usize].fc.freq;
                }
                (*s).bl_tree[16 as i32 as usize].fc.freq = ((*s)
                    .bl_tree[16 as i32 as usize]
                    .fc
                    .freq)
                    .wrapping_add(1);
                (*s).bl_tree[16 as i32 as usize].fc.freq;
            } else if count <= 10 as i32 {
                (*s).bl_tree[17 as i32 as usize].fc.freq = ((*s)
                    .bl_tree[17 as i32 as usize]
                    .fc
                    .freq)
                    .wrapping_add(1);
                (*s).bl_tree[17 as i32 as usize].fc.freq;
            } else {
                (*s).bl_tree[18 as i32 as usize].fc.freq = ((*s)
                    .bl_tree[18 as i32 as usize]
                    .fc
                    .freq)
                    .wrapping_add(1);
                (*s).bl_tree[18 as i32 as usize].fc.freq;
            }
            count = 0 as i32;
            prevlen = curlen;
            if nextlen == 0 as i32 {
                max_count = 138 as i32;
                min_count = 3 as i32;
            } else if curlen == nextlen {
                max_count = 6 as i32;
                min_count = 3 as i32;
            } else {
                max_count = 7 as i32;
                min_count = 4 as i32;
            }
        }
        n += 1;
        n;
    }
}
unsafe extern "C" fn send_tree(
    mut s: *mut deflate_state,
    mut tree: *mut ct_data,
    mut max_code: i32,
) {
    let mut n: i32 = 0;
    let mut prevlen: i32 = -(1 as i32);
    let mut curlen: i32 = 0;
    let mut nextlen: i32 = (*tree.offset(0 as i32 as isize)).dl.len as i32;
    let mut count: i32 = 0 as i32;
    let mut max_count: i32 = 7 as i32;
    let mut min_count: i32 = 4 as i32;
    if nextlen == 0 as i32 {
        max_count = 138 as i32;
        min_count = 3 as i32;
    }
    n = 0 as i32;
    while n <= max_code {
        curlen = nextlen;
        nextlen = (*tree.offset((n + 1 as i32) as isize)).dl.len as i32;
        count += 1;
        if !(count < max_count && curlen == nextlen) {
            if count < min_count {
                loop {
                    let mut len: i32 = (*s).bl_tree[curlen as usize].dl.len as i32;
                    if (*s).bi_valid
                        > ((8 as i32 * 2 as i32) as u64)
                            .wrapping_mul(::core::mem::size_of::<i8>() as u64) as i32
                            - len
                    {
                        let mut val: i32 = (*s).bl_tree[curlen as usize].fc.code as i32;
                        (*s).bi_buf = ((*s).bi_buf as i32
                            | (val as ush as i32) << (*s).bi_valid) as ush;
                        let fresh4 = (*s).pending;
                        (*s).pending = ((*s).pending).wrapping_add(1);
                        *((*s).pending_buf).offset(fresh4 as isize) = ((*s).bi_buf as i32
                            & 0xff as i32) as uch;
                        let fresh5 = (*s).pending;
                        (*s).pending = ((*s).pending).wrapping_add(1);
                        *((*s).pending_buf).offset(fresh5 as isize) = ((*s).bi_buf as i32
                            >> 8 as i32) as uch;
                        (*s).bi_buf = (val as ush as i32
                            >> ((8 as i32 * 2 as i32) as u64)
                                .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                                .wrapping_sub((*s).bi_valid as u64)) as ush;
                        (*s).bi_valid = ((*s).bi_valid as u64)
                            .wrapping_add(
                                (len as u64)
                                    .wrapping_sub(
                                        ((8 as i32 * 2 as i32) as u64)
                                            .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                                    ),
                            ) as i32 as i32;
                    } else {
                        (*s).bi_buf = ((*s).bi_buf as i32
                            | ((*s).bl_tree[curlen as usize].fc.code as i32)
                                << (*s).bi_valid) as ush;
                        (*s).bi_valid += len;
                    }
                    count -= 1;
                    if !(count != 0 as i32) {
                        break;
                    }
                }
            } else if curlen != 0 as i32 {
                if curlen != prevlen {
                    let mut len_0: i32 = (*s).bl_tree[curlen as usize].dl.len as i32;
                    if (*s).bi_valid
                        > ((8 as i32 * 2 as i32) as u64)
                            .wrapping_mul(::core::mem::size_of::<i8>() as u64) as i32
                            - len_0
                    {
                        let mut val_0: i32 = (*s).bl_tree[curlen as usize].fc.code
                            as i32;
                        (*s).bi_buf = ((*s).bi_buf as i32
                            | (val_0 as ush as i32) << (*s).bi_valid) as ush;
                        let fresh6 = (*s).pending;
                        (*s).pending = ((*s).pending).wrapping_add(1);
                        *((*s).pending_buf).offset(fresh6 as isize) = ((*s).bi_buf as i32
                            & 0xff as i32) as uch;
                        let fresh7 = (*s).pending;
                        (*s).pending = ((*s).pending).wrapping_add(1);
                        *((*s).pending_buf).offset(fresh7 as isize) = ((*s).bi_buf as i32
                            >> 8 as i32) as uch;
                        (*s).bi_buf = (val_0 as ush as i32
                            >> ((8 as i32 * 2 as i32) as u64)
                                .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                                .wrapping_sub((*s).bi_valid as u64)) as ush;
                        (*s).bi_valid = ((*s).bi_valid as u64)
                            .wrapping_add(
                                (len_0 as u64)
                                    .wrapping_sub(
                                        ((8 as i32 * 2 as i32) as u64)
                                            .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                                    ),
                            ) as i32 as i32;
                    } else {
                        (*s).bi_buf = ((*s).bi_buf as i32
                            | ((*s).bl_tree[curlen as usize].fc.code as i32)
                                << (*s).bi_valid) as ush;
                        (*s).bi_valid += len_0;
                    }
                    count -= 1;
                    count;
                }
                let mut len_1: i32 = (*s).bl_tree[16 as i32 as usize].dl.len as i32;
                if (*s).bi_valid
                    > ((8 as i32 * 2 as i32) as u64)
                        .wrapping_mul(::core::mem::size_of::<i8>() as u64) as i32 - len_1
                {
                    let mut val_1: i32 = (*s).bl_tree[16 as i32 as usize].fc.code as i32;
                    (*s).bi_buf = ((*s).bi_buf as i32
                        | (val_1 as ush as i32) << (*s).bi_valid) as ush;
                    let fresh8 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf).offset(fresh8 as isize) = ((*s).bi_buf as i32
                        & 0xff as i32) as uch;
                    let fresh9 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf).offset(fresh9 as isize) = ((*s).bi_buf as i32
                        >> 8 as i32) as uch;
                    (*s).bi_buf = (val_1 as ush as i32
                        >> ((8 as i32 * 2 as i32) as u64)
                            .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                            .wrapping_sub((*s).bi_valid as u64)) as ush;
                    (*s).bi_valid = ((*s).bi_valid as u64)
                        .wrapping_add(
                            (len_1 as u64)
                                .wrapping_sub(
                                    ((8 as i32 * 2 as i32) as u64)
                                        .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                                ),
                        ) as i32 as i32;
                } else {
                    (*s).bi_buf = ((*s).bi_buf as i32
                        | ((*s).bl_tree[16 as i32 as usize].fc.code as i32)
                            << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_1;
                }
                let mut len_2: i32 = 2 as i32;
                if (*s).bi_valid
                    > ((8 as i32 * 2 as i32) as u64)
                        .wrapping_mul(::core::mem::size_of::<i8>() as u64) as i32 - len_2
                {
                    let mut val_2: i32 = count - 3 as i32;
                    (*s).bi_buf = ((*s).bi_buf as i32
                        | (val_2 as ush as i32) << (*s).bi_valid) as ush;
                    let fresh10 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf).offset(fresh10 as isize) = ((*s).bi_buf as i32
                        & 0xff as i32) as uch;
                    let fresh11 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf).offset(fresh11 as isize) = ((*s).bi_buf as i32
                        >> 8 as i32) as uch;
                    (*s).bi_buf = (val_2 as ush as i32
                        >> ((8 as i32 * 2 as i32) as u64)
                            .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                            .wrapping_sub((*s).bi_valid as u64)) as ush;
                    (*s).bi_valid = ((*s).bi_valid as u64)
                        .wrapping_add(
                            (len_2 as u64)
                                .wrapping_sub(
                                    ((8 as i32 * 2 as i32) as u64)
                                        .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                                ),
                        ) as i32 as i32;
                } else {
                    (*s).bi_buf = ((*s).bi_buf as i32
                        | ((count - 3 as i32) as ush as i32) << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_2;
                }
            } else if count <= 10 as i32 {
                let mut len_3: i32 = (*s).bl_tree[17 as i32 as usize].dl.len as i32;
                if (*s).bi_valid
                    > ((8 as i32 * 2 as i32) as u64)
                        .wrapping_mul(::core::mem::size_of::<i8>() as u64) as i32 - len_3
                {
                    let mut val_3: i32 = (*s).bl_tree[17 as i32 as usize].fc.code as i32;
                    (*s).bi_buf = ((*s).bi_buf as i32
                        | (val_3 as ush as i32) << (*s).bi_valid) as ush;
                    let fresh12 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf).offset(fresh12 as isize) = ((*s).bi_buf as i32
                        & 0xff as i32) as uch;
                    let fresh13 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf).offset(fresh13 as isize) = ((*s).bi_buf as i32
                        >> 8 as i32) as uch;
                    (*s).bi_buf = (val_3 as ush as i32
                        >> ((8 as i32 * 2 as i32) as u64)
                            .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                            .wrapping_sub((*s).bi_valid as u64)) as ush;
                    (*s).bi_valid = ((*s).bi_valid as u64)
                        .wrapping_add(
                            (len_3 as u64)
                                .wrapping_sub(
                                    ((8 as i32 * 2 as i32) as u64)
                                        .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                                ),
                        ) as i32 as i32;
                } else {
                    (*s).bi_buf = ((*s).bi_buf as i32
                        | ((*s).bl_tree[17 as i32 as usize].fc.code as i32)
                            << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_3;
                }
                let mut len_4: i32 = 3 as i32;
                if (*s).bi_valid
                    > ((8 as i32 * 2 as i32) as u64)
                        .wrapping_mul(::core::mem::size_of::<i8>() as u64) as i32 - len_4
                {
                    let mut val_4: i32 = count - 3 as i32;
                    (*s).bi_buf = ((*s).bi_buf as i32
                        | (val_4 as ush as i32) << (*s).bi_valid) as ush;
                    let fresh14 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf).offset(fresh14 as isize) = ((*s).bi_buf as i32
                        & 0xff as i32) as uch;
                    let fresh15 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf).offset(fresh15 as isize) = ((*s).bi_buf as i32
                        >> 8 as i32) as uch;
                    (*s).bi_buf = (val_4 as ush as i32
                        >> ((8 as i32 * 2 as i32) as u64)
                            .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                            .wrapping_sub((*s).bi_valid as u64)) as ush;
                    (*s).bi_valid = ((*s).bi_valid as u64)
                        .wrapping_add(
                            (len_4 as u64)
                                .wrapping_sub(
                                    ((8 as i32 * 2 as i32) as u64)
                                        .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                                ),
                        ) as i32 as i32;
                } else {
                    (*s).bi_buf = ((*s).bi_buf as i32
                        | ((count - 3 as i32) as ush as i32) << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_4;
                }
            } else {
                let mut len_5: i32 = (*s).bl_tree[18 as i32 as usize].dl.len as i32;
                if (*s).bi_valid
                    > ((8 as i32 * 2 as i32) as u64)
                        .wrapping_mul(::core::mem::size_of::<i8>() as u64) as i32 - len_5
                {
                    let mut val_5: i32 = (*s).bl_tree[18 as i32 as usize].fc.code as i32;
                    (*s).bi_buf = ((*s).bi_buf as i32
                        | (val_5 as ush as i32) << (*s).bi_valid) as ush;
                    let fresh16 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf).offset(fresh16 as isize) = ((*s).bi_buf as i32
                        & 0xff as i32) as uch;
                    let fresh17 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf).offset(fresh17 as isize) = ((*s).bi_buf as i32
                        >> 8 as i32) as uch;
                    (*s).bi_buf = (val_5 as ush as i32
                        >> ((8 as i32 * 2 as i32) as u64)
                            .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                            .wrapping_sub((*s).bi_valid as u64)) as ush;
                    (*s).bi_valid = ((*s).bi_valid as u64)
                        .wrapping_add(
                            (len_5 as u64)
                                .wrapping_sub(
                                    ((8 as i32 * 2 as i32) as u64)
                                        .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                                ),
                        ) as i32 as i32;
                } else {
                    (*s).bi_buf = ((*s).bi_buf as i32
                        | ((*s).bl_tree[18 as i32 as usize].fc.code as i32)
                            << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_5;
                }
                let mut len_6: i32 = 7 as i32;
                if (*s).bi_valid
                    > ((8 as i32 * 2 as i32) as u64)
                        .wrapping_mul(::core::mem::size_of::<i8>() as u64) as i32 - len_6
                {
                    let mut val_6: i32 = count - 11 as i32;
                    (*s).bi_buf = ((*s).bi_buf as i32
                        | (val_6 as ush as i32) << (*s).bi_valid) as ush;
                    let fresh18 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf).offset(fresh18 as isize) = ((*s).bi_buf as i32
                        & 0xff as i32) as uch;
                    let fresh19 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf).offset(fresh19 as isize) = ((*s).bi_buf as i32
                        >> 8 as i32) as uch;
                    (*s).bi_buf = (val_6 as ush as i32
                        >> ((8 as i32 * 2 as i32) as u64)
                            .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                            .wrapping_sub((*s).bi_valid as u64)) as ush;
                    (*s).bi_valid = ((*s).bi_valid as u64)
                        .wrapping_add(
                            (len_6 as u64)
                                .wrapping_sub(
                                    ((8 as i32 * 2 as i32) as u64)
                                        .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                                ),
                        ) as i32 as i32;
                } else {
                    (*s).bi_buf = ((*s).bi_buf as i32
                        | ((count - 11 as i32) as ush as i32) << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_6;
                }
            }
            count = 0 as i32;
            prevlen = curlen;
            if nextlen == 0 as i32 {
                max_count = 138 as i32;
                min_count = 3 as i32;
            } else if curlen == nextlen {
                max_count = 6 as i32;
                min_count = 3 as i32;
            } else {
                max_count = 7 as i32;
                min_count = 4 as i32;
            }
        }
        n += 1;
        n;
    }
}
unsafe extern "C" fn build_bl_tree(mut s: *mut deflate_state) -> i32 {
    let mut max_blindex: i32 = 0;
    scan_tree(s, ((*s).dyn_ltree).as_mut_ptr() as *mut ct_data, (*s).l_desc.max_code);
    scan_tree(s, ((*s).dyn_dtree).as_mut_ptr() as *mut ct_data, (*s).d_desc.max_code);
    build_tree(s, &mut (*s).bl_desc as *mut tree_desc_s as *mut tree_desc);
    max_blindex = 19 as i32 - 1 as i32;
    while max_blindex >= 3 as i32 {
        if (*s).bl_tree[bl_order[max_blindex as usize] as usize].dl.len as i32
            != 0 as i32
        {
            break;
        }
        max_blindex -= 1;
        max_blindex;
    }
    (*s).opt_len = ((*s).opt_len as u64)
        .wrapping_add(
            (3 as i32 * (max_blindex + 1 as i32) + 5 as i32 + 5 as i32 + 4 as i32) as u64,
        ) as ulg as ulg;
    return max_blindex;
}
unsafe extern "C" fn send_all_trees(
    mut s: *mut deflate_state,
    mut lcodes: i32,
    mut dcodes: i32,
    mut blcodes: i32,
) {
    let mut rank: i32 = 0;
    let mut len: i32 = 5 as i32;
    if (*s).bi_valid
        > ((8 as i32 * 2 as i32) as u64)
            .wrapping_mul(::core::mem::size_of::<i8>() as u64) as i32 - len
    {
        let mut val: i32 = lcodes - 257 as i32;
        (*s).bi_buf = ((*s).bi_buf as i32 | (val as ush as i32) << (*s).bi_valid) as ush;
        let fresh20 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh20 as isize) = ((*s).bi_buf as i32 & 0xff as i32)
            as uch;
        let fresh21 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh21 as isize) = ((*s).bi_buf as i32 >> 8 as i32)
            as uch;
        (*s).bi_buf = (val as ush as i32
            >> ((8 as i32 * 2 as i32) as u64)
                .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub((*s).bi_valid as u64)) as ush;
        (*s).bi_valid = ((*s).bi_valid as u64)
            .wrapping_add(
                (len as u64)
                    .wrapping_sub(
                        ((8 as i32 * 2 as i32) as u64)
                            .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                    ),
            ) as i32 as i32;
    } else {
        (*s).bi_buf = ((*s).bi_buf as i32
            | ((lcodes - 257 as i32) as ush as i32) << (*s).bi_valid) as ush;
        (*s).bi_valid += len;
    }
    let mut len_0: i32 = 5 as i32;
    if (*s).bi_valid
        > ((8 as i32 * 2 as i32) as u64)
            .wrapping_mul(::core::mem::size_of::<i8>() as u64) as i32 - len_0
    {
        let mut val_0: i32 = dcodes - 1 as i32;
        (*s).bi_buf = ((*s).bi_buf as i32 | (val_0 as ush as i32) << (*s).bi_valid)
            as ush;
        let fresh22 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh22 as isize) = ((*s).bi_buf as i32 & 0xff as i32)
            as uch;
        let fresh23 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh23 as isize) = ((*s).bi_buf as i32 >> 8 as i32)
            as uch;
        (*s).bi_buf = (val_0 as ush as i32
            >> ((8 as i32 * 2 as i32) as u64)
                .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub((*s).bi_valid as u64)) as ush;
        (*s).bi_valid = ((*s).bi_valid as u64)
            .wrapping_add(
                (len_0 as u64)
                    .wrapping_sub(
                        ((8 as i32 * 2 as i32) as u64)
                            .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                    ),
            ) as i32 as i32;
    } else {
        (*s).bi_buf = ((*s).bi_buf as i32
            | ((dcodes - 1 as i32) as ush as i32) << (*s).bi_valid) as ush;
        (*s).bi_valid += len_0;
    }
    let mut len_1: i32 = 4 as i32;
    if (*s).bi_valid
        > ((8 as i32 * 2 as i32) as u64)
            .wrapping_mul(::core::mem::size_of::<i8>() as u64) as i32 - len_1
    {
        let mut val_1: i32 = blcodes - 4 as i32;
        (*s).bi_buf = ((*s).bi_buf as i32 | (val_1 as ush as i32) << (*s).bi_valid)
            as ush;
        let fresh24 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh24 as isize) = ((*s).bi_buf as i32 & 0xff as i32)
            as uch;
        let fresh25 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh25 as isize) = ((*s).bi_buf as i32 >> 8 as i32)
            as uch;
        (*s).bi_buf = (val_1 as ush as i32
            >> ((8 as i32 * 2 as i32) as u64)
                .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub((*s).bi_valid as u64)) as ush;
        (*s).bi_valid = ((*s).bi_valid as u64)
            .wrapping_add(
                (len_1 as u64)
                    .wrapping_sub(
                        ((8 as i32 * 2 as i32) as u64)
                            .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                    ),
            ) as i32 as i32;
    } else {
        (*s).bi_buf = ((*s).bi_buf as i32
            | ((blcodes - 4 as i32) as ush as i32) << (*s).bi_valid) as ush;
        (*s).bi_valid += len_1;
    }
    rank = 0 as i32;
    while rank < blcodes {
        let mut len_2: i32 = 3 as i32;
        if (*s).bi_valid
            > ((8 as i32 * 2 as i32) as u64)
                .wrapping_mul(::core::mem::size_of::<i8>() as u64) as i32 - len_2
        {
            let mut val_2: i32 = (*s).bl_tree[bl_order[rank as usize] as usize].dl.len
                as i32;
            (*s).bi_buf = ((*s).bi_buf as i32 | (val_2 as ush as i32) << (*s).bi_valid)
                as ush;
            let fresh26 = (*s).pending;
            (*s).pending = ((*s).pending).wrapping_add(1);
            *((*s).pending_buf).offset(fresh26 as isize) = ((*s).bi_buf as i32
                & 0xff as i32) as uch;
            let fresh27 = (*s).pending;
            (*s).pending = ((*s).pending).wrapping_add(1);
            *((*s).pending_buf).offset(fresh27 as isize) = ((*s).bi_buf as i32
                >> 8 as i32) as uch;
            (*s).bi_buf = (val_2 as ush as i32
                >> ((8 as i32 * 2 as i32) as u64)
                    .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                    .wrapping_sub((*s).bi_valid as u64)) as ush;
            (*s).bi_valid = ((*s).bi_valid as u64)
                .wrapping_add(
                    (len_2 as u64)
                        .wrapping_sub(
                            ((8 as i32 * 2 as i32) as u64)
                                .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                        ),
                ) as i32 as i32;
        } else {
            (*s).bi_buf = ((*s).bi_buf as i32
                | ((*s).bl_tree[bl_order[rank as usize] as usize].dl.len as i32)
                    << (*s).bi_valid) as ush;
            (*s).bi_valid += len_2;
        }
        rank += 1;
        rank;
    }
    send_tree(s, ((*s).dyn_ltree).as_mut_ptr() as *mut ct_data, lcodes - 1 as i32);
    send_tree(s, ((*s).dyn_dtree).as_mut_ptr() as *mut ct_data, dcodes - 1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_tr_stored_block(
    mut s: *mut deflate_state,
    mut buf: *mut charf,
    mut stored_len: ulg,
    mut last: i32,
) {
    let mut len: i32 = 3 as i32;
    if (*s).bi_valid
        > ((8 as i32 * 2 as i32) as u64)
            .wrapping_mul(::core::mem::size_of::<i8>() as u64) as i32 - len
    {
        let mut val: i32 = ((0 as i32) << 1 as i32) + last;
        (*s).bi_buf = ((*s).bi_buf as i32 | (val as ush as i32) << (*s).bi_valid) as ush;
        let fresh28 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh28 as isize) = ((*s).bi_buf as i32 & 0xff as i32)
            as uch;
        let fresh29 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh29 as isize) = ((*s).bi_buf as i32 >> 8 as i32)
            as uch;
        (*s).bi_buf = (val as ush as i32
            >> ((8 as i32 * 2 as i32) as u64)
                .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub((*s).bi_valid as u64)) as ush;
        (*s).bi_valid = ((*s).bi_valid as u64)
            .wrapping_add(
                (len as u64)
                    .wrapping_sub(
                        ((8 as i32 * 2 as i32) as u64)
                            .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                    ),
            ) as i32 as i32;
    } else {
        (*s).bi_buf = ((*s).bi_buf as i32
            | ((((0 as i32) << 1 as i32) + last) as ush as i32) << (*s).bi_valid) as ush;
        (*s).bi_valid += len;
    }
    copy_block(s, buf, stored_len as u32, 1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_tr_align(mut s: *mut deflate_state) {
    let mut len: i32 = 3 as i32;
    if (*s).bi_valid
        > ((8 as i32 * 2 as i32) as u64)
            .wrapping_mul(::core::mem::size_of::<i8>() as u64) as i32 - len
    {
        let mut val: i32 = (1 as i32) << 1 as i32;
        (*s).bi_buf = ((*s).bi_buf as i32 | (val as ush as i32) << (*s).bi_valid) as ush;
        let fresh30 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh30 as isize) = ((*s).bi_buf as i32 & 0xff as i32)
            as uch;
        let fresh31 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh31 as isize) = ((*s).bi_buf as i32 >> 8 as i32)
            as uch;
        (*s).bi_buf = (val as ush as i32
            >> ((8 as i32 * 2 as i32) as u64)
                .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub((*s).bi_valid as u64)) as ush;
        (*s).bi_valid = ((*s).bi_valid as u64)
            .wrapping_add(
                (len as u64)
                    .wrapping_sub(
                        ((8 as i32 * 2 as i32) as u64)
                            .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                    ),
            ) as i32 as i32;
    } else {
        (*s).bi_buf = ((*s).bi_buf as i32
            | (((1 as i32) << 1 as i32) as ush as i32) << (*s).bi_valid) as ush;
        (*s).bi_valid += len;
    }
    let mut len_0: i32 = static_ltree[256 as i32 as usize].dl.len as i32;
    if (*s).bi_valid
        > ((8 as i32 * 2 as i32) as u64)
            .wrapping_mul(::core::mem::size_of::<i8>() as u64) as i32 - len_0
    {
        let mut val_0: i32 = static_ltree[256 as i32 as usize].fc.code as i32;
        (*s).bi_buf = ((*s).bi_buf as i32 | (val_0 as ush as i32) << (*s).bi_valid)
            as ush;
        let fresh32 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh32 as isize) = ((*s).bi_buf as i32 & 0xff as i32)
            as uch;
        let fresh33 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh33 as isize) = ((*s).bi_buf as i32 >> 8 as i32)
            as uch;
        (*s).bi_buf = (val_0 as ush as i32
            >> ((8 as i32 * 2 as i32) as u64)
                .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub((*s).bi_valid as u64)) as ush;
        (*s).bi_valid = ((*s).bi_valid as u64)
            .wrapping_add(
                (len_0 as u64)
                    .wrapping_sub(
                        ((8 as i32 * 2 as i32) as u64)
                            .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                    ),
            ) as i32 as i32;
    } else {
        (*s).bi_buf = ((*s).bi_buf as i32
            | (static_ltree[256 as i32 as usize].fc.code as i32) << (*s).bi_valid)
            as ush;
        (*s).bi_valid += len_0;
    }
    bi_flush(s);
    if 1 as i32 + (*s).last_eob_len + 10 as i32 - (*s).bi_valid < 9 as i32 {
        let mut len_1: i32 = 3 as i32;
        if (*s).bi_valid
            > ((8 as i32 * 2 as i32) as u64)
                .wrapping_mul(::core::mem::size_of::<i8>() as u64) as i32 - len_1
        {
            let mut val_1: i32 = (1 as i32) << 1 as i32;
            (*s).bi_buf = ((*s).bi_buf as i32 | (val_1 as ush as i32) << (*s).bi_valid)
                as ush;
            let fresh34 = (*s).pending;
            (*s).pending = ((*s).pending).wrapping_add(1);
            *((*s).pending_buf).offset(fresh34 as isize) = ((*s).bi_buf as i32
                & 0xff as i32) as uch;
            let fresh35 = (*s).pending;
            (*s).pending = ((*s).pending).wrapping_add(1);
            *((*s).pending_buf).offset(fresh35 as isize) = ((*s).bi_buf as i32
                >> 8 as i32) as uch;
            (*s).bi_buf = (val_1 as ush as i32
                >> ((8 as i32 * 2 as i32) as u64)
                    .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                    .wrapping_sub((*s).bi_valid as u64)) as ush;
            (*s).bi_valid = ((*s).bi_valid as u64)
                .wrapping_add(
                    (len_1 as u64)
                        .wrapping_sub(
                            ((8 as i32 * 2 as i32) as u64)
                                .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                        ),
                ) as i32 as i32;
        } else {
            (*s).bi_buf = ((*s).bi_buf as i32
                | (((1 as i32) << 1 as i32) as ush as i32) << (*s).bi_valid) as ush;
            (*s).bi_valid += len_1;
        }
        let mut len_2: i32 = static_ltree[256 as i32 as usize].dl.len as i32;
        if (*s).bi_valid
            > ((8 as i32 * 2 as i32) as u64)
                .wrapping_mul(::core::mem::size_of::<i8>() as u64) as i32 - len_2
        {
            let mut val_2: i32 = static_ltree[256 as i32 as usize].fc.code as i32;
            (*s).bi_buf = ((*s).bi_buf as i32 | (val_2 as ush as i32) << (*s).bi_valid)
                as ush;
            let fresh36 = (*s).pending;
            (*s).pending = ((*s).pending).wrapping_add(1);
            *((*s).pending_buf).offset(fresh36 as isize) = ((*s).bi_buf as i32
                & 0xff as i32) as uch;
            let fresh37 = (*s).pending;
            (*s).pending = ((*s).pending).wrapping_add(1);
            *((*s).pending_buf).offset(fresh37 as isize) = ((*s).bi_buf as i32
                >> 8 as i32) as uch;
            (*s).bi_buf = (val_2 as ush as i32
                >> ((8 as i32 * 2 as i32) as u64)
                    .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                    .wrapping_sub((*s).bi_valid as u64)) as ush;
            (*s).bi_valid = ((*s).bi_valid as u64)
                .wrapping_add(
                    (len_2 as u64)
                        .wrapping_sub(
                            ((8 as i32 * 2 as i32) as u64)
                                .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                        ),
                ) as i32 as i32;
        } else {
            (*s).bi_buf = ((*s).bi_buf as i32
                | (static_ltree[256 as i32 as usize].fc.code as i32) << (*s).bi_valid)
                as ush;
            (*s).bi_valid += len_2;
        }
        bi_flush(s);
    }
    (*s).last_eob_len = 7 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_tr_flush_block(
    mut s: *mut deflate_state,
    mut buf: *mut charf,
    mut stored_len: ulg,
    mut last: i32,
) {
    let mut opt_lenb: ulg = 0;
    let mut static_lenb: ulg = 0;
    let mut max_blindex: i32 = 0 as i32;
    if (*s).level > 0 as i32 {
        if (*(*s).strm).data_type == 2 as i32 {
            (*(*s).strm).data_type = detect_data_type(s);
        }
        build_tree(s, &mut (*s).l_desc as *mut tree_desc_s as *mut tree_desc);
        build_tree(s, &mut (*s).d_desc as *mut tree_desc_s as *mut tree_desc);
        max_blindex = build_bl_tree(s);
        opt_lenb = ((*s).opt_len)
            .wrapping_add(3 as i32 as u64)
            .wrapping_add(7 as i32 as u64) >> 3 as i32;
        static_lenb = ((*s).static_len)
            .wrapping_add(3 as i32 as u64)
            .wrapping_add(7 as i32 as u64) >> 3 as i32;
        if static_lenb <= opt_lenb {
            opt_lenb = static_lenb;
        }
    } else {
        static_lenb = stored_len.wrapping_add(5 as i32 as u64);
        opt_lenb = static_lenb;
    }
    if stored_len.wrapping_add(4 as i32 as u64) <= opt_lenb && !buf.is_null() {
        _glp_zlib_tr_stored_block(s, buf, stored_len, last);
    } else if (*s).strategy == 4 as i32 || static_lenb == opt_lenb {
        let mut len: i32 = 3 as i32;
        if (*s).bi_valid
            > ((8 as i32 * 2 as i32) as u64)
                .wrapping_mul(::core::mem::size_of::<i8>() as u64) as i32 - len
        {
            let mut val: i32 = ((1 as i32) << 1 as i32) + last;
            (*s).bi_buf = ((*s).bi_buf as i32 | (val as ush as i32) << (*s).bi_valid)
                as ush;
            let fresh38 = (*s).pending;
            (*s).pending = ((*s).pending).wrapping_add(1);
            *((*s).pending_buf).offset(fresh38 as isize) = ((*s).bi_buf as i32
                & 0xff as i32) as uch;
            let fresh39 = (*s).pending;
            (*s).pending = ((*s).pending).wrapping_add(1);
            *((*s).pending_buf).offset(fresh39 as isize) = ((*s).bi_buf as i32
                >> 8 as i32) as uch;
            (*s).bi_buf = (val as ush as i32
                >> ((8 as i32 * 2 as i32) as u64)
                    .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                    .wrapping_sub((*s).bi_valid as u64)) as ush;
            (*s).bi_valid = ((*s).bi_valid as u64)
                .wrapping_add(
                    (len as u64)
                        .wrapping_sub(
                            ((8 as i32 * 2 as i32) as u64)
                                .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                        ),
                ) as i32 as i32;
        } else {
            (*s).bi_buf = ((*s).bi_buf as i32
                | ((((1 as i32) << 1 as i32) + last) as ush as i32) << (*s).bi_valid)
                as ush;
            (*s).bi_valid += len;
        }
        compress_block(
            s,
            static_ltree.as_ptr() as *mut ct_data,
            static_dtree.as_ptr() as *mut ct_data,
        );
    } else {
        let mut len_0: i32 = 3 as i32;
        if (*s).bi_valid
            > ((8 as i32 * 2 as i32) as u64)
                .wrapping_mul(::core::mem::size_of::<i8>() as u64) as i32 - len_0
        {
            let mut val_0: i32 = ((2 as i32) << 1 as i32) + last;
            (*s).bi_buf = ((*s).bi_buf as i32 | (val_0 as ush as i32) << (*s).bi_valid)
                as ush;
            let fresh40 = (*s).pending;
            (*s).pending = ((*s).pending).wrapping_add(1);
            *((*s).pending_buf).offset(fresh40 as isize) = ((*s).bi_buf as i32
                & 0xff as i32) as uch;
            let fresh41 = (*s).pending;
            (*s).pending = ((*s).pending).wrapping_add(1);
            *((*s).pending_buf).offset(fresh41 as isize) = ((*s).bi_buf as i32
                >> 8 as i32) as uch;
            (*s).bi_buf = (val_0 as ush as i32
                >> ((8 as i32 * 2 as i32) as u64)
                    .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                    .wrapping_sub((*s).bi_valid as u64)) as ush;
            (*s).bi_valid = ((*s).bi_valid as u64)
                .wrapping_add(
                    (len_0 as u64)
                        .wrapping_sub(
                            ((8 as i32 * 2 as i32) as u64)
                                .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                        ),
                ) as i32 as i32;
        } else {
            (*s).bi_buf = ((*s).bi_buf as i32
                | ((((2 as i32) << 1 as i32) + last) as ush as i32) << (*s).bi_valid)
                as ush;
            (*s).bi_valid += len_0;
        }
        send_all_trees(
            s,
            (*s).l_desc.max_code + 1 as i32,
            (*s).d_desc.max_code + 1 as i32,
            max_blindex + 1 as i32,
        );
        compress_block(
            s,
            ((*s).dyn_ltree).as_mut_ptr() as *mut ct_data,
            ((*s).dyn_dtree).as_mut_ptr() as *mut ct_data,
        );
    }
    init_block(s);
    if last != 0 {
        bi_windup(s);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_tr_tally(
    mut s: *mut deflate_state,
    mut dist: u32,
    mut lc: u32,
) -> i32 {
    *((*s).d_buf).offset((*s).last_lit as isize) = dist as ush;
    let fresh42 = (*s).last_lit;
    (*s).last_lit = ((*s).last_lit).wrapping_add(1);
    *((*s).l_buf).offset(fresh42 as isize) = lc as uch;
    if dist == 0 as i32 as u32 {
        (*s).dyn_ltree[lc as usize].fc.freq = ((*s).dyn_ltree[lc as usize].fc.freq)
            .wrapping_add(1);
        (*s).dyn_ltree[lc as usize].fc.freq;
    } else {
        (*s).matches = ((*s).matches).wrapping_add(1);
        (*s).matches;
        dist = dist.wrapping_sub(1);
        dist;
        (*s)
            .dyn_ltree[(_glp_zlib_length_code[lc as usize] as i32 + 256 as i32
                + 1 as i32) as usize]
            .fc
            .freq = ((*s)
            .dyn_ltree[(_glp_zlib_length_code[lc as usize] as i32 + 256 as i32
                + 1 as i32) as usize]
            .fc
            .freq)
            .wrapping_add(1);
        (*s)
            .dyn_ltree[(_glp_zlib_length_code[lc as usize] as i32 + 256 as i32
                + 1 as i32) as usize]
            .fc
            .freq;
        (*s)
            .dyn_dtree[(if dist < 256 as i32 as u32 {
                _glp_zlib_dist_code[dist as usize] as i32
            } else {
                _glp_zlib_dist_code[(256 as i32 as u32).wrapping_add(dist >> 7 as i32)
                    as usize] as i32
            }) as usize]
            .fc
            .freq = ((*s)
            .dyn_dtree[(if dist < 256 as i32 as u32 {
                _glp_zlib_dist_code[dist as usize] as i32
            } else {
                _glp_zlib_dist_code[(256 as i32 as u32).wrapping_add(dist >> 7 as i32)
                    as usize] as i32
            }) as usize]
            .fc
            .freq)
            .wrapping_add(1);
        (*s)
            .dyn_dtree[(if dist < 256 as i32 as u32 {
                _glp_zlib_dist_code[dist as usize] as i32
            } else {
                _glp_zlib_dist_code[(256 as i32 as u32).wrapping_add(dist >> 7 as i32)
                    as usize] as i32
            }) as usize]
            .fc
            .freq;
    }
    return ((*s).last_lit == ((*s).lit_bufsize).wrapping_sub(1 as i32 as u32)) as i32;
}
unsafe extern "C" fn compress_block(
    mut s: *mut deflate_state,
    mut ltree: *mut ct_data,
    mut dtree: *mut ct_data,
) {
    let mut dist: u32 = 0;
    let mut lc: i32 = 0;
    let mut lx: u32 = 0 as i32 as u32;
    let mut code: u32 = 0;
    let mut extra: i32 = 0;
    if (*s).last_lit != 0 as i32 as u32 {
        loop {
            dist = *((*s).d_buf).offset(lx as isize) as u32;
            let fresh43 = lx;
            lx = lx.wrapping_add(1);
            lc = *((*s).l_buf).offset(fresh43 as isize) as i32;
            if dist == 0 as i32 as u32 {
                let mut len: i32 = (*ltree.offset(lc as isize)).dl.len as i32;
                if (*s).bi_valid
                    > ((8 as i32 * 2 as i32) as u64)
                        .wrapping_mul(::core::mem::size_of::<i8>() as u64) as i32 - len
                {
                    let mut val: i32 = (*ltree.offset(lc as isize)).fc.code as i32;
                    (*s).bi_buf = ((*s).bi_buf as i32
                        | (val as ush as i32) << (*s).bi_valid) as ush;
                    let fresh44 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf).offset(fresh44 as isize) = ((*s).bi_buf as i32
                        & 0xff as i32) as uch;
                    let fresh45 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf).offset(fresh45 as isize) = ((*s).bi_buf as i32
                        >> 8 as i32) as uch;
                    (*s).bi_buf = (val as ush as i32
                        >> ((8 as i32 * 2 as i32) as u64)
                            .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                            .wrapping_sub((*s).bi_valid as u64)) as ush;
                    (*s).bi_valid = ((*s).bi_valid as u64)
                        .wrapping_add(
                            (len as u64)
                                .wrapping_sub(
                                    ((8 as i32 * 2 as i32) as u64)
                                        .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                                ),
                        ) as i32 as i32;
                } else {
                    (*s).bi_buf = ((*s).bi_buf as i32
                        | ((*ltree.offset(lc as isize)).fc.code as i32) << (*s).bi_valid)
                        as ush;
                    (*s).bi_valid += len;
                }
            } else {
                code = _glp_zlib_length_code[lc as usize] as u32;
                let mut len_0: i32 = (*ltree
                    .offset(
                        code
                            .wrapping_add(256 as i32 as u32)
                            .wrapping_add(1 as i32 as u32) as isize,
                    ))
                    .dl
                    .len as i32;
                if (*s).bi_valid
                    > ((8 as i32 * 2 as i32) as u64)
                        .wrapping_mul(::core::mem::size_of::<i8>() as u64) as i32 - len_0
                {
                    let mut val_0: i32 = (*ltree
                        .offset(
                            code
                                .wrapping_add(256 as i32 as u32)
                                .wrapping_add(1 as i32 as u32) as isize,
                        ))
                        .fc
                        .code as i32;
                    (*s).bi_buf = ((*s).bi_buf as i32
                        | (val_0 as ush as i32) << (*s).bi_valid) as ush;
                    let fresh46 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf).offset(fresh46 as isize) = ((*s).bi_buf as i32
                        & 0xff as i32) as uch;
                    let fresh47 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf).offset(fresh47 as isize) = ((*s).bi_buf as i32
                        >> 8 as i32) as uch;
                    (*s).bi_buf = (val_0 as ush as i32
                        >> ((8 as i32 * 2 as i32) as u64)
                            .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                            .wrapping_sub((*s).bi_valid as u64)) as ush;
                    (*s).bi_valid = ((*s).bi_valid as u64)
                        .wrapping_add(
                            (len_0 as u64)
                                .wrapping_sub(
                                    ((8 as i32 * 2 as i32) as u64)
                                        .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                                ),
                        ) as i32 as i32;
                } else {
                    (*s).bi_buf = ((*s).bi_buf as i32
                        | ((*ltree
                            .offset(
                                code
                                    .wrapping_add(256 as i32 as u32)
                                    .wrapping_add(1 as i32 as u32) as isize,
                            ))
                            .fc
                            .code as i32) << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_0;
                }
                extra = extra_lbits[code as usize];
                if extra != 0 as i32 {
                    lc -= base_length[code as usize];
                    let mut len_1: i32 = extra;
                    if (*s).bi_valid
                        > ((8 as i32 * 2 as i32) as u64)
                            .wrapping_mul(::core::mem::size_of::<i8>() as u64) as i32
                            - len_1
                    {
                        let mut val_1: i32 = lc;
                        (*s).bi_buf = ((*s).bi_buf as i32
                            | (val_1 as ush as i32) << (*s).bi_valid) as ush;
                        let fresh48 = (*s).pending;
                        (*s).pending = ((*s).pending).wrapping_add(1);
                        *((*s).pending_buf).offset(fresh48 as isize) = ((*s).bi_buf
                            as i32 & 0xff as i32) as uch;
                        let fresh49 = (*s).pending;
                        (*s).pending = ((*s).pending).wrapping_add(1);
                        *((*s).pending_buf).offset(fresh49 as isize) = ((*s).bi_buf
                            as i32 >> 8 as i32) as uch;
                        (*s).bi_buf = (val_1 as ush as i32
                            >> ((8 as i32 * 2 as i32) as u64)
                                .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                                .wrapping_sub((*s).bi_valid as u64)) as ush;
                        (*s).bi_valid = ((*s).bi_valid as u64)
                            .wrapping_add(
                                (len_1 as u64)
                                    .wrapping_sub(
                                        ((8 as i32 * 2 as i32) as u64)
                                            .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                                    ),
                            ) as i32 as i32;
                    } else {
                        (*s).bi_buf = ((*s).bi_buf as i32
                            | (lc as ush as i32) << (*s).bi_valid) as ush;
                        (*s).bi_valid += len_1;
                    }
                }
                dist = dist.wrapping_sub(1);
                dist;
                code = (if dist < 256 as i32 as u32 {
                    _glp_zlib_dist_code[dist as usize] as i32
                } else {
                    _glp_zlib_dist_code[(256 as i32 as u32)
                        .wrapping_add(dist >> 7 as i32) as usize] as i32
                }) as u32;
                let mut len_2: i32 = (*dtree.offset(code as isize)).dl.len as i32;
                if (*s).bi_valid
                    > ((8 as i32 * 2 as i32) as u64)
                        .wrapping_mul(::core::mem::size_of::<i8>() as u64) as i32 - len_2
                {
                    let mut val_2: i32 = (*dtree.offset(code as isize)).fc.code as i32;
                    (*s).bi_buf = ((*s).bi_buf as i32
                        | (val_2 as ush as i32) << (*s).bi_valid) as ush;
                    let fresh50 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf).offset(fresh50 as isize) = ((*s).bi_buf as i32
                        & 0xff as i32) as uch;
                    let fresh51 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf).offset(fresh51 as isize) = ((*s).bi_buf as i32
                        >> 8 as i32) as uch;
                    (*s).bi_buf = (val_2 as ush as i32
                        >> ((8 as i32 * 2 as i32) as u64)
                            .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                            .wrapping_sub((*s).bi_valid as u64)) as ush;
                    (*s).bi_valid = ((*s).bi_valid as u64)
                        .wrapping_add(
                            (len_2 as u64)
                                .wrapping_sub(
                                    ((8 as i32 * 2 as i32) as u64)
                                        .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                                ),
                        ) as i32 as i32;
                } else {
                    (*s).bi_buf = ((*s).bi_buf as i32
                        | ((*dtree.offset(code as isize)).fc.code as i32)
                            << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_2;
                }
                extra = extra_dbits[code as usize];
                if extra != 0 as i32 {
                    dist = dist.wrapping_sub(base_dist[code as usize] as u32);
                    let mut len_3: i32 = extra;
                    if (*s).bi_valid
                        > ((8 as i32 * 2 as i32) as u64)
                            .wrapping_mul(::core::mem::size_of::<i8>() as u64) as i32
                            - len_3
                    {
                        let mut val_3: i32 = dist as i32;
                        (*s).bi_buf = ((*s).bi_buf as i32
                            | (val_3 as ush as i32) << (*s).bi_valid) as ush;
                        let fresh52 = (*s).pending;
                        (*s).pending = ((*s).pending).wrapping_add(1);
                        *((*s).pending_buf).offset(fresh52 as isize) = ((*s).bi_buf
                            as i32 & 0xff as i32) as uch;
                        let fresh53 = (*s).pending;
                        (*s).pending = ((*s).pending).wrapping_add(1);
                        *((*s).pending_buf).offset(fresh53 as isize) = ((*s).bi_buf
                            as i32 >> 8 as i32) as uch;
                        (*s).bi_buf = (val_3 as ush as i32
                            >> ((8 as i32 * 2 as i32) as u64)
                                .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                                .wrapping_sub((*s).bi_valid as u64)) as ush;
                        (*s).bi_valid = ((*s).bi_valid as u64)
                            .wrapping_add(
                                (len_3 as u64)
                                    .wrapping_sub(
                                        ((8 as i32 * 2 as i32) as u64)
                                            .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                                    ),
                            ) as i32 as i32;
                    } else {
                        (*s).bi_buf = ((*s).bi_buf as i32
                            | (dist as ush as i32) << (*s).bi_valid) as ush;
                        (*s).bi_valid += len_3;
                    }
                }
            }
            if !(lx < (*s).last_lit) {
                break;
            }
        }
    }
    let mut len_4: i32 = (*ltree.offset(256 as i32 as isize)).dl.len as i32;
    if (*s).bi_valid
        > ((8 as i32 * 2 as i32) as u64)
            .wrapping_mul(::core::mem::size_of::<i8>() as u64) as i32 - len_4
    {
        let mut val_4: i32 = (*ltree.offset(256 as i32 as isize)).fc.code as i32;
        (*s).bi_buf = ((*s).bi_buf as i32 | (val_4 as ush as i32) << (*s).bi_valid)
            as ush;
        let fresh54 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh54 as isize) = ((*s).bi_buf as i32 & 0xff as i32)
            as uch;
        let fresh55 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh55 as isize) = ((*s).bi_buf as i32 >> 8 as i32)
            as uch;
        (*s).bi_buf = (val_4 as ush as i32
            >> ((8 as i32 * 2 as i32) as u64)
                .wrapping_mul(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub((*s).bi_valid as u64)) as ush;
        (*s).bi_valid = ((*s).bi_valid as u64)
            .wrapping_add(
                (len_4 as u64)
                    .wrapping_sub(
                        ((8 as i32 * 2 as i32) as u64)
                            .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                    ),
            ) as i32 as i32;
    } else {
        (*s).bi_buf = ((*s).bi_buf as i32
            | ((*ltree.offset(256 as i32 as isize)).fc.code as i32) << (*s).bi_valid)
            as ush;
        (*s).bi_valid += len_4;
    }
    (*s).last_eob_len = (*ltree.offset(256 as i32 as isize)).dl.len as i32;
}
unsafe extern "C" fn detect_data_type(mut s: *mut deflate_state) -> i32 {
    let mut black_mask: u64 = 0xf3ffc07f as u64;
    let mut n: i32 = 0;
    n = 0 as i32;
    while n <= 31 as i32 {
        if black_mask & 1 as i32 as u64 != 0
            && (*s).dyn_ltree[n as usize].fc.freq as i32 != 0 as i32
        {
            return 0 as i32;
        }
        n += 1;
        n;
        black_mask >>= 1 as i32;
    }
    if (*s).dyn_ltree[9 as i32 as usize].fc.freq as i32 != 0 as i32
        || (*s).dyn_ltree[10 as i32 as usize].fc.freq as i32 != 0 as i32
        || (*s).dyn_ltree[13 as i32 as usize].fc.freq as i32 != 0 as i32
    {
        return 1 as i32;
    }
    n = 32 as i32;
    while n < 256 as i32 {
        if (*s).dyn_ltree[n as usize].fc.freq as i32 != 0 as i32 {
            return 1 as i32;
        }
        n += 1;
        n;
    }
    return 0 as i32;
}
unsafe extern "C" fn bi_reverse(mut code: u32, mut len: i32) -> u32 {
    let mut res: u32 = 0 as i32 as u32;
    loop {
        res |= code & 1 as i32 as u32;
        code >>= 1 as i32;
        res <<= 1 as i32;
        len -= 1;
        if !(len > 0 as i32) {
            break;
        }
    }
    return res >> 1 as i32;
}
unsafe extern "C" fn bi_flush(mut s: *mut deflate_state) {
    if (*s).bi_valid == 16 as i32 {
        let fresh56 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh56 as isize) = ((*s).bi_buf as i32 & 0xff as i32)
            as uch;
        let fresh57 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh57 as isize) = ((*s).bi_buf as i32 >> 8 as i32)
            as uch;
        (*s).bi_buf = 0 as i32 as ush;
        (*s).bi_valid = 0 as i32;
    } else if (*s).bi_valid >= 8 as i32 {
        let fresh58 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh58 as isize) = (*s).bi_buf as Byte;
        (*s).bi_buf = ((*s).bi_buf as i32 >> 8 as i32) as ush;
        (*s).bi_valid -= 8 as i32;
    }
}
unsafe extern "C" fn bi_windup(mut s: *mut deflate_state) {
    if (*s).bi_valid > 8 as i32 {
        let fresh59 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh59 as isize) = ((*s).bi_buf as i32 & 0xff as i32)
            as uch;
        let fresh60 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh60 as isize) = ((*s).bi_buf as i32 >> 8 as i32)
            as uch;
    } else if (*s).bi_valid > 0 as i32 {
        let fresh61 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh61 as isize) = (*s).bi_buf as Byte;
    }
    (*s).bi_buf = 0 as i32 as ush;
    (*s).bi_valid = 0 as i32;
}
unsafe extern "C" fn copy_block(
    mut s: *mut deflate_state,
    mut buf: *mut charf,
    mut len: u32,
    mut header: i32,
) {
    bi_windup(s);
    (*s).last_eob_len = 8 as i32;
    if header != 0 {
        let fresh62 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh62 as isize) = (len as ush as i32 & 0xff as i32)
            as uch;
        let fresh63 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh63 as isize) = (len as ush as i32 >> 8 as i32)
            as uch;
        let fresh64 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh64 as isize) = (!len as ush as i32 & 0xff as i32)
            as uch;
        let fresh65 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh65 as isize) = (!len as ush as i32 >> 8 as i32)
            as uch;
    }
    loop {
        let fresh66 = len;
        len = len.wrapping_sub(1);
        if !(fresh66 != 0) {
            break;
        }
        let fresh67 = buf;
        buf = buf.offset(1);
        let fresh68 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh68 as isize) = *fresh67 as Bytef;
    };
}