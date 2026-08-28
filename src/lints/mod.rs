mod unbounded_variables;

use unbounded_variables::UnboundedVariables;
use tree_sitter::{Tree, Point};

pub trait Lint {
    fn check(&self, tree: &Tree, source: &str) -> Vec<Diagnostic>;
    fn display(&self, diagnostic: &Diagnostic) -> ();
}

pub struct Diagnostic {
    code: String,
    message: String,
    position: (Point, Point)
}

impl Diagnostic {
    pub fn new<T, U>(code: T, message: U, position: (Point, Point) ) -> Self
    where
        T: Into<String>,
        U: Into<String>
    {
        Self {
            code: code.into(),
            message: message.into(),
            position: position
        }
    }
}

pub struct LintEngine {
    pub lints: Vec<Box<dyn Lint>>
}

impl LintEngine {
    pub fn new() -> Self {
        Self {
            lints: vec![
                Box::new(UnboundedVariables)
            ]
        }
    }
}
