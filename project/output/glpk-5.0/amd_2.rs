#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn _glp_amd_postorder(
        nn: i32,
        Parent: *mut i32,
        Npiv: *mut i32,
        Fsize: *mut i32,
        Order: *mut i32,
        Child: *mut i32,
        Sibling: *mut i32,
        Stack: *mut i32,
    );
}
unsafe extern "C" fn clear_flag(
    mut wflg: i32,
    mut wbig: i32,
    mut W: *mut i32,
    mut n: i32,
) -> i32 {
    let mut x: i32 = 0;
    if wflg < 2 as i32 || wflg >= wbig {
        x = 0 as i32;
        while x < n {
            if *W.offset(x as isize) != 0 as i32 {
                *W.offset(x as isize) = 1 as i32;
            }
            x += 1;
            x;
        }
        wflg = 2 as i32;
    }
    return wflg;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_amd_2(
    mut n: i32,
    mut Pe: *mut i32,
    mut Iw: *mut i32,
    mut Len: *mut i32,
    mut iwlen: i32,
    mut pfree: i32,
    mut Nv: *mut i32,
    mut Next: *mut i32,
    mut Last: *mut i32,
    mut Head: *mut i32,
    mut Elen: *mut i32,
    mut Degree: *mut i32,
    mut W: *mut i32,
    mut Control: *mut libc::c_double,
    mut Info: *mut libc::c_double,
) {
    let mut deg: i32 = 0;
    let mut degme: i32 = 0;
    let mut dext: i32 = 0;
    let mut lemax: i32 = 0;
    let mut e: i32 = 0;
    let mut elenme: i32 = 0;
    let mut eln: i32 = 0;
    let mut i: i32 = 0;
    let mut ilast: i32 = 0;
    let mut inext: i32 = 0;
    let mut j: i32 = 0;
    let mut jlast: i32 = 0;
    let mut jnext: i32 = 0;
    let mut k: i32 = 0;
    let mut knt1: i32 = 0;
    let mut knt2: i32 = 0;
    let mut knt3: i32 = 0;
    let mut lenj: i32 = 0;
    let mut ln: i32 = 0;
    let mut me: i32 = 0;
    let mut mindeg: i32 = 0;
    let mut nel: i32 = 0;
    let mut nleft: i32 = 0;
    let mut nvi: i32 = 0;
    let mut nvj: i32 = 0;
    let mut nvpiv: i32 = 0;
    let mut slenme: i32 = 0;
    let mut wbig: i32 = 0;
    let mut we: i32 = 0;
    let mut wflg: i32 = 0;
    let mut wnvi: i32 = 0;
    let mut ok: i32 = 0;
    let mut ndense: i32 = 0;
    let mut ncmpa: i32 = 0;
    let mut dense: i32 = 0;
    let mut aggressive: i32 = 0;
    let mut hash: u32 = 0;
    let mut f: libc::c_double = 0.;
    let mut r: libc::c_double = 0.;
    let mut ndiv: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    let mut nms_lu: libc::c_double = 0.;
    let mut nms_ldl: libc::c_double = 0.;
    let mut dmax: libc::c_double = 0.;
    let mut alpha: libc::c_double = 0.;
    let mut lnz: libc::c_double = 0.;
    let mut lnzme: libc::c_double = 0.;
    let mut p: i32 = 0;
    let mut p1: i32 = 0;
    let mut p2: i32 = 0;
    let mut p3: i32 = 0;
    let mut p4: i32 = 0;
    let mut pdst: i32 = 0;
    let mut pend: i32 = 0;
    let mut pj: i32 = 0;
    let mut pme: i32 = 0;
    let mut pme1: i32 = 0;
    let mut pme2: i32 = 0;
    let mut pn: i32 = 0;
    let mut psrc: i32 = 0;
    lnz = 0 as i32 as libc::c_double;
    ndiv = 0 as i32 as libc::c_double;
    nms_lu = 0 as i32 as libc::c_double;
    nms_ldl = 0 as i32 as libc::c_double;
    dmax = 1 as i32 as libc::c_double;
    me = -(1 as i32);
    mindeg = 0 as i32;
    ncmpa = 0 as i32;
    nel = 0 as i32;
    lemax = 0 as i32;
    if !Control.is_null() {
        alpha = *Control.offset(0 as i32 as isize);
        aggressive = (*Control.offset(1 as i32 as isize) != 0 as i32 as libc::c_double)
            as i32;
    } else {
        alpha = 10.0f64;
        aggressive = 1 as i32;
    }
    if alpha < 0 as i32 as libc::c_double {
        dense = n - 2 as i32;
    } else {
        dense = (alpha * sqrt(n as libc::c_double)) as i32;
    }
    dense = if 16 as i32 > dense { 16 as i32 } else { dense };
    dense = if n < dense { n } else { dense };
    i = 0 as i32;
    while i < n {
        *Last.offset(i as isize) = -(1 as i32);
        *Head.offset(i as isize) = -(1 as i32);
        *Next.offset(i as isize) = -(1 as i32);
        *Nv.offset(i as isize) = 1 as i32;
        *W.offset(i as isize) = 1 as i32;
        *Elen.offset(i as isize) = 0 as i32;
        *Degree.offset(i as isize) = *Len.offset(i as isize);
        i += 1;
        i;
    }
    wbig = 2147483647 as i32 - n;
    wflg = clear_flag(0 as i32, wbig, W, n);
    ndense = 0 as i32;
    i = 0 as i32;
    while i < n {
        deg = *Degree.offset(i as isize);
        if deg == 0 as i32 {
            *Elen.offset(i as isize) = -(1 as i32) - 2 as i32;
            nel += 1;
            nel;
            *Pe.offset(i as isize) = -(1 as i32);
            *W.offset(i as isize) = 0 as i32;
        } else if deg > dense {
            ndense += 1;
            ndense;
            *Nv.offset(i as isize) = 0 as i32;
            *Elen.offset(i as isize) = -(1 as i32);
            nel += 1;
            nel;
            *Pe.offset(i as isize) = -(1 as i32);
        } else {
            inext = *Head.offset(deg as isize);
            if inext != -(1 as i32) {
                *Last.offset(inext as isize) = i;
            }
            *Next.offset(i as isize) = inext;
            *Head.offset(deg as isize) = i;
        }
        i += 1;
        i;
    }
    while nel < n {
        deg = mindeg;
        while deg < n {
            me = *Head.offset(deg as isize);
            if me != -(1 as i32) {
                break;
            }
            deg += 1;
            deg;
        }
        mindeg = deg;
        inext = *Next.offset(me as isize);
        if inext != -(1 as i32) {
            *Last.offset(inext as isize) = -(1 as i32);
        }
        *Head.offset(deg as isize) = inext;
        elenme = *Elen.offset(me as isize);
        nvpiv = *Nv.offset(me as isize);
        nel += nvpiv;
        *Nv.offset(me as isize) = -nvpiv;
        degme = 0 as i32;
        if elenme == 0 as i32 {
            pme1 = *Pe.offset(me as isize);
            pme2 = pme1 - 1 as i32;
            p = pme1;
            while p <= pme1 + *Len.offset(me as isize) - 1 as i32 {
                i = *Iw.offset(p as isize);
                nvi = *Nv.offset(i as isize);
                if nvi > 0 as i32 {
                    degme += nvi;
                    *Nv.offset(i as isize) = -nvi;
                    pme2 += 1;
                    *Iw.offset(pme2 as isize) = i;
                    ilast = *Last.offset(i as isize);
                    inext = *Next.offset(i as isize);
                    if inext != -(1 as i32) {
                        *Last.offset(inext as isize) = ilast;
                    }
                    if ilast != -(1 as i32) {
                        *Next.offset(ilast as isize) = inext;
                    } else {
                        *Head.offset(*Degree.offset(i as isize) as isize) = inext;
                    }
                }
                p += 1;
                p;
            }
        } else {
            p = *Pe.offset(me as isize);
            pme1 = pfree;
            slenme = *Len.offset(me as isize) - elenme;
            knt1 = 1 as i32;
            while knt1 <= elenme + 1 as i32 {
                if knt1 > elenme {
                    e = me;
                    pj = p;
                    ln = slenme;
                } else {
                    let fresh0 = p;
                    p = p + 1;
                    e = *Iw.offset(fresh0 as isize);
                    pj = *Pe.offset(e as isize);
                    ln = *Len.offset(e as isize);
                }
                knt2 = 1 as i32;
                while knt2 <= ln {
                    let fresh1 = pj;
                    pj = pj + 1;
                    i = *Iw.offset(fresh1 as isize);
                    nvi = *Nv.offset(i as isize);
                    if nvi > 0 as i32 {
                        if pfree >= iwlen {
                            *Pe.offset(me as isize) = p;
                            *Len.offset(me as isize) -= knt1;
                            if *Len.offset(me as isize) == 0 as i32 {
                                *Pe.offset(me as isize) = -(1 as i32);
                            }
                            *Pe.offset(e as isize) = pj;
                            *Len.offset(e as isize) = ln - knt2;
                            if *Len.offset(e as isize) == 0 as i32 {
                                *Pe.offset(e as isize) = -(1 as i32);
                            }
                            ncmpa += 1;
                            ncmpa;
                            j = 0 as i32;
                            while j < n {
                                pn = *Pe.offset(j as isize);
                                if pn >= 0 as i32 {
                                    *Pe.offset(j as isize) = *Iw.offset(pn as isize);
                                    *Iw.offset(pn as isize) = -j - 2 as i32;
                                }
                                j += 1;
                                j;
                            }
                            psrc = 0 as i32;
                            pdst = 0 as i32;
                            pend = pme1 - 1 as i32;
                            while psrc <= pend {
                                let fresh2 = psrc;
                                psrc = psrc + 1;
                                j = -*Iw.offset(fresh2 as isize) - 2 as i32;
                                if j >= 0 as i32 {
                                    *Iw.offset(pdst as isize) = *Pe.offset(j as isize);
                                    let fresh3 = pdst;
                                    pdst = pdst + 1;
                                    *Pe.offset(j as isize) = fresh3;
                                    lenj = *Len.offset(j as isize);
                                    knt3 = 0 as i32;
                                    while knt3 <= lenj - 2 as i32 {
                                        let fresh4 = psrc;
                                        psrc = psrc + 1;
                                        let fresh5 = pdst;
                                        pdst = pdst + 1;
                                        *Iw.offset(fresh5 as isize) = *Iw.offset(fresh4 as isize);
                                        knt3 += 1;
                                        knt3;
                                    }
                                }
                            }
                            p1 = pdst;
                            psrc = pme1;
                            while psrc <= pfree - 1 as i32 {
                                let fresh6 = pdst;
                                pdst = pdst + 1;
                                *Iw.offset(fresh6 as isize) = *Iw.offset(psrc as isize);
                                psrc += 1;
                                psrc;
                            }
                            pme1 = p1;
                            pfree = pdst;
                            pj = *Pe.offset(e as isize);
                            p = *Pe.offset(me as isize);
                        }
                        degme += nvi;
                        *Nv.offset(i as isize) = -nvi;
                        let fresh7 = pfree;
                        pfree = pfree + 1;
                        *Iw.offset(fresh7 as isize) = i;
                        ilast = *Last.offset(i as isize);
                        inext = *Next.offset(i as isize);
                        if inext != -(1 as i32) {
                            *Last.offset(inext as isize) = ilast;
                        }
                        if ilast != -(1 as i32) {
                            *Next.offset(ilast as isize) = inext;
                        } else {
                            *Head.offset(*Degree.offset(i as isize) as isize) = inext;
                        }
                    }
                    knt2 += 1;
                    knt2;
                }
                if e != me {
                    *Pe.offset(e as isize) = -me - 2 as i32;
                    *W.offset(e as isize) = 0 as i32;
                }
                knt1 += 1;
                knt1;
            }
            pme2 = pfree - 1 as i32;
        }
        *Degree.offset(me as isize) = degme;
        *Pe.offset(me as isize) = pme1;
        *Len.offset(me as isize) = pme2 - pme1 + 1 as i32;
        *Elen.offset(me as isize) = -(nvpiv + degme) - 2 as i32;
        wflg = clear_flag(wflg, wbig, W, n);
        pme = pme1;
        while pme <= pme2 {
            i = *Iw.offset(pme as isize);
            eln = *Elen.offset(i as isize);
            if eln > 0 as i32 {
                nvi = -*Nv.offset(i as isize);
                wnvi = wflg - nvi;
                p = *Pe.offset(i as isize);
                while p <= *Pe.offset(i as isize) + eln - 1 as i32 {
                    e = *Iw.offset(p as isize);
                    we = *W.offset(e as isize);
                    if we >= wflg {
                        we -= nvi;
                    } else if we != 0 as i32 {
                        we = *Degree.offset(e as isize) + wnvi;
                    }
                    *W.offset(e as isize) = we;
                    p += 1;
                    p;
                }
            }
            pme += 1;
            pme;
        }
        pme = pme1;
        while pme <= pme2 {
            i = *Iw.offset(pme as isize);
            p1 = *Pe.offset(i as isize);
            p2 = p1 + *Elen.offset(i as isize) - 1 as i32;
            pn = p1;
            hash = 0 as i32 as u32;
            deg = 0 as i32;
            if aggressive != 0 {
                p = p1;
                while p <= p2 {
                    e = *Iw.offset(p as isize);
                    we = *W.offset(e as isize);
                    if we != 0 as i32 {
                        dext = we - wflg;
                        if dext > 0 as i32 {
                            deg += dext;
                            let fresh8 = pn;
                            pn = pn + 1;
                            *Iw.offset(fresh8 as isize) = e;
                            hash = hash.wrapping_add(e as u32);
                        } else {
                            *Pe.offset(e as isize) = -me - 2 as i32;
                            *W.offset(e as isize) = 0 as i32;
                        }
                    }
                    p += 1;
                    p;
                }
            } else {
                p = p1;
                while p <= p2 {
                    e = *Iw.offset(p as isize);
                    we = *W.offset(e as isize);
                    if we != 0 as i32 {
                        dext = we - wflg;
                        deg += dext;
                        let fresh9 = pn;
                        pn = pn + 1;
                        *Iw.offset(fresh9 as isize) = e;
                        hash = hash.wrapping_add(e as u32);
                    }
                    p += 1;
                    p;
                }
            }
            *Elen.offset(i as isize) = pn - p1 + 1 as i32;
            p3 = pn;
            p4 = p1 + *Len.offset(i as isize);
            p = p2 + 1 as i32;
            while p < p4 {
                j = *Iw.offset(p as isize);
                nvj = *Nv.offset(j as isize);
                if nvj > 0 as i32 {
                    deg += nvj;
                    let fresh10 = pn;
                    pn = pn + 1;
                    *Iw.offset(fresh10 as isize) = j;
                    hash = hash.wrapping_add(j as u32);
                }
                p += 1;
                p;
            }
            if *Elen.offset(i as isize) == 1 as i32 && p3 == pn {
                *Pe.offset(i as isize) = -me - 2 as i32;
                nvi = -*Nv.offset(i as isize);
                degme -= nvi;
                nvpiv += nvi;
                nel += nvi;
                *Nv.offset(i as isize) = 0 as i32;
                *Elen.offset(i as isize) = -(1 as i32);
            } else {
                *Degree.offset(i as isize) = if *Degree.offset(i as isize) < deg {
                    *Degree.offset(i as isize)
                } else {
                    deg
                };
                *Iw.offset(pn as isize) = *Iw.offset(p3 as isize);
                *Iw.offset(p3 as isize) = *Iw.offset(p1 as isize);
                *Iw.offset(p1 as isize) = me;
                *Len.offset(i as isize) = pn - p1 + 1 as i32;
                hash = hash.wrapping_rem(n as u32);
                j = *Head.offset(hash as isize);
                if j <= -(1 as i32) {
                    *Next.offset(i as isize) = -j - 2 as i32;
                    *Head.offset(hash as isize) = -i - 2 as i32;
                } else {
                    *Next.offset(i as isize) = *Last.offset(j as isize);
                    *Last.offset(j as isize) = i;
                }
                *Last.offset(i as isize) = hash as i32;
            }
            pme += 1;
            pme;
        }
        *Degree.offset(me as isize) = degme;
        lemax = if lemax > degme { lemax } else { degme };
        wflg += lemax;
        wflg = clear_flag(wflg, wbig, W, n);
        pme = pme1;
        while pme <= pme2 {
            i = *Iw.offset(pme as isize);
            if *Nv.offset(i as isize) < 0 as i32 {
                hash = *Last.offset(i as isize) as u32;
                j = *Head.offset(hash as isize);
                if j == -(1 as i32) {
                    i = -(1 as i32);
                } else if j < -(1 as i32) {
                    i = -j - 2 as i32;
                    *Head.offset(hash as isize) = -(1 as i32);
                } else {
                    i = *Last.offset(j as isize);
                    *Last.offset(j as isize) = -(1 as i32);
                }
                while i != -(1 as i32) && *Next.offset(i as isize) != -(1 as i32) {
                    ln = *Len.offset(i as isize);
                    eln = *Elen.offset(i as isize);
                    p = *Pe.offset(i as isize) + 1 as i32;
                    while p <= *Pe.offset(i as isize) + ln - 1 as i32 {
                        *W.offset(*Iw.offset(p as isize) as isize) = wflg;
                        p += 1;
                        p;
                    }
                    jlast = i;
                    j = *Next.offset(i as isize);
                    while j != -(1 as i32) {
                        ok = (*Len.offset(j as isize) == ln
                            && *Elen.offset(j as isize) == eln) as i32;
                        p = *Pe.offset(j as isize) + 1 as i32;
                        while ok != 0 && p <= *Pe.offset(j as isize) + ln - 1 as i32 {
                            if *W.offset(*Iw.offset(p as isize) as isize) != wflg {
                                ok = 0 as i32;
                            }
                            p += 1;
                            p;
                        }
                        if ok != 0 {
                            *Pe.offset(j as isize) = -i - 2 as i32;
                            *Nv.offset(i as isize) += *Nv.offset(j as isize);
                            *Nv.offset(j as isize) = 0 as i32;
                            *Elen.offset(j as isize) = -(1 as i32);
                            j = *Next.offset(j as isize);
                            *Next.offset(jlast as isize) = j;
                        } else {
                            jlast = j;
                            j = *Next.offset(j as isize);
                        }
                    }
                    wflg += 1;
                    wflg;
                    i = *Next.offset(i as isize);
                }
            }
            pme += 1;
            pme;
        }
        p = pme1;
        nleft = n - nel;
        pme = pme1;
        while pme <= pme2 {
            i = *Iw.offset(pme as isize);
            nvi = -*Nv.offset(i as isize);
            if nvi > 0 as i32 {
                *Nv.offset(i as isize) = nvi;
                deg = *Degree.offset(i as isize) + degme - nvi;
                deg = if deg < nleft - nvi { deg } else { nleft - nvi };
                inext = *Head.offset(deg as isize);
                if inext != -(1 as i32) {
                    *Last.offset(inext as isize) = i;
                }
                *Next.offset(i as isize) = inext;
                *Last.offset(i as isize) = -(1 as i32);
                *Head.offset(deg as isize) = i;
                mindeg = if mindeg < deg { mindeg } else { deg };
                *Degree.offset(i as isize) = deg;
                let fresh11 = p;
                p = p + 1;
                *Iw.offset(fresh11 as isize) = i;
            }
            pme += 1;
            pme;
        }
        *Nv.offset(me as isize) = nvpiv;
        *Len.offset(me as isize) = p - pme1;
        if *Len.offset(me as isize) == 0 as i32 {
            *Pe.offset(me as isize) = -(1 as i32);
            *W.offset(me as isize) = 0 as i32;
        }
        if elenme != 0 as i32 {
            pfree = p;
        }
        if !Info.is_null() {
            f = nvpiv as libc::c_double;
            r = (degme + ndense) as libc::c_double;
            dmax = if dmax > f + r { dmax } else { f + r };
            lnzme = f * r
                + (f - 1 as i32 as libc::c_double) * f / 2 as i32 as libc::c_double;
            lnz += lnzme;
            ndiv += lnzme;
            s = f * r * r + r * (f - 1 as i32 as libc::c_double) * f
                + (f - 1 as i32 as libc::c_double) * f
                    * (2 as i32 as libc::c_double * f - 1 as i32 as libc::c_double)
                    / 6 as i32 as libc::c_double;
            nms_lu += s;
            nms_ldl += (s + lnzme) / 2 as i32 as libc::c_double;
        }
    }
    if !Info.is_null() {
        f = ndense as libc::c_double;
        dmax = if dmax > ndense as libc::c_double {
            dmax
        } else {
            ndense as libc::c_double
        };
        lnzme = (f - 1 as i32 as libc::c_double) * f / 2 as i32 as libc::c_double;
        lnz += lnzme;
        ndiv += lnzme;
        s = (f - 1 as i32 as libc::c_double) * f
            * (2 as i32 as libc::c_double * f - 1 as i32 as libc::c_double)
            / 6 as i32 as libc::c_double;
        nms_lu += s;
        nms_ldl += (s + lnzme) / 2 as i32 as libc::c_double;
        *Info.offset(9 as i32 as isize) = lnz;
        *Info.offset(10 as i32 as isize) = ndiv;
        *Info.offset(11 as i32 as isize) = nms_ldl;
        *Info.offset(12 as i32 as isize) = nms_lu;
        *Info.offset(6 as i32 as isize) = ndense as libc::c_double;
        *Info.offset(13 as i32 as isize) = dmax;
        *Info.offset(8 as i32 as isize) = ncmpa as libc::c_double;
        *Info.offset(0 as i32 as isize) = 0 as i32 as libc::c_double;
    }
    i = 0 as i32;
    while i < n {
        *Pe.offset(i as isize) = -*Pe.offset(i as isize) - 2 as i32;
        i += 1;
        i;
    }
    i = 0 as i32;
    while i < n {
        *Elen.offset(i as isize) = -*Elen.offset(i as isize) - 2 as i32;
        i += 1;
        i;
    }
    i = 0 as i32;
    while i < n {
        if *Nv.offset(i as isize) == 0 as i32 {
            j = *Pe.offset(i as isize);
            if !(j == -(1 as i32)) {
                while *Nv.offset(j as isize) == 0 as i32 {
                    j = *Pe.offset(j as isize);
                }
                e = j;
                j = i;
                while *Nv.offset(j as isize) == 0 as i32 {
                    jnext = *Pe.offset(j as isize);
                    *Pe.offset(j as isize) = e;
                    j = jnext;
                }
            }
        }
        i += 1;
        i;
    }
    _glp_amd_postorder(n, Pe, Nv, Elen, W, Head, Next, Last);
    k = 0 as i32;
    while k < n {
        *Head.offset(k as isize) = -(1 as i32);
        *Next.offset(k as isize) = -(1 as i32);
        k += 1;
        k;
    }
    e = 0 as i32;
    while e < n {
        k = *W.offset(e as isize);
        if k != -(1 as i32) {
            *Head.offset(k as isize) = e;
        }
        e += 1;
        e;
    }
    nel = 0 as i32;
    k = 0 as i32;
    while k < n {
        e = *Head.offset(k as isize);
        if e == -(1 as i32) {
            break;
        }
        *Next.offset(e as isize) = nel;
        nel += *Nv.offset(e as isize);
        k += 1;
        k;
    }
    i = 0 as i32;
    while i < n {
        if *Nv.offset(i as isize) == 0 as i32 {
            e = *Pe.offset(i as isize);
            if e != -(1 as i32) {
                *Next.offset(i as isize) = *Next.offset(e as isize);
                let ref mut fresh12 = *Next.offset(e as isize);
                *fresh12 += 1;
                *fresh12;
            } else {
                let fresh13 = nel;
                nel = nel + 1;
                *Next.offset(i as isize) = fresh13;
            }
        }
        i += 1;
        i;
    }
    i = 0 as i32;
    while i < n {
        k = *Next.offset(i as isize);
        *Last.offset(k as isize) = i;
        i += 1;
        i;
    }
}