use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::slice;

const BLOCK_MODE: i32 = 0x80;
const MAX_BITS: i32 = 16;
const BUFFER_SIZE: usize = 0x40000;

struct LzwState {
    inbuf: Vec<u8>,
    outbuf: Vec<u8>,
    window: Vec<u8>,
    prev: Vec<u16>,
    insize: usize,
    inptr: usize,
    bytes_in: i64,
    block_mode: bool,
    maxbits: i32,
    exit_code: i32,
    quiet: bool,
    to_stdout: bool,
}

impl LzwState {
    fn new() -> Self {
        Self {
            inbuf: vec![0; BUFFER_SIZE],
            outbuf: vec![0; BUFFER_SIZE],
            window: vec![0; 65536],
            prev: vec![0; 65536],
            insize: 0,
            inptr: 0,
            bytes_in: 0,
            block_mode: false,
            maxbits: 0,
            exit_code: 0,
            quiet: false,
            to_stdout: false,
        }
    }

    fn fill_inbuf(&mut self, input: &mut impl Read, eof_ok: bool) -> io::Result<()> {
        if self.inptr >= self.insize {
            self.insize = input.read(&mut self.inbuf)?;
            self.inptr = 0;
            if self.insize == 0 && !eof_ok {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "unexpected EOF"));
            }
        }
        Ok(())
    }

    fn read_byte(&mut self, input: &mut impl Read) -> io::Result<u8> {
        self.fill_inbuf(input, false)?;
        let b = self.inbuf[self.inptr];
        self.inptr += 1;
        Ok(b)
    }

    fn write_buf(&mut self, output: &mut impl Write, count: usize) -> io::Result<()> {
        output.write_all(&self.outbuf[..count])?;
        Ok(())
    }

    fn unlzw(&mut self, input: &mut impl Read, output: &mut impl Write) -> io::Result<i32> {
        self.maxbits = self.read_byte(input)? as i32;
        self.block_mode = (self.maxbits & BLOCK_MODE) != 0;
        
        if (self.maxbits & 0x60) != 0 && !self.quiet {
            eprintln!("\nwarning: unknown flags 0x{:x}", self.maxbits & 0x60);
            if self.exit_code == 0 {
                self.exit_code = 2;
            }
        }

        self.maxbits &= 0x1f;
        let maxmaxcode = 1 << self.maxbits;

        if self.maxbits > MAX_BITS {
            eprintln!(
                "\ncompressed with {} bits, can only handle {} bits",
                self.maxbits, MAX_BITS
            );
            self.exit_code = 1;
            return Ok(1);
        }

        let mut n_bits = 9;
        let mut maxcode = (1 << n_bits) - 1;
        let mut bitmask = (1 << n_bits) - 1;
        let mut oldcode = -1;
        let mut finchar = 0;
        let mut outpos = 0;
        let mut free_ent = if self.block_mode { 257 } else { 256 };
        let mut posbits = (self.inptr * 8) as i64;

        self.prev[..256].fill(0);
        for code in (0..=255).rev() {
            self.window[code] = code as u8;
        }

        loop {
            let mut rsize = self.insize as i32;
            if rsize == 0 {
                break;
            }

            let mut inbits = if rsize != 0 {
                ((self.insize - (self.insize % n_bits as usize)) as i64) * 8
            } else {
                (self.insize as i64 * 8) - (n_bits - 1) as i64
            };

            while posbits < inbits {
                if free_ent > maxcode {
                    posbits += ((n_bits * 8) as i64 - (posbits % (n_bits * 8) as i64)) % (n_bits * 8) as i64;
                    n_bits += 1;
                    if n_bits == self.maxbits {
                        maxcode = maxmaxcode;
                    } else {
                        maxcode = (1 << n_bits) - 1;
                    }
                    bitmask = (1 << n_bits) - 1;
                    continue;
                }

                let byte_pos = (posbits / 8) as usize;
                let bit_offset = posbits % 8;
                let p = &self.inbuf[byte_pos..byte_pos + 3];
                let code = ((p[0] as u32 | (p[1] as u32) << 8 | (p[2] as u32) << 24) >> (bit_offset as usize) & bitmask;

                posbits += n_bits as i64;

                if oldcode == -1 {
                    if code >= 256 {
                        return Err(io::Error::new(io::ErrorKind::InvalidData, "corrupt input"));
                    }
                    oldcode = code as i32;
                    finchar = oldcode;
                    self.outbuf[outpos] = finchar as u8;
                    outpos += 1;
                } else if code == 256 && self.block_mode {
                    self.prev[..256].fill(0);
                    free_ent = 256;
                    posbits += ((n_bits * 8) as i64 - (posbits % (n_bits * 8) as i64)) % (n_bits * 8) as i64;
                    n_bits = 9;
                    maxcode = (1 << n_bits) - 1;
                    bitmask = (1 << n_bits) - 1;
                } else {
                    let mut incode = code;
                    let mut stackp = BUFFER_SIZE - 1;
                    let mut stack = vec![0; BUFFER_SIZE];

                    if code >= free_ent {
                        if code > free_ent {
                            if outpos > 0 {
                                self.write_buf(output, outpos)?;
                            }
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                if self.to_stdout {
                                    "corrupt input."
                                } else {
                                    "corrupt input. Use zcat to recover some data."
                                },
                            ));
                        }
                        stack[stackp] = finchar as u8;
                        stackp -= 1;
                        incode = oldcode as u32;
                    }

                    while incode >= 256 {
                        stack[stackp] = self.window[incode as usize];
                        stackp -= 1;
                        incode = self.prev[incode as usize] as u32;
                    }

                    finchar = self.window[incode as usize] as i32;
                    stack[stackp] = finchar as u8;
                    stackp -= 1;

                    let count = (BUFFER_SIZE - 1 - stackp) as usize;
                    if outpos + count >= BUFFER_SIZE {
                        let mut remaining = count;
                        while remaining > 0 {
                            let chunk = remaining.min(BUFFER_SIZE - outpos);
                            self.outbuf[outpos..outpos + chunk]
                                .copy_from_slice(&stack[stackp + 1..stackp + 1 + chunk]);
                            outpos += chunk;
                            stackp += chunk;
                            remaining -= chunk;

                            if outpos >= BUFFER_SIZE {
                                self.write_buf(output, BUFFER_SIZE)?;
                                outpos = 0;
                            }
                        }
                    } else {
                        self.outbuf[outpos..outpos + count]
                            .copy_from_slice(&stack[stackp + 1..stackp + 1 + count]);
                        outpos += count;
                    }

                    if (free_ent as u32) < maxmaxcode {
                        self.prev[free_ent as usize] = oldcode as u16;
                        self.window[free_ent as usize] = finchar as u8;
                        free_ent += 1;
                    }
                    oldcode = code as i32;
                }
            }

            rsize = input.read(&mut self.inbuf)? as i32;
            self.insize = rsize as usize;
            self.inptr = 0;
            self.bytes_in += rsize as i64;
            posbits = 0;
        }

        if outpos > 0 {
            self.write_buf(output, outpos)?;
        }

        Ok(0)
    }
}

pub fn unlzw(input: &mut impl Read, output: &mut impl Write) -> io::Result<i32> {
    let mut state = LzwState::new();
    state.unlzw(input, output)
}