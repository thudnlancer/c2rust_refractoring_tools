use std::ptr;
use std::mem;
use std::fmt;

const SHUFFLE_BITS: usize = 9;
const SHUFFLE_MAX: usize = 1 << SHUFFLE_BITS;
const SHUFFLE_MASK: usize = SHUFFLE_MAX - 1;

const TYPE_0: usize = 0;
const BREAK_0: usize = 8;
const DEG_0: usize = 0;
const SEP_0: usize = 0;

const TYPE_1: usize = 1;
const BREAK_1: usize = 32;
const DEG_1: usize = 7;
const SEP_1: usize = 3;

const TYPE_2: usize = 2;
const BREAK_2: usize = 64;
const DEG_2: usize = 15;
const SEP_2: usize = 1;

const TYPE_3: usize = 3;
const BREAK_3: usize = 128;
const DEG_3: usize = 31;
const SEP_3: usize = 3;

const TYPE_4: usize = 4;
const BREAK_4: usize = 256;
const DEG_4: usize = 63;
const SEP_4: usize = 1;

const MAX_TYPES: usize = 5;

const NSHUFF: usize = 50;

static mut shuffle_init: i32 = 1;
static mut shuffle_buffer: [i64; SHUFFLE_MAX] = [0; SHUFFLE_MAX];

static mut randtbl: [u32; DEG_3 + 1] = [
    TYPE_3 as u32,
    0x991539b1, 0x16a5bce3, 0x6774a4cd, 0x3e01511e, 0x4e508aaa, 0x61048c05,
    0xf5500617, 0x846b7115, 0x6a19892c, 0x896a97af, 0xdb48f936, 0x14898454,
    0x37ffd106, 0xb58bff9c, 0x59e17104, 0xcf918a49, 0x09378c83, 0x52c7a471,
    0x8d293ea9, 0x1f4fc301, 0xc3db71be, 0x39b44e1c, 0xf8a44ef9, 0x4c8b80b1,
    0x19edc328, 0x87bf4bdd, 0xc9b240e5, 0xe9ee4b1b, 0x4382aee7, 0x535b6b41,
    0xf3bec5da
];

static mut fptr: *mut u32 = ptr::null_mut();
static mut rptr: *mut u32 = ptr::null_mut();
static mut state: *mut u32 = ptr::null_mut();
static mut rand_type: i32 = TYPE_3 as i32;
static mut rand_deg: i32 = DEG_3 as i32;
static mut rand_sep: i32 = SEP_3 as i32;
static mut end_ptr: *mut u32 = ptr::null_mut();

const DEGREES: [i32; MAX_TYPES] = [DEG_0 as i32, DEG_1 as i32, DEG_2 as i32, DEG_3 as i32, DEG_4 as i32];
const SEPS: [i32; MAX_TYPES] = [SEP_0 as i32, SEP_1 as i32, SEP_2 as i32, SEP_3 as i32, SEP_4 as i32];

fn good_rand(x: i32) -> u32 {
    let mut hi: i32;
    let mut lo: i32;
    let mut x = x;

    if x == 0 {
        x = 123459876;
    }
    hi = x / 127773;
    lo = x % 127773;
    x = 16807 * lo - 2836 * hi;
    if x < 0 {
        x += 0x7fffffff;
    }
    x as u32
}

