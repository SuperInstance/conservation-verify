//! # conservation-verify
//!
//! Tools to verify and test conservation laws in ternary agent systems.
//! This crate provides test harnesses, scale sweeps, invariant checking,
//! regression testing, and statistical reporting for conservation law validation.

#![forbid(unsafe_code)]

mod conservation_test;
mod invariant_checker;
mod regression_test;
mod scale_sweep;
mod statistical_report;
mod types;

pub use conservation_test::ConservationTest;
pub use invariant_checker::InvariantChecker;
pub use regression_test::{BaselineLaw, RegressionTest};
pub use scale_sweep::ScaleSweep;
pub use statistical_report::StatisticalReport;
pub use types::*;
