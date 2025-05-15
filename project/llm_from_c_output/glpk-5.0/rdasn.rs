use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError, ErrorKind as IoErrorKind};
use std::path::Path;
use std::str::FromStr;
use std::collections::HashSet;

pub struct GlpGraph {
    // Graph data structure implementation
    // Placeholder for actual implementation
    v_size: usize,
    a_size: usize,
    vertices: Vec<GlpVertex>,
    arcs: Vec<GlpArc>,
}

pub struct GlpVertex {
    data: Vec<u8>,
}

pub struct GlpArc {
    from: usize,
    to: usize,
    data: Vec<u8>,
}

impl GlpGraph {
    pub fn new(v_size: usize, a_size: usize) -> Self {
        GlpGraph {
            v_size,
            a_size,
            vertices: Vec::new(),
            arcs: Vec::new(),
        }
    }

    pub fn erase_graph(&mut self, v_size: usize, a_size: usize) {
        self.vertices.clear();
        self.arcs.clear();
        self.v_size = v_size;
        self.a_size = a_size;
    }

    pub fn add_vertices(&mut self, count: usize) {
        for _ in 0..count {
            self.vertices.push(GlpVertex {
                data: vec![0; self.v_size],
            });
        }
    }

    pub fn add_arc(&mut self, from: usize, to: usize) -> &mut GlpArc {
        self.arcs.push(GlpArc {
            from,
            to,
            data: vec![0; self.a_size],
        });
        self.arcs.last_mut().unwrap()
    }

    pub fn v(&self, index: usize) -> &GlpVertex {
        &self.vertices[index - 1]
    }

    pub fn v_mut(&mut self, index: usize) -> &mut GlpVertex {
        &mut self.vertices[index - 1]
    }
}

struct DimacsReader {
    fname: String,
    reader: BufReader<File>,
    line_count: usize,
    current_line: String,
    current_field: String,
    current_pos: usize,
}

impl DimacsReader {
    fn new(fname: &str) -> Result<Self, IoError> {
        let file = File::open(fname)?;
        let reader = BufReader::new(file);
        Ok(DimacsReader {
            fname: fname.to_string(),
            reader,
            line_count: 0,
            current_line: String::new(),
            current_field: String::new(),
            current_pos: 0,
        })
    }

    fn read_line(&mut self) -> Result<(), IoError> {
        self.current_line.clear();
        self.current_pos = 0;
        let bytes_read = self.reader.read_line(&mut self.current_line)?;
        if bytes_read == 0 {
            return Err(IoError::new(IoErrorKind::UnexpectedEof, "End of file"));
        }
        self.line_count += 1;
        Ok(())
    }

    fn read_designator(&mut self) -> Result<(), IoError> {
        self.read_field()?;
        Ok(())
    }

    fn read_field(&mut self) -> Result<(), IoError> {
        self.current_field.clear();
        let line = self.current_line.trim_start();
        
        // Skip whitespace
        let mut chars = line[self.current_pos..].chars().peekable();
        while let Some(c) = chars.peek() {
            if c.is_whitespace() {
                chars.next();
                self.current_pos += 1;
            } else {
                break;
            }
        }

        // Read field
        while let Some(c) = chars.next() {
            if c.is_whitespace() {
                break;
            }
            self.current_field.push(c);
            self.current_pos += 1;
        }

        if self.current_field.is_empty() {
            return Err(IoError::new(IoErrorKind::InvalidData, "Empty field"));
        }
        Ok(())
    }

    fn end_of_line(&mut self) -> Result<(), IoError> {
        let remaining = self.current_line[self.current_pos..].trim();
        if !remaining.is_empty() {
            return Err(IoError::new(
                IoErrorKind::InvalidData,
                format!("Unexpected data at end of line: '{}'", remaining),
            ));
        }
        Ok(())
    }
}

