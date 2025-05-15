use std::io::{self, Write};

/// Output buffer for bit strings
struct BitBuffer {
    buf: u16,
    valid_bits: usize,
    #[cfg(debug_assertions)]
    bits_sent: u64,
}

impl BitBuffer {
    fn new() -> Self {
        BitBuffer {
            buf: 0,
            valid_bits: 0,
            #[cfg(debug_assertions)]
            bits_sent: 0,
        }
    }

    /// Initialize the bit string routines
    fn init(&mut self) {
        self.buf = 0;
        self.valid_bits = 0;
        #[cfg(debug_assertions)]
        {
            self.bits_sent = 0;
        }
    }

    /// Send a value on a given number of bits.
    /// # Panics
    /// In debug mode, panics if length > 15
    fn send_bits(&mut self, value: u16, length: usize, writer: &mut impl Write) -> io::Result<()> {
        debug_assert!(length > 0 && length <= 15, "invalid length");
        #[cfg(debug_assertions)]
        {
            self.bits_sent += length as u64;
        }

        const BUF_SIZE: usize = 16; // 16 bits in u16

        if self.valid_bits > BUF_SIZE - length {
            self.buf |= value << self.valid_bits;
            writer.write_all(&self.buf.to_le_bytes())?;
            self.buf = value >> (BUF_SIZE - self.valid_bits);
            self.valid_bits += length - BUF_SIZE;
        } else {
            self.buf |= value << self.valid_bits;
            self.valid_bits += length;
        }
        Ok(())
    }

    /// Write out any remaining bits in an incomplete byte
    fn windup(&mut self, writer: &mut impl Write) -> io::Result<()> {
        if self.valid_bits > 8 {
            writer.write_all(&self.buf.to_le_bytes())?;
        } else if self.valid_bits > 0 {
            writer.write_all(&[self.buf as u8])?;
        }
        self.buf = 0;
        self.valid_bits = 0;
        #[cfg(debug_assertions)]
        {
            self.bits_sent = (self.bits_sent + 7) & !7;
        }
        Ok(())
    }
}

/// Reverse the first len bits of a code
/// # Panics
/// In debug mode, panics if len == 0 or len > 15
fn bi_reverse(mut code: u16, mut len: usize) -> u16 {
    debug_assert!(len > 0 && len <= 15, "invalid length");
    let mut res = 0;
    loop {
        res |= code & 1;
        code >>= 1;
        len -= 1;
        if len == 0 {
            break;
        }
        res <<= 1;
    }
    res
}

/// Copy a stored block to the writer, storing first the length and its one's complement if requested
fn copy_block(
    buf: &[u8],
    header: bool,
    bit_buffer: &mut BitBuffer,
    writer: &mut impl Write,
) -> io::Result<()> {
    bit_buffer.windup(writer)?;

    if header {
        let len = buf.len() as u16;
        writer.write_all(&len.to_le_bytes())?;
        writer.write_all(&(!len).to_le_bytes())?;
        #[cfg(debug_assertions)]
        {
            bit_buffer.bits_sent += 2 * 16;
        }
    }

    #[cfg(debug_assertions)]
    {
        bit_buffer.bits_sent += (buf.len() as u64) << 3;
    }

    writer.write_all(buf)?;
    Ok(())
}