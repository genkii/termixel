use std::{io::BufReader, path::PathBuf};

use crate::{render_image::render_image, rgba_image::RgbaImage};

/// Renders the sprite provided by the user into the terminal
pub fn render_sprite(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open(&path)?;
    let decoder = png::Decoder::new(BufReader::new(file));

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
