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
use super::map::{Map, TileType, AreaIndex};
use std::collections::HashMap;
use super::map_loader::MapLoader;
use super::colors;
use super::collider::Collider;
use super::moving_object::Moving_Object;
use super::config;

pub struct first_level{
    background: Background,
    character: Character,
    key_press: Rc<RefCell<HashMap<Key,bool>>>,
    objects: HashMap<String, Rc<RefCell<Moving_Object>>>,
    objectsInArea: HashMap<AreaIndex, HashMap<String, Rc<RefCell<Moving_Object>>>>,
    camera: Camera,
    map: Map,
    collider: Collider
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

        let map = Map::new(map_loader.load_map("level.map"), [0.0, 0.0], 120, 33, 24.0, Rc::clone(&texture_loader));
        
        let moving_object = Rc::new(RefCell::new(Moving_Object::new(
            [50.0, 300.0],
            [50.0, 50.0],
            [0.0, 1080.0],
            config::ACCELERATION,
            config::WALK_SPEED,
            config::JUMP_SPEED,
            "1ad31e1d-494a-41fe-bb9c-e7b8b83e59f1".to_string())));

        let box_size_x = moving_object.borrow().aabb.half_size[0] * 2.0;
        let box_size_y = moving_object.borrow().aabb.half_size[1] * 2.0;

        let mut objects = HashMap::<String, Rc<RefCell<Moving_Object>>>::new();
        objects.insert("character".to_string(), Rc::clone(&moving_object));

        first_level {
            background: Background::new(background_texture, foreground_texture, 2, 1000.0),
            character: Character::new(Rc::clone(&key_press), Rc::clone(&texture_loader), box_size_x, box_size_y),
            camera: Camera::new(460.0, 660.0),
            objects: objects,
            key_press: key_press,
            map: map,
            collider: Collider::new(8, 8, 120, 32),
            objectsInArea: HashMap::new(),
        }
    }
}

impl GameState for first_level{
    fn render(&mut self, ctx: &Context, mut gl: &mut GlGraphics, glyphs: &mut GlyphCache){
        self.background.render(&ctx, &mut gl);
        self.map.render(&ctx, &mut gl);
        self.character.render(&ctx, &mut gl, Rc::clone(&self.objects["character"]));
    }

    fn update(&mut self, args: &UpdateArgs) -> State<GameData> {
        
        for object in self.objects.values()  {
            self.collider.update_areas(Rc::clone(&object), &self.map, &mut self.objectsInArea);
            object.borrow_mut().allCollidingObjects.clear();
        }    
        self.collider.check_collisions(&mut self.objectsInArea);

        self.character.character_update(args.dt, &self.map, Rc::clone(&self.objects["character"]));
        self.camera.update(&mut self.objects, &mut self.map, &mut self.character, &mut self.background, args.dt);
        return State::None;
    }

    fn key_press(&mut self, args: &Button){
        match *args {
            Keyboard(Key::A) | Keyboard(Key::Left) => {self.character.pressed_left = true},
            Keyboard(Key::D) | Keyboard(Key::Right) => self.character.pressed_right = true,
            Keyboard(Key::S) | Keyboard(Key::Down) => self.character.pressed_drop = true,
            Keyboard(Key::Space) => self.character.pressed_jump = true,
            _ => {}
        }
    }

    fn key_release(&mut self, args: &Button){
        match *args {
            Keyboard(Key::A) | Keyboard(Key::Left) => self.character.pressed_left = false,
            Keyboard(Key::D) | Keyboard(Key::Right) => self.character.pressed_right = false,
            Keyboard(Key::S) | Keyboard(Key::Down) => self.character.pressed_drop = false,
            Keyboard(Key::Space) => self.character.pressed_jump = false,
            _ => {}
        }
    }
}