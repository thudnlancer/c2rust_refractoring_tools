/* See LICENSE file for copyright and license details. */
use crate::internals::*;
use crate::errors::{ZError, LibZahlError};
use crate::zahl::Zahl;

pub fn zmodpowu(
    a: &mut Zahl,
    b: &Zahl,
    c: u64,
    d: &Zahl,
) -> Result<(), LibZahlError> {
    if c == 0 {
        if b.is_zero() {
            return Err(LibZahlError::new(ZError::Pow0));
        } else if d.is_zero() {
            return Err(LibZahlError::new(ZError::Div0));
        } else {
            a.set_u64(1);
            return Ok(());
        }
    } else if d.is_zero() {
        return Err(LibZahlError::new(ZError::Div0));
    } else if b.is_zero() {
        a.set_zero();
        return Ok(());
    }

    let mut tb = Zahl::new();
    zmod(&mut tb, b, d)?;
    let mut td = d.clone();

    if c & 1 != 0 {
        *a = tb.clone();
    } else {
        a.set_u64(1);
    }

    let mut c = c;
    while {
        c >>= 1;
        c != 0
    } {
        zmodsqr(&mut tb, &tb, &td)?;
        if c & 1 != 0 {
            zmodmul(a, a, &tb, &td)?;
        }
    }

    Ok(())
}