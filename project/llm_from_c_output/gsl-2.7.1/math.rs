use std::f64::consts::PI;

const M_PI_2: f64 = PI / 2.0;

#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }

    pub fn polar(r: f64, theta: f64) -> Self {
        Self::new(r * theta.cos(), r * theta.sin())
    }

    pub fn arg(&self) -> f64 {
        if self.real == 0.0 && self.imag == 0.0 {
            0.0
        } else {
            self.imag.atan2(self.real)
        }
    }

    pub fn abs(&self) -> f64 {
        self.real.hypot(self.imag)
    }

    pub fn abs2(&self) -> f64 {
        self.real * self.real + self.imag * self.imag
    }

    pub fn logabs(&self) -> f64 {
        let xabs = self.real.abs();
        let yabs = self.imag.abs();
        let (max, u) = if xabs >= yabs {
            (xabs, yabs / xabs)
        } else {
            (yabs, xabs / yabs)
        };

        max.ln() + 0.5 * (1.0 + u * u).ln()
    }

    pub fn add(&self, other: Self) -> Self {
        Self::new(self.real + other.real, self.imag + other.imag)
    }

    pub fn add_real(&self, x: f64) -> Self {
        Self::new(self.real + x, self.imag)
    }

    pub fn add_imag(&self, y: f64) -> Self {
        Self::new(self.real, self.imag + y)
    }

    pub fn sub(&self, other: Self) -> Self {
        Self::new(self.real - other.real, self.imag - other.imag)
    }

    pub fn sub_real(&self, x: f64) -> Self {
        Self::new(self.real - x, self.imag)
    }

    pub fn sub_imag(&self, y: f64) -> Self {
        Self::new(self.real, self.imag - y)
    }

    pub fn mul(&self, other: Self) -> Self {
        Self::new(
            self.real * other.real - self.imag * other.imag,
            self.real * other.imag + self.imag * other.real,
        )
    }

    pub fn mul_real(&self, x: f64) -> Self {
        Self::new(x * self.real, x * self.imag)
    }

    pub fn mul_imag(&self, y: f64) -> Self {
        Self::new(-y * self.imag, y * self.real)
    }

    pub fn div(&self, other: Self) -> Self {
        let s = 1.0 / other.abs();
        let sbr = s * other.real;
        let sbi = s * other.imag;
        Self::new(
            (self.real * sbr + self.imag * sbi) * s,
            (self.imag * sbr - self.real * sbi) * s,
        )
    }

    pub fn div_real(&self, x: f64) -> Self {
        Self::new(self.real / x, self.imag / x)
    }

    pub fn div_imag(&self, y: f64) -> Self {
        Self::new(self.imag / y, -self.real / y)
    }

    pub fn conjugate(&self) -> Self {
        Self::new(self.real, -self.imag)
    }

    pub fn negative(&self) -> Self {
        Self::new(-self.real, -self.imag)
    }

    pub fn inverse(&self) -> Self {
        let s = 1.0 / self.abs();
        Self::new((self.real * s) * s, -(self.imag * s) * s)
    }

    pub fn sqrt(&self) -> Self {
        if self.real == 0.0 && self.imag == 0.0 {
            Self::new(0.0, 0.0)
        } else {
            let x = self.real.abs();
            let y = self.imag.abs();
            let w = if x >= y {
                let t = y / x;
                x.sqrt() * (0.5 * (1.0 + (1.0 + t * t).sqrt())).sqrt()
            } else {
                let t = x / y;
                y.sqrt() * (0.5 * (t + (1.0 + t * t).sqrt())).sqrt()
            };

            if self.real >= 0.0 {
                Self::new(w, self.imag / (2.0 * w))
            } else {
                let vi = if self.imag >= 0.0 { w } else { -w };
                Self::new(self.imag / (2.0 * vi), vi)
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
        let rho = self.real.exp();
        Self::new(rho * self.imag.cos(), rho * self.imag.sin())
    }

    pub fn pow(&self, b: Self) -> Self {
        if self.real == 0.0 && self.imag == 0.0 {
            if b.real == 0.0 && b.imag == 0.0 {
                Self::new(1.0, 0.0)
            } else {
                Self::new(0.0, 0.0)
            }
        } else if b.real == 1.0 && b.imag == 0.0 {
            *self
        } else if b.real == -1.0 && b.imag == 0.0 {
            self.inverse()
        } else {
            let logr = self.logabs();
            let theta = self.arg();
            let rho = (logr * b.real - b.imag * theta).exp();
            let beta = theta * b.real + b.imag * logr;
            Self::new(rho * beta.cos(), rho * beta.sin())
        }
    }

    pub fn pow_real(&self, b: f64) -> Self {
        if self.real == 0.0 && self.imag == 0.0 {
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
        Self::new(self.logabs(), self.arg())
    }

    pub fn log10(&self) -> Self {
        self.log().mul_real(1.0 / 10.0f64.ln())
    }

    pub fn log_b(&self, b: Self) -> Self {
        self.log().div(b.log())
    }

    pub fn sin(&self) -> Self {
        if self.imag == 0.0 {
            Self::new(self.real.sin(), 0.0)
        } else {
            Self::new(
                self.real.sin() * self.imag.cosh(),
                self.real.cos() * self.imag.sinh(),
            )
        }
    }

    pub fn cos(&self) -> Self {
        if self.imag == 0.0 {
            Self::new(self.real.cos(), 0.0)
        } else {
            Self::new(
                self.real.cos() * self.imag.cosh(),
                self.real.sin() * (-self.imag).sinh(),
            )
        }
    }

    pub fn tan(&self) -> Self {
        let r = self.real;
        let i = self.imag;
        if i.abs() < 1.0 {
            let d = r.cos().powi(2) + i.sinh().powi(2);
            Self::new(0.5 * (2.0 * r).sin() / d, 0.5 * (2.0 * i).sinh() / d)
        } else {
            let d = r.cos().powi(2) + i.sinh().powi(2);
            let f = 1.0 + (r.cos() / i.sinh()).powi(2);
            Self::new(1.0 / (i.tanh() * f), 0.5 * (2.0 * i).sinh() / d)
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
        if self.imag == 0.0 {
            Self::arcsin_real(self.real)
        } else {
            let x = self.real.abs();
            let y = self.imag.abs();
            let r = (x + 1.0).hypot(y);
            let s = (x - 1.0).hypot(y);
            let a = 0.5 * (r + s);
            let b = x / a;
            let y2 = y * y;

            let (real, imag) = if b <= 0.6417 {
                (b.asin(), 0.0)
            } else {
                let real = if x <= 1.0 {
                    let d = 0.5 * (a + x) * (y2 / (r + x + 1.0) + (s + (1.0 - x)));
                    (x / d.sqrt()).atan()
                } else {
                    let apx = a + x;
                    let d = 0.5 * (apx / (r + x + 1.0) + apx / (s + (x - 1.0)));
                    (x / (y * d.sqrt())).atan()
                };
                let imag = if a <= 1.5 {
                    let am1 = if x < 1.0 {
                        0.5 * (y2 / (r + (x + 1.0)) + y2 / (s + (1.0 - x)))
                    } else {
                        0.5 * (y2 / (r + (x + 1.0)) + (s + (x - 1.0)))
                    };
                    (am1 + (am1 * (a + 1.0)).sqrt()).ln_1p()
                } else {
                    (a + (a * a - 1.0).sqrt()).ln()
                };
                (real, imag)
            };

            Self::new(
                if self.real >= 0.0 { real } else { -real },
                if self.imag >= 0.0 { imag } else { -imag },
            )
        }
    }

    pub fn arcsin_real(a: f64) -> Self {
        if a.abs() <= 1.0 {
            Self::new(a.asin(), 0.0)
        } else if a < 0.0 {
            Self::new(-M_PI_2, (-a).acosh())
        } else {
            Self::new(M_PI_2, -a.acosh())
        }
    }

    pub fn arccos(&self) -> Self {
        if self.imag == 0.0 {
            Self::arccos_real(self.real)
        } else {
            let x = self.real.abs();
            let y = self.imag.abs();
            let r = (x + 1.0).hypot(y);
            let s = (x - 1.0).hypot(y);
            let a = 0.5 * (r + s);
            let b = x / a;
            let y2 = y * y;

            let (real, imag) = if b <= 0.6417 {
                (b.acos(), 0.0)
            } else {
                let real = if x <= 1.0 {
                    let d = 0.5 * (a + x) * (y2 / (r + x + 1.0) + (s + (1.0 - x));
                    (d.sqrt() / x).atan()
                } else {
                    let apx = a + x;
                    let d = 0.5 * (apx / (r + x + 1.0) + apx / (s + (x - 1.0)));
                    ((y * d.sqrt()) / x).atan()
                };
                let imag = if a <= 1.5 {
                    let am1 = if x < 1.0 {
                        0.5 * (y2 / (r + (x + 1.0)) + y2 / (s + (1.0 - x))
                    } else {
                        0.5 * (y2 / (r + (x + 1.0)) + (s + (x - 1.0)))
                    };
                    (am1 + (am1 * (a + 1.0)).sqrt()).ln_1p()
                } else {
                    (a + (a * a - 1.0).sqrt()).ln()
                };
                (real, imag)
            };

            Self::new(
                if self.real >= 0.0 { real } else { PI - real },
                if self.imag >= 0.0 { -imag } else { imag },
            )
        }
    }

    pub fn arccos_real(a: f64) -> Self {
        if a.abs() <= 1.0 {
            Self::new(a.acos(), 0.0)
        } else if a < 0.0 {
            Self::new(PI, -(-a).acosh())
        } else {
            Self::new(0.0, a.acosh())
        }
    }

    pub fn arctan(&self) -> Self {
        if self.imag == 0.0 {
            Self::new(self.real.atan(), 0.0)
        } else {
            let r = self.real.hypot(self.imag);
            let u = 2.0 * self.imag / (1.0 + r * r);

            let imag = if u.abs() < 0.1 {
                0.25 * ((1.0 + u).ln_1p() - (1.0 - u).ln_1p())
            } else {
                let a = self.real.hypot(self.imag + 1.0);
                let b = self.real.hypot(self.imag - 1.0);
                0.5 * (a / b).ln()
            };

            if self.real == 0.0 {
                if self.imag > 1.0 {
                    Self::new(M_PI_2, imag)
                } else if self.imag < -1.0 {
                    Self::new(-M_PI_2, imag)
                } else {
                    Self::new(0.0, imag)
                }
            } else {
                Self::new(
                    0.5 * (2.0 * self.real).atan2((1.0 + r) * (1.0 - r)),
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
            Self::new(PI, -(-1.0 / a).acosh())
        }
    }

    pub fn arccsc(&self) -> Self {
        self.inverse().arcsin()
    }

    pub fn arccsc_real(a: f64) -> Self {
        if a <= -1.0 || a >= 1.0 {
            Self::new((1.0 / a).asin(), 0.0)
        } else if a >= 0.0 {
            Self::new(M_PI_2, -(1.0 / a).acosh())
        } else {
            Self::new(-M_PI_2, (-1.0 / a).acosh())
        }
    }

    pub fn arccot(&self) -> Self {
        if self.real == 0.0 && self.imag == 0.0 {
            Self::new(M_PI_2, 0.0)
        } else {
            self.inverse().arctan()
        }
    }

    pub fn sinh(&self) -> Self {
        Self::new(
            self.real.sinh() * self.imag.cos(),
            self.real.cosh() * self.imag.sin(),
        )
    }

    pub fn cosh(&self) -> Self {
        Self::new(
            self.real.cosh() * self.imag.cos(),
            self.real.sinh() * self.imag.sin(),
        )
    }

    pub fn tanh(&self) -> Self {
        let r = self.real;
        let i = self.imag;
        if r.abs() < 1.0 {
            let d = i.cos().powi(2) + r.sinh().powi(2);
            Self::new(r.sinh() * r.cosh() / d, 0.5 * (2.0 * i).sin() / d)
        } else {
            let d = i.cos().powi(2) + r.sinh().powi(2);
            let f = 1.0 + (i.cos() / r.sinh()).powi(2);
            Self::new(1.0 / (r.tanh() *