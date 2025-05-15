/* rng/types.rs
 * 
 * Copyright (C) 2001, 2007 Brian Gough
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

use std::sync::Once;
use std::ptr;

const N: usize = 100;

static mut GENERATOR_TYPES: [*const gsl_rng_type; N] = [ptr::null(); N];
static INIT: Once = Once::new();

fn add_generator_type(types: &mut [*const gsl_rng_type; N], i: &mut usize, t: *const gsl_rng_type) {
    if *i >= N {
        panic!("Generator types array overflow");
    }
    types[*i] = t;
    *i += 1;
}

pub fn gsl_rng_types_setup() -> &'static [*const gsl_rng_type] {
    INIT.call_once(|| {
        let mut i = 0;
        let types = unsafe { &mut GENERATOR_TYPES };

        add_generator_type(types, &mut i, gsl_rng_borosh13);
        add_generator_type(types, &mut i, gsl_rng_cmrg);
        add_generator_type(types, &mut i, gsl_rng_coveyou);
        add_generator_type(types, &mut i, gsl_rng_fishman18);
        add_generator_type(types, &mut i, gsl_rng_fishman20);
        add_generator_type(types, &mut i, gsl_rng_fishman2x);
        add_generator_type(types, &mut i, gsl_rng_gfsr4);
        add_generator_type(types, &mut i, gsl_rng_knuthran);
        add_generator_type(types, &mut i, gsl_rng_knuthran2);
        add_generator_type(types, &mut i, gsl_rng_knuthran2002);
        add_generator_type(types, &mut i, gsl_rng_lecuyer21);
        add_generator_type(types, &mut i, gsl_rng_minstd);
        add_generator_type(types, &mut i, gsl_rng_mrg);
        add_generator_type(types, &mut i, gsl_rng_mt19937);
        add_generator_type(types, &mut i, gsl_rng_mt19937_1999);
        add_generator_type(types, &mut i, gsl_rng_mt19937_1998);
        add_generator_type(types, &mut i, gsl_rng_r250);
        add_generator_type(types, &mut i, gsl_rng_ran0);
        add_generator_type(types, &mut i, gsl_rng_ran1);
        add_generator_type(types, &mut i, gsl_rng_ran2);
        add_generator_type(types, &mut i, gsl_rng_ran3);
        add_generator_type(types, &mut i, gsl_rng_rand);
        add_generator_type(types, &mut i, gsl_rng_rand48);
        add_generator_type(types, &mut i, gsl_rng_random128_bsd);
        add_generator_type(types, &mut i, gsl_rng_random128_glibc2);
        add_generator_type(types, &mut i, gsl_rng_random128_libc5);
        add_generator_type(types, &mut i, gsl_rng_random256_bsd);
        add_generator_type(types, &mut i, gsl_rng_random256_glibc2);
        add_generator_type(types, &mut i, gsl_rng_random256_libc5);
        add_generator_type(types, &mut i, gsl_rng_random32_bsd);
        add_generator_type(types, &mut i, gsl_rng_random32_glibc2);
        add_generator_type(types, &mut i, gsl_rng_random32_libc5);
        add_generator_type(types, &mut i, gsl_rng_random64_bsd);
        add_generator_type(types, &mut i, gsl_rng_random64_glibc2);
        add_generator_type(types, &mut i, gsl_rng_random64_libc5);
        add_generator_type(types, &mut i, gsl_rng_random8_bsd);
        add_generator_type(types, &mut i, gsl_rng_random8_glibc2);
        add_generator_type(types, &mut i, gsl_rng_random8_libc5);
        add_generator_type(types, &mut i, gsl_rng_random_bsd);
        add_generator_type(types, &mut i, gsl_rng_random_glibc2);
        add_generator_type(types, &mut i, gsl_rng_random_libc5);
        add_generator_type(types, &mut i, gsl_rng_randu);
        add_generator_type(types, &mut i, gsl_rng_ranf);
        add_generator_type(types, &mut i, gsl_rng_ranlux);
        add_generator_type(types, &mut i, gsl_rng_ranlux389);
        add_generator_type(types, &mut i, gsl_rng_ranlxd1);
        add_generator_type(types, &mut i, gsl_rng_ranlxd2);
        add_generator_type(types, &mut i, gsl_rng_ranlxs0);
        add_generator_type(types, &mut i, gsl_rng_ranlxs1);
        add_generator_type(types, &mut i, gsl_rng_ranlxs2);
        add_generator_type(types, &mut i, gsl_rng_ranmar);
        add_generator_type(types, &mut i, gsl_rng_slatec);
        add_generator_type(types, &mut i, gsl_rng_taus);
        add_generator_type(types, &mut i, gsl_rng_taus2);
        add_generator_type(types, &mut i, gsl_rng_taus113);
        add_generator_type(types, &mut i, gsl_rng_transputer);
        add_generator_type(types, &mut i, gsl_rng_tt800);
        add_generator_type(types, &mut i, gsl_rng_uni);
        add_generator_type(types, &mut i, gsl_rng_uni32);
        add_generator_type(types, &mut i, gsl_rng_vax);
        add_generator_type(types, &mut i, gsl_rng_waterman14);
        add_generator_type(types, &mut i, gsl_rng_zuf);
        add_generator_type(types, &mut i, ptr::null());
    });

    unsafe { &GENERATOR_TYPES[..] }
}

// Placeholder types - these would be defined elsewhere in the Rust GSL bindings
#[allow(non_camel_case_types)]
pub struct gsl_rng_type;
extern {
    // These would be linked to the actual GSL library implementations
    static gsl_rng_borosh13: gsl_rng_type;
    static gsl_rng_cmrg: gsl_rng_type;
    static gsl_rng_coveyou: gsl_rng_type;
    static gsl_rng_fishman18: gsl_rng_type;
    static gsl_rng_fishman20: gsl_rng_type;
    static gsl_rng_fishman2x: gsl_rng_type;
    static gsl_rng_gfsr4: gsl_rng_type;
    static gsl_rng_knuthran: gsl_rng_type;
    static gsl_rng_knuthran2: gsl_rng_type;
    static gsl_rng_knuthran2002: gsl_rng_type;
    static gsl_rng_lecuyer21: gsl_rng_type;
    static gsl_rng_minstd: gsl_rng_type;
    static gsl_rng_mrg: gsl_rng_type;
    static gsl_rng_mt19937: gsl_rng_type;
    static gsl_rng_mt19937_1999: gsl_rng_type;
    static gsl_rng_mt19937_1998: gsl_rng_type;
    static gsl_rng_r250: gsl_rng_type;
    static gsl_rng_ran0: gsl_rng_type;
    static gsl_rng_ran1: gsl_rng_type;
    static gsl_rng_ran2: gsl_rng_type;
    static gsl_rng_ran3: gsl_rng_type;
    static gsl_rng_rand: gsl_rng_type;
    static gsl_rng_rand48: gsl_rng_type;
    static gsl_rng_random128_bsd: gsl_rng_type;
    static gsl_rng_random128_glibc2: gsl_rng_type;
    static gsl_rng_random128_libc5: gsl_rng_type;
    static gsl_rng_random256_bsd: gsl_rng_type;
    static gsl_rng_random256_glibc2: gsl_rng_type;
    static gsl_rng_random256_libc5: gsl_rng_type;
    static gsl_rng_random32_bsd: gsl_rng_type;
    static gsl_rng_random32_glibc2: gsl_rng_type;
    static gsl_rng_random32_libc5: gsl_rng_type;
    static gsl_rng_random64_bsd: gsl_rng_type;
    static gsl_rng_random64_glibc2: gsl_rng_type;
    static gsl_rng_random64_libc5: gsl_rng_type;
    static gsl_rng_random8_bsd: gsl_rng_type;
    static gsl_rng_random8_glibc2: gsl_rng_type;
    static gsl_rng_random8_libc5: gsl_rng_type;
    static gsl_rng_random_bsd: gsl_rng_type;
    static gsl_rng_random_glibc2: gsl_rng_type;
    static gsl_rng_random_libc5: gsl_rng_type;
    static gsl_rng_randu: gsl_rng_type;
    static gsl_rng_ranf: gsl_rng_type;
    static gsl_rng_ranlux: gsl_rng_type;
    static gsl_rng_ranlux389: gsl_rng_type;
    static gsl_rng_ranlxd1: gsl_rng_type;
    static gsl_rng_ranlxd2: gsl_rng_type;
    static gsl_rng_ranlxs0: gsl_rng_type;
    static gsl_rng_ranlxs1: gsl_rng_type;
    static gsl_rng_ranlxs2: gsl_rng_type;
    static gsl_rng_ranmar: gsl_rng_type;
    static gsl_rng_slatec: gsl_rng_type;
    static gsl_rng_taus: gsl_rng_type;
    static gsl_rng_taus2: gsl_rng_type;
    static gsl_rng_taus113: gsl_rng_type;
    static gsl_rng_transputer: gsl_rng_type;
    static gsl_rng_tt800: gsl_rng_type;
    static gsl_rng_uni: gsl_rng_type;
    static gsl_rng_uni32: gsl_rng_type;
    static gsl_rng_vax: gsl_rng_type;
    static gsl_rng_waterman14: gsl_rng_type;
    static gsl_rng_zuf: gsl_rng_type;
}