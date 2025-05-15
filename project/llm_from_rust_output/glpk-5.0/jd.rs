pub fn glp_jday(d: i32, m: i32, y: i32) -> Result<i32, String> {
    if !(1 <= d && d <= 31 && 1 <= m && m <= 12 && 1 <= y && y <= 4000) {
        return Err("Invalid date parameters".to_string());
    }

    let (m, y) = if m >= 3 {
        (m - 3, y)
    } else {
        (m + 9, y - 1)
    };

    let c = y / 100;
    let ya = y - 100 * c;
    let j = 146097 * c / 4 
        + 1461 * ya / 4 
        + (153 * m + 2) / 5 + d 
        + 1721119;

    let (dd, _, _) = glp_jdate(j)?;
    if d != dd {
        return Err("Date validation failed".to_string());
    }

    Ok(j)
}

pub fn glp_jdate(j: i32) -> Result<(i32, i32, i32), String> {
    if !(1721426 <= j && j <= 3182395) {
        return Err("Julian date out of range".to_string());
    }

    let mut j = j - 1721119;
    let y = (4 * j - 1) / 146097;
    j = (4 * j - 1) % 146097;
    let mut d = j / 4;
    let j = (4 * d + 3) / 1461;
    d = (4 * d + 3) % 1461;
    d = (d + 4) / 4;
    let mut m = (5 * d - 3) / 153;
    d = (5 * d - 3) % 153;
    d = (d + 5) / 5;
    let mut y = 100 * y + j;

    let m = if m <= 9 {
        m + 3
    } else {
        y += 1;
        m - 9
    };

    Ok((d, m, y))
}