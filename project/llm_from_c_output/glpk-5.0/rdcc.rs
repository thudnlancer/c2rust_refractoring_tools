use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError, ErrorKind};
use std::path::Path;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug)]
struct Graph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
    v_size: usize,
    a_size: usize,
}

#[derive(Debug)]
struct Vertex {
    data: Vec<u8>,
}

#[derive(Debug)]
struct Edge {
    from: usize,
    to: usize,
}

impl Graph {
    fn new(v_size: usize, a_size: usize) -> Self {
        Graph {
            vertices: Vec::new(),
            edges: Vec::new(),
            v_size,
            a_size,
        }
    }

    fn erase_graph(&mut self) {
        self.vertices.clear();
        self.edges.clear();
    }

    fn add_vertices(&mut self, count: usize) {
        for _ in 0..count {
            self.vertices.push(Vertex {
                data: vec![0; self.v_size],
            });
        }
    }

    fn add_arc(&mut self, from: usize, to: usize) {
        self.edges.push(Edge { from, to });
    }
}

fn glp_read_ccdata(
    graph: &mut Graph,
    v_wgt: i32,
    fname: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if v_wgt >= 0 && v_wgt > graph.v_size as i32 - std::mem::size_of::<f64>() as i32 {
        return Err(Box::new(IoError::new(
            ErrorKind::InvalidInput,
            format!("glp_read_ccdata: v_wgt = {}; invalid offset", v_wgt),
        )));
    }

    graph.erase_graph();

    println!("Reading graph from '{}'...", fname);
    let file = File::open(fname)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines().enumerate();
    let mut line_count = 0;

    // Read problem line
    let (nv, ne) = {
        let line = match lines.next() {
            Some((_, Ok(line))) => line,
            _ => return Err(Box::new(IoError::new(
                ErrorKind::InvalidData,
                "problem line missing",
            ))),
        };
        line_count += 1;

        let mut fields = line.split_whitespace();
        match fields.next() {
            Some("p") => (),
            _ => return Err(Box::new(IoError::new(
                ErrorKind::InvalidData,
                "problem line missing or invalid",
            ))),
        }

        match fields.next() {
            Some("edge") => (),
            _ => return Err(Box::new(IoError::new(
                ErrorKind::InvalidData,
                "wrong problem designator; 'edge' expected",
            ))),
        }

        let nv = fields.next().ok_or_else(|| {
            IoError::new(ErrorKind::InvalidData, "number of vertices missing")
        })?;
        let nv = nv.parse::<usize>().map_err(|_| {
            IoError::new(ErrorKind::InvalidData, "number of vertices invalid")
        })?;

        let ne = fields.next().ok_or_else(|| {
            IoError::new(ErrorKind::InvalidData, "number of edges missing")
        })?;
        let ne = ne.parse::<usize>().map_err(|_| {
            IoError::new(ErrorKind::InvalidData, "number of edges invalid")
        })?;

        if fields.next().is_some() {
            return Err(Box::new(IoError::new(
                ErrorKind::InvalidData,
                "extra data in problem line",
            )));
        }

        println!(
            "Graph has {} vert{} and {} edge{}",
            nv,
            if nv == 1 { "ex" } else { "ices" },
            ne,
            if ne == 1 { "" } else { "s" }
        );

        (nv, ne)
    };

    if nv > 0 {
        graph.add_vertices(nv);
    }

    // Read node descriptor lines
    let mut flag = vec![false; nv + 1]; // 1-based indexing
    if v_wgt >= 0 {
        let w = 1.0f64;
        for i in 1..=nv {
            let bytes = w.to_ne_bytes();
            graph.vertices[i - 1].data[v_wgt as usize..v_wgt as usize + bytes.len()]
                .copy_from_slice(&bytes);
        }
    }

    loop {
        let line = match lines.next() {
            Some((_, Ok(line))) => line,
            _ => break,
        };
        line_count += 1;

        let mut fields = line.split_whitespace();
        match fields.next() {
            Some("n") => (),
            _ => {
                // Not a node descriptor, push back for edge processing
                line_count -= 1;
                break;
            }
        }

        let i = fields.next().ok_or_else(|| {
            IoError::new(ErrorKind::InvalidData, "vertex number missing")
        })?;
        let i = i.parse::<usize>().map_err(|_| {
            IoError::new(ErrorKind::InvalidData, "vertex number invalid")
        })?;

        if !(1 <= i && i <= nv) {
            return Err(Box::new(IoError::new(
                ErrorKind::InvalidData,
                format!("vertex number {} out of range", i),
            )));
        }

        if flag[i] {
            return Err(Box::new(IoError::new(
                ErrorKind::InvalidData,
                format!("duplicate descriptor of vertex {}", i),
            )));
        }

        let w = fields.next().ok_or_else(|| {
            IoError::new(ErrorKind::InvalidData, "vertex weight missing")
        })?;
        let w = w.parse::<f64>().map_err(|_| {
            IoError::new(ErrorKind::InvalidData, "vertex weight invalid")
        })?;

        if w.fract() != 0.0 {
            return Err(Box::new(IoError::new(
                ErrorKind::InvalidData,
                "vertex weight must be integer",
            )));
        }

        if v_wgt >= 0 {
            let bytes = w.to_ne_bytes();
            graph.vertices[i - 1].data[v_wgt as usize..v_wgt as usize + bytes.len()]
                .copy_from_slice(&bytes);
        }

        flag[i] = true;

        if fields.next().is_some() {
            return Err(Box::new(IoError::new(
                ErrorKind::InvalidData,
                "extra data in node descriptor line",
            )));
        }
    }

    // Read edge descriptor lines
    for _ in 0..ne {
        let line = match lines.next() {
            Some((_, Ok(line))) => line,
            _ => return Err(Box::new(IoError::new(
                ErrorKind::InvalidData,
                "not enough edge descriptor lines",
            ))),
        };
        line_count += 1;

        let mut fields = line.split_whitespace();
        match fields.next() {
            Some("e") => (),
            _ => return Err(Box::new(IoError::new(
                ErrorKind::InvalidData,
                "wrong line designator; 'e' expected",
            ))),
        }

        let i = fields.next().ok_or_else(|| {
            IoError::new(ErrorKind::InvalidData, "first vertex number missing")
        })?;
        let i = i.parse::<usize>().map_err(|_| {
            IoError::new(ErrorKind::InvalidData, "first vertex number invalid")
        })?;

        if !(1 <= i && i <= nv) {
            return Err(Box::new(IoError::new(
                ErrorKind::InvalidData,
                format!("first vertex number {} out of range", i),
            )));
        }

        let j = fields.next().ok_or_else(|| {
            IoError::new(ErrorKind::InvalidData, "second vertex number missing")
        })?;
        let j = j.parse::<usize>().map_err(|_| {
            IoError::new(ErrorKind::InvalidData, "second vertex number invalid")
        })?;

        if !(1 <= j && j <= nv) {
            return Err(Box::new(IoError::new(
                ErrorKind::InvalidData,
                format!("second vertex number {} out of range", j),
            )));
        }

        graph.add_arc(i, j);

        if fields.next().is_some() {
            return Err(Box::new(IoError::new(
                ErrorKind::InvalidData,
                "extra data in edge descriptor line",
            )));
        }
    }

    println!("{} lines were read", line_count);
    Ok(())
}

fn glp_read_graph(graph: &mut Graph, fname: &str) -> Result<(), Box<dyn std::error::Error>> {
    glp_read_ccdata(graph, -1, fname)
}