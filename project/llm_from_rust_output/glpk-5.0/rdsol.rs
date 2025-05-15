use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ptr;
use std::str::FromStr;

#[derive(Debug)]
enum SolutionStatus {
    Undefined,
    Basic,
    Lower,
    Upper,
    Free,
    Fixed,
}

impl From<char> for SolutionStatus {
    fn from(c: char) -> Self {
        match c {
            'b' => SolutionStatus::Basic,
            'l' => SolutionStatus::Lower,
            'u' => SolutionStatus::Upper,
            'f' => SolutionStatus::Free,
            's' => SolutionStatus::Fixed,
            _ => SolutionStatus::Undefined,
        }
    }
}

#[derive(Debug)]
struct Solution {
    status: SolutionStatus,
    primal: f64,
    dual: f64,
}

#[derive(Debug)]
struct ProblemSolution {
    rows: Vec<Solution>,
    cols: Vec<Solution>,
    primal_status: i32,
    dual_status: i32,
    objective: f64,
}

fn read_solution_file(filename: &str, m: usize, n: usize) -> Result<ProblemSolution, String> {
    let file = File::open(filename).map_err(|e| format!("Unable to open '{}' - {}", filename, e))?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // Read header line
    let header = lines.next().ok_or("Empty file")??;
    let header_parts: Vec<&str> = header.split_whitespace().collect();
    if header_parts.len() != 6 {
        return Err("Invalid header format".to_string());
    }

    if header_parts[0] != "s" || header_parts[1] != "bas" {
        return Err("Invalid solution designator".to_string());
    }

    let file_m = header_parts[2].parse::<usize>().map_err(|_| "Invalid row count")?;
    let file_n = header_parts[3].parse::<usize>().map_err(|_| "Invalid column count")?;

    if file_m != m || file_n != n {
        return Err("Problem dimensions mismatch".to_string());
    }

    let primal_status = match header_parts[4] {
        "u" => 1,
        "f" => 2,
        "i" => 3,
        "n" => 4,
        _ => return Err("Invalid primal status".to_string()),
    };

    let dual_status = match header_parts[5] {
        "u" => 1,
        "f" => 2,
        "i" => 3,
        "n" => 4,
        _ => return Err("Invalid dual status".to_string()),
    };

    let objective = header_parts[5].parse::<f64>().map_err(|_| "Invalid objective value")?;

    let mut rows = vec![
        Solution {
            status: SolutionStatus::Undefined,
            primal: 0.0,
            dual: 0.0,
        };
        m
    ];
    let mut cols = vec![
        Solution {
            status: SolutionStatus::Undefined,
            primal: 0.0,
            dual: 0.0,
        };
        n
    ];

    for line in lines {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "i" => {
                if parts.len() != 5 {
                    return Err("Invalid row solution line".to_string());
                }
                let i = parts[1].parse::<usize>().map_err(|_| "Invalid row number")?;
                if i < 1 || i > m {
                    return Err("Row number out of range".to_string());
                }

                let status_char = parts[2].chars().next().ok_or("Missing status")?;
                let primal = parts[3].parse::<f64>().map_err(|_| "Invalid primal value")?;
                let dual = parts[4].parse::<f64>().map_err(|_| "Invalid dual value")?;

                rows[i - 1] = Solution {
                    status: SolutionStatus::from(status_char),
                    primal,
                    dual,
                };
            }
            "j" => {
                if parts.len() != 5 {
                    return Err("Invalid column solution line".to_string());
                }
                let j = parts[1].parse::<usize>().map_err(|_| "Invalid column number")?;
                if j < 1 || j > n {
                    return Err("Column number out of range".to_string());
                }

                let status_char = parts[2].chars().next().ok_or("Missing status")?;
                let primal = parts[3].parse::<f64>().map_err(|_| "Invalid primal value")?;
                let dual = parts[4].parse::<f64>().map_err(|_| "Invalid dual value")?;

                cols[j - 1] = Solution {
                    status: SolutionStatus::from(status_char),
                    primal,
                    dual,
                };
            }
            "e" => break,
            _ => return Err("Invalid line designator".to_string()),
        }
    }

    // Verify all solutions are defined
    for (i, row) in rows.iter().enumerate() {
        if matches!(row.status, SolutionStatus::Undefined) {
            return Err(format!("Missing solution for row {}", i + 1));
        }
    }

    for (j, col) in cols.iter().enumerate() {
        if matches!(col.status, SolutionStatus::Undefined) {
            return Err(format!("Missing solution for column {}", j + 1));
        }
    }

    Ok(ProblemSolution {
        rows,
        cols,
        primal_status,
        dual_status,
        objective,
    })
}

pub fn glp_read_sol(P: *mut glp_prob, fname: *const libc::c_char) -> libc::c_int {
    let filename = unsafe { CStr::from_ptr(fname) }.to_str().unwrap();
    let m = unsafe { (*P).m as usize };
    let n = unsafe { (*P).n as usize };

    match read_solution_file(filename, m, n) {
        Ok(solution) => {
            unsafe {
                (*P).pbs_stat = solution.primal_status;
                (*P).dbs_stat = solution.dual_status;
                (*P).obj_val = solution.objective;
                (*P).it_cnt = 0;
                (*P).some = 0;

                for (i, row) in solution.rows.iter().enumerate() {
                    glp_set_row_stat(P, (i + 1) as i32, row.status as i32);
                    (*(*((*P).row).offset(i as isize))).prim = row.primal;
                    (*(*((*P).row).offset(i as isize))).dual = row.dual;
                }

                for (j, col) in solution.cols.iter().enumerate() {
                    glp_set_col_stat(P, (j + 1) as i32, col.status as i32);
                    (*(*((*P).col).offset(j as isize))).prim = col.primal;
                    (*(*((*P).col).offset(j as isize))).dual = col.dual;
                }
            }
            0
        }
        Err(e) => {
            eprintln!("Error reading solution: {}", e);
            1
        }
    }
}