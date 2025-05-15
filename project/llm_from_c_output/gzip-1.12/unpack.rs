use std::io::{Read, Write};

const MAX_BITLEN: usize = 25;
const LITERALS: usize = 256;
const MAX_PEEK: usize = 12;

struct Unpacker {
    orig_len: u32,
    max_len: usize,
    literal: [u8; LITERALS],
    lit_base: [usize; MAX_BITLEN + 1],
    leaves: [usize; MAX_BITLEN + 1],
    parents: [usize; MAX_BITLEN + 1],
    peek_bits: usize,
    bitbuf: u32,
    valid: usize,
    input: Box<dyn Read>,
    output: Box<dyn Write>,
    bytes_out: u32,
}

impl Unpacker {
    fn new(input: Box<dyn Read>, output: Box<dyn Write>) -> Self {
        Self {
            orig_len: 0,
            max_len: 0,
            literal: [0; LITERALS],
            lit_base: [0; MAX_BITLEN + 1],
            leaves: [0; MAX_BITLEN + 1],
            parents: [0; MAX_BITLEN + 1],
            peek_bits: 0,
            bitbuf: 0,
            valid: 0,
            input,
            output,
            bytes_out: 0,
        }
    }

    fn read_byte(&mut self) -> Result<u8, String> {
        let mut buf = [0; 1];
        match self.input.read_exact(&mut buf) {
            Ok(_) => Ok(buf[0]),
            Err(_) => Err("invalid compressed data -- unexpected end of file".to_string()),
        }
    }

    fn look_bits(&mut self, bits: usize, mask: u32) -> Result<u32, String> {
        while self.valid < bits {
            let byte = self.read_byte()? as u32;
            self.bitbuf = (self.bitbuf << 8) | byte;
            self.valid += 8;
        }
        Ok((self.bitbuf >> (self.valid - bits)) & mask)
    }

    fn skip_bits(&mut self, bits: usize) {
        self.valid -= bits;
    }

    fn clear_bitbuf(&mut self) {
        self.valid = 0;
        self.bitbuf = 0;
    }

    fn read_tree(&mut self) -> Result<(), String> {
        for n in 1..=4 {
            let byte = self.read_byte()? as u32;
            self.orig_len = (self.orig_len << 8) | byte;
        }

        self.max_len = self.read_byte()? as usize;
        if !(0 < self.max_len && self.max_len <= MAX_BITLEN) {
            return Err("invalid compressed data -- Huffman code bit length out of range".to_string());
        }

        let mut n = 0;
        let mut max_leaves = 1;
        for len in 1..=self.max_len {
            self.leaves[len] = self.read_byte()? as usize;
            if max_leaves - (if len == self.max_len { 1 } else { 0 }) < self.leaves[len] {
                return Err("too many leaves in Huffman tree".to_string());
            }
            max_leaves = (max_leaves - self.leaves[len] + 1) * 2 - 1;
            n += self.leaves[len];
        }

        if LITERALS <= n {
            return Err("too many leaves in Huffman tree".to_string());
        }

        self.leaves[self.max_len] += 1;

        let mut base = 0;
        for len in 1..=self.max_len {
            self.lit_base[len] = base;
            for _ in 0..self.leaves[len] {
                self.literal[base] = self.read_byte()?;
                base += 1;
            }
        }
        self.leaves[self.max_len] += 1;
        Ok(())
    }

    fn build_tree(&mut self) -> Result<(), String> {
        let mut nodes = 0;
        for len in (1..=self.max_len).rev() {
            nodes >>= 1;
            self.parents[len] = nodes;
            self.lit_base[len] -= nodes;
            nodes += self.leaves[len];
        }

        if (nodes >> 1) != 1 {
            return Err("too few leaves in Huffman tree".to_string());
        }

        self.peek_bits = std::cmp::min(self.max_len, MAX_PEEK);
        let mut prefix_len = vec![0; 1 << self.peek_bits];
        let mut prefixp = (1 << self.peek_bits) - 1;

        for len in 1..=self.peek_bits {
            let mut prefixes = self.leaves[len] << (self.peek_bits - len);
            while prefixes > 0 {
                prefix_len[prefixp] = len as u8;
                prefixp -= 1;
                prefixes -= 1;
            }
        }

        while prefixp > 0 {
            prefix_len[prefixp] = 0;
            prefixp -= 1;
        }

        Ok(())
    }

    fn unpack(&mut self) -> Result<(), String> {
        self.read_tree()?;
        self.build_tree()?;
        self.clear_bitbuf();
        let peek_mask = (1 << self.peek_bits) - 1;

        let eob = self.leaves[self.max_len] - 1;

        loop {
            let peek = self.look_bits(self.peek_bits, peek_mask)? as usize;
            let mut len = if peek < prefix_len.len() { prefix_len[peek] as usize } else { 0 };

            let mut code = peek;
            if len > 0 {
                code >>= self.peek_bits - len;
            } else {
                let mut mask = peek_mask;
                len = self.peek_bits;

                while code < self.parents[len] {
                    len += 1;
                    mask = (mask << 1) + 1;
                    code = self.look_bits(len, mask)? as usize;
                }
            }

            if code == eob && len == self.max_len {
                break;
            }

            let literal = self.literal[code + self.lit_base[len]];
            self.output.write_all(&[literal]).map_err(|e| e.to_string())?;
            self.bytes_out += 1;
            self.skip_bits(len);
        }

        if self.orig_len != self.bytes_out {
            return Err("invalid compressed data--length error".to_string());
        }

        Ok(())
    }
}

pub fn unpack(input: Box<dyn Read>, output: Box<dyn Write>) -> Result<(), String> {
    let mut unpacker = Unpacker::new(input, output);
    unpacker.unpack()
}