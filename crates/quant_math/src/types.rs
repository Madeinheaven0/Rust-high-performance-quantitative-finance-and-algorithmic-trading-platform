use crate::errors::AnalyticsError;
use ndarray::{Array1, Array2};

/// Validated (T x N) price matrix with T >= 2, N >= 1, and strictly positive finite prices.
#[derive(Debug, Clone, PartialEq)]
pub struct PriceMatrix(pub Array2<f64>);

impl PriceMatrix {
    /// Creates a new `PriceMatrix` after validating dimensions and price values.
    pub fn new(array: Array2<f64>) -> Result<Self, AnalyticsError> {
        let (rows, cols) = array.dim();
        if rows < 2 {
            return Err(AnalyticsError::InvalidStepNumber(rows));
        }
        if cols < 1 {
            return Err(AnalyticsError::InvalidStepNumber(cols));
        }
        if array.iter().any(|&val| !val.is_finite() || val <= 0.0) {
            return Err(AnalyticsError::InvalidPrice);
        }
        Ok(Self(array))
    }

    /// Returns the number of time observations (`T`).
    pub fn n_rows(&self) -> usize {
        self.0.nrows()
    }

    /// Returns the number of assets or series (`N`).
    pub fn n_cols(&self) -> usize {
        self.0.ncols()
    }

    /// Returns a shared reference to the underlying `Array2<f64>`.
    pub fn as_array(&self) -> &Array2<f64> {
        &self.0
    }
}

/// Validated weight vector of length `N` with finite entries.
#[derive(Debug, Clone, PartialEq)]
pub struct Weights(pub Array1<f64>);

impl Weights {
    /// Creates a new `Weights` vector after validating non-empty length and finiteness.
    pub fn new(array: Array1<f64>) -> Result<Self, AnalyticsError> {
        if array.is_empty() {
            return Err(AnalyticsError::InvalidStepNumber(0));
        }
        if array.iter().any(|&w| !w.is_finite()) {
            return Err(AnalyticsError::NoInfiniteValue);
        }
        Ok(Self(array))
    }

    /// Creates an equally weighted vector of length `n` (`1.0 / n` per asset).
    pub fn equal_weights(n: usize) -> Result<Self, AnalyticsError> {
        if n < 1 {
            return Err(AnalyticsError::InvalidStepNumber(n));
        }
        let w = 1.0 / (n as f64);
        Ok(Self(Array1::from_elem(n, w)))
    }

    /// Returns the number of weights.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns a shared reference to the underlying `Array1<f64>`.
    pub fn as_array(&self) -> &Array1<f64> {
        &self.0
    }
}

/// Validated normal quantile `z_alpha` (strictly positive and finite).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConfidenceZ(pub f64);

impl ConfidenceZ {
    /// Creates a new `ConfidenceZ` value.
    pub fn new(z: f64) -> Result<Self, AnalyticsError> {
        if !z.is_finite() {
            return Err(AnalyticsError::NoInfiniteValue);
        }
        if z <= 0.0 {
            return Err(AnalyticsError::InvalidRangeValue(z));
        }
        Ok(Self(z))
    }

    /// Returns the quantile value.
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// Validated confidence level `alpha` in `(0.0, 1.0)`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConfidenceAlpha(pub f64);

impl ConfidenceAlpha {
    /// Creates a new `ConfidenceAlpha` value strictly inside `(0.0, 1.0)`.
    pub fn new(alpha: f64) -> Result<Self, AnalyticsError> {
        if !alpha.is_finite() {
            return Err(AnalyticsError::NoInfiniteValue);
        }
        if alpha <= 0.0 || alpha >= 1.0 {
            return Err(AnalyticsError::InvalidRangeValue(alpha));
        }
        Ok(Self(alpha))
    }
    
    pub fn value(&self) -> f64 {
        self.0
    }

    /// Returns the tail probability `1.0 - alpha`.
    pub fn tail_probability(&self) -> f64 {
        1.0 - self.0
    }
}

/// Validated time horizon `h` (strictly positive and finite).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Horizon(pub f64);

impl Horizon {
    /// Creates a new `Horizon` value.
    pub fn new(h: f64) -> Result<Self, AnalyticsError> {
        if !h.is_finite() {
            return Err(AnalyticsError::NoInfiniteValue);
        }
        if h <= 0.0 {
            return Err(AnalyticsError::InvalidRangeValue(h));
        }
        Ok(Self(h))
    }

