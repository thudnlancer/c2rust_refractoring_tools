/*
 * Implement Heap sort -- direct and indirect sorting
 * Based on descriptions in Sedgewick "Algorithms in C"
 *
 * Copyright (C) 1999  Thomas Walter
 *
 * 18 February 2000: Modified for GSL by Brian Gough
 *
 * This is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the
 * Free Software Foundation; either version 3, or (at your option) any
 * later version.
 *
 * This source is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
 * for more details.
 */

use std::cmp::Ordering;

pub trait Sortable: PartialOrd + Copy {}
impl<T: PartialOrd + Copy> Sortable for T {}

pub fn sort<T: Sortable>(data: &mut [T]) {
    heap_sort(data);
}

pub fn sort_indirect<T: Sortable>(data: &[T], indices: &mut [usize]) {
    heap_sort_indirect(data, indices);
}

fn heap_sort<T: Sortable>(data: &mut [T]) {
    let n = data.len();
    if n == 0 {
        return;
    }

    // Build heap
    for i in (0..n / 2).rev() {
        heap_sift_down(data, i, n - 1);
    }

    // Sort heap
    for i in (1..n).rev() {
        data.swap(0, i);
        heap_sift_down(data, 0, i - 1);
    }
}

fn heap_sort_indirect<T: Sortable>(data: &[T], indices: &mut [usize]) {
    let n = indices.len();
    if n == 0 {
        return;
    }

    // Initialize indices
    for (i, idx) in indices.iter_mut().enumerate() {
        *idx = i;
    }

    // Build heap
    for i in (0..n / 2).rev() {
        heap_sift_down_indirect(data, indices, i, n - 1);
    }

    // Sort heap
    for i in (1..n).rev() {
        indices.swap(0, i);
        heap_sift_down_indirect(data, indices, 0, i - 1);
    }
}

fn heap_sift_down<T: Sortable>(data: &mut [T], root: usize, bottom: usize) {
    let mut root = root;
    loop {
        let mut child = root * 2 + 1;
        if child > bottom {
            break;
        }

        if child < bottom && data[child + 1] > data[child] {
            child += 1;
        }

        if data[root] >= data[child] {
            break;
        }

        data.swap(root, child);
        root = child;
    }
}

fn heap_sift_down_indirect<T: Sortable>(data: &[T], indices: &mut [usize], root: usize, bottom: usize) {
    let mut root = root;
    loop {
        let mut child = root * 2 + 1;
        if child > bottom {
            break;
        }

        if child < bottom && data[indices[child + 1]] > data[indices[child]] {
            child += 1;
        }

        if data[indices[root]] >= data[indices[child]] {
            break;
        }

        indices.swap(root, child);
        root = child;
    }
}

// Generic implementations for different numeric types
macro_rules! impl_sort {
    ($($t:ty),*) => {
        $(
            impl Sortable for $t {}
        )*
    };
}

impl_sort!(f64, f32, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);