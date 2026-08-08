use std::path::PathBuf;

use crate::render_image::render_image;

/// Renders the sprite provided by the user into the terminal
pub fn render_sprite(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let image = image::open(path)?.into_rgba8();

    render_image(&image)?;

    Ok(())
}
