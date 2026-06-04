//! Statistical report: formatted output of verification results.

use crate::types::*;

/// Generates a formatted report from verification results.
pub struct StatisticalReport<'a> {
    summary: &'a VerificationSummary,
}

impl<'a> StatisticalReport<'a> {
    /// Creates a new report from the given verification summary.
    pub fn new(summary: &'a VerificationSummary) -> Self {
        StatisticalReport { summary }
    }

    /// Formats the report as a string.
    pub fn format(&self) -> String {
        let mut out = String::new();

        out.push_str("═".repeat(60).as_str());
        out.push_str("\n");
        out.push_str("  CONSERVATION LAW VERIFICATION REPORT\n");
        out.push_str("═".repeat(60).as_str());
        out.push_str("\n\n");

        // Overall summary
        out.push_str(&format!("Total tests:  {}\n", self.summary.total_tests));
        out.push_str(&format!("Passed:       {}  ✓\n", self.summary.passed));
        out.push_str(&format!("Failed:       {}", self.summary.failed));
        if self.summary.failed > 0 {
            out.push_str("  ✗\n");
        } else {
            out.push_str("\n");
        }
        out.push_str(&format!("Pass rate:    {:.1}%\n\n", self.summary.pass_rate() * 100.0));

        // Scale results
        out.push_str("─".repeat(40).as_str());
        out.push_str("\n  SCALE SWEEP RESULTS\n");
        out.push_str("─".repeat(40).as_str());
        out.push_str("\n\n");

        out.push_str(&format!(
            "{:>10} {:>12} {:>12} {:>8}\n",
            "Pop Size", "Cons. Ratio", "Avoidance", "Status"
        ));
        out.push_str(&format!(
            "{:>10} {:>12} {:>12} {:>8}\n",
            "───────", "───────────", "─────────", "──────"
        ));

        for sr in &self.summary.scale_results {
            let status = if sr.conservation_holds { "✓" } else { "✗" };
            out.push_str(&format!(
                "{:>10} {:>12.6} {:>12.6} {:>8}\n",
                sr.population_size,
                sr.metrics.conservation_ratio,
                sr.metrics.avoidance_ratio,
                status
            ));
        }

        // Invariant results
        if !self.summary.invariant_results.is_empty() {
            out.push_str("\n");
            out.push_str("─".repeat(40).as_str());
            out.push_str("\n  INVARIANT CHECKS\n");
            out.push_str("─".repeat(40).as_str());
            out.push_str("\n\n");

            for inv in &self.summary.invariant_results {
                let status = if inv.passed { "✓" } else { "✗" };
                out.push_str(&format!(
                    "{} {} (observed: {:.6}, threshold: {:.6})\n",
                    status, inv.name, inv.observed_value, inv.threshold
                ));
            }
        }

        // Regression results
        if !self.summary.regression_results.is_empty() {
            out.push_str("\n");
            out.push_str("─".repeat(40).as_str());
            out.push_str("\n  REGRESSION TESTS (vs 5 Baseline Laws)\n");
            out.push_str("─".repeat(40).as_str());
            out.push_str("\n\n");

            out.push_str(&format!(
                "{:<30} {:>10} {:>10} {:>10} {:>6}\n",
                "Law", "Expected", "Observed", "Delta", "Status"
            ));
            out.push_str(&format!(
                "{:<30} {:>10} {:>10} {:>10} {:>6}\n",
                "───", "───────", "────────", "─────", "──────"
            ));

            for reg in &self.summary.regression_results {
                let status = if reg.passed { "✓" } else { "✗" };
                out.push_str(&format!(
                    "{:<30} {:>10.6} {:>10.6} {:>10.6} {:>6}\n",
                    reg.law_name, reg.expected, reg.observed, reg.delta, status
                ));
            }
        }

        // Footer
        out.push_str("\n");
        out.push_str("═".repeat(60).as_str());
        out.push_str("\n");
        if self.summary.all_passed() {
            out.push_str("  ALL TESTS PASSED ✓\n");
        } else {
            out.push_str(&format!("  {} TEST(S) FAILED ✗\n", self.summary.failed));
        }
        out.push_str("═".repeat(60).as_str());
        out.push_str("\n");

        out
    }

    /// Prints the report to stdout.
    pub fn print(&self) {
        println!("{}", self.format());
    }
}
