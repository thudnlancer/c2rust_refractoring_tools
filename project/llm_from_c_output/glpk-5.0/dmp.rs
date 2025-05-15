use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

const DMP_BLK_SIZE: usize = 8000;
const PREFIX_SIZE: usize = ((std::mem::size_of::<Prefix>() + 7) & !7);

#[derive(Debug)]
pub struct Dmp {
    avail: [Option<*mut u8>; 32],
    block: Option<*mut u8>,
    used: usize,
    count: usize,
}

#[derive(Debug)]
struct Prefix {
    pool: *const Dmp,
    size: usize,
}

pub static mut DMP_DEBUG: bool = false;

impl Dmp {
    pub fn create_pool() -> Self {
        unsafe {
            if DMP_DEBUG {
                println!("dmp_create_pool: warning: debug mode is on");
            }
        }

        Dmp {
            avail: [None; 32],
            block: None,
            used: DMP_BLK_SIZE,
            count: 0,
        }
    }

    pub fn get_atom(&mut self, size: usize) -> *mut u8 {
        assert!(1 <= size && size <= 256);
        let need = (size + 7) & !7;
        let k = (need >> 3) - 1;

        let atom = if self.avail[k].is_none() {
            let mut actual_need = need;
            unsafe {
                if DMP_DEBUG {
                    actual_need += PREFIX_SIZE;
                }
            }

            if self.used + actual_need > DMP_BLK_SIZE {
                let layout = Layout::from_size_align(DMP_BLK_SIZE, 8).unwrap();
                let block = unsafe { alloc(layout) };
                if block.is_null() {
                    panic!("Failed to allocate memory block");
                }
                unsafe {
                    ptr::write(block as *mut *mut u8, self.block.unwrap_or(ptr::null_mut()));
                }
                self.block = Some(block);
                self.used = 8;
            }

            let atom = unsafe { (self.block.unwrap() as *mut u8).add(self.used) };
            self.used += actual_need;
            atom
        } else {
            let atom = self.avail[k].unwrap();
            self.avail[k] = unsafe { ptr::read(atom as *const *mut u8) };
            atom
        };

        unsafe {
            if DMP_DEBUG {
                let prefix = atom as *mut Prefix;
                ptr::write(prefix, Prefix {
                    pool: self as *const Dmp,
                    size,
                });
                atom.add(PREFIX_SIZE)
            } else {
                atom
            }
        }
    }

    pub fn free_atom(&mut self, atom: *mut u8, size: usize) {
        assert!(1 <= size && size <= 256);
        let k = ((size + 7) >> 3) - 1;

        let atom = unsafe {
            if DMP_DEBUG {
                let atom = atom.sub(PREFIX_SIZE);
                let prefix = atom as *const Prefix;
                assert_eq!((*prefix).pool as *const Dmp, self as *const Dmp);
                assert_eq!((*prefix).size, size);
                atom
            } else {
                atom
            }
        };

        unsafe {
            ptr::write(atom as *mut *mut u8, self.avail[k].unwrap_or(ptr::null_mut()));
        }
        self.avail[k] = Some(atom);
        assert!(self.count > 0);
        self.count -= 1;
    }

    pub fn in_use(&self) -> usize {
        self.count
    }
}

impl Drop for Dmp {
    fn drop(&mut self) {
        let mut block = self.block;
        while let Some(current_block) = block {
            unsafe {
                let next_block = ptr::read(current_block as *const *mut u8);
                let layout = Layout::from_size_align(DMP_BLK_SIZE, 8).unwrap();
                dealloc(current_block, layout);
                block = if next_block.is_null() {
                    None
                } else {
                    Some(next_block)
                };
            }
        }
    }
}

#[macro_export]
macro_rules! dmp_talloc {
    ($pool:expr, $type:ty) => {
        $pool.get_atom(std::mem::size_of::<$type>()) as *mut $type
    };
}

#[macro_export]
macro_rules! dmp_tfree {
    ($pool:expr, $atom:expr) => {
        $pool.free_atom($atom as *mut u8, std::mem::size_of_val(&*$atom))
    };
}