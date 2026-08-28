use super::{Lint, Diagnostic};
use tree_sitter::{Query, QueryCursor, Tree, StreamingIterator};

pub(super) struct UnboundedVariables;

impl Lint for UnboundedVariables {
    fn check(&self, tree: &Tree, source: &str) -> Vec<Diagnostics> {

        let query = Query::new(
            &tree_sitter_minizinc::LANGUAGE.into(),
            r#"
            (type_base
                "var" @var
                domain: (primitive_type "int" @int))"#
            // r#"
            // (declaration
            //     type: [
            //     (type_base
            //         "var"
            //         domain: (primitive_type "int"))
            //     (_
            //         (type_base
            //         "var"
            //         domain: (primitive_type "int")))
            // ]
            // name: (identifier) @var_int_name)"#
            ).expect("Invalid query");

        let capture_idx_var = query.capture_index_for_name("var").unwrap();
        let capture_idx_int = query.capture_index_for_name("int").unwrap();
        let mut query_cursor = QueryCursor::new();
        let mut matches = query_cursor.matches(&query, tree.root_node(), source.as_bytes());
        let diagnostics = vec![];
        while let Some(m) = matches.next() {
            let start_capture = m.captures.iter().find(|x| x.index == capture_idx_var)
            let end_capture = m.captures.iter().find(|x| x.index == capture_idx_int)

            if let (Some(var_cap), Some(int_cap)) = (start_capture, end_capture) {
                let start_pos = var_cap.node.start_position();
                let end_pos = int_cap.node.end_position();

                diagnostics.push()
            }
        }
        diagnostics
    }
}
