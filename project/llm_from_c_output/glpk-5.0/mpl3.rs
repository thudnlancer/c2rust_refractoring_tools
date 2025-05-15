/* mpl3.rs */

use std::f64::{MAX, MIN, MIN_POSITIVE};
use std::cmp::Ordering;
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::collections::{BTreeSet, HashMap};

const DBL_DIG: usize = 15;
const MAX_LENGTH: usize = 255;
const OUTBUF_SIZE: usize = 1024;

#[derive(Debug, Clone, PartialEq)]
enum Value {
    None,
    Numeric(f64),
    Symbolic(String),
    Logical(bool),
    Tuple(Vec<Value>),
    ElemSet(BTreeSet<Value>),
    ElemVar(Box<ElemVar>),
    Formula(Box<Formula>),
    ElemCon(Box<ElemCon>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Symbol {
    num: f64,
    str: Option<String>,
}

impl Symbol {
    fn new_num(num: f64) -> Self {
        Symbol { num, str: None }
    }

    fn new_str(s: String) -> Self {
        Symbol { num: 0.0, str: Some(s) }
    }
}

#[derive(Debug, Clone)]
struct Tuple {
    symbols: Vec<Symbol>,
}

impl Tuple {
    fn new() -> Self {
        Tuple { symbols: vec![] }
    }

    fn expand(mut self, sym: Symbol) -> Self {
        self.symbols.push(sym);
        self
    }

    fn dimen(&self) -> usize {
        self.symbols.len()
    }
}

#[derive(Debug, Clone)]
struct ElemSet {
    dim: usize,
    members: BTreeSet<Tuple>,
}

impl ElemSet {
    fn new(dim: usize) -> Self {
        ElemSet { dim, members: BTreeSet::new() }
    }

    fn add_tuple(&mut self, tuple: Tuple) -> bool {
        if tuple.dimen() == self.dim {
            self.members.insert(tuple)
        } else {
            false
        }
    }

    fn find_tuple(&self, tuple: &Tuple) -> bool {
        self.members.contains(tuple)
    }

    fn size(&self) -> usize {
        self.members.len()
    }
}

#[derive(Debug, Clone)]
struct ElemVar {
    j: usize,
    var: Variable,
    memb: Member,
    lbnd: f64,
    ubnd: f64,
    temp: f64,
    stat: i32,
    prim: f64,
    dual: f64,
}

#[derive(Debug, Clone)]
struct Formula {
    coef: f64,
    var: Option<ElemVar>,
    next: Option<Box<Formula>>,
}

impl Formula {
    fn constant(coef: f64) -> Self {
        Formula { coef, var: None, next: None }
    }

    fn single_var(var: ElemVar) -> Self {
        Formula { coef: 1.0, var: Some(var), next: None }
    }
}

#[derive(Debug, Clone)]
struct ElemCon {
    i: usize,
    con: Constraint,
    memb: Member,
    form: Formula,
    lbnd: f64,
    ubnd: f64,
    stat: i32,
    prim: f64,
    dual: f64,
}

struct MPL {
    sym_buf: String,
    tup_buf: String,
    rand: Rng,
    a_list: Vec<Array>,
    strings: Vec<String>,
    symbols: Vec<Symbol>,
    tuples: Vec<Tuple>,
    formulae: Vec<Formula>,
    elemvars: Vec<ElemVar>,
    elemcons: Vec<ElemCon>,
    dca: Option<TabDca>,
    prt_fp: Option<std::fs::File>,
    prt_file: Option<String>,
    flag_p: bool,
    stmt: Option<Statement>,
}

impl MPL {
    fn new() -> Self {
        MPL {
            sym_buf: String::with_capacity(256),
            tup_buf: String::with_capacity(256),
            rand: Rng::new(),
            a_list: vec![],
            strings: vec![],
            symbols: vec![],
            tuples: vec![],
            formulae: vec![],
            elemvars: vec![],
            elemcons: vec![],
            dca: None,
            prt_fp: None,
            prt_file: None,
            flag_p: false,
            stmt: None,
        }
    }

    fn error(&self, msg: &str) {
        panic!("{}", msg);
    }

    fn fp_add(&mut self, x: f64, y: f64) -> f64 {
        if (x > 0.0 && y > 0.0 && x > 0.999 * MAX - y) ||
           (x < 0.0 && y < 0.0 && x < -0.999 * MAX - y) {
            self.error(&format!("{:.15} + {:.15}; floating-point overflow", x, y));
        }
        x + y
    }

    fn fp_sub(&mut self, x: f64, y: f64) -> f64 {
        if (x > 0.0 && y < 0.0 && x > 0.999 * MAX + y) ||
           (x < 0.0 && y > 0.0 && x < -0.999 * MAX + y) {
            self.error(&format!("{:.15} - {:.15}; floating-point overflow", x, y));
        }
        x - y
    }

    fn fp_less(&mut self, x: f64, y: f64) -> f64 {
        if x < y {
            0.0
        } else if x > 0.0 && y < 0.0 && x > 0.999 * MAX + y {
            self.error(&format!("{:.15} less {:.15}; floating-point overflow", x, y));
            0.0
        } else {
            x - y
        }
    }

    fn fp_mul(&mut self, x: f64, y: f64) -> f64 {
        if y.abs() > 1.0 && x.abs() > (0.999 * MAX) / y.abs() {
            self.error(&format!("{:.15} * {:.15}; floating-point overflow", x, y));
        }
        x * y
    }

    fn fp_div(&mut self, x: f64, y: f64) -> f64 {
        if y.abs() < MIN_POSITIVE {
            self.error(&format!("{:.15} / {:.15}; floating-point zero divide", x, y));
        }
        if y.abs() < 1.0 && x.abs() > (0.999 * MAX) * y.abs() {
            self.error(&format!("{:.15} / {:.15}; floating-point overflow", x, y));
        }
        x / y
    }

    fn fp_idiv(&mut self, x: f64, y: f64) -> f64 {
        if y.abs() < MIN_POSITIVE {
            self.error(&format!("{:.15} div {:.15}; floating-point zero divide", x, y));
        }
        if y.abs() < 1.0 && x.abs() > (0.999 * MAX) * y.abs() {
            self.error(&format!("{:.15} div {:.15}; floating-point overflow", x, y));
        }
        let res = x / y;
        if res > 0.0 {
            res.floor()
        } else if res < 0.0 {
            res.ceil()
        } else {
            0.0
        }
    }

    fn fp_mod(&mut self, x: f64, y: f64) -> f64 {
        if x == 0.0 {
            0.0
        } else if y == 0.0 {
            x
        } else {
            let mut r = x.abs() % y.abs();
            if r != 0.0 {
                if x < 0.0 {
                    r = -r;
                }
                if (x > 0.0 && y < 0.0) || (x < 0.0 && y > 0.0) {
                    r += y;
                }
            }
            r
        }
    }

    fn fp_power(&mut self, x: f64, y: f64) -> f64 {
        if (x == 0.0 && y <= 0.0) || (x < 0.0 && y != y.floor()) {
            self.error(&format!("{:.15} ** {:.15}; result undefined", x, y));
        }
        if x == 0.0 {
            return 0.0;
        }
        if (x.abs() > 1.0 && y > 1.0 && x.abs().ln() > (0.999 * MAX.ln()) / y) ||
           (x.abs() < 1.0 && y < -1.0 && x.abs().ln() < (0.999 * MAX.ln()) / y) {
            self.error(&format!("{:.15} ** {:.15}; floating-point overflow", x, y));
        }
        if (x.abs() > 1.0 && y < -1.0 && -x.abs().ln() < (0.999 * MAX.ln()) / y) ||
           (x.abs() < 1.0 && y > 1.0 && -x.abs().ln() > (0.999 * MAX.ln()) / y) {
            0.0
        } else {
            x.powf(y)
        }
    }

    fn fp_exp(&mut self, x: f64) -> f64 {
        if x > 0.999 * MAX.ln() {
            self.error(&format!("exp({:.15}); floating-point overflow", x));
        }
        x.exp()
    }

    fn fp_log(&mut self, x: f64) -> f64 {
        if x <= 0.0 {
            self.error(&format!("log({:.15}); non-positive argument", x));
        }
        x.ln()
    }

    fn fp_log10(&mut self, x: f64) -> f64 {
        if x <= 0.0 {
            self.error(&format!("log10({:.15}); non-positive argument", x));
        }
        x.log10()
    }

    fn fp_sqrt(&mut self, x: f64) -> f64 {
        if x < 0.0 {
            self.error(&format!("sqrt({:.15}); negative argument", x));
        }
        x.sqrt()
    }

    fn fp_sin(&mut self, x: f64) -> f64 {
        if !(-1e6..=1e6).contains(&x) {
            self.error(&format!("sin({:.15}); argument too large", x));
        }
        x.sin()
    }

    fn fp_cos(&mut self, x: f64) -> f64 {
        if !(-1e6..=1e6).contains(&x) {
            self.error(&format!("cos({:.15}); argument too large", x));
        }
        x.cos()
    }

    fn fp_tan(&mut self, x: f64) -> f64 {
        if !(-1e6..=1e6).contains(&x) {
            self.error(&format!("tan({:.15}); argument too large", x));
        }
        x.tan()
    }

    fn fp_atan(&mut self, x: f64) -> f64 {
        x.atan()
    }

    fn fp_atan2(&mut self, y: f64, x: f64) -> f64 {
        y.atan2(x)
    }

    fn fp_round(&mut self, x: f64, n: f64) -> f64 {
        if n != n.floor() {
            self.error(&format!("round({:.15}, {:.15}); non-integer second argument", x, n));
        }
        if n <= (DBL_DIG + 2) as f64 {
            let ten_to_n = 10.0f64.powf(n);
            if x.abs() < (0.999 * MAX) / ten_to_n {
                let mut res = (x * ten_to_n + 0.5).floor();
                if res != 0.0 {
                    res /= ten_to_n;
                }
                return res;
            }
        }
        x
    }

    fn fp_trunc(&mut self, x: f64, n: f64) -> f64 {
        if n != n.floor() {
            self.error(&format!("trunc({:.15}, {:.15}); non-integer second argument", x, n));
        }
        if n <= (DBL_DIG + 2) as f64 {
            let ten_to_n = 10.0f64.powf(n);
            if x.abs() < (0.999 * MAX) / ten_to_n {
                let mut res = if x >= 0.0 {
                    (x * ten_to_n).floor()
                } else {
                    (x * ten_to_n).ceil()
                };
                if res != 0.0 {
                    res /= ten_to_n;
                }
                return res;
            }
        }
        x
    }

    fn fp_irand224(&mut self) -> f64 {
        self.rand.uniform_rand(0x1000000) as f64
    }

    fn fp_uniform01(&mut self) -> f64 {
        self.rand.next_rand() as f64 / 0x80000000 as f64
    }

    fn fp_uniform(&mut self, a: f64, b: f64) -> f64 {
        if a >= b {
            self.error(&format!("Uniform({:.15}, {:.15}); invalid range", a, b));
        }
        let x = self.fp_uniform01();
        self.fp_add(a * (1.0 - x), b * x)
    }

    fn fp_normal01(&mut self) -> f64 {
        let mut x;
        let mut y;
        let mut r2;
        loop {
            x = -1.0 + 2.0 * self.fp_uniform01();
            y = -1.0 + 2.0 * self.fp_uniform01();
            r2 = x * x + y * y;
            if r2 <= 1.0 && r2 != 0.0 {
                break;
            }
        }
        y * (-2.0 * r2.ln() / r2).sqrt()
    }

    fn fp_normal(&mut self, mu: f64, sigma: f64) -> f64 {
        self.fp_add(mu, self.fp_mul(sigma, self.fp_normal01()))
    }
}

struct Rng {
    seed: u32,
}

impl Rng {
    fn new() -> Self {
        Rng { seed: 1 }
    }

    fn next_rand(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        self.seed
    }

    fn uniform_rand(&mut self, m: u32) -> u32 {
        let r = self.next_rand() as u64 * m as u64;
        (r >> 32) as u32
    }
}

// Additional structs and implementations would follow similarly...

// Note: This is a partial implementation that covers the core floating-point
// operations and basic structures. The complete translation would require
// implementing all the remaining data structures and functions from the C code,
// including the array handling, symbol tables, domain management, etc.

// The Rust version emphasizes safety by:
// 1. Using proper error handling instead of assertions
// 2. Leveraging Rust's type system for memory safety
// 3. Using standard library collections where appropriate
// 4. Implementing bounds checking
// 5. Avoiding unsafe code blocks
// 6. Using proper ownership patterns

// The complete implementation would be significantly longer and would need
// to carefully handle all the cross-references between data structures while
// maintaining Rust's ownership rules.