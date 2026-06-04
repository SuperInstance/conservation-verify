//! Tests for conservation-verify.

use conservation_verify::*;

// ─── Types Tests ───────────────────────────────────────────

#[test]
fn test_agent_role_all_returns_three() {
    assert_eq!(AgentRole::all().len(), 3);
}

#[test]
fn test_agent_role_names() {
    assert_eq!(AgentRole::Initiator.name(), "Initiator");
    assert_eq!(AgentRole::Responder.name(), "Responder");
    assert_eq!(AgentRole::Mediator.name(), "Mediator");
}

#[test]
fn test_simulation_metrics_default_conservation() {
    let m = SimulationMetrics::new(100);
    assert!(m.conservation_holds(0.01));
    assert_eq!(m.population_size, 100);
}

#[test]
fn test_simulation_metrics_conservation_holds() {
    let mut m = SimulationMetrics::new(50);
    m.conservation_ratio = 1.005;
    assert!(m.conservation_holds(0.01));
    m.conservation_ratio = 1.02;
    assert!(!m.conservation_holds(0.01));
}

#[test]
fn test_verification_summary_new() {
    let s = VerificationSummary::new();
    assert_eq!(s.total_tests, 0);
    assert_eq!(s.passed, 0);
    assert_eq!(s.failed, 0);
    assert!(s.all_passed());
    assert_eq!(s.pass_rate(), 1.0);
}

#[test]
fn test_verification_summary_with_failures() {
    let mut s = VerificationSummary::new();
    s.total_tests = 3;
    s.passed = 2;
    s.failed = 1;
    assert!(!s.all_passed());
    assert!((s.pass_rate() - 0.6666).abs() < 0.01);
}

// ─── Statistical Helpers Tests ─────────────────────────────

#[test]
fn test_mean_empty() {
    assert_eq!(mean(&[]), 0.0);
}

#[test]
fn test_mean_values() {
    let v = [1.0, 2.0, 3.0, 4.0, 5.0];
    assert!((mean(&v) - 3.0).abs() < f64::EPSILON);
}

#[test]
fn test_std_dev_empty() {
    assert_eq!(std_dev(&[]), 0.0);
}

#[test]
fn test_std_dev_constant() {
    let v = [5.0, 5.0, 5.0];
    assert!(std_dev(&v) < f64::EPSILON);
}

#[test]
fn test_std_dev_values() {
    let v = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    let s = std_dev(&v);
    assert!(s > 0.0);
    assert!((s - 2.0).abs() < 0.1);
}

#[test]
fn test_coefficient_of_variation() {
    let v = [10.0, 20.0, 30.0];
    let cv = coefficient_of_variation(&v);
    assert!(cv > 0.0);
}

#[test]
fn test_coefficient_of_variation_zero_mean() {
    let v = [0.0, 0.0, 0.0];
    assert_eq!(coefficient_of_variation(&v), 0.0);
}

// ─── Deterministic Simulation Tests ────────────────────────

#[test]
fn test_deterministic_simulation_perfect_conservation() {
    let sim = DeterministicSimulation::new(0.0, 0.0);
    let m = sim.run(100, 1000);
    assert!((m.conservation_ratio - 1.0).abs() < f64::EPSILON);
    assert!(m.conservation_holds(0.01));
}

#[test]
fn test_deterministic_simulation_with_offset() {
    let sim = DeterministicSimulation::new(0.05, 0.0);
    let m = sim.run(100, 1000);
    assert!((m.conservation_ratio - 1.05).abs() < 0.001);
    assert!(!m.conservation_holds(0.01));
}

#[test]
fn test_deterministic_simulation_noise_decreases_with_population() {
    let sim = DeterministicSimulation::new(0.0, 1.0);
    let m_small = sim.run(10, 1000);
    let m_large = sim.run(10000, 1000);
    // Larger populations should have less noise impact
    assert!(m_large.std_dev <= m_small.std_dev);
}

#[test]
fn test_deterministic_role_interaction_counts() {
    let sim = DeterministicSimulation::new(0.0, 0.0);
    let m = sim.run(100, 1000);
    let total: u64 = m.role_interaction_counts.iter().sum();
    assert_eq!(total, 100 * 1000);
}

// ─── Scale Sweep Tests ────────────────────────────────────

#[test]
fn test_scale_sweep_runs_all_scales() {
    let sim = DeterministicSimulation::new(0.0, 0.0);
    let scales = vec![10, 50, 100];
    let sweep = ScaleSweep::new(&sim, &scales, 1000);
    let results = sweep.run();
    assert_eq!(results.len(), 3);
    assert_eq!(results[0].population_size, 10);
    assert_eq!(results[1].population_size, 50);
    assert_eq!(results[2].population_size, 100);
}

