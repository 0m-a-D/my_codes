mod dataset;
use dataset::Dataset;
use linear_reg::{linear_regression, plot::plot};

fn main() {
    let dataset = Dataset::two_x_plus_one();
    let (bias, weight) = linear_regression(dataset.data_points).unwrap();
    println!("bias -> {}; weight -> {}", bias, weight);

    let input = 403.0;
    println!(
        "your input: {}, predicted output: {}",
        input,
        (input * weight + bias) as u32
    );
    let path = std::path::Path::new("data/1.png");
    let _ = plot(weight, bias, path);

    let two_x_plus_two = Dataset::dataset_generator(20, 2.0, 2.0);
    let (bias, weight) = linear_regression(two_x_plus_two.data_points).unwrap();
    println!("bias -> {}; weight -> {}", bias, weight);
    let input = 403.0;
    println!(
        "your input: {}, predicted output: {}",
        input,
        (input * weight + bias) as u32
    );
    let path = std::path::Path::new("data/2.png");
    let _ = plot(weight, bias, path);
}
