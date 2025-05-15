fn main() {
    let data = [17.2, 18.1, 16.5, 18.3, 12.6];
    
    let mean = data.iter().sum::<f64>() / data.len() as f64;
    let variance = if data.len() > 1 {
        let sum_sq_diff = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>();
        sum_sq_diff / (data.len() - 1) as f64
    } else {
        0.0
    };
    let largest = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let smallest = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));

    println!("The dataset is {}, {}, {}, {}, {}",
             data[0], data[1], data[2], data[3], data[4]);

    println!("The sample mean is {}", mean);
    println!("The estimated variance is {}", variance);
    println!("The largest value is {}", largest);
    println!("The smallest value is {}", smallest);
}