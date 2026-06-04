//! Scale sweep: run the same experiment at multiple population sizes.

use crate::types::*;

/// Runs a simulation at multiple population scales and collects metrics.
pub struct ScaleSweep<'a> {
    simulation: &'a dyn Simulation,
    scales: &'a [usize],
    steps: u64,
}

impl<'a> ScaleSweep<'a> {
    /// Creates a new scale sweep.
    pub fn new(simulation: &'a dyn Simulation, scales: &'a [usize], steps: u64) -> Self {
        ScaleSweep {
            simulation,
            scales,
            steps,
        }
    }

    /// Runs the simulation at all configured scales and returns results.
    pub fn run(&self) -> Vec<ScaleResult> {
        self.scales
            .iter()
            .map(|&pop| {
                let metrics = self.simulation.run(pop, self.steps);
                let holds = metrics.conservation_holds(0.01);
                ScaleResult {
                    population_size: pop,
                    metrics,
                    conservation_holds: holds,
                    elapsed_us: 0, // No timing in pure-Rust no-dep mode; use steps as proxy
                }
            })
            .collect()
    }

    /// Collects conservation ratios across all scales.
    pub fn conservation_ratios(results: &[ScaleResult]) -> Vec<f64> {
        results.iter().map(|r| r.metrics.conservation_ratio).collect()
    }

    /// Collects avoidance ratios across all scales.
    pub fn avoidance_ratios(results: &[ScaleResult]) -> Vec<f64> {
        results.iter().map(|r| r.metrics.avoidance_ratio).collect()
    }

    /// Collects convergence rates across all scales.
    pub fn convergence_rates(results: &[ScaleResult]) -> Vec<f64> {
        results.iter().map(|r| r.metrics.convergence_rate).collect()
    }

    /// Returns the population sizes from results.
    pub fn population_sizes(results: &[ScaleResult]) -> Vec<usize> {
        results.iter().map(|r| r.population_size).collect()
    }

    /// Checks if conservation holds at all scales within tolerance.
    pub fn all_scales_conserved(results: &[ScaleResult], tolerance: f64) -> bool {
        results.iter().all(|r| r.metrics.conservation_holds(tolerance))
    }

    /// Returns the scale with the worst conservation deviation.
    pub fn worst_scale(results: &[ScaleResult]) -> Option<&ScaleResult> {
        results
            .iter()
            .max_by(|a, b| {
                let da = (a.metrics.conservation_ratio - 1.0).abs();
                let db = (b.metrics.conservation_ratio - 1.0).abs();
                da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
            })
    }
}
