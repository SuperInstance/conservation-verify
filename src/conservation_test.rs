//! Conservation test harness.

use crate::scale_sweep::ScaleSweep;
use crate::invariant_checker::InvariantChecker;
use crate::regression_test::RegressionTest;
use crate::statistical_report::StatisticalReport;
use crate::types::*;

/// Default population sizes for scale sweeps.
pub const DEFAULT_SCALES: &[usize] = &[10, 50, 100, 500, 1000, 5000];

/// Default number of simulation steps.
pub const DEFAULT_STEPS: u64 = 10_000;

/// Default tolerance for conservation ratio.
pub const DEFAULT_TOLERANCE: f64 = 0.01;

/// A test harness that runs simulations at multiple scales and checks
/// that conservation laws hold across all scales.
pub struct ConservationTest<'a> {
    /// The simulation to test.
    simulation: &'a dyn Simulation,
    /// Population sizes to sweep.
    scales: Vec<usize>,
    /// Number of steps per simulation.
    steps: u64,
    /// Tolerance for conservation ratio deviation from 1.0.
    tolerance: f64,
}

impl<'a> ConservationTest<'a> {
    /// Creates a new conservation test with default settings.
    pub fn new(simulation: &'a dyn Simulation) -> Self {
        ConservationTest {
            simulation,
            scales: DEFAULT_SCALES.to_vec(),
            steps: DEFAULT_STEPS,
            tolerance: DEFAULT_TOLERANCE,
        }
    }

    /// Sets custom population scales.
    pub fn with_scales(mut self, scales: Vec<usize>) -> Self {
        self.scales = scales;
        self
    }

    /// Sets the number of simulation steps.
    pub fn with_steps(mut self, steps: u64) -> Self {
        self.steps = steps;
        self
    }

    /// Sets the conservation tolerance.
    pub fn with_tolerance(mut self, tolerance: f64) -> Self {
        self.tolerance = tolerance;
        self
    }

    /// Runs the full conservation verification suite.
    pub fn run(&self) -> VerificationSummary {
        let mut summary = VerificationSummary::new();

        // Run scale sweep
        let sweep = ScaleSweep::new(self.simulation, &self.scales, self.steps);
        let scale_results = sweep.run();
        summary.scale_results = scale_results;

        // Run invariant checks
        let checker = InvariantChecker::new(self.tolerance);
        let invariant_results = checker.check_all(&summary.scale_results);
        summary.invariant_results = invariant_results;

        // Run regression tests
        let regression = RegressionTest::new(self.tolerance);
        let regression_results = regression.check(&summary.scale_results);
        summary.regression_results = regression_results;

        // Count pass/fail
        for sr in &summary.scale_results {
            summary.total_tests += 1;
            if sr.conservation_holds {
                summary.passed += 1;
            } else {
                summary.failed += 1;
            }
        }
        for inv in &summary.invariant_results {
            summary.total_tests += 1;
            if inv.passed {
                summary.passed += 1;
            } else {
                summary.failed += 1;
            }
        }
        for reg in &summary.regression_results {
            summary.total_tests += 1;
            if reg.passed {
                summary.passed += 1;
            } else {
                summary.failed += 1;
            }
        }

        summary
    }

    /// Runs the test and prints a formatted report.
    pub fn run_and_report(&self) -> VerificationSummary {
        let summary = self.run();
        let report = StatisticalReport::new(&summary);
        report.print();
        summary
    }
}
