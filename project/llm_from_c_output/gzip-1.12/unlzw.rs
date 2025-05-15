use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

type CharType = u8;
type CodeInt = i32;
type CountInt = u32;
type CountShort = u16;
type CmpCodeInt = u32;

const MAX_BITS: usize = 16;
const INIT_BITS: usize = 9;
const FIRST: CodeInt = 257;
const CLEAR: CodeInt = 256;
const BLOCK_MODE: u8 = 0x80;
const LZW_RESERVED: u8 = 0x60;
const BIT_MASK: u8 = 0x1f;
const INBUFSIZ: usize = 8192;
const OUTBUFSIZ: usize = 8192;
const INBUF_EXTRA: usize = 64;
const DIST_BUFSIZE: usize = 65536;
const BITS: usize = 16;

struct LZWState {
    maxbits: u8,
    block_mode: bool,
    n_bits: usize,
    maxcode: CodeInt,
    maxmaxcode: CodeInt,
    free_ent: CodeInt,
    oldcode: CodeInt,
    finchar: i32,
    bitmask: u32,
    posbits: usize,
    outpos: usize,
    insize: usize,
    tab_prefix: Vec<u16>,
    tab_suffix: Vec<u8>,
    inbuf: Vec<u8>,
    outbuf: Vec<u8>,
    de_stack: Vec<u8>,
}

impl LZWState {
    fn new() -> Self {
        let tab_size = 1 << BITS;
        LZWState {
            maxbits: 0,
            block_mode: false,
            n_bits: INIT_BITS,
            maxcode: (1 << INIT_BITS) - 1,
            maxmaxcode: 0,
            free_ent: 0,
            oldcode: -1,
            finchar: 0,
            bitmask: (1 << INIT_BITS) - 1,
            posbits: 0,
            outpos: 0,
            insize: 0,
            tab_prefix: vec![0; tab_size],
            tab_suffix: vec![0; tab_size],
            inbuf: vec![0; INBUFSIZ + INBUF_EXTRA],
            outbuf: vec![0; OUTBUFSIZ],
            de_stack: vec![0; DIST_BUFSIZE],
        }
    }

    fn clear_tab_prefix(&mut self) {
        for i in 0..256 {
            self.tab_prefix[i] = 0;
        }
    }

