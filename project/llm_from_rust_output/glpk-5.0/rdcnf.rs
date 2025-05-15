use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;
use std::ptr;

struct DMX {
    fname: String,
    reader: BufReader<File>,
    count: i32,
    current_char: Option<char>,
    field: String,
    empty: bool,
    nonint: bool,
}

impl DMX {
    fn new(fname: &str) -> Result<Self, Error> {
        let file = File::open(fname)?;
        let reader = BufReader::new(file);
        Ok(DMX {
            fname: fname.to_string(),
            reader,
            count: 0,
            current_char: None,
            field: String::new(),
            empty: true,
            nonint: false,
        })
    }

    fn read_char(&mut self) -> Result<(), Error> {
        let mut buf = [0u8; 1];
        if self.reader.read_exact(&mut buf).is_err() {
            self.current_char = None;
            return Ok(());
        }
        self.current_char = Some(buf[0] as char);
        Ok(())
    }

    fn read_field(&mut self) -> Result<(), Error> {
        self.field.clear();
        while let Some(c) = self.current_char {
            if c == ' ' || c == '\n' {
                break;
            }
            self.field.push(c);
            self.read_char()?;
        }
        Ok(())
    }

    fn read_designator(&mut self) -> Result<(), Error> {
        self.read_field()
    }

    fn end_of_line(&mut self) -> Result<(), Error> {
        while let Some(c) = self.current_char {
            if c == '\n' {
                self.read_char()?;
                break;
            }
            self.read_char()?;
        }
        Ok(())
    }

    fn error(&self, msg: &str) -> Error {
        Error::new(ErrorKind::InvalidData, msg)
    }
}

struct GlpProb {
    // Simplified representation
    rows: Vec<GlpRow>,
    cols: Vec<GlpCol>,
}

struct GlpRow {
    bounds: (f64, f64),
    coefficients: Vec<(i32, f64)>,
}

struct GlpCol {
    kind: i32,
}

impl GlpProb {
    fn new() -> Self {
        GlpProb {
            rows: Vec::new(),
            cols: Vec::new(),
        }
    }

    fn erase(&mut self) {
        self.rows.clear();
        self.cols.clear();
    }

    fn add_rows(&mut self, n: i32) {
        self.rows.reserve(n as usize);
        for _ in 0..n {
            self.rows.push(GlpRow {
                bounds: (0.0, 0.0),
                coefficients: Vec::new(),
            });
        }
    }

    fn add_cols(&mut self, n: i32) {
        self.cols.reserve(n as usize);
        for _ in 0..n {
            self.cols.push(GlpCol { kind: 0 });
        }
    }

    fn set_row_bnds(&mut self, i: i32, lb: f64, ub: f64) {
        if let Some(row) = self.rows.get_mut(i as usize - 1) {
            row.bounds = (lb, ub);
        }
    }

    fn set_col_kind(&mut self, j: i32, kind: i32) {
        if let Some(col) = self.cols.get_mut(j as usize - 1) {
            col.kind = kind;
        }
    }

    fn set_mat_row(&mut self, i: i32, coefficients: Vec<(i32, f64)>) {
        if let Some(row) = self.rows.get_mut(i as usize - 1) {
            row.coefficients = coefficients;
        }
    }

    fn sort_matrix(&mut self) {
        for row in &mut self.rows {
            row.coefficients.sort_by_key(|&(j, _)| j);
        }
    }
}

pub fn glp_read_cnfsat(fname: &str) -> Result<GlpProb, Error> {
    let mut prob = GlpProb::new();
    let mut dmx = DMX::new(fname)?;

    // Initial read
    dmx.read_char()?;

    // Read problem line
    dmx.read_designator()?;
    if dmx.field != "p" {
        return Err(dmx.error("problem line missing or invalid"));
    }

    dmx.read_field()?;
    if dmx.field != "cnf" {
        return Err(dmx.error("wrong problem designator; 'cnf' expected"));
    }

    dmx.read_field()?;
    let n: i32 = dmx.field.parse().map_err(|_| dmx.error("number of variables missing or invalid"))?;
    if n < 0 {
        return Err(dmx.error("number of variables missing or invalid"));
    }

    dmx.read_field()?;
    let m: i32 = dmx.field.parse().map_err(|_| dmx.error("number of clauses missing or invalid"))?;
    if m < 0 {
        return Err(dmx.error("number of clauses missing or invalid"));
    }

    dmx.end_of_line()?;

    if m > 0 {
        prob.add_rows(m);
    }
    if n > 0 {
        prob.add_cols(n);
        for j in 1..=n {
            prob.set_col_kind(j, 3);
        }
    }

    let mut map = vec![false; (n + 1) as usize];

    for i in 1..=m {
        let mut coefficients = Vec::new();
        let mut rhs = 1;

        loop {
            while let Some(c) = dmx.current_char {
                if c != ' ' && c != '\n' {
                    break;
                }
                dmx.read_char()?;
            }

            dmx.read_field()?;
            if dmx.field.is_empty() {
                break;
            }

            let mut j: i32 = dmx.field.parse().map_err(|_| dmx.error("variable number missing or invalid"))?;
            let neg = if j > 0 { false } else {
                if j == 0 {
                    break;
                }
                j = -j;
                rhs -= 1;
                true
            };

            if j < 1 || j > n {
                return Err(dmx.error("variable number out of range"));
            }

            if map[j as usize] {
                return Err(dmx.error("duplicate variable number"));
            }

            coefficients.push((j, if neg { -1.0 } else { 1.0 }));
            map[j as usize] = true;
        }

        prob.set_row_bnds(i, rhs as f64, 0.0);
        prob.set_mat_row(i, coefficients);

        for &(j, _) in &prob.rows[i as usize - 1].coefficients {
            map[j as usize] = false;
        }
    }

    prob.sort_matrix();
    Ok(prob)
}