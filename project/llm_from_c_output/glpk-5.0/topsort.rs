use std::collections::VecDeque;
use std::mem;

/// Represents a graph vertex
struct Vertex {
    in_arcs: Vec<Arc>,
    out_arcs: Vec<Arc>,
    data: Vec<u8>,
}

/// Represents a graph arc
struct Arc {
    head: usize, // index of head vertex
}

/// Represents a directed graph
struct Graph {
    vertices: Vec<Vertex>,
    v_size: usize,
}

impl Graph {
    /// Performs topological sorting of vertices
    ///
    /// # Arguments
    /// * `v_num` - offset in vertex data block to store the vertex number
    ///
    /// # Returns
    /// Ok(0) if graph is acyclic, otherwise Err(count) where count is the number
    /// of unnumbered vertices
    fn top_sort(&mut self, v_num: Option<usize>) -> Result<(), usize> {
        if let Some(offset) = v_num {
            if offset > self.v_size.saturating_sub(mem::size_of::<i32>()) {
                panic!("glp_top_sort: invalid offset");
            }
        }

        if self.vertices.is_empty() {
            return Ok(());
        }

        let n = self.vertices.len();
        let mut num = vec![0; n + 1]; // 1-based indexing
        let mut indeg = vec![0; n + 1];
        let mut stack = VecDeque::new();

        // Calculate initial indegree and push zero-indegree vertices
        for i in 1..=n {
            num[i] = 0;
            indeg[i] = self.vertices[i - 1].in_arcs.len();
            if indeg[i] == 0 {
                stack.push_back(i);
            }
        }

        let mut cnt = 0;
        while let Some(i) = stack.pop_back() {
            assert_eq!(indeg[i], 0);
            assert_eq!(num[i], 0);
            cnt += 1;
            num[i] = cnt;

            // Update indegree of adjacent vertices
            for arc in &self.vertices[i - 1].out_arcs {
                let j = arc.head;
                assert!(indeg[j] > 0);
                indeg[j] -= 1;
                if indeg[j] == 0 {
                    stack.push_back(j);
                }
            }
        }

        if cnt != n {
            return Err(n - cnt);
        }

        // Store numbers if requested
        if let Some(offset) = v_num {
            for i in 1..=n {
                let num_i = num[i] as i32;
                let v_data = &mut self.vertices[i - 1].data;
                if offset + mem::size_of::<i32>() <= v_data.len() {
                    v_data[offset..offset + mem::size_of::<i32>()]
                        .copy_from_slice(&num_i.to_ne_bytes());
                }
            }
        }

        Ok(())
    }
}