/* mc13d.rs */

// Original C code copyright and license information preserved
// This is a translation of the C code to Rust

pub fn mc13d(
    n: usize,
    icn: &[usize],
    ip: &[usize],
    lenr: &[usize],
    ior: &mut [usize],
    ib: &mut [usize],
    lowl: &mut [usize],
    numb: &mut [usize],
    prev: &mut [usize],
) -> usize {
    let arp = ior;
    let mut icnt = 0;
    let mut num = 0;
    let nnm1 = n + n - 1;

    // Initialization of arrays
    for j in 1..=n {
        numb[j] = 0;
        arp[j] = lenr[j] - 1;
    }

    for isn in 1..=n {
        if numb[isn] != 0 {
            continue;
        }
        let mut iv = isn;
        let mut ist = 1;
        lowl[iv] = 1;
        numb[iv] = 1;
        ib[n] = iv;

        for _ in 1..=nnm1 {
            let mut i1 = arp[iv];
            if i1 >= 0 {
                let i2 = ip[iv] + lenr[iv] - 1;
                i1 = i2 - i1;
                for ii in i1..=i2 {
                    let iw = icn[ii];
                    if numb[iw] == 0 {
                        // Put new node on the stack
                        arp[iv] = (i2 - ii - 1) as usize;
                        prev[iw] = iv;
                        iv = iw;
                        ist += 1;
                        lowl[iv] = ist;
                        numb[iv] = ist;
                        ib[n + 1 - ist] = iv;
                        continue;
                    }
                    // Update value of lowl[iv] if necessary
                    if lowl[iw] < lowl[iv] {
                        lowl[iv] = lowl[iw];
                    }
                }
                // No more edges leaving node iv
                arp[iv] = usize::MAX;
            }

            // Is node iv the root of a block?
            if lowl[iv] < numb[iv] {
                // Backtrack to previous node on path
                let iw = iv;
                iv = prev[iv];
                // Update value of lowl[iv] if necessary
                if lowl[iw] < lowl[iv] {
                    lowl[iv] = lowl[iw];
                }
                continue;
            }

            // Order nodes in a block
            num += 1;
            let ist1 = n + 1 - ist;
            let lcnt = icnt + 1;
            
            // Peel block off the top of the stack
            for stp in ist1..=n {
                let iw = ib[stp];
                lowl[iw] = n + 1;
                icnt += 1;
                numb[iw] = icnt;
                if iw == iv {
                    break;
                }
            }
            ist = n - stp;
            ib[num] = lcnt;

            // Are there any nodes left on the stack?
            if ist != 0 {
                // Backtrack to previous node on path
                let iw = iv;
                iv = prev[iv];
                if lowl[iw] < lowl[iv] {
                    lowl[iv] = lowl[iw];
                }
                continue;
            }

            // Have all the nodes been ordered?
            if icnt < n {
                break;
            }
            break;
        }
    }

    // Put permutation in the required form
    for i in 1..=n {
        arp[numb[i]] = i;
    }

    num
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};

    fn setup(n: usize, a: &[Vec<bool>], ip: &mut [usize], icn: &mut [usize], lenr: &mut [usize]) {
        let mut ind = 1;
        for i in 1..=n {
            ip[i] = ind;
            for j in 1..=n {
                if a[i][j] {
                    lenr[i] += 1;
                    icn[ind] = j;
                    ind += 1;
                }
            }
        }
    }

    fn test_mc13d(n: usize, ipp: usize) {
        let mut a = vec![vec![false; n + 1]; n + 1];
        let mut rng = thread_rng();

        // Initialize diagonal
        for i in 1..=n {
            a[i][i] = true;
        }

        // Add random off-diagonal elements
        for _ in 0..ipp {
            loop {
                let i = rng.gen_range(1..=n);
                let j = rng.gen_range(1..=n);
                if !a[i][j] {
                    a[i][j] = true;
                    break;
                }
            }
        }

        let mut ip = vec![0; n + 1];
        let mut icn = vec![0; 1000]; // Assuming max 1000 non-zeros
        let mut lenr = vec![0; n + 1];
        setup(n, &a, &mut ip, &mut icn, &mut lenr);

        let mut ior = vec![0; n + 1];
        let mut ib = vec![0; n + 2];
        let mut lowl = vec![0; n + 1];
        let mut numb = vec![0; n + 1];
        let mut prev = vec![0; n + 1];

        let num = mc13d(
            n,
            &icn,
            &ip,
            &lenr,
            &mut ior,
            &mut ib,
            &mut lowl,
            &mut numb,
            &mut prev,
        );

        println!("\nMatrix of order {} with {} off-diagonal non-zeros", n, ipp);
        println!("Found {} blocks", num);
    }

    #[test]
    fn test_cases() {
        let test_cases = vec![
            (1, 0),
            (2, 1),
            (2, 2),
            (3, 3),
            (4, 4),
            (5, 10),
            (10, 10),
            (10, 20),
            (20, 20),
            (20, 50),
            (50, 50),
            (50, 200),
        ];

        for (n, ipp) in test_cases {
            test_mc13d(n, ipp);
        }
    }
}