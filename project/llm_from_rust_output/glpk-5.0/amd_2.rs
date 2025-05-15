use std::cmp::{max, min};

fn sqrt(x: f64) -> f64 {
    x.sqrt()
}

fn clear_flag(wflg: i32, wbig: i32, w: &mut [i32]) -> i32 {
    let mut wflg = wflg;
    if wflg < 2 || wflg >= wbig {
        for val in w.iter_mut() {
            if *val != 0 {
                *val = 1;
            }
        }
        wflg = 2;
    }
    wflg
}

fn glp_amd_postorder(
    nn: i32,
    parent: &mut [i32],
    npiv: &mut [i32],
    fsize: &mut [i32],
    order: &mut [i32],
    child: &mut [i32],
    sibling: &mut [i32],
    stack: &mut [i32],
) {
    // Implementation of postorder would go here
}

pub fn glp_amd_2(
    n: i32,
    pe: &mut [i32],
    iw: &mut [i32],
    len: &mut [i32],
    iwlen: i32,
    mut pfree: i32,
    nv: &mut [i32],
    next: &mut [i32],
    last: &mut [i32],
    head: &mut [i32],
    elen: &mut [i32],
    degree: &mut [i32],
    w: &mut [i32],
    control: Option<&[f64]>,
    info: Option<&mut [f64]>,
) {
    let mut lnz = 0.0;
    let mut ndiv = 0.0;
    let mut nms_lu = 0.0;
    let mut nms_ldl = 0.0;
    let mut dmax = 1.0;
    let mut me = -1;
    let mut mindeg = 0;
    let mut ncmpa = 0;
    let mut nel = 0;
    let mut lemax = 0;

    let (alpha, aggressive) = match control {
        Some(ctrl) => (
            ctrl[0],
            ctrl[1] != 0.0,
        ),
        None => (10.0, true),
    };

    let dense = if alpha < 0.0 {
        n - 2
    } else {
        (alpha * sqrt(n as f64)) as i32
    }.max(16).min(n);

    for i in 0..n as usize {
        last[i] = -1;
        head[i] = -1;
        next[i] = -1;
        nv[i] = 1;
        w[i] = 1;
        elen[i] = 0;
        degree[i] = len[i];
    }

    let mut wbig = i32::MAX - n;
    let mut wflg = clear_flag(0, wbig, w);
    let mut ndense = 0;

    for i in 0..n as usize {
        let deg = degree[i];
        if deg == 0 {
            elen[i] = -1 - 2;
            nel += 1;
            pe[i] = -1;
            w[i] = 0;
        } else if deg > dense {
            ndense += 1;
            nv[i] = 0;
            elen[i] = -1;
            nel += 1;
            pe[i] = -1;
        } else {
            let inext = head[deg as usize];
            if inext != -1 {
                last[inext as usize] = i as i32;
            }
            next[i] = inext;
            head[deg as usize] = i as i32;
        }
    }

    while nel < n {
        let mut deg = mindeg;
        loop {
            me = head[deg as usize];
            if me != -1 {
                break;
            }
            deg += 1;
        }
        mindeg = deg;

        let inext = next[me as usize];
        if inext != -1 {
            last[inext as usize] = -1;
        }
        head[deg as usize] = inext;

        let elenme = elen[me as usize];
        let mut nvpiv = nv[me as usize];
        nel += nvpiv;
        nv[me as usize] = -nvpiv;

        let mut degme = 0;
        let (pme1, pme2) = if elenme == 0 {
            let pme1 = pe[me as usize];
            let mut pme2 = pme1 - 1;
            let mut p = pme1;
            while p <= pme1 + len[me as usize] - 1 {
                let i = iw[p as usize];
                let nvi = nv[i as usize];
                if nvi > 0 {
                    degme += nvi;
                    nv[i as usize] = -nvi;
                    pme2 += 1;
                    iw[pme2 as usize] = i;
                    let ilast = last[i as usize];
                    let inext = next[i as usize];
                    if inext != -1 {
                        last[inext as usize] = ilast;
                    }
                    if ilast != -1 {
                        next[ilast as usize] = inext;
                    } else {
                        head[degree[i as usize] as usize] = inext;
                    }
                }
                p += 1;
            }
            (pme1, pme2)
        } else {
            let p = pe[me as usize];
            let pme1 = pfree;
            let slenme = len[me as usize] - elenme;
            let mut knt1 = 1;
            let mut pme2 = pfree - 1;

            while knt1 <= elenme + 1 {
                let (e, pj, ln) = if knt1 > elenme {
                    (me, p, slenme)
                } else {
                    let e = iw[p as usize];
                    p += 1;
                    (e, pe[e as usize], len[e as usize])
                };

                let mut knt2 = 1;
                while knt2 <= ln {
                    let i = iw[pj as usize];
                    pj += 1;
                    let nvi = nv[i as usize];
                    if nvi > 0 {
                        if pfree >= iwlen {
                            pe[me as usize] = p;
                            len[me as usize] -= knt1;
                            if len[me as usize] == 0 {
                                pe[me as usize] = -1;
                            }
                            pe[e as usize] = pj;
                            len[e as usize] = ln - knt2;
                            if len[e as usize] == 0 {
                                pe[e as usize] = -1;
                            }
                            ncmpa += 1;

                            for j in 0..n as usize {
                                let pn = pe[j];
                                if pn >= 0 {
                                    pe[j] = iw[pn as usize];
                                    iw[pn as usize] = -(j as i32) - 2;
                                }
                            }

                            let mut psrc = 0;
                            let mut pdst = 0;
                            let pend = pme1 - 1;
                            while psrc <= pend {
                                let j = (-iw[psrc] - 2) as usize;
                                psrc += 1;
                                if j < n as usize {
                                    iw[pdst] = pe[j];
                                    pdst += 1;
                                    pe[j] = pdst as i32 - 1;
                                    let lenj = len[j];
                                    let mut knt3 = 0;
                                    while knt3 <= lenj - 2 {
                                        iw[pdst] = iw[psrc];
                                        pdst += 1;
                                        psrc += 1;
                                        knt3 += 1;
                                    }
                                }
                            }

                            let p1 = pdst;
                            let mut psrc = pme1;
                            while psrc <= pfree - 1 {
                                iw[pdst] = iw[psrc];
                                pdst += 1;
                                psrc += 1;
                            }
                            pme1 = p1;
                            pfree = pdst;
                            pj = pe[e as usize];
                            p = pe[me as usize];
                        }
                        degme += nvi;
                        nv[i as usize] = -nvi;
                        iw[pfree as usize] = i;
                        pfree += 1;
                        let ilast = last[i as usize];
                        let inext = next[i as usize];
                        if inext != -1 {
                            last[inext as usize] = ilast;
                        }
                        if ilast != -1 {
                            next[ilast as usize] = inext;
                        } else {
                            head[degree[i as usize] as usize] = inext;
                        }
                    }
                    knt2 += 1;
                }
                if e != me {
                    pe[e as usize] = -me - 2;
                    w[e as usize] = 0;
                }
                knt1 += 1;
            }
            (pme1, pme2)
        };

        degree[me as usize] = degme;
        pe[me as usize] = pme1;
        len[me as usize] = pme2 - pme1 + 1;
        elen[me as usize] = -(nvpiv + degme) - 2;
        wflg = clear_flag(wflg, wbig, w);

        let mut pme = pme1;
        while pme <= pme2 {
            let i = iw[pme as usize];
            let eln = elen[i as usize];
            if eln > 0 {
                let nvi = -nv[i as usize];
                let wnvi = wflg - nvi;
                let mut p = pe[i as usize] + 1;
                while p <= pe[i as usize] + eln - 1 {
                    let e = iw[p as usize];
                    let we = w[e as usize];
                    if we >= wflg {
                        w[e as usize] = we - nvi;
                    } else if we != 0 {
                        w[e as usize] = degree[e as usize] + wnvi;
                    }
                    p += 1;
                }
            }
            pme += 1;
        }

        pme = pme1;
        while pme <= pme2 {
            let i = iw[pme as usize];
            let p1 = pe[i as usize];
            let p2 = p1 + elen[i as usize] - 1;
            let mut pn = p1;
            let mut hash = 0u32;
            let mut deg = 0;

            if aggressive {
                let mut p = p1;
                while p <= p2 {
                    let e = iw[p as usize];
                    let we = w[e as usize];
                    if we != 0 {
                        let dext = we - wflg;
                        if dext > 0 {
                            deg += dext;
                            iw[pn as usize] = e;
                            pn += 1;
                            hash = hash.wrapping_add(e as u32);
                        } else {
                            pe[e as usize] = -me - 2;
                            w[e as usize] = 0;
                        }
                    }
                    p += 1;
                }
            } else {
                let mut p = p1;
                while p <= p2 {
                    let e = iw[p as usize];
                    let we = w[e as usize];
                    if we != 0 {
                        let dext = we - wflg;
                        deg += dext;
                        iw[pn as usize] = e;
                        pn += 1;
                        hash = hash.wrapping_add(e as u32);
                    }
                    p += 1;
                }
            }

            elen[i as usize] = (pn - p1 + 1) as i32;
            let p3 = pn;
            let p4 = p1 + len[i as usize];
            let mut p = p2 + 1;
            while p < p4 {
                let j = iw[p as usize];
                let nvj = nv[j as usize];
                if nvj > 0 {
                    deg += nvj;
                    iw[pn as usize] = j;
                    pn += 1;
                    hash = hash.wrapping_add(j as u32);
                }
                p += 1;
            }

            if elen[i as usize] == 1 && p3 == pn {
                pe[i as usize] = -me - 2;
                let nvi = -nv[i as usize];
                degme -= nvi;
                nvpiv += nvi;
                nel += nvi;
                nv[i as usize] = 0;
                elen[i as usize] = -1;
            } else {
                degree[i as usize] = min(degree[i as usize], deg);
                iw[pn as usize] = iw[p3 as usize];
                iw[p3 as usize] = iw[p1 as usize];
                iw[p1 as usize] = me;
                len[i as usize] = (pn - p1 + 1) as i32;
                hash = hash % (n as u32);
                let j = head[hash as usize];
                if j <= -1 {
                    next[i as usize] = -j - 2;
                    head[hash as usize] = -(i as i32) - 2;
                } else {
                    next[i as usize] = last[j as usize];
                    last[j as usize] = i as i32;
                }
                last[i as usize] = hash as i32;
            }
            pme += 1;
        }

        degree[me as usize] = degme;
        lemax = max(lemax, degme);
        wflg += lemax;
        wflg = clear_flag(wflg, wbig, w);

        pme = pme1;
        while pme <= pme2 {
            let i = iw[pme as usize];
            if nv[i as usize] < 0 {
                let hash = last[i as usize] as u32;
                let mut j = head[hash as usize];
                if j == -1 {
                    continue;
                } else if j < -1 {
                    j = -j - 2;
                    head[hash as usize] = -1;
                } else {
                    j = last[j as usize];
                    last[j as usize] = -1;
                }

                while j != -1 && next[j as usize] != -1 {
                    let ln = len[j as usize];
                    let eln = elen[j as usize];
                    let mut p = pe[j as usize] + 1;
                    while p <= pe[j as usize] + ln - 1 {
                        w[iw[p as usize] as usize] = wflg;
                        p += 1;
                    }

                    let mut jlast = j;
                    let mut jnext = next[j as usize];
                    while jnext != -1 {
                        let mut ok = (len[jnext as usize] == ln && elen[jnext as usize] == eln) as i32;
                        let mut p = pe[jnext as usize] + 1;
                        while ok != 0 && p <= pe[jnext as usize] + ln - 1 {
                            if w[iw[p as usize] as usize] != wflg {
                                ok = 0;
                            }
                            p += 1;
                        }

                        if ok != 0 {
                            pe[jnext as usize] = -(j as i32) - 2;
                            nv[j as usize] += nv[jnext as usize];
                            nv[jnext as usize] = 0;
                            elen[jnext as usize] = -1;
                            jnext = next[jnext as usize];
                            next[jlast as usize] = jnext;
                        } else {
                            jlast = jnext;
                            jnext = next[jnext as usize];
                        }
                    }
                    wflg += 1;
                    j = next[j as usize];
                }
            }
            pme += 1;
        }

        let mut p = pme1;
        let nleft = n - nel;
        pme = pme1;
        while pme <= pme2 {
            let i = iw[pme as usize];
            let nvi = -nv[i as usize];
            if nvi > 0 {
                nv[i as usize] = nvi;
                let mut deg = degree[i as usize] + degme - nvi;
                deg = min(deg, nleft - nvi);
                let inext = head[deg as usize];
                if inext != -1 {
                    last[inext as usize] = i as i32;
                }
                next[i as usize] = inext;
                last[i as usize] = -1;
                head[deg as usize] = i as i32;
                mindeg = min(mindeg, deg);
                degree[i as usize] = deg;
                iw[p as usize] = i as i32;
                p += 1;
            }
            pme += 1;
        }

        nv[me as usize] = nvpiv;
        len[me as usize] = (p - pme1) as i32;
        if len[me as usize] == 0 {
            pe[me as usize] = -1;
            w[me as usize] = 0;
        }
        if elenme != 0 {
            pfree = p;
        }

        if let Some(info) = info {
            let f = nvpiv as f64;
            let r = (degme + ndense) as f64;
            dmax = max(dmax, f + r);
            let lnzme = f * r + (f - 1.0) * f / 2.0;
            lnz += lnzme;
            ndiv += lnzme;
            let s = f * r * r + r * (f - 1.0) * f + (f - 1.0) * f * (2.0 * f - 1.0) / 6.0;
            nms_lu += s;
            nms_ldl += (s + lnzme) / 2.0;
        }
    }

    if let Some(info) = info {
        let f = ndense as f64;
        dmax = max(dmax, f);
        let lnzme = (