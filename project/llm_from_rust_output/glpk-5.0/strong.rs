use std::ptr;
use std::mem;
use std::ffi::CString;

#[derive(Debug, Clone)]
pub struct GlpGraph {
    pool: Option<Box<[u8]>>,
    name: Option<CString>,
    nv_max: i32,
    nv: i32,
    na: i32,
    v: Vec<Option<GlpVertex>>,
    index: Option<Box<[u8]>>,
    v_size: i32,
    a_size: i32,
}

#[derive(Debug, Clone)]
pub struct GlpVertex {
    i: i32,
    name: Option<CString>,
    entry: Option<Box<[u8]>>,
    data: Option<Box<[u8]>>,
    temp: Option<Box<[u8]>>,
    in_0: Vec<GlpArc>,
    out: Vec<GlpArc>,
}

#[derive(Debug, Clone)]
pub struct GlpArc {
    tail: usize,
    head: usize,
    data: Option<Box<[u8]>>,
    temp: Option<Box<[u8]>>,
}

pub fn glp_strong_comp(graph: &mut GlpGraph, v_num: i32) -> i32 {
    if v_num >= 0 && v_num > graph.v_size - mem::size_of::<i32>() as i32 {
        panic!("glp_strong_comp: v_num = {}; invalid offset", v_num);
    }

    let n = graph.nv;
    if n == 0 {
        return 0;
    }

    let na = graph.na;
    let mut icn = vec![0; (na + 1) as usize];
    let mut ip = vec![0; (n + 1) as usize];
    let mut lenr = vec![0; (n + 1) as usize];
    let mut ior = vec![0; (n + 1) as usize];
    let mut ib = vec![0; (n + 1) as usize];
    let mut lowl = vec![0; (n + 1) as usize];
    let mut numb = vec![0; (n + 1) as usize];
    let mut prev = vec![0; (n + 1) as usize];

    let mut k = 1;
    for i in 1..=n {
        if let Some(v) = graph.v.get_mut(i as usize).and_then(|v| v.as_mut()) {
            ip[i as usize] = k;
            for a in &v.out {
                icn[k as usize] = graph.v[a.head as usize].as_ref().unwrap().i;
                k += 1;
            }
            lenr[i as usize] = k - ip[i as usize];
        }
    }

    assert_eq!(na, k - 1, "na == k-1");

    // Note: _glp_mc13d is an external C function that would need to be properly wrapped
    // For this translation, we assume it's been properly wrapped in Rust
    let nc = unsafe {
        _glp_mc13d(
            n,
            icn.as_ptr(),
            ip.as_ptr(),
            lenr.as_ptr(),
            ior.as_mut_ptr(),
            ib.as_mut_ptr(),
            lowl.as_mut_ptr(),
            numb.as_mut_ptr(),
            prev.as_mut_ptr(),
        )
    };

    if v_num >= 0 {
        assert_eq!(ib[1], 1, "ib[1] == 1");
        let mut k = 1;
        while k <= nc {
            let last = if k < nc { ib[(k + 1) as usize] } else { n + 1 };
            assert!(ib[k as usize] < last, "ib[k] < last");
            
            let mut i = ib[k as usize];
            while i < last {
                if let Some(v) = graph.v.get_mut(ior[i as usize] as usize).and_then(|v| v.as_mut()) {
                    if let Some(data) = &mut v.data {
                        let offset = v_num as usize * mem::size_of::<i32>();
                        let bytes = k.to_ne_bytes();
                        data[offset..offset + mem::size_of::<i32>()].copy_from_slice(&bytes);
                    }
                }
                i += 1;
            }
            k += 1;
        }
    }

    nc
}

// This would be the safe wrapper around the C function
extern "C" {
    fn _glp_mc13d(
        n: i32,
        icn: *const i32,
        ip: *const i32,
        lenr: *const i32,
        ior: *mut i32,
        ib: *mut i32,
        lowl: *mut i32,
        numb: *mut i32,
        prev: *mut i32,
    ) -> i32;
}