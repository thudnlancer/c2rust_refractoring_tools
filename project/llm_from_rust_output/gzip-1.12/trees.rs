use std::mem;

type Uch = u8;
type Ush = u16;
type Ulg = u32;
type OffT = i64;

#[derive(Clone, Copy)]
struct CtData {
    fc: Ush,
    dl: Ush,
}

struct TreeDesc {
    dyn_tree: Vec<CtData>,
    static_tree: Vec<CtData>,
    extra_bits: Vec<i32>,
    extra_base: i32,
    elems: i32,
    max_length: i32,
    max_code: i32,
}

static EXTRA_LBITS: [i32; 29] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
];

static EXTRA_DBITS: [i32; 30] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13,
];

static EXTRA_BLBITS: [i32; 19] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 7,
];

static BL_ORDER: [Uch; 19] = [
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];

struct CompressionState {
    dyn_ltree: Vec<CtData>,
    dyn_dtree: Vec<CtData>,
    static_ltree: Vec<CtData>,
    static_dtree: Vec<CtData>,
    bl_tree: Vec<CtData>,
    l_desc: TreeDesc,
    d_desc: TreeDesc,
    bl_desc: TreeDesc,
    bl_count: [Ush; 16],
    heap: [i32; 573],
    heap_len: i32,
    heap_max: i32,
    depth: [Uch; 573],
    length_code: [Uch; 256],
    dist_code: [Uch; 512],
    base_length: [i32; 29],
    base_dist: [i32; 30],
    flag_buf: [Uch; 4096],
    last_lit: u32,
    last_dist: u32,
    last_flags: u32,
    flags: Uch,
    flag_bit: Uch,
    opt_len: Ulg,
    static_len: Ulg,
    compressed_len: OffT,
    input_len: OffT,
    file_type: Ush,
    file_method: i32,
    block_start: i64,
    strstart: u32,
    level: i32,
    inbuf: Vec<Uch>,
    d_buf: Vec<Ush>,
}

impl CompressionState {
    fn new() -> Self {
        let mut static_ltree = vec![CtData { fc: 0, dl: 0 }; 288];
        let mut static_dtree = vec![CtData { fc: 0, dl: 0 }; 30];
        
        // Initialize static_ltree
        for n in 0..=143 {
            static_ltree[n].dl = 8;
        }
        for n in 144..=255 {
            static_ltree[n].dl = 9;
        }
        for n in 256..=279 {
            static_ltree[n].dl = 7;
        }
        for n in 280..=287 {
            static_ltree[n].dl = 8;
        }
        
        // Initialize static_dtree
        for n in 0..30 {
            static_dtree[n].dl = 5;
            static_dtree[n].fc = bi_reverse(n as u32, 5) as Ush;
        }
        
        CompressionState {
            dyn_ltree: vec![CtData { fc: 0, dl: 0 }; 573],
            dyn_dtree: vec![CtData { fc: 0, dl: 0 }; 61],
            static_ltree,
            static_dtree,
            bl_tree: vec![CtData { fc: 0, dl: 0 }; 39],
            l_desc: TreeDesc {
                dyn_tree: Vec::new(),
                static_tree: Vec::new(),
                extra_bits: EXTRA_LBITS.to_vec(),
                extra_base: 257,
                elems: 286,
                max_length: 15,
                max_code: 0,
            },
            d_desc: TreeDesc {
                dyn_tree: Vec::new(),
                static_tree: Vec::new(),
                extra_bits: EXTRA_DBITS.to_vec(),
                extra_base: 0,
                elems: 30,
                max_length: 15,
                max_code: 0,
            },
            bl_desc: TreeDesc {
                dyn_tree: Vec::new(),
                static_tree: Vec::new(),
                extra_bits: EXTRA_BLBITS.to_vec(),
                extra_base: 0,
                elems: 19,
                max_length: 7,
                max_code: 0,
            },
            bl_count: [0; 16],
            heap: [0; 573],
            heap_len: 0,
            heap_max: 0,
            depth: [0; 573],
            length_code: [0; 256],
            dist_code: [0; 512],
            base_length: [0; 29],
            base_dist: [0; 30],
            flag_buf: [0; 4096],
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
            block_start: 0,
            strstart: 0,
            level: 0,
            inbuf: Vec::new(),
            d_buf: Vec::new(),
        }
    }

