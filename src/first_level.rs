use opengl_graphics::{Texture, TextureSettings, GlGraphics, OpenGL, GlyphCache};
use piston::input::{RenderArgs, UpdateArgs, Button};
use graphics::Context;

use super::gamestate::GameState;
use super::renderable::Renderable;
use super::gamedata::GameData;
use super::states::State;

pub struct first_level{
    background_texture: Texture
}

impl first_level {
    pub fn new() -> first_level {
        
        let background = Texture::from_path(
            "aa",
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
        
    }

    fn update(&mut self, args: &UpdateArgs) -> State<GameData> {
            return State::None;
    }

    fn key_press(&mut self, args: &Button){
            
    }
}