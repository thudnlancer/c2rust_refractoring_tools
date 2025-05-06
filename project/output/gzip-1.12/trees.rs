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
    fn warning(m: *const i8);
    fn gzip_error(m: *const i8);
    static mut inbuf: [uch; 0];
    static mut d_buf: [ush; 0];
    static mut level: i32;
    fn bi_windup();
    fn send_bits(value: i32, length: i32);
    fn copy_block(buf: *mut i8, len: u32, header: i32);
    fn bi_reverse(value: u32, length: i32) -> u32;
    static mut block_start: i64;
    static mut strstart: u32;
}
pub type __off_t = i64;
pub type off_t = __off_t;
pub type uch = u8;
pub type ush = libc::c_ushort;
pub type ulg = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub freq: ush,
    pub code: ush,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ct_data {
    pub fc: C2RustUnnamed,
    pub dl: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub dad: ush,
    pub len: ush,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tree_desc {
    pub dyn_tree: *mut ct_data,
    pub static_tree: *mut ct_data,
    pub extra_bits: *mut i32,
    pub extra_base: i32,
    pub elems: i32,
    pub max_length: i32,
    pub max_code: i32,
}
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
static mut dyn_ltree: [ct_data; 573] = [ct_data {
    fc: C2RustUnnamed { freq: 0 },
    dl: C2RustUnnamed_0 { dad: 0 },
}; 573];
static mut dyn_dtree: [ct_data; 61] = [ct_data {
    fc: C2RustUnnamed { freq: 0 },
    dl: C2RustUnnamed_0 { dad: 0 },
}; 61];
static mut static_ltree: [ct_data; 288] = [ct_data {
    fc: C2RustUnnamed { freq: 0 },
    dl: C2RustUnnamed_0 { dad: 0 },
}; 288];
static mut static_dtree: [ct_data; 30] = [ct_data {
    fc: C2RustUnnamed { freq: 0 },
    dl: C2RustUnnamed_0 { dad: 0 },
}; 30];
static mut bl_tree: [ct_data; 39] = [ct_data {
    fc: C2RustUnnamed { freq: 0 },
    dl: C2RustUnnamed_0 { dad: 0 },
}; 39];
static mut l_desc: tree_desc = unsafe {
    {
        let mut init = tree_desc {
            dyn_tree: dyn_ltree.as_ptr() as *mut _,
            static_tree: static_ltree.as_ptr() as *mut _,
            extra_bits: extra_lbits.as_ptr() as *mut _,
            extra_base: 256 as i32 + 1 as i32,
            elems: 256 as i32 + 1 as i32 + 29 as i32,
            max_length: 15 as i32,
            max_code: 0 as i32,
        };
        init
    }
};
static mut d_desc: tree_desc = unsafe {
    {
        let mut init = tree_desc {
            dyn_tree: dyn_dtree.as_ptr() as *mut _,
            static_tree: static_dtree.as_ptr() as *mut _,
            extra_bits: extra_dbits.as_ptr() as *mut _,
            extra_base: 0 as i32,
            elems: 30 as i32,
            max_length: 15 as i32,
            max_code: 0 as i32,
        };
        init
    }
};
static mut bl_desc: tree_desc = unsafe {
    {
        let mut init = tree_desc {
            dyn_tree: bl_tree.as_ptr() as *mut _,
            static_tree: 0 as *const ct_data as *mut ct_data,
            extra_bits: extra_blbits.as_ptr() as *mut _,
            extra_base: 0 as i32,
            elems: 19 as i32,
            max_length: 7 as i32,
            max_code: 0 as i32,
        };
        init
    }
};
static mut bl_count: [ush; 16] = [0; 16];
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
static mut heap: [i32; 573] = [0; 573];
static mut heap_len: i32 = 0;
static mut heap_max: i32 = 0;
static mut depth: [uch; 573] = [0; 573];
static mut length_code: [uch; 256] = [0; 256];
static mut dist_code: [uch; 512] = [0; 512];
static mut base_length: [i32; 29] = [0; 29];
static mut base_dist: [i32; 30] = [0; 30];
static mut flag_buf: [uch; 4096] = [0; 4096];
static mut last_lit: u32 = 0;
static mut last_dist: u32 = 0;
static mut last_flags: u32 = 0;
static mut flags: uch = 0;
static mut flag_bit: uch = 0;
static mut opt_len: ulg = 0;
static mut static_len: ulg = 0;
static mut compressed_len: off_t = 0;
static mut input_len: off_t = 0;
static mut file_type: *mut ush = 0 as *const ush as *mut ush;
static mut file_method: *mut i32 = 0 as *const i32 as *mut i32;
#[no_mangle]
pub unsafe extern "C" fn ct_init(mut attr: *mut ush, mut methodp: *mut i32) {
    let mut n: i32 = 0;
    let mut bits: i32 = 0;
    let mut length: i32 = 0;
    let mut code: i32 = 0;
    let mut dist: i32 = 0;
    file_type = attr;
    file_method = methodp;
    input_len = 0 as i64;
    compressed_len = input_len;
    if static_dtree[0 as i32 as usize].dl.len as i32 != 0 as i32 {
        return;
    }
    length = 0 as i32;
    code = 0 as i32;
    while code < 29 as i32 - 1 as i32 {
        base_length[code as usize] = length;
        n = 0 as i32;
        while n < (1 as i32) << extra_lbits[code as usize] {
            let fresh0 = length;
            length = length + 1;
            length_code[fresh0 as usize] = code as uch;
            n += 1;
            n;
        }
        code += 1;
        code;
    }
    length_code[(length - 1 as i32) as usize] = code as uch;
    dist = 0 as i32;
    code = 0 as i32;
    while code < 16 as i32 {
        base_dist[code as usize] = dist;
        n = 0 as i32;
        while n < (1 as i32) << extra_dbits[code as usize] {
            let fresh1 = dist;
            dist = dist + 1;
            dist_code[fresh1 as usize] = code as uch;
            n += 1;
            n;
        }
        code += 1;
        code;
    }
    dist >>= 7 as i32;
    while code < 30 as i32 {
        base_dist[code as usize] = dist << 7 as i32;
        n = 0 as i32;
        while n < (1 as i32) << extra_dbits[code as usize] - 7 as i32 {
            let fresh2 = dist;
            dist = dist + 1;
            dist_code[(256 as i32 + fresh2) as usize] = code as uch;
            n += 1;
            n;
        }
        code += 1;
        code;
    }
    bits = 0 as i32;
    while bits <= 15 as i32 {
        bl_count[bits as usize] = 0 as i32 as ush;
        bits += 1;
        bits;
    }
    n = 0 as i32;
    while n <= 143 as i32 {
        let fresh3 = n;
        n = n + 1;
        static_ltree[fresh3 as usize].dl.len = 8 as i32 as ush;
        bl_count[8 as i32 as usize] = (bl_count[8 as i32 as usize]).wrapping_add(1);
        bl_count[8 as i32 as usize];
    }
    while n <= 255 as i32 {
        let fresh4 = n;
        n = n + 1;
        static_ltree[fresh4 as usize].dl.len = 9 as i32 as ush;
        bl_count[9 as i32 as usize] = (bl_count[9 as i32 as usize]).wrapping_add(1);
        bl_count[9 as i32 as usize];
    }
    while n <= 279 as i32 {
        let fresh5 = n;
        n = n + 1;
        static_ltree[fresh5 as usize].dl.len = 7 as i32 as ush;
        bl_count[7 as i32 as usize] = (bl_count[7 as i32 as usize]).wrapping_add(1);
        bl_count[7 as i32 as usize];
    }
    while n <= 287 as i32 {
        let fresh6 = n;
        n = n + 1;
        static_ltree[fresh6 as usize].dl.len = 8 as i32 as ush;
        bl_count[8 as i32 as usize] = (bl_count[8 as i32 as usize]).wrapping_add(1);
        bl_count[8 as i32 as usize];
    }
    gen_codes(static_ltree.as_mut_ptr(), 256 as i32 + 1 as i32 + 29 as i32 + 1 as i32);
    n = 0 as i32;
    while n < 30 as i32 {
        static_dtree[n as usize].dl.len = 5 as i32 as ush;
        static_dtree[n as usize].fc.code = bi_reverse(n as u32, 5 as i32) as ush;
        n += 1;
        n;
    }
    init_block();
}
unsafe extern "C" fn init_block() {
    let mut n: i32 = 0;
    n = 0 as i32;
    while n < 256 as i32 + 1 as i32 + 29 as i32 {
        dyn_ltree[n as usize].fc.freq = 0 as i32 as ush;
        n += 1;
        n;
    }
    n = 0 as i32;
    while n < 30 as i32 {
        dyn_dtree[n as usize].fc.freq = 0 as i32 as ush;
        n += 1;
        n;
    }
    n = 0 as i32;
    while n < 19 as i32 {
        bl_tree[n as usize].fc.freq = 0 as i32 as ush;
        n += 1;
        n;
    }
    dyn_ltree[256 as i32 as usize].fc.freq = 1 as i32 as ush;
    static_len = 0 as i64 as ulg;
    opt_len = static_len;
    last_flags = 0 as i32 as u32;
    last_dist = last_flags;
    last_lit = last_dist;
    flags = 0 as i32 as uch;
    flag_bit = 1 as i32 as uch;
}
unsafe extern "C" fn pqdownheap(mut tree: *mut ct_data, mut k: i32) {
    let mut v: i32 = heap[k as usize];
    let mut j: i32 = k << 1 as i32;
    while j <= heap_len {
        if j < heap_len
            && (((*tree.offset(heap[(j + 1 as i32) as usize] as isize)).fc.freq as i32)
                < (*tree.offset(heap[j as usize] as isize)).fc.freq as i32
                || (*tree.offset(heap[(j + 1 as i32) as usize] as isize)).fc.freq as i32
                    == (*tree.offset(heap[j as usize] as isize)).fc.freq as i32
                    && depth[heap[(j + 1 as i32) as usize] as usize] as i32
                        <= depth[heap[j as usize] as usize] as i32)
        {
            j += 1;
            j;
        }
        if ((*tree.offset(v as isize)).fc.freq as i32)
            < (*tree.offset(heap[j as usize] as isize)).fc.freq as i32
            || (*tree.offset(v as isize)).fc.freq as i32
                == (*tree.offset(heap[j as usize] as isize)).fc.freq as i32
                && depth[v as usize] as i32 <= depth[heap[j as usize] as usize] as i32
        {
            break;
        }
        heap[k as usize] = heap[j as usize];
        k = j;
        j <<= 1 as i32;
    }
    heap[k as usize] = v;
}
unsafe extern "C" fn gen_bitlen(mut desc: *mut tree_desc) {
    let mut tree: *mut ct_data = (*desc).dyn_tree;
    let mut extra: *mut i32 = (*desc).extra_bits;
    let mut base: i32 = (*desc).extra_base;
    let mut max_code: i32 = (*desc).max_code;
    let mut max_length: i32 = (*desc).max_length;
    let mut stree: *mut ct_data = (*desc).static_tree;
    let mut h: i32 = 0;
    let mut n: i32 = 0;
    let mut m: i32 = 0;
    let mut bits: i32 = 0;
    let mut xbits: i32 = 0;
    let mut f: ush = 0;
    let mut overflow: i32 = 0 as i32;
    bits = 0 as i32;
    while bits <= 15 as i32 {
        bl_count[bits as usize] = 0 as i32 as ush;
        bits += 1;
        bits;
    }
    (*tree.offset(heap[heap_max as usize] as isize)).dl.len = 0 as i32 as ush;
    h = heap_max + 1 as i32;
    while h < 2 as i32 * (256 as i32 + 1 as i32 + 29 as i32) + 1 as i32 {
        n = heap[h as usize];
        bits = (*tree.offset((*tree.offset(n as isize)).dl.dad as isize)).dl.len as i32
            + 1 as i32;
        if bits > max_length {
            bits = max_length;
            overflow += 1;
            overflow;
        }
        (*tree.offset(n as isize)).dl.len = bits as ush;
        if !(n > max_code) {
            bl_count[bits as usize] = (bl_count[bits as usize]).wrapping_add(1);
            bl_count[bits as usize];
            xbits = 0 as i32;
            if n >= base {
                xbits = *extra.offset((n - base) as isize);
            }
            f = (*tree.offset(n as isize)).fc.freq;
            opt_len = (opt_len as u64)
                .wrapping_add((f as ulg).wrapping_mul((bits + xbits) as u64)) as ulg
                as ulg;
            if !stree.is_null() {
                static_len = (static_len as u64)
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
        while bl_count[bits as usize] as i32 == 0 as i32 {
            bits -= 1;
            bits;
        }
        bl_count[bits as usize] = (bl_count[bits as usize]).wrapping_sub(1);
        bl_count[bits as usize];
        bl_count[(bits + 1 as i32) as usize] = (bl_count[(bits + 1 as i32) as usize]
            as i32 + 2 as i32) as ush;
        bl_count[max_length as usize] = (bl_count[max_length as usize]).wrapping_sub(1);
        bl_count[max_length as usize];
        overflow -= 2 as i32;
        if !(overflow > 0 as i32) {
            break;
        }
    }
    bits = max_length;
    while bits != 0 as i32 {
        n = bl_count[bits as usize] as i32;
        while n != 0 as i32 {
            h -= 1;
            m = heap[h as usize];
            if m > max_code {
                continue;
            }
            if (*tree.offset(m as isize)).dl.len as u32 != bits as u32 {
                opt_len = (opt_len as u64)
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
unsafe extern "C" fn gen_codes(mut tree: *mut ct_data, mut max_code: i32) {
    let mut next_code: [ush; 16] = [0; 16];
    let mut code: ush = 0 as i32 as ush;
    let mut bits: i32 = 0;
    let mut n: i32 = 0;
    bits = 1 as i32;
    while bits <= 15 as i32 {
        code = ((code as i32 + bl_count[(bits - 1 as i32) as usize] as i32) << 1 as i32)
            as ush;
        next_code[bits as usize] = code;
        bits += 1;
        bits;
    }
    n = 0 as i32;
    while n <= max_code {
        let mut len: i32 = (*tree.offset(n as isize)).dl.len as i32;
        if !(len == 0 as i32) {
            let fresh7 = next_code[len as usize];
            next_code[len as usize] = (next_code[len as usize]).wrapping_add(1);
            (*tree.offset(n as isize)).fc.code = bi_reverse(fresh7 as u32, len) as ush;
        }
        n += 1;
        n;
    }
}
unsafe extern "C" fn build_tree(mut desc: *mut tree_desc) {
    let mut tree: *mut ct_data = (*desc).dyn_tree;
    let mut stree: *mut ct_data = (*desc).static_tree;
    let mut elems: i32 = (*desc).elems;
    let mut n: i32 = 0;
    let mut m: i32 = 0;
    let mut max_code: i32 = -(1 as i32);
    let mut node: i32 = elems;
    heap_len = 0 as i32;
    heap_max = 2 as i32 * (256 as i32 + 1 as i32 + 29 as i32) + 1 as i32;
    n = 0 as i32;
    while n < elems {
        if (*tree.offset(n as isize)).fc.freq as i32 != 0 as i32 {
            max_code = n;
            heap_len += 1;
            heap[heap_len as usize] = max_code;
            depth[n as usize] = 0 as i32 as uch;
        } else {
            (*tree.offset(n as isize)).dl.len = 0 as i32 as ush;
        }
        n += 1;
        n;
    }
    while heap_len < 2 as i32 {
        heap_len += 1;
        heap[heap_len as usize] = if max_code < 2 as i32 {
            max_code += 1;
            max_code
        } else {
            0 as i32
        };
        let mut new: i32 = heap[heap_len as usize];
        (*tree.offset(new as isize)).fc.freq = 1 as i32 as ush;
        depth[new as usize] = 0 as i32 as uch;
        opt_len = opt_len.wrapping_sub(1);
        opt_len;
        if !stree.is_null() {
            static_len = (static_len as u64)
                .wrapping_sub((*stree.offset(new as isize)).dl.len as u64) as ulg as ulg;
        }
    }
    (*desc).max_code = max_code;
    n = heap_len / 2 as i32;
    while n >= 1 as i32 {
        pqdownheap(tree, n);
        n -= 1;
        n;
    }
    loop {
        n = heap[1 as i32 as usize];
        let fresh8 = heap_len;
        heap_len = heap_len - 1;
        heap[1 as i32 as usize] = heap[fresh8 as usize];
        pqdownheap(tree, 1 as i32);
        m = heap[1 as i32 as usize];
        heap_max -= 1;
        heap[heap_max as usize] = n;
        heap_max -= 1;
        heap[heap_max as usize] = m;
        (*tree.offset(node as isize)).fc.freq = ((*tree.offset(n as isize)).fc.freq
            as i32 + (*tree.offset(m as isize)).fc.freq as i32) as ush;
        depth[node as usize] = ((if depth[n as usize] as i32 >= depth[m as usize] as i32
        {
            depth[n as usize] as i32
        } else {
            depth[m as usize] as i32
        }) + 1 as i32) as uch;
        let ref mut fresh9 = (*tree.offset(m as isize)).dl.dad;
        *fresh9 = node as ush;
        (*tree.offset(n as isize)).dl.dad = *fresh9;
        let fresh10 = node;
        node = node + 1;
        heap[1 as i32 as usize] = fresh10;
        pqdownheap(tree, 1 as i32);
        if !(heap_len >= 2 as i32) {
            break;
        }
    }
    heap_max -= 1;
    heap[heap_max as usize] = heap[1 as i32 as usize];
    gen_bitlen(desc);
    gen_codes(tree, max_code);
}
unsafe extern "C" fn scan_tree(mut tree: *mut ct_data, mut max_code: i32) {
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
                bl_tree[curlen as usize].fc.freq = (bl_tree[curlen as usize].fc.freq
                    as i32 + count) as ush;
            } else if curlen != 0 as i32 {
                if curlen != prevlen {
                    bl_tree[curlen as usize].fc.freq = (bl_tree[curlen as usize].fc.freq)
                        .wrapping_add(1);
                    bl_tree[curlen as usize].fc.freq;
                }
                bl_tree[16 as i32 as usize].fc.freq = (bl_tree[16 as i32 as usize]
                    .fc
                    .freq)
                    .wrapping_add(1);
                bl_tree[16 as i32 as usize].fc.freq;
            } else if count <= 10 as i32 {
                bl_tree[17 as i32 as usize].fc.freq = (bl_tree[17 as i32 as usize]
                    .fc
                    .freq)
                    .wrapping_add(1);
                bl_tree[17 as i32 as usize].fc.freq;
            } else {
                bl_tree[18 as i32 as usize].fc.freq = (bl_tree[18 as i32 as usize]
                    .fc
                    .freq)
                    .wrapping_add(1);
                bl_tree[18 as i32 as usize].fc.freq;
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
unsafe extern "C" fn send_tree(mut tree: *mut ct_data, mut max_code: i32) {
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
                    send_bits(
                        bl_tree[curlen as usize].fc.code as i32,
                        bl_tree[curlen as usize].dl.len as i32,
                    );
                    count -= 1;
                    if !(count != 0 as i32) {
                        break;
                    }
                }
            } else if curlen != 0 as i32 {
                if curlen != prevlen {
                    send_bits(
                        bl_tree[curlen as usize].fc.code as i32,
                        bl_tree[curlen as usize].dl.len as i32,
                    );
                    count -= 1;
                    count;
                }
                send_bits(
                    bl_tree[16 as i32 as usize].fc.code as i32,
                    bl_tree[16 as i32 as usize].dl.len as i32,
                );
                send_bits(count - 3 as i32, 2 as i32);
            } else if count <= 10 as i32 {
                send_bits(
                    bl_tree[17 as i32 as usize].fc.code as i32,
                    bl_tree[17 as i32 as usize].dl.len as i32,
                );
                send_bits(count - 3 as i32, 3 as i32);
            } else {
                send_bits(
                    bl_tree[18 as i32 as usize].fc.code as i32,
                    bl_tree[18 as i32 as usize].dl.len as i32,
                );
                send_bits(count - 11 as i32, 7 as i32);
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
unsafe extern "C" fn build_bl_tree() -> i32 {
    let mut max_blindex: i32 = 0;
    scan_tree(dyn_ltree.as_mut_ptr(), l_desc.max_code);
    scan_tree(dyn_dtree.as_mut_ptr(), d_desc.max_code);
    build_tree(&mut bl_desc as *mut tree_desc);
    max_blindex = 19 as i32 - 1 as i32;
    while max_blindex >= 3 as i32 {
        if bl_tree[bl_order[max_blindex as usize] as usize].dl.len as i32 != 0 as i32 {
            break;
        }
        max_blindex -= 1;
        max_blindex;
    }
    opt_len = (opt_len as u64)
        .wrapping_add(
            (3 as i32 * (max_blindex + 1 as i32) + 5 as i32 + 5 as i32 + 4 as i32) as u64,
        ) as ulg as ulg;
    return max_blindex;
}
unsafe extern "C" fn send_all_trees(mut lcodes: i32, mut dcodes: i32, mut blcodes: i32) {
    let mut rank: i32 = 0;
    send_bits(lcodes - 257 as i32, 5 as i32);
    send_bits(dcodes - 1 as i32, 5 as i32);
    send_bits(blcodes - 4 as i32, 4 as i32);
    rank = 0 as i32;
    while rank < blcodes {
        send_bits(bl_tree[bl_order[rank as usize] as usize].dl.len as i32, 3 as i32);
        rank += 1;
        rank;
    }
    send_tree(dyn_ltree.as_mut_ptr(), lcodes - 1 as i32);
    send_tree(dyn_dtree.as_mut_ptr(), dcodes - 1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn flush_block(
    mut buf: *mut i8,
    mut stored_len: ulg,
    mut pad: i32,
    mut eof: i32,
) -> off_t {
    let mut opt_lenb: ulg = 0;
    let mut static_lenb: ulg = 0;
    let mut max_blindex: i32 = 0;
    flag_buf[last_flags as usize] = flags;
    if *file_type as i32 == 0xffff as i32 as ush as i32 {
        set_file_type();
    }
    build_tree(&mut l_desc as *mut tree_desc);
    build_tree(&mut d_desc as *mut tree_desc);
    max_blindex = build_bl_tree();
    opt_lenb = opt_len.wrapping_add(3 as i32 as u64).wrapping_add(7 as i32 as u64)
        >> 3 as i32;
    static_lenb = static_len.wrapping_add(3 as i32 as u64).wrapping_add(7 as i32 as u64)
        >> 3 as i32;
    input_len = (input_len as u64).wrapping_add(stored_len) as off_t as off_t;
    if static_lenb <= opt_lenb {
        opt_lenb = static_lenb;
    }
    if stored_len <= opt_lenb && eof != 0 && compressed_len == 0 as i64 && 0 as i32 != 0
    {
        if buf.is_null() {
            gzip_error(b"block vanished\0" as *const u8 as *const i8);
        }
        copy_block(buf, stored_len as u32, 0 as i32);
        compressed_len = (stored_len << 3 as i32) as off_t;
        *file_method = 0 as i32;
    } else if stored_len.wrapping_add(4 as i32 as u64) <= opt_lenb && !buf.is_null() {
        send_bits(((0 as i32) << 1 as i32) + eof, 3 as i32);
        compressed_len = compressed_len + 3 as i32 as i64 + 7 as i32 as i64
            & !(7 as i64);
        compressed_len = (compressed_len as u64)
            .wrapping_add(stored_len.wrapping_add(4 as i32 as u64) << 3 as i32) as off_t
            as off_t;
        copy_block(buf, stored_len as u32, 1 as i32);
    } else if static_lenb == opt_lenb {
        send_bits(((1 as i32) << 1 as i32) + eof, 3 as i32);
        compress_block(static_ltree.as_mut_ptr(), static_dtree.as_mut_ptr());
        compressed_len = (compressed_len as u64)
            .wrapping_add((3 as i32 as u64).wrapping_add(static_len)) as off_t as off_t;
    } else {
        send_bits(((2 as i32) << 1 as i32) + eof, 3 as i32);
        send_all_trees(
            l_desc.max_code + 1 as i32,
            d_desc.max_code + 1 as i32,
            max_blindex + 1 as i32,
        );
        compress_block(dyn_ltree.as_mut_ptr(), dyn_dtree.as_mut_ptr());
        compressed_len = (compressed_len as u64)
            .wrapping_add((3 as i32 as u64).wrapping_add(opt_len)) as off_t as off_t;
    }
    init_block();
    if eof != 0 {
        bi_windup();
        compressed_len += 7 as i32 as i64;
    } else if pad != 0 && compressed_len % 8 as i32 as i64 != 0 as i32 as i64 {
        send_bits(((0 as i32) << 1 as i32) + eof, 3 as i32);
        compressed_len = compressed_len + 3 as i32 as i64 + 7 as i32 as i64
            & !(7 as i64);
        copy_block(buf, 0 as i32 as u32, 1 as i32);
    }
    return compressed_len >> 3 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn ct_tally(mut dist: i32, mut lc: i32) -> i32 {
    let fresh11 = last_lit;
    last_lit = last_lit.wrapping_add(1);
    *inbuf.as_mut_ptr().offset(fresh11 as isize) = lc as uch;
    if dist == 0 as i32 {
        dyn_ltree[lc as usize].fc.freq = (dyn_ltree[lc as usize].fc.freq)
            .wrapping_add(1);
        dyn_ltree[lc as usize].fc.freq;
    } else {
        dist -= 1;
        dist;
        dyn_ltree[(length_code[lc as usize] as i32 + 256 as i32 + 1 as i32) as usize]
            .fc
            .freq = (dyn_ltree[(length_code[lc as usize] as i32 + 256 as i32 + 1 as i32)
                as usize]
            .fc
            .freq)
            .wrapping_add(1);
        dyn_ltree[(length_code[lc as usize] as i32 + 256 as i32 + 1 as i32) as usize]
            .fc
            .freq;
        dyn_dtree[(if dist < 256 as i32 {
                dist_code[dist as usize] as i32
            } else {
                dist_code[(256 as i32 + (dist >> 7 as i32)) as usize] as i32
            }) as usize]
            .fc
            .freq = (dyn_dtree[(if dist < 256 as i32 {
                dist_code[dist as usize] as i32
            } else {
                dist_code[(256 as i32 + (dist >> 7 as i32)) as usize] as i32
            }) as usize]
            .fc
            .freq)
            .wrapping_add(1);
        dyn_dtree[(if dist < 256 as i32 {
                dist_code[dist as usize] as i32
            } else {
                dist_code[(256 as i32 + (dist >> 7 as i32)) as usize] as i32
            }) as usize]
            .fc
            .freq;
        let fresh12 = last_dist;
        last_dist = last_dist.wrapping_add(1);
        *d_buf.as_mut_ptr().offset(fresh12 as isize) = dist as ush;
        flags = (flags as i32 | flag_bit as i32) as uch;
    }
    flag_bit = ((flag_bit as i32) << 1 as i32) as uch;
    if last_lit & 7 as i32 as u32 == 0 as i32 as u32 {
        let fresh13 = last_flags;
        last_flags = last_flags.wrapping_add(1);
        flag_buf[fresh13 as usize] = flags;
        flags = 0 as i32 as uch;
        flag_bit = 1 as i32 as uch;
    }
    if level > 2 as i32 && last_lit & 0xfff as i32 as u32 == 0 as i32 as u32 {
        let mut out_length: ulg = (last_lit as ulg).wrapping_mul(8 as i64 as u64);
        let mut in_length: ulg = (strstart as ulg).wrapping_sub(block_start as u64);
        let mut dcode: i32 = 0;
        dcode = 0 as i32;
        while dcode < 30 as i32 {
            out_length = (out_length as u64)
                .wrapping_add(
                    (dyn_dtree[dcode as usize].fc.freq as ulg)
                        .wrapping_mul(
                            (5 as i64 + extra_dbits[dcode as usize] as i64) as u64,
                        ),
                ) as ulg as ulg;
            dcode += 1;
            dcode;
        }
        out_length >>= 3 as i32;
        if last_dist < last_lit.wrapping_div(2 as i32 as u32)
            && out_length < in_length.wrapping_div(2 as i32 as u64)
        {
            return 1 as i32;
        }
    }
    return (last_lit == (0x8000 as i32 - 1 as i32) as u32
        || last_dist == 0x8000 as i32 as u32) as i32;
}
unsafe extern "C" fn compress_block(mut ltree: *mut ct_data, mut dtree: *mut ct_data) {
    let mut dist: u32 = 0;
    let mut lc: i32 = 0;
    let mut lx: u32 = 0 as i32 as u32;
    let mut dx: u32 = 0 as i32 as u32;
    let mut fx: u32 = 0 as i32 as u32;
    let mut flag: uch = 0 as i32 as uch;
    let mut code: u32 = 0;
    let mut extra: i32 = 0;
    if last_lit != 0 as i32 as u32 {
        loop {
            if lx & 7 as i32 as u32 == 0 as i32 as u32 {
                let fresh14 = fx;
                fx = fx.wrapping_add(1);
                flag = flag_buf[fresh14 as usize];
            }
            let fresh15 = lx;
            lx = lx.wrapping_add(1);
            lc = *inbuf.as_mut_ptr().offset(fresh15 as isize) as i32;
            if flag as i32 & 1 as i32 == 0 as i32 {
                send_bits(
                    (*ltree.offset(lc as isize)).fc.code as i32,
                    (*ltree.offset(lc as isize)).dl.len as i32,
                );
            } else {
                code = length_code[lc as usize] as u32;
                send_bits(
                    (*ltree
                        .offset(
                            code
                                .wrapping_add(256 as i32 as u32)
                                .wrapping_add(1 as i32 as u32) as isize,
                        ))
                        .fc
                        .code as i32,
                    (*ltree
                        .offset(
                            code
                                .wrapping_add(256 as i32 as u32)
                                .wrapping_add(1 as i32 as u32) as isize,
                        ))
                        .dl
                        .len as i32,
                );
                extra = extra_lbits[code as usize];
                if extra != 0 as i32 {
                    lc -= base_length[code as usize];
                    send_bits(lc, extra);
                }
                let fresh16 = dx;
                dx = dx.wrapping_add(1);
                dist = *d_buf.as_mut_ptr().offset(fresh16 as isize) as u32;
                code = (if dist < 256 as i32 as u32 {
                    dist_code[dist as usize] as i32
                } else {
                    dist_code[(256 as i32 as u32).wrapping_add(dist >> 7 as i32)
                        as usize] as i32
                }) as u32;
                send_bits(
                    (*dtree.offset(code as isize)).fc.code as i32,
                    (*dtree.offset(code as isize)).dl.len as i32,
                );
                extra = extra_dbits[code as usize];
                if extra != 0 as i32 {
                    dist = dist.wrapping_sub(base_dist[code as usize] as u32);
                    send_bits(dist as i32, extra);
                }
            }
            flag = (flag as i32 >> 1 as i32) as uch;
            if !(lx < last_lit) {
                break;
            }
        }
    }
    send_bits(
        (*ltree.offset(256 as i32 as isize)).fc.code as i32,
        (*ltree.offset(256 as i32 as isize)).dl.len as i32,
    );
}
unsafe extern "C" fn set_file_type() {
    let mut n: i32 = 0 as i32;
    let mut ascii_freq: u32 = 0 as i32 as u32;
    let mut bin_freq: u32 = 0 as i32 as u32;
    while n < 7 as i32 {
        let fresh17 = n;
        n = n + 1;
        bin_freq = bin_freq.wrapping_add(dyn_ltree[fresh17 as usize].fc.freq as u32);
    }
    while n < 128 as i32 {
        let fresh18 = n;
        n = n + 1;
        ascii_freq = ascii_freq.wrapping_add(dyn_ltree[fresh18 as usize].fc.freq as u32);
    }
    while n < 256 as i32 {
        let fresh19 = n;
        n = n + 1;
        bin_freq = bin_freq.wrapping_add(dyn_ltree[fresh19 as usize].fc.freq as u32);
    }
    *file_type = (if bin_freq > ascii_freq >> 2 as i32 { 0 as i32 } else { 1 as i32 })
        as ush;
    if *file_type as i32 == 0 as i32 && 0 as i32 != 0 {
        warning(b"-l used on binary file\0" as *const u8 as *const i8);
    }
}