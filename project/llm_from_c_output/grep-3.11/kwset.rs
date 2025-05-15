use std::collections::HashMap;
use std::ptr;
use std::mem;
use std::cmp;

const NCHAR: usize = 256;
const DEPTH_SIZE: usize = 12; // CHAR_BIT + CHAR_BIT / 2 for CHAR_BIT >= 4

#[derive(Debug)]
pub struct KwSMatch {
    pub index: isize,
    pub offset: isize,
    pub size: isize,
}

#[derive(Debug)]
struct Tree {
    llink: *mut Tree,
    rlink: *mut Tree,
    trie: *mut Trie,
    label: u8,
    balance: i8,
}

#[derive(Debug)]
struct Trie {
    accepting: isize,
    links: *mut Tree,
    parent: *mut Trie,
    next: *mut Trie,
    fail: *mut Trie,
    depth: isize,
    shift: isize,
    maxshift: isize,
}

#[derive(Debug)]
pub struct KwSet {
    words: isize,
    trie: *mut Trie,
    mind: isize,
    delta: [u8; NCHAR],
    next: [*mut Trie; NCHAR],
    target: *mut u8,
    shift: *mut isize,
    trans: *const u8,
    gc1: i32,
    gc1help: i32,
    gc2: u8,
    kwsexec: fn(&KwSet, *const u8, isize, *mut KwSMatch, bool) -> isize,
}

impl KwSet {
    pub fn new(trans: *const u8) -> *mut KwSet {
        unsafe {
            let kwset = Box::into_raw(Box::new(KwSet {
                words: 0,
                trie: ptr::null_mut(),
                mind: isize::MAX,
                delta: [0; NCHAR],
                next: [ptr::null_mut(); NCHAR],
                target: ptr::null_mut(),
                shift: ptr::null_mut(),
                trans,
                gc1: -2,
                gc1help: -1,
                gc2: 0,
                kwsexec: acexec,
            }));

            (*kwset).trie = Box::into_raw(Box::new(Trie {
                accepting: 0,
                links: ptr::null_mut(),
                parent: ptr::null_mut(),
                next: ptr::null_mut(),
                fail: ptr::null_mut(),
                depth: 0,
                shift: 0,
                maxshift: 0,
            }));

            kwset
        }
    }

