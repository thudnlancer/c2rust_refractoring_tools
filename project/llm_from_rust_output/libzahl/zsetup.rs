use std::mem;
use std::ptr;
use std::os::raw::{c_int, c_long, c_ulong, c_ulonglong, c_void};

type size_t = c_ulong;
type uint64_t = c_ulong;
type zahl_char_t = uint64_t;

#[derive(Clone, Copy)]
struct Zahl {
    sign: c_int,
    padding__: c_int,
    used: size_t,
    alloced: size_t,
    chars: *mut zahl_char_t,
}

type z_t = [Zahl; 1];

struct LibzahlState {
    jmp_buf: [__jmp_buf_tag; 1],
    set_up: c_int,
    error: c_int,
    pool: [*mut *mut zahl_char_t; 64],
    pool_n: [size_t; 64],
    pool_alloc: [size_t; 64],
    temp_stack: *mut *mut Zahl,
    temp_stack_head: *mut *mut Zahl,
    temp_stack_end: *mut *mut Zahl,
    temp_allocation: *mut c_void,
    tmp_div: z_t,
    tmp_mod: z_t,
    tmp_str_num: z_t,
    tmp_str_mag: z_t,
    tmp_str_div: z_t,
    tmp_str_rem: z_t,
    tmp_gcd_u: z_t,
    tmp_gcd_v: z_t,
    tmp_sub: z_t,
    tmp_modmul: z_t,
    tmp_pow_b: z_t,
    tmp_pow_c: z_t,
    tmp_pow_d: z_t,
    tmp_modsqr: z_t,
    tmp_divmod_a: z_t,
    tmp_divmod_b: z_t,
    tmp_divmod_d: z_t,
    tmp_ptest_x: z_t,
    tmp_ptest_a: z_t,
    tmp_ptest_d: z_t,
    tmp_ptest_n1: z_t,
    tmp_ptest_n4: z_t,
    const_1e19: z_t,
    const_1: z_t,
    const_2: z_t,
    const_4: z_t,
    tmp_divmod_ds: [[Zahl; 1]; 64],
    constant_chars: [zahl_char_t; 8],
}

impl LibzahlState {
    fn new() -> Self {
        Self {
            jmp_buf: [__jmp_buf_tag {
                __jmpbuf: [0; 8],
                __mask_was_saved: 0,
                __saved_mask: __sigset_t { __val: [0; 16] },
            }; 1],
            set_up: 0,
            error: 0,
            pool: [ptr::null_mut(); 64],
            pool_n: [0; 64],
            pool_alloc: [0; 64],
            temp_stack: ptr::null_mut(),
            temp_stack_head: ptr::null_mut(),
            temp_stack_end: ptr::null_mut(),
            temp_allocation: ptr::null_mut(),
            tmp_div: [Zahl {
                sign: 0,
                padding__: 0,
                used: 0,
                alloced: 0,
                chars: ptr::null_mut(),
            }; 1],
            tmp_mod: [Zahl {
                sign: 0,
                padding__: 0,
                used: 0,
                alloced: 0,
                chars: ptr::null_mut(),
            }; 1],
            tmp_str_num: [Zahl {
                sign: 0,
                padding__: 0,
                used: 0,
                alloced: 0,
                chars: ptr::null_mut(),
            }; 1],
            tmp_str_mag: [Zahl {
                sign: 0,
                padding__: 0,
                used: 0,
                alloced: 0,
                chars: ptr::null_mut(),
            }; 1],
            tmp_str_div: [Zahl {
                sign: 0,
                padding__: 0,
                used: 0,
                alloced: 0,
                chars: ptr::null_mut(),
            }; 1],
            tmp_str_rem: [Zahl {
                sign: 0,
                padding__: 0,
                used: 0,
                alloced: 0,
                chars: ptr::null_mut(),
            }; 1],
            tmp_gcd_u: [Zahl {
                sign: 0,
                padding__: 0,
                used: 0,
                alloced: 0,
                chars: ptr::null_mut(),
            }; 1],
            tmp_gcd_v: [Zahl {
                sign: 0,
                padding__: 0,
                used: 0,
                alloced: 0,
                chars: ptr::null_mut(),
            }; 1],
            tmp_sub: [Zahl {
                sign: 0,
                padding__: 0,
                used: 0,
                alloced: 0,
                chars: ptr::null_mut(),
            }; 1],
            tmp_modmul: [Zahl {
                sign: 0,
                padding__: 0,
                used: 0,
                alloced: 0,
                chars: ptr::null_mut(),
            }; 1],
            tmp_pow_b: [Zahl {
                sign: 0,
                padding__: 0,
                used: 0,
                alloced: 0,
                chars: ptr::null_mut(),
            }; 1],
            tmp_pow_c: [Zahl {
                sign: 0,
                padding__: 0,
                used: 0,
                alloced: 0,
                chars: ptr::null_mut(),
            }; 1],
            tmp_pow_d: [Zahl {
                sign: 0,
                padding__: 0,
                used: 0,
                alloced: 0,
                chars: ptr::null_mut(),
            }; 1],
            tmp_modsqr: [Zahl {
                sign: 0,
                padding__: 0,
                used: 0,
                alloced: 0,
                chars: ptr::null_mut(),
            }; 1],
            tmp_divmod_a: [Zahl {
                sign: 0,
                padding__: 0,
                used: 0,
                alloced: 0,
                chars: ptr::null_mut(),
            }; 1],
            tmp_divmod_b: [Zahl {
                sign: 0,
                padding__: 0,
                used: 0,
                alloced: 0,
                chars: ptr::null_mut(),
            }; 1],
            tmp_divmod_d: [Zahl {
                sign: 0,
                padding__: 0,
                used: 0,
                alloced: 0,
                chars: ptr::null_mut(),
            }; 1],
            tmp_ptest_x: [Zahl {
                sign: 0,
                padding__: 0,
                used: 0,
                alloced: 0,
                chars: ptr::null_mut(),
            }; 1],
            tmp_ptest_a: [Zahl {
                sign: 0,
                padding__: 0,
                used: 0,
                alloced: 0,
                chars: ptr::null_mut(),
            }; 1],
            tmp_ptest_d: [Zahl {
                sign: 0,
                padding__: 0,
                used: 0,
                alloced: 0,
                chars: ptr::null_mut(),
            }; 1],
            tmp_ptest_n1: [Zahl {
                sign: 0,
                padding__: 0,
                used: 0,
                alloced: 0,
                chars: ptr::null_mut(),
            }; 1],
            tmp_ptest_n4: [Zahl {
                sign: 0,
                padding__: 0,
                used: 0,
                alloced: 0,
                chars: ptr::null_mut(),
            }; 1],
            const_1e19: [Zahl {
                sign: 0,
                padding__: 0,
                used: 0,
                alloced: 0,
                chars: ptr::null_mut(),
            }; 1],
            const_1: [Zahl {
                sign: 0,
                padding__: 0,
                used: 0,
                alloced: 0,
                chars: ptr::null_mut(),
            }; 1],
            const_2: [Zahl {
                sign: 0,
                padding__: 0,
                used: 0,
                alloced: 0,
                chars: ptr::null_mut(),
            }; 1],
            const_4: [Zahl {
                sign: 0,
                padding__: 0,
                used: 0,
                alloced: 0,
                chars: ptr::null_mut(),
            }; 1],
            tmp_divmod_ds: [[Zahl {
                sign: 0,
                padding__: 0,
                used: 0,
                alloced: 0,
                chars: ptr::null_mut(),
            }; 1]; 64],
            constant_chars: [0; 8],
        }
    }

