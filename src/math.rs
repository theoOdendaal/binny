use ndarray::{Array1, Array2, s};
use ndarray_linalg::Inverse;

fn mean(observations: &[f64]) -> f64 {
    observations.iter().sum::<f64>() / observations.len() as f64
}

fn sum_of_squared_deviations(observations: &[f64]) -> f64 {
    let observations_mean = mean(observations);
    observations
        .iter()
        .map(|o| (o - observations_mean).powf(2.0))
        .sum::<f64>()
}

fn sample_variance(observations: &[f64]) -> f64 {
    sum_of_squared_deviations(observations) / (observations.len() - 1) as f64
}

fn population_variance(observations: &[f64]) -> f64 {
    sum_of_squared_deviations(observations) / observations.len() as f64
}

fn sample_standard_deviation(observations: &[f64]) -> f64 {
    sample_variance(observations).sqrt()
}

fn population_standard_deviation(observations: &[f64]) -> f64 {
    population_variance(observations).sqrt()
}

pub fn pearson_correlation(x: &[f64], y: &[f64]) -> f64 {
    let mean_x = mean(x);
    let mean_y = mean(y);
    let n = x.len();
    let numerator: f64 = (0..n).map(|i| (x[i] - mean_x) * (y[i] - mean_y)).sum();
    let denominator_x: f64 = (0..n).map(|i| (x[i] - mean_x).powi(2)).sum::<f64>().sqrt();
    let denominator_y: f64 = (0..n).map(|i| (y[i] - mean_y).powi(2)).sum::<f64>().sqrt();
    numerator / (denominator_x * denominator_y)
}

/// Conversion of spot prices to log returns.
pub fn to_log_returns(observations: &[f64]) -> Vec<f64> {
    observations
        .iter()
        .skip(1)
        .zip(observations.iter())
        .map(|(a, b)| (a / b - 1.0).ln())
        .collect()
}

fn difference(observations: &Array1<f64>) -> Array1<f64> {
    observations
        .iter()
        .skip(1)
        .zip(observations.iter())
        .map(|(a, b)| (a - b))
        .collect()
}

fn prepare_augmented_dickey_fuller(y: &Array1<f64>, max_lags: usize) -> (Array2<f64>, Array1<f64>) {
    let dy = difference(y);
    let n = dy.len() - max_lags;

    let mut x = Array2::<f64>::ones((n, 1));
    let y_lag = y
        .slice(s![max_lags - 1..y.len() - 1])
        .to_owned()
        .insert_axis(ndarray::Axis(1));
    x = ndarray::concatenate![ndarray::Axis(1), x, y_lag];

    for i in 1..=max_lags {
        let slice = dy.slice(s![max_lags - i..dy.len() - i]).to_owned();
        let lagged = slice.insert_axis(ndarray::Axis(1));
        x = ndarray::concatenate![ndarray::Axis(1), x, lagged];
    }

    let y_target = dy.slice(s![max_lags..]).to_owned();

    (x, y_target)
}

fn ols_beta(x: &Array2<f64>, y: &Array1<f64>) -> (Array1<f64>, f64) {
    let xtx = x.t().dot(x);
    let xtx_inv = xtx.inv().unwrap();
    let xty = x.t().dot(y);
    let beta = xtx_inv.dot(&xty);

    let residuals = y - &x.dot(&beta);
    let sigma2 = residuals.mapv(|e| e.powi(2)).sum() / (y.len() as f64 - x.shape()[1] as f64);
    let se_squared = xtx_inv[(1, 1)] * sigma2;
    let se = se_squared.sqrt();

    (beta, se)
}

pub fn augmented_dickey_fuller_statistic(y: &Array1<f64>, max_lags: usize) -> f64 {
    let (x, y_diff) = prepare_augmented_dickey_fuller(y, max_lags);
    let (beta, se) = ols_beta(&x, &y_diff);
    let gamma = beta[1];
    gamma / se
}

pub fn interpret_adf(t_stat: f64) {
    println!("ADF Test Statistic: {:.4}", t_stat);
    println!("Critical Values (approx):");
    println!("  1%: -3.43");
    println!("  5%: -2.86");
    println!(" 10%: -2.57");

    if t_stat < -3.43 {
        println!("=> Reject H₀: Series is stationary (1% level)");
    } else if t_stat < -2.86 {
        println!("=> Reject H₀: Series is stationary (5% level)");
    } else if t_stat < -2.57 {
        println!("=> Weak evidence against H₀ (10% level)");
    } else {
        println!("=> Fail to reject H₀: Series is non-stationary");
    }
}
