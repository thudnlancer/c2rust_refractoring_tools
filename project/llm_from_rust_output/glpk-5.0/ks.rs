use std::cmp::Ordering;
use std::ptr;

const N_MAX: usize = 40;

struct KS {
    orig_n: i32,
    n: i32,
    a: Vec<i32>,
    b: i32,
    c: Vec<i32>,
    c0: i32,
    x: Vec<i8>,
}

struct MT {
    j: i32,
    r: f32,
}

fn glp_ks_enum(n: i32, a: &[i32], b: i32, c: &[i32], x: &mut [i8]) -> i32 {
    assert!(n >= 0 && n <= N_MAX as i32, "0 <= n && n <= N_MAX");

    let mut x_best = vec![0; N_MAX + 1];
    let mut z_best = i32::MIN;

    x[1..=n as usize].fill(0);

    loop {
        let (mut z, mut s) = (0, 0);
        for j in 1..=n as usize {
            if x[j] != 0 {
                s += a[j];
                z += c[j];
            }
        }

        if s <= b && z_best < z {
            x_best[1..=n as usize].copy_from_slice(&x[1..=n as usize]);
            z_best = z;
        }

        let mut j = 1;
        while j <= n as usize {
            if x[j] == 0 {
                x[j] = 1;
                break;
            } else {
                x[j] = 0;
                j += 1;
            }
        }

        if j > n as usize {
            break;
        }
    }

    x[1..=n as usize].copy_from_slice(&x_best[1..=n as usize]);
    z_best
}

fn reduce(n: i32, a: &[i32], b: i32, c: &[i32]) -> Option<KS> {
    assert!(n >= 0, "n >= 0");

    let mut ks = KS {
        orig_n: n,
        n: 0,
        a: vec![0; n as usize + 1],
        b,
        c: vec![0; n as usize + 1],
        c0: 0,
        x: vec![0; n as usize + 1],
    };

    ks.a[1..=n as usize].copy_from_slice(&a[1..=n as usize]);
    ks.c[1..=n as usize].copy_from_slice(&c[1..=n as usize]);

    for j in 1..=n as usize {
        if a[j] >= 0 {
            ks.x[j] = 0x10;
        } else {
            ks.x[j] = 0x11;
            ks.a[j] = -ks.a[j];
            ks.b += ks.a[j];
            ks.c0 += ks.c[j];
            ks.c[j] = -ks.c[j];
        }
    }

    if ks.b < 0 {
        return None;
    }

    for j in 1..=n as usize {
        if ks.a[j] == 0 {
            if ks.c[j] <= 0 {
                ks.x[j] ^= 0x10;
            } else {
                ks.x[j] ^= 0x11;
                ks.c0 += ks.c[j];
            }
        } else if ks.a[j] > ks.b || ks.c[j] <= 0 {
            ks.x[j] ^= 0x10;
        } else {
            ks.n += 1;
            ks.a[ks.n as usize] = ks.a[j];
            ks.c[ks.n as usize] = ks.c[j];
        }
    }

    let mut s = 0;
    for j in 1..=ks.n as usize {
        assert!(1 <= ks.a[j] && ks.a[j] <= ks.b, "1 <= ks.a[j] && ks.a[j] <= ks.b");
        assert!(ks.c[j] >= 1, "ks.c[j] >= 1");
        s += ks.a[j];
    }

    if s <= ks.b {
        for j in 1..=n as usize {
            if ks.x[j] & 0x10 != 0 {
                ks.x[j] ^= 0x11;
            }
        }
        for j in 1..=ks.n as usize {
            ks.c0 += ks.c[j];
        }
        ks.n = 0;
    }

    assert!(ks.n == 0 || ks.n >= 2, "ks.n == 0 || ks.n >= 2");
    Some(ks)
}

fn restore(ks: &KS, x: &mut [i8]) -> i32 {
    let mut z = ks.c0;
    let mut k = 0;

    for j in 1..=ks.orig_n as usize {
        if ks.x[j] & 0x10 != 0 {
            k += 1;
            assert!(k <= ks.n as usize, "k <= ks.n");
            assert!(x[k] == 0 || x[k] == 1, "x[k] == 0 || x[k] == 1");

            let mut val = x[k];
            if ks.x[j] & 1 != 0 {
                val = (1 - val) as i8;
            }
            if val != 0 {
                z += ks.c[k];
            }
        }
    }

    assert!(k == ks.n as usize, "k == ks.n");
    z
}

