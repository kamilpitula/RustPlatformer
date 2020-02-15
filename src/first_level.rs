use opengl_graphics::{Texture, TextureSettings, GlGraphics, OpenGL, GlyphCache};
use piston::input::{RenderArgs, UpdateArgs, Button};
use graphics::Context;
use std::path::PathBuf;

use super::gamestate::GameState;
use super::renderable::Renderable;
use super::gamedata::GameData;
use super::states::State;

pub struct first_level{
    background_texture: Texture
}

impl first_level {
    pub fn new(assets_path: PathBuf) -> first_level {
        let background_path = assets_path.join("City Background.png");
        
        let background = Texture::from_path(
            background_path,
            &TextureSettings::new()
        ).unwrap();

        first_level {
            background_texture: background
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