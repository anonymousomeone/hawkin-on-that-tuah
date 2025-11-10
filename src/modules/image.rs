pub struct Image {
    data: Vec<u8>,
    width: usize
}

impl Image {
    pub fn new(data: Vec<u8>, width: usize) -> Image {
        Image {
            data,
            width
        }
    }

    pub fn default() -> Image {
        Image {
            data: vec![],
            width: 0
        }
    }

    /// update image data.
    /// 
    /// this does not update image width!!!
    pub fn put_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    /// update image width.
    pub fn update_width(&mut self, width: usize) {
        self.width = width;
    }

    pub fn get_data(&self) -> &Vec<u8> {
        return &self.data
    }

    /// returns the pixel at x, y as RGBA. the image data should always be in BGRA (format returned by rusty-duplication)
    /// 
    /// 
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<(u8, u8, u8, u8)> {
        if x >= self.width || y >= self.data.len() / (4 * self.width) {
            return None; // Coordinates are out of bounds
        }

        let start_index = (y * self.width + x) * 4;
        let end_index = start_index + 3;
        let pixel_data = &self.data[start_index..=end_index];
        let (b, g, r, a) = (pixel_data[0], pixel_data[1], pixel_data[2], pixel_data[3]);

        Some((r, g, b, a))
    }

    pub fn get_rect(&self, x: u16, y: u16, w: u16, h: u16) -> Vec<u8> {
        let mut rect_data = Vec::with_capacity((w as u32 * h as u32 * 4) as usize);

        for row in y..(y + h) {
            let start_index = ((row as usize * self.width) + x as usize) * 4;
            let end_index = start_index + (w as usize * 4);

            rect_data.extend_from_slice(&self.data[start_index..end_index]);
        }

        rect_data
    }

    pub fn crop(&mut self, x: u16, y: u16, w: u16, h: u16) {
        let cropped_data = self.get_rect(x, y, w, h);
        self.data = cropped_data;
        self.width = w as usize;
    }
}