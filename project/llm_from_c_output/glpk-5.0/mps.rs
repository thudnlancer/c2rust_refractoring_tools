use std::{
    ffi::CString,
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::Path,
    str,
};

const GLP_MPS_DECK: i32 = 1;
const GLP_MPS_FILE: i32 = 2;

const GLP_FR: i32 = 1;
const GLP_LO: i32 = 2;
const GLP_UP: i32 = 3;
const GLP_DB: i32 = 4;
const GLP_FX: i32 = 5;

const GLP_CV: i32 = 1;
const GLP_IV: i32 = 2;

#[derive(Debug, Default)]
struct GlpMpscp {
    blank: char,
    obj_name: Option<String>,
    tol_mps: f64,
}

impl GlpMpscp {
    fn init(&mut self) {
        self.blank = '\0';
        self.obj_name = None;
        self.tol_mps = 1e-12;
    }
}

struct GlpProb {
    m: i32,
    n: i32,
    nnz: i32,
    name: Option<String>,
    obj: Option<String>,
    rows: Vec<GlpRow>,
    cols: Vec<GlpCol>,
    c0: f64,
}

struct GlpRow {
    i: i32,
    name: Option<String>,
    type_: i32,
    lb: f64,
    ub: f64,
    ptr: Option<Box<GlpAij>>,
}

struct GlpCol {
    j: i32,
    name: Option<String>,
    kind: i32,
    type_: i32,
    lb: f64,
    ub: f64,
    coef: f64,
    ptr: Option<Box<GlpAij>>,
}

struct GlpAij {
    row: Option<Box<GlpRow>>,
    col: Option<Box<GlpCol>>,
    val: f64,
    r_next: Option<Box<GlpAij>>,
    c_next: Option<Box<GlpAij>>,
}

impl GlpProb {
    fn new() -> Self {
        GlpProb {
            m: 0,
            n: 0,
            nnz: 0,
            name: None,
            obj: None,
            rows: Vec::new(),
            cols: Vec::new(),
            c0: 0.0,
        }
    }

    fn erase(&mut self) {
        *self = GlpProb::new();
    }

    fn set_prob_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }

    fn set_obj_name(&mut self, name: &str) {
        self.obj = Some(name.to_string());
    }

    fn add_rows(&mut self, n: i32) -> i32 {
        let start = self.m + 1;
        self.m += n;
        for i in start..=self.m {
            self.rows.push(GlpRow {
                i,
                name: None,
                type_: GLP_FR,
                lb: 0.0,
                ub: 0.0,
                ptr: None,
            });
        }
        start
    }

    fn add_cols(&mut self, n: i32) -> i32 {
        let start = self.n + 1;
        self.n += n;
        for j in start..=self.n {
            self.cols.push(GlpCol {
                j,
                name: None,
                kind: GLP_CV,
                type_: GLP_LO,
                lb: 0.0,
                ub: 0.0,
                coef: 0.0,
                ptr: None,
            });
        }
        start
    }

    fn set_row_name(&mut self, i: i32, name: &str) {
        if let Some(row) = self.rows.get_mut((i - 1) as usize) {
            row.name = Some(name.to_string());
        }
    }

    fn set_col_name(&mut self, j: i32, name: &str) {
        if let Some(col) = self.cols.get_mut((j - 1) as usize) {
            col.name = Some(name.to_string());
        }
    }

    fn set_row_bnds(&mut self, i: i32, type_: i32, lb: f64, ub: f64) {
        if let Some(row) = self.rows.get_mut((i - 1) as usize) {
            row.type_ = type_;
            row.lb = lb;
            row.ub = ub;
        }
    }

    fn set_col_bnds(&mut self, j: i32, type_: i32, lb: f64, ub: f64) {
        if let Some(col) = self.cols.get_mut((j - 1) as usize) {
            col.type_ = type_;
            col.lb = lb;
            col.ub = ub;
        }
    }

    fn set_col_kind(&mut self, j: i32, kind: i32) {
        if let Some(col) = self.cols.get_mut((j - 1) as usize) {
            col.kind = kind;
        }
    }

    fn set_obj_coef(&mut self, j: i32, coef: f64) {
        if let Some(col) = self.cols.get_mut((j - 1) as usize) {
            col.coef = coef;
        }
    }

    fn find_row(&self, name: &str) -> i32 {
        self.rows
            .iter()
            .find(|r| r.name.as_deref() == Some(name))
            .map(|r| r.i)
            .unwrap_or(0)
    }

    fn find_col(&self, name: &str) -> i32 {
        self.cols
            .iter()
            .find(|c| c.name.as_deref() == Some(name))
            .map(|c| c.j)
            .unwrap_or(0)
    }

    fn get_num_int(&self) -> i32 {
        self.cols.iter().filter(|c| c.kind == GLP_IV).count() as i32
    }

    fn get_num_bin(&self) -> i32 {
        self.cols
            .iter()
            .filter(|c| c.kind == GLP_IV && c.lb == 0.0 && c.ub == 1.0)
            .count() as i32
    }

    fn del_rows(&mut self, n: i32, num: &[i32]) {
        // Simplified implementation - actual implementation would need to handle matrix updates
        for &i in num.iter().take(n as usize) {
            if let Some(idx) = (i - 1) as usize < self.rows.len() {
                self.rows.remove(idx);
            }
        }
        self.m -= n;
    }

    fn sort_matrix(&mut self) {
        // Placeholder - actual implementation would sort the matrix elements
    }
}

