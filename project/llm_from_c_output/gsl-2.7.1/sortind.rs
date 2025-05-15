/*
 * Implement Heap sort -- direct and indirect sorting
 * Based on descriptions in Sedgewick "Algorithms in C"
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

type ComparisonFn<T> = fn(a: &T, b: &T) -> Ordering;

fn downheap<T>(
    p: &mut [usize],
    data: &[T],
    n: usize,
    k: usize,
    compare: ComparisonFn<T>,
) {
    let pki = p[k];
    let mut k = k;

    while k <= n / 2 {
        let mut j = 2 * k;

        if j < n && compare(&data[p[j]], &data[p[j + 1]]) == Ordering::Less {
            j += 1;
        }

        if compare(&data[pki], &data[p[j]]) != Ordering::Less {
            break;
        }

        p[k] = p[j];
        k = j;
    }

    p[k] = pki;
}

pub fn heapsort_index<T>(
    p: &mut [usize],
    data: &[T],
    compare: ComparisonFn<T>,
) -> Result<(), &'static str> {
    /* Sort the array in ascending order. This is a true inplace
       algorithm with N log N operations. Worst case (an already sorted
       array) is something like 20% slower */

    let count = data.len();

    if count == 0 {
        return Ok(()); // No data to sort
    }

    if p.len() < count {
        return Err("Permutation array too small");
    }

    for (i, item) in p.iter_mut().take(count).enumerate() {
        *item = i; // set permutation to identity
    }

    /* We have n_data elements, last element is at 'n_data-1', first at
       '0' Set N to the last element number. */

    let mut n = count - 1;

    let mut k = n / 2;
    k += 1; // Compensate the first use of 'k--'
    loop {
        k -= 1;
        downheap(p, data, n, k, compare);
        if k == 0 {
            break;
        }
    }

    while n > 0 {
        // first swap the elements
        p.swap(0, n);

        // then process the heap
        n -= 1;

        downheap(p, data, n, 0, compare);
    }

    Ok(())
}