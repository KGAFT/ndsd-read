use ndsd_read::{open_dsd_auto, DSDFormat};

fn read_n(path: &str, n: usize) -> (DSDFormat, Vec<Vec<u8>>) {
    let mut format = DSDFormat::default();
    let mut reader = open_dsd_auto(path, &mut format).expect("open");
    let ch = format.num_channels as usize;
    let mut bufs: Vec<Vec<u8>> = vec![vec![0u8; n]; ch];
    let mut got = 0usize;
    while got < n {
        let mut slices: Vec<&mut [u8]> =
            bufs.iter_mut().map(|b| &mut b.as_mut_slice()[got..]).collect();
        match reader.read(&mut slices, n - got).expect("read") {
            0 => break,
            k => got += k,
        }
    }
    assert_eq!(got, n, "short read from {}", path);
    (format, bufs)
}

fn diff(a: &[u8], b: &[u8]) -> (usize, Option<usize>) {
    let mut bad = 0usize;
    let mut first = None;
    for i in 0..a.len().min(b.len()) {
        if a[i] != b[i] {
            bad += 1;
            if first.is_none() {
                first = Some(i);
            }
        }
    }
    (bad, first)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n: usize = args[3].parse().unwrap();
    let (fa, a) = read_n(&args[1], n);
    let (fb, b) = read_n(&args[2], n);
    println!("A: {:?}", fa);
    println!("B: {:?}", fb);

    for ch in 0..a.len().min(b.len()) {
        let (bad, first) = diff(&a[ch], &b[ch]);
        println!(
            "ch{}: {}/{} bytes differ, first at {:?}",
            ch, bad, n, first
        );
    }
    // channel swap check
    if a.len() == 2 && b.len() == 2 {
        let (bad0, _) = diff(&a[0], &b[1]);
        let (bad1, _) = diff(&a[1], &b[0]);
        println!("swapped: ch0~B1 {} bad, ch1~B0 {} bad", bad0, bad1);
    }
    // bit reversal check on ch0
    let rev: Vec<u8> = b[0].iter().map(|x| x.reverse_bits()).collect();
    let (badr, _) = diff(&a[0], &rev);
    println!("bit-reversed ch0: {} bad", badr);

    // hex dump of first 32 bytes each
    println!("A ch0: {}", hex(&a[0][..32]));
    println!("B ch0: {}", hex(&b[0][..32]));
    println!("A ch1: {}", hex(&a[1][..32]));
    println!("B ch1: {}", hex(&b[1][..32]));
}

fn hex(d: &[u8]) -> String {
    d.iter().map(|b| format!("{:02x}", b)).collect()
}