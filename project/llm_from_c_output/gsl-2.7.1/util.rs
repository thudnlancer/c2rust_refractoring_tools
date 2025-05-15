// integration/util.rs

use std::f64::{EPSILON, MIN};

pub struct IntegrationWorkspace {
    alist: Vec<f64>,
    blist: Vec<f64>,
    rlist: Vec<f64>,
    elist: Vec<f64>,
    level: Vec<usize>,
    i: usize,
    size: usize,
    maximum_level: usize,
}

impl IntegrationWorkspace {
    pub fn update(
        &mut self,
        a1: f64,
        b1: f64,
        area1: f64,
        error1: f64,
        a2: f64,
        b2: f64,
        area2: f64,
        error2: f64,
    ) {
        let i_max = self.i;
        let i_new = self.size;
        let new_level = self.level[i_max] + 1;

        if error2 > error1 {
            self.alist[i_max] = a2;
            self.rlist[i_max] = area2;
            self.elist[i_max] = error2;
            self.level[i_max] = new_level;

            self.alist[i_new] = a1;
            self.blist[i_new] = b1;
            self.rlist[i_new] = area1;
            self.elist[i_new] = error1;
            self.level[i_new] = new_level;
        } else {
            self.blist[i_max] = b1;
            self.rlist[i_max] = area1;
            self.elist[i_max] = error1;
            self.level[i_max] = new_level;

            self.alist[i_new] = a2;
            self.blist[i_new] = b2;
            self.rlist[i_new] = area2;
            self.elist[i_new] = error2;
            self.level[i_new] = new_level;
        }

        self.size += 1;

        if new_level > self.maximum_level {
            self.maximum_level = new_level;
        }

        self.qpsrt();
    }

    pub fn retrieve(&self) -> (f64, f64, f64, f64) {
        let i = self.i;
        (
            self.alist[i],
            self.blist[i],
            self.rlist[i],
            self.elist[i],
        )
    }

    pub fn sum_results(&self) -> f64 {
        self.rlist[..self.size].iter().sum()
    }

    fn qpsrt(&mut self) {
        // Implementation of qpsrt would go here
        // This is a placeholder as the original implementation wasn't provided
    }
}

pub fn subinterval_too_small(a1: f64, a2: f64, b2: f64) -> bool {
    let e = EPSILON;
    let u = MIN;
    let tmp = (1.0 + 100.0 * e) * (a2.abs() + 1000.0 * u);
    a1.abs() <= tmp && b2.abs() <= tmp
}