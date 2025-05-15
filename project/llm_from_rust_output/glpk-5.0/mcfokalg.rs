use std::f64;
use std::mem;
use std::ptr;

#[derive(Debug, Clone, Copy)]
pub struct GlpGraph {
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

#[derive(Debug, Clone, Copy)]
pub struct GlpVertex {
    i: libc::c_int,
    name: *mut libc::c_char,
    entry: *mut libc::c_void,
    data: *mut libc::c_void,
    temp: *mut libc::c_void,
    in_: *mut GlpArc,
    out: *mut GlpArc,
}

#[derive(Debug, Clone, Copy)]
pub struct GlpArc {
    tail: *mut GlpVertex,
    head: *mut GlpVertex,
    data: *mut libc::c_void,
    temp: *mut libc::c_void,
    t_prev: *mut GlpArc,
    t_next: *mut GlpArc,
    h_prev: *mut GlpArc,
    h_next: *mut GlpArc,
}

pub fn glp_mincost_okalg(
    g: &mut GlpGraph,
    v_rhs: libc::c_int,
    a_low: libc::c_int,
    a_cap: libc::c_int,
    a_cost: libc::c_int,
    sol: Option<&mut f64>,
    a_x: libc::c_int,
    v_pi: libc::c_int,
) -> libc::c_int {
    // Validate offsets
    validate_offset(v_rhs, g.v_size, "v_rhs").unwrap();
    validate_offset(a_low, g.a_size, "a_low").unwrap();
    validate_offset(a_cap, g.a_size, "a_cap").unwrap();
    validate_offset(a_cost, g.a_size, "a_cost").unwrap();
    validate_offset(a_x, g.a_size, "a_x").unwrap();
    validate_offset(v_pi, g.v_size, "v_pi").unwrap();

    let s = g.nv + 1;
    let t = s + 1;
    let nv = t;
    let mut na = g.na + 1;

    // First pass to count additional arcs
    for i in 1..=g.nv {
        let v = unsafe { *g.v.offset(i as isize) };
        let temp = if v_rhs >= 0 {
            read_double(v.data, v_rhs)
        } else {
            0.0
        };
        if temp != 0.0 {
            na += 1;
        }
    }

    // Allocate arrays
    let mut tail = vec![0; na as usize + 1];
    let mut head = vec![0; na as usize + 1];
    let mut low = vec![0; na as usize + 1];
    let mut cap = vec![0; na as usize + 1];
    let mut cost = vec![0; na as usize + 1];
    let mut x = vec![0; na as usize + 1];
    let mut pi = vec![0; nv as usize + 1];

    let mut k = 0;
    let mut ret = 0;

    // Process original arcs
    for i in 1..=g.nv {
        let v = unsafe { *g.v.offset(i as isize) };
        let mut a = v.out;

        while !a.is_null() {
            k += 1;
            let arc = unsafe { &*a };
            tail[k] = unsafe { (*arc.tail).i };
            head[k] = unsafe { (*arc.head).i };

            if tail[k] == head[k] {
                ret = 0x12;
                cleanup(&mut tail, &mut head, &mut low, &mut cap, &mut cost, &mut x, &mut pi);
                return ret;
            }

            let temp_low = if a_low >= 0 { read_double(arc.data, a_low) } else { 0.0 };
            if !(0.0 <= temp_low && temp_low <= i32::MAX as f64 && temp_low.floor() == temp_low) {
                ret = 0x12;
                cleanup(&mut tail, &mut head, &mut low, &mut cap, &mut cost, &mut x, &mut pi);
                return ret;
            }
            low[k] = temp_low as i32;

            let temp_cap = if a_cap >= 0 { read_double(arc.data, a_cap) } else { 1.0 };
            if !(low[k] as f64 <= temp_cap && temp_cap <= i32::MAX as f64 && temp_cap.floor() == temp_cap) {
                ret = 0x12;
                cleanup(&mut tail, &mut head, &mut low, &mut cap, &mut cost, &mut x, &mut pi);
                return ret;
            }
            cap[k] = temp_cap as i32;

            let temp_cost = if a_cost >= 0 { read_double(arc.data, a_cost) } else { 0.0 };
            if !(temp_cost.abs() <= i32::MAX as f64 && temp_cost.floor() == temp_cost) {
                ret = 0x12;
                cleanup(&mut tail, &mut head, &mut low, &mut cap, &mut cost, &mut x, &mut pi);
                return ret;
            }
            cost[k] = temp_cost as i32;

            a = arc.t_next;
        }
    }

    // Process node demands
    let mut sum = 0.0;
    for i in 1..=g.nv {
        let v = unsafe { *g.v.offset(i as isize) };
        let temp = if v_rhs >= 0 { read_double(v.data, v_rhs) } else { 0.0 };

        if !(temp.abs() <= i32::MAX as f64 && temp.floor() == temp) {
            ret = 0x12;
            cleanup(&mut tail, &mut head, &mut low, &mut cap, &mut cost, &mut x, &mut pi);
            return ret;
        }

        if temp > 0.0 {
            k += 1;
            tail[k] = s;
            head[k] = i;
            cap[k] = temp as i32;
            low[k] = cap[k];
            cost[k] = 0;
            sum += temp;
        } else if temp < 0.0 {
            k += 1;
            tail[k] = i;
            head[k] = t;
            cap[k] = (-temp) as i32;
            low[k] = cap[k];
            cost[k] = 0;
        }
    }

    // Add circulation arc
    k += 1;
    assert!(k == na);
    tail[k] = t;
    head[k] = s;

    if sum > i32::MAX as f64 {
        ret = 0x12;
    } else {
        cap[k] = sum as i32;
        low[k] = cap[k];
        cost[k] = 0;

        ret = unsafe {
            _glp_okalg(
                nv,
                na,
                tail.as_ptr(),
                head.as_ptr(),
                low.as_ptr(),
                cap.as_ptr(),
                cost.as_ptr(),
                x.as_mut_ptr(),
                pi.as_mut_ptr(),
            )
        };

        match ret {
            0 => ret = 0,
            1 => ret = 0xA,
            2 => ret = 0x13,
            3 => ret = 0x5,
            _ => panic!("Invalid return code from _glp_okalg"),
        }

        if ret == 0 || ret == 0xA {
            if let Some(sol_ref) = sol {
                *sol_ref = cost.iter().zip(x.iter()).map(|(&c, &x)| c as f64 * x as f64).sum();
            }

            if a_x >= 0 {
                let mut k = 0;
                for i in 1..=g.nv {
                    let v = unsafe { *g.v.offset(i as isize) };
                    let mut a = v.out;
                    while !a.is_null() {
                        k += 1;
                        let temp = x[k] as f64;
                        unsafe {
                            ptr::copy_nonoverlapping(
                                &temp as *const f64,
                                ((*a).data as *mut libc::c_char).offset(a_x as isize) as *mut f64,
                                1,
                            );
                        }
                        a = unsafe { (*a).t_next };
                    }
                }
            }

            if v_pi >= 0 {
                for i in 1..=g.nv {
                    let v = unsafe { *g.v.offset(i as isize) };
                    let temp = -pi[i] as f64;
                    unsafe {
                        ptr::copy_nonoverlapping(
                            &temp as *const f64,
                            (v.data as *mut libc::c_char).offset(v_pi as isize) as *mut f64,
                            1,
                        );
                    }
                }
            }
        }
    }

    cleanup(&mut tail, &mut head, &mut low, &mut cap, &mut cost, &mut x, &mut pi);
    ret
}

fn validate_offset(offset: libc::c_int, size: libc::c_int, name: &str) -> Result<(), ()> {
    if offset >= 0 && offset > size - mem::size_of::<f64>() as libc::c_int {
        panic!("{} = {}; invalid offset", name, offset);
    }
    Ok(())
}

fn read_double(ptr: *mut libc::c_void, offset: libc::c_int) -> f64 {
    let mut temp = 0.0f64;
    unsafe {
        ptr::copy_nonoverlapping(
            (ptr as *mut libc::c_char).offset(offset as isize) as *const f64,
            &mut temp as *mut f64,
            1,
        );
    }
    temp
}

fn cleanup(
    tail: &mut Vec<i32>,
    head: &mut Vec<i32>,
    low: &mut Vec<i32>,
    cap: &mut Vec<i32>,
    cost: &mut Vec<i32>,
    x: &mut Vec<i32>,
    pi: &mut Vec<i32>,
) {
    // Vectors will be automatically dropped
}

extern "C" {
    fn _glp_okalg(
        nv: libc::c_int,
        na: libc::c_int,
        tail: *const libc::c_int,
        head: *const libc::c_int,
        low: *const libc::c_int,
        cap: *const libc::c_int,
        cost: *const libc::c_int,
        x: *mut libc::c_int,
        pi: *mut libc::c_int,
    ) -> libc::c_int;
}