use std::mem;
use std::ptr;
use std::slice;
use std::io::{self, Write};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum InflateMode {
    Head,
    Flags,
    Time,
    Os,
    Exlen,
    Extra,
    Name,
    Comment,
    Hcrc,
    Dictid,
    Dict,
    Type,
    Typedo,
    Stored,
    Copy_,
    Copy,
    Table,
    Lenlens,
    Codelens,
    Len_,
    Len,
    Lenext,
    Dist,
    Distext,
    Match,
    Lit,
    Check,
    Length,
    Done,
    Bad,
    Mem,
    Sync,
}

#[derive(Debug)]
struct InflateState {
    mode: InflateMode,
    last: bool,
    wrap: i32,
    havedict: bool,
    flags: i32,
    dmax: u32,
    check: u32,
    total: u32,
    head: Option<Box<GzHeader>>,
    wbits: u32,
    wsize: u32,
    whave: u32,
    wnext: u32,
    window: Vec<u8>,
    hold: u32,
    bits: u32,
    length: u32,
    offset: u32,
    extra: u32,
    lencode: Vec<Code>,
    distcode: Vec<Code>,
    lenbits: u32,
    distbits: u32,
    ncode: u32,
    nlen: u32,
    ndist: u32,
    have: u32,
    next: usize,
    lens: [u16; 320],
    work: [u16; 288],
    codes: Vec<Code>,
    sane: bool,
    back: i32,
    was: u32,
}

#[derive(Debug, Clone, Copy)]
struct Code {
    op: u8,
    bits: u8,
    val: u16,
}

#[derive(Debug)]
struct GzHeader {
    text: i32,
    time: u32,
    xflags: i32,
    os: i32,
    extra: Vec<u8>,
    extra_len: u32,
    extra_max: u32,
    name: Vec<u8>,
    name_max: u32,
    comment: Vec<u8>,
    comm_max: u32,
    hcrc: i32,
    done: i32,
}

struct ZStream {
    next_in: Vec<u8>,
    avail_in: u32,
    total_in: u32,
    next_out: Vec<u8>,
    avail_out: u32,
    total_out: u32,
    msg: String,
    state: InflateState,
    zalloc: fn(u32) -> Vec<u8>,
    zfree: fn(Vec<u8>),
    opaque: (),
    data_type: u32,
    adler: u32,
    reserved: u32,
}

const Z_OK: i32 = 0;
const Z_STREAM_END: i32 = 1;
const Z_NEED_DICT: i32 = 2;
const Z_ERRNO: i32 = -1;
const Z_STREAM_ERROR: i32 = -2;
const Z_DATA_ERROR: i32 = -3;
const Z_MEM_ERROR: i32 = -4;
const Z_BUF_ERROR: i32 = -5;
const Z_VERSION_ERROR: i32 = -6;

const Z_NO_FLUSH: i32 = 0;
const Z_PARTIAL_FLUSH: i32 = 1;
const Z_SYNC_FLUSH: i32 = 2;
const Z_FULL_FLUSH: i32 = 3;
const Z_FINISH: i32 = 4;
const Z_BLOCK: i32 = 5;
const Z_TREES: i32 = 6;

const Z_DEFLATED: i32 = 8;

fn inflate_reset(strm: &mut ZStream) -> i32 {
    if strm.state.is_none() {
        return Z_STREAM_ERROR;
    }
    let state = strm.state.as_mut().unwrap();
    strm.total_in = 0;
    strm.total_out = 0;
    strm.msg.clear();
    strm.adler = 1;
    state.mode = InflateMode::Head;
    state.last = false;
    state.havedict = false;
    state.dmax = 32768;
    state.head = None;
    state.wsize = 0;
    state.whave = 0;
    state.wnext = 0;
    state.hold = 0;
    state.bits = 0;
    state.lencode = state.codes.clone();
    state.distcode = state.codes.clone();
    state.next = 0;
    state.sane = true;
    state.back = -1;
    Z_OK
}

fn inflate_reset2(strm: &mut ZStream, window_bits: i32) -> i32 {
    if strm.state.is_none() {
        return Z_STREAM_ERROR;
    }
    let state = strm.state.as_mut().unwrap();

    let wrap = if window_bits < 0 {
        0
    } else {
        (window_bits >> 4) + 1
    };
    let window_bits = if window_bits < 0 {
        -window_bits
    } else {
        window_bits
    };

    if window_bits != 0 && (window_bits < 8 || window_bits > 15) {
        return Z_STREAM_ERROR;
    }
    if !state.window.is_empty() && state.wbits != window_bits as u32 {
        state.window.clear();
    }

    state.wrap = wrap;
    state.wbits = window_bits as u32;
    inflate_reset(strm)
}

