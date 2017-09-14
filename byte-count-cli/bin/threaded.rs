use std::thread;
use std::sync;
use std::io::Read;

fn main() {
    let file_path = std::env::args_os().skip(1).next().unwrap();

    let (read_tx, read_rx) = sync::mpsc::channel::<Vec<u8>>();
    let (proc_tx, proc_rx) = sync::mpsc::channel::<Vec<u8>>();

    let reader = thread::spawn(move || {
        let mut file = std::fs::File::open(file_path).unwrap();

        for mut buf in read_rx {
            let bytes_read = file.read(&mut buf).unwrap();

            if bytes_read == 0 { break; }

            unsafe { buf.set_len(bytes_read); }

            proc_tx.send(buf).unwrap();
        }
    });

    // Send the initial buffer.
    read_tx.send(vec![0u8; 8*1024]).unwrap();

    let mut back_buf = vec![0u8; 8*1024];
    let mut counts = vec![0u32; 256];

    for buf in proc_rx {
        // Have the read thread start the next file read operation.
        read_tx.send(back_buf).unwrap();

        // Replace back_buf with buf.
        back_buf = buf;

        // process back_buf
        for &byte in &back_buf {
            counts[byte as usize] += 1;
        }

        // Simulated additional computation time. The threaded version can actually be slightly more performant.
        // std::thread::sleep(std::time::Duration::from_millis(1));
    }

    reader.join().unwrap();

    for (i, v) in counts.iter().enumerate() {
        println!("0x{:02x}: {}", i, v);
    }
}
