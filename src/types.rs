//! Core types for conservation law verification.

/// The three agent roles in a ternary system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AgentRole {
    /// The agent that initiates interactions.
    Initiator,
    /// The agent that responds to interactions.
    Responder,
    /// The agent that mediates between initiator and responder.
    Mediator,
}

impl AgentRole {
    /// Returns all three roles as a slice.
    pub fn all() -> &'static [AgentRole] {
        &[AgentRole::Initiator, AgentRole::Responder, AgentRole::Mediator]
    }

    /// Returns the name of the role as a static string.
    pub fn name(&self) -> &'static str {
        match self {
            AgentRole::Initiator => "Initiator",
            AgentRole::Responder => "Responder",
            AgentRole::Mediator => "Mediator",
        }
    }
}

/// Metrics collected from a single simulation run.
#[derive(Debug, Clone)]
pub struct SimulationMetrics {
    /// Population size used in this simulation.
    pub population_size: usize,
    /// Number of steps simulated.
    pub steps: u64,
    /// Mean value of the measured quantity.
    pub mean: f64,
    /// Standard deviation of the measured quantity.
    pub std_dev: f64,
    /// Minimum value observed.
    pub min: f64,
    /// Maximum value observed.
    pub max: f64,
    /// The conservation ratio (should be ~1.0 for conservation).
    pub conservation_ratio: f64,
    /// Per-role interaction counts: [initiator, responder, mediator].
    pub role_interaction_counts: [u64; 3],
    /// Avoidance ratio (fraction of interactions avoided).
    pub avoidance_ratio: f64,
    /// Convergence rate (how fast the system reaches steady state).
    pub convergence_rate: f64,
}

impl SimulationMetrics {
    /// Creates a new metrics instance with the given population size.
    pub fn new(population_size: usize) -> Self {
        SimulationMetrics {
            population_size,
            steps: 0,
            mean: 0.0,
            std_dev: 0.0,
            min: f64::MAX,
            max: f64::MIN,
            conservation_ratio: 1.0,
            role_interaction_counts: [0, 0, 0],
            avoidance_ratio: 0.0,
            convergence_rate: 0.0,
        }
    }

    /// Checks if the conservation ratio is within tolerance of 1.0.
    pub fn conservation_holds(&self, tolerance: f64) -> bool {
        (self.conservation_ratio - 1.0).abs() < tolerance
    }
}

/// Result of a single invariant check.
#[derive(Debug, Clone)]
pub struct InvariantResult {
    /// Name of the invariant that was checked.
    pub name: String,
    /// Whether the invariant held.
    pub passed: bool,
    /// The observed value.
    pub observed_value: f64,
    /// The threshold that was applied.
    pub threshold: f64,
    /// The population size at which this was checked.
    pub population_size: usize,
}

/// Result of a single regression test comparison.
#[derive(Debug, Clone)]
pub struct RegressionResult {
    /// Name of the baseline law being tested.
    pub law_name: String,
    /// Whether the regression test passed.
    pub passed: bool,
    /// Expected value from the baseline.
    pub expected: f64,
    /// Actually observed value.
    pub observed: f64,
    /// Absolute difference between expected and observed.
    pub delta: f64,
    /// Whether the delta is within tolerance.
    pub within_tolerance: bool,
}

/// A verification summary combining all test results.
#[derive(Debug, Clone)]
pub struct VerificationSummary {
    /// Total number of tests run.
    pub total_tests: usize,
    /// Number of tests that passed.
    pub passed: usize,
    /// Number of tests that failed.
    pub failed: usize,
    /// Per-scale results.
    pub scale_results: Vec<ScaleResult>,
    /// Invariant check results.
    pub invariant_results: Vec<InvariantResult>,
    /// Regression test results.
    pub regression_results: Vec<RegressionResult>,
}

impl VerificationSummary {
    /// Creates an empty summary.
    pub fn new() -> Self {
        VerificationSummary {
            total_tests: 0,
            passed: 0,
            failed: 0,
            scale_results: Vec::new(),
            invariant_results: Vec::new(),
            regression_results: Vec::new(),
        }
    }

    /// Returns true if all tests passed.
    pub fn all_passed(&self) -> bool {
        self.failed == 0
    }

    /// Returns the overall pass rate as a fraction.
    pub fn pass_rate(&self) -> f64 {
        if self.total_tests == 0 {
            1.0
        } else {
            self.passed as f64 / self.total_tests as f64
        }
    }
}

/// Results from a single population scale.
#[derive(Debug, Clone)]
pub struct ScaleResult {
    /// Population size.
    pub population_size: usize,
    /// Metrics collected at this scale.
    pub metrics: SimulationMetrics,
    /// Whether the conservation law held at this scale.
    pub conservation_holds: bool,
    /// Wall-clock time for the simulation in microseconds.
    pub elapsed_us: u64,
}

/// A trait for simulation functions that can be tested.
pub trait Simulation {
    /// Run the simulation at a given population size for a number of steps.
    /// Returns the collected metrics.
    fn run(&self, population_size: usize, steps: u64) -> SimulationMetrics;
}

/// A simple deterministic simulation for testing purposes.
pub struct DeterministicSimulation {
    /// Base conservation ratio offset from 1.0.
    pub conservation_offset: f64,
    /// Noise level added to measurements.
    pub noise_level: f64,
}

impl DeterministicSimulation {
    /// Creates a new deterministic simulation.
    pub fn new(conservation_offset: f64, noise_level: f64) -> Self {
        DeterministicSimulation {
            conservation_offset,
            noise_level,
        }
    }
}

impl Simulation for DeterministicSimulation {
    fn run(&self, population_size: usize, steps: u64) -> SimulationMetrics {
        let mut m = SimulationMetrics::new(population_size);
        m.steps = steps;

        // Deterministic "simulation": the conservation ratio is 1.0 + offset
        // with small noise proportional to 1/sqrt(population).
        let noise = self.noise_level / (population_size as f64).sqrt();
        let ratio = 1.0 + self.conservation_offset + noise;
        m.conservation_ratio = ratio;
        m.mean = ratio;
        m.std_dev = noise.abs();
        m.min = ratio - noise.abs();
        m.max = ratio + noise.abs();

        // Role interactions scale with population
        let total = population_size as u64 * steps;
        m.role_interaction_counts = [
            total / 3,
            total / 3,
            total - 2 * (total / 3),
        ];

        // Avoidance ratio converges toward 0.5 with more agents
        m.avoidance_ratio = 0.5 + 0.01 / (population_size as f64).sqrt();
        m.convergence_rate = 1.0 / (population_size as f64).log2().max(1.0);

        m
    }
}

/// Compute mean of a slice of f64 values.
pub fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<f64>() / values.len() as f64
}

/// Compute standard deviation of a slice of f64 values (population std dev).
pub fn std_dev(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let m = mean(values);
    let variance = values.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / values.len() as f64;
    variance.sqrt()
}

/// Compute the coefficient of variation.
pub fn coefficient_of_variation(values: &[f64]) -> f64 {
    let m = mean(values);
    if m.abs() < f64::EPSILON {
        return 0.0;
    }
    std_dev(values) / m.abs()
}
