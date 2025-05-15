use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError, ErrorKind};
use std::path::Path;
use std::collections::HashSet;

struct DimacsReader {
    fname: String,
    reader: BufReader<File>,
    line_count: usize,
    current_line: String,
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
            current_pos: 0,
        })
    }

    fn read_line(&mut self) -> Result<(), IoError> {
        self.current_line.clear();
        self.current_pos = 0;
        let bytes_read = self.reader.read_line(&mut self.current_line)?;
        if bytes_read == 0 {
            return Err(IoError::new(ErrorKind::UnexpectedEof, "Unexpected EOF"));
        }
        self.line_count += 1;
        Ok(())
    }

    fn skip_whitespace(&mut self) {
        while self.current_pos < self.current_line.len() {
            let c = self.current_line.as_bytes()[self.current_pos];
            if c != b' ' && c != b'\t' && c != b'\n' {
                break;
            }
            self.current_pos += 1;
        }
    }

    fn read_field(&mut self) -> Result<String, IoError> {
        self.skip_whitespace();
        if self.current_pos >= self.current_line.len() {
            return Err(IoError::new(
                ErrorKind::InvalidInput,
                "Unexpected end of line",
            ));
        }

        let start = self.current_pos;
        while self.current_pos < self.current_line.len() {
            let c = self.current_line.as_bytes()[self.current_pos];
            if c == b' ' || c == b'\t' || c == b'\n' {
                break;
            }
            self.current_pos += 1;
        }

        Ok(self.current_line[start..self.current_pos].to_string())
    }

    fn end_of_line(&mut self) -> Result<(), IoError> {
        self.skip_whitespace();
        if self.current_pos < self.current_line.len() {
            return Err(IoError::new(
                ErrorKind::InvalidInput,
                "Extra characters at end of line",
            ));
        }
        Ok(())
    }
}

#[derive(Debug)]
struct CnfSatProblem {
    num_vars: usize,
    num_clauses: usize,
    clauses: Vec<Vec<(usize, bool)>>,
}

fn read_cnfsat<P: AsRef<Path>>(path: P) -> Result<CnfSatProblem, IoError> {
    let mut reader = DimacsReader::new(path.as_ref().to_str().unwrap())?;
    let mut num_vars = 0;
    let mut num_clauses = 0;
    let mut clauses = Vec::new();

    // Read problem line
    reader.read_line()?;
    let field = reader.read_field()?;
    if field != "p" {
        return Err(IoError::new(
            ErrorKind::InvalidInput,
            "Problem line missing or invalid",
        ));
    }

    let field = reader.read_field()?;
    if field != "cnf" {
        return Err(IoError::new(
            ErrorKind::InvalidInput,
            "Wrong problem designator; 'cnf' expected",
        ));
    }

    let field = reader.read_field()?;
    num_vars = field.parse().map_err(|_| {
        IoError::new(
            ErrorKind::InvalidInput,
            "Number of variables missing or invalid",
        )
    })?;

    let field = reader.read_field()?;
    num_clauses = field.parse().map_err(|_| {
        IoError::new(
            ErrorKind::InvalidInput,
            "Number of clauses missing or invalid",
        )
    })?;

    reader.end_of_line()?;

    println!(
        "Instance has {} variable{} and {} clause{}",
        num_vars,
        if num_vars == 1 { "" } else { "s" },
        num_clauses,
        if num_clauses == 1 { "" } else { "s" }
    );

    // Read clauses
    for _ in 0..num_clauses {
        reader.read_line()?;
        let mut clause = Vec::new();
        let mut used_vars = HashSet::new();
        let mut rhs = 1;

        loop {
            reader.skip_whitespace();
            let field = reader.read_field()?;
            let j: i32 = field.parse().map_err(|_| {
                IoError::new(
                    ErrorKind::InvalidInput,
                    "Variable number missing or invalid",
                )
            })?;

            if j == 0 {
                break;
            }

            let (var, neg) = if j > 0 {
                (j as usize, false)
            } else {
                rhs -= 1;
                ((-j) as usize, true)
            };

            if var < 1 || var > num_vars {
                return Err(IoError::new(
                    ErrorKind::InvalidInput,
                    "Variable number out of range",
                ));
            }

            if used_vars.contains(&var) {
                return Err(IoError::new(
                    ErrorKind::InvalidInput,
                    "Duplicate variable number",
                ));
            }

            clause.push((var, neg));
            used_vars.insert(var);
        }

        clauses.push(clause);
        reader.end_of_line()?;
    }

    println!("{} lines were read", reader.line_count);
    Ok(CnfSatProblem {
        num_vars,
        num_clauses,
        clauses,
    })
}