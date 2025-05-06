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
    static mut inbuf: [uch; 0];
    static mut outbuf: [uch; 0];
    static mut window: [uch; 0];
    static mut insize: u32;
    static mut inptr: u32;
    static mut outcnt: u32;
    static mut bytes_out: off_t;
    static mut ifd: i32;
    static mut ofd: i32;
    fn gzip_error(m: *const i8);
    fn flush_window();
    fn fill_inbuf(eof_ok: i32) -> i32;
}
pub type __off_t = i64;
pub type off_t = __off_t;
pub type uch = u8;
pub type ulg = u64;
static mut orig_len: ulg = 0;
static mut max_len: i32 = 0;
static mut literal: [uch; 256] = [0; 256];
static mut lit_base: [i32; 26] = [0; 26];
static mut leaves: [i32; 26] = [0; 26];
static mut parents: [i32; 26] = [0; 26];
static mut peek_bits: i32 = 0;
static mut bitbuf: ulg = 0;
static mut valid: i32 = 0;
unsafe extern "C" fn read_byte() -> u8 {
    let mut b: i32 = if inptr < insize {
        let fresh0 = inptr;
        inptr = inptr.wrapping_add(1);
        *inbuf.as_mut_ptr().offset(fresh0 as isize) as i32
    } else {
        fill_inbuf(0 as i32)
    };
    if b < 0 as i32 {
        gzip_error(
            b"invalid compressed data -- unexpected end of file\0" as *const u8
                as *const i8,
        );
    }
    return b as u8;
}
unsafe extern "C" fn read_tree() {
    let mut len: i32 = 0;
    let mut base: i32 = 0;
    let mut n: i32 = 0;
    let mut max_leaves: i32 = 1 as i32;
    orig_len = 0 as i32 as ulg;
    n = 1 as i32;
    while n <= 4 as i32 {
        orig_len = orig_len << 8 as i32 | read_byte() as u64;
        n += 1;
        n;
    }
    max_len = read_byte() as i32;
    if !((0 as i32) < max_len && max_len <= 25 as i32) {
        gzip_error(
            b"invalid compressed data -- Huffman code bit length out of range\0"
                as *const u8 as *const i8,
        );
    }
    n = 0 as i32;
    len = 1 as i32;
    while len <= max_len {
        leaves[len as usize] = read_byte() as i32;
        if (max_leaves - (len == max_len) as i32) < leaves[len as usize] {
            gzip_error(b"too many leaves in Huffman tree\0" as *const u8 as *const i8);
        }
        max_leaves = (max_leaves - leaves[len as usize] + 1 as i32) * 2 as i32
            - 1 as i32;
        n += leaves[len as usize];
        len += 1;
        len;
    }
    if 256 as i32 <= n {
        gzip_error(b"too many leaves in Huffman tree\0" as *const u8 as *const i8);
    }
    leaves[max_len as usize] += 1;
    leaves[max_len as usize];
    base = 0 as i32;
    len = 1 as i32;
    while len <= max_len {
        lit_base[len as usize] = base;
        n = leaves[len as usize];
        while n > 0 as i32 {
            let fresh1 = base;
            base = base + 1;
            literal[fresh1 as usize] = read_byte();
            n -= 1;
            n;
        }
        len += 1;
        len;
    }
    leaves[max_len as usize] += 1;
    leaves[max_len as usize];
}
unsafe extern "C" fn build_tree() {
    let mut nodes: i32 = 0 as i32;
    let mut len: i32 = 0;
    let mut prefixp: *mut uch = 0 as *mut uch;
    len = max_len;
    while len >= 1 as i32 {
        nodes >>= 1 as i32;
        parents[len as usize] = nodes;
        lit_base[len as usize] -= nodes;
        nodes += leaves[len as usize];
        len -= 1;
        len;
    }
    if nodes >> 1 as i32 != 1 as i32 {
        gzip_error(b"too few leaves in Huffman tree\0" as *const u8 as *const i8);
    }
    peek_bits = if max_len <= 12 as i32 { max_len } else { 12 as i32 };
    prefixp = &mut *outbuf.as_mut_ptr().offset(((1 as i32) << peek_bits) as isize)
        as *mut uch;
    len = 1 as i32;
    while len <= peek_bits {
        let mut prefixes: i32 = leaves[len as usize] << peek_bits - len;
        loop {
            let fresh2 = prefixes;
            prefixes = prefixes - 1;
            if !(fresh2 != 0) {
                break;
            }
            prefixp = prefixp.offset(-1);
            *prefixp = len as uch;
        }
        len += 1;
        len;
    }
    while prefixp > outbuf.as_mut_ptr() {
        prefixp = prefixp.offset(-1);
        *prefixp = 0 as i32 as uch;
    }
}
#[no_mangle]
pub unsafe extern "C" fn unpack(mut in_0: i32, mut out: i32) -> i32 {
    let mut len: i32 = 0;
    let mut eob: u32 = 0;
    let mut peek: u32 = 0;
    let mut peek_mask: u32 = 0;
    ifd = in_0;
    ofd = out;
    read_tree();
    build_tree();
    valid = 0 as i32;
    bitbuf = 0 as i32 as ulg;
    peek_mask = (((1 as i32) << peek_bits) - 1 as i32) as u32;
    eob = (leaves[max_len as usize] - 1 as i32) as u32;
    loop {
        while valid < peek_bits {
            bitbuf = bitbuf << 8 as i32 | read_byte() as u64;
            valid += 8 as i32;
        }
        peek = (bitbuf >> valid - peek_bits & peek_mask as u64) as u32;
        len = *outbuf.as_mut_ptr().offset(peek as isize) as i32;
        if len > 0 as i32 {
            peek >>= peek_bits - len;
        } else {
            let mut mask: ulg = peek_mask as ulg;
            len = peek_bits;
            while peek < parents[len as usize] as u32 {
                len += 1;
                len;
                mask = (mask << 1 as i32).wrapping_add(1 as i32 as u64);
                while valid < len {
                    bitbuf = bitbuf << 8 as i32 | read_byte() as u64;
                    valid += 8 as i32;
                }
                peek = (bitbuf >> valid - len & mask) as u32;
            }
        }
        if peek == eob && len == max_len {
            break;
        }
        let fresh3 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *window.as_mut_ptr().offset(fresh3 as isize) = literal[peek
            .wrapping_add(lit_base[len as usize] as u32) as usize];
        if outcnt == 0x8000 as i32 as u32 {
            flush_window();
        }
        valid -= len;
    }
    flush_window();
    if orig_len != (bytes_out & 0xffffffff as u32 as i64) as ulg {
        gzip_error(b"invalid compressed data--length error\0" as *const u8 as *const i8);
    }
    return 0 as i32;
}