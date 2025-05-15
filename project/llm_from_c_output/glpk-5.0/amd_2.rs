use std::cmp::{max, min};
use std::i32::{MAX, MIN};

const EMPTY: i32 = -1;
const AMD_OK: f64 = 1.0;
const AMD_DEFAULT_DENSE: f64 = 10.0;
const AMD_DEFAULT_AGGRESSIVE: i32 = 1;

#[derive(Debug)]
struct ControlParams {
    dense: f64,
    aggressive: bool,
}

impl Default for ControlParams {
    fn default() -> Self {
        ControlParams {
            dense: AMD_DEFAULT_DENSE,
            aggressive: AMD_DEFAULT_AGGRESSIVE != 0,
        }
    }
}

fn clear_flag(wflg: i32, wbig: i32, w: &mut [i32], n: usize) -> i32 {
    let mut wflg = wflg;
    if wflg < 2 || wflg >= wbig {
        for x in 0..n {
            if w[x] != 0 {
                w[x] = 1;
            }
        }
        wflg = 2;
    }
    wflg
}

fn flip(i: i32) -> i32 {
    -i - 2
}

fn unflip(i: i32) -> i32 {
    if i < 0 {
        flip(i)
    } else {
        i
    }
}

fn amd_postorder(
    n: usize,
    parent: &mut [i32],
    nv: &[i32],
    elen: &mut [i32],
    w: &mut [i32],
    head: &mut [i32],
    next: &mut [i32],
    last: &mut [i32],
) {
    let mut i = 0;
    let mut nv = nv.to_vec();
    let mut elen = elen.to_vec();

    for i in 0..n {
        w[i] = 1;
    }

    for i in 0..n {
        if nv[i] == 0 {
            let mut j = parent[i];
            if j != EMPTY {
                while nv[j as usize] == 0 {
                    j = parent[j as usize];
                }
                w[j as usize] += w[i];
            }
        }
    }

    for i in 0..n {
        head[i] = EMPTY;
        last[i] = EMPTY;
        next[i] = EMPTY;
    }

    for i in 0..n {
        if nv[i] == 0 {
            let mut j = parent[i];
            if j != EMPTY {
                while nv[j as usize] == 0 {
                    j = parent[j as usize];
                }
                next[i] = head[j as usize];
                head[j as usize] = i as i32;
            }
        }
    }

    let mut k = 0;
    for i in 0..n {
        if nv[i] > 0 {
            let mut stack_top = EMPTY;
            let mut j = i as i32;
            'outer: loop {
                if w[j as usize] != 0 {
                    w[j as usize] = 0;
                    let mut child = head[j as usize];
                    if child != EMPTY {
                        head[j as usize] = next[child as usize];
                        next[j as usize] = stack_top;
                        stack_top = j;
                        j = child;
                        continue 'outer;
                    }
                }
                k += 1;
                w[j as usize] = k;
                if stack_top == EMPTY {
                    break;
                }
                let p = stack_top;
                stack_top = next[p as usize];
                j = p;
            }
        }
    }

    for i in 0..n {
        if nv[i] == 0 {
            w[i] = w[parent[i] as usize];
        }
    }
}

