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
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    static mut window: [uch; 0];
    static mut prev: [ush; 0];
    static mut rsync: i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn flush_block(buf: *mut i8, stored_len: ulg, pad: i32, eof: i32) -> off_t;
    fn ct_tally(dist: i32, lc: i32) -> i32;
    static mut read_buf: Option<unsafe extern "C" fn(*mut i8, u32) -> i32>;
    fn gzip_error(m: *const i8);
}
pub type __off_t = i64;
pub type off_t = __off_t;
pub type voidp = *mut libc::c_void;
pub type uch = u8;
pub type ush = libc::c_ushort;
pub type ulg = u64;
pub type Pos = ush;
pub type IPos = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config {
    pub good_length: ush,
    pub max_lazy: ush,
    pub nice_length: ush,
    pub max_chain: ush,
}
static mut window_size: ulg = 0;
#[no_mangle]
pub static mut block_start: i64 = 0;
static mut ins_h: u32 = 0;
#[no_mangle]
pub static mut prev_length: u32 = 0;
#[no_mangle]
pub static mut strstart: u32 = 0;
#[no_mangle]
pub static mut match_start: u32 = 0;
static mut eofile: i32 = 0;
static mut lookahead: u32 = 0;
#[no_mangle]
pub static mut max_chain_length: u32 = 0;
static mut max_lazy_match: u32 = 0;
#[no_mangle]
pub static mut good_match: u32 = 0;
static mut rsync_sum: ulg = 0;
static mut rsync_chunk_end: ulg = 0;
static mut nice_match: i32 = 0;
static mut configuration_table: [config; 10] = [
    {
        let mut init = config {
            good_length: 0 as i32 as ush,
            max_lazy: 0 as i32 as ush,
            nice_length: 0 as i32 as ush,
            max_chain: 0 as i32 as ush,
        };
        init
    },
    {
        let mut init = config {
            good_length: 4 as i32 as ush,
            max_lazy: 4 as i32 as ush,
            nice_length: 8 as i32 as ush,
            max_chain: 4 as i32 as ush,
        };
        init
    },
    {
        let mut init = config {
            good_length: 4 as i32 as ush,
            max_lazy: 5 as i32 as ush,
            nice_length: 16 as i32 as ush,
            max_chain: 8 as i32 as ush,
        };
        init
    },
    {
        let mut init = config {
            good_length: 4 as i32 as ush,
            max_lazy: 6 as i32 as ush,
            nice_length: 32 as i32 as ush,
            max_chain: 32 as i32 as ush,
        };
        init
    },
    {
        let mut init = config {
            good_length: 4 as i32 as ush,
            max_lazy: 4 as i32 as ush,
            nice_length: 16 as i32 as ush,
            max_chain: 16 as i32 as ush,
        };
        init
    },
    {
        let mut init = config {
            good_length: 8 as i32 as ush,
            max_lazy: 16 as i32 as ush,
            nice_length: 32 as i32 as ush,
            max_chain: 32 as i32 as ush,
        };
        init
    },
    {
        let mut init = config {
            good_length: 8 as i32 as ush,
            max_lazy: 16 as i32 as ush,
            nice_length: 128 as i32 as ush,
            max_chain: 128 as i32 as ush,
        };
        init
    },
    {
        let mut init = config {
            good_length: 8 as i32 as ush,
            max_lazy: 32 as i32 as ush,
            nice_length: 128 as i32 as ush,
            max_chain: 256 as i32 as ush,
        };
        init
    },
    {
        let mut init = config {
            good_length: 32 as i32 as ush,
            max_lazy: 128 as i32 as ush,
            nice_length: 258 as i32 as ush,
            max_chain: 1024 as i32 as ush,
        };
        init
    },
    {
        let mut init = config {
            good_length: 32 as i32 as ush,
            max_lazy: 258 as i32 as ush,
            nice_length: 258 as i32 as ush,
            max_chain: 4096 as i32 as ush,
        };
        init
    },
];
unsafe extern "C" fn lm_init(mut pack_level: i32) {
    let mut j: u32 = 0;
    if pack_level < 1 as i32 || pack_level > 9 as i32 {
        gzip_error(b"bad pack level\0" as *const u8 as *const i8);
    }
    memset(
        prev.as_mut_ptr().offset(0x8000 as i32 as isize) as *mut i8 as voidp,
        0 as i32,
        (((1 as i32) << 15 as i32) as u32 as u64)
            .wrapping_mul(::core::mem::size_of::<ush>() as u64),
    );
    rsync_chunk_end = 0xffffffff as u64;
    rsync_sum = 0 as i32 as ulg;
    max_lazy_match = configuration_table[pack_level as usize].max_lazy as u32;
    good_match = configuration_table[pack_level as usize].good_length as u32;
    nice_match = configuration_table[pack_level as usize].nice_length as i32;
    max_chain_length = configuration_table[pack_level as usize].max_chain as u32;
    strstart = 0 as i32 as u32;
    block_start = 0 as i64;
    lookahead = read_buf
        .expect(
            "non-null function pointer",
        )(
        window.as_mut_ptr() as *mut i8,
        if ::core::mem::size_of::<i32>() as u64 <= 2 as i32 as u64 {
            0x8000 as i32 as u32
        } else {
            (2 as i32 * 0x8000 as i32) as u32
        },
    ) as u32;
    if lookahead == 0 as i32 as u32 || lookahead == -(1 as i32) as u32 {
        eofile = 1 as i32;
        lookahead = 0 as i32 as u32;
        return;
    }
    eofile = 0 as i32;
    while lookahead < (258 as i32 + 3 as i32 + 1 as i32) as u32 && eofile == 0 {
        fill_window();
    }
    ins_h = 0 as i32 as u32;
    j = 0 as i32 as u32;
    while j < (3 as i32 - 1 as i32) as u32 {
        ins_h = (ins_h << (15 as i32 + 3 as i32 - 1 as i32) / 3 as i32
            ^ *window.as_mut_ptr().offset(j as isize) as u32)
            & (((1 as i32) << 15 as i32) as u32).wrapping_sub(1 as i32 as u32);
        j = j.wrapping_add(1);
        j;
    }
}
unsafe extern "C" fn longest_match(mut cur_match: IPos) -> i32 {
    let mut chain_length: u32 = max_chain_length;
    let mut scan: *mut uch = window.as_mut_ptr().offset(strstart as isize);
    let mut match_0: *mut uch = 0 as *mut uch;
    let mut len: i32 = 0;
    let mut best_len: i32 = prev_length as i32;
    let mut limit: IPos = if strstart
        > (0x8000 as i32 - (258 as i32 + 3 as i32 + 1 as i32)) as IPos
    {
        strstart
            .wrapping_sub((0x8000 as i32 - (258 as i32 + 3 as i32 + 1 as i32)) as IPos)
    } else {
        0 as i32 as u32
    };
    let mut strend: *mut uch = window
        .as_mut_ptr()
        .offset(strstart as isize)
        .offset(258 as i32 as isize);
    let mut scan_end1: uch = *scan.offset((best_len - 1 as i32) as isize);
    let mut scan_end: uch = *scan.offset(best_len as isize);
    if prev_length >= good_match {
        chain_length >>= 2 as i32;
    }
    loop {
        match_0 = window.as_mut_ptr().offset(cur_match as isize);
        if !(*match_0.offset(best_len as isize) as i32 != scan_end as i32
            || *match_0.offset((best_len - 1 as i32) as isize) as i32 != scan_end1 as i32
            || *match_0 as i32 != *scan as i32
            || {
                match_0 = match_0.offset(1);
                *match_0 as i32 != *scan.offset(1 as i32 as isize) as i32
            })
        {
            scan = scan.offset(2 as i32 as isize);
            match_0 = match_0.offset(1);
            match_0;
            loop {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                if !(*scan as i32 == *match_0 as i32
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        *scan as i32 == *match_0 as i32
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        *scan as i32 == *match_0 as i32
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        *scan as i32 == *match_0 as i32
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        *scan as i32 == *match_0 as i32
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        *scan as i32 == *match_0 as i32
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        *scan as i32 == *match_0 as i32
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        *scan as i32 == *match_0 as i32
                    } && scan < strend)
                {
                    break;
                }
            }
            len = 258 as i32 - strend.offset_from(scan) as i64 as i32;
            scan = strend.offset(-(258 as i32 as isize));
            if len > best_len {
                match_start = cur_match;
                best_len = len;
                if len >= nice_match {
                    break;
                }
                scan_end1 = *scan.offset((best_len - 1 as i32) as isize);
                scan_end = *scan.offset(best_len as isize);
            }
        }
        cur_match = *prev
            .as_mut_ptr()
            .offset((cur_match & (0x8000 as i32 - 1 as i32) as u32) as isize) as IPos;
        if !(cur_match > limit
            && {
                chain_length = chain_length.wrapping_sub(1);
                chain_length != 0 as i32 as u32
            })
        {
            break;
        }
    }
    return best_len;
}
unsafe extern "C" fn fill_window() {
    let mut n: u32 = 0;
    let mut m: u32 = 0;
    let mut more: u32 = window_size
        .wrapping_sub(lookahead as ulg)
        .wrapping_sub(strstart as ulg) as u32;
    if more == -(1 as i32) as u32 {
        more = more.wrapping_sub(1);
        more;
    } else if strstart
        >= (0x8000 as i32 + (0x8000 as i32 - (258 as i32 + 3 as i32 + 1 as i32))) as u32
    {
        memcpy(
            window.as_mut_ptr() as *mut i8 as *mut libc::c_void,
            (window.as_mut_ptr() as *mut i8).offset(0x8000 as i32 as isize)
                as *const libc::c_void,
            0x8000 as i32 as u32 as u64,
        );
        match_start = match_start.wrapping_sub(0x8000 as i32 as u32);
        strstart = strstart.wrapping_sub(0x8000 as i32 as u32);
        if rsync_chunk_end != 0xffffffff as u64 {
            rsync_chunk_end = (rsync_chunk_end as u64).wrapping_sub(0x8000 as i32 as u64)
                as ulg as ulg;
        }
        block_start -= 0x8000 as i32 as i64;
        n = 0 as i32 as u32;
        while n < ((1 as i32) << 15 as i32) as u32 {
            m = *prev.as_mut_ptr().offset(0x8000 as i32 as isize).offset(n as isize)
                as u32;
            *prev.as_mut_ptr().offset(0x8000 as i32 as isize).offset(n as isize) = (if m
                >= 0x8000 as i32 as u32
            {
                m.wrapping_sub(0x8000 as i32 as u32)
            } else {
                0 as i32 as u32
            }) as Pos;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as i32 as u32;
        while n < 0x8000 as i32 as u32 {
            m = *prev.as_mut_ptr().offset(n as isize) as u32;
            *prev.as_mut_ptr().offset(n as isize) = (if m >= 0x8000 as i32 as u32 {
                m.wrapping_sub(0x8000 as i32 as u32)
            } else {
                0 as i32 as u32
            }) as Pos;
            n = n.wrapping_add(1);
            n;
        }
        more = more.wrapping_add(0x8000 as i32 as u32);
    }
    if eofile == 0 {
        n = read_buf
            .expect(
                "non-null function pointer",
            )(
            (window.as_mut_ptr() as *mut i8)
                .offset(strstart as isize)
                .offset(lookahead as isize),
            more,
        ) as u32;
        if n == 0 as i32 as u32 || n == -(1 as i32) as u32 {
            eofile = 1 as i32;
            memset(
                window.as_mut_ptr().offset(strstart as isize).offset(lookahead as isize)
                    as voidp,
                0 as i32,
                (3 as i32 - 1 as i32) as u64,
            );
        } else {
            lookahead = lookahead.wrapping_add(n);
        }
    }
}
unsafe extern "C" fn rsync_roll(mut start: u32, mut num: u32) {
    let mut i: u32 = 0;
    if start < 4096 as i32 as u32 {
        i = start;
        while i < 4096 as i32 as u32 {
            if i == start.wrapping_add(num) {
                return;
            }
            rsync_sum = (rsync_sum as u64)
                .wrapping_add(*window.as_mut_ptr().offset(i as isize) as ulg) as ulg
                as ulg;
            i = i.wrapping_add(1);
            i;
        }
        num = num.wrapping_sub((4096 as i32 as u32).wrapping_sub(start));
        start = 4096 as i32 as u32;
    }
    i = start;
    while i < start.wrapping_add(num) {
        rsync_sum = (rsync_sum as u64)
            .wrapping_add(*window.as_mut_ptr().offset(i as isize) as ulg) as ulg as ulg;
        rsync_sum = (rsync_sum as u64)
            .wrapping_sub(
                *window.as_mut_ptr().offset(i.wrapping_sub(4096 as i32 as u32) as isize)
                    as ulg,
            ) as ulg as ulg;
        if rsync_chunk_end == 0xffffffff as u64
            && rsync_sum.wrapping_rem(4096 as i32 as u64) == 0 as i32 as u64
        {
            rsync_chunk_end = i as ulg;
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn deflate_fast() -> off_t {
    let mut hash_head: IPos = 0;
    let mut flush: i32 = 0 as i32;
    let mut match_length: u32 = 0 as i32 as u32;
    prev_length = (3 as i32 - 1 as i32) as u32;
    while lookahead != 0 as i32 as u32 {
        ins_h = (ins_h << (15 as i32 + 3 as i32 - 1 as i32) / 3 as i32
            ^ *window
                .as_mut_ptr()
                .offset(
                    strstart.wrapping_add(3 as i32 as u32).wrapping_sub(1 as i32 as u32)
                        as isize,
                ) as u32)
            & (((1 as i32) << 15 as i32) as u32).wrapping_sub(1 as i32 as u32);
        hash_head = *prev
            .as_mut_ptr()
            .offset(0x8000 as i32 as isize)
            .offset(ins_h as isize) as IPos;
        *prev
            .as_mut_ptr()
            .offset((strstart & (0x8000 as i32 - 1 as i32) as u32) as isize) = hash_head
            as ush;
        *prev.as_mut_ptr().offset(0x8000 as i32 as isize).offset(ins_h as isize) = strstart
            as ush;
        if hash_head != 0 as i32 as u32
            && strstart.wrapping_sub(hash_head)
                <= (0x8000 as i32 - (258 as i32 + 3 as i32 + 1 as i32)) as u32
            && strstart as u64
                <= window_size.wrapping_sub((258 as i32 + 3 as i32 + 1 as i32) as u64)
        {
            match_length = longest_match(hash_head) as u32;
            if match_length > lookahead {
                match_length = lookahead;
            }
        }
        if match_length >= 3 as i32 as u32 {
            flush = ct_tally(
                strstart.wrapping_sub(match_start) as i32,
                match_length.wrapping_sub(3 as i32 as u32) as i32,
            );
            lookahead = lookahead.wrapping_sub(match_length);
            if rsync != 0 {
                rsync_roll(strstart, match_length);
            }
            if match_length <= max_lazy_match {
                match_length = match_length.wrapping_sub(1);
                match_length;
                loop {
                    strstart = strstart.wrapping_add(1);
                    strstart;
                    ins_h = (ins_h << (15 as i32 + 3 as i32 - 1 as i32) / 3 as i32
                        ^ *window
                            .as_mut_ptr()
                            .offset(
                                strstart
                                    .wrapping_add(3 as i32 as u32)
                                    .wrapping_sub(1 as i32 as u32) as isize,
                            ) as u32)
                        & (((1 as i32) << 15 as i32) as u32)
                            .wrapping_sub(1 as i32 as u32);
                    hash_head = *prev
                        .as_mut_ptr()
                        .offset(0x8000 as i32 as isize)
                        .offset(ins_h as isize) as IPos;
                    *prev
                        .as_mut_ptr()
                        .offset(
                            (strstart & (0x8000 as i32 - 1 as i32) as u32) as isize,
                        ) = hash_head as ush;
                    *prev
                        .as_mut_ptr()
                        .offset(0x8000 as i32 as isize)
                        .offset(ins_h as isize) = strstart as ush;
                    match_length = match_length.wrapping_sub(1);
                    if !(match_length != 0 as i32 as u32) {
                        break;
                    }
                }
                strstart = strstart.wrapping_add(1);
                strstart;
            } else {
                strstart = strstart.wrapping_add(match_length);
                match_length = 0 as i32 as u32;
                ins_h = *window.as_mut_ptr().offset(strstart as isize) as u32;
                ins_h = (ins_h << (15 as i32 + 3 as i32 - 1 as i32) / 3 as i32
                    ^ *window
                        .as_mut_ptr()
                        .offset(strstart.wrapping_add(1 as i32 as u32) as isize) as u32)
                    & (((1 as i32) << 15 as i32) as u32).wrapping_sub(1 as i32 as u32);
            }
        } else {
            flush = ct_tally(
                0 as i32,
                *window.as_mut_ptr().offset(strstart as isize) as i32,
            );
            if rsync != 0 {
                rsync_roll(strstart, 1 as i32 as u32);
            }
            lookahead = lookahead.wrapping_sub(1);
            lookahead;
            strstart = strstart.wrapping_add(1);
            strstart;
        }
        if rsync != 0 && strstart as u64 > rsync_chunk_end {
            rsync_chunk_end = 0xffffffff as u64;
            flush = 2 as i32;
        }
        if flush != 0 {
            flush_block(
                (if block_start >= 0 as i64 {
                    &mut *window.as_mut_ptr().offset(block_start as u32 as isize)
                        as *mut uch as *mut i8
                } else {
                    0 as *mut libc::c_void as *mut i8
                }),
                (strstart as i64 - block_start) as ulg,
                flush - 1 as i32,
                0 as i32,
            );
            block_start = strstart as i64;
        }
        while lookahead < (258 as i32 + 3 as i32 + 1 as i32) as u32 && eofile == 0 {
            fill_window();
        }
    }
    return flush_block(
        if block_start >= 0 as i64 {
            &mut *window.as_mut_ptr().offset(block_start as u32 as isize) as *mut uch
                as *mut i8
        } else {
            0 as *mut libc::c_void as *mut i8
        },
        (strstart as i64 - block_start) as ulg,
        flush - 1 as i32,
        1 as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn deflate(mut pack_level: i32) -> off_t {
    let mut hash_head: IPos = 0;
    let mut prev_match: IPos = 0;
    let mut flush: i32 = 0 as i32;
    let mut match_available: i32 = 0 as i32;
    let mut match_length: u32 = (3 as i32 - 1 as i32) as u32;
    lm_init(pack_level);
    if pack_level <= 3 as i32 {
        return deflate_fast();
    }
    while lookahead != 0 as i32 as u32 {
        ins_h = (ins_h << (15 as i32 + 3 as i32 - 1 as i32) / 3 as i32
            ^ *window
                .as_mut_ptr()
                .offset(
                    strstart.wrapping_add(3 as i32 as u32).wrapping_sub(1 as i32 as u32)
                        as isize,
                ) as u32)
            & (((1 as i32) << 15 as i32) as u32).wrapping_sub(1 as i32 as u32);
        hash_head = *prev
            .as_mut_ptr()
            .offset(0x8000 as i32 as isize)
            .offset(ins_h as isize) as IPos;
        *prev
            .as_mut_ptr()
            .offset((strstart & (0x8000 as i32 - 1 as i32) as u32) as isize) = hash_head
            as ush;
        *prev.as_mut_ptr().offset(0x8000 as i32 as isize).offset(ins_h as isize) = strstart
            as ush;
        prev_length = match_length;
        prev_match = match_start;
        match_length = (3 as i32 - 1 as i32) as u32;
        if hash_head != 0 as i32 as u32 && prev_length < max_lazy_match
            && strstart.wrapping_sub(hash_head)
                <= (0x8000 as i32 - (258 as i32 + 3 as i32 + 1 as i32)) as u32
            && strstart as u64
                <= window_size.wrapping_sub((258 as i32 + 3 as i32 + 1 as i32) as u64)
        {
            match_length = longest_match(hash_head) as u32;
            if match_length > lookahead {
                match_length = lookahead;
            }
            if match_length == 3 as i32 as u32
                && strstart.wrapping_sub(match_start) > 4096 as i32 as u32
            {
                match_length = match_length.wrapping_sub(1);
                match_length;
            }
        }
        if prev_length >= 3 as i32 as u32 && match_length <= prev_length {
            flush = ct_tally(
                strstart.wrapping_sub(1 as i32 as u32).wrapping_sub(prev_match) as i32,
                prev_length.wrapping_sub(3 as i32 as u32) as i32,
            );
            lookahead = lookahead
                .wrapping_sub(prev_length.wrapping_sub(1 as i32 as u32));
            prev_length = prev_length.wrapping_sub(2 as i32 as u32);
            if rsync != 0 {
                rsync_roll(strstart, prev_length.wrapping_add(1 as i32 as u32));
            }
            loop {
                strstart = strstart.wrapping_add(1);
                strstart;
                ins_h = (ins_h << (15 as i32 + 3 as i32 - 1 as i32) / 3 as i32
                    ^ *window
                        .as_mut_ptr()
                        .offset(
                            strstart
                                .wrapping_add(3 as i32 as u32)
                                .wrapping_sub(1 as i32 as u32) as isize,
                        ) as u32)
                    & (((1 as i32) << 15 as i32) as u32).wrapping_sub(1 as i32 as u32);
                hash_head = *prev
                    .as_mut_ptr()
                    .offset(0x8000 as i32 as isize)
                    .offset(ins_h as isize) as IPos;
                *prev
                    .as_mut_ptr()
                    .offset((strstart & (0x8000 as i32 - 1 as i32) as u32) as isize) = hash_head
                    as ush;
                *prev
                    .as_mut_ptr()
                    .offset(0x8000 as i32 as isize)
                    .offset(ins_h as isize) = strstart as ush;
                prev_length = prev_length.wrapping_sub(1);
                if !(prev_length != 0 as i32 as u32) {
                    break;
                }
            }
            match_available = 0 as i32;
            match_length = (3 as i32 - 1 as i32) as u32;
            strstart = strstart.wrapping_add(1);
            strstart;
            if rsync != 0 && strstart as u64 > rsync_chunk_end {
                rsync_chunk_end = 0xffffffff as u64;
                flush = 2 as i32;
            }
            if flush != 0 {
                flush_block(
                    (if block_start >= 0 as i64 {
                        &mut *window.as_mut_ptr().offset(block_start as u32 as isize)
                            as *mut uch as *mut i8
                    } else {
                        0 as *mut libc::c_void as *mut i8
                    }),
                    (strstart as i64 - block_start) as ulg,
                    flush - 1 as i32,
                    0 as i32,
                );
                block_start = strstart as i64;
            }
        } else if match_available != 0 {
            flush = ct_tally(
                0 as i32,
                *window
                    .as_mut_ptr()
                    .offset(strstart.wrapping_sub(1 as i32 as u32) as isize) as i32,
            );
            if rsync != 0 && strstart as u64 > rsync_chunk_end {
                rsync_chunk_end = 0xffffffff as u64;
                flush = 2 as i32;
            }
            if flush != 0 {
                flush_block(
                    (if block_start >= 0 as i64 {
                        &mut *window.as_mut_ptr().offset(block_start as u32 as isize)
                            as *mut uch as *mut i8
                    } else {
                        0 as *mut libc::c_void as *mut i8
                    }),
                    (strstart as i64 - block_start) as ulg,
                    flush - 1 as i32,
                    0 as i32,
                );
                block_start = strstart as i64;
            }
            if rsync != 0 {
                rsync_roll(strstart, 1 as i32 as u32);
            }
            strstart = strstart.wrapping_add(1);
            strstart;
            lookahead = lookahead.wrapping_sub(1);
            lookahead;
        } else {
            if rsync != 0 && strstart as u64 > rsync_chunk_end {
                rsync_chunk_end = 0xffffffff as u64;
                flush = 2 as i32;
                flush_block(
                    (if block_start >= 0 as i64 {
                        &mut *window.as_mut_ptr().offset(block_start as u32 as isize)
                            as *mut uch as *mut i8
                    } else {
                        0 as *mut libc::c_void as *mut i8
                    }),
                    (strstart as i64 - block_start) as ulg,
                    flush - 1 as i32,
                    0 as i32,
                );
                block_start = strstart as i64;
            }
            match_available = 1 as i32;
            if rsync != 0 {
                rsync_roll(strstart, 1 as i32 as u32);
            }
            strstart = strstart.wrapping_add(1);
            strstart;
            lookahead = lookahead.wrapping_sub(1);
            lookahead;
        }
        while lookahead < (258 as i32 + 3 as i32 + 1 as i32) as u32 && eofile == 0 {
            fill_window();
        }
    }
    if match_available != 0 {
        ct_tally(
            0 as i32,
            *window.as_mut_ptr().offset(strstart.wrapping_sub(1 as i32 as u32) as isize)
                as i32,
        );
    }
    return flush_block(
        if block_start >= 0 as i64 {
            &mut *window.as_mut_ptr().offset(block_start as u32 as isize) as *mut uch
                as *mut i8
        } else {
            0 as *mut libc::c_void as *mut i8
        },
        (strstart as i64 - block_start) as ulg,
        flush - 1 as i32,
        1 as i32,
    );
}
unsafe extern "C" fn run_static_initializers() {
    window_size = (2 as i32 as ulg).wrapping_mul(0x8000 as i32 as u64);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];