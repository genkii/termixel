use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use termixel::{render_image::render_image, rgba_image::RgbaImage};

fn benchmark_sprite(c: &mut Criterion) {
    let image = RgbaImage::from_pixel(16, 16, [255, 0, 0, 255]);

    c.bench_function("render 16x16 sprite", |b| {
        b.iter(|| {
            render_image(black_box(&image)).unwrap();
        });
    });
}

criterion_group!(benches, benchmark_sprite);
criterion_main!(benches);
