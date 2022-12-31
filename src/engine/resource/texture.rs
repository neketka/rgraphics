use super::loadable::Loadable;

pub struct TextureData {}

impl Loadable<TextureData> for TextureData {
    fn load(path: &str) -> Result<TextureData, std::io::Error> {
        todo!()
    }
}
