use gsl::{
    eigen::{francis, nonsymm},
    linalg::{balance, hessenberg},
    matrix::Matrix,
    vector::{Vector, VectorComplex},
    Value,
};

pub struct NonSymmWorkspace {
    size: usize,
    diag: Vector,
    tau: Vector,
    z: Option<Matrix>,
    do_balance: bool,
    n_evals: usize,
    francis_workspace: francis::Workspace,
}

impl NonSymmWorkspace {
    pub fn new(n: usize) -> Option<Self> {
        if n == 0 {
            gsl::error("matrix dimension must be positive integer", "nonsymm.rs", 61, Value::Invalid);
            return None;
        }

        let diag = Vector::new(n)?;
        let tau = Vector::new(n)?;
        let francis_workspace = francis::Workspace::new()?;

        Some(Self {
            size: n,
            diag,
            tau,
            z: None,
            do_balance: false,
            n_evals: 0,
            francis_workspace,
        })
    }

    pub fn params(&mut self, compute_t: bool, balance: bool) {
        self.francis_workspace.set_compute_t(compute_t);
        self.do_balance = balance;
    }

    pub fn eigen(&mut self, a: &mut Matrix, eval: &mut VectorComplex) -> Result<(), Value> {
        if a.size1() != a.size2() {
            gsl::error(
                "matrix must be square to compute eigenvalues",
                "nonsymm.rs",
                181,
                Value::NotSquare,
            );
            return Err(Value::NotSquare);
        }

        if eval.size() != a.size1() {
            gsl::error(
                "eigenvalue vector must match matrix size",
                "nonsymm.rs",
                185,
                Value::BadLength,
            );
            return Err(Value::BadLength);
        }

        if self.do_balance {
            balance::matrix(a, &mut self.diag)?;
        }

        hessenberg::decomp(a, &mut self.tau)?;

        let result = match &mut self.z {
            Some(z) => {
                hessenberg::unpack(a, &self.tau, z)?;
                let res = francis::eigen_z(a, eval, z, &mut self.francis_workspace);
                if self.do_balance {
                    balance::accum(z, &mut self.diag)?;
                }
                res
            }
            None => francis::eigen(a, eval, &mut self.francis_workspace),
        };

        self.n_evals = self.francis_workspace.n_evals();
        result
    }

    pub fn eigen_z(
        &mut self,
        a: &mut Matrix,
        eval: &mut VectorComplex,
        z: &mut Matrix,
    ) -> Result<(), Value> {
        if a.size1() != a.size2() {
            gsl::error(
                "matrix must be square to compute eigenvalues",
                "nonsymm.rs",
                271,
                Value::NotSquare,
            );
            return Err(Value::NotSquare);
        }

        if eval.size() != a.size1() {
            gsl::error(
                "eigenvalue vector must match matrix size",
                "nonsymm.rs",
                275,
                Value::BadLength,
            );
            return Err(Value::BadLength);
        }

        if z.size1() != z.size2() || z.size1() != a.size1() {
            gsl::error(
                "Z matrix has wrong dimensions",
                "nonsymm.rs",
                279,
                Value::BadLength,
            );
            return Err(Value::BadLength);
        }

        self.z = Some(z.clone());
        let result = self.eigen(a, eval);
        self.z = None;
        result
    }
}