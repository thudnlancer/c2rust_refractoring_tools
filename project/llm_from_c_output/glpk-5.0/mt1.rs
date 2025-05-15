// mt1.rs (0-1 knapsack problem; Martello & Toth algorithm)

/***********************************************************************
*  This code is part of GLPK (GNU Linear Programming Kit).
*  Copyright (C) 2017-2018 Free Software Foundation, Inc.
*  Written by Andrew Makhorin <mao@gnu.org>.
*
*  GLPK is free software: you can redistribute it and/or modify it
*  under the terms of the GNU General Public License as published by
*  the Free Software Foundation, either version 3 of the License, or
*  (at your option) any later version.
*
*  GLPK is distributed in the hope that it will be useful, but WITHOUT
*  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
*  or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public
*  License for more details.
*
*  You should have received a copy of the GNU General Public License
*  along with GLPK. If not, see <http://www.gnu.org/licenses/>.
***********************************************************************/

pub fn mt1(
    n: i32,
    p: &[i32],
    w: &[i32],
    c: i32,
    x: &mut [i32],
    jck: i32,
    xx: &mut [i32],
    min: &mut [i32],
    psign: &mut [i32],
    wsign: &mut [i32],
    zsign: &mut [i32],
) -> i32 {
    let mut z = 0;
    let jdim = n + 1;
    
    if jck == 1 {
        if let Err(e) = chmt1(n, p, w, c, &mut z, jdim) {
            z = e;
        }
    }
    
    if z < 0 {
        return z;
    }
    
    let mut ch = c;
    let mut ip = 0;
    let mut chs = ch;
    let mut ll = 0;
    
    for j in 0..n as usize {
        if w[j] > chs {
            break;
        }
        ip += p[j];
        chs -= w[j];
        ll = j as i32 + 1;
    }
    
    if chs == 0 {
        z = ip;
        for j in 0..ll as usize {
            x[j] = 1;
        }
        for j in ll as usize..n as usize {
            x[j] = 0;
        }
        return z;
    }
    
    let mut p = p.to_vec();
    let mut w = w.to_vec();
    p.push(0);
    w.push(ch + 1);
    
    let mut lim = ip + (chs * p[ll as usize + 1] / w[ll as usize + 1]);
    let a = (w[ll as usize] - chs) as f32;
    let b = (ip + p[ll as usize]) as f32;
    let mut lim1 = (b - a * p[ll as usize - 1] as f32 / w[ll as usize - 1] as f32) as i32;
    
    if lim1 > lim {
        lim = lim1;
    }
    
    let mut mink = ch + 1;
    min[n as usize - 1] = mink;
    
    for j in 2..=n {
        let kk = n + 2 - j;
        if w[kk as usize - 1] < mink {
            mink = w[kk as usize - 1];
        }
        min[kk as usize - 2] = mink;
    }
    
    for j in 0..n as usize {
        xx[j] = 0;
    }
    
    z = 0;
    let mut profit = 0;
    let mut lold = n;
    let mut ii = 1;
    
    loop {
        if ii > n {
            break;
        }
        
        if w[ii as usize - 1] <= ch {
            ip = psign[ii as usize - 1];
            chs = ch - wsign[ii as usize - 1];
            let in_ = zsign[ii as usize - 1];
            
            let mut ll = in_;
            while ll <= n {
                if w[ll as usize - 1] > chs {
                    break;
                }
                ip += p[ll as usize - 1];
                chs -= w[ll as usize - 1];
                ll += 1;
            }
            
            ll -= 1;
            let iu = (chs * p[ll as usize] / w[ll as usize]) as i32;
            
            if iu == 0 {
                if z < ip + profit {
                    z = ip + profit;
                    for j in 0..(ii - 1) as usize {
                        x[j] = xx[j];
                    }
                    for j in (ii - 1)..ll as usize {
                        x[j] = 1;
                    }
                    if ll < n {
                        for j in ll as usize..n as usize {
                            x[j] = 0;
                        }
                    }
                    if z == lim {
                        return z;
                    }
                }
            } else if z < profit + ip + iu {
                wsign[ii as usize - 1] = ch - chs;
                psign[ii as usize - 1] = ip;
                zsign[ii as usize - 1] = ll + 1;
                xx[ii as usize - 1] = 1;
                
                let nn = ll - 1;
                if nn >= ii {
                    for j in ii..nn {
                        wsign[j as usize] = wsign[j as usize - 1] - w[j as usize - 1];
                        psign[j as usize] = psign[j as usize - 1] - p[j as usize - 1];
                        zsign[j as usize] = ll + 1;
                        xx[j as usize] = 1;
                    }
                }
                
                let j1 = ll + 1;
                for j in j1..=lold {
                    wsign[j as usize - 1] = 0;
                    psign[j as usize - 1] = 0;
                    zsign[j as usize - 1] = j;
                }
                
                lold = ll;
                ch = chs;
                profit += ip;
                
                if ll > n - 2 {
                    ii = n;
                } else if ll == n - 2 {
                    if ch >= w[n as usize - 1] {
                        ch -= w[n as usize - 1];
                        profit += p[n as usize - 1];
                        xx[n as usize - 1] = 1;
                    }
                    ii = n - 1;
                } else {
                    ii = ll + 2;
                    if ch >= min[ii as usize - 1] {
                        continue;
                    }
                }
                
                if z < profit {
                    z = profit;
                    for j in 0..n as usize {
                        x[j] = xx[j];
                    }
                    if z == lim {
                        return z;
                    }
                }
                
                if xx[n as usize - 1] != 0 {
                    xx[n as usize - 1] = 0;
                    ch += w[n as usize - 1];
                    profit -= p[n as usize - 1];
                }
            }
        } else {
            let ii1 = ii + 1;
            if z >= (ch * p[ii1 as usize - 1] / w[ii1 as usize - 1] + profit) {
                break;
            }
            ii = ii1;
            continue;
        }
        
        let nn = ii - 1;
        if nn == 0 {
            break;
        }
        
        let mut kk = 0;
        for j in 1..=nn {
            kk = ii - j;
            if xx[kk as usize - 1] == 1 {
                break;
            }
        }
        
        let r = ch;
        ch += w[kk as usize - 1];
        profit -= p[kk as usize - 1];
        xx[kk as usize - 1] = 0;
        
        if r >= min[kk as usize - 1] {
            ii = kk + 1;
            continue;
        }
        
        let nn = kk + 1;
        ii = kk;
        
        loop {
            if nn > n {
                break;
            }
            
            if z >= profit + (ch * p[nn as usize - 1] / w[nn as usize - 1]) {
                break;
            }
            
            let diff = w[nn as usize - 1] - w[kk as usize - 1];
            if diff < 0 {
                let t = r - diff;
                if t < min[nn as usize - 1] {
                    nn += 1;
                    continue;
                }
                
                if z >= profit + p[nn as usize - 1] + (t * p[nn as usize] / w[nn as usize]) {
                    break;
                }
                
                ch -= w[nn as usize - 1];
                profit += p[nn as usize - 1];
                xx[nn as usize - 1] = 1;
                ii = nn + 1;
                wsign[nn as usize - 1] = w[nn as usize - 1];
                psign[nn as usize - 1] = p[nn as usize - 1];
                zsign[nn as usize - 1] = ii;
                
                let n1 = nn + 1;
                for j in n1..=lold {
                    wsign[j as usize - 1] = 0;
                    psign[j as usize - 1] = 0;
                    zsign[j as usize - 1] = j;
                }
                
                lold = nn;
                continue;
            } else if diff == 0 {
                nn += 1;
                continue;
            } else {
                if diff > r {
                    nn += 1;
                    continue;
                }
                
                if z >= profit + p[nn as usize - 1] {
                    nn += 1;
                    continue;
                }
                
                z = profit + p[nn as usize - 1];
                for j in 0..kk as usize {
                    x[j] = xx[j];
                }
                for j in kk as usize..n as usize {
                    x[j] = 0;
                }
                x[nn as usize - 1] = 1;
                
                if z == lim {
                    return z;
                }
                
                let r = r - diff;
                kk = nn;
                nn += 1;
            }
        }
    }
    
    z
}

