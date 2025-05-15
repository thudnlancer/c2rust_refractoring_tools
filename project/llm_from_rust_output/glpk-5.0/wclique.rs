use std::time::Instant;

struct Csa {
    n: i32,
    wt: Vec<i32>,
    a: Vec<u8>,
    record: i32,
    rec_level: i32,
    rec: Vec<i32>,
    clique: Vec<i32>,
    set: Vec<i32>,
}

impl Csa {
    fn new(n: i32, w: &[i32], a: &[u8]) -> Self {
        Csa {
            n,
            wt: w.to_vec(),
            a: a.to_vec(),
            record: 0,
            rec_level: 0,
            rec: vec![0; n as usize],
            clique: vec![0; n as usize],
            set: vec![0; n as usize],
        }
    }

    fn sub(&mut self, ct: i32, table: &[i32], level: i32, weight: i32, l_weight: i32) {
        if ct <= 0 {
            if ct == 0 {
                let mut level = level;
                self.set[level as usize] = table[0];
                level += 1;
                let weight = weight + l_weight;
                
                if weight > self.record {
                    self.record = weight;
                    self.rec_level = level;
                    self.rec[..level as usize].copy_from_slice(&self.set[..level as usize]);
                }
            }
            return;
        }

        let mut newtable = vec![0; self.n as usize];
        
        for i in (0..=ct).rev() {
            if level == 0 && i < ct {
                break;
            }
            
            let k = table[i as usize];
            if level > 0 && self.clique[k as usize] <= self.record - weight {
                break;
            }
            
            self.set[level as usize] = k;
            let curr_weight = weight + self.wt[k as usize];
            let l_weight = l_weight - self.wt[k as usize];
            
            if l_weight <= self.record - curr_weight {
                break;
            }
            
            let mut left_weight = 0;
            let mut p = 0;
            
            for &j in table.iter().take(i as usize) {
                if j == k {
                    continue;
                }
                
                let connected = if j > k {
                    let idx = (j * (j - 1) / 2 + k) / 8;
                    let bit = 7 - (j * (j - 1) / 2 + k) % 8;
                    (self.a[idx as usize] >> bit) & 1 != 0
                } else {
                    let idx = (k * (k - 1) / 2 + j) / 8;
                    let bit = 7 - (k * (k - 1) / 2 + j) % 8;
                    (self.a[idx as usize] >> bit) & 1 != 0
                };
                
                if connected {
                    newtable[p] = j;
                    p += 1;
                    left_weight += self.wt[j as usize];
                }
            }
            
            if left_weight > self.record - curr_weight {
                self.sub(p as i32 - 1, &newtable[..p], level + 1, curr_weight, left_weight);
            }
        }
    }
}

pub fn glp_wclique(n: i32, w: &[i32], a: &[u8], ind: &mut [i32]) -> i32 {
    assert!(n > 0, "n must be positive");
    
    let mut csa = Csa::new(n, w, a);
    let mut used = vec![0; n as usize];
    let mut nwt = vec![0; n as usize];
    let mut pos = vec![0; n as usize];
    let timer = Instant::now();
    
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            
            let connected = if i > j {
                let idx = (i * (i - 1) / 2 + j) / 8;
                let bit = 7 - (i * (i - 1) / 2 + j) % 8;
                (a[idx as usize] >> bit) & 1 != 0
            } else {
                let idx = (j * (j - 1) / 2 + i) / 8;
                let bit = 7 - (j * (j - 1) / 2 + i) % 8;
                (a[idx as usize] >> bit) & 1 != 0
            };
            
            if connected {
                nwt[i as usize] += w[j as usize];
            }
        }
    }
    
    for i in (0..n).rev() {
        let (mut max_wt, mut max_nwt, mut p) = (-1, -1, 0);
        
        for j in 0..n {
            if used[j as usize] == 0 && 
               (w[j as usize] > max_wt || 
                (w[j as usize] == max_wt && nwt[j as usize] > max_nwt)) 
            {
                max_wt = w[j as usize];
                max_nwt = nwt[j as usize];
                p = j;
            }
        }
        
        pos[i as usize] = p;
        used[p as usize] = 1;
        
        for j in 0..n {
            if used[j as usize] == 0 && j != p {
                let connected = if p > j {
                    let idx = (p * (p - 1) / 2 + j) / 8;
                    let bit = 7 - (p * (p - 1) / 2 + j) % 8;
                    (a[idx as usize] >> bit) & 1 != 0
                } else {
                    let idx = (j * (j - 1) / 2 + p) / 8;
                    let bit = 7 - (j * (j - 1) / 2 + p) % 8;
                    (a[idx as usize] >> bit) & 1 != 0
                };
                
                if connected {
                    nwt[j as usize] -= w[p as usize];
                }
            }
        }
    }
    
    let mut wth = 0;
    for i in 0..n {
        wth += w[pos[i as usize] as usize];
        csa.sub(i, &pos[..=i as usize], 0, 0, wth as i32);
        csa.clique[pos[i as usize] as usize] = csa.record;
        
        if timer.elapsed().as_secs_f64() >= 5.0 - 0.001 {
            println!("level = {} ({}); best = {}", i + 1, csa.n, csa.record);
        }
    }
    
    for i in 1..=csa.rec_level {
        ind[i as usize] += 1;
    }
    
    csa.rec_level
}