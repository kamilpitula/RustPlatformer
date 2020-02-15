use opengl_graphics::{Texture, TextureSettings, GlGraphics, OpenGL, GlyphCache};
use piston::input::{RenderArgs, UpdateArgs, Button};
use graphics::Context;
use std::path::PathBuf;
use std::rc::Rc;

use super::gamestate::GameState;
use super::renderable::Renderable;
use super::gamedata::GameData;
use super::states::State;
use super::texture_loader::Texture_Loader;

pub struct first_level{
    background_texture: Texture,
}

impl first_level {
    pub fn new(texture_loader: Rc<Texture_Loader>) -> first_level {
        let background_texture = texture_loader.load_texture("City Background.png");
        
        first_level {
            background_texture: background_texture
        }
    }
}

impl GameState for first_level{
    fn render(&mut self, ctx: &Context, mut gl: &mut GlGraphics, glyphs: &mut GlyphCache){
        use graphics::*;
        
        image(&self.background_texture, ctx.transform, gl);
    }

    fn update(&mut self, args: &UpdateArgs) -> State<GameData> {
            return State::None;
    }

    fn key_press(&mut self, args: &Button){
            
    }
}