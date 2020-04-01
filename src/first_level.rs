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
use super::gamedata::GameData;
use super::states::State;
use super::texture_loader::Texture_Loader;
use super::background::Background;
use super::character::Character;
use super::camera::{Camera, camera_dependent_object};
use super::map::{Map, TileType};
use std::collections::HashMap;
use super::map_loader::MapLoader;
use super::colors;

pub struct first_level{
    background: Background,
    character: Character,
    key_press: Rc<RefCell<HashMap<Key,bool>>>,
    objects: Vec<Box<camera_dependent_object>>,
    camera: Camera,
    map: Map
}

impl first_level {
    pub fn new(texture_loader: Rc<Texture_Loader>, map_loader: Rc<MapLoader>) -> first_level {
        let background_texture = texture_loader.load_texture("City Background.png");
        let foreground_texture = texture_loader.load_texture("City Foreground.png");

        let mut key_press = Rc::new(RefCell::new(HashMap::new()));
        (*key_press.borrow_mut()).insert(Key::Left, false); 
        (*key_press.borrow_mut()).insert(Key::Right, false); 
        (*key_press.borrow_mut()).insert(Key::Space, false);
        (*key_press.borrow_mut()).insert(Key::A, false); 
        (*key_press.borrow_mut()).insert(Key::D, false); 

        let map = Map::new(map_loader.load_map("level.map"), [0.0, 0.0], 40, 28, 24.0);

        first_level {
            background: Background::new(background_texture, foreground_texture, 2, 1000.0),
            character: Character::new(Rc::clone(&key_press), Rc::clone(&texture_loader)),
            camera: Camera::new(460.0, 660.0),
            objects: Vec::new(),
            key_press: key_press,
            map: map
        }
    }
}

impl GameState for first_level{
    fn render(&mut self, ctx: &Context, mut gl: &mut GlGraphics, glyphs: &mut GlyphCache){
        self.background.render(&ctx, &mut gl);
        self.map.render(&ctx, &mut gl);
        self.character.render(&ctx, &mut gl);
    }

    fn update(&mut self, args: &UpdateArgs) -> State<GameData> {
            self.character.character_update(args.dt, &self.map);
            self.camera.update(&mut self.objects, &mut self.map, &mut self.character, &mut self.background, args.dt);
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