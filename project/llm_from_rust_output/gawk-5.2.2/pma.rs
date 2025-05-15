use libc::{c_int, c_void, c_char, c_ulong, c_long, c_uint, size_t, off_t, ssize_t};
use std::ptr;
use std::mem;
use std::ffi::CString;
use std::os::raw::c_void as CVoid;
use std::sync::Once;

const WDSZ: usize = 8;
const NFL: usize = 422;
const VERS: u64 = 2;
const ALGN: usize = 1073741824;
const IU: u8 = 0;
const PIU: u8 = 1;
const GROWN: u8 = 2;

static mut UB: [size_t; NFL] = [0; NFL];
static mut lomask: usize = 0x7;
static mut himask: usize = !0x7;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct Timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct Stat {
    st_dev: c_ulong,
    st_ino: c_ulong,
    st_nlink: c_ulong,
    st_mode: c_uint,
    st_uid: c_uint,
    st_gid: c_uint,
    __pad0: c_int,
    st_rdev: c_ulong,
    st_size: off_t,
    st_blksize: c_long,
    st_blocks: c_long,
    st_atim: Timespec,
    st_mtim: Timespec,
    st_ctim: Timespec,
    __glibc_reserved: [c_long; 3],
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct Ao {
    anext: *mut Ao,
    fprev: *mut Ao,
    fnext: *mut Ao,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct PmaHdr {
    mapaddr: *mut c_void,
    bf_vers: u64,
    nallocs: u64,
    nfrees: u64,
    res_0: u64,
    root: *mut c_void,
    afirst: *mut Ao,
    abound: *mut Ao,
    free: [Ao; NFL],
}

#[derive(Debug)]
struct State {
    init: c_int,
    vrb: c_int,
    file: *const c_char,
    hdr: *mut PmaHdr,
}

static mut STATE: State = State {
    init: 0,
    vrb: 0,
    file: ptr::null(),
    hdr: ptr::null_mut(),
};

static INIT_UB: Once = Once::new();

fn sc(nbytes: size_t) -> usize {
    INIT_UB.call_once(|| {
        unsafe {
            UB[0] = 3;
            let mut c = 1;
            loop {
                let hi = ((1 + UB[c - 1]) as f64 * 1.1).floor() as usize;
                if hi >= (1 << 61) {
                    UB[c] = 1 << 61;
                    break;
                } else {
                    UB[c] = hi;
                    c += 1;
                }
            }
        }
    });

    let nwords = nbytes / WDSZ + if nbytes % WDSZ != 0 { 1 } else { 0 };
    unsafe {
        UB.iter()
            .position(|&ub| nwords <= ub)
            .unwrap_or(NFL - 1)
    }
}

fn slobh(p: *mut Ao, iu: u8, piu: u8, grown: u8) {
    unsafe {
        let h = (p as usize & himask) | ((iu as usize) | ((piu as usize) << 1) | ((grown as usize) << 2);
        (*p).anext = h as *mut Ao;
    }
}

fn globh(p: *const Ao) -> (u8, u8, u8) {
    unsafe {
        let h = p as usize;
        let iu = (h & 0x1 != 0) as u8;
        let piu = (h & 0x2 != 0) as u8;
        let grown = (h & 0x4 != 0) as u8;
        (iu, piu, grown)
    }
}

fn getbit(p: *const Ao, bit: u8) -> u8 {
    let (iu, piu, grown) = globh(p);
    match bit {
        0 => iu,
        1 => piu,
        2 => grown,
        _ => {
            unsafe {
                if STATE.vrb > 0 {
                    libc::fprintf(
                        libc::stderr,
                        b"pma.c:148: ERROR: bad bit: %d\n\0".as_ptr() as *const c_char,
                        bit,
                    );
                }
            }
            0
        }
    }
}

fn fli(p: *mut Ao) {
    unsafe {
        let size = ((*p).anext as usize & himask) - (p as usize & himask) - WDSZ;
        let fl = sc(size);
        let h = &mut (*STATE.hdr).free[fl] as *mut Ao;
        (*p).fprev = h;
        (*p).fnext = (*h).fnext;
        (*(*h).fnext).fprev = p;
        (*h).fnext = p;
    }
}

fn flr(p: *mut Ao) {
    unsafe {
        (*(*p).fnext).fprev = (*p).fprev;
        (*(*p).fprev).fnext = (*p).fnext;
        (*p).fnext = ptr::null_mut();
        (*p).fprev = (*p).fnext;
    }
}

fn addrgap(n: off_t) -> *mut c_void {
    unsafe {
        let mut a = ptr::null_mut();
        let mut amax = ptr::null_mut();
        let mut max = 0;
        let n = n as usize;
        
        if n < mem::size_of::<PmaHdr>() + 40960 {
            if STATE.vrb > 0 {
                libc::fprintf(
                    libc::stderr,
                    b"pma.c:353: ERROR: file size %zu too small\n\0".as_ptr() as *const c_char,
                    n,
                );
            }
            return ptr::null_mut();
        }

        let mut l = 1;
        let mut u = usize::MAX;
        while l <= u {
            let m = l + (u - l) / 2;
            a = libc::mmap(
                ptr::null_mut(),
                m,
                0,
                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_NORESERVE,
                -1,
                0,
            );
            if a != libc::MAP_FAILED {
                max = m;
                amax = a;
                libc::munmap(a, m);
                if m == usize::MAX {
                    break;
                }
                l = m + 1;
            } else {
                u = m - 1;
            }
        }

        if max < n + ALGN * 2 {
            if STATE.vrb > 0 {
                libc::fprintf(
                    libc::stderr,
                    b"pma.c:368: ERROR: max gap %zu too small for required %zu\n\0".as_ptr() as *const c_char,
                    max,
                    n,
                );
            }
            return ptr::null_mut();
        }

        let mut r = (amax as usize + (max - n) / 2) as *mut c_char;
        if r as usize % ALGN != 0 {
            r = r.add(ALGN - (r as usize % ALGN));
        }
        r as *mut c_void
    }
}

pub fn pma_init(verbose: c_int, file: *const c_char) -> c_int {
    unsafe {
        if !(0 <= verbose && verbose <= 3) {
            return 386;
        }
        STATE.vrb = verbose;

        if STATE.init != 0 {
            if STATE.vrb > 0 {
                libc::fprintf(
                    libc::stderr,
                    b"pma.c:396: ERROR: already initialized\n\0".as_ptr() as *const c_char,
                );
            }
            return 396;
        }

        if file.is_null() {
            STATE.init = 2;
            STATE.file = ptr::null();
            STATE.hdr = ptr::null_mut();
            return 0;
        }

        let ps = libc::sysconf(libc::_SC_PAGESIZE);
        if ps < 4096 {
            if STATE.vrb > 0 {
                libc::fprintf(
                    libc::stderr,
                    b"pma.c:418: ERROR: bad page size %ld, errno '%s'\n\0".as_ptr() as *const c_char,
                    ps,
                    libc::strerror(*libc::__errno_location()),
                );
            }
            return 418;
        }

        let fd = libc::open(file, libc::O_RDWR);
        if fd < 0 {
            if STATE.vrb > 0 {
                libc::fprintf(
                    libc::stderr,
                    b"pma.c:424: ERROR: open() errno => '%s'\n\0".as_ptr() as *const c_char,
                    libc::strerror(*libc::__errno_location()),
                );
            }
            return 424;
        }

        let mut s: Stat = mem::zeroed();
        if libc::fstat(fd, &mut s) != 0 {
            if STATE.vrb > 0 {
                libc::fprintf(
                    libc::stderr,
                    b"pma.c:425: ERROR: fstat() errno => '%s'\n\0".as_ptr() as *const c_char,
                    libc::strerror(*libc::__errno_location()),
                );
            }
            return 425;
        }

        let mut a1 = ptr::null_mut();
        if libc::read(fd, &mut a1 as *mut *mut c_void as *mut c_void, mem::size_of::<*mut c_void>()) != mem::size_of::<*mut c_void>() as isize {
            if STATE.vrb > 0 {
                libc::fprintf(
                    libc::stderr,
                    b"pma.c:427: ERROR: read() errno => '%s'\n\0".as_ptr() as *const c_char,
                    libc::strerror(*libc::__errno_location()),
                );
            }
            return 427;
        }

        if a1.is_null() {
            a1 = addrgap(s.st_size);
        }

        let a2 = libc::mmap(
            a1,
            s.st_size as usize,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd,
            0,
        );

        libc::close(fd);

        if a1 != a2 {
            if STATE.vrb > 0 {
                libc::fprintf(
                    libc::stderr,
                    b"pma.c:431: ERROR: mmap() errno => '%s'\n\0".as_ptr() as *const c_char,
                    libc::strerror(*libc::__errno_location()),
                );
            }
            return 431;
        }

        STATE.init = 1;
        STATE.file = file;
        STATE.hdr = a2 as *mut PmaHdr;

        if (*STATE.hdr).mapaddr.is_null() {
            if s.st_size % ps != 0 {
                if STATE.vrb > 0 {
                    libc::fprintf(
                        libc::stderr,
                        b"pma.c:441: ERROR: backing file size %jd not multiple of page size %ld\n\0".as_ptr() as *const c_char,
                        s.st_size,
                        ps,
                    );
                }
                return 442;
            }

            for i in 0..NFL {
                (*STATE.hdr).free[i].fnext = &mut (*STATE.hdr).free[i];
                (*STATE.hdr).free[i].fprev = (*STATE.hdr).free[i].fnext;
            }

            (*STATE.hdr).mapaddr = STATE.hdr as *mut c_void;
            (*STATE.hdr).bf_vers = VERS;
            (*STATE.hdr).nallocs = 0;
            (*STATE.hdr).nfrees = 0;
            (*STATE.hdr).res_0 = 0;
            (*STATE.hdr).afirst = STATE.hdr.offset(1) as *mut Ao;
            (*STATE.hdr).abound = (STATE.hdr as *mut u8).offset(s.st_size as isize) as *mut Ao;

            let w = (*STATE.hdr).afirst;
            (*w).anext = (*STATE.hdr).abound;
            let ftr = ((*STATE.hdr).abound as *mut *mut Ao).offset(-1);
            *ftr = w;
            fli(w);
        }

        0
    }
}