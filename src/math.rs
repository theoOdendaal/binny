use nalgebra::{DMatrix, DVector};

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

pub struct OLSRegression {}

pub fn compute_residuals(y: &[f64], x: &[f64]) -> Option<Vec<f64>> {
    if y.len() != x.len() || y.len() < 2 {
        return None;
    }

    let n = y.len();

    // Construct design matrix with a column of ones (for intercept) and x values
    let mut data = Vec::with_capacity(n * 2);
    for &xi in x {
        data.push(1.0);
        data.push(xi);
    }

    let x_matrix = DMatrix::from_row_slice(n, 2, &data);
    let y_vector = DVector::from_row_slice(y);

    // OLS: beta = (X^T.X)^(-1).(X^T).(y)
    let xtx = x_matrix.transpose() * &x_matrix;
    let xtx_inv = xtx.try_inverse()?;
    let xty = x_matrix.transpose() * y_vector.clone();
    let beta = xtx_inv * xty;

    // Residuals: y - X.b
    let predicted = x_matrix * beta;
    let residuals = y_vector - predicted;

    Some(residuals.iter().copied().collect())
}

pub struct AugmentedDicketFuller {}

impl AugmentedDicketFuller {
    /// Construct y delta's and lagged variables given y-coordinates,
    /// used for Augmented Dickey Fuller regression.
    fn generate_variables(y: &[f64], max_lags: usize) -> (Vec<Vec<f64>>, Vec<f64>) {
        // Requires at least one lag, in order to compensate for change in y calculation
        // reducing y length with 1.
        let l = max_lags.max(1);
        let n = y.len() - l;

        // y delta's
        // Assumes the collection is sorted from oldest to
        // newest.
        let dy: Vec<f64> = y.windows(2).map(|w| w[1] - w[0]).collect();
        let dy = dy[l - 1..].to_vec();

        // Lagged y's
        let mut ly = Vec::new();
        for i in (0..(max_lags)).rev() {
            ly.push(y[i..(n + i)].to_vec());
        }

        (ly, dy)
    }

    // Matrix friendly Ordinary Least Squares (OLS) regression
    // Returns
    fn ols_beta(x: &DMatrix<f64>, y: &DVector<f64>) -> Option<(DVector<f64>, f64)> {
        let n = x.nrows() as f64;
        let k = x.ncols() as f64;

        // (X^T X)
        let xtx = x.transpose() * x;
        // Try inverse
        let xtx_inv = xtx.try_inverse()?;

        // (X^T y)
        let xty = x.transpose() * y;

        // beta = (X^T X)^-1 X^T y
        let beta = &xtx_inv * xty;

        // residuals = y - X beta
        let residuals = y - &(x * &beta);

        // sigma^2 = RSS / (n - k)
        let rss = residuals.dot(&residuals);
        let sigma2 = rss / (n - k);

        // Standard error for coefficient at index 1
        let se_squared = xtx_inv[(1, 1)] * sigma2;
        let se = se_squared.sqrt();

        Some((beta, se))
    }

    /// Calculates ADF gamma.
    pub fn statistic(y: &[f64], max_lags: usize) -> Option<f64> {
        let (x, y) = Self::generate_variables(y, max_lags);
        if let Some((beta, se)) = Self::ols_beta(
            &DMatrix::from_vec(x.len(), x[0].len(), x.concat()),
            &DVector::from_vec(y),
        ) {
            return Some(beta[1] / se);
        }
        None
    }
}

/*


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
*/
