use image::DynamicImage;

pub struct ProcessedImage {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

pub fn load_and_format(path: &str) -> Result<ProcessedImage, String> {
    todo!()
}