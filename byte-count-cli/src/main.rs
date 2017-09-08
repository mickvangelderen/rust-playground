extern crate byte_count;

use byte_count::byte_count;
use std::error::Error;
use std::io::Read;

fn print_byte_count(counts: &[u32; 0x100]) {
    for (byte, &count) in counts
        .iter()
        .enumerate()
        .filter(|&(_, &count)| count > 0)
    {
        println!("0x{:>02x}: {}", byte, count);
    }
}

fn main() {
    let path = match std::env::args_os().skip(1).next() {
        Some(value) => std::path::PathBuf::from(value),
        None => panic!("Pass a file path to count the bytes from"),
    };

    let mut file = match std::fs::File::open(&path) {
        Err(err) => panic!("Failed to open {}: {}", path.display(), err.description()),
        Ok(file) => file
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(err) => panic!("Failed to read {}: {}", path.display(), err.description()),
        Ok(contents) => contents
    };

    print_byte_count(&byte_count(&contents.into_bytes()));
}
