use std::f64::consts::PI;

#[derive(Copy, Clone, Debug)]
pub struct GslComplex {
    pub dat: [f64; 2],
}

impl GslComplex {
    pub fn new(real: f64, imag: f64) -> Self {
        Self { dat: [real, imag] }
    }

    pub fn polar(r: f64, theta: f64) -> Self {
        Self::new(r * theta.cos(), r * theta.sin())
    }

    pub fn arg(&self) -> f64 {
        let x = self.dat[0];
        let y = self.dat[1];
        if x == 0.0 && y == 0.0 {
            0.0
        } else {
            y.atan2(x)
        }
    }

    pub fn abs(&self) -> f64 {
        self.dat[0].hypot(self.dat[1])
    }

    pub fn abs2(&self) -> f64 {
        let x = self.dat[0];
        let y = self.dat[1];
        x * x + y * y
    }

    pub fn logabs(&self) -> f64 {
        let xabs = self.dat[0].abs();
        let yabs = self.dat[1].abs();
        let (max, u) = if xabs >= yabs {
            (xabs, yabs / xabs)
        } else {
            (yabs, xabs / yabs)
        };
        max.ln() + 0.5 * (1.0 + u * u).ln()
    }

    pub fn add(&self, other: Self) -> Self {
        Self::new(
            self.dat[0] + other.dat[0],
            self.dat[1] + other.dat[1],
        )
    }

    pub fn add_real(&self, x: f64) -> Self {
        Self::new(self.dat[0] + x, self.dat[1])
    }

    pub fn add_imag(&self, y: f64) -> Self {
        Self::new(self.dat[0], self.dat[1] + y)
    }

    pub fn sub(&self, other: Self) -> Self {
        Self::new(
            self.dat[0] - other.dat[0],
            self.dat[1] - other.dat[1],
        )
    }

    pub fn sub_real(&self, x: f64) -> Self {
        Self::new(self.dat[0] - x, self.dat[1])
    }

    pub fn sub_imag(&self, y: f64) -> Self {
        Self::new(self.dat[0], self.dat[1] - y)
    }

    pub fn mul(&self, other: Self) -> Self {
        let ar = self.dat[0];
        let ai = self.dat[1];
        let br = other.dat[0];
        let bi = other.dat[1];
        Self::new(ar * br - ai * bi, ar * bi + ai * br)
    }

    pub fn mul_real(&self, x: f64) -> Self {
        Self::new(x * self.dat[0], x * self.dat[1])
    }

    pub fn mul_imag(&self, y: f64) -> Self {
        Self::new(-y * self.dat[1], y * self.dat[0])
    }

    pub fn div(&self, other: Self) -> Self {
        let ar = self.dat[0];
        let ai = self.dat[1];
        let br = other.dat[0];
        let bi = other.dat[1];
        let s = 1.0 / other.abs();
        let sbr = s * br;
        let sbi = s * bi;
        let zr = (ar * sbr + ai * sbi) * s;
        let zi = (ai * sbr - ar * sbi) * s;
        Self::new(zr, zi)
    }

    pub fn div_real(&self, x: f64) -> Self {
        Self::new(self.dat[0] / x, self.dat[1] / x)
    }

    pub fn div_imag(&self, y: f64) -> Self {
        Self::new(self.dat[1] / y, -self.dat[0] / y)
    }

    pub fn conjugate(&self) -> Self {
        Self::new(self.dat[0], -self.dat[1])
    }

    pub fn negative(&self) -> Self {
        Self::new(-self.dat[0], -self.dat[1])
    }

    pub fn inverse(&self) -> Self {
        let s = 1.0 / self.abs();
        Self::new(
            self.dat[0] * s * s,
            -(self.dat[1] * s) * s,
        )
    }