fn chmt1(n: i32, p: &[i32], w: &[i32], c: i32, z: &mut i32, jdim: i32) -> Result<(), i32> {
    if n < 2 || n > jdim - 1 {
        *z = -1;
        return Err(-1);
    }
    
    if c <= 0 {
        *z = -2;
        return Err(-2);
    }
    
    let mut jsw = 0;
    let mut rr = p[0] as f32 / w[0] as f32;
    
    for j in 0..n as usize {
        let r = rr;
        if p[j] <= 0 || w[j] <= 0 {
            *z = -2;
            return Err(-2);
        }
        
        jsw += w[j];
        if w[j] > c {
            *z = -3;
            return Err(-3);
        }
        
        rr = p[j] as f32 / w[j] as f32;
        if rr > r {
            *z = -5;
            return Err(-5);
        }
    }
    
    if jsw <= c {
        *z = -4;
        return Err(-4);
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt1() {
        let n = 4;
        let p = vec![10, 40, 30, 50];
        let w = vec![5, 4, 6, 3];
        let c = 10;
        let mut x = vec![0; 4];
        let jck = 1;
        let mut xx = vec![0; 4];
        let mut min = vec![0; 4];
        let mut psign = vec![0; 4];
        let mut wsign = vec![0; 4];
        let mut zsign = vec![0; 4];
        
        let z = mt1(
            n,
            &p,
            &w,
            c,
            &mut x,
            jck,
            &mut xx,
            &mut min,
            &mut psign,
            &mut wsign,
            &mut zsign,
        );
        
        assert_eq!(z, 90);
        assert_eq!(x, vec![0, 1, 0, 1]);
    }
}