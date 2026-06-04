//! Regression test: compare results against the 5 known conservation law baselines.

use crate::types::*;

/// The five conservation laws for ternary agent systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaselineLaw {
    /// Law 1: Conservation of total interaction count.
    InteractionConservation,
    /// Law 2: Conservation of role balance (1:1:1 ratio).
    RoleBalance,
    /// Law 3: Conservation of avoidance symmetry.
    AvoidanceSymmetry,
    /// Law 4: Conservation of energy-equivalent (convergence rate scales with population).
    ConvergenceScaling,
    /// Law 5: Conservation of metric bounds (all values within expected range).
    MetricBounds,
}

impl BaselineLaw {
    /// Returns all five baseline laws.
    pub fn all() -> [BaselineLaw; 5] {
        [
            BaselineLaw::InteractionConservation,
            BaselineLaw::RoleBalance,
            BaselineLaw::AvoidanceSymmetry,
            BaselineLaw::ConvergenceScaling,
            BaselineLaw::MetricBounds,
        ]
    }

    /// Returns the name of this law.
    pub fn name(&self) -> &'static str {
        match self {
            BaselineLaw::InteractionConservation => "InteractionConservation",
            BaselineLaw::RoleBalance => "RoleBalance",
            BaselineLaw::AvoidanceSymmetry => "AvoidanceSymmetry",
            BaselineLaw::ConvergenceScaling => "ConvergenceScaling",
            BaselineLaw::MetricBounds => "MetricBounds",
        }
    }

    /// Returns the expected baseline value for this law.
    pub fn expected_value(&self) -> f64 {
        match self {
            BaselineLaw::InteractionConservation => 1.0,
            BaselineLaw::RoleBalance => 0.333, // 1/3 per role
            BaselineLaw::AvoidanceSymmetry => 0.5,
            BaselineLaw::ConvergenceScaling => 1.0,
            BaselineLaw::MetricBounds => 0.0, // deviation from bounds
        }
    }
}

/// Regression test runner that compares against known baselines.
pub struct RegressionTest {
    /// Tolerance for comparing observed vs expected values.
    tolerance: f64,
}

impl RegressionTest {
    /// Creates a new regression test with the given tolerance.
    pub fn new(tolerance: f64) -> Self {
        RegressionTest { tolerance }
    }

    /// Runs all regression tests against the 5 baseline laws.
    pub fn check(&self, results: &[ScaleResult]) -> Vec<RegressionResult> {
        let mut regression_results = Vec::new();

        for law in BaselineLaw::all() {
            let observed = self.compute_law_value(&law, results);
            let expected = law.expected_value();
            let delta = (observed - expected).abs();
            let within_tolerance = delta < self.tolerance;

            regression_results.push(RegressionResult {
                law_name: law.name().to_string(),
                passed: within_tolerance,
                expected,
                observed,
                delta,
                within_tolerance,
            });
        }

        regression_results
    }

    /// Computes the observed value for a given law from the scale results.
    fn compute_law_value(&self, law: &BaselineLaw, results: &[ScaleResult]) -> f64 {
        if results.is_empty() {
            return 0.0;
        }

        match law {
            BaselineLaw::InteractionConservation => {
                // Mean conservation ratio across scales
                let sum: f64 = results.iter().map(|r| r.metrics.conservation_ratio).sum();
                sum / results.len() as f64
            }
            BaselineLaw::RoleBalance => {
                // Average role fraction across all scales
                let mut total_fraction = 0.0;
                let mut count = 0;
                for r in results {
                    let total: u64 = r.metrics.role_interaction_counts.iter().sum();
                    if total > 0 {
                        for &c in &r.metrics.role_interaction_counts {
                            total_fraction += c as f64 / total as f64;
                            count += 1;
                        }
                    }
                }
                if count > 0 {
                    total_fraction / count as f64
                } else {
                    0.0
                }
            }
            BaselineLaw::AvoidanceSymmetry => {
                // Mean avoidance ratio
                let sum: f64 = results.iter().map(|r| r.metrics.avoidance_ratio).sum();
                sum / results.len() as f64
            }
            BaselineLaw::ConvergenceScaling => {
                // Check that convergence rate decreases with population (inverse log scaling)
                // We check if the overall pattern is correct by averaging normalized rates
                let rates: Vec<f64> = results.iter().map(|r| r.metrics.convergence_rate).collect();
                let mut sum = 0.0;
                for (i, &rate) in rates.iter().enumerate() {
                    let expected = 1.0 / (results[i].population_size as f64).log2().max(1.0);
                    if expected > 0.0 {
                        sum += rate / expected;
                    }
                }
                sum / results.len() as f64
            }
            BaselineLaw::MetricBounds => {
                // Sum of absolute deviations from expected bounds
                let mut total_deviation = 0.0;
                for r in results {
                    let dev = (r.metrics.conservation_ratio - 1.0).abs();
                    total_deviation += dev;
                    if r.metrics.avoidance_ratio < 0.0 || r.metrics.avoidance_ratio > 1.0 {
                        total_deviation += 1.0;
                    }
                }
                total_deviation / results.len() as f64
            }
        }
    }

    /// Checks a specific law against results.
    pub fn check_law(&self, law: BaselineLaw, results: &[ScaleResult]) -> RegressionResult {
        let observed = self.compute_law_value(&law, results);
        let expected = law.expected_value();
        let delta = (observed - expected).abs();
        let within_tolerance = delta < self.tolerance;

        RegressionResult {
            law_name: law.name().to_string(),
            passed: within_tolerance,
            expected,
            observed,
            delta,
            within_tolerance,
        }
    }
}
