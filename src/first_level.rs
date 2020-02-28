use opengl_graphics::{Texture, TextureSettings, GlGraphics, OpenGL, GlyphCache};
use piston::input::{RenderArgs, UpdateArgs, Button};
use piston::input::Button::Keyboard;
use piston::input::keyboard::Key;
use graphics::Context;
use std::path::PathBuf;
use std::rc::Rc;

use super::gamestate::GameState;
use super::renderable::Renderable;
use super::camera::camera_dependent_object;
use super::gamedata::GameData;
use super::states::State;
use super::texture_loader::Texture_Loader;
use super::background::Background;
use super::character::Character;

pub struct first_level{
    background: Background,
    character: Character
}

impl first_level {
    pub fn new(texture_loader: Rc<Texture_Loader>) -> first_level {
        let background_texture = texture_loader.load_texture("City Background.png");
        
        first_level {
            background: Background::new(background_texture),
            character: Character::new()
        }
    }
}

impl GameState for first_level{
    fn render(&mut self, ctx: &Context, mut gl: &mut GlGraphics, glyphs: &mut GlyphCache){
        self.background.render(&ctx, &mut gl);
        self.character.render(&ctx, &mut gl);
    }

    fn update(&mut self, args: &UpdateArgs) -> State<GameData> {
            return State::None;
    }

    fn key_press(&mut self, args: &Button){
        match *args {
            Keyboard(Key::A) | Keyboard(Key::Left) => self.background.move_object(2.0, 0.0),
            Keyboard(Key::D) | Keyboard(Key::Right) => self.background.move_object(-2.0, 0.0),
            _ => {/* Do nothing */}
        }
    }
}