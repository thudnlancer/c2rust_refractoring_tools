use std::collections::HashMap;
use std::mem;
use std::ops::{BitAnd, BitOr, BitOrAssign, Not};
use std::ptr;

const NOTCHAR: usize = 1 << 8;
const CHARCLASS_WORD_BITS: usize = 64;
const CHARCLASS_WORDS: usize = (NOTCHAR + CHARCLASS_WORD_BITS - 1) / CHARCLASS_WORD_BITS;

#[derive(Clone, Copy, Debug)]
struct CharclassWord(u64);

#[derive(Clone, Debug)]
struct Charclass {
    w: [CharclassWord; CHARCLASS_WORDS],
}

impl Charclass {
    fn new() -> Self {
        Charclass {
            w: [CharclassWord(0); CHARCLASS_WORDS],
        }
    }

    fn setbit(&mut self, b: usize) {
        let idx = b / CHARCLASS_WORD_BITS;
        let bit = b % CHARCLASS_WORD_BITS;
        self.w[idx].0 |= 1 << bit;
    }

    fn clrbit(&mut self, b: usize) {
        let idx = b / CHARCLASS_WORD_BITS;
        let bit = b % CHARCLASS_WORD_BITS;
        self.w[idx].0 &= !(1 << bit);
    }

    fn tstbit(&self, b: usize) -> bool {
        let idx = b / CHARCLASS_WORD_BITS;
        let bit = b % CHARCLASS_WORD_BITS;
        (self.w[idx].0 >> bit) & 1 != 0
    }

    fn zeroset(&mut self) {
        for word in &mut self.w {
            word.0 = 0;
        }
    }

    fn fillset(&mut self) {
        for word in &mut self.w {
            word.0 = !0;
        }
    }

    fn notset(&mut self) {
        for word in &mut self.w {
            word.0 = !word.0;
        }
    }

    fn equal(&self, other: &Self) -> bool {
        self.w.iter().zip(other.w.iter()).all(|(a, b)| a.0 == b.0)
    }

    fn emptyset(&self) -> bool {
        self.w.iter().all(|word| word.0 == 0)
    }
}

impl BitAnd for Charclass {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut res = self;
        for (a, b) in res.w.iter_mut().zip(rhs.w.iter()) {
            a.0 &= b.0;
        }
        res
    }
}

impl BitOr for Charclass {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut res = self;
        for (a, b) in res.w.iter_mut().zip(rhs.w.iter()) {
            a.0 |= b.0;
        }
        res
    }
}

impl BitOrAssign for Charclass {
    fn bitor_assign(&mut self, rhs: Self) {
        for (a, b) in self.w.iter_mut().zip(rhs.w.iter()) {
            a.0 |= b.0;
        }
    }
}

impl Not for Charclass {
    type Output = Self;

    fn not(self) -> Self::Output {
        let mut res = self;
        res.notset();
        res
    }
}

#[derive(Clone, Copy, Debug)]
struct Position {
    index: usize,
    constraint: u32,
}

#[derive(Clone, Debug)]
struct PositionSet {
    elems: Vec<Position>,
    nelem: usize,
}

