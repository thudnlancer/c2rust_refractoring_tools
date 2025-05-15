use std::cmp;

type Integer = i32;
type Real = f32;

struct KnapsackState {
    n: Integer,
    p: Vec<Integer>,
    w: Vec<Integer>,
    c: Integer,
    z: Integer,
    x: Vec<Integer>,
    jdim: Integer,
    jck: Integer,
    xx: Vec<Integer>,
    min: Vec<Integer>,
    psign: Vec<Integer>,
    wsign: Vec<Integer>,
    zsign: Vec<Integer>,
}

impl KnapsackState {
    fn new(
        n: Integer,
        p: Vec<Integer>,
        w: Vec<Integer>,
        c: Integer,
        jdim: Integer,
        jck: Integer,
    ) -> Self {
        let size = n as usize + 1;
        Self {
            n,
            p,
            w,
            c,
            z: 0,
            x: vec![0; size],
            jdim,
            jck,
            xx: vec![0; size],
            min: vec![0; size],
            psign: vec![0; size],
            wsign: vec![0; size],
            zsign: vec![0; size],
        }
    }

    fn chmt1(&mut self) -> bool {
        if self.n < 2 || self.n > self.jdim - 1 {
            self.z = -1;
            return false;
        }

        if self.c <= 0 {
            self.z = -2;
            return false;
        }

        let mut jsw = 0;
        let mut rr = self.p[1] as Real / self.w[1] as Real;

        for j in 1..=self.n as usize {
            let r = rr;
            if self.p[j] <= 0 || self.w[j] <= 0 {
                self.z = -5;
                return false;
            }

            jsw += self.w[j];
            if self.w[j] > self.c {
                self.z = -3;
                return false;
            }

            rr = self.p[j] as Real / self.w[j] as Real;
            if rr > r {
                self.z = -5;
                return false;
            }
        }

        if jsw > self.c {
            self.z = -4;
            false
        } else {
            true
        }
    }

    fn mt1(&mut self) {
        self.z = 0;
        if self.jck == 1 && !self.chmt1() {
            return;
        }

        if self.z < 0 {
            return;
        }

        let mut ch = self.c;
        let mut ip = 0;
        let mut chs = ch;
        let mut ll = 1;

        while ll <= self.n {
            if self.w[ll as usize] > chs {
                break;
            }
            ip += self.p[ll as usize];
            chs -= self.w[ll as usize];
            ll += 1;
        }

        ll -= 1;

        if chs == 0 {
            self.z = ip;
            for j in 1..=ll as usize {
                self.x[j] = 1;
            }
            for j in (ll + 1) as usize..=self.n as usize {
                self.x[j] = 0;
            }
            return;
        }

        let n = self.n as usize;
        self.p[n + 1] = 0;
        self.w[n + 1] = ch + 1;

        let lim = ip + chs * self.p[ll as usize + 2] / self.w[ll as usize + 2];
        let a = (self.w[ll as usize + 1] - chs) as Real;
        let b = (ip + self.p[ll as usize + 1]) as Real;
        let lim1 = (b - a * self.p[ll as usize] as Real / self.w[ll as usize] as Real) as Integer;
        let lim = cmp::max(lim, lim1);

        let mut mink = ch + 1;
        self.min[n] = mink;

        for j in 2..=n {
            let kk = n + 2 - j;
            if self.w[kk] < mink {
                mink = self.w[kk];
            }
            self.min[kk - 1] = mink;
        }

        for j in 1..=n {
            self.xx[j] = 0;
        }

        self.z = 0;
        let mut profit = 0;
        let mut lold = n as Integer;
        let mut ii = 1;

        loop {
            self.wsign[ii as usize] = ch - chs;
            self.psign[ii as usize] = ip;
            self.zsign[ii as usize] = ll + 1;
            self.xx[ii as usize] = 1;

            let nn = ll - 1;
            if nn >= ii {
                for j in ii..=nn {
                    self.wsign[(j + 1) as usize] = self.wsign[j as usize] - self.w[j as usize];
                    self.psign[(j + 1) as usize] = self.psign[j as usize] - self.p[j as usize];
                    self.zsign[(j + 1) as usize] = ll + 1;
                    self.xx[(j + 1) as usize] = 1;
                }
            }

            let j1 = ll + 1;
            for j in j1..=lold {
                self.wsign[j as usize] = 0;
                self.psign[j as usize] = 0;
                self.zsign[j as usize] = j;
            }

            lold = ll;
            ch = chs;
            profit += ip;

            if ll - (self.n - 2) < 0 {
                ii = ll + 2;
                if ch >= self.min[(ii - 1) as usize] {
                    continue;
                }
            } else if ll - (self.n - 2) == 0 {
                if ch >= self.w[n as usize] {
                    ch -= self.w[n as usize];
                    profit += self.p[n as usize];
                    self.xx[n as usize] = 1;
                }
                ii = self.n - 1;
            } else {
                ii = self.n;
            }

            if self.z < profit {
                self.z = profit;
                for j in 1..=n {
                    self.x[j] = self.xx[j];
                }
                if self.z == lim {
                    return;
                }
            }

            if self.xx[n as usize] == 0 {
                break;
            }

            self.xx[n as usize] = 0;
            ch += self.w[n as usize];
            profit -= self.p[n as usize];
        }
    }
}

pub fn glp_mt1(
    n: Integer,
    p: &mut [Integer],
    w: &mut [Integer],
    c: Integer,
    x: &mut [Integer],
    jck: Integer,
    xx: &mut [Integer],
    min: &mut [Integer],
    psign: &mut [Integer],
    wsign: &mut [Integer],
    zsign: &mut [Integer],
) -> Integer {
    let mut state = KnapsackState::new(
        n,
        p.to_vec(),
        w.to_vec(),
        c,
        n + 1,
        jck,
    );

    state.mt1();

    let z = state.z;
    let mut s1 = 0;
    let mut s2 = 0;

    for j in 1..=n as usize {
        assert!(x[j] == 0 || x[j] == 1);
        if x[j] != 0 {
            s1 += p[j];
            s2 += w[j];
        }
    }

    assert_eq!(s1, z);
    assert!(s2 <= c);

    z
}