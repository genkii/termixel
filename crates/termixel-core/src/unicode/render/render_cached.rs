use std::io::{self, Write};

use crate::unicode::cached_image::CachedImage;

pub fn render_cached(data: &CachedImage) -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    stdout.write_all(&data.output)?;
    stdout.flush()?;

    Ok(())
}
