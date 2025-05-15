/* wavelet/haar.rs
 * 
 * Copyright (C) 2004 Ivo Alxneit
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

use std::f64::consts::FRAC_1_SQRT_2;

const CH_2: [f64; 2] = [FRAC_1_SQRT_2, FRAC_1_SQRT_2];
const CG_2: [f64; 2] = [FRAC_1_SQRT_2, -FRAC_1_SQRT_2];

#[derive(Debug, PartialEq)]
enum WaveletResult {
    Success,
    Failure,
}

fn haar_init(
    member: usize,
) -> Result<(&'static [f64], &'static [f64], &'static [f64], &'static [f64], usize, usize), WaveletResult> {
    if member != 2 {
        return Err(WaveletResult::Failure);
    }

    Ok((&CH_2, &CG_2, &CH_2, &CG_2, 2, 0))
}

fn haar_centered_init(
    member: usize,
) -> Result<(&'static [f64], &'static [f64], &'static [f64], &'static [f64], usize, usize), WaveletResult> {
    if member != 2 {
        return Err(WaveletResult::Failure);
    }

    Ok((&CH_2, &CG_2, &CH_2, &CG_2, 2, 1))
}

struct WaveletType {
    name: &'static str,
    init_fn: fn(usize) -> Result<(&'static [f64], &'static [f64], &'static [f64], &'static [f64], usize, usize), WaveletResult>,
}

static HAAR_TYPE: WaveletType = WaveletType {
    name: "haar",
    init_fn: haar_init,
};

static HAAR_CENTERED_TYPE: WaveletType = WaveletType {
    name: "haar-centered",
    init_fn: haar_centered_init,
};

pub static GSL_WAVELET_HAAR: &WaveletType = &HAAR_TYPE;
pub static GSL_WAVELET_HAAR_CENTERED: &WaveletType = &HAAR_CENTERED_TYPE;