use std::io::{Read, Write};

const DICBIT: u32 = 13;
const DICSIZ: usize = 1 << DICBIT;
const CHAR_BIT: u32 = 8;
const UCHAR_MAX: u8 = 255;
const BITBUFSIZ: u32 = CHAR_BIT * 2;
const MAXMATCH: usize = 256;
const THRESHOLD: usize = 3;
const NC: usize = UCHAR_MAX as usize + MAXMATCH + 2 - THRESHOLD;
const CBIT: u32 = 9;
const CODE_BIT: u32 = 16;
const NP: usize = DICBIT as usize + 1;
const NT: usize = CODE_BIT as usize + 3;
const PBIT: u32 = 4;
const TBIT: u32 = 5;
const NPT: usize = 1 << TBIT;

struct LzhDecoder<R: Read> {
    reader: R,
    bitbuf: u16,
    subbitbuf: u8,
    bitcount: u32,
    blocksize: u16,
    pt_len: [u8; NPT],
    pt_table: [u16; 256],
    c_len: [u8; NC],
    c_table: [u16; 4096],
    left: [u16; 2 * NC - 1],
    right: [u16; 2 * NC - 1],
    window: [u8; DICSIZ],
    j: usize,
    done: bool,
}

impl<R: Read> LzhDecoder<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            bitbuf: 0,
            subbitbuf: 0,
            bitcount: 0,
            blocksize: 0,
            pt_len: [0; NPT],
            pt_table: [0; 256],
            c_len: [0; NC],
            c_table: [0; 4096],
            left: [0; 2 * NC - 1],
            right: [0; 2 * NC - 1],
            window: [0; DICSIZ],
            j: 0,
            done: false,
        }
    }

    fn fillbuf(&mut self, n: u32) -> std::io::Result<()> {
        self.bitbuf <<= n;
        let mut remaining = n;
        while remaining > self.bitcount {
            self.bitbuf |= (self.subbitbuf as u16) << (remaining - self.bitcount);
            remaining -= self.bitcount;
            let mut buf = [0];
            if self.reader.read_exact(&mut buf).is_err() {
                self.subbitbuf = 0;
            } else {
                self.subbitbuf = buf[0];
            }
            self.bitcount = CHAR_BIT;
        }
        self.bitbuf |= (self.subbitbuf as u16) >> (self.bitcount - remaining);
        self.bitcount -= remaining;
        Ok(())
    }

    fn getbits(&mut self, n: u32) -> std::io::Result<u16> {
        let x = self.bitbuf >> (BITBUFSIZ - n);
        self.fillbuf(n)?;
        Ok(x)
    }

    fn init_getbits(&mut self) -> std::io::Result<()> {
        self.bitbuf = 0;
        self.subbitbuf = 0;
        self.bitcount = 0;
        self.fillbuf(BITBUFSIZ)
    }

    fn make_table(&mut self, nchar: usize, bitlen: &[u8], tablebits: u32, table: &mut [u16]) -> std::io::Result<()> {
        let mut count = [0; 17];
        let mut weight = [0; 17];
        let mut start = [0; 18];

        for &len in bitlen.iter().take(nchar) {
            count[len as usize] += 1;
        }

        start[1] = 0;
        for i in 1..=16 {
            start[i + 1] = start[i] + (count[i] << (16 - i));
        }

        if (start[17] & 0xffff) != 0 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Bad table"));
        }

        let jutbits = 16 - tablebits;
        for i in 1..=tablebits {
            start[i as usize] >>= jutbits;
            weight[i as usize] = 1 << (tablebits - i);
        }

        for i in (tablebits + 1)..=16 {
            weight[i as usize] = 1 << (16 - i);
        }

        let mut i = start[(tablebits + 1) as usize] >> jutbits;
        let k = 1 << tablebits;
        while i != k {
            table[i as usize] = 0;
            i += 1;
        }

        let mut avail = nchar;
        let mask = 1 << (15 - tablebits);
        for ch in 0..nchar {
            let len = bitlen[ch] as usize;
            if len == 0 {
                continue;
            }

            let mut nextcode = start[len] + weight[len];
            if len <= tablebits as usize {
                if (1 << tablebits) < nextcode {
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Bad table"));
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
                        self.right[avail] = 0;
                        self.left[avail] = 0;
                        *p = avail as u16;
                        avail += 1;
                    }
                    if k & mask != 0 {
                        p = &mut self.right[*p as usize];
                    } else {
                        p = &mut self.left[*p as usize];
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

    fn read_pt_len(&mut self, nn: usize, nbit: u32, i_special: i32) -> std::io::Result<()> {
        let n = self.getbits(nbit)? as usize;
        if n == 0 {
            let c = self.getbits(nbit)? as u8;
            for item in self.pt_len.iter_mut().take(nn) {
                *item = 0;
            }
            for item in self.pt_table.iter_mut() {
                *item = c as u16;
            }
        } else {
            let mut i = 0;
            while i < n {
                let mut c = (self.bitbuf >> (BITBUFSIZ - 3)) as u8;
                if c == 7 {
                    let mut mask = 1 << (BITBUFSIZ - 1 - 3);
                    while mask & self.bitbuf != 0 {
                        mask >>= 1;
                        c += 1;
                    }
                    if c > 16 {
                        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Bad table"));
                    }
                }
                self.fillbuf(if c < 7 { 3 } else { c as u32 - 3 })?;
                self.pt_len[i] = c;
                i += 1;
                if i == i_special as usize {
                    let c = self.getbits(2)?;
                    for _ in 0..c {
                        self.pt_len[i] = 0;
                        i += 1;
                    }
                }
            }
            while i < nn {
                self.pt_len[i] = 0;
                i += 1;
            }
            self.make_table(nn, &self.pt_len, 8, &mut self.pt_table)?;
        }
        Ok(())
    }

    fn read_c_len(&mut self) -> std::io::Result<()> {
        let n = self.getbits(CBIT)? as usize;
        if n == 0 {
            let c = self.getbits(CBIT)? as u8;
            for item in self.c_len.iter_mut().take(NC) {
                *item = 0;
            }
            for item in self.c_table.iter_mut() {
                *item = c as u16;
            }
        } else {
            let mut i = 0;
            while i < n {
                let mut c = self.pt_table[(self.bitbuf >> (BITBUFSIZ - 8)) as usize] as usize;
                if c >= NT {
                    let mut mask = 1 << (BITBUFSIZ - 1 - 8);
                    loop {
                        if self.bitbuf & mask != 0 {
                            c = self.right[c] as usize;
                        } else {
                            c = self.left[c] as usize;
                        }
                        mask >>= 1;
                        if c < NT {
                            break;
                        }
                    }
                }
                self.fillbuf(self.pt_len[c] as u32)?;
                if c <= 2 {
                    c = match c {
                        0 => 1,
                        1 => self.getbits(4)? as usize + 3,
                        _ => self.getbits(CBIT)? as usize + 20,
                    };
                    for _ in 0..c {
                        self.c_len[i] = 0;
                        i += 1;
                    }
                } else {
                    self.c_len[i] = (c - 2) as u8;
                    i += 1;
                }
            }
            while i < NC {
                self.c_len[i] = 0;
                i += 1;
            }
            self.make_table(NC, &self.c_len, 12, &mut self.c_table)?;
        }
        Ok(())
    }

    fn decode_c(&mut self) -> std::io::Result<usize> {
        if self.blocksize == 0 {
            self.blocksize = self.getbits(16)?;
            if self.blocksize == 0 {
                return Ok(NC);
            }
            self.read_pt_len(NT, TBIT, 3)?;
            self.read_c_len()?;
            self.read_pt_len(NP, PBIT, -1)?;
        }
        self.blocksize -= 1;
        let mut j = self.c_table[(self.bitbuf >> (BITBUFSIZ - 12)) as usize] as usize;
        if j >= NC {
            let mut mask = 1 << (BITBUFSIZ - 1 - 12);
            loop {
                if self.bitbuf & mask != 0 {
                    j = self.right[j] as usize;
                } else {
                    j = self.left[j] as usize;
                }
                mask >>= 1;
                if j < NC {
                    break;
                }
            }
        }
        self.fillbuf(self.c_len[j] as u32)?;
        Ok(j)
    }

    fn decode_p(&mut self) -> std::io::Result<usize> {
        let mut j = self.pt_table[(self.bitbuf >> (BITBUFSIZ - 8)) as usize] as usize;
        if j >= NP {
            let mut mask = 1 << (BITBUFSIZ - 1 - 8);
            loop {
                if self.bitbuf & mask != 0 {
                    j = self.right[j] as usize;
                } else {
                    j = self.left[j] as usize;
                }
                mask >>= 1;
                if j < NP {
                    break;
                }
            }
        }
        self.fillbuf(self.pt_len[j] as u32)?;
        if j != 0 {
            j = (1 << (j - 1)) + self.getbits((j - 1) as u32)? as usize;
        }
        Ok(j)
    }

    fn huf_decode_start(&mut self) -> std::io::Result<()> {
        self.init_getbits()?;
        self.blocksize = 0;
        Ok(())
    }

    fn decode_start(&mut self) -> std::io::Result<()> {
        self.huf_decode_start()?;
        self.j = 0;
        self.done = false;
        Ok(())
    }

    fn decode(&mut self, count: usize, buffer: &mut [u8]) -> std::io::Result<usize> {
        let mut r = 0;
        let mut i = 0;
        while self.j > 0 {
            buffer[r] = buffer[i];
            i = (i + 1) & (DICSIZ - 1);
            self.j -= 1;
            r += 1;
            if r == count {
                return Ok(r);
            }
        }

        loop {
            let c = self.decode_c()?;
            if c == NC {
                self.done = true;
                return Ok(r);
            }
            if c <= UCHAR_MAX as usize {
                buffer[r] = c as u8;
                r += 1;
                if r == count {
                    return Ok(r);
                }
            } else {
                self.j = c - (UCHAR_MAX as usize + 1 - THRESHOLD);
                i = (r - self.decode_p()? - 1) & (DICSIZ - 1);
                while self.j > 0 {
                    buffer[r] = buffer[i];
                    i = (i + 1) & (DICSIZ - 1);
                    self.j -= 1;
                    r += 1;
                    if r == count {
                        return Ok(r);
                    }
                }
            }
        }
    }
}

pub fn unlzh<R: Read, W: Write>(mut input: R, mut output: W) -> std::io::Result<()> {
    let mut decoder = LzhDecoder::new(&mut input);
    decoder.decode_start()?;
    while !decoder.done {
        let n = decoder.decode(DICSIZ, &mut decoder.window)?;
        if n > 0 {
            output.write_all(&decoder.window[..n])?;
        }
    }
    Ok(())
}