fn inflate_init2(strm: &mut ZStream, window_bits: i32, version: &str, stream_size: usize) -> i32 {
    if version != "1.2.11" || stream_size != mem::size_of::<ZStream>() {
        return Z_VERSION_ERROR;
    }
    if strm.state.is_none() {
        strm.msg.clear();
        if strm.zalloc.is_none() {
            strm.zalloc = zcalloc;
        }
        if strm.zfree.is_none() {
            strm.zfree = zcfree;
        }
        strm.state = Some(InflateState::new());
    }
    let ret = inflate_reset2(strm, window_bits);
    if ret != Z_OK {
        strm.state = None;
    }
    ret
}

fn inflate_init(strm: &mut ZStream, version: &str, stream_size: usize) -> i32 {
    inflate_init2(strm, 15, version, stream_size)
}

fn inflate_prime(strm: &mut ZStream, bits: i32, value: i32) -> i32 {
    if strm.state.is_none() {
        return Z_STREAM_ERROR;
    }
    let state = strm.state.as_mut().unwrap();
    if bits < 0 {
        state.hold = 0;
        state.bits = 0;
        return Z_OK;
    }
    if bits > 16 || state.bits + bits as u32 > 32 {
        return Z_STREAM_ERROR;
    }
    state.hold += (value as u32 & ((1 << bits) - 1)) << state.bits;
    state.bits += bits as u32;
    Z_OK
}

fn inflate(strm: &mut ZStream, flush: i32) -> i32 {
    if strm.state.is_none() || strm.next_out.is_empty() || 
       (strm.next_in.is_empty() && strm.avail_in != 0) {
        return Z_STREAM_ERROR;
    }
    let state = strm.state.as_mut().unwrap();
    if state.mode == InflateMode::Type {
        state.mode = InflateMode::Typedo;
    }

    let mut ret = Z_OK;
    loop {
        match state.mode {
            InflateMode::Head => {
                if state.wrap == 0 {
                    state.mode = InflateMode::Typedo;
                    continue;
                }
                if strm.avail_in < 2 {
                    return ret;
                }
                if state.wrap == 2 && state.hold == 0x8b1f {
                    state.check = crc32(0, &[]);
                    state.mode = InflateMode::Flags;
                    continue;
                }
                if state.wrap != 1 || ((state.hold >> 8) + (state.hold & 0xff)) % 31 != 0 {
                    strm.msg = "incorrect header check".to_string();
                    state.mode = InflateMode::Bad;
                    continue;
                }
                if (state.hold >> 4) & 0xf != Z_DEFLATED as u32 {
                    strm.msg = "unknown compression method".to_string();
                    state.mode = InflateMode::Bad;
                    continue;
                }
                state.mode = if state.hold & 0x200 != 0 {
                    InflateMode::Dictid
                } else {
                    InflateMode::Type
                };
            },
            _ => break,
        }
    }
    ret
}

fn inflate_end(strm: &mut ZStream) -> i32 {
    if strm.state.is_none() {
        return Z_STREAM_ERROR;
    }
    strm.state = None;
    Z_OK
}

fn inflate_set_dictionary(strm: &mut ZStream, dictionary: &[u8], dict_length: u32) -> i32 {
    if strm.state.is_none() {
        return Z_STREAM_ERROR;
    }
    let state = strm.state.as_mut().unwrap();
    if state.wrap != 0 && state.mode != InflateMode::Dict {
        return Z_STREAM_ERROR;
    }

    if state.mode == InflateMode::Dict {
        let id = adler32(0, &[]);
        let id = adler32(id, dictionary);
        if id != state.check {
            return Z_DATA_ERROR;
        }
    }

    if update_window(strm, strm.avail_out) != 0 {
        state.mode = InflateMode::Mem;
        return Z_MEM_ERROR;
    }

    if dict_length > state.wsize {
        state.window.copy_from_slice(&dictionary[dictionary.len() - state.wsize as usize..]);
        state.whave = state.wsize;
    } else {
        let start = state.wsize - dict_length;
        state.window[start as usize..].copy_from_slice(dictionary);
        state.whave = dict_length;
    }
    state.havedict = true;
    Z_OK
}

fn inflate_get_header(strm: &mut ZStream, head: Option<Box<GzHeader>>) -> i32 {
    if strm.state.is_none() {
        return Z_STREAM_ERROR;
    }
    let state = strm.state.as_mut().unwrap();
    if (state.wrap & 2) == 0 {
        return Z_STREAM_ERROR;
    }
    state.head = head;
    if let Some(h) = &mut state.head {
        h.done = 0;
    }
    Z_OK
}