fn glp_init_mpscp(parm: &mut GlpMpscp) {
    parm.init();
}

fn check_parm(func: &str, parm: &GlpMpscp) -> Result<(), String> {
    if !(0x00 <= parm.blank as u32 && parm.blank as u32 <= 0xFF)
        || !(parm.blank == '\0' || parm.blank.is_ascii_graphic())
    {
        return Err(format!(
            "{}: blank = 0x{:02X}; invalid parameter",
            func,
            parm.blank as u32
        ));
    }

    if let Some(obj_name) = &parm.obj_name {
        if obj_name.len() > 255 {
            return Err(format!(
                "{}: obj_name = \"{:.12}...\"; parameter too long",
                func,
                obj_name
            ));
        }
    }

    if !(0.0 <= parm.tol_mps && parm.tol_mps < 1.0) {
        return Err(format!(
            "{}: tol_mps = {}; invalid parameter",
            func, parm.tol_mps
        ));
    }

    Ok(())
}

struct Csa<'a> {
    P: &'a mut GlpProb,
    deck: bool,
    parm: &'a GlpMpscp,
    fname: &'a str,
    fp: Option<BufReader<File>>,
    recno: i32,
    recpos: i32,
    c: char,
    fldno: i32,
    field: String,
    w80: i32,
    wef: i32,
    obj_row: i32,
}

impl<'a> Csa<'a> {
    fn new(
        P: &'a mut GlpProb,
        deck: bool,
        parm: &'a GlpMpscp,
        fname: &'a str,
    ) -> Self {
        Csa {
            P,
            deck,
            parm,
            fname,
            fp: None,
            recno: 0,
            recpos: 0,
            c: '\n',
            fldno: 0,
            field: String::new(),
            w80: 0,
            wef: 0,
            obj_row: 0,
        }
    }

    fn error(&self, msg: &str) -> io::Error {
        io::Error::new(io::ErrorKind::InvalidData, msg)
    }

    fn warning(&self, msg: &str) {
        eprintln!("{}:{}: warning: {}", self.fname, self.recno, msg);
    }

