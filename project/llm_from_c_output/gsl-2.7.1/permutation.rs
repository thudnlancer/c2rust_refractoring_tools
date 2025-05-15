use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum PermutationError {
    InvalidValue(String),
    DuplicateIndex(String),
    LengthMismatch(String),
    NoNextPermutation,
    NoPrevPermutation,
}

impl fmt::Display for PermutationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PermutationError::InvalidValue(msg) => write!(f, "Invalid value: {}", msg),
            PermutationError::DuplicateIndex(msg) => write!(f, "Duplicate index: {}", msg),
            PermutationError::LengthMismatch(msg) => write!(f, "Length mismatch: {}", msg),
            PermutationError::NoNextPermutation => write!(f, "No next permutation"),
            PermutationError::NoPrevPermutation => write!(f, "No previous permutation"),
        }
    }
}

impl Error for PermutationError {}

#[derive(Debug, Clone)]
pub struct Permutation {
    data: Vec<usize>,
}

impl Permutation {
    pub fn new(size: usize) -> Self {
        let mut data = Vec::with_capacity(size);
        for i in 0..size {
            data.push(i);
        }
        Permutation { data }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn data(&self) -> &[usize] {
        &self.data
    }

    pub fn swap(&mut self, i: usize, j: usize) -> Result<(), PermutationError> {
        let size = self.size();
        if i >= size {
            return Err(PermutationError::InvalidValue(
                "first index is out of range".to_string(),
            ));
        }
        if j >= size {
            return Err(PermutationError::InvalidValue(
                "second index is out of range".to_string(),
            ));
        }
        if i != j {
            self.data.swap(i, j);
        }
        Ok(())
    }

    pub fn is_valid(&self) -> Result<(), PermutationError> {
        let size = self.size();
        for i in 0..size {
            if self.data[i] >= size {
                return Err(PermutationError::InvalidValue(
                    "permutation index outside range".to_string(),
                ));
            }
            for j in 0..i {
                if self.data[i] == self.data[j] {
                    return Err(PermutationError::DuplicateIndex(
                        "duplicate permutation index".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn reverse(&mut self) {
        let size = self.size();
        for i in 0..size / 2 {
            let j = size - i - 1;
            self.data.swap(i, j);
        }
    }

    pub fn inverse(&self) -> Result<Self, PermutationError> {
        let size = self.size();
        let mut inv = Permutation::new(size);
        for i in 0..size {
            inv.data[self.data[i]] = i;
        }
        Ok(inv)
    }

    pub fn next(&mut self) -> Result<(), PermutationError> {
        let size = self.size();
        if size < 2 {
            return Err(PermutationError::NoNextPermutation);
        }

        let mut i = size - 2;
        while i > 0 && self.data[i] > self.data[i + 1] {
            i -= 1;
        }

        if i == 0 && self.data[0] > self.data[1] {
            return Err(PermutationError::NoNextPermutation);
        }

        let mut k = i + 1;
        for j in i + 2..size {
            if self.data[j] > self.data[i] && self.data[j] < self.data[k] {
                k = j;
            }
        }

        self.data.swap(i, k);

        let mut left = i + 1;
        let mut right = size - 1;
        while left < right {
            self.data.swap(left, right);
            left += 1;
            right -= 1;
        }

        Ok(())
    }

    pub fn prev(&mut self) -> Result<(), PermutationError> {
        let size = self.size();
        if size < 2 {
            return Err(PermutationError::NoPrevPermutation);
        }

        let mut i = size - 2;
        while i > 0 && self.data[i] < self.data[i + 1] {
            i -= 1;
        }

        if i == 0 && self.data[0] < self.data[1] {
            return Err(PermutationError::NoPrevPermutation);
        }

        let mut k = i + 1;
        for j in i + 2..size {
            if self.data[j] < self.data[i] && self.data[j] > self.data[k] {
                k = j;
            }
        }

        self.data.swap(i, k);

        let mut left = i + 1;
        let mut right = size - 1;
        while left < right {
            self.data.swap(left, right);
            left += 1;
            right -= 1;
        }

        Ok(())
    }

    pub fn multiply(
        &self,
        pa: &Permutation,
        pb: &Permutation,
    ) -> Result<Self, PermutationError> {
        let size = self.size();
        if pa.size() != size {
            return Err(PermutationError::LengthMismatch(
                "size of result does not match size of pa".to_string(),
            ));
        }
        if pb.size() != size {
            return Err(PermutationError::LengthMismatch(
                "size of result does not match size of pb".to_string(),
            ));
        }

        let mut result = Permutation::new(size);
        for i in 0..size {
            result.data[i] = pb.data[pa.data[i]];
        }
        Ok(result)
    }

    pub fn copy_from(&mut self, src: &Permutation) -> Result<(), PermutationError> {
        if self.size() != src.size() {
            return Err(PermutationError::LengthMismatch(
                "permutation lengths are not equal".to_string(),
            ));
        }
        self.data.copy_from_slice(&src.data);
        Ok(())
    }
}