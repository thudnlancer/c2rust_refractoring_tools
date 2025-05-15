pub fn glp_mc21a(
    n: i32,
    icn: &[i32],
    ip: &[i32],
    lenr: &[i32],
    iperm: &mut [i32],
    pr: &mut [i32],
    arp: &mut [i32],
    cv: &mut [i32],
    out: &mut [i32],
) -> i32 {
    for i in 1..=n {
        arp[i as usize] = lenr[i as usize] - 1;
        iperm[i as usize] = 0;
        cv[i as usize] = iperm[i as usize];
    }

    let mut numnz = 0;
    let mut jord = 1;

    while jord <= n {
        let mut j = jord;
        pr[j as usize] = -1;
        let mut k = 1;

        let (mut i, mut in2, mut ii) = (0, 0, 0);
        let mut found = false;

        'outer: while k <= jord {
            let mut in1 = arp[j as usize];
            if in1 >= 0 {
                in2 = ip[j as usize] + lenr[j as usize] - 1;
                in1 = in2 - in1;
                ii = in1;

                while ii <= in2 {
                    i = icn[ii as usize];
                    if iperm[i as usize] == 0 {
                        found = true;
                        break 'outer;
                    }
                    ii += 1;
                }
                arp[j as usize] = -1;
            }

            out[j as usize] = lenr[j as usize] - 1;
            let mut kk = 1;

            while kk <= jord {
                in1 = out[j as usize];
                if in1 >= 0 {
                    in2 = ip[j as usize] + lenr[j as usize] - 1;
                    in1 = in2 - in1;
                    ii = in1;

                    while ii <= in2 {
                        i = icn[ii as usize];
                        if cv[i as usize] != jord {
                            let j1 = j;
                            j = iperm[i as usize];
                            cv[i as usize] = jord;
                            pr[j as usize] = j1;
                            out[j1 as usize] = in2 - ii - 1;
                            break;
                        }
                        ii += 1;
                    }
                }

                j = pr[j as usize];
                if j == -1 {
                    break 'outer;
                }
                kk += 1;
            }
            k += 1;
        }

        if found {
            iperm[i as usize] = j;
            arp[j as usize] = in2 - ii - 1;
            numnz += 1;

            let mut k = 1;
            let mut j = iperm[i as usize];
            while k <= jord {
                j = pr[j as usize];
                if j == -1 {
                    break;
                }
                ii = ip[j as usize] + lenr[j as usize] - out[j as usize] - 2;
                i = icn[ii as usize];
                iperm[i as usize] = j;
                k += 1;
            }
        }

        jord += 1;
    }

    if numnz < n {
        for i in 1..=n {
            arp[i as usize] = 0;
        }

        let mut k = 0;
        for i in 1..=n {
            if iperm[i as usize] == 0 {
                k += 1;
                out[k as usize] = i;
            } else {
                arp[iperm[i as usize] as usize] = i;
            }
        }

        let mut k = 0;
        for i in 1..=n {
            if arp[i as usize] == 0 {
                k += 1;
                iperm[out[k as usize] as usize] = i;
            }
        }
    }

    numnz
}