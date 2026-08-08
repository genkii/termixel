use criterion::{Criterion, criterion_group, criterion_main};
use image::{Rgba, RgbaImage};
use std::hint::black_box;
use termixel::render_image::render_image;

fn benchmark_sprite(c: &mut Criterion) {
    let image = RgbaImage::from_pixel(16, 16, Rgba([255, 0, 0, 255]));

    c.bench_function("render 16x16 sprite", |b| {
        b.iter(|| {
            render_image(black_box(&image)).unwrap();
        });
    });
}

criterion_group!(benches, benchmark_sprite);
criterion_main!(benches);