#[test]
fn test_scale_sweep_perfect_conservation() {
    let sim = DeterministicSimulation::new(0.0, 0.0);
    let scales = vec![10, 100, 1000];
    let sweep = ScaleSweep::new(&sim, &scales, 1000);
    let results = sweep.run();
    assert!(ScaleSweep::all_scales_conserved(&results, 0.01));
}

#[test]
fn test_scale_sweep_broken_conservation() {
    let sim = DeterministicSimulation::new(0.1, 0.0);
    let scales = vec![100];
    let sweep = ScaleSweep::new(&sim, &scales, 1000);
    let results = sweep.run();
    assert!(!ScaleSweep::all_scales_conserved(&results, 0.01));
}

// ─── Invariant Checker Tests ──────────────────────────────

#[test]
fn test_invariant_checker_perfect() {
    let sim = DeterministicSimulation::new(0.0, 0.0);
    let scales = vec![10, 100, 1000];
    let sweep = ScaleSweep::new(&sim, &scales, 1000);
    let results = sweep.run();
    let checker = InvariantChecker::new(0.01);
    let inv_results = checker.check_all(&results);
    assert!(inv_results.iter().all(|r| r.passed));
}

#[test]
fn test_invariant_checker_custom() {
    let sim = DeterministicSimulation::new(0.0, 0.0);
    let scales = vec![100];
    let sweep = ScaleSweep::new(&sim, &scales, 1000);
    let results = sweep.run();
    let checker = InvariantChecker::new(0.01);
    let result = checker.check_custom("all_positive", &results, |r| {
        let all_pos = r.iter().all(|sr| sr.metrics.mean > 0.0);
        (all_pos, 1.0, 0.0)
    });
    assert!(result.passed);
}

// ─── Regression Test Tests ────────────────────────────────

#[test]
fn test_regression_all_laws_perfect() {
    let sim = DeterministicSimulation::new(0.0, 0.0);
    let scales = vec![10, 50, 100, 500, 1000, 5000];
    let sweep = ScaleSweep::new(&sim, &scales, 1000);
    let results = sweep.run();
    let regression = RegressionTest::new(0.05);
    let reg_results = regression.check(&results);
    assert_eq!(reg_results.len(), 5);
    // With perfect conservation, most should pass
    assert!(reg_results.iter().filter(|r| r.passed).count() >= 3);
}

#[test]
fn test_regression_check_single_law() {
    let sim = DeterministicSimulation::new(0.0, 0.0);
    let scales = vec![100, 1000];
    let sweep = ScaleSweep::new(&sim, &scales, 1000);
    let results = sweep.run();
    let regression = RegressionTest::new(0.05);
    let result = regression.check_law(BaselineLaw::InteractionConservation, &results);
    assert!(result.passed);
}

#[test]
fn test_baseline_law_all_returns_five() {
    assert_eq!(BaselineLaw::all().len(), 5);
}

#[test]
fn test_baseline_law_names_distinct() {
    let names: Vec<&str> = BaselineLaw::all().iter().map(|l| l.name()).collect();
    for i in 0..names.len() {
        for j in (i + 1)..names.len() {
            assert_ne!(names[i], names[j], "Law names should be distinct");
        }
    }
}

// ─── Full Harness Tests ───────────────────────────────────

#[test]
fn test_conservation_test_harness_perfect() {
    let sim = DeterministicSimulation::new(0.0, 0.0);
    let test = ConservationTest::new(&sim)
        .with_scales(vec![10, 50, 100])
        .with_steps(1000)
        .with_tolerance(0.01);
    let summary = test.run();
    assert!(summary.all_passed());
}

#[test]
fn test_conservation_test_harness_with_report() {
    let sim = DeterministicSimulation::new(0.0, 0.0);
    let test = ConservationTest::new(&sim)
        .with_scales(vec![10, 100])
        .with_steps(500);
    let summary = test.run_and_report();
    assert!(summary.all_passed());
}

// ─── Report Formatting Tests ──────────────────────────────

#[test]
fn test_statistical_report_format() {
    let sim = DeterministicSimulation::new(0.0, 0.0);
    let test = ConservationTest::new(&sim)
        .with_scales(vec![10, 100])
        .with_steps(500);
    let summary = test.run();
    let report = StatisticalReport::new(&summary);
    let text = report.format();
    assert!(text.contains("CONSERVATION LAW VERIFICATION REPORT"));
    assert!(text.contains("SCALE SWEEP RESULTS"));
    assert!(text.contains("INVARIANT CHECKS"));
    assert!(text.contains("REGRESSION TESTS"));
}
