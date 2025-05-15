use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct GraphError {
    details: String,
}

impl GraphError {
    fn new(msg: &str) -> GraphError {
        GraphError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for GraphError {
    fn description(&self) -> &str {
        &self.details
    }
}

struct Vertex<T> {
    data: T,
    in_arcs: Vec<Arc>,
    out_arcs: Vec<Arc>,
}

struct Arc {
    tail: usize,
    head: usize,
}

struct Graph<T> {
    vertices: Vec<Vertex<T>>,
    v_size: usize,
}

impl<T> Graph<T> {
    fn nv(&self) -> usize {
        self.vertices.len()
    }

    fn weak_comp(&mut self, v_num: Option<usize>) -> Result<usize, GraphError> {
        if let Some(offset) = v_num {
            if offset > self.v_size - std::mem::size_of::<usize>() {
                return Err(GraphError::new(
                    "glp_weak_comp: v_num out of bounds for vertex data",
                ));
            }
        }

        let nv = self.nv();
        if nv == 0 {
            return Ok(0);
        }

        let mut prev = vec![0; nv + 1];
        let mut next = vec![0; nv + 1];
        let mut list = vec![0; nv + 1];

        let mut f = 1;
        for i in 1..=nv {
            prev[i] = i - 1;
            next[i] = i + 1;
        }
        next[nv] = 0;

        let mut nc = 0;
        while f != 0 {
            let i = f;
            f = next[i];
            if f != 0 {
                prev[f] = 0;
            }

            prev[i] = -1;
            nc += 1;
            next[i] = nc;

            list[1] = i;
            let mut pos1 = 1;
            let mut pos2 = 1;

            while pos1 <= pos2 {
                let i = list[pos1];
                pos1 += 1;

                for arc in &self.vertices[i - 1].in_arcs {
                    let j = arc.tail;
                    if prev[j] >= 0 {
                        if prev[j] == 0 {
                            f = next[j];
                        } else {
                            next[prev[j] as usize] = next[j];
                        }

                        if next[j] != 0 {
                            prev[next[j] as usize] = prev[j];
                        }

                        prev[j] = -1;
                        next[j] = nc;
                        pos2 += 1;
                        list[pos2] = j;
                    }
                }

                for arc in &self.vertices[i - 1].out_arcs {
                    let j = arc.head;
                    if prev[j] >= 0 {
                        if prev[j] == 0 {
                            f = next[j];
                        } else {
                            next[prev[j] as usize] = next[j];
                        }

                        if next[j] != 0 {
                            prev[next[j] as usize] = prev[j];
                        }

                        prev[j] = -1;
                        next[j] = nc;
                        pos2 += 1;
                        list[pos2] = j;
                    }
                }
            }
        }

        if let Some(offset) = v_num {
            for i in 1..=nv {
                let component = next[i];
                unsafe {
                    let v_data = &mut self.vertices[i - 1].data as *mut T as *mut u8;
                    std::ptr::copy_nonoverlapping(
                        &component as *const usize as *const u8,
                        v_data.add(offset),
                        std::mem::size_of::<usize>(),
                    );
                }
            }
        }

        Ok(nc)
    }
}