use super::{Diagnostic, Lint, LintMetadata, Severity};
use crate::lint_engine::LintContext;
use tree_sitter::{Query, QueryCursor, StreamingIterator};

pub struct UnboundedVariables;

impl UnboundedVariables {
    pub const METADATA: LintMetadata = LintMetadata {
        lint_id: "E001",
        name: "unbounded_vars",
        severity: Severity::Warning,
    };
}

impl Lint for UnboundedVariables {
    fn lint_info(&self) -> &LintMetadata {
        &Self::METADATA
    }

    fn check(&self, context: &LintContext) -> Vec<Diagnostic> {
        let query = Query::new(
            &tree_sitter_minizinc::LANGUAGE.into(),
            r#"
            (type_base
                "var" @var
                domain: (primitive_type "int" @int))"#,
        )
        .expect("Invalid query");

        let capture_idx_var = query.capture_index_for_name("var").unwrap();
        let capture_idx_int = query.capture_index_for_name("int").unwrap();
        let mut query_cursor = QueryCursor::new();
        let mut matches =
            query_cursor.matches(&query, context.tree.root_node(), context.source.as_bytes());
        let mut diagnostics = vec![];
        while let Some(m) = matches.next() {
            let start_capture = m.captures.iter().find(|x| x.index == capture_idx_var);
            let end_capture = m.captures.iter().find(|x| x.index == capture_idx_int);

            if let (Some(var_cap), Some(int_cap)) = (start_capture, end_capture) {
                let start_pos = var_cap.node.start_position();
                let end_pos = int_cap.node.end_position();

                diagnostics.push(Diagnostic::new(&Self, start_pos, end_pos))
            }
        }
        diagnostics
    }
}
