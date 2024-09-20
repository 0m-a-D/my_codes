pub mod plot;
/// returns Option<(bias, weight)>
pub fn linear_regression(data_points: Vec<(f32, f32)>) -> Option<(f32, f32)> {
    if data_points.is_empty() {
        return None;
    }

    let count = data_points.len() as f32;
    let mean_x = data_points.iter().fold(0.0, |sum, y| sum + y.0) / count;
    let mean_y = data_points.iter().fold(0.0, |sum, y| sum + y.1) / count;

    let mut covariance = 0.0;
    let mut std_dev_sqr_x = 0.0;
    let mut std_dev_sqr_y = 0.0;

    for data_point in data_points {
        covariance += (data_point.0 - mean_x) * (data_point.1 - mean_y);
        std_dev_sqr_x += (data_point.0 - mean_x).powi(2);
        std_dev_sqr_y += (data_point.1 - mean_y).powi(2);
    }

    let std_dev_x = std_dev_sqr_x.sqrt();
    let std_dev_y = std_dev_sqr_y.sqrt();
    let std_dev_prod = std_dev_x * std_dev_y;

    let pcc = covariance / std_dev_prod; //Pearson's correlation constant
    let b = pcc * (std_dev_y / std_dev_x); //Slope of the line --> WEIGHT
    let a = mean_y - b * mean_x; //Y-Intercept of the line --> BIAS

    Some((a, b))
}
