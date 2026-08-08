use image::RgbaImage;
use std::fmt::Write as FmtWrite;
use std::io::{Write, stdout};

/// renders an image to the terminal using unicode characters
pub fn render_image(image: &RgbaImage) -> Result<(), Box<dyn std::error::Error>> {
    let height = image.height();
    let width = image.width();

    let mut output = String::with_capacity((width * height * 20) as usize);

    // Iterate over the pixels in the image, printing unicode characters to the terminal
    // Each pixel is a whole line on the x axis and a half line on the y axis
    // That means two pixels fit vertically into a single line
    for pixel_y in (0..height).step_by(2) {
        for pixel_x in 0..width {
            let pixel = image.get_pixel(pixel_x, pixel_y);

            let pixel2 = if pixel_y + 1 < height {
                *image.get_pixel(pixel_x, pixel_y + 1)
            } else {
                image::Rgba([0, 0, 0, 0])
            };

            let top_alpha = pixel[3];
            let bottom_alpha = pixel2[3];

            // Both pixels are not transparent, so print a block character
            if top_alpha > 0 && bottom_alpha > 0 {
                write!(
                    output,
                    "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m▀",
                    pixel[0], pixel[1], pixel[2], pixel2[0], pixel2[1], pixel2[2],
                )?;
            }
            // Top pixel is transparent, bottom pixel is not, so print a half block character
            else if top_alpha == 0 && bottom_alpha > 0 {
                write!(
                    output,
                    "\x1b[38;2;{};{};{}m\x1b[49m▄",
                    pixel2[0], pixel2[1], pixel2[2]
                )?;
            }
            // Top pixel is not transparent, bottom pixel is transparent, so print a half block character
            else if top_alpha > 0 && bottom_alpha == 0 {
                write!(
                    output,
                    "\x1b[38;2;{};{};{}m\x1b[49m▀",
                    pixel[0], pixel[1], pixel[2],
                )?;
            }
            // Both pixels are transparent, so print a space character
            else {
                write!(output, "\x1b[39m\x1b[49m ")?;
            }
        }

        // Print a newline character to move to the next line
        output.push('\n');
    }

    let mut stdout = stdout();
    stdout.write_all(output.as_bytes())?;
    stdout.flush()?;

    Ok(())
}
