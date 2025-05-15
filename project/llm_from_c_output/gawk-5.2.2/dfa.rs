use std::collections::{HashMap, HashSet};
use std::mem;
use std::ops::{BitAnd, BitOr, BitOrAssign, BitAndAssign, Not};
use std::ptr;
use std::slice;

#[derive(Debug, Clone)]
struct Position {
    index: usize,
    constraint: u32,
}

#[derive(Debug, Clone)]
struct PositionSet {
    elems: Vec<Position>,
    nelem: usize,
}

impl PositionSet {
    fn new() -> Self {
        Self {
            elems: Vec::new(),
            nelem: 0,
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            elems: Vec::with_capacity(capacity),
            nelem: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    End,
    Char(u8),
    Empty,
    Qmark,
    Star,
    Plus,
    Repmn,
    Cat,
    Or,
    Lparen,
    Rparen,
    Wchar(char),
    AnyChar,
    Beg,
    Begline,
    Endline,
    Begword,
    Endword,
    Limword,
    Notlimword,
    Backref,
    Mbcset,
    Cset(usize),
}

#[derive(Debug, Clone)]
struct DfaState {
    hash: u64,
    elems: PositionSet,
    context: u8,
    constraint: u16,
    mbps: PositionSet,
    mb_trindex: i32,
}

#[derive(Debug, Clone)]
struct Dfa {
    charclasses: Vec<CharClass>,
    cindex: usize,
    calloc: usize,
    canychar: i32,
    tokens: Vec<Token>,
    tindex: usize,
    talloc: usize,
    depth: usize,
    nleaves: usize,
    nregexps: usize,
    fast: bool,
    epsilon: bool,
    utf8_anychar_classes: [usize; 9],
    multibyte_prop: Vec<u8>,
    superset: Option<Box<Dfa>>,
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
    syntax: Syntax,
    localeinfo: LocaleInfo,
}

#[derive(Debug, Clone, Copy)]
struct Syntax {
    syntax_bits: u32,
    dfaopts: i32,
    syntax_bits_set: bool,
    case_fold: bool,
    eolbyte: u8,
    newline: CharClass,
    letters: CharClass,
    never_trail: [bool; 256],
    sbit: [u8; 256],
}

#[derive(Debug, Clone, Copy)]
struct LocaleInfo {
    multibyte: bool,
    using_utf8: bool,
    sbctowc: [char; 256],
}

#[derive(Debug, Clone, Copy, Default)]
struct CharClass {
    w: [u64; 4],
}

impl BitAnd for CharClass {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        let mut res = Self::default();
        for i in 0..4 {
            res.w[i] = self.w[i] & rhs.w[i];
        }
        res
    }
}

impl BitOr for CharClass {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        let mut res = Self::default();
        for i in 0..4 {
            res.w[i] = self.w[i] | rhs.w[i];
        }
        res
    }
}

impl BitOrAssign for CharClass {
    fn bitor_assign(&mut self, rhs: Self) {
        for i in 0..4 {
            self.w[i] |= rhs.w[i];
        }
    }
}

impl BitAndAssign for CharClass {
    fn bitand_assign(&mut self, rhs: Self) {
        for i in 0..4 {
            self.w[i] &= rhs.w[i];
        }
    }
}

impl Not for CharClass {
    type Output = Self;

    fn not(self) -> Self {
        let mut res = Self::default();
        for i in 0..4 {
            res.w[i] = !self.w[i];
        }
        res
    }
}

impl CharClass {
    fn zero() -> Self {
        Self::default()
    }

    fn fill() -> Self {
        Self {
            w: [u64::MAX; 4],
        }
    }

    fn set_bit(&mut self, b: u8) {
        self.w[b as usize / 64] |= 1 << (b % 64);
    }

    fn clear_bit(&mut self, b: u8) {
        self.w[b as usize / 64] &= !(1 << (b % 64));
    }

    fn test_bit(&self, b: u8) -> bool {
        (self.w[b as usize / 64] >> (b % 64)) & 1 != 0
    }

    fn is_empty(&self) -> bool {
        self.w.iter().all(|&x| x == 0)
    }
}

fn dfaalloc() -> Box<Dfa> {
    Box::new(Dfa {
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
        superset: None,
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
        syntax: Syntax {
            syntax_bits: 0,
            dfaopts: 0,
            syntax_bits_set: false,
            case_fold: false,
            eolbyte: 0,
            newline: CharClass::zero(),
            letters: CharClass::zero(),
            never_trail: [false; 256],
            sbit: [0; 256],
        },
        localeinfo: LocaleInfo {
            multibyte: false,
            using_utf8: false,
            sbctowc: ['\0'; 256],
        },
    })
}

fn dfasyntax(dfa: &mut Dfa, linfo: &LocaleInfo, bits: u32, dfaopts: i32) {
    dfa.localeinfo = *linfo;
    dfa.fast = !dfa.localeinfo.multibyte;

    dfa.canychar = -1;
    dfa.syntax.syntax_bits_set = true;
    dfa.syntax.case_fold = (bits & 1) != 0; // RE_ICASE
    dfa.syntax.eolbyte = if dfaopts & 2 != 0 { 0 } else { b'\n' }; // DFA_EOL_NUL
    dfa.syntax.syntax_bits = bits;
    dfa.syntax.dfaopts = dfaopts;

    for i in 0..=255 {
        let uc = i as u8;
        dfa.syntax.sbit[uc as usize] = char_context(dfa, uc);
        match dfa.syntax.sbit[uc as usize] {
            2 => dfa.syntax.letters.set_bit(uc), // CTX_LETTER
            4 => dfa.syntax.newline.set_bit(uc), // CTX_NEWLINE
            _ => (),
        }

        dfa.syntax.never_trail[uc as usize] = if dfa.localeinfo.using_utf8 {
            (uc & 0xc0) != 0x80
        } else {
            b"\n\r./\0".contains(&uc)
        };
    }
}

fn char_context(dfa: &Dfa, c: u8) -> u8 {
    if c == dfa.syntax.eolbyte && (dfa.syntax.dfaopts & 1) == 0 {
        4 // CTX_NEWLINE
    } else if unibyte_word_constituent(dfa, c) {
        2 // CTX_LETTER
    } else {
        1 // CTX_NONE
    }
}

fn unibyte_word_constituent(dfa: &Dfa, c: u8) -> bool {
    dfa.localeinfo.sbctowc[c as usize] != '\0' && (c.is_ascii_alphanumeric() || c == b'_')
}

fn dfacopysyntax(to: &mut Dfa, from: &Dfa) {
    to.canychar = -1;
    to.fast = from.fast;
    to.syntax = from.syntax;
    to.localeinfo = from.localeinfo;
}

// Additional helper functions and implementations would go here...