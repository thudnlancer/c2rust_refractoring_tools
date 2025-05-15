use std::ptr;
use std::mem;

type size_t = usize;
type uint64_t = u64;
type zahl_char_t = uint64_t;

#[derive(Clone)]
struct Zahl {
    sign: i32,
    padding__: i32,
    used: size_t,
    alloced: size_t,
    chars: Option<Box<[zahl_char_t]>>,
}

type Z = [Zahl; 1];

struct LibZahlState {
    tmp_div: Zahl,
    tmp_mod: Zahl,
    tmp_str_num: Zahl,
    tmp_str_mag: Zahl,
    tmp_str_div: Zahl,
    tmp_str_rem: Zahl,
    tmp_gcd_u: Zahl,
    tmp_gcd_v: Zahl,
    tmp_sub: Zahl,
    tmp_modmul: Zahl,
    tmp_pow_b: Zahl,
    tmp_pow_c: Zahl,
    tmp_pow_d: Zahl,
    tmp_modsqr: Zahl,
    tmp_divmod_a: Zahl,
    tmp_divmod_b: Zahl,
    tmp_divmod_d: Zahl,
    tmp_ptest_x: Zahl,
    tmp_ptest_a: Zahl,
    tmp_ptest_d: Zahl,
    tmp_ptest_n1: Zahl,
    tmp_ptest_n4: Zahl,
    tmp_divmod_ds: [Zahl; 64],
    pool: Vec<Vec<Box<[zahl_char_t]>>>,
    temp_stack: Vec<Box<Zahl>>,
    is_set_up: bool,
}

impl LibZahlState {
    fn new() -> Self {
        Self {
            tmp_div: Zahl::new(),
            tmp_mod: Zahl::new(),
            tmp_str_num: Zahl::new(),
            tmp_str_mag: Zahl::new(),
            tmp_str_div: Zahl::new(),
            tmp_str_rem: Zahl::new(),
            tmp_gcd_u: Zahl::new(),
            tmp_gcd_v: Zahl::new(),
            tmp_sub: Zahl::new(),
            tmp_modmul: Zahl::new(),
            tmp_pow_b: Zahl::new(),
            tmp_pow_c: Zahl::new(),
            tmp_pow_d: Zahl::new(),
            tmp_modsqr: Zahl::new(),
            tmp_divmod_a: Zahl::new(),
            tmp_divmod_b: Zahl::new(),
            tmp_divmod_d: Zahl::new(),
            tmp_ptest_x: Zahl::new(),
            tmp_ptest_a: Zahl::new(),
            tmp_ptest_d: Zahl::new(),
            tmp_ptest_n1: Zahl::new(),
            tmp_ptest_n4: Zahl::new(),
            tmp_divmod_ds: [(); 64].map(|_| Zahl::new()),
            pool: vec![Vec::new(); 64],
            temp_stack: Vec::new(),
            is_set_up: false,
        }
    }

    fn unsetup(&mut self) {
        if !self.is_set_up {
            return;
        }

        self.is_set_up = false;

        // All fields will be automatically dropped when the struct goes out of scope
    }
}

impl Zahl {
    fn new() -> Self {
        Self {
            sign: 0,
            padding__: 0,
            used: 0,
            alloced: 0,
            chars: None,
        }
    }
}

impl Drop for LibZahlState {
    fn drop(&mut self) {
        self.unsetup();
    }
}