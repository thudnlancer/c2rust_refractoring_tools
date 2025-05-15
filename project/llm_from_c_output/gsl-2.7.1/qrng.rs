use gsl::qrng::{QRng, QRngType};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut q = QRng::new(QRngType::sobol(), 2)?;

    for _ in 0..1024 {
        let v = q.get()?;
        println!("{:.5} {:.5}", v[0], v[1]);
    }

    Ok(())
}