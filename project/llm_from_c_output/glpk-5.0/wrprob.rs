use std::fs::File;
use std::io::{Write, Error, ErrorKind};
use std::path::Path;

struct GlpProb {
    dir: GlpDir,
    m: i32,
    n: i32,
    nnz: i32,
    name: Option<String>,
    obj: Option<String>,
    c0: f64,
    row: Vec<GLPRow>,
    col: Vec<GLPCol>,
}

enum GlpDir {
    Min,
    Max,
    Unknown,
}

struct GLPRow {
    type_: GlpType,
    lb: f64,
    ub: f64,
    name: Option<String>,
    ptr: Option<Box<GLPAIJ>>,
}

struct GLPCol {
    type_: GlpType,
    kind: GlpKind,
    lb: f64,
    ub: f64,
    name: Option<String>,
    coef: f64,
    j: i32,
}

struct GLPAIJ {
    col: Box<GLPCol>,
    val: f64,
    r_next: Option<Box<GLPAIJ>>,
}

enum GlpType {
    FR,
    LO,
    UP,
    DB,
    FX,
}

enum GlpKind {
    CV,
    IV,
}

fn glp_write_prob(p: &GlpProb, flags: i32, fname: &str) -> Result<(), Error> {
    if flags != 0 {
        return Err(Error::new(ErrorKind::InvalidInput, "glp_write_prob: flags != 0; invalid parameter"));
    }

    println!("Writing problem data to '{}'...", fname);
    let path = Path::new(fname);
    let mut file = match File::create(&path) {
        Ok(f) => f,
        Err(e) => {
            println!("Unable to create '{}' - {}", fname, e);
            return Err(e);
        }
    };

    let mut count = 0;
    let mip = glp_get_num_int(p) > 0;

    // write problem line
    let prob_line = format!("p {} {} {} {} {}\n",
        if !mip { "lp" } else { "mip" },
        match p.dir {
            GlpDir::Min => "min",
            GlpDir::Max => "max",
            GlpDir::Unknown => "???",
        },
        p.m, p.n, p.nnz);
    file.write_all(prob_line.as_bytes())?;
    count += 1;

    if let Some(name) = &p.name {
        let name_line = format!("n p {}\n", name);
        file.write_all(name_line.as_bytes())?;
        count += 1;
    }

    if let Some(obj) = &p.obj {
        let obj_line = format!("n z {}\n", obj);
        file.write_all(obj_line.as_bytes())?;
        count += 1;
    }

    // write row descriptors
    for (i, row) in p.row.iter().enumerate() {
        let i = i + 1;
        if !(row.type_ == GlpType::FX && row.lb == 0.0) {
            let mut row_line = format!("i {} ", i);
            match row.type_ {
                GlpType::FR => row_line.push_str("f\n"),
                GlpType::LO => row_line.push_str(&format!("l {:.15}\n", row.lb)),
                GlpType::UP => row_line.push_str(&format!("u {:.15}\n", row.ub)),
                GlpType::DB => row_line.push_str(&format!("d {:.15} {:.15}\n", row.lb, row.ub)),
                GlpType::FX => row_line.push_str(&format!("s {:.15}\n", row.lb)),
            }
            file.write_all(row_line.as_bytes())?;
            count += 1;
        }

        if let Some(name) = &row.name {
            let name_line = format!("n i {} {}\n", i, name);
            file.write_all(name_line.as_bytes())?;
            count += 1;
        }
    }

    // write column descriptors
    for (j, col) in p.col.iter().enumerate() {
        let j = j + 1;
        if !(!mip && col.type_ == GlpType::LO && col.lb == 0.0) &&
           !(mip && col.kind == GlpKind::IV && col.type_ == GlpType::DB &&
             col.lb == 0.0 && col.ub == 1.0) {
            let mut col_line = format!("j {} ", j);
            if mip {
                match col.kind {
                    GlpKind::CV => col_line.push_str("c "),
                    GlpKind::IV => col_line.push_str("i "),
                }
            }
            match col.type_ {
                GlpType::FR => col_line.push_str("f\n"),
                GlpType::LO => col_line.push_str(&format!("l {:.15}\n", col.lb)),
                GlpType::UP => col_line.push_str(&format!("u {:.15}\n", col.ub)),
                GlpType::DB => col_line.push_str(&format!("d {:.15} {:.15}\n", col.lb, col.ub)),
                GlpType::FX => col_line.push_str(&format!("s {:.15}\n", col.lb)),
            }
            file.write_all(col_line.as_bytes())?;
            count += 1;
        }

        if let Some(name) = &col.name {
            let name_line = format!("n j {} {}\n", j, name);
            file.write_all(name_line.as_bytes())?;
            count += 1;
        }
    }

    // write objective coefficient descriptors
    if p.c0 != 0.0 {
        let c0_line = format!("a 0 0 {:.15}\n", p.c0);
        file.write_all(c0_line.as_bytes())?;
        count += 1;
    }

    for (j, col) in p.col.iter().enumerate() {
        let j = j + 1;
        if col.coef != 0.0 {
            let coef_line = format!("a 0 {} {:.15}\n", j, col.coef);
            file.write_all(coef_line.as_bytes())?;
            count += 1;
        }
    }

    // write constraint coefficient descriptors
    for (i, row) in p.row.iter().enumerate() {
        let i = i + 1;
        let mut aij = &row.ptr;
        while let Some(a) = aij {
            let aij_line = format!("a {} {} {:.15}\n", i, a.col.j, a.val);
            file.write_all(aij_line.as_bytes())?;
            count += 1;
            aij = &a.r_next;
        }
    }

    // write end line
    file.write_all(b"e o f\n")?;
    count += 1;

    println!("{} lines were written", count);
    Ok(())
}

fn glp_get_num_int(p: &GlpProb) -> i32 {
    p.col.iter().filter(|c| matches!(c.kind, GlpKind::IV)).count() as i32
}