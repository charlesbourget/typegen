use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::Write;
use progenitor::GenerationSettings;
use typegen::generate_code;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to OpenAPI specification
    spec: String,

    /// Path to Output file
    output: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let generation_settings = GenerationSettings::default();
    let content = generate_code(args.spec, &generation_settings)?;

    // Create output file
    File::create(args.output)
        .context("Could not create output file")?
        .write_all(content.as_bytes())
        .context("Could not write to output file")?;

    Ok(())
}
