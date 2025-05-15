// sort/subsetind.rs
//
// Copyright (C) 2001, 2007 Brian Gough
//
// This is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the
// Free Software Foundation; either version 3, or (at your option) any
// later version.
//
// This source is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License
// for more details.

use std::cmp::Ordering;

macro_rules! impl_subset_indices {
    ($type:ty) => {
        pub fn subset_indices(
            src: &[$type],
            k: usize,
            dest: &mut [usize],
        ) -> Result<(), &'static str> {
            if k > src.len() {
                return Err("k is larger than source length");
            }
            if dest.len() < k {
                return Err("destination too small");
            }

            let mut indices: Vec<usize> = (0..src.len()).collect();
            indices.sort_by(|&i, &j| {
                src[i]
                    .partial_cmp(&src[j])
                    .unwrap_or_else(|| {
                        if src[i] < src[j] {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    })
            });

            dest[..k].copy_from_slice(&indices[..k]);
            Ok(())
        }
    };
}

mod long_double_impl {
    impl_subset_indices!(f64); // Using f64 as closest Rust equivalent to long double
}

mod double_impl {
    impl_subset_indices!(f64);
}

mod float_impl {
    impl_subset_indices!(f32);
}

mod ulong_impl {
    impl_subset_indices!(u64);
}

mod long_impl {
    impl_subset_indices!(i64);
}

mod uint_impl {
    impl_subset_indices!(u32);
}

mod int_impl {
    impl_subset_indices!(i32);
}

mod ushort_impl {
    impl_subset_indices!(u16);
}

mod short_impl {
    impl_subset_indices!(i16);
}

mod uchar_impl {
    impl_subset_indices!(u8);
}

mod char_impl {
    impl_subset_indices!(i8);
}