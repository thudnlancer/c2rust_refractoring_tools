use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::os::unix::io::AsRawFd;
use std::ptr;
use libc::{c_int, c_void, c_char, c_uchar, c_long, c_ulong, size_t, off_t};
use nix::sys::mman::{mmap, munmap, ProtFlags, MapFlags};
use nix::unistd::{getpagesize, lseek};

type uintptr_t = c_ulong;
type pageinfo_t = c_uchar;
type MINCORE_ADDR_T = *mut c_void;

#[derive(Clone)]
struct VmaStruct {
    start: uintptr_t,
    end: uintptr_t,
    is_near_this: Option<fn(uintptr_t, &mut VmaStruct) -> c_int>,
    prev_end: uintptr_t,
}

#[derive(Clone)]
struct CallbackLocals {
    address: uintptr_t,
    vma: VmaStruct,
    prev: uintptr_t,
    retval: c_int,
}

#[derive(Clone)]
struct Rofile {
    position: size_t,
    filled: size_t,
    eof_seen: c_int,
    buffer: Vec<c_char>,
    auxmap: Option<Vec<c_char>>,
    auxmap_start: uintptr_t,
    auxmap_end: uintptr_t,
}

static mut PAGESIZE: uintptr_t = 0;

fn simple_is_near_this(addr: uintptr_t, vma: &mut VmaStruct) -> c_int {
    ((vma.start.wrapping_sub(addr)) <= (vma.start.wrapping_sub(vma.prev_end) / 2)) as c_int
}

