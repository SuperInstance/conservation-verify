# conservation-verify

Tools to verify and test conservation laws in ternary agent systems.

This is the testing/companion crate to `conservation-matrix`. It provides test harnesses, scale sweeps, invariant checking, regression testing, and statistical reporting for validating that conservation laws hold in simulations of ternary (three-role) agent systems.

## Conservation Laws (Baselines)

The crate verifies five conservation laws:

1. **Interaction Conservation** — Total interaction count is conserved across scales
2. **Role Balance** — Interactions are distributed ~1:1:1 across the three agent roles
3. **Avoidance Symmetry** — Avoidance ratios converge to a symmetric equilibrium (~0.5)
4. **Convergence Scaling** — Convergence rate scales inversely with log(population)
5. **Metric Bounds** — All measured quantities remain within expected bounds

## Verification Methodology

### Multi-Scale Sweep

Simulations are run at multiple population sizes (10, 50, 100, 500, 1000, 5000 agents) to verify that conservation laws are scale-invariant. A law that holds at small scale but breaks at large scale indicates a numerical or algorithmic issue.

### Invariant Checking

For each scale, we verify:
- Conservation ratio is within tolerance of 1.0
- Avoidance ratio standard deviation across scales is below threshold
- Mean conservation ratio across scales is close to 1.0
- Conservation ratio variance across scales is small
- Role interaction counts are balanced (each role ≈ 1/3 of total)

### Regression Testing

Observed values are compared against the five baseline conservation laws. Each comparison produces:
- Expected value (from baseline)
- Observed value (from simulation)
- Delta (absolute difference)
- Pass/fail based on configurable tolerance

### Statistical Reporting

Results are formatted into a human-readable report showing:
- Overall pass rate
- Per-scale conservation ratios and status
- All invariant check results
- Regression test comparison table

## Usage

```rust
use conservation_verify::*;

// Create a simulation (implement the Simulation trait for your system)
let sim = DeterministicSimulation::new(0.0, 0.0);

// Run full verification with report
let test = ConservationTest::new(&sim)
    .with_scales(vec![10, 50, 100, 500, 1000, 5000])
    .with_steps(10_000)
    .with_tolerance(0.01);

let summary = test.run_and_report();
assert!(summary.all_passed());
```

### Custom Invariants

```rust
let checker = InvariantChecker::new(0.01);
let result = checker.check_custom("my_invariant", &results, |r| {
    let all_pos = r.iter().all(|sr| sr.metrics.mean > 0.0);
    (all_pos, 1.0, 0.0)
});
```

### Individual Regression Tests

```rust
let regression = RegressionTest::new(0.05);
let result = regression.check_law(BaselineLaw::InteractionConservation, &results);
println!("{}: expected={}, observed={}, delta={}",
    result.law_name, result.expected, result.observed, result.delta);
```

## Architecture

- **`ConservationTest`** — Top-level test harness orchestrating all checks
- **`ScaleSweep`** — Runs simulations at multiple population sizes
- **`InvariantChecker`** — Verifies invariants across scale results
- **`RegressionTest`** — Compares against the 5 baseline conservation laws
- **`StatisticalReport`** — Formats and prints verification results

## Requirements

- Pure Rust, no unsafe code, no external dependencies
- Edition 2021

## License

MIT