    fn read_char(&mut self) -> io::Result<()> {
        if self.c == '\n' {
            self.recno += 1;
            self.recpos = 0;
        }
        self.recpos += 1;

        let mut buf = [0; 1];
        match self.fp.as_mut().unwrap().read_exact(&mut buf) {
            Ok(_) => {
                let c = buf[0] as char;
                if c == '\n' {
                    // Normal case
                } else if self.c == '\r' && c == '\r' {
                    // Handle CR in fixed MPS format
                    return self.read_char();
                } else if c == ' ' || c == '\t' {
                    // Convert whitespace to space
                    self.c = ' ';
                    return Ok(());
                } else if c.is_control() {
                    return Err(self.error(&format!("invalid control character 0x{:02X}", c as u32)));
                }

                if self.deck && self.recpos == 81 && c != '\n' && self.w80 < 1 {
                    self.warning("in fixed MPS format record must not be longer than 80 characters");
                    self.w80 += 1;
                }

                self.c = c;
                Ok(())
            }
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                if self.c == '\n' {
                    Err(self.error("unexpected end of file"))
                } else {
                    self.warning("missing final end of line");
                    self.c = '\n';
                    Ok(())
                }
            }
            Err(e) => Err(e),
        }
    }

    fn indicator(&mut self, name: bool) -> io::Result<bool> {
        self.fldno = 0;
        loop {
            assert_eq!(self.c, '\n');
            self.read_char()?;

            if self.c == ' ' || self.c == '\n' {
                return Ok(false);
            } else if self.c == '*' {
                // Comment record
                while self.c != '\n' {
                    self.read_char()?;
                }
                continue;
            } else {
                // Indicator record
                let mut len = 0;
                self.field.clear();
                while self.c != ' ' && self.c != '\n' && len < 12 {
                    self.field.push(self.c);
                    len += 1;
                    self.read_char()?;
                }

                if !["NAME", "ROWS", "COLUMNS", "RHS", "RANGES", "BOUNDS", "ENDATA"]
                    .contains(&self.field.as_str())
                {
                    return Err(self.error("invalid indicator record"));
                }

                if !name {
                    while self.c != '\n' {
                        self.read_char()?;
                    }
                }
                return Ok(true);
            }
        }
    }

    fn read_field(&mut self) -> io::Result<()> {
        self.fldno += 1;
        self.field.clear();

        if self.deck {
            // Fixed MPS format
            let (beg, end) = match self.fldno {
                1 => (2, 3),
                2 => (5, 12),
                3 => (15, 22),
                4 => (25, 36),
                5 => (40, 47),
                6 => (50, 61),
                _ => panic!("invalid field number"),
            };

            // Skip blanks preceding current field
            if self.c != '\n' {
                let pos = self.recpos;
                while self.recpos < beg {
                    if self.c == ' ' {
                        // OK
                    } else if self.c == '\n' {
                        break;
                    } else {
                        return Err(self.error(&format!(
                            "in fixed MPS format positions {}-{} must be blank",
                            pos, beg - 1
                        )));
                    }
                    self.read_char()?;
                }
            }

            // Skip possible comment in field 3 or 5
            if (self.fldno == 3 || self.fldno == 5) && self.c == '$' {
                while self.c != '\n' {
                    self.read_char()?;
                }
                return Ok(());
            }

            // Read current field
            for pos in beg..=end {
                if self.c == '\n' {
                    break;
                }
                self.field.push(self.c);
                self.read_char()?;
            }

            // Trim field
            self.field = self.field.trim().to_string();

            // Skip blanks following last field
            if self.fldno == 6 && self.c != '\n' {
                while self.recpos <= 72 {
                    if self.c == ' ' {
                        // OK
                    } else if self.c == '\n' {
                        break;
                    } else {
                        return Err(self.error(
                            "in fixed MPS format positions 62-72 must be blank",
                        ));
                    }
                    self.read_char()?;
                }
                while self.c != '\n' {
                    self.read_char()?;
                }
            }
        } else {
            // Free MPS format
            // Skip blanks preceding current field
            while self.c == ' ' {
                self.read_char()?;
            }

            // Skip possible comment
            if self.c == '$' {
                while self.c != '\n' {
                    self.read_char()?;
                }
                return Ok(());
            }

            // Read current field
            let mut len = 0;
            while !(self.c == ' ' || self.c == '\n') {
                if len == 255 {
                    return Err(self.error(&format!(
                        "length of field {} exceeds 255 characters",
                        self.fldno
                    )));
                }
                self.field.push(self.c);
                len += 1;
                self.read_char()?;
            }

            // Skip anything following last field (considered comments)
            if self.fldno == 6 {
                while self.c == ' ' {
                    self.read_char()?;
                }
                if self.c != '$' && self.c != '\n' && self.wef < 1 {
                    self.warning("some extra field(s) detected beyond field 6; field(s) ignored");
                    self.wef += 1;
                }
                while self.c != '\n' {
                    self.read_char()?;
                }
            }
        }

        Ok(())
    }

    fn patch_name(&self, name: &mut String) {
        if self.parm.blank == '\0' {
            *name = name.replace(' ', "");
        } else {
            *name = name.replace(' ', &self.parm.blank.to_string());
        }
    }

    fn read_number(&mut self) -> io::Result<f64> {
        self.read_field()?;
        assert!(self.fldno == 4 || self.fldno == 6);

        if self.field.is_empty() {
            return Err(self.error(&format!(
                "missing numeric value in field {}",
                self.fldno
            )));
        }

        match self.field.trim().parse::<f64>() {
            Ok(x) => Ok(x),
            Err(_) => Err(self.error(&format!(
                "cannot convert '{}' to floating-point number",
                self.field
            ))),
        }
    }

    fn skip_field(&mut self) -> io::Result<()> {
        self.read_field()?;
        if !self.field.is_empty() {
            return Err(self.error(&format!("field {} must be blank", self.fldno)));
        }
        Ok(())
    }
}

fn glp_read_mps(
    P: &mut GlpProb,
    fmt: i32,
    parm: Option<&GlpMpscp>,
    fname: &str,
) -> io::Result<()> {
    let mut _parm = GlpMpscp::default();
    let parm = match parm {
        Some(p) => p,
        None => {
            glp_init_mpscp(&mut _parm);
            &_parm
        }
    };

    check_parm("glp_read_mps", parm).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    let mut csa = Csa::new(P, fmt == GLP_MPS_DECK, parm, fname);

    println!("Reading problem data from '{}'...", fname);

    if !(fmt == GLP_MPS_DECK || fmt == GLP_MPS_FILE) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("glp_read_mps: fmt = {}; invalid parameter", fmt),
        ));
    }

    // Open input file
    csa.fp = Some(BufReader::new(File::open(fname)?));

    // Read NAME