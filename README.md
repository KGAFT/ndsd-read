# ndsd-read

A lightweight rust library for reading dsd data, from the dsd container.

# Supported formats:

Format | Support
--- | --- 
dsf | Supports playback/meta parsing fully
dsdiff(dff) | Supports playback with dst decoding. Meta parsing is supported*
Wavpack dsd | TODO
SACD dsd | TODO


*The dsdiff meta parsing, may not parse some old tags for track info.
However, it works both with id3 tags and old tags

# DST Decoding

This crate is using dst decoder from SACD foobar extension. If you want to read dst files, you need to enable it with feature dstdec.

