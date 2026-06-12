use ndsd_read::{open_dsd_auto, DSDFormat};

// Read `total` bytes/ch from `path` using fixed chunk size `chunk`.
fn read_chunked(path: &str, total: usize, chunk: usize) -> Vec<Vec<u8>> {
    let mut format = DSDFormat::default();
    let mut reader = open_dsd_auto(path, &mut format).expect("open");
    let ch = format.num_channels as usize;
    let mut out: Vec<Vec<u8>> = vec![vec![0u8; total]; ch];
    let mut got = 0usize;
    while got < total {
        let want = chunk.min(total - got);
        let mut s: Vec<&mut [u8]> = out
            .iter_mut()
            .map(|b| &mut b.as_mut_slice()[got..got + want])
            .collect();
        match reader.read(&mut s, want).expect("read") {
            0 => break,
            k => got += k,
        }
    }
    assert_eq!(got, total, "short read");
    out
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let total = 9408 * 300; // 300 DST frames = 4 seconds of DSD128
    for chunk in [9408usize, 32768, 12345, 1234, 100000] {
        let a = read_chunked(&args[1], total, chunk);
        let b = read_chunked(&args[2], total, 65536);
        let mut bad = 0usize;
        let mut first = None;
        for c in 0..a.len() {
            for i in 0..total {
                if a[c][i] != b[c][i] {
                    bad += 1;
                    if first.is_none() { first = Some((c, i)); }
                }
            }
        }
        println!("chunk {:>6}: {} bad of {}x{}, first {:?}", chunk, bad, a.len(), total, first);
    }
}
