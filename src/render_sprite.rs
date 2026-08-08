use crossterm::execute;
use crossterm::style::{Color, Print, SetBackgroundColor, SetForegroundColor};
use image::GenericImageView;
use std::io::{Write, stdout};
use std::path::PathBuf;

pub fn render_sprite(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let image = image::open(path)?;

    let height = image.height();
    let width = image.width();

    let mut stdout = stdout();

    for pixel_y in (0..height).step_by(2) {
        for pixel_x in 0..width {
            let pixel = image.get_pixel(pixel_x, pixel_y);
            let pixel2 = image.get_pixel(pixel_x, pixel_y + 1);

            let top_alpha = pixel[3];
            let bottom_alpha = pixel2[3];

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
            } else if top_alpha == 0 && bottom_alpha > 0 {
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
            } else if top_alpha > 0 && bottom_alpha == 0 {
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
            } else {
                execute!(
                    stdout,
                    SetBackgroundColor(Color::Reset),
                    SetForegroundColor(Color::Reset),
                    Print(" ")
                )?;
            }

            // If top is transparent then print ▄ with the color and backgroud color to reset
            // If bottom pixel is transparent then print ▀ with color and background color to reset
            // If both are transparent then print " " with the color and background color to reset
            // If both pixels are visible then print ▀ with top color foreground color and bottom color background color
        }

        execute!(stdout, Print("\n"))?;
    }

    stdout.flush()?;

    Ok(())
}
