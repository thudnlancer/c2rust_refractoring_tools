// This file is part of GNU cflow.
// Copyright (C) 2008, 2009, 2010 Sergey Poznyakoff
//
// GNU cflow is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3, or (at your option)
// any later version.
//
// GNU cflow is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with GNU cflow.  If not, see <http://www.gnu.org/licenses/>.

use std::alloc::{alloc_zeroed, Layout};
use std::ptr;

const CHAR_BIT: usize = 8;
const BITS_PER_WORD: usize = std::mem::size_of::<usize>() * CHAR_BIT;

fn wordsize(n: usize) -> usize {
    (n + BITS_PER_WORD - 1) / BITS_PER_WORD
}

fn setbit(x: &mut [usize], i: usize) {
    x[i / BITS_PER_WORD] |= 1 << (i % BITS_PER_WORD);
}

fn resetbit(x: &mut [usize], i: usize) {
    x[i / BITS_PER_WORD] &= !(1 << (i % BITS_PER_WORD));
}

fn bitisset(x: &[usize], i: usize) -> bool {
    (x[i / BITS_PER_WORD] & (1 << (i % BITS_PER_WORD))) != 0
}

fn transitive_closure(r: &mut [usize], n: usize) {
    let rowsize = wordsize(n) * std::mem::size_of::<usize>();
    let relend = unsafe { r.as_ptr().add(n * rowsize / std::mem::size_of::<usize>()) };

    let mut cword = r.as_mut_ptr();
    let mut mask = 1usize;
    let mut rowi = r.as_mut_ptr();

    while rowi < relend {
        let mut ccol = cword;
        let mut rowj = r.as_mut_ptr();

        while rowj < relend {
            if unsafe { *ccol } & mask != 0 {
                let mut rp = rowi;
                let rend = unsafe { rowj.add(rowsize / std::mem::size_of::<usize>()) };

                while rowj < rend {
                    unsafe {
                        *rowj |= *rp;
                        rowj = rowj.add(1);
                        rp = rp.add(1);
                    }
                }
            } else {
                unsafe {
                    rowj = rowj.add(rowsize / std::mem::size_of::<usize>());
                }
            }

            unsafe {
                ccol = ccol.add(rowsize / std::mem::size_of::<usize>());
            }
        }

        mask <<= 1;
        if mask == 0 {
            mask = 1;
            unsafe {
                cword = cword.add(1);
            }
        }
        unsafe {
            rowi = rowi.add(rowsize / std::mem::size_of::<usize>());
        }
    }
}

pub struct CflowDepmap {
    nrows: usize,
    rowlen: usize,
    r: Vec<usize>,
}

impl CflowDepmap {
    pub fn new(count: usize) -> Self {
        let size = wordsize(count);
        let total_size = count * size;
        let r = vec![0; total_size];

        CflowDepmap {
            nrows: count,
            rowlen: size,
            r,
        }
    }

    fn row_ptr(&self, row: usize) -> &[usize] {
        let start = self.rowlen * row;
        &self.r[start..start + self.rowlen]
    }

    fn row_ptr_mut(&mut self, row: usize) -> &mut [usize] {
        let start = self.rowlen * row;
        &mut self.r[start..start + self.rowlen]
    }

    pub fn set(&mut self, row: usize, col: usize) {
        let rptr = self.row_ptr_mut(row);
        setbit(rptr, col);
    }

    pub fn is_set(&self, row: usize, col: usize) -> bool {
        let rptr = self.row_ptr(row);
        bitisset(rptr, col)
    }

    pub fn transitive_closure(&mut self) {
        transitive_closure(&mut self.r, self.nrows);
    }
}