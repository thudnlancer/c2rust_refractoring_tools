use std::cmp;
use std::convert::TryFrom;
use std::mem;
use std::ptr;
use std::slice;
use std::str;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until, take_while},
    character::complete::{char, digit1, space0, space1},
    combinator::{map, map_res, opt, recognize},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

const MEMCACHE_MAX_KEY_LENGTH: usize = 250;
const CR: u8 = b'\r';
const LF: u8 = b'\n';
const CRLF: &[u8] = b"\r\n";
const CRLF_LEN: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MsgType {
    Unknown,
    ReqMcGet,
    ReqMcGets,
    ReqMcSet,
    ReqMcAdd,
    ReqMcReplace,
    ReqMcAppend,
    ReqMcPrepend,
    ReqMcCas,
    ReqMcIncr,
    ReqMcDecr,
    ReqMcDelete,
    ReqMcTouch,
    ReqMcQuit,
    ReqMcVersion,
    RspMcValue,
    RspMcEnd,
    RspMcStored,
    RspMcNotStored,
    RspMcExists,
    RspMcNotFound,
    RspMcDeleted,
    RspMcTouched,
    RspMcError,
    RspMcClientError,
    RspMcServerError,
    RspMcVersion,
    Sentinel,
}

#[derive(Debug)]
pub struct KeyPos {
    start: usize,
    end: usize,
}

#[derive(Debug)]
pub struct Msg {
    pub request: bool,
    pub redis: bool,
    pub id: u64,
    pub pos: usize,
    pub state: usize,
    pub result: usize,
    pub type_: MsgType,
    pub token: Option<usize>,
    pub keys: Vec<KeyPos>,
    pub narg: usize,
    pub vlen: u32,
    pub noreply: bool,
    pub quit: bool,
    pub end: Option<usize>,
    pub frag_id: u32,
    pub nfrag: usize,
    pub frag_owner: Option<Box<Msg>>,
    pub frag_seq: Vec<Option<Box<Msg>>>,
    pub error: bool,
    pub err: i32,
    pub peer: Option<Box<Msg>>,
    pub mlen: usize,
    pub mhdr: Vec<Bytes>,
}

impl Msg {
    pub fn new(request: bool, redis: bool, id: u64) -> Self {
        Msg {
            request,
            redis,
            id,
            pos: 0,
            state: 0,
            result: 0,
            type_: MsgType::Unknown,
            token: None,
            keys: Vec::new(),
            narg: 0,
            vlen: 0,
            noreply: false,
            quit: false,
            end: None,
            frag_id: 0,
            nfrag: 0,
            frag_owner: None,
            frag_seq: Vec::new(),
            error: false,
            err: 0,
            peer: None,
            mlen: 0,
            mhdr: Vec::new(),
        }
    }

    pub fn ensure_mbuf(&mut self, len: usize) -> Option<BytesMut> {
        let mut buf = BytesMut::with_capacity(len);
        buf.put_slice(&vec![0; len]);
        Some(buf.freeze())
    }

    pub fn set_placeholder_key(&mut self) -> bool {
        let key = KeyPos {
            start: 0,
            end: 0,
        };
        self.keys.push(key);
        true
    }

    pub fn append(&mut self, data: &[u8]) -> bool {
        let mut buf = BytesMut::with_capacity(data.len());
        buf.put_slice(data);
        self.mhdr.push(buf.freeze());
        self.mlen += data.len();
        true
    }

    pub fn prepend(&mut self, data: &[u8]) -> bool {
        let mut buf = BytesMut::with_capacity(data.len());
        buf.put_slice(data);
        self.mhdr.insert(0, buf.freeze());
        self.mlen += data.len();
        true
    }

    pub fn gen_frag_id(&mut self) -> u32 {
        0
    }

    pub fn backend_idx(&self, _start: usize, _len: usize) -> usize {
        0
    }
}

