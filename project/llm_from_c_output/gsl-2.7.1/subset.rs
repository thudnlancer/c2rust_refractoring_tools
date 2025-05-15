// sort/subset.rs
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

macro_rules! impl_subset {
    ($type:ty) => {
        pub fn subset<T>(src: &[T], k: usize, dest: &mut [T]) -> Result<(), &'static str>
        where
            T: Clone + PartialOrd,
        {
            if k > src.len() || k > dest.len() {
                return Err("subset size exceeds source or destination length");
            }

            let mut indices: Vec<usize> = (0..src.len()).collect();
            indices.sort_by(|&a, &b| src[a].partial_cmp(&src[b]).unwrap_or(Ordering::Equal));

            for i in 0..k {
                dest[i] = src[indices[i]].clone();
            }

            Ok(())
        }
    };
}

impl_subset!(f64);
impl_subset!(f32);
impl_subset!(i64);
impl_subset!(u64);
impl_subset!(i32);
impl_subset!(u32);
impl_subset!(i16);
impl_subset!(u16);
impl_subset!(i8);
impl_subset!(u8);
impl_subset!(char);