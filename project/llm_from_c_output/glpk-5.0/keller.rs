use std::collections::HashSet;

struct VertexSet {
    size: usize,
    list: Vec<usize>,
    pos: Vec<usize>,
}

impl VertexSet {
    fn new(n: usize) -> Self {
        VertexSet {
            size: 0,
            list: vec![0; n + 1],
            pos: vec![0; n + 1],
        }
    }

    fn contains(&self, i: usize) -> bool {
        self.pos[i] != 0
    }

    fn insert(&mut self, i: usize) {
        if self.pos[i] == 0 {
            self.size += 1;
            self.list[self.size] = i;
            self.pos[i] = self.size;
        }
    }

    fn remove(&mut self, i: usize) {
        if self.pos[i] != 0 {
            if self.pos[i] != self.size {
                let j = self.list[self.size];
                self.list[self.pos[i]] = j;
                self.pos[j] = self.pos[i];
            }
            self.size -= 1;
            self.pos[i] = 0;
        }
    }

    fn clear(&mut self) {
        for i in 1..=self.size {
            self.pos[self.list[i]] = 0;
        }
        self.size = 0;
    }
}

pub struct Graph {
    vertices: Vec<HashSet<usize>>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Graph {
            vertices: vec![HashSet::new(); n + 1],
        }
    }

    pub fn add_vertex(&mut self) -> usize {
        self.vertices.push(HashSet::new());
        self.vertices.len() - 1
    }

    pub fn add_edge(&mut self, i: usize, j: usize) {
        self.vertices[i].insert(j);
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices.len() - 1
    }
}

pub fn kellerman<F>(n: usize, mut func: F, info: &mut dyn std::any::Any) -> (usize, Graph)
where
    F: FnMut(&mut dyn std::any::Any, usize, &mut Vec<usize>) -> usize,
{
    assert!(n >= 0 as usize);
    let mut h = Graph::new(n);
    let mut w = VertexSet::new(n);
    let mut v = VertexSet::new(n);
    let mut k = 0;

    for i in 1..=n {
        w.clear();

        let mut adj_list = vec![0; n + 1];
        let len = func(info, i, &mut adj_list);
        assert!(len <= n);

        for t in 1..=len {
            let j = adj_list[t];
            assert!(1 <= j && j <= n);
            if j >= i {
                continue;
            }
            w.insert(j);
        }

        if w.size == 0 {
            k += 1;
            let clique = h.add_vertex();
            h.add_edge(i, clique);
            continue;
        }

        v.clear();
        let current_k = h.num_vertices() - n;

        for m in 1..=current_k {
            if v.size == w.size {
                break;
            }

            let mut is_subset = true;
            for &j in &h.vertices[n + m] {
                if !w.contains(j) {
                    is_subset = false;
                    break;
                }
            }

            if !is_subset {
                continue;
            }

            h.add_edge(i, n + m);
            for &j in &h.vertices[n + m] {
                if !v.contains(j) {
                    v.insert(j);
                }
            }
        }

        for t in 1..=v.size {
            let j = v.list[t];
            v.pos[j] = 0;
            if w.contains(j) {
                w.remove(j);
            }
        }
        v.size = 0;

        while w.size > 0 {
            let mut m = 0;
            let mut best = 0;
            let current_k = h.num_vertices() - n;

            for t in 1..=current_k {
                let mut card = 0;
                for &j in &h.vertices[n + t] {
                    if w.contains(j) {
                        card += 1;
                    }
                }
                if best < card {
                    m = t;
                    best = card;
                }
            }

            assert!(m > 0);
            k += 1;
            let clique = h.add_vertex();

            let mut to_remove = Vec::new();
            for &j in &h.vertices[n + m] {
                if w.contains(j) {
                    h.add_edge(j, clique);
                    to_remove.push(j);
                }
            }

            for j in to_remove {
                w.remove(j);
            }

            h.add_edge(i, clique);
        }
    }

    (k, h)
}