use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError, ErrorKind as IoErrorKind};
use std::path::Path;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug)]
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
        Ok(Self {
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
            return Err(IoError::new(IoErrorKind::UnexpectedEof, "Unexpected EOF"));
        }
        self.line_count += 1;
        Ok(())
    }

    fn read_designator(&mut self) -> Result<(), IoError> {
        self.skip_whitespace();
        let start = self.current_pos;
        while self.current_pos < self.current_line.len() {
            let c = self.current_line.as_bytes()[self.current_pos];
            if c.is_ascii_whitespace() {
                break;
            }
            self.current_pos += 1;
        }
        self.current_field = self.current_line[start..self.current_pos].to_string();
        Ok(())
    }

    fn read_field(&mut self) -> Result<(), IoError> {
        self.skip_whitespace();
        let start = self.current_pos;
        while self.current_pos < self.current_line.len() {
            let c = self.current_line.as_bytes()[self.current_pos];
            if c.is_ascii_whitespace() {
                break;
            }
            self.current_pos += 1;
        }
        self.current_field = self.current_line[start..self.current_pos].to_string();
        Ok(())
    }

    fn skip_whitespace(&mut self) {
        while self.current_pos < self.current_line.len() {
            let c = self.current_line.as_bytes()[self.current_pos];
            if !c.is_ascii_whitespace() {
                break;
            }
            self.current_pos += 1;
        }
    }

    fn end_of_line(&mut self) -> Result<(), IoError> {
        self.skip_whitespace();
        if self.current_pos < self.current_line.len() {
            return Err(IoError::new(
                IoErrorKind::InvalidData,
                format!("Extra characters at line {}", self.line_count),
            ));
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Graph {
    vertices: Vec<Vertex>,
    arcs: Vec<Arc>,
    v_size: usize,
    a_size: usize,
}

#[derive(Debug)]
struct Vertex {
    data: Vec<u8>,
    outgoing: Vec<usize>,
}

#[derive(Debug)]
struct Arc {
    data: Vec<u8>,
    source: usize,
    target: usize,
}

impl Graph {
    fn new(v_size: usize, a_size: usize) -> Self {
        Self {
            vertices: Vec::new(),
            arcs: Vec::new(),
            v_size,
            a_size,
        }
    }

    fn add_vertices(&mut self, count: usize) {
        for _ in 0..count {
            self.vertices.push(Vertex {
                data: vec![0; self.v_size],
                outgoing: Vec::new(),
            });
        }
    }

    fn add_arc(&mut self, i: usize, j: usize) -> &mut Arc {
        let arc_idx = self.arcs.len();
        self.arcs.push(Arc {
            data: vec![0; self.a_size],
            source: i - 1,
            target: j - 1,
        });
        self.vertices[i - 1].outgoing.push(arc_idx);
        &mut self.arcs[arc_idx]
    }

    fn erase(&mut self) {
        self.vertices.clear();
        self.arcs.clear();
    }
}

fn glp_read_mincost(
    graph: &mut Graph,
    v_rhs: Option<usize>,
    a_low: Option<usize>,
    a_cap: Option<usize>,
    a_cost: Option<usize>,
    fname: &str,
) -> Result<(), String> {
    // Validate offsets
    if let Some(offset) = v_rhs {
        if offset > graph.v_size - std::mem::size_of::<f64>() {
            return Err(format!("v_rhs = {}; invalid offset", offset));
        }
    }
    if let Some(offset) = a_low {
        if offset > graph.a_size - std::mem::size_of::<f64>() {
            return Err(format!("a_low = {}; invalid offset", offset));
        }
    }
    if let Some(offset) = a_cap {
        if offset > graph.a_size - std::mem::size_of::<f64>() {
            return Err(format!("a_cap = {}; invalid offset", offset));
        }
    }
    if let Some(offset) = a_cost {
        if offset > graph.a_size - std::mem::size_of::<f64>() {
            return Err(format!("a_cost = {}; invalid offset", offset));
        }
    }

    graph.erase();
    let mut reader = DimacsReader::new(fname).map_err(|e| e.to_string())?;

    println!("Reading min-cost flow problem data from '{}'...", fname);

    // Read problem line
    reader.read_line().map_err(|e| e.to_string())?;
    reader.read_designator().map_err(|e| e.to_string())?;
    if reader.current_field != "p" {
        return Err("problem line missing or invalid".to_string());
    }

    reader.read_field().map_err(|e| e.to_string())?;
    if reader.current_field != "min" {
        return Err("wrong problem designator; 'min' expected".to_string());
    }

    reader.read_field().map_err(|e| e.to_string())?;
    let nv: usize = reader.current_field.parse().map_err(|_| {
        format!("number of nodes missing or invalid at line {}", reader.line_count)
    })?;

    reader.read_field().map_err(|e| e.to_string())?;
    let na: usize = reader.current_field.parse().map_err(|_| {
        format!("number of arcs missing or invalid at line {}", reader.line_count)
    })?;

    println!(
        "Flow network has {} node{} and {} arc{}",
        nv,
        if nv == 1 { "" } else { "s" },
        na,
        if na == 1 { "" } else { "s" }
    );

    if nv > 0 {
        graph.add_vertices(nv);
    }
    reader.end_of_line().map_err(|e| e.to_string())?;

    // Read node descriptor lines
    let mut node_flags = vec![false; nv];
    if let Some(offset) = v_rhs {
        for vertex in &mut graph.vertices {
            let rhs = 0.0f64;
            vertex.data[offset..offset + std::mem::size_of::<f64>()]
                .copy_from_slice(&rhs.to_ne_bytes());
        }
    }

    loop {
        reader.read_line().map_err(|e| e.to_string())?;
        reader.read_designator().map_err(|e| e.to_string())?;
        if reader.current_field != "n" {
            break;
        }

        reader.read_field().map_err(|e| e.to_string())?;
        let i: usize = reader.current_field.parse().map_err(|_| {
            format!(
                "node number missing or invalid at line {}",
                reader.line_count
            )
        })?;

        if i < 1 || i > nv {
            return Err(format!(
                "node number {} out of range at line {}",
                i, reader.line_count
            ));
        }

        if node_flags[i - 1] {
            return Err(format!(
                "duplicate descriptor of node {} at line {}",
                i, reader.line_count
            ));
        }

        reader.read_field().map_err(|e| e.to_string())?;
        let rhs: f64 = reader.current_field.parse().map_err(|_| {
            format!(
                "node supply/demand missing or invalid at line {}",
                reader.line_count
            )
        })?;

        if rhs.fract() != 0.0 {
            return Err(format!(
                "non-integer value at line {}",
                reader.line_count
            ));
        }

        if let Some(offset) = v_rhs {
            let vertex = &mut graph.vertices[i - 1];
            vertex.data[offset..offset + std::mem::size_of::<f64>()]
                .copy_from_slice(&rhs.to_ne_bytes());
        }

        node_flags[i - 1] = true;
        reader.end_of_line().map_err(|e| e.to_string())?;
    }

    // Read arc descriptor lines
    for k in 1..=na {
        if k > 1 {
            reader.read_line().map_err(|e| e.to_string())?;
            reader.read_designator().map_err(|e| e.to_string())?;
        }

        if reader.current_field != "a" {
            return Err(format!(
                "wrong line designator; 'a' expected at line {}",
                reader.line_count
            ));
        }

        reader.read_field().map_err(|e| e.to_string())?;
        let i: usize = reader.current_field.parse().map_err(|_| {
            format!(
                "starting node number missing or invalid at line {}",
                reader.line_count
            )
        })?;

        if i < 1 || i > nv {
            return Err(format!(
                "starting node number {} out of range at line {}",
                i, reader.line_count
            ));
        }

        reader.read_field().map_err(|e| e.to_string())?;
        let j: usize = reader.current_field.parse().map_err(|_| {
            format!(
                "ending node number missing or invalid at line {}",
                reader.line_count
            )
        })?;

        if j < 1 || j > nv {
            return Err(format!(
                "ending node number {} out of range at line {}",
                j, reader.line_count
            ));
        }

        reader.read_field().map_err(|e| e.to_string())?;
        let low: f64 = reader.current_field.parse().map_err(|_| {
            format!(
                "lower bound of arc flow missing or invalid at line {}",
                reader.line_count
            )
        })?;

        if low < 0.0 || low.fract() != 0.0 {
            return Err(format!(
                "invalid lower bound at line {}",
                reader.line_count
            ));
        }

        reader.read_field().map_err(|e| e.to_string())?;
        let cap: f64 = reader.current_field.parse().map_err(|_| {
            format!(
                "upper bound of arc flow missing or invalid at line {}",
                reader.line_count
            )
        })?;

        if cap < low || cap.fract() != 0.0 {
            return Err(format!(
                "invalid upper bound at line {}",
                reader.line_count
            ));
        }

        reader.read_field().map_err(|e| e.to_string())?;
        let cost: f64 = reader.current_field.parse().map_err(|_| {
            format!(
                "per-unit cost of arc flow missing or invalid at line {}",
                reader.line_count
            )
        })?;

        if cost.fract() != 0.0 {
            return Err(format!("non-integer cost at line {}", reader.line_count));
        }

        let arc = graph.add_arc(i, j);
        if let Some(offset) = a_low {
            arc.data[offset..offset + std::mem::size_of::<f64>()]
                .copy_from_slice(&low.to_ne_bytes());
        }
        if let Some(offset) = a_cap {
            arc.data[offset..offset + std::mem::size_of::<f64>()]
                .copy_from_slice(&cap.to_ne_bytes());
        }
        if let Some(offset) = a_cost {
            arc.data[offset..offset + std::mem::size_of::<f64>()]
                .copy_from_slice(&cost.to_ne_bytes());
        }

        reader.end_of_line().map_err(|e| e.to_string())?;
    }

    println!("{} lines were read", reader.line_count);
    Ok(())
}