pub mod dead_code;
pub mod unbounded_variables;

use crate::lint_engine::LintContext;
use owo_colors::OwoColorize;
use std::fmt;
use tree_sitter::Point;

pub trait Lint {
    fn check(&self, context: &LintContext) -> Vec<Diagnostic>;
    fn lint_info(&self) -> &LintMetadata;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Severity {
    Error,
    Warning,
}

pub struct LintMetadata {
    lint_id: &'static str,
    name: &'static str,
    severity: Severity,
}

pub struct Diagnostic {
    lint_id: &'static str,
    name: &'static str,
    severity: Severity,

    start_pos: Point,
    end_pos: Point,
}

impl Diagnostic {
    pub fn new<L: Lint + ?Sized>(lint: &L, start_pos: Point, end_pos: Point) -> Self {
        let meta = lint.lint_info();

        Self {
            lint_id: meta.lint_id,
            name: meta.name,
            severity: meta.severity,
            start_pos,
            end_pos,
        }
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let severity = match self.severity {
            Severity::Error => "error".red().bold().to_string(),
            Severity::Warning => "warning".yellow().bold().to_string(),
        };

        write!(
            f,
            "{} [{}] {} at {}:{} to {}:{} ",
            severity,
            self.lint_id,
            self.name,
            self.start_pos.row + 1,
            self.start_pos.column + 1,
            self.end_pos.row + 1,
            self.end_pos.column,
        )
    }
}
