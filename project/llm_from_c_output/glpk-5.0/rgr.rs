use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

const BMP_HEADER_SIZE: u32 = 14;
const BMP_INFO_HEADER_SIZE: u32 = 40;
const COLOR_TABLE_SIZE: u32 = 16 * 4;
const DPI: u32 = 2953; // 75 dpi

fn put_byte<W: Write>(fp: &mut W, c: u8) -> io::Result<()> {
    fp.write_all(&[c])
}

fn put_word<W: Write>(fp: &mut W, w: u16) -> io::Result<()> {
    fp.write_all(&w.to_le_bytes())
}

fn put_dword<W: Write>(fp: &mut W, d: u32) -> io::Result<()> {
    fp.write_all(&d.to_le_bytes())
}

pub fn rgr_write_bmp16(fname: &Path, m: i32, n: i32, map: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    if !(1 <= m && m <= 32767) {
        return Err(format!("rgr_write_bmp16: m = {}; invalid height", m).into());
    }
    if !(1 <= n && n <= 32767) {
        return Err(format!("rgr_write_bmp16: n = {}; invalid width", n).into());
    }

    let mut fp = File::create(fname)?;
    let offset = BMP_HEADER_SIZE + BMP_INFO_HEADER_SIZE + COLOR_TABLE_SIZE;
    let row_size = ((4 * n + 31) / 32) as u32 * 4;
    let bmsize = row_size * m as u32;

    // BMP file header (14 bytes)
    put_byte(&mut fp, b'B')?;
    put_byte(&mut fp, b'M')?;
    put_dword(&mut fp, offset + bmsize)?;
    put_word(&mut fp, 0)?; // reserved1
    put_word(&mut fp, 0)?; // reserved2
    put_dword(&mut fp, offset)?;

    // BMP info header (40 bytes)
    put_dword(&mut fp, BMP_INFO_HEADER_SIZE)?;
    put_dword(&mut fp, n as u32)?;
    put_dword(&mut fp, m as u32)?;
    put_word(&mut fp, 1)?; // planes
    put_word(&mut fp, 4)?; // bit count
    put_dword(&mut fp, 0)?; // compression (BI_RGB)
    put_dword(&mut fp, 0)?; // size image
    put_dword(&mut fp, DPI)?; // x pixels per meter
    put_dword(&mut fp, DPI)?; // y pixels per meter
    put_dword(&mut fp, 0)?; // colors used
    put_dword(&mut fp, 0)?; // colors important

    // Color table (16 * 4 = 64 bytes)
    let colors = [
        0x000000, // black
        0x000080, // blue
        0x008000, // green
        0x008080, // cyan
        0x800000, // red
        0x800080, // magenta
        0x808000, // brown
        0xC0C0C0, // light gray
        0x808080, // dark gray
        0x0000FF, // bright blue
        0x00FF00, // bright green
        0x00FFFF, // bright cyan
        0xFF0000, // bright red
        0xFF00FF, // bright magenta
        0xFFFF00, // yellow
        0xFFFFFF, // white
    ];

    for &color in &colors {
        put_dword(&mut fp, color)?;
    }

    // Pixel data
    let mut b = 0u8;
    for i in (0..m).rev() {
        for j in 0..((n + 7) / 8) * 8 {
            b <<= 4;
            b |= if j < n { map[(i * n + j) as usize] & 0x0F } else { 0 };
            if j & 1 != 0 {
                put_byte(&mut fp, b)?;
            }
        }
    }

    fp.flush()?;
    Ok(())
}