    pub fn incr(&mut self, text: *const u8, len: isize) {
        unsafe {
            let mut trie = self.trie;
            let trans = self.trans;
            let reverse = self.kwsexec == bmexec;

            let mut text_ptr = text;
            if reverse {
                text_ptr = text_ptr.offset(len);
            }

            for _ in 0..len {
                let uc = if reverse {
                    text_ptr = text_ptr.offset(-1);
                    *text_ptr
                } else {
                    let c = *text_ptr;
                    text_ptr = text_ptr.offset(1);
                    c
                };

                let label = if !trans.is_null() {
                    *trans.offset(uc as isize)
                } else {
                    uc
                };

                let mut cur = (*trie).links;
                let mut links: [*mut Tree; DEPTH_SIZE] = [ptr::null_mut(); DEPTH_SIZE];
                let mut dirs: [u8; DEPTH_SIZE] = [0; DEPTH_SIZE];
                links[0] = &mut (*trie).links as *mut _ as *mut Tree;
                dirs[0] = b'L';
                let mut depth = 1;

                while !cur.is_null() && label != (*cur).label {
                    links[depth] = cur;
                    if label < (*cur).label {
                        dirs[depth] = b'L';
                        depth += 1;
                        cur = (*cur).llink;
                    } else {
                        dirs[depth] = b'R';
                        depth += 1;
                        cur = (*cur).rlink;
                    }
                }

                if cur.is_null() {
                    cur = Box::into_raw(Box::new(Tree {
                        llink: ptr::null_mut(),
                        rlink: ptr::null_mut(),
                        trie: Box::into_raw(Box::new(Trie {
                            accepting: 0,
                            links: ptr::null_mut(),
                            parent: trie,
                            next: ptr::null_mut(),
                            fail: ptr::null_mut(),
                            depth: (*trie).depth + 1,
                            shift: 0,
                            maxshift: 0,
                        })),
                        label,
                        balance: 0,
                    }));

                    depth -= 1;
                    if dirs[depth] == b'L' {
                        (*links[depth]).llink = cur;
                    } else {
                        (*links[depth]).rlink = cur;
                    }

                    while depth > 0 && (*links[depth]).balance == 0 {
                        if dirs[depth] == b'L' {
                            (*links[depth]).balance -= 1;
                        } else {
                            (*links[depth]).balance += 1;
                        }
                        depth -= 1;
                    }

                    if depth > 0
                        && ((dirs[depth] == b'L' && (*links[depth]).balance == -2)
                        || (dirs[depth] == b'R' && (*links[depth]).balance == 2)
                    {
                        let mut t: *mut Tree = ptr::null_mut();
                        let mut r: *mut Tree = ptr::null_mut();
                        let mut l: *mut Tree = ptr::null_mut();
                        let mut rl: *mut Tree = ptr::null_mut();
                        let mut lr: *mut Tree = ptr::null_mut();

                        match (*links[depth]).balance {
                            -2 => match dirs[depth + 1] {
                                b'L' => {
                                    r = links[depth];
                                    t = (*r).llink;
                                    rl = (*t).rlink;
                                    (*t).rlink = r;
                                    (*r).llink = rl;
                                    (*t).balance = 0;
                                    (*r).balance = 0;
                                }
                                b'R' => {
                                    r = links[depth];
                                    l = (*r).llink;
                                    t = (*l).rlink;
                                    rl = (*t).rlink;
                                    lr = (*t).llink;
                                    (*t).llink = l;
                                    (*l).rlink = lr;
                                    (*t).rlink = r;
                                    (*r).llink = rl;
                                    (*l).balance = if (*t).balance != 1 { 0 } else { -1 };
                                    (*r).balance = if (*t).balance != -1 { 0 } else { 1 };
                                    (*t).balance = 0;
                                }
                                _ => panic!("Invalid direction"),
                            },
                            2 => match dirs[depth + 1] {
                                b'R' => {
                                    l = links[depth];
                                    t = (*l).rlink;
                                    lr = (*t).llink;
                                    (*t).llink = l;
                                    (*l).rlink = lr;
                                    (*t).balance = 0;
                                    (*l).balance = 0;
                                }
                                b'L' => {
                                    l = links[depth];
                                    r = (*l).rlink;
                                    t = (*r).llink;
                                    lr = (*t).llink;
                                    rl = (*t).rlink;
                                    (*t).llink = l;
                                    (*l).rlink = lr;
                                    (*t).rlink = r;
                                    (*r).llink = rl;
                                    (*l).balance = if (*t).balance != 1 { 0 } else { -1 };
                                    (*r).balance = if (*t).balance != -1 { 0 } else { 1 };
                                    (*t).balance = 0;
                                }
                                _ => panic!("Invalid direction"),
                            },
                            _ => panic!("Invalid balance"),
                        }

                        if dirs[depth - 1] == b'L' {
                            (*links[depth - 1]).llink = t;
                        } else {
                            (*links[depth - 1]).rlink = t;
                        }
                    }
                }

                trie = (*cur).trie;
            }

            if (*trie).accepting == 0 {
                (*trie).accepting = 2 * self.words + 1;
            }
            self.words += 1;

            if (*trie).depth < self.mind {
                self.mind = (*trie).depth;
            }
        }
    }

    pub fn words(&self) -> isize {
        self.words
    }

    pub fn prep(&mut self) {
        unsafe {
            let trans = self.trans;
            let mut delta = [self.mind.min(u8::MAX as isize) as u8; NCHAR];
            let mut curr = self.trie;
            let mut last = self.trie;

            let reverse = self.words == 1;

            if reverse {
                let mut new_kwset = KwSet::new(self.trans);
                (*new_kwset).kwsexec = bmexec;

                for curr in &mut (*self.trie).next {
                    enqueue(*curr, &mut last);
                }

                (*self).target = Box::into_raw(Box::new(0));
                curr = self.trie;
                for i in 0..self.mind {
                    *(*self).target.offset(i) = (*(*curr).links).label;
                    curr = (*curr).next;
                }

                (*new_kwset).incr((*self).target, self.mind);
                *self = *Box::from_raw(new_kwset);
            }

            for curr in &mut (*self.trie).next {
                enqueue(*curr, &mut last);
                treedelta(*curr, (*curr).depth, &mut delta);
                treefails(*curr, (*curr).fail, self.trie, reverse);
            }

            if reverse {
                let mut nextbuf: [*mut Trie; NCHAR] = [ptr::null_mut(); NCHAR];
                treenext((*self.trie).links, &mut nextbuf);
                let mut gc1 = -2;
                let mut gc1help = -1;

                for i in 0..NCHAR {
                    let ti = if !trans.is_null() {
                        *trans.offset(i as isize) as usize
                    } else {
                        i
                    };
                    (*self).next[i] = nextbuf[ti];

                    if !(*self).next[i].is_null() {
                        if gc1 < -1 {
                            gc1 = ti as i32;
                            gc1help = i as i32;
                        } else if gc1 == ti as i32 {
                            gc1help = if gc1help == ti as i32 { i as i32 } else { -1 };
                        } else if i == ti && gc1 == gc1help {
                            gc1help = i as i32;
                        } else {
                            gc1 = -1;
                        }
                    }
                }

                (*self).gc1 = gc1;
                (*self).gc1help = gc1help;

                (*self).target = Box::into_raw(Box::new(0));
                curr = self.trie;
                for i in (0..self.mind).rev() {
                    *(*self).target.offset(i) = (*(*curr).links).label;
                    curr = (*curr).next;
                }

                if self.mind > 1 {
                    (*self).shift = Box::into_raw(Box::new(0));
                    curr = (*self.trie).next;
                    for i in 0..self.mind - 1 {
                        *(*self).shift.offset(i) = (*curr).shift;
                        curr = (*curr).next;
                    }

                    (*self).gc2 = tr(trans, *(*self).target.offset(self.mind - 2));
                }
            }

            if !trans.is_null() {
                for i in 0..NCHAR {
                    (*self).delta[i] = delta[*trans.offset(i as isize) as usize];
                }
            }
        }
    }

    pub fn exec(
        &self,
        text: *const u8,
        size: isize,
        kwsmatch: *mut KwSMatch,
        longest: bool,
    ) -> isize {
        let func = self.kwsexec;
        func(self, text, size, kwsmatch, longest)
    }
}

pub fn kwsalloc(trans: *const u8) -> *mut KwSet {
    KwSet::new(trans)
}

pub fn kwsincr(kwset: *mut KwSet, text: *const u8, len: isize) {
    unsafe {
        (*kwset).incr(text, len);
    }
}

pub fn kwswords(kwset: *const KwSet) -> isize {
    unsafe { (*kwset).words() }
}

pub fn kwsprep(kwset: *mut KwSet) {
    unsafe {
        (*kwset).prep();
    }
}

pub fn kwsexec(
    kwset: *const KwSet,
    text: *const u8,
    size: isize,
    kwsmatch: *mut KwSMatch,
    longest: bool,
) -> isize {
    unsafe { (*kwset).exec(text, size, kwsmatch, longest) }
}

pub fn kwsfree(kwset: *mut KwSet) {
    unsafe {
        drop(Box::from_raw(kwset));
    }
}

unsafe fn enqueue(tree: *mut Tree, last: &mut *mut Trie) {
    if tree.is_null() {
        return;
    }
    enqueue((*tree).llink, last);
    enqueue((*tree).rlink, last);
    (**last).next = (*tree).trie;
    *last = (*tree).trie;
}

unsafe fn treefails(
    tree: *const Tree,
    fail: *const Trie,
    recourse: *mut Trie,
    reverse: bool,
) {
    if tree.is_null() {
        return;
    }
    treefails((*tree).llink, fail, recourse, reverse);
    treefails((*tree).rlink, fail, recourse, reverse);

    let mut fail_ptr = fail;
    while !fail_ptr.is_null() {
        let mut cur = (*fail_ptr).links;
        while !cur.is_null() && (*tree).label != (*cur).label {
            cur = if (*tree).label < (*cur).label {
                (*cur).llink
            } else {
                (*cur).rlink
            };
        }
        if !cur.is_null() {
            (*(*tree).trie).fail = (*cur).trie;
            if !reverse && (*cur).trie.accepting != 0 && (*tree).trie.accepting == 0 {
                (*(*tree).trie).accepting = -1;
            }
            return;
        }
        fail_ptr = (*fail_ptr).fail;
    }

    (*(*tree).trie).fail = recourse;
}

unsafe fn treedelta(tree: *const Tree, depth: isize, delta: &mut [u8]) {
    if tree.is_null() {
        return;
    }
    treedelta((*tree).llink, depth, delta);
    treedelta((*tree).rlink, depth, delta);
    if depth < delta[(*tree).label as usize] as isize {
        delta[(*tree).label as usize] = depth as u8;
    }
}

unsafe fn hasevery(a: *const Tree, b: *const Tree) -> bool {
    if b.is_null() {
        return true;
    }
    if !hasevery(a, (*b).llink) {
        return false;
    }
    if !hasevery(a, (*b).rlink) {
        return false;
    }
    let mut a_ptr = a;
    while !a_ptr.is_null() && (*b).label != (*a_ptr).label {
        a_ptr = if (*b).label < (*a_ptr).label {
            (*a_ptr).llink
        } else {
            (*a_ptr).rlink
        };
    }
    !a_ptr.is_null()
}

unsafe fn treenext(tree: *const Tree, next: &mut [*mut Trie]) {
    if tree.is_null() {
        return;
    }
    treenext((*tree).llink, next);
    treenext((*tree).rlink, next);
    next[(*tree).label as usize] = (*tree).trie;
}

unsafe fn bm_delta2_search(
    tpp: &mut *const u8,
    ep: *const u8,
    sp: *const u8,
    len: isize,
    trans: *const u8,
    gc1: u8,
    gc2: u8,
    d1: *const u8,
    kwset: *const KwSet,
) -> bool {
    let mut tp = *tpp;
    let mut d = len;
    let mut skip = 0;

    loop {
        let mut i = 2;
        if tr(trans, *tp.offset(-2)) == gc2 {
            i += 1;
            while i <= d {
                if tr(trans, *tp.offset(-i as isize)) != tr(trans, *sp.offset(-i as isize)) {
                    break;
                }
                i += 1;
            }
            if i > d {
                i = d + skip + 1;
                while i <= len {
                    if tr(trans, *tp.offset(-i as isize)) != tr(trans, *sp.offset(-i as isize)) {
                        break;
                    }
                    i += 1;
                }
                if i > len {
                    *tpp = tp.offset(-len);
                    return true;
                }
            }
        }

        d = (*(*kwset).shift.offset(i as isize - 2)) as isize;
        tp = tp.offset(d);
        if tp > ep {
            break;
        }
        if tr(trans, *tp.offset(-1)) != gc1 {
            if !d1.is_null() {
                tp = tp.offset(*d1.offset((*