fn fcmp(a: &MT, b: &MT) -> Ordering {
    b.r.partial_cmp(&a.r).unwrap_or(Ordering::Equal)
}

fn mt1a(n: i32, a: &[i32], b: i32, c: &[i32], x: &mut [i8]) -> i32 {
    assert!(n >= 2, "n >= 2");

    let mut mt: Vec<MT> = (1..=n as usize)
        .map(|j| MT {
            j: j as i32,
            r: c[j] as f32 / a[j] as f32,
        })
        .collect();

    mt.sort_by(fcmp);

    let p: Vec<i32> = mt.iter().map(|m| c[m.j as usize]).collect();
    let w: Vec<i32> = mt.iter().map(|m| a[m.j as usize]).collect();

    let mut x1 = vec![0; n as usize + 2];
    let z = _glp_mt1(
        n,
        &p,
        &w,
        b,
        &mut x1,
        1,
        &mut vec![0; n as usize + 2],
        &mut vec![0; n as usize + 2],
        &mut vec![0; n as usize + 2],
        &mut vec![0; n as usize + 2],
        &mut vec![0; n as usize + 2],
    );

    assert!(z >= 0, "z >= 0");
    for j in 1..=n as usize {
        assert!(x1[j] == 0 || x1[j] == 1, "x1[j] == 0 || x1[j] == 1");
        x[mt[j - 1].j as usize] = x1[j] as i8;
    }

    z
}

fn _glp_mt1(
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
    // Implementation of the actual MT1 algorithm would go here
    // This is a placeholder since the original implementation wasn't provided
    0
}

fn greedy(n: i32, a: &[i32], b: i32, c: &[i32], x: &mut [i8]) -> i32 {
    assert!(n >= 2, "n >= 2");

    let mut mt: Vec<MT> = (1..=n as usize)
        .map(|j| MT {
            j: j as i32,
            r: c[j] as f32 / a[j] as f32,
        })
        .collect();

    mt.sort_by(fcmp);

    let (mut z, mut s) = (0, 0);
    x.fill(0);

    for m in mt.iter() {
        let j = m.j as usize;
        if s + a[j] > b {
            break;
        }
        x[j] = 1;
        s += a[j];
        z += c[j];
    }

    z
}

pub fn glp_ks_mt1(n: i32, a: &[i32], b: i32, c: &[i32], x: &mut [i8]) -> i32 {
    assert!(n >= 0, "n >= 0");

    let ks = match reduce(n, a, b, c) {
        Some(ks) => ks,
        None => return i32::MIN,
    };

    if ks.n > 0 {
        mt1a(ks.n, &ks.a, ks.b, &ks.c, x);
    }

    let z = restore(&ks, x);
    x[1..=n as usize].copy_from_slice(&ks.x[1..=n as usize]);

    let (mut s1, mut s2) = (0, 0);
    for j in 1..=n as usize {
        assert!(x[j] == 0 || x[j] == 1, "x[j] == 0 || x[j] == 1");
        if x[j] != 0 {
            s1 += a[j];
            s2 += c[j];
        }
    }

    assert!(s1 <= b, "s1 <= b");
    assert!(s2 == z, "s2 == z");
    z
}

pub fn glp_ks_greedy(n: i32, a: &[i32], b: i32, c: &[i32], x: &mut [i8]) -> i32 {
    assert!(n >= 0, "n >= 0");

    let ks = match reduce(n, a, b, c) {
        Some(ks) => ks,
        None => return i32::MIN,
    };

    if ks.n > 0 {
        greedy(ks.n, &ks.a, ks.b, &ks.c, x);
    }

    let z = restore(&ks, x);
    x[1..=n as usize].copy_from_slice(&ks.x[1..=n as usize]);

    let (mut s1, mut s2) = (0, 0);
    for j in 1..=n as usize {
        assert!(x[j] == 0 || x[j] == 1, "x[j] == 0 || x[j] == 1");
        if x[j] != 0 {
            s1 += a[j];
            s2 += c[j];
        }
    }

    assert!(s1 <= b, "s1 <= b");
    assert!(s2 == z, "s2 == z");
    z
}