use anyhow::{Context, Result};
use clap::Parser;

/// Represents the command-line arguments passed to the program.
///
/// This struct is used for parsing command-line arguments using the `clap` crate.
/// Each field in this struct corresponds to a possible command-line argument or flag.
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

/// The main entry point of the application.
///
/// # Errors
/// Returns an `Err` if any operation within the function fails, encapsulating the error
/// within the `Result`'s `Err` variant.
///
/// # Returns
/// On successful execution, returns `Ok(())`.
fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
