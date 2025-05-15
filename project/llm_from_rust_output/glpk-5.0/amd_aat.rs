use std::ptr;

pub type SizeT = usize;

#[derive(Debug, Default)]
pub struct AmdInfo {
    pub n: f64,
    pub nz: f64,
    pub symmetry: f64,
    pub nzdiag: f64,
    pub nzaat: f64,
}

pub fn glp_amd_aat(n: i32, ap: &[i32], ai: &[i32]) -> (SizeT, Option<AmdInfo>) {
    let mut len = vec![0; n as usize];
    let mut tp = vec![0; n as usize];
    let mut nzdiag = 0;
    let mut nzboth = 0;
    let nz = ap[n as usize];

    for k in 0..n as usize {
        let p1 = ap[k];
        let p2 = ap[k + 1];
        let mut p = p1;

        while p < p2 {
            let j = ai[p as usize];
            if j < k as i32 {
                len[j as usize] += 1;
                len[k] += 1;
                p += 1;

                let pj2 = ap[(j + 1) as usize];
                let mut pj = tp[j as usize];
                while pj < pj2 {
                    let i = ai[pj as usize];
                    if i < k as i32 {
                        len[i as usize] += 1;
                        len[j as usize] += 1;
                        pj += 1;
                    } else if i == k as i32 {
                        pj += 1;
                        nzboth += 1;
                        break;
                    } else {
                        break;
                    }
                }
                tp[j as usize] = pj;
            } else if j == k as i32 {
                p += 1;
                nzdiag += 1;
                break;
            } else {
                break;
            }
        }
        tp[k] = p;
    }

    for j in 0..n as usize {
        let mut pj = tp[j];
        while pj < ap[j + 1] {
            let i = ai[pj as usize];
            len[i] += 1;
            len[j] += 1;
            pj += 1;
        }
    }

    let symmetry = if nz == nzdiag {
        1.0
    } else {
        2.0 * nzboth as f64 / (nz - nzdiag) as f64
    };

    let nzaat = len.iter().sum::<i32>() as SizeT;

    let info = AmdInfo {
        n: n as f64,
        nz: nz as f64,
        symmetry,
        nzdiag: nzdiag as f64,
        nzaat: nzaat as f64,
    };

    (nzaat, Some(info))
}