fn rof_open(filename: &str) -> io::Result<Rofile> {
    let mut file = File::open(filename)?;
    let pagesize = getpagesize() as uintptr_t;
    let mut size = pagesize.max(73 + 4096) as size_t;
    
    let mut rof = Rofile {
        position: 0,
        filled: 0,
        eof_seen: 0,
        buffer: vec![0; 1],
        auxmap: None,
        auxmap_start: 0,
        auxmap_end: 0,
    };

    loop {
        let mut buffer = vec![0; size];
        match file.read(&mut buffer) {
            Ok(n) if n > 0 => {
                rof.filled = n as size_t;
                loop {
                    let remaining = size - rof.filled;
                    if remaining == 0 {
                        break;
                    }
                    match file.read(&mut buffer[rof.filled..]) {
                        Ok(0) => {
                            return Ok(rof);
                        }
                        Ok(m) => {
                            rof.filled += m as size_t;
                        }
                        Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                        Err(e) => return Err(e),
                    }
                }
                rof.buffer = buffer;
                return Ok(rof);
            }
            Ok(_) => return Ok(rof),
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }
}

fn rof_peekchar(rof: &Rofile) -> Option<c_int> {
    if rof.position == rof.filled {
        None
    } else {
        Some(rof.buffer[rof.position] as c_int)
    }
}

fn rof_getchar(rof: &mut Rofile) -> Option<c_int> {
    rof_peekchar(rof).map(|c| {
        rof.position += 1;
        c
    })
}

fn rof_scanf_lx(rof: &mut Rofile) -> Option<uintptr_t> {
    let mut value = 0;
    let mut numdigits = 0;

    while let Some(c) = rof_peekchar(rof) {
        let digit = match c {
            b'0'..=b'9' => c - b'0',
            b'A'..=b'F' => c - b'A' + 10,
            b'a'..=b'f' => c - b'a' + 10,
            _ => break,
        };
        value = (value << 4) | digit as uintptr_t;
        numdigits += 1;
        rof_getchar(rof);
    }

    if numdigits > 0 { Some(value) } else { None }
}

fn vma_iterate_proc(locals: &mut CallbackLocals) -> c_int {
    match rof_open("/proc/self/maps") {
        Ok(mut rof) => {
            let auxmap_start = rof.auxmap_start;
            let auxmap_end = rof.auxmap_end;

            while let (Some(start), Some('-'), Some(end)) = (
                rof_scanf_lx(&mut rof),
                rof_getchar(&mut rof),
                rof_scanf_lx(&mut rof),
            ) {
                // Skip until newline
                while let Some(c) = rof_getchar(&mut rof) {
                    if c == b'\n' as c_int {
                        break;
                    }
                }

                if start <= auxmap_start && auxmap_end - 1 <= end - 1 {
                    if start < auxmap_start && callback(locals, start, auxmap_start) != 0 {
                        break;
                    }
                    if auxmap_end - 1 < end - 1 && callback(locals, auxmap_end, end) != 0 {
                        break;
                    }
                } else if callback(locals, start, end) != 0 {
                    break;
                }
            }
            0
        }
        Err(_) => -1,
    }
}

fn is_mapped(addr: uintptr_t) -> bool {
    unsafe {
        let mut vec = [0; 1];
        libc::mincore(addr as MINCORE_ADDR_T, PAGESIZE, vec.as_mut_ptr()) >= 0
    }
}

fn mapped_range_start(addr: uintptr_t) -> uintptr_t {
    unsafe {
        let mut vec = [0; 1024];
        let mut stepsize = std::mem::size_of::<[pageinfo_t; 1024]>() as uintptr_t;
        let mut current = addr;

        loop {
            if current == 0 {
                return current;
            }

            let max_remaining = current / PAGESIZE;
            stepsize = stepsize.min(max_remaining);
            if stepsize == 0 {
                break;
            }

            if libc::mincore(
                current - stepsize * PAGESIZE,
                stepsize * PAGESIZE,
                vec.as_mut_ptr(),
            ) < 0
            {
                break;
            }
            current -= stepsize * PAGESIZE;
        }

        while stepsize > 1 {
            let halfstepsize1 = (stepsize + 1) / 2;
            let halfstepsize2 = stepsize / 2;

            if libc::mincore(
                current - halfstepsize1 * PAGESIZE,
                halfstepsize1 * PAGESIZE,
                vec.as_mut_ptr(),
            ) < 0
            {
                stepsize = halfstepsize1;
            } else {
                current -= halfstepsize1 * PAGESIZE;
                stepsize = halfstepsize2;
            }
        }

        current
    }
}

fn mapped_range_end(addr: uintptr_t) -> uintptr_t {
    unsafe {
        let mut vec = [0; 1024];
        let mut stepsize = std::mem::size_of::<[pageinfo_t; 1024]>() as uintptr_t;
        let mut current = addr + PAGESIZE;

        loop {
            if current == 0 {
                return current;
            }

            let max_remaining = (!current + 1) / PAGESIZE;
            stepsize = stepsize.min(max_remaining);
            if stepsize == 0 {
                break;
            }

            if libc::mincore(current, stepsize * PAGESIZE, vec.as_mut_ptr()) < 0 {
                break;
            }
            current += stepsize * PAGESIZE;
        }

        while stepsize > 1 {
            let halfstepsize1 = (stepsize + 1) / 2;
            let halfstepsize2 = stepsize / 2;

            if libc::mincore(current, halfstepsize1 * PAGESIZE, vec.as_mut_ptr()) < 0 {
                stepsize = halfstepsize1;
            } else {
                current += halfstepsize1 * PAGESIZE;
                stepsize = halfstepsize2;
            }
        }

        current
    }
}

fn mincore_get_vma(address: uintptr_t) -> VmaStruct {
    unsafe {
        if PAGESIZE == 0 {
            PAGESIZE = getpagesize() as uintptr_t;
        }

        let aligned_addr = address / PAGESIZE * PAGESIZE;
        let start = mapped_range_start(aligned_addr);
        let end = mapped_range_end(aligned_addr);

        VmaStruct {
            start,
            end,
            is_near_this: Some(mincore_is_near_this),
            prev_end: 0,
        }
    }
}

fn sigsegv_get_vma(address: uintptr_t) -> Result<VmaStruct, ()> {
    let mut locals = CallbackLocals {
        address,
        vma: VmaStruct {
            start: 0,
            end: 0,
            is_near_this: None,
            prev_end: 0,
        },
        prev: 0,
        retval: -1,
    };

    if vma_iterate_proc(&mut locals) == 0 && locals.retval == 0 {
        locals.vma.is_near_this = Some(simple_is_near_this);
        Ok(locals.vma)
    } else {
        Ok(mincore_get_vma(address))
    }
}