    fn input(&mut self, n_bits: usize) -> CodeInt {
        let mut c: CodeInt;
        let byte_pos = self.posbits >> 3;
        let p = &self.inbuf[byte_pos..byte_pos + 3];

        c = ((p[0] as CodeInt) | ((p[1] as CodeInt) << 8) | ((p[2] as CodeInt) << 16);
        c = (c >> (self.posbits & 0x7)) & ((1 << n_bits) - 1);
        self.posbits += n_bits;
        c
    }
}

fn unlzw(input: &mut dyn Read, output: &mut dyn Write) -> io::Result<()> {
    let mut state = LZWState::new();

    let mut buf = [0; 1];
    input.read_exact(&mut buf)?;
    state.maxbits = buf[0];
    state.block_mode = (state.maxbits & BLOCK_MODE) != 0;
    
    if (state.maxbits & LZW_RESERVED) != 0 {
        eprintln!("warning: unknown flags 0x{:x}", state.maxbits & LZW_RESERVED);
    }
    
    state.maxbits &= BIT_MASK;
    state.maxmaxcode = 1 << state.maxbits;

    if state.maxbits > BITS as u8 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("compressed with {} bits, can only handle {} bits", state.maxbits, BITS),
        ));
    }

    state.n_bits = INIT_BITS;
    state.maxcode = (1 << INIT_BITS) - 1;
    state.bitmask = state.maxcode as u32;
    state.oldcode = -1;
    state.finchar = 0;
    state.outpos = 0;
    state.posbits = 0;
    state.free_ent = if state.block_mode { FIRST } else { 256 };

    state.clear_tab_prefix();
    for code in (0..=255).rev() {
        state.tab_suffix[code as usize] = code as u8;
    }

    loop {
        let rsize = read_buffer(input, &mut state.inbuf[state.insize..])?;
        if rsize == 0 {
            break;
        }
        state.insize += rsize;
        
        let inbits = if rsize != 0 {
            ((state.insize - state.insize % state.n_bits) * 8) as usize
        } else {
            (state.insize * 8) - (state.n_bits - 1)
        };

        while inbits > state.posbits {
            if state.free_ent > state.maxcode {
                state.posbits = ((state.posbits - 1) + 
                    ((state.n_bits * 8) - (state.posbits - 1 + state.n_bits * 8) % (state.n_bits * 8)));
                state.n_bits += 1;
                if state.n_bits == state.maxbits as usize {
                    state.maxcode = state.maxmaxcode;
                } else {
                    state.maxcode = (1 << state.n_bits) - 1;
                }
                state.bitmask = state.maxcode as u32;
                continue;
            }

            let code = state.input(state.n_bits);
            
            if state.oldcode == -1 {
                if code >= 256 {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "corrupt input"));
                }
                state.outbuf[state.outpos] = code as u8;
                state.outpos += 1;
                state.finchar = code as i32;
                state.oldcode = code;
                continue;
            }

            if code == CLEAR && state.block_mode {
                state.clear_tab_prefix();
                state.free_ent = FIRST - 1;
                state.posbits = ((state.posbits - 1) + 
                    ((state.n_bits * 8) - (state.posbits - 1 + state.n_bits * 8) % (state.n_bits * 8)));
                state.n_bits = INIT_BITS;
                state.maxcode = (1 << INIT_BITS) - 1;
                state.bitmask = state.maxcode as u32;
                continue;
            }

            let incode = code;
            let mut stackp = state.de_stack.len();

            if code >= state.free_ent {
                if code > state.free_ent {
                    if state.outpos > 0 {
                        output.write_all(&state.outbuf[..state.outpos])?;
                        state.outpos = 0;
                    }
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "corrupt input"));
                }
                stackp -= 1;
                state.de_stack[stackp] = state.finchar as u8;
                state.oldcode = code;
            }

            let mut code = code;
            while code >= 256 {
                stackp -= 1;
                state.de_stack[stackp] = state.tab_suffix[code as usize];
                code = state.tab_prefix[code as usize] as CodeInt;
            }

            stackp -= 1;
            state.de_stack[stackp] = code as u8;
            state.finchar = code as i32;

            let count = state.de_stack.len() - stackp;
            if state.outpos + count > OUTBUFSIZ {
                let mut remaining = count;
                let mut offset = 0;
                while remaining > 0 {
                    let chunk = std::cmp::min(OUTBUFSIZ - state.outpos, remaining);
                    if chunk > 0 {
                        state.outbuf[state.outpos..state.outpos + chunk]
                            .copy_from_slice(&state.de_stack[stackp + offset..stackp + offset + chunk]);
                        state.outpos += chunk;
                        offset += chunk;
                        remaining -= chunk;
                    }
                    if state.outpos >= OUTBUFSIZ {
                        output.write_all(&state.outbuf)?;
                        state.outpos = 0;
                    }
                }
            } else {
                state.outbuf[state.outpos..state.outpos + count]
                    .copy_from_slice(&state.de_stack[stackp..stackp + count]);
                state.outpos += count;
            }

            if state.free_ent < state.maxmaxcode {
                state.tab_prefix[state.free_ent as usize] = state.oldcode as u16;
                state.tab_suffix[state.free_ent as usize] = state.finchar as u8;
                state.free_ent += 1;
            }
            state.oldcode = incode;
        }
    }

    if state.outpos > 0 {
        output.write_all(&state.outbuf[..state.outpos])?;
    }
    Ok(())
}

fn read_buffer(input: &mut dyn Read, buf: &mut [u8]) -> io::Result<usize> {
    input.read(buf)
}

fn write_buf(output: &mut dyn Write, buf: &[u8], len: usize) -> io::Result<()> {
    output.write_all(&buf[..len])
}