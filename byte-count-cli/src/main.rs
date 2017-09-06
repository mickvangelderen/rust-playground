extern crate byte_count;

use byte_count::byte_count;

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
    print_byte_count(&byte_count(&[0, 1, 1, 2, 4, 100, 100]));
}
