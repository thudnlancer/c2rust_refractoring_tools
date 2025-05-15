use std::ffi::CString;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::ptr;
use std::slice;

pub type ArgvIterErr = u32;
pub const AI_ERR_OK: ArgvIterErr = 1;
pub const AI_ERR_EOF: ArgvIterErr = 2;
pub const AI_ERR_MEM: ArgvIterErr = 3;
pub const AI_ERR_READ: ArgvIterErr = 4;

pub enum ArgvIterator {
    Stream {
        reader: BufReader<File>,
        item_idx: usize,
    },
    Argv {
        args: Vec<CString>,
        current: usize,
    },
}

impl ArgvIterator {
    pub fn init_argv(argv: Vec<CString>) -> Option<Self> {
        Some(ArgvIterator::Argv {
            args: argv,
            current: 0,
        })
    }

    pub fn init_stream(file: File) -> Option<Self> {
        Some(ArgvIterator::Stream {
            reader: BufReader::new(file),
            item_idx: 0,
        })
    }

    pub fn next(&mut self) -> Result<Option<CString>, ArgvIterErr> {
        match self {
            ArgvIterator::Stream { reader, item_idx } => {
                let mut line = String::new();
                match reader.read_line(&mut line) {
                    Ok(0) => Ok(None),
                    Ok(_) => {
                        *item_idx += 1;
                        CString::new(line.trim_end().as_bytes())
                            .map(Some)
                            .map_err(|_| AI_ERR_MEM)
                    }
                    Err(_) => Err(AI_ERR_READ),
                }
            }
            ArgvIterator::Argv { args, current } => {
                if *current >= args.len() {
                    Ok(None)
                } else {
                    let arg = args[*current].clone();
                    *current += 1;
                    Ok(Some(arg))
                }
            }
        }
    }

    pub fn n_args(&self) -> usize {
        match self {
            ArgvIterator::Stream { item_idx, .. } => *item_idx,
            ArgvIterator::Argv { current, .. } => *current,
        }
    }
}

impl Drop for ArgvIterator {
    fn drop(&mut self) {
        // No explicit cleanup needed as Rust handles memory automatically
    }
}