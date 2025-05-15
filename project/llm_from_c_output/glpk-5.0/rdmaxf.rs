use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError, ErrorKind as IoErrorKind};
use std::path::Path;
use std::str::FromStr;
use std::fmt;

#[derive(Debug)]
struct DimacsError {
    message: String,
    line: usize,
}

impl fmt::Display for DimacsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error at line {}: {}", self.line, self.message)
    }
}

impl std::error::Error for DimacsError {}

struct DimacsParser {
    reader: BufReader<File>,
    current_line: String,
    line_count: usize,
    current_field: String,
}

impl DimacsParser {
    fn new(file: File) -> Self {
        DimacsParser {
            reader: BufReader::new(file),
            current_line: String::new(),
            line_count: 0,
            current_field: String::new(),
        }
    }

    fn read_line(&mut self) -> Result<(), DimacsError> {
        self.current_line.clear();
        self.line_count += 1;
        match self.reader.read_line(&mut self.current_line) {
            Ok(0) => Err(DimacsError {
                message: "Unexpected end of file".to_string(),
                line: self.line_count,
            }),
            Ok(_) => Ok(()),
            Err(e) => Err(DimacsError {
                message: format!("I/O error: {}", e),
                line: self.line_count,
            }),
        }
    }

    fn read_designator(&mut self) -> Result<(), DimacsError> {
        loop {
            self.read_line()?;
            let line = self.current_line.trim();
            if line.is_empty() || line.starts_with('c') {
                continue;
            }
            let mut fields = line.split_whitespace();
            self.current_field = match fields.next() {
                Some(field) => field.to_string(),
                None => continue,
            };
            return Ok(());
        }
    }

    fn read_field(&mut self) -> Result<(), DimacsError> {
        let line = self.current_line.trim();
        let mut fields = line.split_whitespace();
        let mut count = 0;
        while let Some(field) = fields.next() {
            if count > 0 {
                self.current_field = field.to_string();
                return Ok(());
            }
            count += 1;
        }
        Err(DimacsError {
            message: "Missing field".to_string(),
            line: self.line_count,
        })
    }

    fn end_of_line(&mut self) -> Result<(), DimacsError> {
        let line = self.current_line.trim();
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() > 0 && !fields[0].is_empty() {
            Ok(())
        } else {
            Err(DimacsError {
                message: "Expected end of line".to_string(),
                line: self.line_count,
            })
        }
    }

    fn parse_int<T: FromStr>(&self) -> Result<T, DimacsError> {
        self.current_field.parse().map_err(|_| DimacsError {
            message: format!("Invalid integer: {}", self.current_field),
            line: self.line_count,
        })
    }

    fn parse_float<T: FromStr>(&self) -> Result<T, DimacsError> {
        self.current_field.parse().map_err(|_| DimacsError {
            message: format!("Invalid float: {}", self.current_field),
            line: self.line_count,
        })
    }
}

struct Graph {
    vertices: Vec<()>,
    arcs: Vec<(usize, usize, f64)>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            vertices: Vec::new(),
            arcs: Vec::new(),
        }
    }

    fn add_vertices(&mut self, count: usize) {
        self.vertices.resize(count, ());
    }

    fn add_arc(&mut self, from: usize, to: usize, capacity: f64) {
        self.arcs.push((from, to, capacity));
    }
}

fn read_maxflow(
    graph: &mut Graph,
    source: &mut Option<usize>,
    sink: &mut Option<usize>,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filename).map_err(|e| {
        IoError::new(
            IoErrorKind::Other,
            format!("Unable to open '{}' - {}", filename, e),
        )
    })?;

    let mut parser = DimacsParser::new(file);
    println!("Reading maximum flow problem data from '{}'...", filename);

    // Read problem line
    parser.read_designator()?;
    if parser.current_field != "p" {
        return Err(Box::new(DimacsError {
            message: "problem line missing or invalid".to_string(),
            line: parser.line_count,
        }));
    }

    parser.read_field()?;
    if parser.current_field != "max" {
        return Err(Box::new(DimacsError {
            message: "wrong problem designator; 'max' expected".to_string(),
            line: parser.line_count,
        }));
    }

    parser.read_field()?;
    let nv: usize = parser.parse_int()?;
    if nv < 2 {
        return Err(Box::new(DimacsError {
            message: "number of nodes must be at least 2".to_string(),
            line: parser.line_count,
        }));
    }

    parser.read_field()?;
    let na: usize = parser.parse_int()?;
    println!(
        "Flow network has {} node{} and {} arc{}",
        nv,
        if nv == 1 { "" } else { "s" },
        na,
        if na == 1 { "" } else { "s" }
    );

    graph.add_vertices(nv);
    parser.end_of_line()?;

    // Read node descriptor lines
    let mut s = None;
    let mut t = None;

    loop {
        if let Err(_) = parser.read_designator() {
            break;
        }
        if parser.current_field != "n" {
            break;
        }

        parser.read_field()?;
        let i: usize = parser.parse_int()?;
        if !(1..=nv).contains(&i) {
            return Err(Box::new(DimacsError {
                message: format!("node number {} out of range", i),
                line: parser.line_count,
            }));
        }

        parser.read_field()?;
        match parser.current_field.as_str() {
            "s" => {
                if s.is_some() {
                    return Err(Box::new(DimacsError {
                        message: "only one source node allowed".to_string(),
                        line: parser.line_count,
                    }));
                }
                s = Some(i);
            }
            "t" => {
                if t.is_some() {
                    return Err(Box::new(DimacsError {
                        message: "only one sink node allowed".to_string(),
                        line: parser.line_count,
                    }));
                }
                t = Some(i);
            }
            _ => {
                return Err(Box::new(DimacsError {
                    message: "wrong node designator; 's' or 't' expected".to_string(),
                    line: parser.line_count,
                }));
            }
        }

        if s.is_some() && t.is_some() && s == t {
            return Err(Box::new(DimacsError {
                message: "source and sink nodes must be distinct".to_string(),
                line: parser.line_count,
            }));
        }

        parser.end_of_line()?;
    }

    let s = s.ok_or_else(|| {
        Box::new(DimacsError {
            message: "source node descriptor missing".to_string(),
            line: parser.line_count,
        })
    })?;

    let t = t.ok_or_else(|| {
        Box::new(DimacsError {
            message: "sink node descriptor missing".to_string(),
            line: parser.line_count,
        })
    })?;

    *source = Some(s);
    *sink = Some(t);

    // Read arc descriptor lines
    for _ in 1..=na {
        parser.read_designator()?;
        if parser.current_field != "a" {
            return Err(Box::new(DimacsError {
                message: "wrong line designator; 'a' expected".to_string(),
                line: parser.line_count,
            }));
        }

        parser.read_field()?;
        let i: usize = parser.parse_int()?;
        if !(1..=nv).contains(&i) {
            return Err(Box::new(DimacsError {
                message: format!("starting node number {} out of range", i),
                line: parser.line_count,
            }));
        }

        parser.read_field()?;
        let j: usize = parser.parse_int()?;
        if !(1..=nv).contains(&j) {
            return Err(Box::new(DimacsError {
                message: format!("ending node number {} out of range", j),
                line: parser.line_count,
            }));
        }

        parser.read_field()?;
        let cap: f64 = parser.parse_float()?;
        if cap < 0.0 {
            return Err(Box::new(DimacsError {
                message: "arc capacity must be non-negative".to_string(),
                line: parser.line_count,
            }));
        }

        graph.add_arc(i, j, cap);
        parser.end_of_line()?;
    }

    println!("{} lines were read", parser.line_count);
    Ok(())
}