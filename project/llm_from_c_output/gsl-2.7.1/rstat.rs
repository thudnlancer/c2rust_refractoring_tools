use statrs::statistics::{Distribution, Statistics};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let data = [17.2, 18.1, 16.5, 18.3, 12.6];
    
    // Calculate statistics
    let mean = data.mean();
    let variance = data.variance();
    let largest = data.max().unwrap();
    let smallest = data.min().unwrap();
    let median = data.median();
    let sd = data.std_dev();
    let rms = (data.iter().map(|x| x.powi(2)).sum::<f64>() / data.len() as f64).sqrt();
    let sd_mean = sd / (data.len() as f64).sqrt();
    let skew = data.skewness();
    let kurtosis = data.kurtosis();
    let n = data.len();

    println!("The dataset is {}, {}, {}, {}, {}",
             data[0], data[1], data[2], data[3], data[4]);

    println!("The sample mean is {}", mean);
    println!("The estimated variance is {}", variance);
    println!("The largest value is {}", largest);
    println!("The smallest value is {}", smallest);
    println!("The median is {}", median);
    println!("The standard deviation is {}", sd);
    println!("The root mean square is {}", rms);
    println!("The standard deviation of the mean is {}", sd_mean);
    println!("The skew is {}", skew);
    println!("The kurtosis is {}", kurtosis);
    println!("There are {} items in the dataset", n);

    // Reset is not directly applicable in Rust's statrs, but we can create a new empty dataset
    let empty_data: [f64; 0] = [];
    let n_empty = empty_data.len();
    println!("There are {} items in the dataset", n_empty);

    Ok(())
}