fn inflate_sync(strm: &mut ZStream) -> i32 {
    if strm.state.is_none() {
        return Z_STREAM_ERROR;
    }
    let state = strm.state.as_mut().unwrap();
    if strm.avail_in == 0 && state.bits < 8 {
        return Z_BUF_ERROR;
    }

    if state.mode != InflateMode::Sync {
        state.mode = InflateMode::Sync;
        state.hold <<= state.bits & 7;
        state.bits -= state.bits & 7;
        let mut buf = Vec::new();
        while state.bits >= 8 {
            buf.push((state.hold & 0xff) as u8);
            state.hold >>= 8;
            state.bits -= 8;
        }
        state.have = 0;
        sync_search(&mut state.have, &buf);
    }

    let len = sync_search(&mut state.have, &strm.next_in);
    strm.avail_in -= len;
    strm.next_in = strm.next_in[len..].to_vec();
    strm.total_in += len as u32;

    if state.have != 4 {
        return Z_DATA_ERROR;
    }
    let total_in = strm.total_in;
    let total_out = strm.total_out;
    inflate_reset(strm);
    strm.total_in = total_in;
    strm.total_out = total_out;
    state.mode = InflateMode::Type;
    Z_OK
}

fn sync_search(have: &mut u32, buf: &[u8]) -> usize {
    let mut got = *have;
    let mut next = 0;
    while next < buf.len() && got < 4 {
        if buf[next] == if got < 2 { 0 } else { 0xff } {
            got += 1;
        } else if buf[next] != 0 {
            got = 0;
        } else {
            got = 4 - got;
        }
        next += 1;
    }
    *have = got;
    next
}

fn inflate_sync_point(strm: &ZStream) -> i32 {
    if strm.state.is_none() {
        return Z_STREAM_ERROR;
    }
    let state = strm.state.as_ref().unwrap();
    (state.mode == InflateMode::Stored && state.bits == 0) as i32
}

fn inflate_copy(dest: &mut ZStream, source: &ZStream) -> i32 {
    if source.state.is_none() {
        return Z_STREAM_ERROR;
    }
    let state = source.state.as_ref().unwrap();

    dest.state = Some(state.clone());
    if !state.window.is_empty() {
        dest.state.as_mut().unwrap().window = state.window.clone();
    }
    Z_OK
}

fn inflate_undermine(strm: &mut ZStream, subvert: i32) -> i32 {
    if strm.state.is_none() {
        return Z_STREAM_ERROR;
    }
    let state = strm.state.as_mut().unwrap();
    state.sane = subvert == 0;
    Z_OK
}

fn inflate_mark(strm: &ZStream) -> i64 {
    if strm.state.is_none() {
        return -1 << 16;
    }
    let state = strm.state.as_ref().unwrap();
    ((state.back as i64) << 16) + match state.mode {
        InflateMode::Copy => state.length as i64,
        InflateMode::Match => (state.was - state.length) as i64,
        _ => 0,
    }
}

fn zcalloc(size: u32) -> Vec<u8> {
    vec![0; size as usize]
}

fn zcfree(buf: Vec<u8>) {
    // Rust will automatically drop the vector
}

fn adler32(adler: u32, buf: &[u8]) -> u32 {
    const BASE: u32 = 65521;
    let mut a = adler & 0xffff;
    let mut b = (adler >> 16) & 0xffff;
    for &byte in buf {
        a = (a + byte as u32) % BASE;
        b = (b + a) % BASE;
    }
    (b << 16) | a
}

fn crc32(crc: u32, buf: &[u8]) -> u32 {
    let mut crc = !crc;
    for &byte in buf {
        crc ^= byte as u32;
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                0xedb88320 ^ (crc >> 1)
            } else {
                crc >> 1
            };
        }
    }
    !crc
}

fn update_window(strm: &mut ZStream, out: u32) -> i32 {
    if strm.state.is_none() {
        return 1;
    }
    let state = strm.state.as_mut().unwrap();

    if state.window.is_empty() {
        state.window = vec![0; 1 << state.wbits];
        if state.window.is_empty() {
            return 1;
        }
    }

    if state.wsize == 0 {
        state.wsize = 1 << state.wbits;
        state.wnext = 0;
        state.whave = 0;
    }

    let copy = out - strm.avail_out;
    if copy >= state.wsize {
        state.window.copy_from_slice(&strm.next_out[strm.next_out.len() - state.wsize as usize..]);
        state.wnext = 0;
        state.whave = state.wsize;
    } else {
        let dist = state.wsize - state.wnext;
        let copy_dist = if dist > copy { copy } else { dist };
        let start = state.wnext as usize;
        state.window[start..start + copy_dist as usize]
            .copy_from_slice(&strm.next_out[strm.next_out.len() - copy as usize..]);
        if copy > dist {
            state.window[0..(copy - dist) as usize]
                .copy_from_slice(&strm.next_out[strm.next_out.len() - (copy - dist) as usize..]);
            state.wnext = copy - dist;
            state.whave = state.wsize;
        } else {
            state.wnext += copy;
            if state.wnext == state.wsize {
                state.wnext = 0;
            }
            if state.whave < state.wsize {
                state.whave += copy;
            }
        }
    }
    0
}