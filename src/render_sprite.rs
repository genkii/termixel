use crate::{render_image::render_image, rgba_image::RgbaImage};

use std::{io::BufReader, path::PathBuf};

/// Renders the sprite provided by the user into the terminal
pub fn render_sprite(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let mut decoder = png::Decoder::new(BufReader::new(file));

    decoder.set_transformations(
        png::Transformations::EXPAND | png::Transformations::STRIP_16 | png::Transformations::ALPHA,
    );

    let mut reader = decoder.read_info()?;
    let mut data = vec![
        0;
        reader
            .output_buffer_size()
            .ok_or("Failed to get buffer size")?
    ];

    let info = reader.next_frame(&mut data)?;

    let image = RgbaImage {
        width: info.width,
        height: info.height,
        data,
    };

    render_image(&image)?;

    Ok(())
}