    /// Returns `sqrt(h)` for volatility scaling.
    pub fn sqrt(&self) -> f64 {
        self.0.sqrt()
    }
}

/// Validated `(T - 1) x N` log-return matrix with finite entries.
#[derive(Debug, Clone, PartialEq)]
pub struct LogReturns(pub Array2<f64>);

impl LogReturns {
    /// Creates a new `LogReturns` matrix after checking dimensions and finiteness.
    pub fn new(array: Array2<f64>) -> Result<Self, AnalyticsError> {
        let (rows, cols) = array.dim();
        if rows < 1 {
            return Err(AnalyticsError::InvalidStepNumber(rows));
        }
        if cols < 1 {
            return Err(AnalyticsError::InvalidStepNumber(cols));
        }
        if array.iter().any(|&r| !r.is_finite()) {
            return Err(AnalyticsError::NoInfiniteValue);
        }
        Ok(Self(array))
    }

    /// Returns a shared reference to the underlying `Array2<f64>`.
    pub fn as_array(&self) -> &Array2<f64> {
        &self.0
    }
}

/// Validated square `(N x N)` covariance matrix with finite entries.
#[derive(Debug, Clone, PartialEq)]
pub struct CovarianceMatrix(pub Array2<f64>);

impl CovarianceMatrix {
    /// Creates a new square `CovarianceMatrix` after validating shape and finiteness.
    pub fn new(array: Array2<f64>) -> Result<Self, AnalyticsError> {
        let (rows, cols) = array.dim();
        if rows < 1 || cols < 1 {
            return Err(AnalyticsError::InvalidStepNumber(rows.min(cols)));
        }
        if rows != cols {
            return Err(AnalyticsError::DimensionMismatch {
                expected: rows,
                actual: cols,
            });
        }
        if array.iter().any(|&val| !val.is_finite()) {
            return Err(AnalyticsError::NoInfiniteValue);
        }
        Ok(Self(array))
    }

    /// Returns the dimension `N` of the square matrix.
    pub fn size(&self) -> usize {
        self.0.nrows()
    }

    /// Returns a shared reference to the underlying `Array2<f64>`.
    pub fn as_array(&self) -> &Array2<f64> {
        &self.0
    }
}

/// Validated non-negative finite portfolio variance.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PortfolioVariance(pub f64);

impl PortfolioVariance {
    /// Creates a new `PortfolioVariance`.
    pub fn new(value: f64) -> Result<Self, AnalyticsError> {
        if !value.is_finite() {
            return Err(AnalyticsError::NoInfiniteValue);
        }
        if value < 0.0 {
            return Err(AnalyticsError::NegativeValue);
        }
        Ok(Self(value))
    }

    /// Returns the variance value.
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// Validated non-negative finite portfolio volatility (`sqrt(variance)`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PortfolioVolatility(pub f64);

impl PortfolioVolatility {
    /// Creates a new `PortfolioVolatility`.
    pub fn new(value: f64) -> Result<Self, AnalyticsError> {
        if !value.is_finite() {
            return Err(AnalyticsError::NoInfiniteValue);
        }
        if value < 0.0 {
            return Err(AnalyticsError::NegativeValue);
        }
        Ok(Self(value))
    }

    /// Returns the volatility value.
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// Validated non-negative finite parametric Value at Risk.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ParametricVar(pub f64);

impl ParametricVar {
    /// Creates a new `ParametricVar`.
    pub fn new(value: f64) -> Result<Self, AnalyticsError> {
        if !value.is_finite() {
            return Err(AnalyticsError::NoInfiniteValue);
        }
        if value < 0.0 {
            return Err(AnalyticsError::NegativeValue);
        }
        Ok(Self(value))
    }

    /// Returns the VaR value.
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// Validated non-negative finite Gaussian Expected Shortfall.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExpectedShortfall(pub f64);

impl ExpectedShortfall {
    /// Creates a new `ExpectedShortfall`.
    pub fn new(value: f64) -> Result<Self, AnalyticsError> {
        if !value.is_finite() {
            return Err(AnalyticsError::NoInfiniteValue);
        }
        if value < 0.0 {
            return Err(AnalyticsError::NegativeValue);
        }
        Ok(Self(value))
    }

    /// Returns the Expected Shortfall value.
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// Risk parameters shared across VaR and Expected Shortfall calculations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RiskParams {
    pub confidence_z: ConfidenceZ,
    pub confidence_alpha: ConfidenceAlpha,
    pub horizon: Horizon,
}

/// Computed risk analytics output for a matrix and weight vector.
#[derive(Debug, Clone, PartialEq)]
pub struct RiskMetrics {
    pub covariance: CovarianceMatrix,
    pub variance: PortfolioVariance,
    pub volatility: PortfolioVolatility,
    pub var: ParametricVar,
    pub expected_shortfall: ExpectedShortfall,
}
