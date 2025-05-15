use std::fs::File;
use std::io::{Write, Error, ErrorKind};
use std::path::Path;

fn put_byte<W: Write>(fp: &mut W, c: u8) -> Result<(), Error> {
    fp.write_all(&[c])
}

fn put_word<W: Write>(fp: &mut W, w: u16) -> Result<(), Error> {
    fp.write_all(&w.to_le_bytes())
}

fn put_dword<W: Write>(fp: &mut W, d: u32) -> Result<(), Error> {
    fp.write_all(&d.to_le_bytes())
}

pub fn _glp_rgr_write_bmp16(
    fname: &Path,
    m: i32,
    n: i32,
    map: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    if !(1 <= m && m <= 32767) {
        return Err(Box::new(Error::new(
            ErrorKind::InvalidInput,
            format!("rgr_write_bmp16: m = {}; invalid height", m),
        ));
    }
    if !(1 <= n && n <= 32767) {
        return Err(Box::new(Error::new(
            ErrorKind::InvalidInput,
            format!("rgr_write_bmp16: n = {}; invalid width", n),
        ));
    }

    let mut fp = File::create(fname).map_err(|e| {
        format!("rgr_write_bmp16: unable to create '{}' - {}", 
                fname.display(), e)
    })?;

    let offset = 14 + 40 + 16 * 4;
    let bmsize = (4 * n + 31) / 32;
    let file_size = offset + bmsize * 4;

    put_byte(&mut fp, b'B')?;
    put_byte(&mut fp, b'M')?;
    put_dword(&mut fp, file_size as u32)?;
    put_word(&mut fp, 0)?;
    put_word(&mut fp, 0)?;
    put_dword(&mut fp, offset as u32)?;
    put_dword(&mut fp, 40)?;
    put_dword(&mut fp, n as u32)?;
    put_dword(&mut fp, m as u32)?;
    put_word(&mut fp, 1)?;
    put_word(&mut fp, 4)?;
    put_dword(&mut fp, 0)?;
    put_dword(&mut fp, 0)?;
    put_dword(&mut fp, 2953)?;
    put_dword(&mut fp, 2953)?;
    put_dword(&mut fp, 0)?;
    put_dword(&mut fp, 0)?;
    put_dword(&mut fp, 0)?;
    put_dword(&mut fp, 0x80)?;
    put_dword(&mut fp, 0x8000)?;
    put_dword(&mut fp, 0x8080)?;
    put_dword(&mut fp, 0x800000)?;
    put_dword(&mut fp, 0x800080)?;
    put_dword(&mut fp, 0x808000)?;
    put_dword(&mut fp, 0xc0c0c0)?;
    put_dword(&mut fp, 0x808080)?;
    put_dword(&mut fp, 0xff)?;
    put_dword(&mut fp, 0xff00)?;
    put_dword(&mut fp, 0xffff)?;
    put_dword(&mut fp, 0xff0000)?;
    put_dword(&mut fp, 0xff00ff)?;
    put_dword(&mut fp, 0xffff00)?;
    put_dword(&mut fp, 0xffffff)?;

    let mut b = 0u8;
    for i in (0..m).rev() {
        for j in 0..((n + 7) / 8 * 8) {
            b <<= 4;
            b |= if j < n {
                map[(i * n + j) as usize] & 0x0F
            } else {
                0
            };
            if j & 1 != 0 {
                put_byte(&mut fp, b)?;
            }
        }
    }

    fp.flush()?;

    Ok(())
}