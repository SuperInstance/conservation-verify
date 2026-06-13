# conservation-verify

A Rust library for **verifying conservation laws in ternary agent systems**, providing a complete test harness: scale sweeps, invariant checking, regression testing against five baseline conservation laws, and formatted statistical reporting.

## Why It Matters

Any system that claims conservation properties must be **empirically verified** across scales. This crate provides the verification infrastructure for ternary agent systems (Initiator/Responder/Mediator), answering:

- Does the conservation ratio hold at population sizes from 10 to 5000?
- Are the three roles balanced (each ≈ 1/3 of interactions)?
- Is avoidance symmetry preserved?
- Does convergence rate scale correctly with population?
- Are all metrics within expected bounds?

This is essential for:

- **Multi-agent system verification** — proving emergent behaviors are scale-invariant
- **Physics-inspired computing** — conservation laws as correctness criteria
- **Regression testing** — ensuring algorithm changes don't break invariants
- **Reproducible research** — the harness produces formatted, citable reports

## How It Works

### Five Conservation Laws

The crate tests against five baseline laws:

| Law | Baseline | Formula |
|-----|----------|---------|
| **Interaction Conservation** | Ratio = 1.0 | Total interactions in / total interactions out |
| **Role Balance** | Fraction = 1/3 | Each role gets equal interaction share |
| **Avoidance Symmetry** | Ratio = 0.5 | Symmetric avoidance between agent pairs |
| **Convergence Scaling** | Rate ∝ 1/log₂(N) | Convergence rate matches population scaling |
| **Metric Bounds** | Deviation = 0 | All metrics within expected range |

### Scale Sweep Architecture

The `ScaleSweep` runner executes the same simulation at multiple population sizes:

$$\text{sweep}(f, [N_1, N_2, \ldots, N_k]) = [f(N_1), f(N_2), \ldots, f(N_k)]$$

Default scales: [10, 50, 100, 500, 1000, 5000] with 10,000 steps each.

### Invariant Checking

The `InvariantChecker` validates cross-scale invariants:

1. **Per-scale conservation**: $|r_N - 1| < \epsilon$ for each scale $N$
2. **Cross-scale stability**: $\sigma(\{r_{N_i}\}) < \epsilon$ (low variance across scales)
3. **Mean conservation**: $|\bar{r} - 1| < \epsilon$ (average close to 1)
4. **Role balance**: $|c_i / \sum c_j - 1/3| < 0.05$ for each role
5. **Avoidance stability**: $\sigma(\{a_{N_i}\}) < 0.05$

### Statistical Report

The report formats results as:

```
══════════════════════════════════════════
  CONSERVATION LAW VERIFICATION REPORT
══════════════════════════════════════════

Total tests:  17
Passed:       17  ✓
Pass rate:    100.0%

──────────────────────────
  SCALE SWEEP RESULTS
──────────────────────────

  Pop Size  Cons. Ratio   Avoidance  Status
  -------  -----------  ----------  ------
       10      1.000100    0.503162  ✓
      100      1.000010    0.501000  ✓
```

### Convergence Scaling Model

The expected convergence rate at population $N$:

$$\text{rate}(N) = \frac{1}{\max(1, \log_2 N)}$$

The regression test normalizes observed rates by this expected value and checks for unit ratio.

### Big-O Complexity

| Operation | Time | Space |
|-----------|------|-------|
| Scale sweep (k scales, S steps) | O(k × S × N_max) | O(k) results |
| Invariant checking | O(k × roles) | O(k) |
| Regression test (5 laws) | O(5 × k) | O(5) |
| Full verification suite | O(k × S × N_max) | O(k + invariants) |
| Report formatting | O(k + tests) | O(output size) |

## Quick Start

```rust
use conservation_verify::{ConservationTest, DeterministicSimulation};

let sim = DeterministicSimulation::new(0.0, 0.001); // offset=0, small noise

let summary = ConservationTest::new(&sim)
    .with_scales(vec![10, 100, 1000])
    .with_steps(10_000)
    .with_tolerance(0.01)
    .run_and_report();

assert!(summary.all_passed());
```

## API

| Type / Method | Description |
|---------------|-------------|
| `ConservationTest::new(&dyn Simulation)` | Create test harness |
| `.with_scales(Vec<usize>)` | Configure population sizes |
| `.with_steps(u64)` | Steps per simulation |
| `.with_tolerance(f64)` | Conservation ratio tolerance |
| `.run() → VerificationSummary` | Execute full suite |
| `.run_and_report() → VerificationSummary` | Execute + print report |
| `DeterministicSimulation` | Test simulation with configurable offset/noise |
| `ScaleSweep` | Standalone scale sweep runner |
| `InvariantChecker` | Standalone invariant checker |
| `RegressionTest` | Standalone regression tester |

## Architecture Notes

The **γ + η = C** link: the simulation runner (γ) generates metrics at each scale, while the invariant checker and regression tester (η) validate that these metrics satisfy conservation laws. Together they conserve the verification invariant C — if any transform in the simulation pipeline violates a conservation law, the harness detects it and marks the test failed. The five baseline laws form a complete verification suite: they cover magnitude (conservation ratio), distribution (role balance), symmetry (avoidance), scaling (convergence), and bounds (metric range). The `DeterministicSimulation` allows controlled injection of violations via the `conservation_offset` parameter for negative testing.

## References

- Hehner, E. C. R. (1993). *A Practical Theory of Programming.* Springer. (Invariant-based verification.)
- Lamport, L. (2002). *Specifying Systems.* Addison-Wesley. (TLA+ invariant checking.)
- Mitchell, M. (2009). *Complexity: A Guided Tour.* Oxford. (Emergent conservation in complex systems.)
- Claerbout, J. F., & Karrenbach, M. (1992). *Electronic documents give reproducible research.* (Reproducibility framework.)
- Nielson, F., Nielson, H. R., & Hankin, C. (2005). *Principles of Program Analysis.* Springer. (Static invariant detection.)

## License

MIT
