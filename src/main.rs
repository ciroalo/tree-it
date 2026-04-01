use std::env;
use std::path::PathBuf;
use std::process;

use tree_it::app::run::{CliRequest, run};

fn main() {
    let request = match parse_args() {
        Ok(request) => request,
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };

    match run(request) {
        Ok(output) => {
            println!("{output}");
        }
        Err(error) => {
            eprintln!("Error: {error}");
            process::exit(1);
        }
    }
}

fn parse_args() -> Result<CliRequest, String> {
    let mut args = env::args().skip(1);

    let mut target_path: Option<PathBuf> = None;
    let mut profile: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => {
                print_help();
                process::exit(0);
            }
            "--profile" => {
                let value = args
                    .next()
                    .ok_or_else(|| "Missing value for --profile".to_string())?;
                profile = Some(value.to_lowercase());
            }
            _ => {
                if target_path.is_none() {
                    target_path = Some(PathBuf::from(arg));
                } else {
                    return Err(format!("Unexpected arguments: {arg}"));
                }
            }
        }
    }

    let target_path = match target_path {
        Some(path) => path,
        None => env::current_dir().map_err(|e| format!("failed to get current directory: {e}"))?,
    };

    Ok(CliRequest {
        target_path,
        profile,
    })
}

fn print_help() {
    println!(
        "\
tree-it 0.1.1

Generate documentation-friendly directory trees.

USAGE:
    tree-it [path] [--profile <name>]
    tree-it --help

ARGS:
    [path]
        Target directory to analyze.
        If omitted, the current working directory is used.

OPTIONS:
    --profile <name>
        Generate only the selected profile from .treeignore

    -h, --help
        Show this help message

BEHAVIOR:
    - If .treeignore exists, it is used
    - Otherwise, if .gitignore exists, it is used
    - Otherwise, the full tree is generated
    - Without --profile, tree-it prints:
        * the general tree
        * all profile trees defined in .treeignore

EXAMPLES:
    tree-it
    tree-it ./project
    tree-it --profile tree_docs
    tree-it ./project --profile tree_docs

DEVELOPMENT:
    When using Cargo, pass program arguments after --:
        cargo run -- --help
        cargo run -- --profile tree_docs
"
    );
}
