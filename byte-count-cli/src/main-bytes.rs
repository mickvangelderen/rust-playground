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
    let path = std::env::args_os().skip(1).next().unwrap();
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut counts = [0u32; 0x100];
    for byte in reader.bytes() {
        counts[byte.unwrap() as usize] += 1;
    }

    print_byte_count(&counts);
}
