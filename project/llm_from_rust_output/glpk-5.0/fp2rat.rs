use std::f64;

pub fn glp_fp2rat(x: f64, eps: f64) -> Option<(f64, f64, i32)> {
    assert!(0.0 <= x && x < 1.0, "x must be in range [0.0, 1.0)");

    let mut k = 0;
    let mut xk = x;
    let mut akm1 = 1.0;
    let mut ak = 0.0;
    let mut bkm1 = 0.0;
    let mut bk = 1.0;

    for _ in 0..100 {
        if k == 0 {
            xk = x;
            akm1 = 1.0;
            ak = 0.0;
            bkm1 = 0.0;
            bk = 1.0;
        } else {
            let temp = xk - xk.floor();
            assert!(temp != 0.0, "temp must not be zero");
            xk = 1.0 / temp;
            let ak_term = 1.0;
            let bk_term = xk.floor();
            
            let new_ak = bk_term * ak + ak_term * akm1;
            akm1 = ak;
            ak = new_ak;
            
            let new_bk = bk_term * bk + ak_term * bkm1;
            bkm1 = bk;
            bk = new_bk;
        }

        let fk = ak / bk;
        if (x - fk).abs() <= eps {
            return Some((ak, bk, k));
        }
        k += 1;
    }

    None
}