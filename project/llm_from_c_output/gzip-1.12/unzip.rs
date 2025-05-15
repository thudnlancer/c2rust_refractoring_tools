use std::io::{self, Read, Write};
use std::fs::File;
use std::path::Path;
use crc32fast::Hasher as Crc32;

const LOCSIG: u32 = 0x04034b50;      // four-byte lead-in (lsb first)
const LOCFLG: usize = 6;             // offset of bit flag
const CRPFLG: u8 = 1;                // bit for encrypted entry
const EXTFLG: u8 = 8;                // bit for extended local header
const LOCHOW: usize = 8;             // offset of compression method
const LOCCRC: usize = 14;            // offset of crc
const LOCSIZ: usize = 18;            // offset of compressed size
const LOCLEN: usize = 22;            // offset of uncompressed length
const LOCFIL: usize = 26;            // offset of file name field length
const LOCEXT: usize = 28;            // offset of extra field length
const LOCHDR: usize = 30;            // size of local header, including sig
const EXTHDR: usize = 16;            // size of extended local header, inc sig
const RAND_HEAD_LEN: usize = 12;     // length of encryption random header

struct UnzipState {
    unzip_crc: u32,
    decrypt: bool,
    pkzip: bool,
    ext_header: bool,
    method: u8,
    inptr: usize,
    insize: usize,
    inbuf: Vec<u8>,
    outbuf: Vec<u8>,
    bytes_out: usize,
    crc32: Crc32,
}

impl UnzipState {
    fn new() -> Self {
        Self {
            unzip_crc: 0,
            decrypt: false,
            pkzip: false,
            ext_header: false,
            method: 0,
            inptr: 0,
            insize: 0,
            inbuf: Vec::new(),
            outbuf: Vec::new(),
            bytes_out: 0,
            crc32: Crc32::new(),
        }
    }

    fn check_zipfile(&mut self, input: &mut impl Read) -> io::Result<()> {
        self.inbuf.resize(LOCHDR, 0);
        input.read_exact(&mut self.inbuf)?;
        self.insize = self.inbuf.len();

        let h = &self.inbuf;
        
        // Check validity of local header, and skip name and extra fields
        let name_len = u16::from_le_bytes([h[LOCFIL], h[LOCFIL + 1]]);
        let extra_len = u16::from_le_bytes([h[LOCEXT], h[LOCEXT + 1]]);
        self.inptr = LOCHDR + name_len as usize + extra_len as usize;

        if self.inptr > self.insize || u32::from_le_bytes([h[0], h[1], h[2], h[3]]) != LOCSIG {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "not a valid zip file"));
        }

        self.method = h[LOCHOW];
        if self.method != 0 && self.method != 8 {  // STORED and DEFLATED
            return Err(io::Error::new(io::ErrorKind::InvalidData, 
                "first entry not deflated or stored -- use unzip"));
        }

        // If entry encrypted, decrypt and validate encryption header
        if (h[LOCFLG] & CRPFLG) != 0 {
            self.decrypt = true;
            return Err(io::Error::new(io::ErrorKind::InvalidData, 
                "encrypted file -- use unzip"));
        }

        // Save flags for unzip()
        self.ext_header = (h[LOCFLG] & EXTFLG) != 0;
        self.pkzip = true;

        Ok(())
    }

    fn unzip(&mut self, input: &mut impl Read, output: &mut impl Write) -> io::Result<()> {
        let mut orig_crc = 0;
        let mut orig_len = 0;
        let mut buf = [0u8; EXTHDR];

        self.crc32.reset();

        if self.pkzip && !self.ext_header {
            orig_crc = u32::from_le_bytes([
                self.inbuf[LOCCRC],
                self.inbuf[LOCCRC + 1],
                self.inbuf[LOCCRC + 2],
                self.inbuf[LOCCRC + 3],
            ]);
            orig_len = u32::from_le_bytes([
                self.inbuf[LOCLEN],
                self.inbuf[LOCLEN + 1],
                self.inbuf[LOCLEN + 2],
                self.inbuf[LOCLEN + 3],
            ]);
        }

        // Decompress
        if self.method == 8 {  // DEFLATED
            // TODO: Implement DEFLATE decompression
            return Err(io::Error::new(io::ErrorKind::Unsupported, "DEFLATE not implemented"));
        } else if self.pkzip && self.method == 0 {  // STORED
            let n = u32::from_le_bytes([
                self.inbuf[LOCLEN],
                self.inbuf[LOCLEN + 1],
                self.inbuf[LOCLEN + 2],
                self.inbuf[LOCLEN + 3],
            ]);
            
            let expected_size = u32::from_le_bytes([
                self.inbuf[LOCSIZ],
                self.inbuf[LOCSIZ + 1],
                self.inbuf[LOCSIZ + 2],
                self.inbuf[LOCSIZ + 3],
            ]);
            
            if n != expected_size - (if self.decrypt { RAND_HEAD_LEN as u32 } else { 0 }) {
                return Err(io::Error::new(io::ErrorKind::InvalidData, 
                    "invalid compressed data--length mismatch"));
            }

            let mut remaining = n as usize;
            while remaining > 0 {
                let mut chunk = vec![0u8; remaining.min(4096)];
                input.read_exact(&mut chunk)?;
                output.write_all(&chunk)?;
                self.crc32.update(&chunk);
                self.bytes_out += chunk.len();
                remaining -= chunk.len();
            }
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, 
                "internal error, invalid method"));
        }

        // Get the crc and original length
        if !self.pkzip {
            input.read_exact(&mut buf[..8])?;
            orig_crc = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
            orig_len = u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]);
        } else if self.ext_header {
            input.read_exact(&mut buf)?;
            orig_crc = u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]);
            orig_len = u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]);
        }

        // Validate decompression
        if orig_crc != self.crc32.finalize() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, 
                "invalid compressed data--crc error"));
        }
        if orig_len != (self.bytes_out as u32) {
            return Err(io::Error::new(io::ErrorKind::InvalidData, 
                "invalid compressed data--length error"));
        }

        // Check if there are more entries in a pkzip file
        if self.pkzip && self.inptr + 4 < self.insize {
            let sig = u32::from_le_bytes([
                self.inbuf[self.inptr],
                self.inbuf[self.inptr + 1],
                self.inbuf[self.inptr + 2],
                self.inbuf[self.inptr + 3],
            ]);
            if sig == LOCSIG {
                // TODO: Handle multiple entries warning
            }
        }

        self.ext_header = false;
        self.pkzip = false;
        self.unzip_crc = orig_crc;

        Ok(())
    }
}