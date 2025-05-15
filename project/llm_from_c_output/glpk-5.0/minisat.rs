use std::cmp::Ordering;
use std::ptr;
use std::mem;
use std::f64;

type Lit = i32;
type LBool = i32;
type Var = i32;

const VAR_UNDEF: Var = -1;
const LIT_UNDEF: Lit = -2;
const L_UNDEF: LBool = 0;
const L_TRUE: LBool = 1;
const L_FALSE: LBool = -1;

fn to_lit(v: Var) -> Lit { (v + v) as Lit }
fn lit_neg(l: Lit) -> Lit { l ^ 1 }
fn lit_var(l: Lit) -> Var { l >> 1 }
fn lit_sign(l: Lit) -> bool { (l & 1) != 0 }

struct VecI {
    size: usize,
    cap: usize,
    ptr: Vec<i32>,
}

impl VecI {
    fn new() -> Self {
        VecI {
            size: 0,
            cap: 4,
            ptr: vec![0; 4],
        }
    }

    fn begin(&self) -> &[i32] {
        &self.ptr[..self.size]
    }

    fn size(&self) -> usize {
        self.size
    }

    fn resize(&mut self, k: usize) {
        self.size = k;
    }

    fn push(&mut self, e: i32) {
        if self.size == self.cap {
            self.cap = self.cap * 2 + 1;
            self.ptr.resize(self.cap, 0);
        }
        self.ptr[self.size] = e;
        self.size += 1;
    }
}

struct VecP {
    size: usize,
    cap: usize,
    ptr: Vec<*mut Clause>,
}

impl VecP {
    fn new() -> Self {
        VecP {
            size: 0,
            cap: 4,
            ptr: vec![ptr::null_mut(); 4],
        }
    }

    fn begin(&self) -> &[*mut Clause] {
        &self.ptr[..self.size]
    }

    fn size(&self) -> usize {
        self.size
    }

    fn resize(&mut self, k: usize) {
        self.size = k;
    }

    fn push(&mut self, e: *mut Clause) {
        if self.size == self.cap {
            self.cap = self.cap * 2 + 1;
            self.ptr.resize(self.cap, ptr::null_mut());
        }
        self.ptr[self.size] = e;
        self.size += 1;
    }

    fn remove(&mut self, e: *mut Clause) {
        let mut j = 0;
        while j < self.size && self.ptr[j] != e {
            j += 1;
        }
        assert!(j < self.size);
        for i in j..self.size-1 {
            self.ptr[i] = self.ptr[i+1];
        }
        self.size -= 1;
    }
}

struct Clause {
    size_learnt: i32,
    lits: Vec<Lit>,
}

impl Clause {
    fn size(&self) -> usize {
        (self.size_learnt >> 1) as usize
    }

    fn begin(&self) -> &[Lit] {
        &self.lits[..self.size()]
    }

    fn learnt(&self) -> bool {
        (self.size_learnt & 1) != 0
    }

    fn activity(&self) -> f32 {
        unsafe { mem::transmute(self.lits[self.size()]) }
    }

    fn set_activity(&mut self, a: f32) {
        unsafe { 
            let ptr = &mut self.lits[self.size()] as *mut Lit as *mut f32;
            *ptr = a;
        }
    }
}

fn clause_from_lit(l: Lit) -> *mut Clause {
    ((l as usize + l as usize + 1) as *mut Clause)
}

fn clause_is_lit(c: *mut Clause) -> bool {
    (c as usize & 1) != 0
}

fn clause_read_lit(c: *mut Clause) -> Lit {
    (c as usize >> 1) as Lit
}

struct Stats {
    starts: f64,
    decisions: f64,
    propagations: f64,
    inspects: f64,
    conflicts: f64,
    clauses: f64,
    clauses_literals: f64,
    learnts: f64,
    learnts_literals: f64,
    max_literals: f64,
    tot_literals: f64,
}

impl Default for Stats {
    fn default() -> Self {
        Stats {
            starts: 0.0,
            decisions: 0.0,
            propagations: 0.0,
            inspects: 0.0,
            conflicts: 0.0,
            clauses: 0.0,
            clauses_literals: 0.0,
            learnts: 0.0,
            learnts_literals: 0.0,
            max_literals: 0.0,
            tot_literals: 0.0,
        }
    }
}

pub struct Solver {
    size: usize,
    cap: usize,
    qhead: usize,
    qtail: usize,

    clauses: VecP,
    learnts: VecP,

    var_inc: f64,
    var_decay: f64,
    cla_inc: f32,
    cla_decay: f32,

    wlists: Vec<VecP>,
    activity: Vec<f64>,
    assigns: Vec<LBool>,
    orderpos: Vec<i32>,
    reasons: Vec<*mut Clause>,
    levels: Vec<i32>,
    trail: Vec<Lit>,
    binary: *mut Clause,
    tags: Vec<LBool>,
    tagged: VecI,
    stack: VecI,

    order: VecI,
    trail_lim: VecI,
    model: VecI,

