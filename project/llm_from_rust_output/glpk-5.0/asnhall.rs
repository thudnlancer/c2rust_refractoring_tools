use std::ptr;

struct GlpGraph {
    pool: *mut libc::c_void,
    name: *mut libc::c_char,
    nv_max: libc::c_int,
    nv: libc::c_int,
    na: libc::c_int,
    v: *mut *mut GlpVertex,
    index: *mut libc::c_void,
    v_size: libc::c_int,
    a_size: libc::c_int,
}

struct GlpVertex {
    i: libc::c_int,
    name: *mut libc::c_char,
    entry: *mut libc::c_void,
    data: *mut libc::c_void,
    temp: *mut libc::c_void,
    in_: *mut GlpArc,
    out: *mut GlpArc,
}

struct GlpArc {
    tail: *mut GlpVertex,
    head: *mut GlpVertex,
    data: *mut libc::c_void,
    temp: *mut libc::c_void,
    t_prev: *mut GlpArc,
    t_next: *mut GlpArc,
    h_prev: *mut GlpArc,
    h_next: *mut GlpArc,
}

fn glp_asnprob_hall(
    G: *mut GlpGraph,
    v_set: libc::c_int,
    a_x: libc::c_int,
) -> libc::c_int {
    unsafe {
        if v_set >= 0 && v_set > (*G).v_size - std::mem::size_of::<libc::c_int>() as libc::c_int {
            panic!("glp_asnprob_hall: v_set = {}; invalid offset", v_set);
        }
        if a_x >= 0 && a_x > (*G).a_size - std::mem::size_of::<libc::c_int>() as libc::c_int {
            panic!("glp_asnprob_hall: a_x = {}; invalid offset", a_x);
        }

        if glp_check_asnprob(G, v_set) != 0 {
            return -1;
        }

        let nv = (*G).nv;
        let mut num = vec![0; (nv + 1) as usize];
        let mut n1 = 0;
        let mut n2 = 0;

        for i in 1..=nv {
            let v = *(*G).v.offset(i as isize);
            if (*v).in_.is_null() && !(*v).out.is_null() {
                n1 += 1;
                num[i as usize] = 0;
            } else if !(*v).in_.is_null() && (*v).out.is_null() {
                n2 += 1;
                num[i as usize] = n2;
            } else {
                assert!((*v).in_.is_null() && (*v).out.is_null());
                num[i as usize] = -1;
            }
        }

        let n = std::cmp::max(n1, n2);
        let mut icn = vec![0; (*G).na as usize + 1];
        let mut ip = vec![0; n as usize + 1];
        let mut lenr = vec![0; n as usize + 1];
        let mut iperm = vec![0; n as usize + 1];
        let mut pr = vec![0; n as usize + 1];
        let mut arp = vec![0; n as usize + 1];
        let mut cv = vec![0; n as usize + 1];
        let mut out = vec![0; n as usize + 1];

        let mut k = 0;
        let mut loc = 1;

        for i in 1..=nv {
            if num[i as usize] == 0 {
                k += 1;
                ip[k as usize] = loc;
                let v = *(*G).v.offset(i as isize);
                let mut a = (*v).out;
                while !a.is_null() {
                    assert!(num[(*(*a).head).i as usize] != 0);
                    icn[loc as usize] = num[(*(*a).head).i as usize];
                    loc += 1;
                    a = (*a).t_next;
                }
                lenr[k as usize] = loc - ip[k as usize];
            }
        }

        assert_eq!(loc - 1, (*G).na);

        k += 1;
        while k <= n {
            ip[k as usize] = loc;
            lenr[k as usize] = 0;
            k += 1;
        }

        let card = _glp_mc21a(
            n,
            icn.as_ptr(),
            ip.as_ptr(),
            lenr.as_ptr(),
            iperm.as_mut_ptr(),
            pr.as_mut_ptr(),
            arp.as_mut_ptr(),
            cv.as_mut_ptr(),
            out.as_mut_ptr(),
        );

        arp.iter_mut().for_each(|x| *x = 0);

        for i in 1..=card {
            let k = iperm[i as usize];
            assert!(1 <= k && k <= n);
            assert_eq!(arp[k as usize], 0);
            arp[k as usize] = i;
        }

        if a_x >= 0 {
            let mut k = 0;
            for i in 1..=nv {
                if num[i as usize] == 0 {
                    k += 1;
                    let v = *(*G).v.offset(i as isize);
                    let mut a = (*v).out;
                    while !a.is_null() {
                        let xij = if arp[k as usize] == num[(*(*a).head).i as usize] {
                            assert_ne!(arp[k as usize], 0);
                            1
                        } else {
                            0
                        };
                        ptr::copy_nonoverlapping(
                            &xij as *const libc::c_int,
                            ((*a).data as *mut libc::c_char).offset(a_x as isize) as *mut libc::c_void,
                            std::mem::size_of::<libc::c_int>(),
                        );
                        a = (*a).t_next;
                    }
                }
            }
        }

        card
    }
}

extern "C" {
    fn glp_check_asnprob(G: *mut GlpGraph, v_set: libc::c_int) -> libc::c_int;
    fn _glp_mc21a(
        n: libc::c_int,
        icn: *const libc::c_int,
        ip: *const libc::c_int,
        lenr: *const libc::c_int,
        iperm: *mut libc::c_int,
        pr: *mut libc::c_int,
        arp: *mut libc::c_int,
        cv: *mut libc::c_int,
        out: *mut libc::c_int,
    ) -> libc::c_int;
}