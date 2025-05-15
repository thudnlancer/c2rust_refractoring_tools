use std::ptr;
use std::mem;
use std::ffi::CString;

#[derive(Debug, Clone)]
struct GlpGraph {
    pool: Option<Box<[u8]>>,
    name: Option<CString>,
    nv_max: i32,
    nv: i32,
    na: i32,
    v: Vec<Option<Box<GlpVertex>>>,
    index: Option<Box<[u8]>>,
    v_size: i32,
    a_size: i32,
}

#[derive(Debug, Clone)]
struct GlpVertex {
    i: i32,
    name: Option<CString>,
    entry: Option<Box<[u8]>>,
    data: Option<Box<[u8]>>,
    temp: Option<Box<[u8]>>,
    in_0: Vec<GlpArc>,
    out: Vec<GlpArc>,
}

#[derive(Debug, Clone)]
struct GlpArc {
    tail: Box<GlpVertex>,
    head: Box<GlpVertex>,
    data: Option<Box<[u8]>>,
    temp: Option<Box<[u8]>>,
    t_prev: Option<Box<GlpArc>>,
    t_next: Option<Box<GlpArc>>,
    h_prev: Option<Box<GlpArc>>,
    h_next: Option<Box<GlpArc>>,
}

#[derive(Debug, Clone)]
struct Set {
    size: i32,
    list: Vec<i32>,
    pos: Vec<i32>,
}

fn glp_kellerman(
    n: i32,
    func: impl Fn(&mut dyn std::any::Any, i32, &mut [i32]) -> i32,
    info: &mut dyn std::any::Any,
    h: &mut GlpGraph,
) -> i32 {
    assert!(n >= 0, "n must be non-negative");

    h.erase_graph();
    h.add_vertices(n);

    let mut w = Set {
        size: 0,
        list: vec![0; (n + 1) as usize],
        pos: vec![0; (n + 1) as usize],
    };

    let mut v = Set {
        size: 0,
        list: vec![0; (n + 1) as usize],
        pos: vec![0; (n + 1) as usize],
    };

    for i in 1..=n {
        assert_eq!(w.size, 0, "W.size must be 0 at start of iteration");

        let len = func(info, i, &mut w.list[1..]);
        assert!((0..=n).contains(&len), "len must be between 0 and n");

        for t in 1..=len {
            let j = w.list[t as usize];
            assert!((1..=n).contains(&j), "j must be between 1 and n");

            if j < i {
                assert_eq!(w.pos[j as usize], 0, "W.pos[j] must be 0");
                w.size += 1;
                w.list[w.size as usize] = j;
                w.pos[j as usize] = w.size;
            }
        }

        if w.size == 0 {
            let k = h.add_vertices(1) - n;
            h.add_arc(i, n + k);
        } else {
            assert_eq!(v.size, 0, "V.size must be 0 at start of iteration");
            let k = h.nv - n;

            for m in 1..=k {
                if v.size == w.size {
                    break;
                }

                let mut found = false;
                for arc in &h.v[(n + m) as usize].as_ref().unwrap().in_0 {
                    let j = arc.tail.i;
                    if w.pos[j as usize] == 0 {
                        found = true;
                        break;
                    }
                }

                if !found {
                    h.add_arc(i, n + m);
                    for arc in &h.v[(n + m) as usize].as_ref().unwrap().in_0 {
                        let j = arc.tail.i;
                        if v.pos[j as usize] == 0 {
                            v.size += 1;
                            v.list[v.size as usize] = j;
                            v.pos[j as usize] = v.size;
                        }
                    }
                }
            }

            for t in 1..=v.size {
                let j = v.list[t as usize];
                v.pos[j as usize] = 0;

                if w.pos[j as usize] != 0 {
                    if w.pos[j as usize] != w.size {
                        let jj = w.list[w.size as usize];
                        w.list[w.pos[j as usize] as usize] = jj;
                        w.pos[jj as usize] = w.pos[j as usize];
                    }
                    w.size -= 1;
                    w.pos[j as usize] = 0;
                }
            }

            v.size = 0;
            while w.size > 0 {
                let mut m = 0;
                let mut best = -1;

                for t in 1..=k {
                    let mut card = 0;
                    for arc in &h.v[(n + t) as usize].as_ref().unwrap().in_0 {
                        let j = arc.tail.i;
                        if w.pos[j as usize] != 0 {
                            card += 1;
                        }
                    }

                    if best < card {
                        m = t;
                        best = card;
                    }
                }

                assert!(m > 0, "m must be positive");
                let new_k = h.add_vertices(1) - n;

                for arc in &h.v[(n + m) as usize].as_ref().unwrap().in_0 {
                    let j = arc.tail.i;
                    if w.pos[j as usize] != 0 {
                        h.add_arc(j, n + new_k);
                        if w.pos[j as usize] != w.size {
                            let jj = w.list[w.size as usize];
                            w.list[w.pos[j as usize] as usize] = jj;
                            w.pos[jj as usize] = w.pos[j as usize];
                        }
                        w.size -= 1;
                        w.pos[j as usize] = 0;
                    }
                }

                h.add_arc(i, n + new_k);
            }
        }
    }

    h.nv - n
}

impl GlpGraph {
    fn erase_graph(&mut self) {
        self.pool = None;
        self.name = None;
        self.nv_max = 0;
        self.nv = 0;
        self.na = 0;
        self.v.clear();
        self.index = None;
    }

    fn add_vertices(&mut self, n: i32) -> i32 {
        for _ in 0..n {
            self.v.push(Some(Box::new(GlpVertex {
                i: self.nv + 1,
                name: None,
                entry: None,
                data: None,
                temp: None,
                in_0: Vec::new(),
                out: Vec::new(),
            })));
            self.nv += 1;
        }
        self.nv
    }

    fn add_arc(&mut self, i: i32, j: i32) {
        let tail = self.v[(i - 1) as usize].as_mut().unwrap();
        let head = self.v[(j - 1) as usize].as_mut().unwrap();

        let arc = GlpArc {
            tail: Box::new(tail.clone()),
            head: Box::new(head.clone()),
            data: None,
            temp: None,
            t_prev: None,
            t_next: None,
            h_prev: None,
            h_next: None,
        };

        tail.out.push(arc.clone());
        head.in_0.push(arc);
        self.na += 1;
    }
}