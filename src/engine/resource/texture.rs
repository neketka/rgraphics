use std::{fs::File, io::BufReader};

use super::loadable::Loadable;

pub struct TextureData {
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>,
}

impl Loadable<TextureData> for TextureData {
    fn load(path: &str) -> Result<TextureData, std::io::Error> {
        let reader = BufReader::new(File::open(path)?);
        let img = image::load(reader, image::ImageFormat::Png).unwrap();
        Ok(TextureData {
            width: img.width() as i32,
            height: img.height() as i32,
            data: img.to_rgba8().to_vec(),
        })
    }
}
