use num_traits::ToPrimitive;
use std::f64;

#[derive(Clone)]
pub struct GslHistogram2d {
    pub nx: usize,
    pub ny: usize,
    pub xrange: Vec<f64>,
    pub yrange: Vec<f64>,
    pub bin: Vec<f64>,
}

impl GslHistogram2d {
    pub fn sum(&self) -> f64 {
        self.bin.iter().sum()
    }

    pub fn xmean(&self) -> f64 {
        let mut wmean = 0.0;
        let mut w = 0.0;

        for i in 0..self.nx {
            let xi = (self.xrange[i + 1] + self.xrange[i]) / 2.0;
            let wi: f64 = (0..self.ny)
                .map(|j| self.bin[i * self.ny + j])
                .filter(|&wij| wij > 0.0)
                .sum();

            if wi > 0.0 {
                w += wi;
                wmean += (xi - wmean) * (wi / w);
            }
        }

        wmean
    }

    pub fn ymean(&self) -> f64 {
        let mut wmean = 0.0;
        let mut w = 0.0;

        for j in 0..self.ny {
            let yj = (self.yrange[j + 1] + self.yrange[j]) / 2.0;
            let wj: f64 = (0..self.nx)
                .map(|i| self.bin[i * self.ny + j])
                .filter(|&wij| wij > 0.0)
                .sum();

            if wj > 0.0 {
                w += wj;
                wmean += (yj - wmean) * (wj / w);
            }
        }

        wmean
    }

    pub fn xsigma(&self) -> f64 {
        let xmean = self.xmean();
        let mut wvariance = 0.0;
        let mut w = 0.0;

        for i in 0..self.nx {
            let xi = (self.xrange[i + 1] + self.xrange[i]) / 2.0 - xmean;
            let wi: f64 = (0..self.ny)
                .map(|j| self.bin[i * self.ny + j])
                .filter(|&wij| wij > 0.0)
                .sum();

            if wi > 0.0 {
                w += wi;
                wvariance += (xi * xi - wvariance) * (wi / w);
            }
        }

        wvariance.sqrt()
    }

    pub fn ysigma(&self) -> f64 {
        let ymean = self.ymean();
        let mut wvariance = 0.0;
        let mut w = 0.0;

        for j in 0..self.ny {
            let yj = (self.yrange[j + 1] + self.yrange[j]) / 2.0 - ymean;
            let wj: f64 = (0..self.nx)
                .map(|i| self.bin[i * self.ny + j])
                .filter(|&wij| wij > 0.0)
                .sum();

            if wj > 0.0 {
                w += wj;
                wvariance += (yj * yj - wvariance) * (wj / w);
            }
        }

        wvariance.sqrt()
    }

    pub fn cov(&self) -> f64 {
        let xmean = self.xmean();
        let ymean = self.ymean();
        let mut wcovariance = 0.0;
        let mut w = 0.0;

        for j in 0..self.ny {
            for i in 0..self.nx {
                let xi = (self.xrange[i + 1] + self.xrange[i]) / 2.0 - xmean;
                let yj = (self.yrange[j + 1] + self.yrange[j]) / 2.0 - ymean;
                let wij = self.bin[i * self.ny + j];

                if wij > 0.0 {
                    w += wij;
                    wcovariance += (xi * yj - wcovariance) * (wij / w);
                }
            }
        }

        wcovariance
    }
}