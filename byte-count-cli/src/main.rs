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
    let mut file = std::fs::File::open(path).unwrap();
    let mut buffer = Box::new([0u8; 8*1024]);
    let mut counts = [0u32; 0x100];
    loop
    {
        let read_count = file.read(&mut * buffer).unwrap();
        if read_count == 0 { break; }
        for &byte in &buffer[..read_count]
        {
            counts[byte as usize] += 1;
        }

        // Simulated additional computation time. The threaded version can actually be slightly more performant.
        // std::thread::sleep(std::time::Duration::from_millis(1));
    }

    print_byte_count(&counts);
}
