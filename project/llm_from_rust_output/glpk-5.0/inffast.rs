use std::ptr;

#[derive(Debug, Clone, Copy)]
pub enum InflateMode {
    Head = 0,
    Flags = 1,
    Time = 2,
    Os = 3,
    Exlen = 4,
    Extra = 5,
    Name = 6,
    Comment = 7,
    Hcrc = 8,
    Dictid = 9,
    Dict = 10,
    Type = 11,
    Typedo = 12,
    Stored = 13,
    Copy = 14,
    Copy_ = 15,
    Table = 16,
    Lenlens = 17,
    Codelens = 18,
    Len_ = 19,
    Len = 20,
    Lenext = 21,
    Dist = 22,
    Distext = 23,
    Match = 24,
    Lit = 25,
    Check = 26,
    Length = 27,
    Done = 28,
    Bad = 29,
    Mem = 30,
    Sync = 31,
}

#[derive(Debug, Clone, Copy)]
pub struct Code {
    pub op: u8,
    pub bits: u8,
    pub val: u16,
}

#[derive(Debug)]
pub struct InflateState {
    pub mode: InflateMode,
    pub last: i32,
    pub wrap: i32,
    pub havedict: i32,
    pub flags: i32,
    pub dmax: u32,
    pub check: u64,
    pub total: u64,
    pub head: Option<Box<GzHeader>>,
    pub wbits: u32,
    pub wsize: u32,
    pub whave: u32,
    pub wnext: u32,
    pub window: Vec<u8>,
    pub hold: u64,
    pub bits: u32,
    pub length: u32,
    pub offset: u32,
    pub extra: u32,
    pub lencode: Vec<Code>,
    pub distcode: Vec<Code>,
    pub lenbits: u32,
    pub distbits: u32,
    pub ncode: u32,
    pub nlen: u32,
    pub ndist: u32,
    pub have: u32,
    pub next: Vec<Code>,
    pub lens: [u16; 320],
    pub work: [u16; 288],
    pub codes: [Code; 1444],
    pub sane: i32,
    pub back: i32,
    pub was: u32,
}

#[derive(Debug)]
pub struct GzHeader {
    pub text: i32,
    pub time: u64,
    pub xflags: i32,
    pub os: i32,
    pub extra: Vec<u8>,
    pub name: Vec<u8>,
    pub comment: Vec<u8>,
    pub hcrc: i32,
    pub done: i32,
}

#[derive(Debug)]
pub struct ZStream {
    pub next_in: Vec<u8>,
    pub next_out: Vec<u8>,
    pub avail_in: u32,
    pub avail_out: u32,
    pub total_in: u64,
    pub total_out: u64,
    pub msg: Option<String>,
    pub state: InflateState,
    pub data_type: i32,
    pub adler: u64,
}

pub fn inflate_fast(mut strm: ZStream, start: u32) -> Result<ZStream, String> {
    let mut state = strm.state;
    let mut in_buf = strm.next_in;
    let mut out_buf = strm.next_out;
    
    let mut in_ptr = 0;
    let mut out_ptr = 0;
    
    let last = in_buf.len().saturating_sub(5);
    let end = out_buf.len().saturating_sub(257);
    
    let mut hold = state.hold;
    let mut bits = state.bits;
    
    let lmask = (1 << state.lenbits) - 1;
    let dmask = (1 << state.distbits) - 1;
    
    while in_ptr < last && out_ptr < end {
        while bits < 15 {
            if in_ptr >= in_buf.len() {
                break;
            }
            hold += (in_buf[in_ptr] as u64) << bits;
            bits += 8;
            in_ptr += 1;
        }
        
        let here = state.lencode[(hold & lmask as u64) as usize];
        let op = here.bits as u32;
        hold >>= op;
        bits -= op;
        
        match here.op {
            0 => {
                if out_ptr >= out_buf.len() {
                    return Err("Output buffer overflow".to_string());
                }
                out_buf[out_ptr] = here.val as u8;
                out_ptr += 1;
            }
            op if op & 16 != 0 => {
                let mut len = here.val as u32;
                let op = op & 15;
                
                if op != 0 {
                    if bits < op {
                        if in_ptr >= in_buf.len() {
                            break;
                        }
                        hold += (in_buf[in_ptr] as u64) << bits;
                        bits += 8;
                        in_ptr += 1;
                    }
                    len += (hold as u32) & ((1 << op) - 1);
                    hold >>= op;
                    bits -= op;
                }
                
                while bits < 15 {
                    if in_ptr >= in_buf.len() {
                        break;
                    }
                    hold += (in_buf[in_ptr] as u64) << bits;
                    bits += 8;
                    in_ptr += 1;
                }
                
                let here = state.distcode[(hold & dmask as u64) as usize];
                let op = here.bits as u32;
                hold >>= op;
                bits -= op;
                
                match here.op {
                    op if op & 16 != 0 => {
                        let mut dist = here.val as u32;
                        let op = op & 15;
                        
                        if bits < op {
                            if in_ptr >= in_buf.len() {
                                break;
                            }
                            hold += (in_buf[in_ptr] as u64) << bits;
                            bits += 8;
                            in_ptr += 1;
                            
                            if bits < op {
                                if in_ptr >= in_buf.len() {
                                    break;
                                }
                                hold += (in_buf[in_ptr] as u64) << bits;
                                bits += 8;
                                in_ptr += 1;
                            }
                        }
                        
                        dist += (hold as u32) & ((1 << op) - 1);
                        hold >>= op;
                        bits -= op;
                        
                        if dist as usize > out_ptr {
                            if state.sane != 0 {
                                return Err("Invalid distance too far back".to_string());
                            }
                        }
                        
                        let mut from = if dist as usize <= out_ptr {
                            out_ptr - dist as usize
                        } else {
                            let offset = dist as usize - out_ptr;
                            if offset > state.window.len() {
                                return Err("Invalid distance".to_string());
                            }
                            state.window.len() - offset
                        };
                        
                        while len > 0 {
                            if out_ptr >= out_buf.len() {
                                return Err("Output buffer overflow".to_string());
                            }
                            
                            let byte = if from < state.window.len() {
                                state.window[from]
                            } else {
                                out_buf[from - state.window.len()]
                            };
                            
                            out_buf[out_ptr] = byte;
                            out_ptr += 1;
                            from += 1;
                            len -= 1;
                        }
                    }
                    op if op & 64 == 0 => {
                        // Continue processing
                    }
                    _ => {
                        return Err("Invalid distance code".to_string());
                    }
                }
            }
            op if op & 64 == 0 => {
                // Continue processing
            }
            op if op & 32 != 0 => {
                state.mode = InflateMode::Type;
                break;
            }
            _ => {
                return Err("Invalid literal/length code".to_string());
            }
        }
    }
    
    let len = bits / 8;
    in_ptr = in_ptr.saturating_sub(len as usize);
    bits -= len * 8;
    hold &= (1 << bits) - 1;
    
    strm.next_in = in_buf[in_ptr..].to_vec();
    strm.next_out = out_buf[out_ptr..].to_vec();
    strm.avail_in = (in_buf.len() - in_ptr) as u32;
    strm.avail_out = (out_buf.len() - out_ptr) as u32;
    state.hold = hold;
    state.bits = bits;
    strm.state = state;
    
    Ok(strm)
}