use std::io::{Read, Write};

const INBUF_SIZE: usize = 1 << 13;
const OUTBUF_SIZE: usize = 1 << 13;
const D_BUF_SIZE: usize = 4096;
const WINDOW_SIZE: usize = 1 << 13;
const PREV_SIZE: usize = 1 << 16;

struct Decoder<R: Read, W: Write> {
    reader: R,
    writer: W,
    
    inbuf: [u8; INBUF_SIZE],
    outbuf: [u8; OUTBUF_SIZE],
    d_buf: [u16; D_BUF_SIZE],
    window: [u8; WINDOW_SIZE],
    prev: [u16; PREV_SIZE],
    
    insize: usize,
    inptr: usize,
    
    pt_len: [u8; 32],
    blocksize: u32,
    pt_table: [u16; 256],
    bitbuf: u16,
    subbitbuf: u32,
    bitcount: i32,
    
    j: i32,
    done: bool,
}

impl<R: Read, W: Write> Decoder<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            reader,
            writer,
            inbuf: [0; INBUF_SIZE],
            outbuf: [0; OUTBUF_SIZE],
            d_buf: [0; D_BUF_SIZE],
            window: [0; WINDOW_SIZE],
            prev: [0; PREV_SIZE],
            insize: 0,
            inptr: 0,
            pt_len: [0; 32],
            blocksize: 0,
            pt_table: [0; 256],
            bitbuf: 0,
            subbitbuf: 0,
            bitcount: 0,
            j: 0,
            done: false,
        }
    }
    
    fn fill_inbuf(&mut self, eof_ok: bool) -> Result<u8, &'static str> {
        if self.inptr >= self.insize {
            self.insize = self.reader.read(&mut self.inbuf).map_err(|_| "Read error")?;
            self.inptr = 0;
            if self.insize == 0 {
                if eof_ok {
                    return Ok(0);
                }
                return Err("Unexpected EOF");
            }
        }
        let byte = self.inbuf[self.inptr];
        self.inptr += 1;
        Ok(byte)
    }
    
    fn write_buf(&mut self, buf: &[u8]) -> Result<(), &'static str> {
        self.writer.write_all(buf).map_err(|_| "Write error")
    }
    
    fn gzip_error(&mut self, msg: &str) -> Result<(), &'static str> {
        Err(msg)
    }
    
    fn fillbuf(&mut self, n: i32) -> Result<(), &'static str> {
        self.bitbuf <<= n;
        let mut remaining = n;
        
        while remaining > self.bitcount {
            remaining -= self.bitcount;
            self.bitbuf |= (self.subbitbuf << remaining) as u16;
            
            let byte = self.fill_inbuf(true)? as u32;
            self.subbitbuf = if byte == 0xFFFFFFFF { 0 } else { byte };
            self.bitcount = 8;
        }
        
        self.bitcount -= remaining;
        self.bitbuf |= (self.subbitbuf >> self.bitcount) as u16;
        Ok(())
    }
    
    fn getbits(&mut self, n: i32) -> Result<u32, &'static str> {
        let x = (self.bitbuf as u32) >> (16 - n);
        self.fillbuf(n)?;
        Ok(x)
    }
    
    fn init_getbits(&mut self) -> Result<(), &'static str> {
        self.bitbuf = 0;
        self.subbitbuf = 0;
        self.bitcount = 0;
        self.fillbuf(16)
    }
    
    fn make_table(
        &mut self,
        nchar: i32,
        bitlen: &[u8],
        tablebits: i32,
        table: &mut [u16],
    ) -> Result<(), &'static str> {
        let mut count = [0u16; 17];
        let mut weight = [0u16; 17];
        let mut start = [0u16; 18];
        
        for &len in bitlen.iter().take(nchar as usize) {
            count[len as usize] += 1;
        }
        
        start[1] = 0;
        for i in 1..=16 {
            start[i + 1] = start[i] + (count[i] << (16 - i));
        }
        
        if start[17] != 0 {
            return self.gzip_error("Bad table");
        }
        
        let jutbits = 16 - tablebits;
        for i in 1..=tablebits as usize {
            start[i] >>= jutbits;
            weight[i] = 1 << (tablebits - i as i32);
        }
        
        for i in (tablebits as usize + 1)..=16 {
            weight[i] = 1 << (16 - i);
        }
        
        let mut i = start[tablebits as usize + 1] >> jutbits;
        if i != 0 {
            let k = 1 << tablebits;
            while i < k as u16 {
                table[i as usize] = 0;
                i += 1;
            }
        }
        
        let mut avail = nchar as u16;
        let mask = 1 << (15 - tablebits);
        
        for ch in 0..nchar as usize {
            let len = bitlen[ch] as usize;
            if len == 0 {
                continue;
            }
            
            let nextcode = start[len] + weight[len];
            if len <= tablebits as usize {
                if (1 << tablebits) < nextcode as i32 {
                    return self.gzip_error("Bad table");
                }
                
                for i in start[len]..nextcode {
                    table[i as usize] = ch as u16;
                }
            } else {
                let mut k = start[len];
                let mut p = &mut table[(k >> jutbits) as usize];
                let mut i = len - tablebits as usize;
                
                while i != 0 {
                    if *p == 0 {
                        self.prev[avail as usize] = 0;
                        self.prev[0x8000 + avail as usize] = 0;
                        *p = avail;
                        avail += 1;
                    }
                    
                    if k & mask != 0 {
                        p = &mut self.prev[0x8000 + *p as usize];
                    } else {
                        p = &mut self.prev[*p as usize];
                    }
                    
                    k <<= 1;
                    i -= 1;
                }
                
                *p = ch as u16;
            }
            
            start[len] = nextcode;
        }
        
        Ok(())
    }
    
    fn read_pt_len(&mut self, nn: i32, nbit: i32, i_special: i32) -> Result<(), &'static str> {
        let n = self.getbits(nbit)? as i32;
        
        if n == 0 {
            let c = self.getbits(nbit)? as i32;
            self.pt_len[..nn as usize].fill(0);
            self.pt_table[..256].fill(c as u16);
            return Ok(());
        }
        
        let mut i = 0;
        while i < n {
            let mut c = (self.bitbuf as i32) >> (16 - 3);
            
            if c == 7 {
                let mut mask = 1 << (16 - 1 - 3);
                while mask & self.bitbuf as u32 != 0 {
                    mask >>= 1;
                    c += 1;
                }
                
                if c > 16 {
                    return self.gzip_error("Bad table");
                }
            }
            
            self.fillbuf(if c < 7 { 3 } else { c - 3 })?;
            self.pt_len[i as usize] = c as u8;
            i += 1;
            
            if i == i_special {
                let mut c = self.getbits(2)? as i32;
                while c > 0 {
                    c -= 1;
                    self.pt_len[i as usize] = 0;
                    i += 1;
                }
            }
        }
        
        while i < nn {
            self.pt_len[i as usize] = 0;
            i += 1;
        }
        
        self.make_table(nn, &self.pt_len, 8, &mut self.pt_table)
    }
    
    fn read_c_len(&mut self) -> Result<(), &'static str> {
        let n = self.getbits(9)? as i32;
        
        if n == 0 {
            let c = self.getbits(9)? as i32;
            self.outbuf[..(255 + 256 + 2 - 3) as usize].fill(0);
            self.d_buf[..4096].fill(c as u16);
            return Ok(());
        }
        
        let mut i = 0;
        while i < n {
            let mut c = self.pt_table[(self.bitbuf >> (16 - 8)) as usize] as i32;
            
            if c >= 19 {
                let mut mask = 1 << (16 - 1 - 8);
                loop {
                    if self.bitbuf as u32 & mask != 0 {
                        c = self.prev[0x8000 + c as usize] as i32;
                    } else {
                        c = self.prev[c as usize] as i32;
                    }
                    mask >>= 1;
                    if c < 19 {
                        break;
                    }
                }
            }
            
            self.fillbuf(self.pt_len[c as usize] as i32)?;
            
            if c <= 2 {
                c = match c {
                    0 => 1,
                    1 => 3 + self.getbits(4)? as i32,
                    2 => 20 + self.getbits(9)? as i32,
                    _ => unreachable!(),
                };
                
                while c > 0 {
                    c -= 1;
                    self.outbuf[i as usize] = 0;
                    i += 1;
                }
            } else {
                self.outbuf[i as usize] = (c - 2) as u8;
                i += 1;
            }
        }
        
        while i < (255 + 256 + 2 - 3) {
            self.outbuf[i as usize] = 0;
            i += 1;
        }
        
        self.make_table(
            255 + 256 + 2 - 3,
            &self.outbuf,
            12,
            &mut self.d_buf,
        )
    }
    
    fn decode_c(&mut self) -> Result<u32, &'static str> {
        if self.blocksize == 0 {
            self.blocksize = self.getbits(16)?;
            if self.blocksize == 0 {
                return Ok((255 + 256 + 2 - 3) as u32);
            }
            
            self.read_pt_len(19, 5, 3)?;
            self.read_c_len()?;
            self.read_pt_len(14, 4, -1)?;
        }
        
        self.blocksize -= 1;
        
        let mut j = self.d_buf[(self.bitbuf >> (16 - 12)) as usize] as u32;
        
        if j >= (255 + 256 + 2 - 3) as u32 {
            let mut mask = 1 << (16 - 1 - 12);
            loop {
                if self.bitbuf as u32 & mask != 0 {
                    j = self.prev[0x8000 + j as usize] as u32;
                } else {
                    j = self.prev[j as usize] as u32;
                }
                mask >>= 1;
                if j < (255 + 256 + 2 - 3) as u32 {
                    break;
                }
            }
        }
        
        self.fillbuf(self.outbuf[j as usize] as i32)?;
        Ok(j)
    }
    
    fn decode_p(&mut self) -> Result<u32, &'static str> {
        let mut j = self.pt_table[(self.bitbuf >> (16 - 8)) as usize] as u32;
        
        if j >= 14 {
            let mut mask = 1 << (16 - 1 - 8);
            loop {
                if self.bitbuf as u32 & mask != 0 {
                    j = self.prev[0x8000 + j as usize] as u32;
                } else {
                    j = self.prev[j as usize] as u32;
                }
                mask >>= 1;
                if j < 14 {
                    break;
                }
            }
        }
        
        self.fillbuf(self.pt_len[j as usize] as i32)?;
        
        if j != 0 {
            j = (1 << (j - 1)) + self.getbits((j - 1) as i32)?;
        }
        
        Ok(j)
    }
    
    fn huf_decode_start(&mut self) -> Result<(), &'static str> {
        self.init_getbits()?;
        self.blocksize = 0;
        Ok(())
    }
    
    fn decode_start(&mut self) -> Result<(), &'static str> {
        self.huf_decode_start()?;
        self.j = 0;
        self.done = false;
        Ok(())
    }
    
    fn decode_block(&mut self, count: usize, buffer: &mut [u8]) -> Result<usize, &'static str> {
        let mut r = 0;
        let mut i = 0;
        
        loop {
            self.j -= 1;
            if self.j < 0 {
                break;
            }
            
            buffer[r] = buffer[i];
            i = (i + 1) & (WINDOW_SIZE - 1);
            r += 1;
            
            if r == count {
                return Ok(r);
            }
        }
        
        loop {
            let c = self.decode_c()?;
            if c == (255 + 256 + 2 - 3) as u32 {
                self.done = true;
                return Ok(r);
            }
            
            if c <= 255 {
                buffer[r] = c as u8;
                r += 1;
                if r == count {
                    return Ok(r);
                }
            } else {
                self.j = (c - (255 + 1 - 3)) as i32;
                i = (r as u32).wrapping_sub(self.decode_p()?).wrapping_sub(1) as usize & (WINDOW_SIZE - 1);
                
                loop {
                    self.j -= 1;
                    if self.j < 0 {
                        break;
                    }
                    
                    buffer[r] = buffer[i];
                    i = (i + 1) & (WINDOW_SIZE - 1);
                    r += 1;
                    
                    if r == count {
                        return Ok(r);
                    }
                }
            }
        }
    }
    
    pub fn decode(&mut self) -> Result<(), &'static str> {
        self.decode_start()?;
        
        while !self.done {
            let n = self.decode_block(WINDOW_SIZE, &mut self.window)?;
            if n > 0 {
                self.write_buf(&self.window[..n])?;
            }
        }
        
        Ok(())
    }
}

pub fn unlzh<R: Read, W: Write>(input: R, output: W) -> Result<(), &'static str> {
    let mut decoder = Decoder::new(input, output);
    decoder.decode()
}