    fn ct_init(&mut self, attr: Ush, method: i32) {
        self.file_type = attr;
        self.file_method = method;
        self.input_len = 0;
        self.compressed_len = 0;
        
        if self.static_dtree[0].dl != 0 {
            return;
        }
        
        // Initialize length codes
        let mut length = 0;
        let mut code = 0;
        while code < 28 {
            self.base_length[code as usize] = length;
            for n in 0..(1 << EXTRA_LBITS[code as usize]) {
                self.length_code[length as usize] = code as Uch;
                length += 1;
            }
            code += 1;
        }
        self.length_code[(length - 1) as usize] = code as Uch;
        
        // Initialize distance codes
        let mut dist = 0;
        code = 0;
        while code < 16 {
            self.base_dist[code as usize] = dist;
            for n in 0..(1 << EXTRA_DBITS[code as usize]) {
                self.dist_code[dist as usize] = code as Uch;
                dist += 1;
            }
            code += 1;
        }
        
        dist >>= 7;
        while code < 30 {
            self.base_dist[code as usize] = dist << 7;
            for n in 0..(1 << (EXTRA_DBITS[code as usize] - 7)) {
                self.dist_code[256 + dist as usize] = code as Uch;
                dist += 1;
            }
            code += 1;
        }
        
        // Initialize bl_count
        for bits in 0..=15 {
            self.bl_count[bits as usize] = 0;
        }
        
        self.init_block();
    }

    fn init_block(&mut self) {
        for n in 0..286 {
            self.dyn_ltree[n].fc = 0;
        }
        for n in 0..30 {
            self.dyn_dtree[n].fc = 0;
        }
        for n in 0..19 {
            self.bl_tree[n].fc = 0;
        }
        
        self.dyn_ltree[256].fc = 1;
        self.static_len = 0;
        self.opt_len = 0;
        self.last_flags = 0;
        self.last_dist = 0;
        self.last_lit = 0;
        self.flags = 0;
        self.flag_bit = 1;
    }

    fn pqdownheap(&mut self, tree: &[CtData], k: i32) {
        let v = self.heap[k as usize];
        let mut j = k << 1;
        
        while j <= self.heap_len {
            if j < self.heap_len && 
               (tree[self.heap[j as usize + 1] as usize].fc < tree[self.heap[j as usize] as usize].fc ||
                (tree[self.heap[j as usize + 1] as usize].fc == tree[self.heap[j as usize] as usize].fc &&
                 self.depth[self.heap[j as usize + 1] as usize] <= self.depth[self.heap[j as usize] as usize])) 
            {
                j += 1;
            }
            
            if tree[v as usize].fc < tree[self.heap[j as usize] as usize].fc ||
               (tree[v as usize].fc == tree[self.heap[j as usize] as usize].fc &&
                self.depth[v as usize] <= self.depth[self.heap[j as usize] as usize])
            {
                break;
            }
            
            self.heap[k as usize] = self.heap[j as usize];
            let k = j;
            j <<= 1;
        }
        
        self.heap[k as usize] = v;
    }