impl PositionSet {
    fn new() -> Self {
        PositionSet {
            elems: Vec::new(),
            nelem: 0,
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        PositionSet {
            elems: Vec::with_capacity(capacity),
            nelem: 0,
        }
    }

    fn copy(&mut self, src: &Self) {
        self.elems.clear();
        self.elems.extend_from_slice(&src.elems[..src.nelem]);
        self.nelem = src.nelem;
    }

    fn insert(&mut self, pos: Position) {
        match self.elems[..self.nelem].binary_search_by(|p| p.index.cmp(&pos.index)) {
            Ok(idx) => {
                self.elems[idx].constraint |= pos.constraint;
            }
            Err(idx) => {
                self.elems.insert(idx, pos);
                self.nelem += 1;
            }
        }
    }

    fn append(&mut self, pos: Position) {
        self.elems.push(pos);
        self.nelem += 1;
    }

    fn merge(&mut self, s1: &Self, s2: &Self) {
        let mut i = 0;
        let mut j = 0;
        self.elems.clear();

        while i < s1.nelem || j < s2.nelem {
            if j >= s2.nelem || (i < s1.nelem && s1.elems[i].index <= s2.elems[j].index) {
                let mut c = 0;
                if i < s1.nelem && j < s2.nelem && s1.elems[i].index == s2.elems[j].index {
                    c = s2.elems[j].constraint;
                    j += 1;
                }
                self.elems.push(Position {
                    index: s1.elems[i].index,
                    constraint: s1.elems[i].constraint | c,
                });
                i += 1;
            } else {
                if s2.elems[j].constraint != 0 {
                    self.elems.push(Position {
                        index: s2.elems[j].index,
                        constraint: s2.elems[j].constraint,
                    });
                }
                j += 1;
            }
        }
        self.nelem = self.elems.len();
    }

    fn merge_constrained(&mut self, s1: &Self, s2: &Self, c2: u32) {
        let mut i = 0;
        let mut j = 0;
        self.elems.clear();

        while i < s1.nelem || j < s2.nelem {
            if j >= s2.nelem || (i < s1.nelem && s1.elems[i].index <= s2.elems[j].index) {
                let mut c = 0;
                if i < s1.nelem && j < s2.nelem && s1.elems[i].index == s2.elems[j].index {
                    c = s2.elems[j].constraint & c2;
                    j += 1;
                }
                self.elems.push(Position {
                    index: s1.elems[i].index,
                    constraint: s1.elems[i].constraint | c,
                });
                i += 1;
            } else {
                if (s2.elems[j].constraint & c2) != 0 {
                    self.elems.push(Position {
                        index: s2.elems[j].index,
                        constraint: s2.elems[j].constraint & c2,
                    });
                }
                j += 1;
            }
        }
        self.nelem = self.elems.len();
    }
}

#[derive(Debug)]
struct DfaState {
    hash: usize,
    elems: PositionSet,
    context: u8,
    constraint: u16,
    mbps: PositionSet,
    mb_trindex: isize,
}

impl DfaState {
    fn new() -> Self {
        DfaState {
            hash: 0,
            elems: PositionSet::new(),
            context: 0,
            constraint: 0,
            mbps: PositionSet::new(),
            mb_trindex: -1,
        }
    }
}

#[derive(Debug)]
struct Dfa {
    charclasses: Vec<Charclass>,
    cindex: usize,
    calloc: usize,
    canychar: isize,
    tokens: Vec<i32>,
    tindex: usize,
    talloc: usize,
    depth: usize,
    nleaves: usize,
    nregexps: usize,
    fast: bool,
    epsilon: bool,
    utf8_anychar_classes: [i32; 9],
    multibyte_prop: Vec<u8>,
    states: Vec<DfaState>,
    sindex: usize,
    salloc: usize,
    follows: Vec<PositionSet>,
    searchflag: bool,
    constraints: Vec<i32>,
    separates: Vec<u8>,
    tralloc: usize,
    trcount: usize,
    min_trcount: usize,
    trans: Vec<Vec<i32>>,
    fails: Vec<Vec<i32>>,
    success: Vec<u8>,
    newlines: Vec<i32>,
    initstate_notbol: i32,
    mb_follows: PositionSet,
    mb_trans: Vec<Vec<i32>>,
    mb_trcount: usize,
    superset: Option<Box<Dfa>>,
    dfaexec: fn(&Dfa, &[u8], &mut usize, bool, &mut usize, &mut bool) -> bool,
}

impl Dfa {
    fn new() -> Self {
        Dfa {
            charclasses: Vec::new(),
            cindex: 0,
            calloc: 0,
            canychar: -1,
            tokens: Vec::new(),
            tindex: 0,
            talloc: 0,
            depth: 0,
            nleaves: 0,
            nregexps: 0,
            fast: false,
            epsilon: false,
            utf8_anychar_classes: [0; 9],
            multibyte_prop: Vec::new(),
            states: Vec::new(),
            sindex: 0,
            salloc: 0,
            follows: Vec::new(),
            searchflag: false,
            constraints: Vec::new(),
            separates: Vec::new(),
            tralloc: 0,
            trcount: 0,
            min_trcount: 0,
            trans: Vec::new(),
            fails: Vec::new(),
            success: Vec::new(),
            newlines: Vec::new(),
            initstate_notbol: 0,
            mb_follows: PositionSet::new(),
            mb_trans: Vec::new(),
            mb_trcount: 0,
            superset: None,
            dfaexec: |_, _, _, _, _, _| false,
        }
    }
}

fn dfaalloc() -> Box<Dfa> {
    Box::new(Dfa::new())
}

fn dfasyntax(dfa: &mut Dfa, _linfo: ()) {
    dfa.dfaexec = |_, _, _, _, _, _| false;
    dfa.canychar = -1;
}

fn dfacopysyntax(to: &mut Dfa, from: &Dfa) {
    to.canychar = from.canychar;
    to.fast = from.fast;
    to.dfaexec = from.dfaexec;
}

fn dfafree(mut dfa: Box<Dfa>) {
    if let Some(superset) = dfa.superset.take() {
        dfafree(superset);
    }
}

fn dfaparse(_s: &str, _len: usize, _d: &mut Dfa) {}

fn dfacomp(_s: &str, _len: usize, _d: &mut Dfa, _searchflag: bool) {}

fn dfaexec(_d: &Dfa, _begin: &[u8], _end: &mut usize, _allow_nl: bool, _count: &mut usize, _backref: &mut bool) -> bool {
    false
}

fn dfamust(_d: &Dfa) -> Option<String> {
    None
}

fn dfamustfree(_dm: Option<String>) {}

fn dfaisfast(_d: &Dfa) -> bool {
    false
}

fn dfasupported(_d: &Dfa) -> bool {
    false
}

fn dfasuperset(_d: &Dfa) -> Option<Box<Dfa>> {
    None
}