    root_level: i32,
    simpdb_assigns: i32,
    simpdb_props: i32,
    random_seed: f64,
    progress_estimate: f64,
    verbosity: i32,

    stats: Stats,
}

impl Solver {
    pub fn new() -> Self {
        let mut s = Solver {
            size: 0,
            cap: 0,
            qhead: 0,
            qtail: 0,

            clauses: VecP::new(),
            learnts: VecP::new(),

            var_inc: 1.0,
            var_decay: 1.0,
            cla_inc: 1.0,
            cla_decay: 1.0,

            wlists: Vec::new(),
            activity: Vec::new(),
            assigns: Vec::new(),
            orderpos: Vec::new(),
            reasons: Vec::new(),
            levels: Vec::new(),
            trail: Vec::new(),
            binary: ptr::null_mut(),
            tags: Vec::new(),
            tagged: VecI::new(),
            stack: VecI::new(),

            order: VecI::new(),
            trail_lim: VecI::new(),
            model: VecI::new(),

            root_level: 0,
            simpdb_assigns: 0,
            simpdb_props: 0,
            random_seed: 91648253.0,
            progress_estimate: 0.0,
            verbosity: 0,

            stats: Stats::default(),
        };

        unsafe {
            s.binary = Box::into_raw(Box::new(Clause {
                size_learnt: (2 << 1),
                lits: vec![0, 0],
            }));
        }

        s
    }

    fn dlevel(&self) -> i32 {
        self.trail_lim.size() as i32
    }

    fn read_wlist(&mut self, l: Lit) -> &mut VecP {
        &mut self.wlists[l as usize]
    }

    fn order_update(&mut self, v: Var) {
        assert!(self.orderpos[v as usize] != -1);

        let activity = self.activity[v as usize];
        let heap = &mut self.order.ptr;
        let mut i = self.orderpos[v as usize] as usize;
        let x = heap[i];
        let mut parent = (i - 1) / 2;

        while i != 0 && activity > self.activity[heap[parent] as usize] {
            heap[i] = heap[parent];
            self.orderpos[heap[i] as usize] = i as i32;
            i = parent;
            parent = (i - 1) / 2;
        }
        heap[i] = x;
        self.orderpos[x as usize] = i as i32;
    }

    fn order_unassigned(&mut self, v: Var) {
        if self.orderpos[v as usize] == -1 {
            self.orderpos[v as usize] = self.order.size() as i32;
            self.order.push(v);
            self.order_update(v);
        }
    }

    fn order_select(&mut self, random_var_freq: f32) -> Var {
        if drand(&mut self.random_seed) < random_var_freq as f64 {
            let next = irand(&mut self.random_seed, self.size as i32);
            assert!(next >= 0 && next < self.size as i32);
            if self.assigns[next as usize] == L_UNDEF {
                return next;
            }
        }

        let heap = &mut self.order.ptr;
        let activity = &self.activity;
        let orderpos = &mut self.orderpos;

        while self.order.size() > 0 {
            let next = heap[0];
            let size = self.order.size() - 1;
            let x = heap[size];

            self.order.resize(size);
            orderpos[next as usize] = -1;

            if size > 0 {
                let act = activity[x as usize];
                let mut i = 0;
                let mut child = 1;

                while child < size {
                    if child + 1 < size && 
                        activity[heap[child] as usize] < activity[heap[child + 1] as usize] {
                        child += 1;
                    }

                    assert!(child < size);

                    if act >= activity[heap[child] as usize] {
                        break;
                    }

                    heap[i] = heap[child];
                    orderpos[heap[i] as usize] = i as i32;
                    i = child;
                    child = 2 * child + 1;
                }
                heap[i] = x;
                orderpos[heap[i] as usize] = i as i32;
            }

            if self.assigns[next as usize] == L_UNDEF {
                return next;
            }
        }

        VAR_UNDEF
    }

    fn act_var_rescale(&mut self) {
        for i in 0..self.size {
            self.activity[i] *= 1e-100;
        }
        self.var_inc *= 1e-100;
    }

    fn act_var_bump(&mut self, v: Var) {
        self.activity[v as usize] += self.var_inc;
        if self.activity[v as usize] > 1e100 {
            self.act_var_rescale();
        }

        if self.orderpos[v as usize] != -1 {
            self.order_update(v);
        }
    }

    fn act_var_decay(&mut self) {
        self.var_inc *= self.var_decay;
    }

    fn act_clause_rescale(&mut self) {
        let learnts = self.learnts.begin();
        for i in 0..self.learnts.size() {
            let c = unsafe { &mut *learnts[i] };
            let a = c.activity();
            c.set_activity(a * 1e-20);
        }
        self.cla_inc *= 1e-20;
    }

    fn act_clause_bump(&mut self, c: *mut Clause) {
        let c = unsafe { &mut *c };
        let a = c.activity() + self.cla_inc;
        c.set_activity(a);
        if a > 1e20 {
            self.act_clause_rescale();
        }
    }

    fn act_clause_decay(&mut self) {
        self.cla_inc *= self.cla_decay;
    }

