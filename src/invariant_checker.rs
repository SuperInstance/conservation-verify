//! Invariant checker: verify that invariants hold across all scales.

use crate::types::*;
use crate::{mean, std_dev};

/// Checks that various invariants hold across simulation results.
pub struct InvariantChecker {
    /// Tolerance for conservation ratio.
    conservation_tolerance: f64,
    /// Threshold for avoidance ratio standard deviation.
    avoidance_std_threshold: f64,
    /// Threshold for convergence rate monotonicity check.
    convergence_threshold: f64,
}

impl InvariantChecker {
    /// Creates a new invariant checker with the given conservation tolerance.
    pub fn new(conservation_tolerance: f64) -> Self {
        InvariantChecker {
            conservation_tolerance,
            avoidance_std_threshold: 0.05,
            convergence_threshold: 0.1,
        }
    }

    /// Sets the avoidance ratio std dev threshold.
    pub fn with_avoidance_threshold(mut self, threshold: f64) -> Self {
        self.avoidance_std_threshold = threshold;
        self
    }

    /// Sets the convergence rate threshold.
    pub fn with_convergence_threshold(mut self, threshold: f64) -> Self {
        self.convergence_threshold = threshold;
        self
    }

    /// Runs all invariant checks and returns results.
    pub fn check_all(&self, results: &[ScaleResult]) -> Vec<InvariantResult> {
        let mut checks = Vec::new();

        // Invariant 1: Conservation ratio at each scale
        for r in results {
            checks.push(InvariantResult {
                name: format!("conservation_ratio@{}", r.population_size),
                passed: r.metrics.conservation_holds(self.conservation_tolerance),
                observed_value: r.metrics.conservation_ratio,
                threshold: self.conservation_tolerance,
                population_size: r.population_size,
            });
        }

        // Invariant 2: Avoidance ratio std dev across scales
        let avoidance: Vec<f64> = results.iter().map(|r| r.metrics.avoidance_ratio).collect();
        let avoidance_std = std_dev(&avoidance);
        checks.push(InvariantResult {
            name: "avoidance_ratio_std".to_string(),
            passed: avoidance_std < self.avoidance_std_threshold,
            observed_value: avoidance_std,
            threshold: self.avoidance_std_threshold,
            population_size: 0, // cross-scale
        });

        // Invariant 3: Mean conservation ratio across scales close to 1.0
        let ratios: Vec<f64> = results.iter().map(|r| r.metrics.conservation_ratio).collect();
        let mean_ratio = mean(&ratios);
        checks.push(InvariantResult {
            name: "mean_conservation_ratio".to_string(),
            passed: (mean_ratio - 1.0).abs() < self.conservation_tolerance,
            observed_value: mean_ratio,
            threshold: self.conservation_tolerance,
            population_size: 0,
        });

        // Invariant 4: Conservation ratio std dev across scales is small
        let ratio_std = std_dev(&ratios);
        checks.push(InvariantResult {
            name: "conservation_ratio_std".to_string(),
            passed: ratio_std < self.conservation_tolerance,
            observed_value: ratio_std,
            threshold: self.conservation_tolerance,
            population_size: 0,
        });

        // Invariant 5: Role interaction counts are balanced (each ~1/3)
        for r in results {
            let total: u64 = r.metrics.role_interaction_counts.iter().sum();
            if total > 0 {
                let max_deviation = r.metrics.role_interaction_counts.iter().map(|&c| {
                    let fraction = c as f64 / total as f64;
                    (fraction - 1.0 / 3.0).abs()
                }).fold(0.0_f64, f64::max);
                checks.push(InvariantResult {
                    name: format!("role_balance@{}", r.population_size),
                    passed: max_deviation < 0.05,
                    observed_value: max_deviation,
                    threshold: 0.05,
                    population_size: r.population_size,
                });
            }
        }

        checks
    }

    /// Checks a single custom invariant.
    pub fn check_custom<F>(&self, name: &str, results: &[ScaleResult], check_fn: F) -> InvariantResult
    where
        F: Fn(&[ScaleResult]) -> (bool, f64, f64),
    {
        let (passed, observed, threshold) = check_fn(results);
        InvariantResult {
            name: name.to_string(),
            passed,
            observed_value: observed,
            threshold,
            population_size: 0,
        }
    }
}
