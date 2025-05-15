use std::fs::File;
use std::io::{Read, Write};
use std::num::Wrapping;

const INBUF_SIZE: usize = 0x8000;
const OUTBUF_SIZE: usize = 0x8000;
const WINDOW_SIZE: usize = 0x8000;

struct GzipDecoder {
    inbuf: [u8; INBUF_SIZE],
    outbuf: [u8; OUTBUF_SIZE],
    window: [u8; WINDOW_SIZE],
    insize: usize,
    inptr: usize,
    outcnt: usize,
    bytes_out: usize,
    ifd: File,
    ofd: File,
    orig_len: usize,
    max_len: i32,
    literal: [u8; 256],
    lit_base: [i32; 26],
    leaves: [i32; 26],
    parents: [i32; 26],
    peek_bits: i32,
    bitbuf: usize,
    valid: i32,
}

impl GzipDecoder {
    fn new(input: File, output: File) -> Self {
        Self {
            inbuf: [0; INBUF_SIZE],
            outbuf: [0; OUTBUF_SIZE],
            window: [0; WINDOW_SIZE],
            insize: 0,
            inptr: 0,
            outcnt: 0,
            bytes_out: 0,
            ifd: input,
            ofd: output,
            orig_len: 0,
            max_len: 0,
            literal: [0; 256],
            lit_base: [0; 26],
            leaves: [0; 26],
            parents: [0; 26],
            peek_bits: 0,
            bitbuf: 0,
            valid: 0,
        }
    }

    fn gzip_error(&self, msg: &str) -> ! {
        eprintln!("{}", msg);
        std::process::exit(1);
    }

    fn fill_inbuf(&mut self, eof_ok: bool) -> Result<u8, &'static str> {
        if self.inptr >= self.insize {
            match self.ifd.read(&mut self.inbuf) {
                Ok(n) => {
                    if n == 0 {
                        if eof_ok {
                            return Err("unexpected EOF");
                        } else {
                            self.gzip_error("invalid compressed data -- unexpected end of file");
                        }
                    }
                    self.insize = n;
                    self.inptr = 0;
                }
                Err(_) => self.gzip_error("read error"),
            }
        }
        let b = self.inbuf[self.inptr];
        self.inptr += 1;
        Ok(b)
    }

    fn flush_window(&mut self) {
        if self.outcnt > 0 {
            if let Err(_) = self.ofd.write_all(&self.window[..self.outcnt]) {
                self.gzip_error("write error");
            }
            self.bytes_out += self.outcnt;
            self.outcnt = 0;
        }
    }

    fn read_byte(&mut self) -> u8 {
        match self.fill_inbuf(false) {
            Ok(b) => b,
            Err(_) => self.gzip_error("invalid compressed data -- unexpected end of file"),
        }
    }

    fn read_tree(&mut self) {
        self.orig_len = 0;
        for _ in 0..4 {
            self.orig_len = (self.orig_len << 8) | self.read_byte() as usize;
        }

        self.max_len = self.read_byte() as i32;
        if !(0 < self.max_len && self.max_len <= 25) {
            self.gzip_error("invalid compressed data -- Huffman code bit length out of range");
        }

        let mut n = 0;
        let mut max_leaves = 1;
        let mut len = 1;

        while len <= self.max_len {
            self.leaves[len as usize] = self.read_byte() as i32;
            if (max_leaves - (len == self.max_len) as i32) < self.leaves[len as usize] {
                self.gzip_error("too many leaves in Huffman tree");
            }
            max_leaves = (max_leaves - self.leaves[len as usize] + 1) * 2 - 1;
            n += self.leaves[len as usize];
            len += 1;
        }

        if 256 <= n {
            self.gzip_error("too many leaves in Huffman tree");
        }

        self.leaves[self.max_len as usize] += 1;

        let mut base = 0;
        len = 1;
        while len <= self.max_len {
            self.lit_base[len as usize] = base;
            let mut remaining = self.leaves[len as usize];
            while remaining > 0 {
                self.literal[base as usize] = self.read_byte();
                base += 1;
                remaining -= 1;
            }
            len += 1;
        }

        self.leaves[self.max_len as usize] += 1;
    }

    fn build_tree(&mut self) {
        let mut nodes = 0;
        let mut len = self.max_len;

        while len >= 1 {
            nodes >>= 1;
            self.parents[len as usize] = nodes;
            self.lit_base[len as usize] -= nodes;
            nodes += self.leaves[len as usize];
            len -= 1;
        }

        if nodes >> 1 != 1 {
            self.gzip_error("too few leaves in Huffman tree");
        }

        self.peek_bits = if self.max_len <= 12 { self.max_len } else { 12 };
        
        let mut prefix_pos = (1 << self.peek_bits) as usize;
        len = 1;
        while len <= self.peek_bits {
            let mut prefixes = self.leaves[len as usize] << (self.peek_bits - len);
            while prefixes > 0 {
                prefix_pos -= 1;
                self.outbuf[prefix_pos] = len as u8;
                prefixes -= 1;
            }
            len += 1;
        }

        while prefix_pos > 0 {
            prefix_pos -= 1;
            self.outbuf[prefix_pos] = 0;
        }
    }

    pub fn unpack(&mut self) -> i32 {
        self.read_tree();
        self.build_tree();

        self.valid = 0;
        self.bitbuf = 0;

        let peek_mask = ((1 << self.peek_bits) - 1) as usize;
        let eob = (self.leaves[self.max_len as usize] - 1) as usize;

        loop {
            while (self.valid as i32) < self.peek_bits {
                self.bitbuf = (self.bitbuf << 8) | self.read_byte() as usize;
                self.valid += 8;
            }

            let mut peek = (self.bitbuf >> (self.valid - self.peek_bits as i32) as usize) & peek_mask;
            let mut len = self.outbuf[peek] as i32;

            if len > 0 {
                peek >>= self.peek_bits - len;
            } else {
                let mut mask = peek_mask;
                len = self.peek_bits;
                while peek < self.parents[len as usize] as usize {
                    len += 1;
                    mask = (mask << 1) + 1;
                    while (self.valid as i32) < len {
                        self.bitbuf = (self.bitbuf << 8) | self.read_byte() as usize;
                        self.valid += 8;
                    }
                    peek = (self.bitbuf >> (self.valid - len as i32) as usize) & mask;
                }
            }

            if peek == eob && len == self.max_len {
                break;
            }

            self.window[self.outcnt] = self.literal[peek + self.lit_base[len as usize] as usize];
            self.outcnt += 1;

            if self.outcnt == WINDOW_SIZE {
                self.flush_window();
            }

            self.valid -= len;
        }

        self.flush_window();

        if self.orig_len != (self.bytes_out & 0xFFFFFFFF) {
            self.gzip_error("invalid compressed data--length error");
        }

        0
    }
}