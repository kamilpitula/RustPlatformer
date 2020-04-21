use opengl_graphics::{Texture, TextureSettings};
use std::path::PathBuf;
use std::rc::Rc;

pub struct TextureLoader {
    assets_path: Rc<PathBuf>
}

impl TextureLoader {
    pub fn new(assets_path: Rc<PathBuf>) -> TextureLoader {
        TextureLoader {
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