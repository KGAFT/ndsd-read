use ndsd_read::{open_dsd_auto, DSDFormat};

fn main() {
    for path in std::env::args().skip(1) {
        let mut format = DSDFormat::default();
        match open_dsd_auto(&path, &mut format) {
            Err(e) => println!("OPEN-FAIL\t{}\t{}", e, path),
            Ok(mut reader) => {
                let ch = format.num_channels as usize;
                let mut bufs: Vec<Vec<u8>> = vec![vec![0u8; 32768]; ch];
                let mut total = 0usize;
                let mut err = None;
                for _ in 0..8 {
                    let mut slices: Vec<&mut [u8]> =
                        bufs.iter_mut().map(|b| b.as_mut_slice()).collect();
                    match reader.read(&mut slices, 32768) {
                        Ok(0) => break,
                        Ok(n) => total += n,
                        Err(e) => {
                            err = Some(e);
                            break;
                        }
                    }
                }
                if let Some(e) = err {
                    println!("READ-FAIL\t{}\tafter {} bytes/ch\t{}", e, total, path);
                    continue;
                }

                // Seek to the middle and read again to exercise the DSTI path.
                let mut seek_total = 0usize;
                let seek_err = reader.seek_percent(0.5).err().or_else(|| {
                    let mut slices: Vec<&mut [u8]> =
                        bufs.iter_mut().map(|b| b.as_mut_slice()).collect();
                    match reader.read(&mut slices, 32768) {
                        Ok(n) => {
                            seek_total = n;
                            None
                        }
                        Err(e) => Some(e),
                    }
                });
                match seek_err {
                    Some(e) => println!("SEEK-FAIL\t{}\t{}", e, path),
                    None if seek_total == 0 => println!("SEEK-EMPTY\t{}", path),
                    None => println!(
                        "OK\tread {} bytes/ch, post-seek {} bytes/ch\t{}",
                        total, seek_total, path
                    ),
                }
            }
        }
    }
}