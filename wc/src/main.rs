use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use anyhow::Result;
use clap::Parser;

/// This is a simple example of using clap with derive
#[derive(Parser, Debug)]
#[command(name = "ccwc")]
struct Cli {
    #[arg(short, long)]
    count: bool,

    #[arg(short, long)]
    lines: bool,

    input: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let filepath = Path::new(&cli.input);
    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file);

    let mut total_bytes: usize = 0;
    let mut total_lines: usize = 0;

    let mut buffer = [0; 1024];
    loop {
        let num_read = reader.read(&mut buffer)?;

        if num_read == 0 {
            break;
        }

        for i in 0..num_read {
            let c = buffer[i];
            if c == b'\n' {
                total_lines += 1;
            }
        }

        total_bytes += num_read;
    }

    if cli.count {
        print!("{} {}\n", total_bytes, &cli.input);
    }

    if cli.lines {
        print!("{} {}\n", total_lines, &cli.input);
    }

    Ok(())
}
