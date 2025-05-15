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

fn heap_sort<T: Sortable>(data: &mut [T]) {
    let n = data.len();
    if n == 0 {
        return;
    }

    // Build heap
    for i in (0..n / 2).rev() {
        sift_down(data, i, n - 1);
    }

    // Sort heap
    for i in (1..n).rev() {
        data.swap(0, i);
        sift_down(data, 0, i - 1);
    }
}

fn sift_down<T: Sortable>(data: &mut [T], root: usize, bottom: usize) {
    let mut root = root;
    loop {
        let mut child = root * 2 + 1;
        if child > bottom {
            break;
        }

        if child < bottom && data[child] < data[child + 1] {
            child += 1;
        }

        if data[root] >= data[child] {
            break;
        }

        data.swap(root, child);
        root = child;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut data = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
        sort(&mut data);
        assert_eq!(data, [1, 1, 2, 3, 3, 4, 5, 5, 6, 9]);

        let mut data = ['d', 'a', 'c', 'b'];
        sort(&mut data);
        assert_eq!(data, ['a', 'b', 'c', 'd']);

        let mut data: [i32; 0] = [];
        sort(&mut data);
        assert_eq!(data, []);

        let mut data = [5.3, 1.2, 8.4, 3.7];
        sort(&mut data);
        assert_eq!(data, [1.2, 3.7, 5.3, 8.4]);
    }
}