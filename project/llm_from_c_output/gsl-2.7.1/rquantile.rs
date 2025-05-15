use rand::distributions::{Distribution, Rayleigh};
use rand::rngs::ThreadRng;
use rand::thread_rng;
use statrs::statistics::{OrderStatistics, Quantile};

fn main() {
    const N: usize = 10000;
    let mut data = vec![0.0; N];
    let mut rng = thread_rng();
    let rayleigh = Rayleigh::new(1.0).unwrap();

    // Initialize quantile estimators
    let mut work_25 = kolmogorov_smirnov::Empirical::new();
    let mut work_50 = kolmogorov_smirnov::Empirical::new();
    let mut work_75 = kolmogorov_smirnov::Empirical::new();

    // Add data to quantile accumulators
    for i in 0..N {
        data[i] = rayleigh.sample(&mut rng);
        work_25.add(data[i]);
        work_50.add(data[i]);
        work_75.add(data[i]);
    }

    // Exact values from sorted data
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let exact_p25 = data.quantile(0.25);
    let exact_p50 = data.quantile(0.5);
    let exact_p75 = data.quantile(0.75);

    // Estimated values
    let val_p25 = work_25.quantile(0.25);
    let val_p50 = work_50.quantile(0.5);
    let val_p75 = work_75.quantile(0.75);

    println!(
        "The dataset is {}, {}, {}, {}, {}, ...",
        data[0], data[1], data[2], data[3], data[4]
    );

    println!(
        "0.25 quartile: exact = {:.5}, estimated = {:.5}, error = {:.6e}",
        exact_p25,
        val_p25,
        (val_p25 - exact_p25) / exact_p25
    );
    println!(
        "0.50 quartile: exact = {:.5}, estimated = {:.5}, error = {:.6e}",
        exact_p50,
        val_p50,
        (val_p50 - exact_p50) / exact_p50
    );
    println!(
        "0.75 quartile: exact = {:.5}, estimated = {:.5}, error = {:.6e}",
        exact_p75,
        val_p75,
        (val_p75 - exact_p75) / exact_p75
    );
}