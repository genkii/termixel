# Termixel Core

A fast Rust library for rendering images in the terminal.

`termixel-core` is the rendering library behind [Termixel](https://github.com/genkii/termixel).

It is made for rendering images directly in the terminal, with a focus on keeping the rendering process fast and avoiding unnecessary work.

## Features

- Render RGBA images in the terminal
- Unicode and ANSI-based rendering
- Cache rendered images for fast repeated output
- Minimal binary size and footprint

## Example

```rust
use termixel_core::unicode::{
    cached_image::CachedImage,
    rgba_image::RgbaImage,
};

let image = RgbaImage::from_pixel(16, 16, [255, 0, 0, 255]);
let cached = CachedImage::new(&image)?;
```

## Status

This project is still in early development, so the API may change.