    fn zinit(&mut self, a: &mut Zahl) {
        a.alloced = 0;
        a.chars = ptr::null_mut();
    }

    fn zsetu(&mut self, a: &mut Zahl, b: uint64_t) {
        if b == 0 {
            a.sign = 0;
            return;
        }
        if a.alloced < 1 {
            // libzahl_realloc(a, 1);
            unimplemented!();
        }
        a.sign = 1;
        unsafe { *a.chars = b };
        a.used = 1;
    }

    fn setup(&mut self, env: &mut __jmp_buf_tag) {
        self.jmp_buf[0] = *env;
        if self.set_up == 0 {
            self.set_up = 1;
            self.pool = [ptr::null_mut(); 64];
            self.pool_n = [0; 64];
            self.pool_alloc = [0; 64];
            
            self.zinit(&mut self.tmp_div[0]);
            self.zinit(&mut self.tmp_mod[0]);
            self.zinit(&mut self.tmp_str_num[0]);
            self.zinit(&mut self.tmp_str_mag[0]);
            self.zinit(&mut self.tmp_str_div[0]);
            self.zinit(&mut self.tmp_str_rem[0]);
            self.zinit(&mut self.tmp_gcd_u[0]);
            self.zinit(&mut self.tmp_gcd_v[0]);
            self.zinit(&mut self.tmp_sub[0]);
            self.zinit(&mut self.tmp_modmul[0]);
            self.zinit(&mut self.tmp_pow_b[0]);
            self.zinit(&mut self.tmp_pow_c[0]);
            self.zinit(&mut self.tmp_pow_d[0]);
            self.zinit(&mut self.tmp_modsqr[0]);
            self.zinit(&mut self.tmp_divmod_a[0]);
            self.zinit(&mut self.tmp_divmod_b[0]);
            self.zinit(&mut self.tmp_divmod_d[0]);
            self.zinit(&mut self.tmp_ptest_x[0]);
            self.zinit(&mut self.tmp_ptest_a[0]);
            self.zinit(&mut self.tmp_ptest_d[0]);
            self.zinit(&mut self.tmp_ptest_n1[0]);
            self.zinit(&mut self.tmp_ptest_n4[0]);

            self.const_1e19[0].alloced = 1;
            self.const_1e19[0].chars = self.constant_chars.as_mut_ptr();
            self.zsetu(&mut self.const_1e19[0], 10000000000000000000u64);

            self.const_1[0].alloced = 1;
            self.const_1[0].chars = unsafe { self.constant_chars.as_mut_ptr().offset(1) };
            self.zsetu(&mut self.const_1[0], 1);

            self.const_2[0].alloced = 1;
            self.const_2[0].chars = unsafe { self.constant_chars.as_mut_ptr().offset(2) };
            self.zsetu(&mut self.const_2[0], 2);

            self.const_4[0].alloced = 1;
            self.const_4[0].chars = unsafe { self.constant_chars.as_mut_ptr().offset(3) };
            self.zsetu(&mut self.const_4[0], 4);

            for i in (0..64).rev() {
                self.zinit(&mut self.tmp_divmod_ds[i][0]);
            }

            // Initialize temp stack
            let stack_size = 256 * mem::size_of::<*mut Zahl>();
            unsafe {
                self.temp_stack = libc::malloc(stack_size) as *mut *mut Zahl;
                if self.temp_stack.is_null() {
                    // libzahl_memfailure();
                    unimplemented!();
                }
                self.temp_stack_head = self.temp_stack;
                self.temp_stack_end = self.temp_stack.offset(256);
            }
        }
    }
}

#[derive(Clone, Copy)]
struct __jmp_buf_tag {
    __jmpbuf: [c_long; 8],
    __mask_was_saved: c_int,
    __saved_mask: __sigset_t,
}

#[derive(Clone, Copy)]
struct __sigset_t {
    __val: [c_ulong; 16],
}