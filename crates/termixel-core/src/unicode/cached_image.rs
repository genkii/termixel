use crate::unicode::rgba_image::RgbaImage;
use std::io::Write;

pub struct CachedImage {
    pub output: Vec<u8>,
}

impl CachedImage {
    pub fn new(image: &RgbaImage) -> Result<Self, Box<dyn std::error::Error>> {
        let height = image.height;
        let width = image.width;

        let mut output = Vec::with_capacity((width * height * 20) as usize);

        for pixel_y in (0..height).step_by(2) {
            for pixel_x in 0..width {
                let pixel = image.get_pixel(pixel_x, pixel_y);

                let pixel2 = if pixel_y + 1 < height {
                    image.get_pixel(pixel_x, pixel_y + 1)
                } else {
                    &[0, 0, 0, 0]
                };

                let top_alpha = pixel[3];
                let bottom_alpha = pixel2[3];

                if top_alpha > 0 && bottom_alpha > 0 {
                    write!(
                        output,
                        "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m▀",
                        pixel[0], pixel[1], pixel[2], pixel2[0], pixel2[1], pixel2[2],
                    )?;
                } else if top_alpha == 0 && bottom_alpha > 0 {
                    write!(
                        output,
                        "\x1b[38;2;{};{};{}m\x1b[49m▄",
                        pixel2[0], pixel2[1], pixel2[2],
                    )?;
                } else if top_alpha > 0 && bottom_alpha == 0 {
                    write!(
                        output,
                        "\x1b[38;2;{};{};{}m\x1b[49m▀",
                        pixel[0], pixel[1], pixel[2],
                    )?;
                } else {
                    write!(output, "\x1b[39m\x1b[49m ")?;
                }
            }

            output.push(b'\n');
        }

        output.extend_from_slice(b"\x1b[0m");
        Ok(Self { output })
    }
}
