use std::{
    cmp::Reverse,
    collections::HashMap,
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail, ensure, Result};
use bitvec::prelude::*;
use clap::Parser;

/// This is a simple example of using clap with derive
#[derive(Parser, Debug)]
#[command(name = "huffman")]
struct Cli {
    file: PathBuf,
}

struct BufferBitReader<'a> {
    buffer: &'a [u8],
    current_byte: Option<u8>,
    current_index: u8,
}

impl BufferBitReader<'_> {
    fn new(buffer: &[u8]) -> Result<Self> {
        Ok(BufferBitReader {
            buffer,
            current_byte: None,
            current_index: 0,
        })
    }

    fn next(
}

fn calculate_frequencies(contents: &[u8]) -> Result<HashMap<u8, usize>> {
    let mut frequencies = HashMap::new();

    for b in contents {
        let entry = frequencies.entry(*b).or_insert(0usize);
        *entry += 1;
    }

    Ok(frequencies)
}

fn create_huffman_encoding(frequencies: &HashMap<u8, usize>) -> Result<HashMap<u8, BitVec<u8>>> {
    let mut result = HashMap::new();

    let mut entries: Vec<(_, _)> = frequencies.iter().collect();
    entries.sort_by_key(|&(_, v)| Reverse(v)); // Sort by frequency descending

    for (k, v) in &entries {
        let mut bitvec = BitVec::new();
        for _ in 0..result.len() {
            bitvec.push(false);
        }
        bitvec.push(true);

        print!("{} -> {} -> {}\n", k, v, &bitvec);
        result.insert(**k, bitvec);
    }

    Ok(result)
}

fn create_decoding_from_encoding(
    encoding: &HashMap<u8, BitVec<u8>>,
) -> Result<HashMap<BitVec<u8>, u8>> {
    let mut result = HashMap::new();

    for (k, v) in encoding {
        result.insert(v.clone(), *k);
    }

    Ok(result)
}

fn encode(buffer: &[u8], encoding: &HashMap<u8, BitVec<u8>>) -> Result<Vec<u8>> {
    let mut result: BitVec<u8, Lsb0> = BitVec::new();

    for b in buffer {
        let encoded_byte = encoding
            .get(b)
            .ok_or_else(|| anyhow!("No encoding for {}", b))?;
        result.extend_from_bitslice(encoded_byte);
    }

    Ok(result.into_vec())
}

fn decode(buffer: &[u8], decoding: &HashMap<BitVec<u8>, u8>) -> Result<Vec<u8>> {
    let mut result = Vec::new();
    let mut buffer_bit_reader = BufferBitReader::new(buffer);

    while iter.next() {}

    for b in buffer {
        let mut encoded: BitVec<u8, Lsb0> = BitVec::new();

        // Read bitstring
    }

    Ok(result)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    ensure!(cli.file.exists());

    let filepath = Path::new(&cli.file);
    let mut file = File::open(filepath)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let frequencies = calculate_frequencies(&contents)?;

    let encoding = create_huffman_encoding(&frequencies)?;
    let decoding = create_decoding_from_encoding(&encoding)?;

    let encoded = encode(&contents, &encoding);
    let decoded = encode(&contents, &encoding);

    Ok(())
}
