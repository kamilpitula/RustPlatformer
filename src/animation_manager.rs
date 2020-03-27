use graphics::math::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::cell::RefMut;
use opengl_graphics::Texture;
use super::texture_loader::Texture_Loader;
use super::animator::Animator;

pub struct AnimationManager {
    animators: HashMap<String, RefCell<Animator>>,
    tex_loader: Rc<Texture_Loader>
}

impl AnimationManager {
    pub fn new(tex_loader: Rc<Texture_Loader>) -> AnimationManager {
        AnimationManager {
            animators: HashMap::<String, RefCell<Animator>>::new(),
            tex_loader: tex_loader
        }
    }

    pub fn add_sequence(&mut self, name: String, fileName: &str, interval: f64, start: i8, stop: i8 ) {
        let mut textures = Vec::<Texture>::new();
        for i in start..stop + 1 {
            let texture = self.tex_loader.load_texture(&format!("{} ({}).png", fileName, i));
            textures.push(texture);
        }
        let animator = RefCell::new(Animator::new(textures, interval));
        self.animators.insert(name, animator);
    }

    pub fn get_animator(&self, name: String) -> RefMut<Animator> {
        self.animators[&name].borrow_mut()
    }
}