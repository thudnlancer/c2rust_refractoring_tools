use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError, ErrorKind};
use std::path::Path;
use std::str::FromStr;

struct Dmx {
    fname: String,
    reader: BufReader<File>,
    count: usize,
    current_line: String,
    fields: Vec<String>,
    current_field: usize,
}

impl Dmx {
    fn new(fname: &str) -> Result<Self, IoError> {
        let file = File::open(fname)?;
        let reader = BufReader::new(file);
        Ok(Self {
            fname: fname.to_string(),
            reader,
            count: 0,
            current_line: String::new(),
            fields: Vec::new(),
            current_field: 0,
        })
    }

    fn read_line(&mut self) -> Result<(), IoError> {
        self.current_line.clear();
        let bytes_read = self.reader.read_line(&mut self.current_line)?;
        if bytes_read == 0 {
            return Err(IoError::new(ErrorKind::UnexpectedEof, "End of file"));
        }
        self.count += 1;
        self.fields = self.current_line.trim().split_whitespace()
            .map(|s| s.to_string())
            .collect();
        self.current_field = 0;
        Ok(())
    }

    fn read_field(&mut self) -> Result<&str, IoError> {
        if self.current_field >= self.fields.len() {
            self.read_line()?;
        }
        let field = &self.fields[self.current_field];
        self.current_field += 1;
        Ok(field)
    }

    fn read_designator(&mut self) -> Result<&str, IoError> {
        self.read_field()
    }

    fn end_of_line(&mut self) -> Result<(), IoError> {
        if self.current_field < self.fields.len() {
            Err(IoError::new(ErrorKind::InvalidData, "Extra fields in line"))
        } else {
            Ok(())
        }
    }
}

fn str2int(s: &str) -> Result<i32, String> {
    i32::from_str(s).map_err(|_| format!("Invalid integer: {}", s))
}

fn str2num(s: &str) -> Result<f64, String> {
    f64::from_str(s).map_err(|_| format!("Invalid number: {}", s))
}

struct GlpProb {
    m: i32,
    n: i32,
    ipt_stat: i32,
    ipt_obj: f64,
    row: Vec<GlpRowCol>,
    col: Vec<GlpRowCol>,
}

struct GlpRowCol {
    pval: f64,
    dval: f64,
}

impl GlpProb {
    fn new(m: i32, n: i32) -> Self {
        Self {
            m,
            n,
            ipt_stat: 0,
            ipt_obj: 0.0,
            row: vec![GlpRowCol { pval: 0.0, dval: 0.0 }; m as usize + 1],
            col: vec![GlpRowCol { pval: 0.0, dval: 0.0 }; n as usize + 1],
        }
    }
}

const GLP_OPT: i32 = 1;
const GLP_INFEAS: i32 = 2;
const GLP_NOFEAS: i32 = 3;
const GLP_UNDEF: i32 = 4;
const GLP_BS: char = 'B';

fn glp_read_ipt(p: &mut GlpProb, fname: &str) -> Result<(), String> {
    let mut dmx = Dmx::new(fname).map_err(|e| format!("Unable to open '{}' - {}", fname, e))?;

    println!("Reading interior-point solution from '{}'...", fname);

    // Read solution line
    let designator = dmx.read_designator()
        .map_err(|e| format!("Error reading designator: {}", e))?;
    if designator != "s" {
        return Err("solution line missing or invalid".to_string());
    }

    let sol_type = dmx.read_field()
        .map_err(|e| format!("Error reading solution type: {}", e))?;
    if sol_type != "ipt" {
        return Err("wrong solution designator; 'ipt' expected".to_string());
    }

    let m = dmx.read_field()
        .and_then(|f| str2int(f))
        .map_err(|e| format!("number of rows missing or invalid: {}", e))?;
    if m != p.m {
        return Err("number of rows mismatch".to_string());
    }

    let n = dmx.read_field()
        .and_then(|f| str2int(f))
        .map_err(|e| format!("number of columns missing or invalid: {}", e))?;
    if n != p.n {
        return Err("number of columns mismatch".to_string());
    }

    let sst = match dmx.read_field()
        .map_err(|e| format!("Error reading solution status: {}", e))?
    {
        "o" => GLP_OPT,
        "i" => GLP_INFEAS,
        "n" => GLP_NOFEAS,
        "u" => GLP_UNDEF,
        _ => return Err("solution status missing or invalid".to_string()),
    };

    let obj = dmx.read_field()
        .and_then(|f| str2num(f))
        .map_err(|e| format!("objective value missing or invalid: {}", e))?;

    dmx.end_of_line()
        .map_err(|e| format!("Error at end of solution line: {}", e))?;

    // Allocate working arrays
    let total = (m + n) as usize;
    let mut stat = vec!['?'; total + 1];
    let mut prim = vec![0.0; total + 1];
    let mut dual = vec![0.0; total + 1];

    // Read solution descriptor lines
    loop {
        let desig = dmx.read_designator()
            .map_err(|e| format!("Error reading designator: {}", e))?;

        match desig {
            "i" => {
                // Row solution descriptor
                let i = dmx.read_field()
                    .and_then(|f| str2int(f))
                    .map_err(|e| format!("row number missing or invalid: {}", e))?;
                if !(1 <= i && i <= m) {
                    return Err("row number out of range".to_string());
                }
                if stat[i as usize] != '?' {
                    return Err("duplicate row solution descriptor".to_string());
                }
                stat[i as usize] = GLP_BS;

                prim[i as usize] = dmx.read_field()
                    .and_then(|f| str2num(f))
                    .map_err(|e| format!("row primal value missing or invalid: {}", e))?;

                dual[i as usize] = dmx.read_field()
                    .and_then(|f| str2num(f))
                    .map_err(|e| format!("row dual value missing or invalid: {}", e))?;

                dmx.end_of_line()
                    .map_err(|e| format!("Error at end of row descriptor: {}", e))?;
            }
            "j" => {
                // Column solution descriptor
                let j = dmx.read_field()
                    .and_then(|f| str2int(f))
                    .map_err(|e| format!("column number missing or invalid: {}", e))?;
                if !(1 <= j && j <= n) {
                    return Err("column number out of range".to_string());
                }
                if stat[(m + j) as usize] != '?' {
                    return Err("duplicate column solution descriptor".to_string());
                }
                stat[(m + j) as usize] = GLP_BS;

                prim[(m + j) as usize] = dmx.read_field()
                    .and_then(|f| str2num(f))
                    .map_err(|e| format!("column primal value missing or invalid: {}", e))?;

                dual[(m + j) as usize] = dmx.read_field()
                    .and_then(|f| str2num(f))
                    .map_err(|e| format!("column dual value missing or invalid: {}", e))?;

                dmx.end_of_line()
                    .map_err(|e| format!("Error at end of column descriptor: {}", e))?;
            }
            "e" => break,
            _ => return Err("line designator missing or invalid".to_string()),
        }
    }

    // Verify all solution components are present
    for k in 1..=total {
        if stat[k] == '?' {
            return Err("incomplete interior-point solution".to_string());
        }
    }

    // Store solution components into problem object
    p.ipt_stat = sst;
    p.ipt_obj = obj;
    for i in 1..=m as usize {
        p.row[i].pval = prim[i];
        p.row[i].dval = dual[i];
    }
    for j in 1..=n as usize {
        p.col[j].pval = prim[(m as usize) + j];
        p.col[j].dval = dual[(m as usize) + j];
    }

    println!("{} lines were read", dmx.count);
    Ok(())
}