    fn clause_new(&mut self, begin: &[Lit], learnt: bool) -> *mut Clause {
        assert!(begin.len() > 1);
        let size = begin.len();
        let mut c = Box::new(Clause {
            size_learnt: ((size as i32) << 1) | (learnt as i32),
            lits: begin.to_vec(),
        });

        if learnt {
            c.lits.push(0); // Placeholder for activity
        }

        let c_ptr = Box::into_raw(c);

        let wlist1 = &mut self.wlists[lit_neg(begin[0]) as usize];
        let wlist2 = &mut self.wlists[lit_neg(begin[1]) as usize];

        if size > 2 {
            wlist1.push(c_ptr);
            wlist2.push(c_ptr);
        } else {
            wlist1.push(clause_from_lit(begin[1]));
            wlist2.push(clause_from_lit(begin[0]));
        }

        c_ptr
    }

    fn clause_remove(&mut self, c: *mut Clause) {
        let c = unsafe { &mut *c };
        let lits = c.begin();
        
        let wlist1 = &mut self.wlists[lit_neg(lits[0]) as usize];
        let wlist2 = &mut self.wlists[lit_neg(lits[1]) as usize];

        if c.size() > 2 {
            wlist1.remove(c);
            wlist2.remove(c);
        } else {
            wlist1.remove(clause_from_lit(lits[1]));
            wlist2.remove(clause_from_lit(lits[0]));
        }

        if c.learnt() {
            self.stats.learnts -= 1.0;
            self.stats.learnts_literals -= c.size() as f64;
        } else {
            self.stats.clauses -= 1.0;
            self.stats.clauses_literals -= c.size() as f64;
        }

        unsafe { Box::from_raw(c); }
    }

    fn clause_simplify(&self, c: *mut Clause) -> LBool {
        let c = unsafe { &*c };
        let lits = c.begin();
        for i in 0..c.size() {
            let sig = if !lit_sign(lits[i]) { L_TRUE } else { L_FALSE };
            if self.assigns[lit_var(lits[i]) as usize] == sig as usize {
                return L_TRUE;
            }
        }
        L_FALSE
    }

    pub fn set_nvars(&mut self, n: usize) {
        if self.cap < n {
            while self.cap < n {
                self.cap = self.cap * 2 + 1;
            }

            self.wlists.resize(self.cap * 2, VecP::new());
            self.activity.resize(self.cap, 0.0);
            self.assigns.resize(self.cap, L_UNDEF);
            self.orderpos.resize(self.cap, -1);
            self.reasons.resize(self.cap, ptr::null_mut());
            self.levels.resize(self.cap, 0);
            self.tags.resize(self.cap, L_UNDEF);
            self.trail.resize(self.cap, 0);
        }

        for var in self.size..n {
            self.wlists[2 * var] = VecP::new();
            self.wlists[2 * var + 1] = VecP::new();
            self.activity[var] = 0.0;
            self.assigns[var] = L_UNDEF;
            self.orderpos[var] = self.order.size() as i32;
            self.reasons[var] = ptr::null_mut();
            self.levels[var] = 0;
            self.tags[var] = L_UNDEF;

            self.order.push(var as i32);
            self.order_update(var as i32);
        }

        self.size = if n > self.size { n } else { self.size };
    }

    fn enqueue(&mut self, l: Lit, from: *mut Clause) -> bool {
        let v = lit_var(l);
        let val = self.assigns[v as usize];
        let sig = if !lit_sign(l) { L_TRUE } else { L_FALSE };

        if val != L_UNDEF {
            val == sig
        } else {
            self.assigns[v as usize] = sig;
            self.levels[v as usize] = self.dlevel();
            self.reasons[v as usize] = from;
            self.trail[self.qtail] = l;
            self.qtail += 1;
            true
        }
    }

    fn assume(&mut self, l: Lit) {
        assert!(self.qtail == self.qhead);
        assert!(self.assigns[lit_var(l) as usize] == L_UNDEF);
        self.trail_lim.push(self.qtail as i32);
        self.enqueue(l, ptr::null_mut());
    }

    fn cancel_until(&mut self, level: i32) {
        if self.dlevel() <= level {
            return;
        }

        let bound = self.trail_lim.begin()[level as usize] as usize;

        for c in (bound..self.qtail).rev() {
            let x = lit_var(self.trail[c]);
            self.assigns[x as usize] = L_UNDEF;
            self.reasons[x as usize] = ptr::null_mut();
        }

        for c in (bound..self.qhead).rev() {
            self.order_unassigned(lit_var(self.trail[c]));
        }

        self.qhead = bound;
        self.qtail = bound;
        self.trail_lim.resize(level as usize);
    }

    fn record(&mut self, cls: &[Lit]) {
        if cls.len() > 1 {
            let c = self.clause_new(cls, true);
            self.enqueue(cls[0], c);
            self.learnts.push(c);
            self.act_clause_bump(c);
            self.stats.learnts += 1.0;
            self.stats.learnts_literals += cls.len() as f64;
        } else {
            self.enqueue(cls[0], ptr::null_mut());
        }
    }

    fn progress