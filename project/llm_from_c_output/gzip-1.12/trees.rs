use std::cmp::max;

const MAX_BITS: usize = 15;
const MAX_BL_BITS: usize = 7;
const LENGTH_CODES: usize = 29;
const LITERALS: usize = 256;
const END_BLOCK: usize = 256;
const L_CODES: usize = LITERALS + 1 + LENGTH_CODES;
const D_CODES: usize = 30;
const BL_CODES: usize = 19;
const REP_3_6: usize = 16;
const REPZ_3_10: usize = 17;
const REPZ_11_138: usize = 18;
const HEAP_SIZE: usize = 2 * L_CODES + 1;
const SMALLEST: usize = 1;

#[derive(Debug, Clone, Copy)]
struct CtData {
    freq: u16,
    code: u16,
    dad: u16,
    len: u16,
}

impl CtData {
    fn new() -> Self {
        CtData {
            freq: 0,
            code: 0,
            dad: 0,
            len: 0,
        }
    }
}

struct TreeDesc {
    dyn_tree: Vec<CtData>,
    static_tree: Option<Vec<CtData>>,
    extra_bits: Vec<i32>,
    extra_base: i32,
    elems: i32,
    max_length: i32,
    max_code: i32,
}

struct HuffmanEncoder {
    dyn_ltree: Vec<CtData>,
    dyn_dtree: Vec<CtData>,
    static_ltree: Vec<CtData>,
    static_dtree: Vec<CtData>,
    bl_tree: Vec<CtData>,
    bl_count: Vec<u16>,
    bl_order: Vec<u8>,
    heap: Vec<i32>,
    heap_len: i32,
    heap_max: i32,
    depth: Vec<u8>,
    length_code: Vec<u8>,
    dist_code: Vec<u8>,
    base_length: Vec<i32>,
    base_dist: Vec<i32>,
    l_buf: Vec<u8>,
    d_buf: Vec<u16>,
    flag_buf: Vec<u8>,
    last_lit: usize,
    last_dist: usize,
    last_flags: usize,
    flags: u8,
    flag_bit: u8,
    opt_len: u64,
    static_len: u64,
    compressed_len: u64,
    input_len: u64,
    file_type: u16,
    file_method: i32,
}

impl HuffmanEncoder {
    fn new() -> Self {
        let bl_order = vec![
            16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
        ];

        HuffmanEncoder {
            dyn_ltree: vec![CtData::new(); HEAP_SIZE],
            dyn_dtree: vec![CtData::new(); 2 * D_CODES + 1],
            static_ltree: vec![CtData::new(); L_CODES + 2],
            static_dtree: vec![CtData::new(); D_CODES],
            bl_tree: vec![CtData::new(); 2 * BL_CODES + 1],
            bl_count: vec![0; MAX_BITS + 1],
            bl_order,
            heap: vec![0; 2 * L_CODES + 1],
            heap_len: 0,
            heap_max: 0,
            depth: vec![0; 2 * L_CODES + 1],
            length_code: vec![0; 256],
            dist_code: vec![0; 512],
            base_length: vec![0; LENGTH_CODES],
            base_dist: vec![0; D_CODES],
            l_buf: Vec::new(),
            d_buf: Vec::new(),
            flag_buf: vec![0; 1024],
            last_lit: 0,
            last_dist: 0,
            last_flags: 0,
            flags: 0,
            flag_bit: 1,
            opt_len: 0,
            static_len: 0,
            compressed_len: 0,
            input_len: 0,
            file_type: 0,
            file_method: 0,
        }
    }

