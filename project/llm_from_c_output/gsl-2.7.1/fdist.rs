// randist/fdist.rs
// 
// Copyright (C) 1996, 1997, 1998, 1999, 2000, 2007, 2010 James Theiler, Brian Gough
// 
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or (at
// your option) any later version.
// 
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.

use rand::Rng;
use statrs::function::gamma::ln_gamma;

/// The F distribution has the form
///
/// p(x) dx = (nu1^(nu1/2) nu2^(nu2/2) Gamma((nu1 + nu2)/2) /
/// Gamma(nu1/2) Gamma(nu2/2)) *
/// x^(nu1/2 - 1) (nu2 + nu1 * x)^(-nu1/2 -nu2/2) dx
///
/// The method used here is the one described in Knuth
pub fn ran_fdist<R: Rng>(rng: &mut R, nu1: f64, nu2: f64) -> f64 {
    let y1 = rand_distr::Gamma::new(nu1 / 2.0, 2.0).unwrap().sample(rng);
    let y2 = rand_distr::Gamma::new(nu2 / 2.0, 2.0).unwrap().sample(rng);
    
    (y1 * nu2) / (y2 * nu1)
}

pub fn ran_fdist_pdf(x: f64, nu1: f64, nu2: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else {
        let lglg = (nu1 / 2.0) * nu1.ln() + (nu2 / 2.0) * nu2.ln();
        
        let lg12 = ln_gamma((nu1 + nu2) / 2.0);
        let lg1 = ln_gamma(nu1 / 2.0);
        let lg2 = ln_gamma(nu2 / 2.0);
        
        (lglg + lg12 - lg1 - lg2 + (nu1 / 2.0 - 1.0) * x.ln() 
            - ((nu1 + nu2) / 2.0) * (nu2 + nu1 * x).ln()).exp()
    }
}