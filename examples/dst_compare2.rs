use ndsd_read::{open_dsd_auto, DSDFormat};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut fa = DSDFormat::default();
    let mut fb = DSDFormat::default();
    let mut ra = open_dsd_auto(&args[1], &mut fa).expect("open A");
    let mut rb = open_dsd_auto(&args[2], &mut fb).expect("open B");
    let ch = fa.num_channels as usize;
    let cfs = 9408usize; // bytes per channel per DST frame at DSD128
    let total = fb.total_samples.min(fa.total_samples);

    let n = cfs * 2;
    let mut a: Vec<Vec<u8>> = vec![vec![0u8; n]; ch];
    let mut b: Vec<Vec<u8>> = vec![vec![0u8; n]; ch];

    for pct in [5u64, 10, 25, 40, 50, 60, 75, 90, 99] {
        let frame = (total * pct / 100) / cfs as u64;
        let pos = frame * cfs as u64;
        ra.seek_samples(pos).expect("seek A");
        rb.seek_samples(pos).expect("seek B");
        let mut ga = 0; let mut gb = 0;
        while ga < n {
            let mut s: Vec<&mut [u8]> = a.iter_mut().map(|x| &mut x.as_mut_slice()[ga..]).collect();
            match ra.read(&mut s, n - ga) { Ok(0) => break, Ok(k) => ga += k, Err(e) => { println!("pct {}: A read err {}", pct, e); break; } }
        }
        while gb < n {
            let mut s: Vec<&mut [u8]> = b.iter_mut().map(|x| &mut x.as_mut_slice()[gb..]).collect();
            match rb.read(&mut s, n - gb) { Ok(0) => break, Ok(k) => gb += k, Err(e) => { println!("pct {}: B read err {}", pct, e); break; } }
        }
        let m = ga.min(gb);
        let mut bad = 0usize; let mut first = None;
        for c in 0..ch {
            for i in 0..m {
                if a[c][i] != b[c][i] { bad += 1; if first.is_none() { first = Some((c, i)); } }
            }
        }
        println!("pct {:>2} frame {:>7}: cmp {} bytes/ch, {} bad, first {:?}", pct, frame, m, bad, first);
    }
}
