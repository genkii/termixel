pub struct RgbaImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl RgbaImage {
    #[inline]
    pub fn get_pixel(&self, x: u32, y: u32) -> &[u8] {
        let index = ((y * self.width + x) * 4) as usize;
        &self.data[index..index + 4]
    }

    pub fn from_pixel(width: u32, height: u32, pixel: [u8; 4]) -> Self {
        let mut data = Vec::with_capacity((width * height * 4) as usize);

        for _ in 0..width * height {
            data.extend_from_slice(&pixel);
        }

        Self {
            width,
            height,
            data,
        }
    }
}
