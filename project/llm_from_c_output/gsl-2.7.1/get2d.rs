/* histogram/get2d.rs
 * 
 * Copyright (C) 1996, 1997, 1998, 1999, 2000, 2007 Brian Gough
 * 
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or (at
 * your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.
 */

pub struct Histogram2D {
    nx: usize,
    ny: usize,
    bin: Vec<f64>,
    xrange: Vec<f64>,
    yrange: Vec<f64>,
}

#[derive(Debug)]
pub enum Error {
    DomainError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::DomainError(msg) => write!(f, "Domain error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

impl Histogram2D {
    pub fn get(&self, i: usize, j: usize) -> Result<f64> {
        if i >= self.nx {
            return Err(Error::DomainError(
                "index i lies outside valid range of 0 .. nx - 1".to_string(),
            ));
        }

        if j >= self.ny {
            return Err(Error::DomainError(
                "index j lies outside valid range of 0 .. ny - 1".to_string(),
            ));
        }

        Ok(self.bin[i * self.ny + j])
    }

    pub fn get_xrange(&self, i: usize) -> Result<(f64, f64)> {
        if i >= self.nx {
            return Err(Error::DomainError(
                "index i lies outside valid range of 0 .. nx - 1".to_string(),
            ));
        }

        Ok((self.xrange[i], self.xrange[i + 1]))
    }

    pub fn get_yrange(&self, j: usize) -> Result<(f64, f64)> {
        if j >= self.ny {
            return Err(Error::DomainError(
                "index j lies outside valid range of 0 .. ny - 1".to_string(),
            ));
        }

        Ok((self.yrange[j], self.yrange[j + 1]))
    }

    pub fn find(&self, x: f64, y: f64) -> Result<(usize, usize)> {
        let i = find(self.nx, &self.xrange, x)
            .map_err(|_| Error::DomainError("x not found in range of h".to_string()))?;
        let j = find(self.ny, &self.yrange, y)
            .map_err(|_| Error::DomainError("y not found in range of h".to_string()))?;
        Ok((i, j))
    }
}

fn find(n: usize, range: &[f64], x: f64) -> Result<usize> {
    if x < range[0] || x >= range[n] {
        return Err(Error::DomainError("value not found in range".to_string()));
    }

    let mut i = 0;
    let mut j = n;

    while j - i > 1 {
        let k = (i + j) / 2;
        if x >= range[k] {
            i = k;
        } else {
            j = k;
        }
    }

    Ok(i)
}