use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::slice;

const NV_MAX: usize = 100_000_000;
const NA_MAX: usize = 500_000_000;

pub struct GlpGraph {
    name: Option<String>,
    vertices: Vec<GlpVertex>,
    arcs: Vec<GlpArc>,
    v_size: usize,
    a_size: usize,
    index: Option<HashMap<String, usize>>,
}

pub struct GlpVertex {
    i: usize,
    name: Option<String>,
    data: Option<Vec<u8>>,
    in_arcs: Vec<usize>,
    out_arcs: Vec<usize>,
}

pub struct GlpArc {
    tail: usize,
    head: usize,
    data: Option<Vec<u8>>,
}

impl GlpGraph {
    pub fn create_graph(v_size: usize, a_size: usize) -> Result<Self, String> {
        if v_size > 256 {
            return Err(format!("glp_create_graph: v_size = {}; invalid size of vertex data", v_size));
        }
        if a_size > 256 {
            return Err(format!("glp_create_graph: a_size = {}; invalid size of arc data", a_size));
        }

        Ok(GlpGraph {
            name: None,
            vertices: Vec::new(),
            arcs: Vec::new(),
            v_size,
            a_size,
            index: None,
        })
    }

    pub fn set_graph_name(&mut self, name: Option<&str>) -> Result<(), String> {
        if let Some(name_str) = name {
            if name_str.len() > 255 {
                return Err("glp_set_graph_name: graph name too long".to_string());
            }
            if name_str.chars().any(|c| c.is_control()) {
                return Err("glp_set_graph_name: graph name contains invalid character(s)".to_string());
            }
            self.name = Some(name_str.to_string());
        } else {
            self.name = None;
        }
        Ok(())
    }

    pub fn add_vertices(&mut self, nadd: usize) -> Result<usize, String> {
        if nadd == 0 {
            return Err("glp_add_vertices: nadd = 0; invalid number of vertices".to_string());
        }
        if self.vertices.len() + nadd > NV_MAX {
            return Err("glp_add_vertices: too many vertices".to_string());
        }

        let first_new = self.vertices.len() + 1;
        for i in first_new..first_new + nadd {
            let mut data = None;
            if self.v_size > 0 {
                data = Some(vec![0; self.v_size]);
            }
            self.vertices.push(GlpVertex {
                i,
                name: None,
                data,
                in_arcs: Vec::new(),
                out_arcs: Vec::new(),
            });
        }
        Ok(first_new)
    }

    pub fn set_vertex_name(&mut self, i: usize, name: Option<&str>) -> Result<(), String> {
        if i == 0 || i > self.vertices.len() {
            return Err(format!("glp_set_vertex_name: i = {}; vertex number out of range", i));
        }

        let vertex = &mut self.vertices[i - 1];
        if let Some(ref old_name) = vertex.name {
            if let Some(ref mut index) = self.index {
                index.remove(old_name);
            }
        }

        if let Some(name_str) = name {
            if name_str.len() > 255 {
                return Err(format!("glp_set_vertex_name: i = {}; vertex name too long", i));
            }
            if name_str.chars().any(|c| c.is_control()) {
                return Err(format!("glp_set_vertex_name: i = {}; vertex name contains invalid character(s)", i));
            }

            vertex.name = Some(name_str.to_string());
            if let Some(ref mut index) = self.index {
                index.insert(name_str.to_string(), i);
            }
        } else {
            vertex.name = None;
        }
        Ok(())
    }

    pub fn add_arc(&mut self, i: usize, j: usize) -> Result<usize, String> {
        if i == 0 || i > self.vertices.len() {
            return Err(format!("glp_add_arc: i = {}; tail vertex number out of range", i));
        }
        if j == 0 || j > self.vertices.len() {
            return Err(format!("glp_add_arc: j = {}; head vertex number out of range", j));
        }
        if self.arcs.len() >= NA_MAX {
            return Err("glp_add_arc: too many arcs".to_string());
        }

        let arc_id = self.arcs.len();
        let mut data = None;
        if self.a_size > 0 {
            data = Some(vec![0; self.a_size]);
        }

        self.arcs.push(GlpArc {
            tail: i - 1,
            head: j - 1,
            data,
        });

        self.vertices[i - 1].out_arcs.push(arc_id);
        self.vertices[j - 1].in_arcs.push(arc_id);

        Ok(arc_id)
    }