pub fn amd_2(
    n: usize,
    pe: &mut [i32],
    iw: &mut [i32],
    len: &mut [i32],
    iwlen: usize,
    pfree: &mut usize,
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
    let control_params = if let Some(ctrl) = control {
        ControlParams {
            dense: ctrl[0],
            aggressive: ctrl[1] != 0.0,
        }
    } else {
        ControlParams::default()
    };

    let mut lnz = 0.0;
    let mut ndiv = 0.0;
    let mut nms_lu = 0.0;
    let mut nms_ldl = 0.0;
    let mut dmax = 1.0;
    let mut me = EMPTY;
    let mut mindeg = 0;
    let mut ncmpa = 0;
    let mut nel = 0;
    let mut lemax = 0;
    let mut dense = if control_params.dense < 0.0 {
        (n - 2) as i32
    } else {
        (control_params.dense * (n as f64).sqrt()) as i32
    };
    dense = max(16, dense);
    dense = min(n as i32, dense);
    let mut ndense = 0;

    for i in 0..n {
        last[i] = EMPTY;
        head[i] = EMPTY;
        next[i] = EMPTY;
        nv[i] = 1;
        w[i] = 1;
        elen[i] = 0;
        degree[i] = len[i];
    }

    let mut wbig = MAX - n as i32;
    let mut wflg = clear_flag(0, wbig, w, n);

    for i in 0..n {
        let deg = degree[i];
        if deg == 0 {
            elen[i] = flip(1);
            nel += 1;
            pe[i] = EMPTY;
            w[i] = 0;
        } else if deg > dense {
            ndense += 1;
            nv[i] = 0;
            elen[i] = EMPTY;
            nel += 1;
            pe[i] = EMPTY;
        } else {
            let inext = head[deg as usize];
            if inext != EMPTY {
                last[inext as usize] = i as i32;
            }
            next[i] = inext;
            head[deg as usize] = i as i32;
        }
    }

    while nel < n {
        let mut deg = mindeg;
        while deg < n as i32 {
            me = head[deg as usize];
            if me != EMPTY {
                break;
            }
            deg += 1;
        }
        mindeg = deg;
        let mut inext = next[me as usize];
        if inext != EMPTY {
            last[inext as usize] = EMPTY;
        }
        head[deg as usize] = inext;

        let elenme = elen[me as usize];
        let nvpiv = nv[me as usize];
        nel += nvpiv;
        nv[me as usize] = -nvpiv;
        let mut degme = 0;
        let mut pme1 = pe[me as usize] as usize;
        let mut pme2 = pme1 - 1;

        if elenme == 0 {
            let mut p = pme1;
            while p <= pme1 + len[me as usize] - 1 {
                let i = iw[p];
                let nvi = nv[i as usize];
                if nvi > 0 {
                    degme += nvi;
                    nv[i as usize] = -nvi;
                    pme2 += 1;
                    iw[pme2] = i;
                    let ilast = last[i as usize];
                    let inext = next[i as usize];
                    if inext != EMPTY {
                        last[inext as usize] = ilast;
                    }
                    if ilast != EMPTY {
                        next[ilast as usize] = inext;
                    } else {
                        head[degree[i as usize] as usize] = inext;
                    }
                }
                p += 1;
            }
        } else {
            let mut p = pe[me as usize] as usize;
            pme1 = *pfree;
            let slenme = len[me as usize] - elenme;
            let mut knt1 = 1;
            while knt1 <= elenme + 1 {
                let (e, pj, ln);
                if knt1 > elenme {
                    e = me;
                    pj = p;
                    ln = slenme;
                } else {
                    e = iw[p];
                    p += 1;
                    pj = pe[e as usize] as usize;
                    ln = len[e as usize];
                }
                let mut knt2 = 1;
                while knt2 <= ln {
                    let i = iw[pj];
                    pj += 1;
                    let nvi = nv[i as usize];
                    if nvi > 0 {
                        if *pfree >= iwlen {
                            pe[me as usize] = p as i32;
                            len[me as usize] -= knt1;
                            if len[me as usize] == 0 {
                                pe[me as usize] = EMPTY;
                            }
                            pe[e as usize] = pj as i32;
                            len[e as usize] = ln - knt2;
                            if len[e as usize] == 0 {
                                pe[e as usize] = EMPTY;
                            }
                            ncmpa += 1;
                            for j in 0..n {
                                let mut pn = pe[j];
                                if pn >= 0 {
                                    pe[j] = iw[pn as usize];
                                    iw[pn as usize] = flip(j as i32);
                                }
                            }
                            let mut psrc = 0;
                            let mut pdst = 0;
                            let pend = pme1 - 1;
                            while psrc <= pend {
                                let j = flip(iw[psrc]);
                                psrc += 1;
                                if j >= 0 {
                                    iw[pdst] = pe[j as usize];
                                    pe[j as usize] = pdst as i32;
                                    pdst += 1;
                                    let lenj = len[j as usize];
                                    for _ in 0..lenj - 1 {
                                        iw[pdst] = iw[psrc];
                                        pdst += 1;
                                        psrc += 1;
                                    }
                                }
                            }
                            let mut p1 = pdst;
                            for psrc in pme1..*pfree {
                                iw[pdst] = iw[psrc];
                                pdst += 1;
                                psrc += 1;
                            }
                            pme1 = p1;
                            *pfree = pdst;
                            pj = pe[e as usize] as usize;
                            p = pe[me as usize] as usize;
                        }
                        degme += nvi;
                        nv[i as usize] = -nvi;
                        iw[*pfree] = i;
                        *pfree += 1;
                        let ilast = last[i as usize];
                        let inext = next[i as usize];
                        if inext != EMPTY {
                            last[inext as usize] = ilast;
                        }
                        if ilast != EMPTY {
                            next[ilast as usize] = inext;
                        } else {
                            head[degree[i as usize] as usize] = inext;
                        }
                    }
                    knt2 += 1;
                }
                if e != me {
                    pe[e as usize] = flip(me);
                    w[e as usize] = 0;
                }
                knt1 += 1;
            }
            pme2 = *pfree - 1;
        }
        degree[me as usize] = degme;
        pe[me as usize] = pme1 as i32;
        len[me as usize] = (pme2 - pme1 + 1) as i32;
        elen[me as usize] = flip((nvpiv + degme) as i32);
        wflg = clear_flag(wflg, wbig, w, n);

        for pme in pme1..=pme2 {
            let i = iw[pme];
            let eln = elen[i as usize];
            if eln > 0 {
                let nvi = -nv[i as usize];
                let wnvi = wflg - nvi;
                let mut p = pe[i as usize] as usize;
                while p <= pe[i as usize] as usize + eln - 1 {
                    let e = iw[p];
                    let we = w[e as usize];
                    if we >= wflg {
                        w[e as usize] = we - nvi;
                    } else if we != 0 {
                        w[e as usize] = degree[e as usize] + wnvi;
                    }
                    p += 1;
                }
            }
        }

        for pme in pme1..=pme2 {
            let i = iw[pme];
            if nv[i as usize] < 0 {
                let p1 = pe[i as usize] as usize;
                let p2 = p1 + elen[i as usize] as usize - 1;
                let mut pn = p1;
                let mut hash = 0;
                let mut deg = 0;
                if control_params.aggressive {
                    let mut p = p1;
                    while p <= p2 {
                        let e = iw[p];
                        let we = w[e as usize];
                        if we != 0 {
                            let dext = we - wflg;
                            if dext > 0 {
                                deg += dext;
                                iw[pn] = e;
                                pn += 1;
                                hash += e;
                            } else {
                                pe[e as usize] = flip(me);
                                w[e as usize] = 0;
                            }
                        }
                        p += 1;
                    }
                } else {
                    let mut p = p1;
                    while p <= p2 {
                        let e = iw[p];
                        let we = w[e as usize];
                        if we != 0 {
                            let dext = we - wflg;
                            deg += dext;
                            iw[pn] = e;
                            pn += 1;
                            hash += e;
                        }
                        p += 1;
                    }
                }
                elen[i as usize] = (pn - p1 + 1) as i32;
                let p3 = pn;
                let p4 = p1 + len[i as usize] as usize;
                let mut p = p2 + 1;
                while p < p4 {
                    let j = iw[p];
                    let nvj = nv[j as usize];
                    if nvj > 0 {
                        deg += nvj;
                        iw[pn] = j;
                        pn += 1;
                        hash += j;
                    }
                    p += 1;
                }
                if elen[i as usize] == 1 && p3 == pn {
                    pe[i as usize] = flip(me);
                    let nvi = -nv[i as usize];
                    degme -= nvi;
                    nvpiv += nvi;
                    nel += nvi;
                    nv[i as usize] = 0;
                    elen[i as usize] = EMPTY;
                } else {
                    degree[i as usize] = min(degree[i as usize], deg);
                    iw[pn] = iw[p3];
                    iw[p3] = iw[p1];
                    iw[p1] = me;
                    len[i as usize] = (pn - p1 + 1) as i32;
                    hash %= n as i32;
                    let j = head[hash as usize];
                    if j <= EMPTY {
                        next[i as usize] = flip(j);
                        head[hash as usize] = flip(i as i32);
                    } else {
                        next[i as usize] = last[j as usize];
                        last[j as usize] = i as i32;
                    }
                    last[i as usize] = hash;
                }
            }
        }
        degree[me as usize] = degme;
        lemax = max(lemax, degme);
        wflg += lemax;
        wflg = clear_flag(wflg, wbig, w, n);

        for pme in pme1..=pme2 {
            let i = iw[pme];
            if nv[i as usize] < 0 {
                let hash = last[i as usize];
                let mut j = head[hash as usize];
                if j == EMPTY {
                    continue;
                } else if j < EMPTY {
                    head[hash as usize] = EMPTY;
                    j = flip(j);
                } else {
                    head[hash as usize] = EMPTY;
                }
                let mut jlast = i;
                let mut j = next[j as usize];
                while j != EMPTY {
                    let ln = len[j as usize];
                    let eln = elen[j as usize];
                    let mut ok = (ln == len[i as usize]) && (eln == elen[i as usize]);
                    let mut p = pe[j as usize] as usize + 1;
                    while ok && p <= pe[j as usize] as usize + ln - 1 {
                        if w[iw[p] as usize] != wflg as usize {
                            ok = false;
                        }
                        p += 1;
                    }
                    if ok {
                        pe[j as usize] = flip(i as i32);
                        nv[i as usize] += nv[j as usize];
                        nv[j as usize] = 0;
                        elen[j as usize] = EMPTY;
                        let jnext = next[j as usize];
                        next[j as usize] = next[jlast as usize];
                        j = jnext;
                    } else {
                        jlast = j;
                        j = next[j as usize];
                    }
                }
                wflg += 1;
            }
        }

        let mut p = pme1;
        let nleft = n - nel;
        for pme in pme1..=pme2 {
            let i = iw[pme];
            let nvi = -nv[i as usize];
            if nvi > 0 {
                nv[i as usize] = nvi;
                let mut deg