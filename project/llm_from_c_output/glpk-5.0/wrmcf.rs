use std::fs::File;
use std::io::{Write, Error, ErrorKind};
use std::path::Path;
use std::fmt::{Display, Formatter, Result as FmtResult};

struct GlpGraph {
    name: Option<String>,
    nv: usize,
    na: usize,
    v: Vec<GlpVertex>,
    v_size: usize,
    a_size: usize,
}

struct GlpVertex {
    i: usize,
    out: Option<Box<GlpArc>>,
    data: Vec<u8>,
}

struct GlpArc {
    tail: *const GlpVertex,
    head: *const GlpVertex,
    t_next: Option<Box<GlpArc>>,
    data: Vec<u8>,
}

impl GlpGraph {
    fn write_mincost(
        &self,
        v_rhs: Option<usize>,
        a_low: Option<usize>,
        a_cap: Option<usize>,
        a_cost: Option<usize>,
        fname: &str,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        if let Some(offset) = v_rhs {
            if offset > self.v_size - std::mem::size_of::<f64>() {
                return Err(Box::new(Error::new(
                    ErrorKind::InvalidInput,
                    format!("glp_write_mincost: v_rhs = {}; invalid offset", offset),
                )));
            }
        }

        if let Some(offset) = a_low {
            if offset > self.a_size - std::mem::size_of::<f64>() {
                return Err(Box::new(Error::new(
                    ErrorKind::InvalidInput,
                    format!("glp_write_mincost: a_low = {}; invalid offset", offset),
                )));
            }
        }

        if let Some(offset) = a_cap {
            if offset > self.a_size - std::mem::size_of::<f64>() {
                return Err(Box::new(Error::new(
                    ErrorKind::InvalidInput,
                    format!("glp_write_mincost: a_cap = {}; invalid offset", offset),
                )));
            }
        }

        if let Some(offset) = a_cost {
            if offset > self.a_size - std::mem::size_of::<f64>() {
                return Err(Box::new(Error::new(
                    ErrorKind::InvalidInput,
                    format!("glp_write_mincost: a_cost = {}; invalid offset", offset),
                )));
            }
        }

        println!("Writing min-cost flow problem data to '{}'...", fname);

        let mut file = File::create(fname)?;
        let mut count = 0;

        writeln!(
            file,
            "c {}",
            self.name.as_ref().map_or("unknown", |n| n.as_str())
        )?;
        count += 1;

        writeln!(file, "p min {} {}", self.nv, self.na)?;
        count += 1;

        if let Some(offset) = v_rhs {
            for i in 1..=self.nv {
                let v = &self.v[i - 1];
                let rhs = unsafe {
                    let ptr = v.data.as_ptr().add(offset) as *const f64;
                    *ptr
                };
                if rhs != 0.0 {
                    writeln!(file, "n {} {:.15}", i, rhs)?;
                    count += 1;
                }
            }
        }

        for i in 1..=self.nv {
            let v = &self.v[i - 1];
            let mut a = &v.out;
            while let Some(arc) = a {
                let low = if let Some(offset) = a_low {
                    unsafe {
                        let ptr = arc.data.as_ptr().add(offset) as *const f64;
                        *ptr
                    }
                } else {
                    0.0
                };

                let cap = if let Some(offset) = a_cap {
                    unsafe {
                        let ptr = arc.data.as_ptr().add(offset) as *const f64;
                        *ptr
                    }
                } else {
                    1.0
                };

                let cost = if let Some(offset) = a_cost {
                    unsafe {
                        let ptr = arc.data.as_ptr().add(offset) as *const f64;
                        *ptr
                    }
                } else {
                    0.0
                };

                writeln!(
                    file,
                    "a {} {} {:.15} {:.15} {:.15}",
                    unsafe { (*arc.tail).i },
                    unsafe { (*arc.head).i },
                    low,
                    cap,
                    cost
                )?;
                count += 1;
                a = &arc.t_next;
            }
        }

        writeln!(file, "c eof")?;
        count += 1;

        file.sync_all()?;
        println!("{} lines were written", count);
        Ok(count)
    }
}

// Note: The unsafe blocks are necessary here due to pointer usage in the original C code.
// In a real Rust implementation, you would want to redesign the data structures
// to avoid raw pointers and unsafe code where possible.