    pub fn del_vertices(&mut self, vertices: &[usize]) -> Result<(), String> {
        if vertices.is_empty() || vertices.len() > self.vertices.len() {
            return Err("glp_del_vertices: invalid number of vertices".to_string());
        }

        // Mark vertices to be deleted
        let mut to_delete = vec![false; self.vertices.len()];
        for &i in vertices {
            if i == 0 || i > self.vertices.len() {
                return Err(format!("glp_del_vertices: num[{}] = {}; vertex number out of range", i, i));
            }
            if to_delete[i - 1] {
                return Err(format!("glp_del_vertices: num[{}] = {}; duplicate vertex numbers not allowed", i, i));
            }
            to_delete[i - 1] = true;
        }

        // Delete all arcs connected to these vertices
        let mut arcs_to_keep = Vec::new();
        for (arc_id, arc) in self.arcs.iter().enumerate() {
            if !to_delete[arc.tail] && !to_delete[arc.head] {
                arcs_to_keep.push(arc_id);
            }
        }

        // Rebuild arcs and update vertex connections
        let mut new_arcs = Vec::with_capacity(arcs_to_keep.len());
        let mut arc_mapping = vec![None; self.arcs.len()];
        for (new_id, &old_id) in arcs_to_keep.iter().enumerate() {
            arc_mapping[old_id] = Some(new_id);
            new_arcs.push(self.arcs[old_id].clone());
        }
        self.arcs = new_arcs;

        // Rebuild vertices and update indices
        let mut new_vertices = Vec::new();
        let mut vertex_mapping = vec![0; self.vertices.len()];
        let mut new_index = 1;
        for (old_id, vertex) in self.vertices.iter().enumerate() {
            if !to_delete[old_id] {
                vertex_mapping[old_id] = new_index;
                new_index += 1;

                let mut new_vertex = GlpVertex {
                    i: vertex_mapping[old_id],
                    name: vertex.name.clone(),
                    data: vertex.data.clone(),
                    in_arcs: Vec::new(),
                    out_arcs: Vec::new(),
                };

                // Update arc references
                for &arc_id in &vertex.in_arcs {
                    if let Some(new_arc_id) = arc_mapping[arc_id] {
                        new_vertex.in_arcs.push(new_arc_id);
                    }
                }
                for &arc_id in &vertex.out_arcs {
                    if let Some(new_arc_id) = arc_mapping[arc_id] {
                        new_vertex.out_arcs.push(new_arc_id);
                    }
                }

                new_vertices.push(new_vertex);
            }
        }
        self.vertices = new_vertices;

        // Update index if it exists
        if let Some(ref mut index) = self.index {
            index.clear();
            for vertex in &self.vertices {
                if let Some(ref name) = vertex.name {
                    index.insert(name.clone(), vertex.i);
                }
            }
        }

        Ok(())
    }

    pub fn del_arc(&mut self, arc_id: usize) -> Result<(), String> {
        if arc_id >= self.arcs.len() {
            return Err("glp_del_arc: arc does not exist".to_string());
        }

        let arc = &self.arcs[arc_id];
        let tail = arc.tail;
        let head = arc.head;

        // Remove from tail's out_arcs
        if let Some(pos) = self.vertices[tail].out_arcs.iter().position(|&x| x == arc_id) {
            self.vertices[tail].out_arcs.remove(pos);
        }

        // Remove from head's in_arcs
        if let Some(pos) = self.vertices[head].in_arcs.iter().position(|&x| x == arc_id) {
            self.vertices[head].in_arcs.remove(pos);
        }

        // Mark as deleted (we'll actually remove it in a compaction step)
        // In a real implementation, we'd need a more sophisticated approach
        // This is simplified for demonstration
        self.arcs[arc_id].tail = usize::MAX;
        self.arcs[arc_id].head = usize::MAX;

        Ok(())
    }

    pub fn erase_graph(&mut self, v_size: usize, a_size: usize) -> Result<(), String> {
        if v_size > 256 {
            return Err("glp_erase_graph: v_size too large".to_string());
        }
        if a_size > 256 {
            return Err("glp_erase_graph: a_size too large".to_string());
        }

        *self = GlpGraph::create_graph(v_size, a_size)?;
        Ok(())
    }

    pub fn create_v_index(&mut self) {
        if self.index.is_none() {
            let mut index = HashMap::new();
            for vertex in &self.vertices {
                if let Some(ref name) = vertex.name {
                    index.insert(name.clone(), vertex.i);
                }
            }
            self.index = Some(index);
        }
    }

    pub fn find_vertex(&self, name: &str) -> usize {
        if let Some(ref index) = self.index {
            if !name.is_empty() && name.len() <= 255 {
                if let Some(&i) = index.get(name) {
                    return i;
                }
            }
        }
        0
    }

    pub fn delete_v_index(&mut self) {
        self.index = None;
    }
}

impl Drop for GlpGraph {
    fn drop(&mut self) {
        // Rust's ownership system will automatically clean up the memory
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_graph() {
        let graph = GlpGraph::create_graph(0, 0).unwrap();
        assert_eq!(graph.vertices.len(), 0);
        assert_eq!(graph.arcs.len(), 0);
    }

    #[test]
    fn test_add_vertices() {
        let mut graph = GlpGraph::create_graph(0, 0).unwrap();
        let first = graph.add_vertices(5).unwrap();
        assert_eq!(first, 1);
        assert_eq!(graph.vertices.len(), 5);
    }
}