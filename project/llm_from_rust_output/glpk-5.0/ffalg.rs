use std::ptr;

struct Graph {
    nv: i32,
    na: i32,
    tail: Vec<i32>,
    head: Vec<i32>,
    cap: Vec<i32>,
    x: Vec<i32>,
    cut: Option<Vec<i8>>,
}

impl Graph {
    fn new(nv: i32, na: i32, tail: Vec<i32>, head: Vec<i32>, cap: Vec<i32>, x: Vec<i32>, cut: Option<Vec<i8>>) -> Self {
        assert!(nv >= 2, "nv must be >= 2");
        assert!(na >= 0, "na must be >= 0");
        assert!(1 <= tail[0] && tail[0] <= nv, "invalid tail");
        assert!(1 <= head[0] && head[0] <= nv, "invalid head");
        assert!(tail[0] != head[0], "tail and head must be different");
        assert!(cap[0] >= 0, "capacity must be >= 0");

        Graph { nv, na, tail, head, cap, x, cut }
    }

    fn ffalg(&mut self, s: i32, t: i32) {
        assert!(1 <= s && s <= self.nv, "invalid source node");
        assert!(1 <= t && t <= self.nv, "invalid sink node");
        assert!(s != t, "source and sink must be different");

        let mut ptr = vec![0; (self.nv + 2) as usize];
        let mut arc = vec![0; (2 * self.na + 1) as usize];
        let mut link = vec![0; (self.nv + 1) as usize];
        let mut list = vec![0; (self.nv + 1) as usize];

        // Initialize ptr
        for i in 1..=self.nv {
            ptr[i as usize] = 0;
        }

        // Count arcs per node
        for a in 1..=self.na {
            ptr[self.tail[a as usize] as usize] += 1;
            ptr[self.head[a as usize] as usize] += 1;
        }

        // Compute prefix sums
        ptr[1] += 1;
        for i in 1..self.nv {
            ptr[(i + 1) as usize] += ptr[i as usize];
        }
        ptr[(self.nv + 1) as usize] = ptr[self.nv as usize];

        // Build arc lists
        for a in 1..=self.na {
            ptr[self.tail[a as usize] as usize] -= 1;
            arc[ptr[self.tail[a as usize] as usize]] = a;
            ptr[self.head[a as usize] as usize] -= 1;
            arc[ptr[self.head[a as usize] as usize]] = a;
        }

        // Initialize flow
        for a in 1..=self.na {
            self.x[a as usize] = 0;
        }

        loop {
            // Reset link and list
            for i in 1..=self.nv {
                link[i as usize] = 0;
            }
            link[s as usize] = -1;
            list[1] = s;
            let mut pos1 = 1;
            let mut pos2 = 1;

            // BFS loop
            let mut found = false;
            while pos1 <= pos2 {
                let i = list[pos1 as usize];
                pos1 += 1;

                for k in ptr[i as usize]..ptr[(i + 1) as usize] {
                    let a = arc[k as usize];
                    let (j, valid) = if self.tail[a as usize] == i {
                        (self.head[a as usize], self.x[a as usize] < self.cap[a as usize])
                    } else if self.head[a as usize] == i {
                        (self.tail[a as usize], self.x[a as usize] > 0)
                    } else {
                        panic!("invalid arc");
                    };

                    if link[j as usize] == 0 && valid {
                        link[j as usize] = a;
                        pos2 += 1;
                        list[pos2 as usize] = j;
                        if j == t {
                            found = true;
                            break;
                        }
                    }
                }
                if found {
                    break;
                }
            }

            if !found {
                break;
            }

            // Find minimum delta
            let mut delta = i32::MAX;
            let mut j = t;
            while j != s {
                let a = link[j as usize];
                let (i, temp) = if self.head[a as usize] == j {
                    (self.tail[a as usize], self.cap[a as usize] - self.x[a as usize])
                } else {
                    (self.head[a as usize], self.x[a as usize])
                };
                delta = delta.min(temp);
                j = i;
            }
            assert!(delta > 0, "delta must be positive");

            // Update flow
            j = t;
            while j != s {
                let a = link[j as usize];
                if self.head[a as usize] == j {
                    let i = self.tail[a as usize];
                    self.x[a as usize] += delta;
                } else {
                    let i = self.head[a as usize];
                    self.x[a as usize] -= delta;
                }
                j = link[j as usize];
            }
        }

        // Set cut if needed
        if let Some(cut) = &mut self.cut {
            for i in 1..=self.nv {
                cut[i as usize] = (link[i as usize] != 0) as i8;
            }
        }
    }
}

pub fn glp_ffalg(
    nv: i32,
    na: i32,
    tail: &[i32],
    head: &[i32],
    s: i32,
    t: i32,
    cap: &[i32],
    x: &mut [i32],
    cut: Option<&mut [i8]>,
) {
    let cut_vec = cut.map(|c| c.to_vec());
    let mut graph = Graph::new(
        nv,
        na,
        tail.to_vec(),
        head.to_vec(),
        cap.to_vec(),
        x.to_vec(),
        cut_vec,
    );
    graph.ffalg(s, t);
    x.copy_from_slice(&graph.x);
    if let (Some(cut_slice), Some(cut_vec)) = (cut, graph.cut) {
        cut_slice.copy_from_slice(&cut_vec);
    }
}