use ndsd_read::{open_dsd_auto, DSDFormat};
use std::time::Instant;

fn main() {
    for path in std::env::args().skip(1) {
        let mut format = DSDFormat::default();
        let mut reader = open_dsd_auto(&path, &mut format).expect("open");
        let ch = format.num_channels as usize;
        let n = 9408usize; // one DST frame per read
        let mut bufs: Vec<Vec<u8>> = vec![vec![0u8; n]; ch];
        // warmup
        for _ in 0..10 {
            let mut s: Vec<&mut [u8]> = bufs.iter_mut().map(|b| b.as_mut_slice()).collect();
            reader.read(&mut s, n).unwrap();
        }
        let frames = 300;
        let t = Instant::now();
        for _ in 0..frames {
            let mut s: Vec<&mut [u8]> = bufs.iter_mut().map(|b| b.as_mut_slice()).collect();
            if reader.read(&mut s, n).unwrap() == 0 { break; }
        }
        let el = t.elapsed().as_secs_f64();
        println!(
            "{:6.1} frames/s ({:5.2}x realtime @75fps)  {}",
            frames as f64 / el,
            frames as f64 / el / 75.0,
            path
        );
    }
}
