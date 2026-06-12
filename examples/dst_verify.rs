use ndsd_read::{open_dsd_auto, DSDFormat};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut fa = DSDFormat::default();
    let mut fb = DSDFormat::default();
    let mut ra = open_dsd_auto(&args[1], &mut fa).expect("open A");
    let mut rb = open_dsd_auto(&args[2], &mut fb).expect("open B");
    let ch = fa.num_channels as usize;
    let n = 9408usize; // one DST frame per iteration
    let mut a: Vec<Vec<u8>> = vec![vec![0u8; n]; ch];
    let mut b: Vec<Vec<u8>> = vec![vec![0u8; n]; ch];
    let total_frames = (fb.total_samples.min(fa.total_samples) / n as u64) as usize;

    let mut bad_frames = 0usize;
    let mut logged = 0usize;
    for frame in 0..total_frames {
        let mut sa: Vec<&mut [u8]> = a.iter_mut().map(|x| x.as_mut_slice()).collect();
        let ka = match ra.read(&mut sa, n) {
            Ok(k) => k,
            Err(e) => { println!("frame {}: A read error: {}", frame, e); break; }
        };
        let mut sb: Vec<&mut [u8]> = b.iter_mut().map(|x| x.as_mut_slice()).collect();
        let kb = match rb.read(&mut sb, n) {
            Ok(k) => k,
            Err(e) => { println!("frame {}: B read error: {}", frame, e); break; }
        };
        if ka != n || kb != n {
            println!("frame {}: short read a={} b={}", frame, ka, kb);
            break;
        }
        let mut bad = 0usize;
        for c in 0..ch {
            for i in 0..n {
                if a[c][i] != b[c][i] { bad += 1; }
            }
        }
        if bad > 0 {
            bad_frames += 1;
            if logged < 60 {
                println!("frame {}: {} bad bytes of {}", frame, bad, n * ch);
                logged += 1;
            }
        }
        if frame % 20000 == 0 {
            eprintln!("progress {}/{} bad_frames={}", frame, total_frames, bad_frames);
        }
    }
    println!("DONE: {} bad frames of {}", bad_frames, total_frames);
}
