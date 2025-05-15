use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError, ErrorKind};
use std::path::Path;
use std::ptr;

#[derive(Debug)]
enum ReadError {
    Io(IoError),
    Parse(String),
    Validation(String),
}

impl From<IoError> for ReadError {
    fn from(err: IoError) -> Self {
        ReadError::Io(err)
    }
}

#[derive(Debug)]
struct MaxFlowProblem {
    num_nodes: i32,
    num_arcs: i32,
    source: i32,
    sink: i32,
    arcs: Vec<(i32, i32, f64)>,
}

fn read_maxflow(
    filename: &str,
) -> Result<MaxFlowProblem, ReadError> {
    let file = File::open(Path::new(filename))?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // Read problem line
    let problem_line = lines.next().ok_or(ReadError::Parse("Empty file".to_string()))??;
    let mut parts = problem_line.split_whitespace();
    if parts.next() != Some("p") {
        return Err(ReadError::Parse("Missing problem line".to_string()));
    }
    if parts.next() != Some("max") {
        return Err(ReadError::Parse("Expected 'max' problem type".to_string()));
    }

    let num_nodes = parts.next()
        .ok_or(ReadError::Parse("Missing node count".to_string()))?
        .parse::<i32>()
        .map_err(|_| ReadError::Parse("Invalid node count".to_string()))?;
    
    let num_arcs = parts.next()
        .ok_or(ReadError::Parse("Missing arc count".to_string()))?
        .parse::<i32>()
        .map_err(|_| ReadError::Parse("Invalid arc count".to_string()))?;

    if num_nodes < 2 {
        return Err(ReadError::Validation("At least 2 nodes required".to_string()));
    }
    if num_arcs < 0 {
        return Err(ReadError::Validation("Negative arc count".to_string()));
    }

    let mut source = None;
    let mut sink = None;
    let mut arcs = Vec::with_capacity(num_arcs as usize);

    // Read node descriptors
    for line in &mut lines {
        let line = line?;
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("n") => {
                let node = parts.next()
                    .ok_or(ReadError::Parse("Missing node number".to_string()))?
                    .parse::<i32>()
                    .map_err(|_| ReadError::Parse("Invalid node number".to_string()))?;
                
                if node < 1 || node > num_nodes {
                    return Err(ReadError::Validation(format!("Node {} out of range", node)));
                }

                match parts.next() {
                    Some("s") => {
                        if source.is_some() {
                            return Err(ReadError::Validation("Multiple source nodes".to_string()));
                        }
                        source = Some(node);
                    }
                    Some("t") => {
                        if sink.is_some() {
                            return Err(ReadError::Validation("Multiple sink nodes".to_string()));
                        }
                        sink = Some(node);
                    }
                    _ => return Err(ReadError::Parse("Expected 's' or 't'".to_string())),
                }
            }
            Some("a") => break, // Start of arcs section
            _ => continue, // Skip comments or empty lines
        }
    }

    let source = source.ok_or(ReadError::Validation("Missing source node".to_string()))?;
    let sink = sink.ok_or(ReadError::Validation("Missing sink node".to_string()))?;
    if source == sink {
        return Err(ReadError::Validation("Source and sink must be distinct".to_string()));
    }

    // Read arcs
    for line in lines {
        let line = line?;
        let mut parts = line.split_whitespace();
        if parts.next() != Some("a") {
            continue; // Skip non-arc lines
        }

        let from = parts.next()
            .ok_or(ReadError::Parse("Missing from node".to_string()))?
            .parse::<i32>()
            .map_err(|_| ReadError::Parse("Invalid from node".to_string()))?;
        
        let to = parts.next()
            .ok_or(ReadError::Parse("Missing to node".to_string()))?
            .parse::<i32>()
            .map_err(|_| ReadError::Parse("Invalid to node".to_string()))?;
        
        let capacity = parts.next()
            .ok_or(ReadError::Parse("Missing capacity".to_string()))?
            .parse::<f64>()
            .map_err(|_| ReadError::Parse("Invalid capacity".to_string()))?;

        if from < 1 || from > num_nodes {
            return Err(ReadError::Validation(format!("From node {} out of range", from)));
        }
        if to < 1 || to > num_nodes {
            return Err(ReadError::Validation(format!("To node {} out of range", to)));
        }
        if capacity < 0.0 {
            return Err(ReadError::Validation("Negative capacity".to_string()));
        }

        arcs.push((from, to, capacity));
    }

    if arcs.len() != num_arcs as usize {
        return Err(ReadError::Validation(format!(
            "Expected {} arcs, found {}", num_arcs, arcs.len()
        )));
    }

    Ok(MaxFlowProblem {
        num_nodes,
        num_arcs,
        source,
        sink,
        arcs,
    })
}

fn glp_read_maxflow_safe(
    filename: &str,
) -> Result<(), String> {
    match read_maxflow(filename) {
        Ok(problem) => {
            println!("Successfully read maxflow problem:");
            println!("Nodes: {}", problem.num_nodes);
            println!("Arcs: {}", problem.num_arcs);
            println!("Source: {}", problem.source);
            println!("Sink: {}", problem.sink);
            Ok(())
        }
        Err(e) => match e {
            ReadError::Io(err) => Err(format!("IO error: {}", err)),
            ReadError::Parse(msg) => Err(format!("Parse error: {}", msg)),
            ReadError::Validation(msg) => Err(format!("Validation error: {}", msg)),
        },
    }
}