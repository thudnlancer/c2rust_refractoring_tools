/* See LICENSE file for copyright and license details. */
use std::num::TryFromIntError;

#[derive(Debug)]
pub struct Z;

static mut LIBZAHL_TMP_MODSQR: Z = Z;

pub fn zmodsqr(a: &mut Z, b: &Z, c: &Z) -> Result<(), TryFromIntError> {
    /* TODO What is the fastest way to do zmodsqr? */
    if std::ptr::eq(a, c) {
        unsafe {
            libzahl_tmp_modsqr = c.clone();
            zsqr(a, b)?;
            zmod(a, a, &libzahl_tmp_modsqr)?;
        }
    } else {
        zsqr(a, b)?;
        zmod(a, a, c)?;
    }
    Ok(())
}

fn zsqr(_a: &mut Z, _b: &Z) -> Result<(), TryFromIntError> {
    // Implementation of zsqr
    Ok(())
}

fn zmod(_a: &mut Z, _b: &Z, _c: &Z) -> Result<(), TryFromIntError> {
    // Implementation of zmod
    Ok(())
}

impl Clone for Z {
    fn clone(&self) -> Self {
        Z
    }
}