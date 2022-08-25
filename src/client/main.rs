use std::fs::File;
use std::path::Path;

use anyhow::Context;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    // location of file to send
    #[clap(short, long, value_parser)]
    stl_file_location: String,
}

fn open_stl_file(stl_file_location: &str) -> anyhow::Result<File> {
    let path = Path::new(stl_file_location);
    let extension = path
        .extension()
        .expect("File does not contain extension. It must be of type .stl.");
    if extension != "stl" {
        anyhow::bail!("File must have .stl extension");
    }

    let f = File::open(stl_file_location)
        .with_context(|| format!("Unable to open .stl file at {}", path.display()))?;
    Ok(f)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let _file = open_stl_file(&args.stl_file_location)?;

    Ok(())
}
