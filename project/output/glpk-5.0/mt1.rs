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
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
}
pub type integer = i32;
pub type real = libc::c_float;
unsafe extern "C" fn mt1_(
    mut n: *mut integer,
    mut p: *mut integer,
    mut w: *mut integer,
    mut c__: *mut integer,
    mut z__: *mut integer,
    mut x: *mut integer,
    mut jdim: *mut integer,
    mut jck: *mut integer,
    mut xx: *mut integer,
    mut min__: *mut integer,
    mut psign: *mut integer,
    mut wsign: *mut integer,
    mut zsign: *mut integer,
) -> i32 {
    let mut current_block: u64;
    let mut i__1: integer = 0;
    static mut a: real = 0.;
    static mut b: real = 0.;
    static mut j: integer = 0;
    static mut r__: integer = 0;
    static mut t: integer = 0;
    static mut j1: integer = 0;
    static mut n1: integer = 0;
    static mut ch: integer = 0;
    static mut ii: integer = 0;
    static mut jj: integer = 0;
    static mut kk: integer = 0;
    static mut in_0: integer = 0;
    static mut ll: integer = 0;
    static mut ip: integer = 0;
    static mut nn: integer = 0;
    static mut iu: integer = 0;
    static mut ii1: integer = 0;
    static mut chs: integer = 0;
    static mut lim: integer = 0;
    static mut lim1: integer = 0;
    static mut diff: integer = 0;
    static mut lold: integer = 0;
    static mut mink: integer = 0;
    static mut profit: integer = 0;
    zsign = zsign.offset(-1);
    zsign;
    wsign = wsign.offset(-1);
    wsign;
    psign = psign.offset(-1);
    psign;
    min__ = min__.offset(-1);
    min__;
    xx = xx.offset(-1);
    xx;
    x = x.offset(-1);
    x;
    w = w.offset(-1);
    w;
    p = p.offset(-1);
    p;
    *z__ = 0 as i32;
    if *jck == 1 as i32 {
        chmt1_(
            n,
            &mut *p.offset(1 as i32 as isize),
            &mut *w.offset(1 as i32 as isize),
            c__,
            z__,
            jdim,
        );
    }
    if *z__ < 0 as i32 {
        return 0 as i32;
    }
    ch = *c__;
    ip = 0 as i32;
    chs = ch;
    i__1 = *n;
    ll = 1 as i32;
    while ll <= i__1 {
        if *w.offset(ll as isize) > chs {
            break;
        }
        ip += *p.offset(ll as isize);
        chs -= *w.offset(ll as isize);
        ll += 1;
        ll;
    }
    ll -= 1;
    ll;
    if chs == 0 as i32 {
        *z__ = ip;
        i__1 = ll;
        j = 1 as i32;
        while j <= i__1 {
            *x.offset(j as isize) = 1 as i32;
            j += 1;
            j;
        }
        nn = ll + 1 as i32;
        i__1 = *n;
        j = nn;
        while j <= i__1 {
            *x.offset(j as isize) = 0 as i32;
            j += 1;
            j;
        }
        return 0 as i32;
    } else {
        *p.offset((*n + 1 as i32) as isize) = 0 as i32;
        *w.offset((*n + 1 as i32) as isize) = ch + 1 as i32;
        lim = ip
            + chs * *p.offset((ll + 2 as i32) as isize)
                / *w.offset((ll + 2 as i32) as isize);
        a = (*w.offset((ll + 1 as i32) as isize) - chs) as real;
        b = (ip + *p.offset((ll + 1 as i32) as isize)) as real;
        lim1 = (b - a * *p.offset(ll as isize) as real / *w.offset(ll as isize) as real)
            as integer;
        if lim1 > lim {
            lim = lim1;
        }
        mink = ch + 1 as i32;
        *min__.offset(*n as isize) = mink;
        i__1 = *n;
        j = 2 as i32;
        while j <= i__1 {
            kk = *n + 2 as i32 - j;
            if *w.offset(kk as isize) < mink {
                mink = *w.offset(kk as isize);
            }
            *min__.offset((kk - 1 as i32) as isize) = mink;
            j += 1;
            j;
        }
        i__1 = *n;
        j = 1 as i32;
        while j <= i__1 {
            *xx.offset(j as isize) = 0 as i32;
            j += 1;
            j;
        }
        *z__ = 0 as i32;
        profit = 0 as i32;
        lold = *n;
        ii = 1 as i32;
        loop {
            *wsign.offset(ii as isize) = ch - chs;
            *psign.offset(ii as isize) = ip;
            *zsign.offset(ii as isize) = ll + 1 as i32;
            *xx.offset(ii as isize) = 1 as i32;
            nn = ll - 1 as i32;
            if !(nn < ii) {
                i__1 = nn;
                j = ii;
                while j <= i__1 {
                    *wsign.offset((j + 1 as i32) as isize) = *wsign.offset(j as isize)
                        - *w.offset(j as isize);
                    *psign.offset((j + 1 as i32) as isize) = *psign.offset(j as isize)
                        - *p.offset(j as isize);
                    *zsign.offset((j + 1 as i32) as isize) = ll + 1 as i32;
                    *xx.offset((j + 1 as i32) as isize) = 1 as i32;
                    j += 1;
                    j;
                }
            }
            j1 = ll + 1 as i32;
            i__1 = lold;
            j = j1;
            while j <= i__1 {
                *wsign.offset(j as isize) = 0 as i32;
                *psign.offset(j as isize) = 0 as i32;
                *zsign.offset(j as isize) = j;
                j += 1;
                j;
            }
            lold = ll;
            ch = chs;
            profit += ip;
            i__1 = ll - (*n - 2 as i32);
            if i__1 < 0 as i32 {
                ii = ll + 2 as i32;
                if ch >= *min__.offset((ii - 1 as i32) as isize) {
                    current_block = 13515666132482594926;
                } else {
                    current_block = 11520893786184715627;
                }
            } else {
                if i__1 == 0 as i32 {
                    if !(ch < *w.offset(*n as isize)) {
                        ch -= *w.offset(*n as isize);
                        profit += *p.offset(*n as isize);
                        *xx.offset(*n as isize) = 1 as i32;
                    }
                    ii = *n - 1 as i32;
                } else {
                    ii = *n;
                }
                current_block = 11520893786184715627;
            }
            match current_block {
                11520893786184715627 => {
                    if !(*z__ >= profit) {
                        *z__ = profit;
                        i__1 = *n;
                        j = 1 as i32;
                        while j <= i__1 {
                            *x.offset(j as isize) = *xx.offset(j as isize);
                            j += 1;
                            j;
                        }
                        if *z__ == lim {
                            return 0 as i32;
                        }
                    }
                    if *xx.offset(*n as isize) == 0 as i32 {
                        current_block = 4970656539799580057;
                    } else {
                        *xx.offset(*n as isize) = 0 as i32;
                        ch += *w.offset(*n as isize);
                        profit -= *p.offset(*n as isize);
                        current_block = 4970656539799580057;
                    }
                }
                _ => {}
            }
            '_L80: loop {
                match current_block {
                    13515666132482594926 => {
                        if *w.offset(ii as isize) <= ch {
                            ip = *psign.offset(ii as isize);
                            chs = ch - *wsign.offset(ii as isize);
                            in_0 = *zsign.offset(ii as isize);
                            i__1 = *n;
                            ll = in_0;
                            loop {
                                if !(ll <= i__1) {
                                    current_block = 1874315696050160458;
                                    break;
                                }
                                if *w.offset(ll as isize) > chs {
                                    current_block = 14189294902102472859;
                                    break;
                                }
                                ip += *p.offset(ll as isize);
                                chs -= *w.offset(ll as isize);
                                ll += 1;
                                ll;
                            }
                            match current_block {
                                1874315696050160458 => {
                                    ll = *n;
                                }
                                _ => {
                                    iu = chs * *p.offset(ll as isize) / *w.offset(ll as isize);
                                    ll -= 1;
                                    ll;
                                    if !(iu == 0 as i32) {
                                        if *z__ >= profit + ip + iu {
                                            current_block = 4970656539799580057;
                                            continue;
                                        } else {
                                            break;
                                        }
                                    }
                                }
                            }
                            if *z__ >= ip + profit {
                                current_block = 4970656539799580057;
                                continue;
                            }
                            *z__ = ip + profit;
                            nn = ii - 1 as i32;
                            i__1 = nn;
                            j = 1 as i32;
                            while j <= i__1 {
                                *x.offset(j as isize) = *xx.offset(j as isize);
                                j += 1;
                                j;
                            }
                            i__1 = ll;
                            j = ii;
                            while j <= i__1 {
                                *x.offset(j as isize) = 1 as i32;
                                j += 1;
                                j;
                            }
                            if !(ll == *n) {
                                nn = ll + 1 as i32;
                                i__1 = *n;
                                j = nn;
                                while j <= i__1 {
                                    *x.offset(j as isize) = 0 as i32;
                                    j += 1;
                                    j;
                                }
                            }
                            if *z__ != lim {
                                current_block = 4970656539799580057;
                                continue;
                            }
                            return 0 as i32;
                        } else {
                            ii1 = ii + 1 as i32;
                            if *z__
                                >= ch * *p.offset(ii1 as isize) / *w.offset(ii1 as isize)
                                    + profit
                            {
                                current_block = 4970656539799580057;
                                continue;
                            }
                            ii = ii1;
                            current_block = 13515666132482594926;
                        }
                    }
                    _ => {
                        nn = ii - 1 as i32;
                        if nn == 0 as i32 {
                            return 0 as i32;
                        }
                        i__1 = nn;
                        j = 1 as i32;
                        loop {
                            if !(j <= i__1) {
                                current_block = 11957990509374275265;
                                break;
                            }
                            kk = ii - j;
                            if *xx.offset(kk as isize) == 1 as i32 {
                                current_block = 15776489420005083232;
                                break;
                            }
                            j += 1;
                            j;
                        }
                        match current_block {
                            11957990509374275265 => return 0 as i32,
                            _ => {
                                r__ = ch;
                                ch += *w.offset(kk as isize);
                                profit -= *p.offset(kk as isize);
                                *xx.offset(kk as isize) = 0 as i32;
                                if r__ < *min__.offset(kk as isize) {
                                    nn = kk + 1 as i32;
                                    ii = kk;
                                    loop {
                                        if *z__
                                            >= profit
                                                + ch * *p.offset(nn as isize) / *w.offset(nn as isize)
                                        {
                                            current_block = 4970656539799580057;
                                            continue '_L80;
                                        }
                                        diff = *w.offset(nn as isize) - *w.offset(kk as isize);
                                        if diff < 0 as i32 {
                                            t = r__ - diff;
                                            if !(t < *min__.offset(nn as isize)) {
                                                break;
                                            }
                                        } else if !(diff == 0 as i32) {
                                            if !(diff > r__) {
                                                if !(*z__ >= profit + *p.offset(nn as isize)) {
                                                    *z__ = profit + *p.offset(nn as isize);
                                                    i__1 = kk;
                                                    j = 1 as i32;
                                                    while j <= i__1 {
                                                        *x.offset(j as isize) = *xx.offset(j as isize);
                                                        j += 1;
                                                        j;
                                                    }
                                                    jj = kk + 1 as i32;
                                                    i__1 = *n;
                                                    j = jj;
                                                    while j <= i__1 {
                                                        *x.offset(j as isize) = 0 as i32;
                                                        j += 1;
                                                        j;
                                                    }
                                                    *x.offset(nn as isize) = 1 as i32;
                                                    if *z__ == lim {
                                                        return 0 as i32;
                                                    }
                                                    r__ -= diff;
                                                    kk = nn;
                                                    nn += 1;
                                                    nn;
                                                    continue;
                                                }
                                            }
                                        }
                                        nn += 1;
                                        nn;
                                    }
                                    if *z__
                                        >= profit + *p.offset(nn as isize)
                                            + t * *p.offset((nn + 1 as i32) as isize)
                                                / *w.offset((nn + 1 as i32) as isize)
                                    {
                                        current_block = 4970656539799580057;
                                        continue;
                                    }
                                    ch -= *w.offset(nn as isize);
                                    profit += *p.offset(nn as isize);
                                    *xx.offset(nn as isize) = 1 as i32;
                                    ii = nn + 1 as i32;
                                    *wsign.offset(nn as isize) = *w.offset(nn as isize);
                                    *psign.offset(nn as isize) = *p.offset(nn as isize);
                                    *zsign.offset(nn as isize) = ii;
                                    n1 = nn + 1 as i32;
                                    i__1 = lold;
                                    j = n1;
                                    while j <= i__1 {
                                        *wsign.offset(j as isize) = 0 as i32;
                                        *psign.offset(j as isize) = 0 as i32;
                                        *zsign.offset(j as isize) = j;
                                        j += 1;
                                        j;
                                    }
                                    lold = nn;
                                    current_block = 13515666132482594926;
                                } else {
                                    ii = kk + 1 as i32;
                                    current_block = 13515666132482594926;
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn chmt1_(
    mut n: *mut integer,
    mut p: *mut integer,
    mut w: *mut integer,
    mut c__: *mut integer,
    mut z__: *mut integer,
    mut jdim: *mut integer,
) -> i32 {
    let mut current_block: u64;
    let mut i__1: integer = 0;
    static mut j: integer = 0;
    static mut r__: real = 0.;
    static mut rr: real = 0.;
    static mut jsw: integer = 0;
    w = w.offset(-1);
    w;
    p = p.offset(-1);
    p;
    if *n >= 2 as i32 && *n <= *jdim - 1 as i32 {
        if *c__ > 0 as i32 {
            jsw = 0 as i32;
            rr = *p.offset(1 as i32 as isize) as real
                / *w.offset(1 as i32 as isize) as real;
            i__1 = *n;
            j = 1 as i32;
            loop {
                if !(j <= i__1) {
                    current_block = 17478428563724192186;
                    break;
                }
                r__ = rr;
                if *p.offset(j as isize) <= 0 as i32 {
                    current_block = 3199781220463482640;
                    break;
                }
                if *w.offset(j as isize) <= 0 as i32 {
                    current_block = 3199781220463482640;
                    break;
                }
                jsw += *w.offset(j as isize);
                if *w.offset(j as isize) <= *c__ {
                    rr = *p.offset(j as isize) as real / *w.offset(j as isize) as real;
                    if rr <= r__ {
                        j += 1;
                        j;
                    } else {
                        *z__ = -(5 as i32);
                        return 0 as i32;
                    }
                } else {
                    *z__ = -(3 as i32);
                    return 0 as i32;
                }
            }
            match current_block {
                3199781220463482640 => {}
                _ => {
                    if jsw > *c__ {
                        return 0 as i32;
                    }
                    *z__ = -(4 as i32);
                    return 0 as i32;
                }
            }
        }
        *z__ = -(2 as i32);
        return 0 as i32;
    } else {
        *z__ = -(1 as i32);
        return 0 as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mt1(
    mut n: i32,
    mut p: *mut i32,
    mut w: *mut i32,
    mut c: i32,
    mut x: *mut i32,
    mut jck: i32,
    mut xx: *mut i32,
    mut min: *mut i32,
    mut psign: *mut i32,
    mut wsign: *mut i32,
    mut zsign: *mut i32,
) -> i32 {
    let mut z: i32 = 0;
    let mut jdim: i32 = n + 1 as i32;
    let mut j: i32 = 0;
    let mut s1: i32 = 0;
    let mut s2: i32 = 0;
    mt1_(
        &mut n,
        &mut *p.offset(1 as i32 as isize),
        &mut *w.offset(1 as i32 as isize),
        &mut c,
        &mut z,
        &mut *x.offset(1 as i32 as isize),
        &mut jdim,
        &mut jck,
        &mut *xx.offset(1 as i32 as isize),
        &mut *min.offset(1 as i32 as isize),
        &mut *psign.offset(1 as i32 as isize),
        &mut *wsign.offset(1 as i32 as isize),
        &mut *zsign.offset(1 as i32 as isize),
    );
    s2 = 0 as i32;
    s1 = s2;
    j = 1 as i32;
    while j <= n {
        (*x.offset(j as isize) == 0 as i32 || *x.offset(j as isize) == 1 as i32
            || {
                glp_assert_(
                    b"x[j] == 0 || x[j] == 1\0" as *const u8 as *const i8,
                    b"\0" as *const u8 as *const i8,
                    290 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if *x.offset(j as isize) != 0 {
            s1 += *p.offset(j as isize);
            s2 += *w.offset(j as isize);
        }
        j += 1;
        j;
    }
    (s1 == z
        || {
            glp_assert_(
                b"s1 == z\0" as *const u8 as *const i8,
                b"\0" as *const u8 as *const i8,
                294 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (s2 <= c
        || {
            glp_assert_(
                b"s2 <= c\0" as *const u8 as *const i8,
                b"\0" as *const u8 as *const i8,
                295 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return z;
}