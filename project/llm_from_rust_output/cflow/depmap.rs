use std::alloc::{alloc_zeroed, Layout};
use std::mem;

#[derive(Clone)]
pub struct CflowDepmap {
    nrows: usize,
    rowlen: usize,
    r: Vec<u32>,
}

pub type CflowDepmapRef = Box<CflowDepmap>;

impl CflowDepmap {
    pub fn new(count: usize) -> CflowDepmapRef {
        let rowlen = (count + mem::size_of::<u32>() * 8 - 1) / (mem::size_of::<u32>() * 8);
        let total_size = count * rowlen;
        
        Box::new(CflowDepmap {
            nrows: count,
            rowlen,
            r: vec![0; total_size],
        })
    }

    fn row_ptr(&mut self, row: usize) -> &mut [u32] {
        let start = row * self.rowlen;
        let end = start + self.rowlen;
        &mut self.r[start..end]
    }

    pub fn set(&mut self, row: usize, col: usize) {
        let rptr = self.row_ptr(row);
        let word = col / (mem::size_of::<u32>() * 8);
        let bit = col % (mem::size_of::<u32>() * 8);
        rptr[word] |= 1 << bit;
    }

    pub fn is_set(&self, row: usize, col: usize) -> bool {
        let start = row * self.rowlen;
        let word = col / (mem::size_of::<u32>() * 8);
        let bit = col % (mem::size_of::<u32>() * 8);
        (self.r[start + word] & (1 << bit)) != 0
    }

    pub fn transitive_closure(&mut self) {
        let n = self.nrows;
        let rowsize = self.rowlen;
        let r = &mut self.r;

        for i in 0..n {
            let rowi_start = i * rowsize;
            let rowi = &mut r[rowi_start..rowi_start + rowsize];
            
            for j in 0..n {
                let word = j / (mem::size_of::<u32>() * 8);
                let bit = j % (mem::size_of::<u32>() * 8);
                
                if (rowi[word] & (1 << bit)) != 0 {
                    let rowj_start = j * rowsize;
                    let rowj = &mut r[rowj_start..rowj_start + rowsize];
                    
                    for k in 0..rowsize {
                        rowj[k] |= rowi[k];
                    }
                }
            }
        }
    }
}