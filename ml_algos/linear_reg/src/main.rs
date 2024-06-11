mod dataset;
use dataset::Dataset;
use linear_reg::linear_regression;

fn main() {
    let dataset = Dataset::two_x_plus_one();
    let (bias, weight) = linear_regression(dataset.data_points).unwrap();
    println!("bias -> {}; weight -> {}", bias, weight);

    println!(
        "your input: 300, predicted output: {}",
        300_f32 * weight.round() + bias.round()
    );
}
