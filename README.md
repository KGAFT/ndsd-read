# ndsd-read

A lightweight Rust library for reading DSD (Direct Stream Digital) audio files.

Supports both major container formats — **DSF** and **DFF (DSDIFF)** — including files with DST-compressed audio.

## Supported formats

| Format | Playback | Metadata |
|---|---|---|
| DSF | Full | Full (ID3) |
| DFF (DSDIFF) | Full (incl. DST) | Full (ID3 + legacy tags)* |
| WavPack DSD | TODO | — |
| SACD ISO | TODO | — |

\* Some older DSDIFF tag formats may not parse all fields. Both ID3 and native DSDIFF marker/label tags are supported.

## Features

- Auto-detection of DSF and DFF files via `open_dsd_auto`
- Streaming read API with per-channel buffers
- Seek by sample index or percentage
- ID3 metadata extraction (title, artist, album, cover art, lyrics, etc.)
- Optional DST decompression via a bundled C++ decoder (`dstdec` feature)
- Charset-aware text decoding for DSDIFF markers and labels

## Installation

```toml
[dependencies]
ndsd-read = "0.1"
```

To enable DST decoding (requires a C++ compiler):

```toml
[dependencies]
ndsd-read = { version = "0.1", features = ["dstdec"] }
```

## Quick start

```rust
use ndsd_read::{open_dsd_auto, DSDFormat};

fn main() -> std::io::Result<()> {
    let mut format = DSDFormat::default();
    let mut reader = open_dsd_auto("track.dsf", &mut format)?;

    println!(
        "{}ch @ {} Hz, {} samples",
        format.num_channels, format.sampling_rate, format.total_samples
    );

    if let Some(meta) = reader.get_metadata() {
        meta.pretty_print();
    }

    let ch = format.num_channels as usize;
    let mut bufs: Vec<Vec<u8>> = vec![vec![0u8; 4096]; ch];

    loop {
        let mut slices: Vec<&mut [u8]> = bufs.iter_mut().map(|b| b.as_mut_slice()).collect();
        match reader.read(&mut slices, 4096)? {
            0 => break,
            _ => { /* process bufs */ }
        }
    }
    Ok(())
}
```

## API

### `open_dsd_auto(path, format) -> io::Result<Box<dyn DSDReader>>`

Opens a DSF or DFF file, fills `format` with channel/sample-rate info, and returns a boxed reader. Prefer this over constructing `DFFReader`/`DSFReader` directly.

### `DSDReader` trait

| Method | Description |
|---|---|
| `read(data, bytes_per_channel)` | Fill per-channel buffers; returns bytes read per channel |
| `seek_percent(f64)` | Seek to a position in `[0.0, 1.0]` |
| `seek_samples(u64)` | Seek to an absolute sample index |
| `get_position_frames()` | Current frame position |
| `get_position_percent()` | Current position as `[0.0, 1.0]` |
| `get_metadata()` | Returns `Option<&DSDMeta>` |
| `eof()` | Whether the end of file has been reached |
| `reset()` | Rewind to the beginning |

### `DSDFormat`

```rust
pub struct DSDFormat {
    pub sampling_rate: u32,   // e.g. 2822400 (DSD64), 5644800 (DSD128)
    pub num_channels: u32,
    pub total_samples: u64,
    pub is_lsb_first: bool,
}
```

### `DSDMeta`

Populated from embedded ID3 tags. Available fields: `artist`, `album`, `title`, `comment`, `genre`, `year`, `lyrics`, `cover_art`, `id3_raw`.

## DST decoding

DST-compressed DFF files require the `dstdec` feature. The bundled C++ decoder (from the SACD foobar2000 plugin) is compiled automatically via `build.rs`. A C++17-capable compiler must be available at build time.

## Examples

```sh
# Read and verify a file (open, read some blocks, seek to 50%)
cargo run --example dst_check -- track.dff

# Benchmark decode throughput
cargo run --example dst_bench -- track.dff

# Compare two files sample-by-sample
cargo run --example dst_compare -- a.dff b.dff 65536
```
