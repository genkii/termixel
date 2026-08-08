use std::io::{Write, stdout};

use crossterm::{
    execute,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
};
use image::RgbaImage;

/// renders an image to the terminal using unicode characters
pub fn render_image(image: &RgbaImage) -> Result<(), Box<dyn std::error::Error>> {
    let height = image.height();
    let width = image.width();

    let mut stdout = stdout();

    // Iterate over the pixels in the image, printing unicode characters to the terminal
    // Each pixel is a whole line on the x axis and a half line on the y axis
    // That means two pixels fit vertically into a single line
    for pixel_y in (0..height).step_by(2) {
        for pixel_x in 0..width {
            let pixel = image.get_pixel(pixel_x, pixel_y);
            let pixel2 = image.get_pixel(pixel_x, pixel_y + 1);

            let top_alpha = pixel[3];
            let bottom_alpha = pixel2[3];

            // Both pixels are not transparent, so print a block character
            if top_alpha > 0 && bottom_alpha > 0 {
                let color_top = Color::Rgb {
                    r: pixel[0],
                    g: pixel[1],
                    b: pixel[2],
                };
                let color_bottom = Color::Rgb {
                    r: pixel2[0],
                    g: pixel2[1],
                    b: pixel2[2],
                };

                execute!(
                    stdout,
                    SetForegroundColor(color_top),
                    SetBackgroundColor(color_bottom),
                    Print("▀")
                )?;
            }
            // Top pixel is transparent, bottom pixel is not, so print a half block character
            else if top_alpha == 0 && bottom_alpha > 0 {
                let color_bottom = Color::Rgb {
                    r: pixel2[0],
                    g: pixel2[1],
                    b: pixel2[2],
                };

                execute!(
                    stdout,
                    SetForegroundColor(color_bottom),
                    SetBackgroundColor(Color::Reset),
                    Print("▄")
                )?;
            }
            // Top pixel is not transparent, bottom pixel is transparent, so print a half block character
            else if top_alpha > 0 && bottom_alpha == 0 {
                let color_top = Color::Rgb {
                    r: pixel[0],
                    g: pixel[1],
                    b: pixel[2],
                };

                execute!(
                    stdout,
                    SetForegroundColor(color_top),
                    SetBackgroundColor(Color::Reset),
                    Print("▀")
                )?;
            }
            // Both pixels are transparent, so print a space character
            else {
                execute!(
                    stdout,
                    SetBackgroundColor(Color::Reset),
                    SetForegroundColor(Color::Reset),
                    Print(" ")
                )?;
            }
        }

        // Print a newline character to move to the next line
        execute!(stdout, Print("\n"))?;
    }

    stdout.flush()?;

    Ok(())
}