    fn ct_init(&mut self, attr: u16, method: i32) {
        self.file_type = attr;
        self.file_method = method;
        self.compressed_len = 0;
        self.input_len = 0;

        if self.static_dtree[0].len != 0 {
            return;
        }

        let mut length = 0;
        for code in 0..LENGTH_CODES - 1 {
            self.base_length[code] = length as i32;
            for _ in 0..(1 << self.extra_lbits()[code]) {
                self.length_code[length] = code as u8;
                length += 1;
            }
        }
        assert!(length == 256, "ct_init: length != 256");
        self.length_code[length - 1] = (LENGTH_CODES - 1) as u8;

        let mut dist = 0;
        for code in 0..16 {
            self.base_dist[code] = dist as i32;
            for _ in 0..(1 << self.extra_dbits()[code]) {
                self.dist_code[dist] = code as u8;
                dist += 1;
            }
        }
        assert!(dist == 256, "ct_init: dist != 256");
        dist >>= 7;
        for code in 16..D_CODES {
            self.base_dist[code] = (dist << 7) as i32;
            for _ in 0..(1 << (self.extra_dbits()[code] - 7)) {
                self.dist_code[256 + dist] = code as u8;
                dist += 1;
            }
        }
        assert!(dist == 256, "ct_init: 256+dist != 512");

        for bits in 0..=MAX_BITS {
            self.bl_count[bits] = 0;
        }

        let mut n = 0;
        while n <= 143 {
            self.static_ltree[n].len = 8;
            self.bl_count[8] += 1;
            n += 1;
        }
        while n <= 255 {
            self.static_ltree[n].len = 9;
            self.bl_count[9] += 1;
            n += 1;
        }
        while n <= 279 {
            self.static_ltree[n].len = 7;
            self.bl_count[7] += 1;
            n += 1;
        }
        while n <= 287 {
            self.static_ltree[n].len = 8;
            self.bl_count[8] += 1;
            n += 1;
        }

        self.gen_codes(&mut self.static_ltree, L_CODES + 1);

        for n in 0..D_CODES {
            self.static_dtree[n].len = 5;
            self.static_dtree[n].code = self.bi_reverse(n as u16, 5);
        }

        self.init_block();
    }

    fn init_block(&mut self) {
        for n in 0..L_CODES {
            self.dyn_ltree[n].freq = 0;
        }
        for n in 0..D_CODES {
            self.dyn_dtree[n].freq = 0;
        }
        for n in 0..BL_CODES {
            self.bl_tree[n].freq = 0;
        }

        self.dyn_ltree[END_BLOCK].freq = 1;
        self.opt_len = 0;
        self.static_len = 0;
        self.last_lit = 0;
        self.last_dist = 0;
        self.last_flags = 0;
        self.flags = 0;
        self.flag_bit = 1;
    }

    fn bi_reverse(&self, mut code: u16, mut len: usize) -> u16 {
        let mut res = 0;
        loop {
            res |= code & 1;
            code >>= 1;
            len -= 1;
            if len == 0 {
                break;
            }
            res <<= 1;
        }
        res
    }

