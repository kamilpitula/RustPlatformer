use opengl_graphics::{Texture, TextureSettings, GlGraphics, OpenGL, GlyphCache};
use piston::input::{RenderArgs, UpdateArgs, Button};
use piston::input::Button::Keyboard;
use piston::input::keyboard::Key;
use graphics::Context;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;

use super::gamestate::GameState;
use super::renderable::Renderable;
use super::camera::camera_dependent_object;
use super::gamedata::GameData;
use super::states::State;
use super::texture_loader::Texture_Loader;
use super::background::Background;
use super::character::Character;
use std::collections::HashMap;

pub struct first_level{
    background: Background,
    character: Character,
    key_press: Rc<RefCell<HashMap<Key,bool>>>
}

impl first_level {
    pub fn new(texture_loader: Rc<Texture_Loader>) -> first_level {
        let background_texture = texture_loader.load_texture("City Background.png");
        let mut key_press = Rc::new(RefCell::new(HashMap::new()));
        (*key_press.borrow_mut()).insert(Key::Left, false); 
        (*key_press.borrow_mut()).insert(Key::Right, false); 
        (*key_press.borrow_mut()).insert(Key::Space, false); 

        first_level {
            background: Background::new(background_texture),
            character: Character::new(Rc::clone(&key_press)),
            key_press: key_press
        }
    }
}

impl GameState for first_level{
    fn render(&mut self, ctx: &Context, mut gl: &mut GlGraphics, glyphs: &mut GlyphCache){
        self.background.render(&ctx, &mut gl);
        self.character.render(&ctx, &mut gl);
    }

    fn update(&mut self, args: &UpdateArgs) -> State<GameData> {
            self.character.character_update(args.dt);
            return State::None;
    }

    fn key_press(&mut self, args: &Button){
        match *args {
            Keyboard(Key::A) | Keyboard(Key::Left) => {self.character.pressed_left = true},
            Keyboard(Key::D) | Keyboard(Key::Right) => self.character.pressed_right = true,
            Keyboard(Key::Space) => self.character.pressed_jump = true,
            _ => {}
        }
    }

    fn key_release(&mut self, args: &Button){
        match *args {
            Keyboard(Key::A) | Keyboard(Key::Left) => self.character.pressed_left = false,
            Keyboard(Key::D) | Keyboard(Key::Right) => self.character.pressed_right = false,
            Keyboard(Key::Space) => self.character.pressed_jump = false,
            _ => {}
        }
    }
}