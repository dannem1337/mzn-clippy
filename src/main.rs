mod lints;

use lints::LintEngine;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

#[derive(Parser)]
#[command(name = "mzn clippy")]
struct Cli {
    #[arg(required = true)]
    files: Vec<PathBuf>,
}

#[warn(unused)]
fn print_node(node: tree_sitter::Node, source: &str, depth: usize) {
    let indent = "  ".repeat(depth);
    let text = if node.child_count() == 0 {
        format!(" {:?}", &source[node.start_byte()..node.end_byte()])
    } else {
        String::new()
    };
    println!(
        "{}{}[{}]{}",
        indent,
        node.kind(),
        if node.is_named() { "named" } else { "anon" },
        text
    );
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        print_node(child, source, depth + 1);
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    for path in &cli.files {
        let source = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("error: can not read file {}, {}", path.display(), e);
                return ExitCode::FAILURE;
            }
        };
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_minizinc::LANGUAGE.into())
            .expect("Error loading MiniZinc grammar");
        let tree = match parser.parse(&source, None) {
            Some(t) => t,
            None => {
                eprintln!("error: tree-sitter failed to parse {}", path.display());
                return ExitCode::FAILURE;
            }
        };
        //println!("node print: {}", tree.root_node());
        //print_node(tree.root_node(), &source, 0);

        let lint_engine = LintEngine::new();
        for lint in lint_engine.lints {
            let diagnostics = lint.check(&tree, &source);
            diagnostics.iter().for_each(|diag| println!("{}", diag));
        }
    }
    ExitCode::SUCCESS
}