    fn extra_lbits(&self) -> Vec<i32> {
        vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0]
    }

    fn extra_dbits(&self) -> Vec<i32> {
        vec![0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13]
    }

    fn extra_blbits(&self) -> Vec<i32> {
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 7]
    }

    fn gen_codes(&mut self, tree: &mut [CtData], max_code: usize) {
        let mut next_code = vec![0; MAX_BITS + 1];
        let mut code = 0;

        for bits in 1..=MAX_BITS {
            code = (code + self.bl_count[bits - 1] as u16) << 1;
            next_code[bits] = code;
        }

        for n in 0..=max_code {
            let len = tree[n].len as usize;
            if len == 0 {
                continue;
            }
            tree[n].code = self.bi_reverse(next_code[len], len);
            next_code[len] += 1;
        }
    }

    fn pqdownheap(&mut self, tree: &[CtData], mut k: i32) {
        let v = self.heap[k as usize];
        let mut j = k << 1;
        while j <= self.heap_len {
            if j < self.heap_len && self.smaller(tree, self.heap[(j + 1) as usize], self.heap[j as usize]) {
                j += 1;
            }
            if self.smaller(tree, v, self.heap[j as usize]) {
                break;
            }
            self.heap[k as usize] = self.heap[j as usize];
            k = j;
            j <<= 1;
        }
        self.heap[k as usize] = v;
    }

    fn smaller(&self, tree: &[CtData], n: i32, m: i32) -> bool {
        tree[n as usize].freq < tree[m as usize].freq
            || (tree[n as usize].freq == tree[m as usize].freq
                && self.depth[n as usize] <= self.depth[m as usize])
    }

    fn gen_bitlen(&mut self, desc: &mut TreeDesc) {
        let tree = &mut desc.dyn_tree;
        let extra = &desc.extra_bits;
        let base = desc.extra_base;
        let max_code = desc.max_code;
        let max_length = desc.max_length;
        let stree = desc.static_tree.as_ref();

        let mut overflow = 0;

        for bits in 0..=MAX_BITS {
            self.bl_count[bits] = 0;
        }

        tree[self.heap[self.heap_max as usize] as usize].len = 0;

        for h in (self.heap_max + 1)..HEAP_SIZE as i32 {
            let n = self.heap[h as usize];
            let mut bits = tree[tree[n as usize].dad as usize].len + 1;
            if bits > max_length as u16 {
                bits = max_length as u16;
                overflow += 1;
            }
            tree[n as usize].len = bits;

            if n > max_code as i32 {
                continue;
            }

            self.bl_count[bits as usize] += 1;
            let mut xbits = 0;
            if n >= base {
                xbits = extra[(n - base) as usize];
            }
            let f = tree[n as usize].freq as u64;
            self.opt_len += f * (bits as u64 + xbits as u64);
            if let Some(stree) = stree {
                self.static_len += f * (stree[n as usize].len as u64 + xbits as u64);
            }
        }

        if overflow == 0 {
            return;
        }

        let mut bits = max_length - 1;
        loop {
            if self.bl_count[bits as usize] != 0 {
                break;
            }
            bits -= 1;
        }

        self.bl_count[bits as usize] -= 1;
        self.bl_count[(bits + 1) as usize] += 2;
        self.bl_count[max_length as usize] -= 1;
        overflow -= 2;

        while overflow > 0 {
            bits = max_length - 1;
            while self.bl_count[bits as usize] == 0 {
                bits -= 1;
            }
            self.bl_count[bits as usize] -= 1;
            self.bl_count[(bits + 1) as usize] += 1;
            self.bl_count[max_length as usize] -= 1;
            overflow -= 1;
        }

        for bits in (1..=max_length).rev() {
            let mut n = self.bl_count[bits as usize];
            while n != 0 {
                let m = self.heap[(self.heap_len - 1) as usize];
                self.heap_len -= 1;
                if m > max_code as i32 {
                    continue;
                }
                if tree[m as usize].len != bits as u16 {
                    self.opt_len +=
                        (bits as i64 - tree[m as usize].len as i64) * tree[m as usize].freq as i64;
                    tree[m as usize].len = bits as u16;
                }
                n -= 1;
            }
        }
    }

    fn build_tree(&mut self, desc: &mut TreeDesc) {
        let tree = &mut desc.dyn_tree;
        let stree = desc.static_tree.as_ref();
        let elems = desc.elems;
        let mut max_code = -1;
        let mut node = elems;

        self.heap_len = 0;
        self.heap_max = HEAP_SIZE as i32;

        for n in 0..elems {
            if tree[n as usize].freq != 0 {
                self.heap_len += 1;
                self.heap[self.heap_len as usize] = n;
                max_code = n;
                self.depth[n as usize] = 0;
            } else {
                tree[n as usize].len = 0;
            }
        }

        while self.heap_len < 2 {
            let new = if max_code < 2 {
                max_code += 1;
                max_code
            } else {
                0
            };
            self.heap_len += 1;
            self.heap[self.heap_len as usize] = new;
            tree[new as usize].freq = 1;
            self.depth[new as usize] = 0;
            self.opt_len -= 1;
            if let Some(stree) = stree {
                self.static_len -= stree[new as usize].len as u64;
            }
        }
        desc.max_code = max_code;

        for n in (1..=(self.heap_len / 2)).rev() {
            self.pqdownheap(tree, n);
        }

        loop {
            let n = self.heap[SMALLEST];
            self.heap[SMALLEST] = self.heap[self.heap_len as usize];
            self.heap_len -= 1;
            self.pqdownheap(tree, SMALLEST as i32);

            let m = self.heap[SMALLEST];
            self.heap[self.heap_max as usize] = n;
            self.heap_max -= 1;
            self.heap[self.heap_max as usize] = m;
            self.heap_max -= 1;

            node += 1;
            tree[node as usize].freq = tree[n as usize].freq + tree[m as usize].freq;
            self.depth[node as usize] = (max(self.depth[n as usize], self.depth[m as usize]) + 1) as u8;
            tree[n as usize].dad = node as u16;
            tree[m as usize].dad = node as u16;

            self.heap[SMALLEST] = node;
            self.pqdownheap(tree, SMALLEST as i32);

            if self.heap_len < 2 {
                break;
            }
        }

        self.heap_max -= 1;
        self.heap[self.heap_max as usize] = self.heap[SMALLEST];

        self.gen_bitlen(desc);
        self.gen_codes(tree, max_code as usize);
    }
}