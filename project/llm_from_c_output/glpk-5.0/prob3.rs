use std::collections::BTreeMap;
use std::ffi::CStr;

pub struct GlpProb {
    m: usize,
    n: usize,
    row: Vec<GlpRow>,
    col: Vec<GlpCol>,
    r_tree: Option<BTreeMap<String, usize>>,
    c_tree: Option<BTreeMap<String, usize>>,
}

pub struct GlpRow {
    name: Option<String>,
    i: usize,
}

pub struct GlpCol {
    name: Option<String>,
    j: usize,
}

impl GlpProb {
    /// Creates the name index for the specified problem object.
    /// The name index is an auxiliary data structure for quickly finding rows and columns by name.
    pub fn create_index(&mut self) {
        // Create row name index
        if self.r_tree.is_none() {
            let mut r_tree = BTreeMap::new();
            for row in &self.row {
                if let Some(ref name) = row.name {
                    r_tree.insert(name.clone(), row.i);
                }
            }
            self.r_tree = Some(r_tree);
        }

        // Create column name index
        if self.c_tree.is_none() {
            let mut c_tree = BTreeMap::new();
            for col in &self.col {
                if let Some(ref name) = col.name {
                    c_tree.insert(name.clone(), col.j);
                }
            }
            self.c_tree = Some(c_tree);
        }
    }

    /// Finds a row by its name.
    /// Returns the ordinal number of the row if found, otherwise 0.
    pub fn find_row(&self, name: &str) -> usize {
        if self.r_tree.is_none() {
            panic!("glp_find_row: row name index does not exist");
        }

        if name.is_empty() || name.len() > 255 {
            return 0;
        }

        self.r_tree.as_ref()
            .and_then(|tree| tree.get(name))
            .copied()
            .unwrap_or(0)
    }

    /// Finds a column by its name.
    /// Returns the ordinal number of the column if found, otherwise 0.
    pub fn find_col(&self, name: &str) -> usize {
        if self.c_tree.is_none() {
            panic!("glp_find_col: column name index does not exist");
        }

        if name.is_empty() || name.len() > 255 {
            return 0;
        }

        self.c_tree.as_ref()
            .and_then(|tree| tree.get(name))
            .copied()
            .unwrap_or(0)
    }

    /// Deletes the name index and frees the allocated memory.
    pub fn delete_index(&mut self) {
        self.r_tree = None;
        self.c_tree = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_operations() {
        let mut prob = GlpProb {
            m: 2,
            n: 2,
            row: vec![
                GlpRow { name: Some("row1".to_string()), i: 1 },
                GlpRow { name: Some("row2".to_string()), i: 2 },
            ],
            col: vec![
                GlpCol { name: Some("col1".to_string()), j: 1 },
                GlpCol { name: Some("col2".to_string()), j: 2 },
            ],
            r_tree: None,
            c_tree: None,
        };

        prob.create_index();
        assert_eq!(prob.find_row("row1"), 1);
        assert_eq!(prob.find_col("col1"), 1);
        assert_eq!(prob.find_row("nonexistent"), 0);

        prob.delete_index();
        assert!(prob.r_tree.is_none());
        assert!(prob.c_tree.is_none());
    }
}