pub fn memcache_storage(r: &Msg) -> bool {
    matches!(
        r.type_,
        MsgType::ReqMcSet
            | MsgType::ReqMcCas
            | MsgType::ReqMcAdd
            | MsgType::ReqMcReplace
            | MsgType::ReqMcAppend
            | MsgType::ReqMcPrepend
    )
}

pub fn memcache_cas(r: &Msg) -> bool {
    r.type_ == MsgType::ReqMcCas
}

pub fn memcache_retrieval(r: &Msg) -> bool {
    matches!(r.type_, MsgType::ReqMcGet | MsgType::ReqMcGets)
}

pub fn memcache_should_fragment(r: &Msg) -> bool {
    match r.type_ {
        MsgType::ReqMcGet | MsgType::ReqMcGets => r.keys.len() != 1,
        _ => false,
    }
}

pub fn memcache_arithmetic(r: &Msg) -> bool {
    matches!(r.type_, MsgType::ReqMcIncr | MsgType::ReqMcDecr)
}

pub fn memcache_delete(r: &Msg) -> bool {
    r.type_ == MsgType::ReqMcDelete
}

pub fn memcache_touch(r: &Msg) -> bool {
    r.type_ == MsgType::ReqMcTouch
}

pub fn memcache_parse_req(r: &mut Msg) {
    let mut state = r.state;
    let b = r.mhdr.last().unwrap();
    let mut p = r.pos;

    assert!(r.request);
    assert!(!r.redis);
    assert!(state >= 0 && state < 22);
    assert!(p <= b.len());

    for ch in b[p..].iter() {
        let ch = *ch;

        match state {
            0 => {
                if ch == b' ' {
                    continue;
                }
                if !ch.is_ascii_lowercase() {
                    r.result = 1;
                    r.state = state;
                    return;
                }
                r.token = Some(p);
                state = 1;
            }
            1 => {
                if ch == b' ' || ch == CR {
                    let m = r.token.unwrap();
                    r.token = None;
                    r.type_ = MsgType::Unknown;
                    r.narg += 1;

                    match p - m {
                        3 => {
                            if b[m..].starts_with(b"get ") {
                                r.type_ = MsgType::ReqMcGet;
                            } else if b[m..].starts_with(b"set ") {
                                r.type_ = MsgType::ReqMcSet;
                            } else if b[m..].starts_with(b"add ") {
                                r.type_ = MsgType::ReqMcAdd;
                            } else if b[m..].starts_with(b"cas ") {
                                r.type_ = MsgType::ReqMcCas;
                            }
                        }
                        4 => {
                            if b[m..].starts_with(b"gets") {
                                r.type_ = MsgType::ReqMcGets;
                            } else if b[m..].starts_with(b"incr") {
                                r.type_ = MsgType::ReqMcIncr;
                            } else if b[m..].starts_with(b"decr") {
                                r.type_ = MsgType::ReqMcDecr;
                            } else if b[m..].starts_with(b"quit") {
                                r.type_ = MsgType::ReqMcQuit;
                                r.quit = true;
                            }
                        }
                        5 => {
                            if b[m..].starts_with(b"touch") {
                                r.type_ = MsgType::ReqMcTouch;
                            }
                        }
                        6 => {
                            if b[m..].starts_with(b"append") {
                                r.type_ = MsgType::ReqMcAppend;
                            } else if b[m..].starts_with(b"delete") {
                                r.type_ = MsgType::ReqMcDelete;
                            }
                        }
                        7 => {
                            if b[m..].starts_with(b"prepend") {
                                r.type_ = MsgType::ReqMcPrepend;
                            } else if b[m..].starts_with(b"replace") {
                                r.type_ = MsgType::ReqMcReplace;
                            } else if b[m..].starts_with(b"version") {
                                r.type_ = MsgType::ReqMcVersion;
                                if !r.set_placeholder_key() {
                                    r.result = 1;
                                    r.state = state;
                                    return;
                                }
                            }
                        }
                        _ => {}
                    }

                    match r.type_ {
                        MsgType::ReqMcGet
                        | MsgType::ReqMcGets
                        | MsgType::ReqMcDelete
                        | MsgType::ReqMcCas
                        | MsgType::ReqMcSet
                        | MsgType::ReqMcAdd
                        | MsgType::ReqMcReplace
                        | MsgType::ReqMcAppend
                        | MsgType::ReqMcPrepend
                        | MsgType::ReqMcIncr
                        | MsgType::ReqMcDecr
                        | MsgType::ReqMcTouch => {
                            if ch == CR {
                                r.result = 1;
                                r.state = state;
                                return;
                            }
                            state = 2;
                        }
                        MsgType::ReqMcVersion | MsgType::ReqMcQuit => {
                            p -= 1;
                            state = 13;
                        }
                        MsgType::Unknown => {
                            r.result = 1;
                            r.state = state;
                            return;
                        }
                        _ => unreachable!(),
                    }
                } else if !ch.is_ascii_lowercase() {
                    r.result = 1;
                    r.state = state;
                    return;
                }
            }
            2 => {
                if ch != b' ' {
                    p -= 1;
                    r.token = None;
                    state = 3;
                }
            }
            3 => {
                if r.token.is_none() {
                    r.token = Some(p);
                }
                if ch == b' ' || ch == CR {
                    let keylen = p - r.token.unwrap();
                    if keylen > MEMCACHE_MAX_KEY_LENGTH {
                        r.result = 1;
                        r.state = state;
                        return;
                    } else if keylen == 0 {
                        r.result = 1;
                        r.state = state;
                        return;
                    }

                    let kpos = KeyPos {
                        start: r.token.unwrap(),
                        end: p,
                    };
                    r.keys.push(kpos);
                    r.narg += 1;
                    r.token = None;

                    if memcache_storage(r) {
                        state = 5;
                    } else if memcache_arithmetic(r) || memcache_touch(r) {
                        state = 15;
                    } else if memcache_retrieval(r) {
                        state = 4;
                    } else {
                        state = 12;
                    }

                    if ch == CR {
                        if memcache_storage(r) || memcache_arithmetic(r) {
                            r.result = 1;
                            r.state = state;
                            return;
                        }
                        p -= 1;
                    }
                }
            }
            4 => {
                assert!(memcache_retrieval(r));
                match ch {
                    b' ' => {}
                    CR => state = 20,
                    _ => {
                        r.token = None;
                        p -= 1;
                        state = 3;
                    }
                }
            }
            5 => {
                if ch != b' ' {
                    if !ch.is_ascii_digit() {
                        r.result = 1;
                        r.state = state;
                        return;
                    }
                    r.token = Some(p);
                    state = 6;
                }
            }
            6 => {
                if ch.is_ascii_digit() {
                } else if ch == b' ' {
                    r.token = None;
                    state = 7;
                } else {
                    r.result = 1;
                    r.state = state;
                    return;
                }
            }
            7 => {
                if ch != b' ' {
                    if !ch.is_ascii_digit() {
                        r.result = 1;
                        r.state = state;
                        return;
                    }
                    r.token = Some(p);
                    state = 8;
                }
            }
            8 => {
                if ch.is_ascii_digit() {
                } else if ch == b' ' {
                    r.token = None;
                    state = 9;
                } else {
                    r.result = 1;
                    r.state = state;
                    return;
                }
            }
            9 => {
                if ch != b' ' {
                    if !ch.is_ascii_digit() {
                        r.result = 1;
                        r.state = state;
                        return;
                    }
                    r.vlen = (ch - b'0') as u32;
                    state = 10;
                }
            }
            10 => {
                if ch.is_ascii_digit() {
                    r.vlen = r.vlen * 10 + (ch - b'0') as u32;
                } else if memcache_cas(r) {
                    if ch != b' ' {
                        r.result = 1;
                        r.state = state;
                        return;
                    }
                    p -= 1;
                    r.token = None;
                    state = 11;
                } else if ch == b' ' || ch == CR {
                    p -= 1;
                    r.token = None;
                    state = 12;
                } else {
                    r.result = 1;
                    r.state = state;
                    return;
                }
            }
            11 => {
                if ch != b' ' {
                    if !ch.is_ascii_digit() {
                        r.result = 1;
                        r.state = state;
                        return;
                    }
                    r.token = Some(p);
                    state = 12;
                }
            }
            12 => {
                if ch.is_ascii_digit() {
                } else if ch == b' ' || ch == CR {
                    p -= 1;
                    r.token = None;
                    state = 12;
                } else {
                    r.result = 1;
                    r.state = state;
                    return;
                }
            }
            13 => {
                match ch {
                    b' ' => {}
                    CR => state = 20,
                    _ => {
                        r.result = 1;
                        r.state = state;
                        return;
                    }
                }
            }
            14 => {
                match ch {
                    LF => state = 20,
                    _ => {
                        r.result = 1;
                        r.state = state;
                        return;
                    }
                }
            }
            15 => {
                if ch != b' ' {
                    if !(ch.is_ascii_digit() || ch == b'-') {
                        r.result = 1;
                        r.state = state;
                        return;
                    }
                    r.token = Some(p);
                    state = 16;
                }
            }
            16 => {
                if ch.is_ascii_digit() {
                } else if ch == b' ' || ch == CR {
                    r.token = None;
                    p -= 1;
                    state = 12;
                } else {
                    r.result = 1;
                    r.state = state;
                    return;
                }
            }
            17 => {
                match ch {
                    b' ' => {}
                    b'n' => {
                        if memcache_storage(r)
                            || memcache_arithmetic(r)
                            || memcache_delete(r)
                            || memcache_touch(r)
                        {
                            r.token = Some(p);
                            state = 18;
                        } else {
                            r.result = 1;
                            r.state = state;
                            return;
                        }
                    }
                    CR => {
                        if memcache_storage(r) {
                            state = 14;
                        } else {
                            state = 20;
                        }
                    }
                    _ => {
                        r.result = 1;
                        r.state = state;
                        return;
                    }
                }
            }
            18 => {
                match ch {
                    b' ' | CR => {
                        let m = r.token.unwrap();
                        if (p - m) == 7 && b[m..].starts_with(b"noreply") {
                            assert!(
                                memcache_storage(r)
                                    || memcache_arithmetic(r)
                                    || memcache_delete(r)
                                    || memcache_touch(r)
                            );
                            r.token = None;
                            r.noreply = true;
                            state = 19;
                            p -= 1;
                        } else {
                            r.result = 1;
                            r.state = state;
                            return;
                        }
                    }
                    _ => {}
                }
            }
            19 => {
                match ch {
                    b' ' => {}
                    CR => {
                        if memcache_storage(r) {
                            state = 14;
                        } else {
                            state = 20;
                        }
                    }
                    _ => {
                        r.result = 1;
                        r.state = state;
                        return;
                    }
                }
            }
            20 => {
                match ch {
                    LF => {
                        r.pos = p + 1;
                        r.state = 0;
                        r.result = 0;
                        return;
                    }
                    _ => {
                        r.result = 1;
                        r.state = state;
                        return;
                    }
                }
            }
            21 | _ => unreachable!(),
        }
        p += 1;
    }

    r.pos = p;
    r.state = state;

    if b.len() == p && r.token.is_some() {
        r.pos = r.token.unwrap();
        r.token = None;
        r.result = 2;
    } else {
        r.result = 3;
    }
}

pub fn memcache_parse_rsp(r: &mut Msg) {
    let mut state = r.state;
    let b = r.mhdr.last().unwrap();
    let mut p = r.pos;

    assert!(!r.request);
    assert!(!r.redis);
    assert!(state >= 0 && state < 16);
    assert!(p <= b.len());

    for ch in b[p..].iter