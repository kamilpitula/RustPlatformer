use opengl_graphics::{Texture, TextureSettings};
use std::path::PathBuf;
use std::rc::Rc;

pub struct Texture_Loader{
    assets_path: Rc<PathBuf>
}

impl Texture_Loader {
    pub fn new(assets_path: Rc<PathBuf>) -> Texture_Loader {
        Texture_Loader{
            assets_path: assets_path
        }
    }
    pub fn load_texture(&self, path: &str) -> Texture {
         let texture_path = self.assets_path.join(path);

         Texture::from_path(
            texture_path,
            &TextureSettings::new()
        ).unwrap()
    }
}