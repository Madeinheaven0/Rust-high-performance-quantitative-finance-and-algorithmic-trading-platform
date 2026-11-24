use crate::errors::AnalyticsError;
use crate::types::{
    ConfidenceZ, CovarianceMatrix, ExpectedShortfall, Horizon, LogReturns, ParametricVar,
    PortfolioVariance, PortfolioVolatility, PriceMatrix, RiskMetrics, RiskParams, Weights, ConfidenceAlpha
};
use ndarray::linalg::Dot;
use ndarray::{Array2, Axis, array};
use std::f64::consts::{PI, SQRT_2};


/// Approximates the Gauss error function `erf(x)` in pure stable Rust
/// (Abramowitz & Stegun 7.1.26, max error ~1.5e-7).
fn erf_approx(x: f64) -> f64 {
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let abs_x = x.abs();

    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;

    let t = 1.0 / (1.0 + p * abs_x);
    let poly = (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t;
    let y = 1.0 - poly * (-abs_x * abs_x).exp();

    sign * y
}

/// Standard normal cumulative distribution function `Phi(z) = 0.5 * (1 + erf(z / sqrt(2)))`.
pub fn normal_cdf(z: f64) -> f64 {
    0.5 * (1.0 + erf_approx(z / SQRT_2))
}

/// Standard normal probability density function `phi(z) = (1 / sqrt(2*pi)) * exp(-z^2 / 2)`.
fn normal_pdf(z: f64) -> f64 {
    let norm_const = 1.0 / ((2.0 * PI).sqrt());
    norm_const * (-0.5 * z.powi(2)).exp()
}

/// Validates that `z` and `alpha` correspond to the same normal confidence level within `tolerance`.
pub fn validate_confidence_consistency(
    z: ConfidenceZ,
    alpha: ConfidenceAlpha,
    tolerance: f64,
) -> Result<(), AnalyticsError> {
    if !tolerance.is_finite() || tolerance <= 0.0 {
        return Err(AnalyticsError::InvalidRangeValue(tolerance));
    }

    let implied_alpha = normal_cdf(z.value());
    let diff = (implied_alpha - alpha.value()).abs();
    if diff > tolerance {
        return Err(AnalyticsError::InvalidRangeValue(diff));
    }

    Ok(())
}

/// Computes `(T - 1) x N` log-returns `r_{t,i} = ln(P_{t,i} / P_{t-1,i})` from a `PriceMatrix`.
pub fn log_returns(prices: &PriceMatrix) -> Result<LogReturns, AnalyticsError> {
    let prices = prices.as_array();
    let curr = prices.slice(ndarray::s![1.., ..]);
    let prev = prices.slice(ndarray::s![..-1, ..]);

    // Element-wise log ratio: ln(P_t / P_{t-1}) = ln(P_t) - ln(P_{t-1}).
    let ratio = &curr / &prev;
    let returns: Array2<f64> = ratio.mapv(f64::ln);

    LogReturns::new(returns)
}

/// Computes the unbiased `(N x N)` covariance matrix `Sigma = (1 / (T - 2)) * R_tilde^T * R_tilde`.
pub fn covariance_matrix(returns: LogReturns) -> Result<CovarianceMatrix, AnalyticsError> {
    let returns = returns.as_array();
    let m = returns.nrows();
    let n = returns.ncols();

    if m < 2 {
        return Err(AnalyticsError::InsufficientObservations {
            available: m,
            required: 2,
        });
    }

    let r_mean = returns
        .mean_axis(Axis(0))
        .ok_or(AnalyticsError::InvalidStepNumber(m))?;
    let centered = returns - &r_mean.insert_axis(Axis(0));
    let dof = (m - 1) as f64;

    let cov_arr = centered.t().dot(&centered) / dof;

    CovarianceMatrix::new(cov_arr)
}

/// Computes portfolio variance `sigma_p^2 = w^T * Sigma * w`.
pub fn portfolio_variance(
    weights: Weights,
    cov: CovarianceMatrix,
) -> Result<PortfolioVariance, AnalyticsError> {
    let n = cov.size();

    if weights.len() != n {
        return Err(AnalyticsError::DimensionMismatch {
            expected: n,
            actual: weights.len(),
        });
    }

    let sigma = cov.as_array().dot(weights.as_array());
    let var = weights.as_array().t().dot(&sigma);

    PortfolioVariance::new(var)
}

/// Computes portfolio volatility `sigma_p = sqrt(sigma_p^2)`.
pub fn portfolio_volatility(
    variance: PortfolioVariance,
) -> Result<PortfolioVolatility, AnalyticsError> {
    PortfolioVolatility::new(variance.value().sqrt())
}

/// Computes parametric Value at Risk `VaR = z_alpha * sigma_p * sqrt(h)`.
pub fn parametric_var(
    sigma: PortfolioVolatility,
    z: ConfidenceZ,
    h: Horizon,
) -> Result<ParametricVar, AnalyticsError> {
    let var = z.value() * sigma.value() * h.sqrt();
    ParametricVar::new(var)
}



/// Computes Gaussian Expected Shortfall `ES = sigma_p * sqrt(h) * phi(z_alpha) / (1 - alpha)`.
pub fn expected_shortfall(
    sigma: PortfolioVolatility,
    risk_params: RiskParams,
) -> Result<ExpectedShortfall, AnalyticsError> {
    let tail_prob = risk_params.confidence_alpha.tail_probability();
    if tail_prob <= 0.0 || !tail_prob.is_finite() {
        return Err(AnalyticsError::InvalidRangeValue(tail_prob));
    }

    let phi = normal_pdf(risk_params.confidence_z.value());
    let shortfall = sigma.value() * risk_params.horizon.sqrt() * (phi / tail_prob);

    ExpectedShortfall::new(shortfall)
}

/// Assembles `RiskMetrics` after verifying `confidence_z` and `confidence_alpha` consistency.
pub fn metrics_risk(
    covariance: CovarianceMatrix,
    variance: PortfolioVariance,
    volatility: PortfolioVolatility,
    var: ParametricVar,
    expected_shortfall: ExpectedShortfall,
    risk_params: RiskParams,
) -> Result<RiskMetrics, AnalyticsError> {
    validate_confidence_consistency(
        risk_params.confidence_z,
        risk_params.confidence_alpha,
        1e-2,
    )?;

    Ok(RiskMetrics {
        covariance,
        variance,
        volatility,
        var,
        expected_shortfall,
    })
}


#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{array, Array1, Array2};
    use proptest::prelude::*;
    use rstest::rstest;

    fn default_params() -> RiskParams {
        RiskParams {
            confidence_z: ConfidenceZ::new(1.6448536269514722).unwrap(),
            confidence_alpha: ConfidenceAlpha::new(0.95).unwrap(),
            horizon: Horizon::new(1.0).unwrap(),
        }
    }

    // --- 1. Standard Unit Tests ---

    #[test]
    fn price_matrix_rejects_non_positive_and_non_finite_values() {
        let zero_price = array![[100.0, 200.0], [0.0, 202.0]];
        assert_eq!(
            PriceMatrix::new(zero_price),
            Err(AnalyticsError::InvalidPrice)
        );

        let nan_price = array![[100.0, f64::NAN], [101.0, 202.0]];
        assert_eq!(
            PriceMatrix::new(nan_price),
            Err(AnalyticsError::InvalidPrice)
        );

        let too_few_rows = array![[100.0, 200.0]];
        assert_eq!(
            PriceMatrix::new(too_few_rows),
            Err(AnalyticsError::InvalidStepNumber(1))
        );
    }

    #[test]
    fn confidence_alpha_enforces_open_unit_interval() {
        assert_eq!(
            ConfidenceAlpha::new(0.0),
            Err(AnalyticsError::InvalidRangeValue(0.0))
        );
        assert_eq!(
            ConfidenceAlpha::new(1.0),
            Err(AnalyticsError::InvalidRangeValue(1.0))
        );
        assert_eq!(
            ConfidenceAlpha::new(f64::INFINITY),
            Err(AnalyticsError::NoInfiniteValue)
        );
    }

    #[test]
    fn covariance_rejects_insufficient_return_observations() {
        // T = 2 prices yield only M = 1 log-return row, while unbiased covariance needs M >= 2.
        let prices = PriceMatrix::new(array![[100.0, 50.0], [102.0, 51.0]]).unwrap();
        let returns = log_returns(&prices).unwrap();

        assert_eq!(
            covariance_matrix(returns),
            Err(AnalyticsError::InsufficientObservations {
                available: 1,
                required: 2,
            })
        );
    }

    #[test]
    fn portfolio_variance_checks_weight_dimension_mismatch() {
        let prices = PriceMatrix::new(array![
            [100.0, 200.0],
            [101.0, 198.0],
            [103.0, 202.0],
        ])
            .unwrap();
        let returns = log_returns(&prices).unwrap();
        let cov = covariance_matrix(returns).unwrap();
        let bad_weights = Weights::new(array![1.0]).unwrap();

        assert_eq!(
            portfolio_variance(bad_weights, cov),
            Err(AnalyticsError::DimensionMismatch {
                expected: 2,
                actual: 1,
            })
        );
    }

    #[test]
    fn confidence_consistency_accepts_matching_and_rejects_mismatched_pairs() {
        let z_95 = ConfidenceZ::new(1.645).unwrap();
        let alpha_95 = ConfidenceAlpha::new(0.95).unwrap();
        assert!(validate_confidence_consistency(z_95, alpha_95, 1e-3).is_ok());

        let z_99 = ConfidenceZ::new(2.326).unwrap();
        assert!(validate_confidence_consistency(z_99, alpha_95, 1e-3).is_err());
    }

    #[test]
    fn full_pipeline_computes_consistent_risk_metrics() {
        let prices = PriceMatrix::new(array![
            [100.0, 200.0],
            [102.0, 204.0],
            [101.0, 203.0],
            [104.0, 208.0],
        ])
            .unwrap();
        let weights = Weights::equal_weights(2).unwrap();
        let params = default_params();

        let returns = log_returns(&prices).unwrap();
        let cov = covariance_matrix(returns).unwrap();
        let variance = portfolio_variance(weights, cov.clone()).unwrap();
        let volatility = portfolio_volatility(variance).unwrap();
        let var = parametric_var(volatility, params.confidence_z, params.horizon).unwrap();
        let es = expected_shortfall(volatility, params).unwrap();
        let metrics = metrics_risk(cov, variance, volatility, var, es, params).unwrap();

        assert_eq!(metrics.covariance.size(), 2);
        assert!(metrics.variance.value() >= 0.0);
        assert!((metrics.volatility.value() - metrics.variance.value().sqrt()).abs() < 1e-12);
        assert!(metrics.expected_shortfall.value() > metrics.var.value());
    }

    // --- 2. Parameterized Tests with rstest ---

    #[rstest]
    #[case(0.90, 1.2815515655446004, 1.0)]
    #[case(0.95, 1.6448536269514722, 1.0)]
    #[case(0.95, 1.6448536269514722, 4.0)]
    #[case(0.99, 2.3263478740408408, 10.0)]
    fn var_and_es_scale_with_horizon_and_quantile(
        #[case] alpha: f64,
        #[case] z: f64,
        #[case] horizon: f64,
    ) {
        let vol = PortfolioVolatility::new(0.02).unwrap();
        let cz = ConfidenceZ::new(z).unwrap();
        let ca = ConfidenceAlpha::new(alpha).unwrap();
        let h = Horizon::new(horizon).unwrap();
        let params = RiskParams {
            confidence_z: cz,
            confidence_alpha: ca,
            horizon: h,
        };

        let var: ParametricVar = parametric_var(vol, cz, h).unwrap();
        let es: ExpectedShortfall = expected_shortfall(vol, params).unwrap();

        let expected_var = z * 0.02 * horizon.sqrt();
        assert!((var.value() - expected_var).abs() < 1e-12);
        assert!(es.value() > var.value());
        assert!(es.value().is_finite());
    }

    #[rstest]
    #[case(3, 1)]
    #[case(5, 2)]
    #[case(10, 4)]
    fn equal_weights_matches_asset_count(#[case] n_rows: usize, #[case] n_cols: usize) {
        let prices = PriceMatrix::new(Array2::from_elem((n_rows, n_cols), 100.0)).unwrap();
        let weights = Weights::equal_weights(n_cols).unwrap();
        let returns: LogReturns = log_returns(&prices).unwrap();

        assert_eq!(returns.as_array().dim(), (n_rows - 1, n_cols));
        assert_eq!(weights.len(), n_cols);
    }

    // --- 3. Property-Based Tests with proptest ---

    fn price_matrix_strategy() -> impl Strategy<Value = PriceMatrix> {
        (3usize..=12, 1usize..=5).prop_flat_map(|(rows, cols)| {
            prop::collection::vec(0.01f64..10_000.0f64, rows * cols).prop_map(move |data| {
                let arr = Array2::from_shape_vec((rows, cols), data).unwrap();
                PriceMatrix::new(arr).unwrap()
            })
        })
    }

    proptest! {
        #[test]
        fn prop_covariance_is_square_symmetric_and_risk_is_non_negative(
            prices in price_matrix_strategy(),
            horizon in 0.1f64..30.0f64,
        ) {
            let cols = prices.n_cols();
            let weights = Weights::equal_weights(cols).unwrap();
            let params = RiskParams {
                confidence_z: ConfidenceZ::new(1.6448536269514722).unwrap(),
                confidence_alpha: ConfidenceAlpha::new(0.95).unwrap(),
                horizon: Horizon::new(horizon).unwrap(),
            };

            let returns = log_returns(&prices).unwrap();
            prop_assert_eq!(returns.as_array().dim(), (prices.n_rows() - 1, cols));
            prop_assert!(returns.as_array().iter().all(|v| v.is_finite()));

            let cov = covariance_matrix(returns).unwrap();
            let cov_arr = cov.as_array();
            prop_assert_eq!(cov_arr.dim(), (cols, cols));

            // Symmetry check: Sigma[i, j] == Sigma[j, i].
            for i in 0..cols {
                for j in 0..cols {
                    prop_assert!((cov_arr[[i, j]] - cov_arr[[j, i]]).abs() < 1e-10);
                }
            }

            let var_p: PortfolioVariance = portfolio_variance(weights, cov).unwrap();
            let vol_p: PortfolioVolatility = portfolio_volatility(var_p).unwrap();
            let var_metric = parametric_var(vol_p, params.confidence_z, params.horizon).unwrap();
            let es_metric = expected_shortfall(vol_p, params).unwrap();

            prop_assert!(var_p.value().is_finite() && var_p.value() >= 0.0);
            prop_assert!(vol_p.value().is_finite() && vol_p.value() >= 0.0);
            prop_assert!(var_metric.value().is_finite() && var_metric.value() >= 0.0);
            prop_assert!(es_metric.value().is_finite() && es_metric.value() >= 0.0);
            prop_assert!(es_metric.value() >= var_metric.value());
        }

        #[test]
        fn prop_four_x_horizon_doubles_var_and_es(
            base_vol in 0.0001f64..0.5f64,
            h in 0.25f64..20.0f64,
        ) {
            let vol = PortfolioVolatility::new(base_vol).unwrap();
            let cz = ConfidenceZ::new(1.6448536269514722).unwrap();
            let ca = ConfidenceAlpha::new(0.95).unwrap();

            let params_1 = RiskParams {
                confidence_z: cz,
                confidence_alpha: ca,
                horizon: Horizon::new(h).unwrap(),
            };
            let params_4 = RiskParams {
                confidence_z: cz,
                confidence_alpha: ca,
                horizon: Horizon::new(4.0 * h).unwrap(),
            };

            let var_1 = parametric_var(vol, cz, params_1.horizon).unwrap().value();
            let var_4 = parametric_var(vol, cz, params_4.horizon).unwrap().value();
            let es_1 = expected_shortfall(vol, params_1).unwrap().value();
            let es_4 = expected_shortfall(vol, params_4).unwrap().value();

            // Doubling sqrt(h) via 4*h scales linear risk measures by 2.
            prop_assert!((var_4 - 2.0 * var_1).abs() < 1e-10);
            prop_assert!((es_4 - 2.0 * es_1).abs() < 1e-10);
        }

        #[test]
        fn prop_custom_weights_preserve_finite_variance(
            prices in price_matrix_strategy(),
            raw_weights in prop::collection::vec(-5.0f64..5.0f64, 1..=5),
        ) {
            let cols = prices.n_cols();
            let mut w_vec = raw_weights;
            w_vec.resize(cols, 0.2);
            let weights = Weights::new(Array1::from_vec(w_vec)).unwrap();

            let returns = log_returns(&prices).unwrap();
            let cov = covariance_matrix(returns).unwrap();
            let var_p = portfolio_variance(weights, cov).unwrap();

            prop_assert!(var_p.value().is_finite());
        }
    }
}