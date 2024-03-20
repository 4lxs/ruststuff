use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    file_names: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    for file_name in cli.file_names {
        let path = Path::new(&file_name);
        let file = BufReader::new(File::open(path)?);

        for (linenr, line_result) in file.lines().enumerate() {
            let line = line_result?;
            println!("{}: {}", linenr, line);
        }
    }
    Ok(())
}
