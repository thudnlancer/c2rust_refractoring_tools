pub fn glp_mc13d(
    n: i32,
    icn: &[i32],
    ip: &[i32],
    lenr: &[i32],
    ior: &mut [i32],
    ib: &mut [i32],
    lowl: &mut [i32],
    numb: &mut [i32],
    prev: &mut [i32],
) -> i32 {
    let mut icnt = 0;
    let mut num = 0;
    let nnm1 = n + n - 1;
    
    for j in 1..=n {
        numb[j as usize] = 0;
        ior[j as usize] = lenr[j as usize] - 1;
    }

    let mut isn = 1;
    'outer: while isn <= n {
        if numb[isn as usize] == 0 {
            let mut iv = isn;
            let mut ist = 1;
            numb[iv as usize] = 1;
            lowl[iv as usize] = 1;
            ib[n as usize] = iv;
            
            let mut dummy = 1;
            while dummy <= nnm1 {
                let mut i1 = ior[iv as usize];
                if i1 >= 0 {
                    let i2 = ip[iv as usize] + lenr[iv as usize] - 1;
                    i1 = i2 - i1;
                    let mut ii = i1;
                    
                    let mut found = false;
                    while ii <= i2 {
                        let iw = icn[ii as usize];
                        if numb[iw as usize] == 0 {
                            ior[iv as usize] = i2 - ii - 1;
                            prev[iw as usize] = iv;
                            iv = iw;
                            ist += 1;
                            numb[iv as usize] = ist;
                            lowl[iv as usize] = ist;
                            ib[(n + 1 - ist) as usize] = iv;
                            found = true;
                            break;
                        }
                        if lowl[iw as usize] < lowl[iv as usize] {
                            lowl[iv as usize] = lowl[iw as usize];
                        }
                        ii += 1;
                    }
                    
                    if !found {
                        ior[iv as usize] = -1;
                    }
                }
                
                if ior[iv as usize] == -1 {
                    if lowl[iv as usize] >= numb[iv as usize] {
                        num += 1;
                        let ist1 = n + 1 - ist;
                        let lcnt = icnt + 1;
                        let mut stp = ist1;
                        
                        while stp <= n {
                            let iw = ib[stp as usize];
                            lowl[iw as usize] = n + 1;
                            icnt += 1;
                            numb[iw as usize] = icnt;
                            if iw == iv {
                                break;
                            }
                            stp += 1;
                        }
                        
                        ist = n - stp;
                        ib[num as usize] = lcnt;
                        
                        if ist == 0 {
                            if icnt < n {
                                break;
                            } else {
                                break 'outer;
                            }
                        }
                    }
                    
                    let iw = iv;
                    iv = prev[iv as usize];
                    if lowl[iw as usize] < lowl[iv as usize] {
                        lowl[iv as usize] = lowl[iw as usize];
                    }
                }
                
                dummy += 1;
            }
        }
        isn += 1;
    }
    
    for i in 1..=n {
        ior[numb[i as usize] as usize] = i;
    }
    
    num
}