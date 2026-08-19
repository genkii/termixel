use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use termixel_core::unicode::{
    cached_image::CachedImage, render::render_cached::render_cached, rgba_image::RgbaImage,
};

fn benchmark_sprite(c: &mut Criterion) {
    let image = RgbaImage::from_pixel(16, 16, [255, 0, 0, 255]);

    let cached = CachedImage::new(&image).unwrap();

    c.bench_function("render 16x16 sprite", |b| {
        b.iter(|| {
            render_cached(black_box(&cached)).unwrap();
        });
    });
}

criterion_group!(benches, benchmark_sprite);
criterion_main!(benches);