pub unsafe extern "C" fn srandom(x: u32) {
    let mut i: i32;
    let mut lim: i32;

    shuffle_init = 1;

    *state.offset(0) = x;
    if rand_type == TYPE_0 as i32 {
        lim = NSHUFF as i32;
    } else {
        i = 1;
        while i < rand_deg {
            *state.offset(i as isize) = good_rand(*state.offset((i - 1) as isize) as i32;
            i += 1;
        }
        fptr = state.offset(rand_sep as isize);
        rptr = state.offset(0);
        lim = 10 * rand_deg;
    }
    i = 0;
    while i < lim {
        random();
        i += 1;
    }
}

pub unsafe extern "C" fn initstate(seed: u32, arg_state: *mut u8, n: usize) -> *mut u8 {
    let ostate: *mut u8 = state.offset(-1) as *mut u8;
    let int_arg_state: *mut u32 = arg_state as *mut u32;

    if rand_type == TYPE_0 as i32 {
        *state.offset(-1) = rand_type as u32;
    } else {
        *state.offset(-1) = (MAX_TYPES as u32).wrapping_mul((rptr as isize - state as isize) as u32) + rand_type as u32;
    }
    if n < BREAK_0 {
        eprintln!("random: not enough state ({} bytes); ignored.", n);
        return ptr::null_mut();
    }
    if n < BREAK_1 {
        rand_type = TYPE_0 as i32;
        rand_deg = DEG_0 as i32;
        rand_sep = SEP_0 as i32;
    } else if n < BREAK_2 {
        rand_type = TYPE_1 as i32;
        rand_deg = DEG_1 as i32;
        rand_sep = SEP_1 as i32;
    } else if n < BREAK_3 {
        rand_type = TYPE_2 as i32;
        rand_deg = DEG_2 as i32;
        rand_sep = SEP_2 as i32;
    } else if n < BREAK_4 {
        rand_type = TYPE_3 as i32;
        rand_deg = DEG_3 as i32;
        rand_sep = SEP_3 as i32;
    } else {
        rand_type = TYPE_4 as i32;
        rand_deg = DEG_4 as i32;
        rand_sep = SEP_4 as i32;
    }
    state = int_arg_state.offset(1);
    end_ptr = state.offset(rand_deg as isize);
    srandom(seed);
    if rand_type == TYPE_0 as i32 {
        *int_arg_state.offset(0) = rand_type as u32;
    } else {
        *int_arg_state.offset(0) = (MAX_TYPES as u32).wrapping_mul((rptr as isize - state as isize) as u32) + rand_type as u32;
    }
    ostate
}

pub unsafe extern "C" fn setstate(arg_state: *mut u8) -> *mut u8 {
    let new_state: *mut u32 = arg_state as *mut u32;
    let type_ = *new_state.offset(0) % MAX_TYPES as u32;
    let rear = *new_state.offset(0) / MAX_TYPES as u32;
    let ostate: *mut u8 = state.offset(-1) as *mut u8;

    if rand_type == TYPE_0 as i32 {
        *state.offset(-1) = rand_type as u32;
    } else {
        *state.offset(-1) = (MAX_TYPES as u32).wrapping_mul((rptr as isize - state as isize) as u32) + rand_type as u32;
    }
    match type_ as usize {
        TYPE_0 | TYPE_1 | TYPE_2 | TYPE_3 | TYPE_4 => {
            rand_type = type_ as i32;
            rand_deg = DEGREES[type_ as usize];
            rand_sep = SEPS[type_ as usize];
        }
        _ => {
            eprintln!("random: state info corrupted; not changed.");
        }
    }
    state = new_state.offset(1);
    if rand_type != TYPE_0 as i32 {
        rptr = state.offset(rear as isize);
        fptr = state.offset((rear as i32 + rand_sep) as isize % rand_deg as isize);
    }
    end_ptr = state.offset(rand_deg as isize);
    ostate
}

unsafe fn random_old() -> i64 {
    let mut i: u32;
    let mut f: *mut u32;
    let mut r: *mut u32;

    if rand_type == TYPE_0 as i32 {
        i = *state.offset(0);
        *state.offset(0) = i = good_rand(i as i32) & 0x7fffffff;
    } else {
        f = fptr;
        r = rptr;
        *f = (*f).wrapping_add(*r);
        i = (*f >> 1) & 0x7fffffff;
        f = f.offset(1);
        if f >= end_ptr {
            f = state;
            r = r.offset(1);
        } else {
            r = r.offset(1);
            if r >= end_ptr {
                r = state;
            }
        }
        fptr = f;
        rptr = r;
    }
    i as i64
}

pub unsafe extern "C" fn random() -> i64 {
    let mut k: usize;
    let mut r: i64;
    static mut s: i64 = 0xcafefeed;

    if shuffle_init != 0 {
        for k in 0..SHUFFLE_MAX {
            shuffle_buffer[k] = random_old();
        }
        s = random_old();
        shuffle_init = 0;
    }

    r = random_old();
    k = (s as usize) & SHUFFLE_MASK;
    assert!(k < SHUFFLE_MAX);
    s = shuffle_buffer[k];
    shuffle_buffer[k] = r;
    s
}