use std::f64::consts;
use std::ops::{Index, IndexMut};

pub struct Histogram2D {
    nx: usize,
    ny: usize,
    xrange: Vec<f64>,
    yrange: Vec<f64>,
    bin: Vec<f64>,
}

impl Histogram2D {
    pub fn sum(&self) -> f64 {
        self.bin.iter().sum()
    }

    pub fn xmean(&self) -> f64 {
        let mut wmean = 0.0f64;
        let mut w = 0.0f64;

        for i in 0..self.nx {
            let xi = (self.xrange[i + 1] + self.xrange[i]) / 2.0;
            let mut wi = 0.0;

            for j in 0..self.ny {
                let wij = self.bin[i * self.ny + j];
                if wij > 0.0 {
                    wi += wij;
                }
            }

            if wi > 0.0 {
                w += wi;
                wmean += (xi - wmean) * (wi / w);
            }
        }

        wmean
    }

    pub fn ymean(&self) -> f64 {
        let mut wmean = 0.0f64;
        let mut w = 0.0f64;

        for j in 0..self.ny {
            let yj = (self.yrange[j + 1] + self.yrange[j]) / 2.0;
            let mut wj = 0.0;

            for i in 0..self.nx {
                let wij = self.bin[i * self.ny + j];
                if wij > 0.0 {
                    wj += wij;
                }
            }

            if wj > 0.0 {
                w += wj;
                wmean += (yj - wmean) * (wj / w);
            }
        }

        wmean
    }

    pub fn xsigma(&self) -> f64 {
        let xmean = self.xmean();
        let mut wvariance = 0.0f64;
        let mut w = 0.0f64;

        for i in 0..self.nx {
            let xi = (self.xrange[i + 1] + self.xrange[i]) / 2.0 - xmean;
            let mut wi = 0.0;

            for j in 0..self.ny {
                let wij = self.bin[i * self.ny + j];
                if wij > 0.0 {
                    wi += wij;
                }
            }

            if wi > 0.0 {
                w += wi;
                wvariance += ((xi * xi) - wvariance) * (wi / w);
            }
        }

        wvariance.sqrt()
    }

    pub fn ysigma(&self) -> f64 {
        let ymean = self.ymean();
        let mut wvariance = 0.0f64;
        let mut w = 0.0f64;

        for j in 0..self.ny {
            let yj = (self.yrange[j + 1] + self.yrange[j]) / 2.0 - ymean;
            let mut wj = 0.0;

            for i in 0..self.nx {
                let wij = self.bin[i * self.ny + j];
                if wij > 0.0 {
                    wj += wij;
                }
            }

            if wj > 0.0 {
                w += wj;
                wvariance += ((yj * yj) - wvariance) * (wj / w);
            }
        }

        wvariance.sqrt()
    }

    pub fn cov(&self) -> f64 {
        let xmean = self.xmean();
        let ymean = self.ymean();
        let mut wcovariance = 0.0f64;
        let mut w = 0.0f64;

        for j in 0..self.ny {
            for i in 0..self.nx {
                let xi = (self.xrange[i + 1] + self.xrange[i]) / 2.0 - xmean;
                let yj = (self.yrange[j + 1] + self.yrange[j]) / 2.0 - ymean;
                let wij = self.bin[i * self.ny + j];

                if wij > 0.0 {
                    w += wij;
                    wcovariance += ((xi * yj) - wcovariance) * (wij / w);
                }
            }
        }

        wcovariance
    }
}

impl Index<(usize, usize)> for Histogram2D {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.bin[index.0 * self.ny + index.1]
    }
}

impl IndexMut<(usize, usize)> for Histogram2D {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.bin[index.0 * self.ny + index.1]
    }
}