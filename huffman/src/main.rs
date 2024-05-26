use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

use anyhow::{ensure, Result};
use clap::Parser;

/// This is a simple example of using clap with derive
#[derive(Parser, Debug)]
#[command(name = "huffman")]
struct Cli {
    file: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    ensure!(cli.file.exists());

    let filepath = Path::new(&cli.file);
    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file);

    let mut frequency = HashMap::new();

    let mut buffer = [0; 1024];
    loop {
        let num_read = reader.read(&mut buffer)?;

        if num_read == 0 {
            break;
        }

        for i in 0..num_read {
            let byte = buffer[i];
            let entry = frequency.entry(byte).or_insert(0usize);
            *entry += 1;
        }
    }

    let mut entries: Vec<(_, _)> = frequency.iter().collect();
    entries.sort_by_key(|&(_, v)| v);

    for (k, v) in &entries {
        if k.is_ascii_alphabetic() {
            print!("{} -> {}\n", **k as char, v);
        } else {
            print!("{} -> {}\n", k, v);
        }
    }

    Ok(())
}
