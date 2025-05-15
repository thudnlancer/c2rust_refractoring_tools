use std::cmp::{max, min};
use std::ptr;

struct Graph {
    nv: i32,
    na: i32,
    tail: Vec<i32>,
    head: Vec<i32>,
    low: Vec<i32>,
    cap: Vec<i32>,
    cost: Vec<i32>,
}

impl Graph {
    fn new(nv: i32, na: i32) -> Self {
        Graph {
            nv,
            na,
            tail: vec![0; (na + 1) as usize],
            head: vec![0; (na + 1) as usize],
            low: vec![0; (na + 1) as usize],
            cap: vec![0; (na + 1) as usize],
            cost: vec![0; (na + 1) as usize],
        }
    }

    fn overflow(u: i32, v: i32) -> bool {
        (u > 0 && v > 0 && u + v < 0) || (u < 0 && v < 0 && u + v > 0)
    }

    fn okalg(&self, x: &mut Vec<i32>, pi: &mut Vec<i32>) -> i32 {
        assert!(self.nv >= 0);
        assert!(self.na >= 0);

        for a in 1..=self.na {
            let i = self.tail[a as usize];
            let j = self.head[a as usize];
            assert!(1 <= i && i <= self.nv);
            assert!(1 <= j && j <= self.nv);
            assert!(i != j);
            assert!(0 <= self.low[a as usize] && self.low[a as usize] <= self.cap[a as usize]);
        }

        let mut ptr = vec![0; (self.nv + 2) as usize];
        let mut arc = vec![0; (2 * self.na + 1) as usize];
        let mut link = vec![0; (self.nv + 1) as usize];
        let mut list = vec![0; (self.nv + 1) as usize];

        for i in 1..=self.nv {
            ptr[i as usize] = 0;
        }

        for a in 1..=self.na {
            ptr[self.tail[a as usize] as usize] += 1;
            ptr[self.head[a as usize] as usize] += 1;
        }

        ptr[1] += 1;
        for i in 1..self.nv {
            ptr[(i + 1) as usize] += ptr[i as usize];
        }
        ptr[(self.nv + 1) as usize] = ptr[self.nv as usize];

        for a in (1..=self.na).rev() {
            ptr[self.tail[a as usize] as usize] -= 1;
            arc[ptr[self.tail[a as usize] as usize] as usize] = a;
            ptr[self.head[a as usize] as usize] -= 1;
            arc[ptr[self.head[a as usize] as usize] as usize] = a;
        }

        assert!(ptr[1] == 1);
        assert!(ptr[(self.nv + 1) as usize] == self.na + self.na + 1);

        for a in 1..=self.na {
            x[a as usize] = 0;
        }

        for i in 1..=self.nv {
            pi[i as usize] = 0;
        }

        loop {
            let mut aok = 0;
            let (mut s, mut t) = (0, 0);

            for a in 1..=self.na {
                let i = self.tail[a as usize];
                let j = self.head[a as usize];
                if Self::overflow(self.cost[a as usize], pi[i as usize] - pi[j as usize]) {
                    return 2;
                }

                let lambda = self.cost[a as usize] + (pi[i as usize] - pi[j as usize]);

                if x[a as usize] < self.low[a as usize]
                    || (lambda < 0 && x[a as usize] < self.cap[a as usize])
                {
                    aok = a;
                    s = j;
                    t = i;
                    break;
                } else if x[a as usize] > self.cap[a as usize]
                    || (lambda > 0 && x[a as usize] > self.low[a as usize])
                {
                    aok = a;
                    s = i;
                    t = j;
                    break;
                }
            }

            if aok == 0 {
                break;
            }

            link.iter_mut().for_each(|x| *x = 0);
            link[s as usize] = aok;
            list[1] = s;
            let (mut pos1, mut pos2) = (1, 1);

            let mut found = false;
            while pos1 <= pos2 {
                let i = list[pos1 as usize];
                pos1 += 1;

                let mut k = ptr[i as usize];
                while k < ptr[(i + 1) as usize] {
                    let a = arc[k as usize];
                    let (j, is_forward) = if self.tail[a as usize] == i {
                        (self.head[a as usize], true)
                    } else {
                        (self.tail[a as usize], false)
                    };

                    if link[j as usize] != 0 {
                        k += 1;
                        continue;
                    }

                    let (allow, lambda) = if is_forward {
                        if x[a as usize] >= self.cap[a as usize] {
                            (false, 0)
                        } else if Self::overflow(self.cost[a as usize], pi[i as usize] - pi[j as usize]) {
                            return 2;
                        } else {
                            let lambda = self.cost[a as usize] + (pi[i as usize] - pi[j as usize]);
                            (lambda <= 0 || x[a as usize] < self.low[a as usize], lambda)
                        }
                    } else {
                        if x[a as usize] <= self.low[a as usize] {
                            (false, 0)
                        } else if Self::overflow(self.cost[a as usize], pi[j as usize] - pi[i as usize]) {
                            return 2;
                        } else {
                            let lambda = self.cost[a as usize] + (pi[j as usize] - pi[i as usize]);
                            (lambda >= 0 || x[a as usize] > self.cap[a as usize], lambda)
                        }
                    };

                    if allow {
                        link[j as usize] = a;
                        pos2 += 1;
                        list[pos2 as usize] = j;
                        if j == t {
                            found = true;
                            break;
                        }
                    }
                    k += 1;
                }
                if found {
                    break;
                }
            }

            if found {
                let mut delta = 0;
                let mut j = t;
                loop {
                    let a = link[j as usize];
                    let (i, temp) = if self.head[a as usize] == j {
                        let i = self.tail[a as usize];
                        let lambda = self.cost[a as usize] + (pi[i as usize] - pi[j as usize]);
                        let temp = if lambda > 0 && x[a as usize] < self.low[a as usize] {
                            self.low[a as usize] - x[a as usize]
                        } else if lambda <= 0 && x[a as usize] < self.cap[a as usize] {
                            self.cap[a as usize] - x[a as usize]
                        } else {
                            unreachable!()
                        };
                        (i, temp)
                    } else {
                        let i = self.head[a as usize];
                        let lambda = self.cost[a as usize] + (pi[j as usize] - pi[i as usize]);
                        let temp = if lambda < 0 && x[a as usize] > self.cap[a as usize] {
                            x[a as usize] - self.cap[a as usize]
                        } else if lambda >= 0 && x[a as usize] > self.low[a as usize] {
                            x[a as usize] - self.low[a as usize]
                        } else {
                            unreachable!()
                        };
                        (i, temp)
                    };

                    delta = if delta == 0 { temp } else { min(delta, temp) };
                    if i == t {
                        break;
                    }
                    j = i;
                }

                assert!(delta > 0);
                let mut j = t;
                loop {
                    let a = link[j as usize];
                    let i = if self.head[a as usize] == j {
                        x[a as usize] += delta;
                        self.tail[a as usize]
                    } else {
                        x[a as usize] -= delta;
                        self.head[a as usize]
                    };
                    if i == t {
                        break;
                    }
                    j = i;
                }
            } else {
                let mut delta = 0;
                for a in 1..=self.na {
                    let i = self.tail[a as usize];
                    let j = self.head[a as usize];
                    if link[i as usize] != 0 && link[j as usize] == 0 {
                        if Self::overflow(self.cost[a as usize], pi[i as usize] - pi[j as usize]) {
                            return 2;
                        }
                        let lambda = self.cost[a as usize] + (pi[i as usize] - pi[j as usize]);
                        if x[a as usize] <= self.cap[a as usize] && lambda > 0 {
                            delta = if delta == 0 {
                                lambda
                            } else {
                                min(delta, lambda)
                            };
                        }
                    } else if link[i as usize] == 0 && link[j as usize] != 0 {
                        if Self::overflow(self.cost[a as usize], pi[i as usize] - pi[j as usize]) {
                            return 2;
                        }
                        let lambda = self.cost[a as usize] + (pi[i as usize] - pi[j as usize]);
                        if x[a as usize] >= self.low[a as usize] && lambda < 0 {
                            delta = if delta == 0 {
                                -lambda
                            } else {
                                min(delta, -lambda)
                            };
                        }
                    }
                }

                if delta == 0 {
                    return 1;
                }

                for i in 1..=self.nv {
                    if link[i as usize] == 0 {
                        if Self::overflow(pi[i as usize], delta) {
                            return 2;
                        }
                        pi[i as usize] += delta;
                    }
                }
            }
        }

        for a in 1..=self.na {
            if !(self.low[a as usize] <= x[a as usize] && x[a as usize] <= self.cap[a as usize]) {
                return 3;
            }
        }

        for i in 1..=self.nv {
            let mut temp = 0;
            let mut k = ptr[i as usize];
            while k < ptr[(i + 1) as usize] {
                let a = arc[k as usize];
                if self.tail[a as usize] == i {
                    temp += x[a as usize];
                } else {
                    temp -= x[a as usize];
                }
                k += 1;
            }
            if temp != 0 {
                return 3;
            }
        }

        for a in 1..=self.na {
            let i = self.tail[a as usize];
            let j = self.head[a as usize];
            let lambda = self.cost[a as usize] + (pi[i as usize] - pi[j as usize]);
            if (lambda > 0 && x[a as usize] != self.low[a as usize])
                || (lambda < 0 && x[a as usize] != self.cap[a as usize])
            {
                return 3;
            }
        }

        0
    }
}

pub fn glp_okalg(
    nv: i32,
    na: i32,
    tail: &[i32],
    head: &[i32],
    low: &[i32],
    cap: &[i32],
    cost: &[i32],
    x: &mut [i32],
    pi: &mut [i32],
) -> i32 {
    let mut graph = Graph::new(nv, na);
    graph.tail[1..=na as usize].copy_from_slice(&tail[1..=na as usize]);
    graph.head[1..=na as usize].copy_from_slice(&head[1..=na as usize]);
    graph.low[1..=na as usize].copy_from_slice(&low[1..=na as usize]);
    graph.cap[1..=na as usize].copy_from_slice(&cap[1..=na as usize]);
    graph.cost[1..=na as usize].copy_from_slice(&cost[1..=na as usize]);

    graph.okalg(x, pi)
}