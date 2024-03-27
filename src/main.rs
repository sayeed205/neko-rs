use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}
fn main() -> Result<()> {
    let args = Cli::parse();
    let path = args.path.display().to_string();
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("could not read file `{}`", path))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