    fn gen_bitlen(&mut self, desc: &mut TreeDesc) {
        let tree = &mut desc.dyn_tree;
        let extra = &desc.extra_bits;
        let base = desc.extra_base;
        let max_code = desc.max_code;
        let max_length = desc.max_length;
        let stree = &desc.static_tree;
        
        let mut overflow = 0;
        
        for bits in 0..=15 {
            self.bl_count[bits as usize] = 0;
        }
        
        tree[self.heap[self.heap_max as usize] as usize].dl = 0;
        
        let mut h = self.heap_max + 1;
        while h < 2 * 286 + 1 {
            let n = self.heap[h as usize];
            let mut bits = tree[tree[n as usize].dl as usize as i32 + 1;
            
            if bits > max_length {
                bits = max_length;
                overflow += 1;
            }
            
            tree[n as usize].dl = bits as Ush;
            
            if n <= max_code {
                self.bl_count[bits as usize] += 1;
                let xbits = if n >= base { extra[(n - base) as usize] } else { 0 };
                let f = tree[n as usize].fc as Ulg;
                
                self.opt_len += f * (bits + xbits) as Ulg;
                
                if !stree.is_empty() {
                    self.static_len += f * (stree[n as usize].dl as i32 + xbits) as Ulg;
                }
            }
            
            h += 1;
        }
        
        if overflow == 0 {
            return;
        }
        
        loop {
            let mut bits = max_length - 1;
            while self.bl_count[bits as usize] == 0 {
                bits -= 1;
            }
            
            self.bl_count[bits as usize] -= 1;
            self.bl_count[bits as usize + 1] += 2;
            self.bl_count[max_length as usize] -= 1;
            
            overflow -= 2;
            if overflow <= 0 {
                break;
            }
        }
        
        let mut bits = max_length;
        while bits != 0 {
            let mut n = self.bl_count[bits as usize] as i32;
            while n != 0 {
                h -= 1;
                let m = self.heap[h as usize];
                if m > max_code {
                    continue;
                }
                
                if tree[m as usize].dl as i32 != bits {
                    self.opt_len += ((bits - tree[m as usize].dl as i32) * tree[m as usize].fc as i32) as Ulg;
                    tree[m as usize].dl = bits as Ush;
                }
                
                n -= 1;
            }
            
            bits -= 1;
        }
    }

    fn gen_codes(&mut self, tree: &mut [CtData], max_code: i32) {
        let mut next_code = [0; 16];
        let mut code = 0;
        
        for bits in 1..=15 {
            code = (code + self.bl_count[bits as usize - 1] as i32) << 1;
            next_code[bits as usize] = code as Ush;
        }
        
        for n in 0..=max_code {
            let len = tree[n as usize].dl as i32;
            if len != 0 {
                tree[n as usize].fc = bi_reverse(next_code[len as usize] as u32, len) as Ush;
                next_code[len as usize] += 1;
            }
        }
    }

    fn build_tree(&mut self, desc: &mut TreeDesc) {
        let tree = &mut desc.dyn_tree;
        let stree = &desc.static_tree;
        let elems = desc.elems;
        let mut max_code = -1;
        let mut node = elems;
        
        self.heap_len = 0;
        self.heap_max = 2 * 286 + 1;
        
        for n in 0..elems {
            if tree[n as usize].fc != 0 {
                max_code = n;
                self.heap_len += 1;
                self.heap[self.heap_len as usize] = n;
                self.depth[n as usize] = 0;
            } else {
                tree[n as usize].dl = 0;
            }
        }
        
        while self.heap_len < 2 {
            self.heap_len += 1;
            self.heap[self.heap_len as usize] = if max_code < 2 {
                max_code += 1;
                max_code
            } else {
                0
            };
            
            let new = self.heap[self.heap_len as usize];
            tree[new as usize].fc = 1;
            self.depth[new as usize] = 0;
            self.opt_len -= 1;
            
            if !stree.is_empty() {
                self.static_len -= stree[new as usize].dl as Ulg;
            }
        }
        
        desc.max_code = max_code;
        
        for n in (1..=self.heap_len / 2).rev() {
            self.pqdownheap(tree, n);
        }
        
        loop {
            let n = self.heap[1];
            self.heap[1] = self.heap[self.heap_len as usize];
            self.heap_len -= 1;
            self.pqdownheap(tree, 1);
            
            let m = self.heap[1];
            self.heap_max -= 1;
            self.heap[self.heap_max as usize] = n;
            self.heap_max -= 1;
            self.heap[self.heap_max as usize] = m;
            
            tree[node as usize].fc = (tree[n as usize].fc + tree[m as usize].fc) as Ush;
            self.depth[node as usize] = (self.depth[n as usize].max(self.depth[m as usize]) + 1;
            tree[m as usize].dl = node as Ush;
            tree[n as usize].dl = node as Ush;
            
            self.heap[1] = node;
            node += 1;
            self.pqdownheap(tree, 1);
            
            if self.heap_len < 2 {
                break;
            }
        }
        
        self.heap_max -= 1;
        self.heap[self.heap_max as usize] = self.heap[1];
        
        self.gen_bitlen(desc);
        self.gen_codes(tree, max_code);
    }

    fn scan_tree(&mut self, tree: &[CtData], max_code: i32) {
        let mut prevlen = -1;
        let mut nextlen = tree[0].dl as i32;
        let mut count = 0;
        let mut max_count = 7;
        let mut min_count = 4;
        
        if nextlen == 0 {
            max_count = 138;
            min_count = 3;
        }
        
        for n in 0..=max_code {
            let curlen = nextlen;
            nextlen = tree[(n + 1) as usize].dl as i32;
            count += 1;
            
            if count < max_count && curlen == nextlen {
                continue;
            }
            
            if count < min_count {
                self.bl_tree[curlen as usize].fc += count as Ush;
            } else if curlen != 0 {
                if curlen != prevlen {
                    self.bl_tree[curlen as usize].fc += 1;
                }
                self.bl_tree[16].fc += 1;
            } else if count <= 10 {
                self.bl_tree[17].fc += 1;
            } else {
                self.bl_tree[18].fc += 1;
            }
            
            count = 0;
            prevlen = curlen;
            
            if nextlen == 0 {
                max_count = 138;
                min_count = 3;
            } else if curlen