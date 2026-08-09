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
}