    pub fn sqrt(&self) -> Self {
        if self.dat[0] == 0.0 && self.dat[1] == 0.0 {
            Self::new(0.0, 0.0)
        } else {
            let x = self.dat[0].abs();
            let y = self.dat[1].abs();
            let w = if x >= y {
                let t = y / x;
                x.sqrt() * (0.5 * (1.0 + (1.0 + t * t).sqrt())).sqrt()
            } else {
                let t = x / y;
                y.sqrt() * (0.5 * (t + (1.0 + t * t).sqrt()).sqrt()
            };

            if self.dat[0] >= 0.0 {
                let ai = self.dat[1];
                Self::new(w, ai / (2.0 * w))
            } else {
                let ai = self.dat[1];
                let vi = if ai >= 0.0 { w } else { -w };
                Self::new(ai / (2.0 * vi), vi)
            }
        }
    }

    pub fn sqrt_real(x: f64) -> Self {
        if x >= 0.0 {
            Self::new(x.sqrt(), 0.0)
        } else {
            Self::new(0.0, (-x).sqrt())
        }
    }

    pub fn exp(&self) -> Self {
        let rho = self.dat[0].exp();
        let theta = self.dat[1];
        Self::new(rho * theta.cos(), rho * theta.sin())
    }

    pub fn pow(&self, b: Self) -> Self {
        if self.dat[0] == 0.0 && self.dat[1] == 0.0 {
            if b.dat[0] == 0.0 && b.dat[1] == 0.0 {
                Self::new(1.0, 0.0)
            } else {
                Self::new(0.0, 0.0)
            }
        } else if b.dat[0] == 1.0 && b.dat[1] == 0.0 {
            *self
        } else if b.dat[0] == -1.0 && b.dat[1] == 0.0 {
            self.inverse()
        } else {
            let logr = self.logabs();
            let theta = self.arg();
            let br = b.dat[0];
            let bi = b.dat[1];
            let rho = (logr * br - bi * theta).exp();
            let beta = theta * br + bi * logr;
            Self::new(rho * beta.cos(), rho * beta.sin())
        }
    }

    pub fn pow_real(&self, b: f64) -> Self {
        if self.dat[0] == 0.0 && self.dat[1] == 0.0 {
            if b == 0.0 {
                Self::new(1.0, 0.0)
            } else {
                Self::new(0.0, 0.0)
            }
        } else {
            let logr = self.logabs();
            let theta = self.arg();
            let rho = (logr * b).exp();
            let beta = theta * b;
            Self::new(rho * beta.cos(), rho * beta.sin())
        }
    }

    pub fn log(&self) -> Self {
        let logr = self.logabs();
        let theta = self.arg();
        Self::new(logr, theta)
    }

    pub fn log10(&self) -> Self {
        self.log().mul_real(1.0 / 10.0f64.ln())
    }

    pub fn log_b(&self, b: Self) -> Self {
        self.log().div(b.log())
    }

    pub fn sin(&self) -> Self {
        let R = self.dat[0];
        let I = self.dat[1];
        if I == 0.0 {
            Self::new(R.sin(), 0.0)
        } else {
            Self::new(R.sin() * I.cosh(), R.cos() * I.sinh())
        }
    }

    pub fn cos(&self) -> Self {
        let R = self.dat[0];
        let I = self.dat[1];
        if I == 0.0 {
            Self::new(R.cos(), 0.0)
        } else {
            Self::new(R.cos() * I.cosh(), R.sin() * (-I).sinh())
        }
    }

    pub fn tan(&self) -> Self {
        let R = self.dat[0];
        let I = self.dat[1];
        if I.abs() < 1.0 {
            let D = R.cos().powi(2) + I.sinh().powi(2);
            Self::new(0.5 * (2.0 * R).sin() / D, 0.5 * (2.0 * I).sinh() / D)
        } else {
            let D = R.cos().powi(2) + I.sinh().powi(2);
            let F = 1.0 + (R.cos() / I.sinh()).powi(2);
            Self::new(0.5 * (2.0 * R).sin() / D, 1.0 / (I.tanh() * F))
        }
    }

    pub fn sec(&self) -> Self {
        self.cos().inverse()
    }

    pub fn csc(&self) -> Self {
        self.sin().inverse()
    }

    pub fn cot(&self) -> Self {
        self.tan().inverse()
    }

    pub fn arcsin(&self) -> Self {
        let R = self.dat[0];
        let I = self.dat[1];
        if I == 0.0 {
            Self::arcsin_real(R)
        } else {
            let x = R.abs();
            let y = I.abs();
            let r = (x + 1.0).hypot(y);
            let s = (x - 1.0).hypot(y);
            let A = 0.5 * (r + s);
            let B = x / A;
            let y2 = y * y;

            let real = if B <= 0.6417 {
                B.asin()
            } else if x <= 1.0 {
                let D = 0.5 * (A + x) * (y2 / (r + x + 1.0) + (s + (1.0 - x)));
                (x / D.sqrt()).atan()
            } else {
                let Apx = A + x;
                let D = 0.5 * (Apx / (r + x + 1.0) + Apx / (s + (x - 1.0));
                (x / (y * D.sqrt())).atan()
            };

            let imag = if A <= 1.5 {
                let Am1 = if x < 1.0 {
                    0.5 * (y2 / (r + (x + 1.0)) + y2 / (s + (1.0 - x)))
                } else {
                    0.5 * (y2 / (r + (x + 1.0)) + (s + (x - 1.0)))
                };
                (1.0 + Am1 + (Am1 * (A + 1.0)).sqrt()).ln_1p()
            } else {
                (A + (A * A - 1.0).sqrt()).ln()
            };

            Self::new(
                if R >= 0.0 { real } else { -real },
                if I >= 0.0 { imag } else { -imag },
            )
        }
    }

    pub fn arcsin_real(a: f64) -> Self {
        if a.abs() <= 1.0 {
            Self::new(a.asin(), 0.0)
        } else if a < 0.0 {
            Self::new(-PI / 2.0, (-a).acosh())
        } else {
            Self::new(PI / 2.0, (-a).acosh())
        }
    }

    pub fn arccos(&self) -> Self {
        let R = self.dat[0];
        let I = self.dat[1];
        if I == 0.0 {
            Self::arccos_real(R)
        } else {
            let x = R.abs();
            let y = I.abs();
            let r = (x + 1.0).hypot(y);
            let s = (x - 1.0).hypot(y);
            let A = 0.5 * (r + s);
            let B = x / A;
            let y2 = y * y;

            let real = if B <= 0.6417 {
                B.acos()
            } else if x <= 1.0 {
                let D = 0.5 * (A + x) * (y2 / (r + x + 1.0) + (s + (1.0 - x)));
                (D.sqrt() / x).atan()
            } else {
                let Apx = A + x;
                let D = 0.5 * (Apx / (r + x + 1.0) + Apx / (s + (x - 1.0)));
                (y * D.sqrt() / x).atan()
            };

            let imag = if A <= 1.5 {
                let Am1 = if x < 1.0 {
                    0.5 * (y2 / (r + (x + 1.0)) + y2 / (s + (1.0 - x)))
                } else {
                    0.5 * (y2 / (r + (x + 1.0)) + (s + (x - 1.0)))
                };
                (1.0 + Am1 + (Am1 * (A + 1.0)).sqrt()).ln_1p()
            } else {
                (A + (A * A - 1.0).sqrt()).ln()
            };

            Self::new(
                if R >= 0.0 { real } else { PI - real },
                if I >= 0.0 { -imag } else { imag },
            )
        }
    }

    pub fn arccos_real(a: f64) -> Self {
        if a.abs() <= 1.0 {
            Self::new(a.acos(), 0.0)
        } else if a < 0.0 {
            Self::new(PI, (-a).acosh())
        } else {
            Self::new(0.0, a.acosh())
        }
    }

    pub fn arctan(&self) -> Self {
        let R = self.dat[0];
        let I = self.dat[1];
        if I == 0.0 {
            Self::new(R.atan(), 0.0)
        } else {
            let r = R.hypot(I);
            let u = 2.0 * I / (1.0 + r * r);
            let imag = if u.abs() < 0.1 {
                0.25 * ((1.0 + u).ln_1p() - (1.0 - u).ln_1p())
            } else {
                let A = R.hypot(I + 1.0);
                let B = R.hypot(I - 1.0);
                0.5 * (A / B).ln()
            };

            if R == 0.0 {
                if I > 1.0 {
                    Self::new(PI / 2.0, imag)
                } else if I < -1.0 {
                    Self::new(-PI / 2.0, imag)
                } else {
                    Self::new(0.0, imag)
                }
            } else {
                Self::new(
                    0.5 * (2.0 * R).atan2((1.0 + r) * (1.0 - r)),
                    imag,
                )
            }
        }
    }

    pub fn arcsec(&self) -> Self {
        self.inverse().arccos()
    }

    pub fn arcsec_real(a: f64) -> Self {
        if a <= -1.0 || a >= 1.0 {
            Self::new((1.0 / a).acos(), 0.0)
        } else if a >= 0.0 {
            Self::new(0.0, (1.0 / a).acosh())
        } else {
            Self::new(PI, (-1.0 / a).acosh())
        }
    }

    pub fn arccsc(&self) -> Self {
        self.inverse().arcsin()
    }

    pub fn arccsc_real(a: f64) -> Self {
        if a <= -1.0 || a >= 1.0 {
            Self::new((1.0 / a).asin(), 0.0)
        } else if a >= 0.0 {
            Self::new(PI / 2.0, (-1.0 / a).acosh())
        } else {
            Self::new(-PI / 2.0, (1.0 / -a).acosh())
        }
    }

    pub fn arccot(&self) -> Self {
        if self.dat[0] == 0.0 && self.dat[1] == 0.0 {
            Self::new(PI / 2.0, 0.0)
        } else {
            self.inverse().arctan()
        }
    }

    pub fn sinh(&self) -> Self {
        let R = self.dat[0];
        let I = self.dat[1];
        Self::new(R.sinh() * I.cos(), R.cosh() * I.sin())
    }

    pub fn cosh(&self) -> Self {
        let R = self.dat[0];
       