pub fn glp_read_asnprob(
    graph: &mut GlpGraph,
    v_set: Option<usize>,
    a_cost: Option<usize>,
    fname: &str,
) -> Result<(), String> {
    // Validate offsets
    if let Some(vs) = v_set {
        if vs > graph.v_size.saturating_sub(std::mem::size_of::<i32>()) {
            return Err(format!("glp_read_asnprob: v_set = {}; invalid offset", vs));
        }
    }

    if let Some(ac) = a_cost {
        if ac > graph.a_size.saturating_sub(std::mem::size_of::<f64>()) {
            return Err(format!("glp_read_asnprob: a_cost = {}; invalid offset", ac));
        }
    }

    graph.erase_graph(graph.v_size, graph.a_size);

    println!("Reading assignment problem data from '{}'...", fname);

    let mut reader = DimacsReader::new(fname).map_err(|e| e.to_string())?;

    // Read problem line
    reader.read_designator().map_err(|e| e.to_string())?;
    if reader.current_field != "p" {
        return Err("problem line missing or invalid".to_string());
    }

    reader.read_field().map_err(|e| e.to_string())?;
    if reader.current_field != "asn" {
        return Err("wrong problem designator; 'asn' expected".to_string());
    }

    reader.read_field().map_err(|e| e.to_string())?;
    let nv: usize = reader.current_field.parse().map_err(|_| {
        "number of nodes missing or invalid".to_string()
    })?;

    reader.read_field().map_err(|e| e.to_string())?;
    let na: usize = reader.current_field.parse().map_err(|_| {
        "number of arcs missing or invalid".to_string()
    })?;

    if nv > 0 {
        graph.add_vertices(nv);
    }

    reader.end_of_line().map_err(|e| e.to_string())?;

    // Read node descriptor lines
    let mut flag = vec![false; nv + 1]; // 1-based indexing
    let mut n1 = 0;

    loop {
        if let Err(_) = reader.read_designator() {
            break;
        }

        if reader.current_field != "n" {
            break;
        }

        reader.read_field().map_err(|e| e.to_string())?;
        let i: usize = reader.current_field.parse().map_err(|_| {
            "node number missing or invalid".to_string()
        })?;

        if !(1..=nv).contains(&i) {
            return Err(format!("node number {} out of range", i));
        }

        if flag[i] {
            return Err(format!("duplicate descriptor of node {}", i));
        }

        flag[i] = true;
        n1 += 1;

        reader.end_of_line().map_err(|e| e.to_string())?;
    }

    println!(
        "Assignment problem has {} + {} = {} node{} and {} arc{}",
        n1,
        nv - n1,
        nv,
        if nv == 1 { "" } else { "s" },
        na,
        if na == 1 { "" } else { "s" }
    );

    if let Some(vs) = v_set {
        for i in 1..=nv {
            let k = if flag[i] { 0 } else { 1 };
            let v = graph.v_mut(i);
            let k_bytes = k.to_ne_bytes();
            v.data[vs..vs + std::mem::size_of::<i32>()].copy_from_slice(&k_bytes);
        }
    }

    // Read arc descriptor lines
    for k in 1..=na {
        if k > 1 {
            reader.read_designator().map_err(|e| e.to_string())?;
        }

        if reader.current_field != "a" {
            return Err("wrong line designator; 'a' expected".to_string());
        }

        reader.read_field().map_err(|e| e.to_string())?;
        let i: usize = reader.current_field.parse().map_err(|_| {
            "starting node number missing or invalid".to_string()
        })?;

        if !(1..=nv).contains(&i) {
            return Err(format!("starting node number {} out of range", i));
        }

        if !flag[i] {
            return Err(format!("node {} cannot be a starting node", i));
        }

        reader.read_field().map_err(|e| e.to_string())?;
        let j: usize = reader.current_field.parse().map_err(|_| {
            "ending node number missing or invalid".to_string()
        })?;

        if !(1..=nv).contains(&j) {
            return Err(format!("ending node number {} out of range", j));
        }

        if flag[j] {
            return Err(format!("node {} cannot be an ending node", j));
        }

        reader.read_field().map_err(|e| e.to_string())?;
        let cost: f64 = reader.current_field.parse().map_err(|_| {
            "arc cost missing or invalid".to_string()
        })?;

        // Check if cost is integer (with some tolerance for floating point)
        if (cost - cost.round()).abs() > 1e-9 {
            return Err("non-integer arc cost".to_string());
        }

        let a = graph.add_arc(i, j);
        if let Some(ac) = a_cost {
            let cost_bytes = cost.to_ne_bytes();
            a.data[ac..ac + std::mem::size_of::<f64>()].copy_from_slice(&cost_bytes);
        }

        reader.end_of_line().map_err(|e| e.to_string())?;
    }

    println!("{} lines were read", reader.line_count);
    Ok(())
}