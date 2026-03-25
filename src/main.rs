use std::env;
use std::path::PathBuf;
use std::process;

use tree_it::app::run::{run, CliRequest};

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
        if arg == "--profile" {
            let value = args
                .next()
                .ok_or_else(|| "Missing value for --profile".to_string())?;
            profile = Some(value.to_lowercase());
        } else if target_path.is_none() {
            target_path = Some(PathBuf::from(arg));
        } else {
            return Err(format!("Unexpected argument:{arg}"));
        }
    }

    let target_path = match target_path {
        Some(path) => path,
        None => env::current_dir().map_err(|e| format!("failed to get current directory: {e}"))?,
    };

    Ok(CliRequest { target_path, profile })
}