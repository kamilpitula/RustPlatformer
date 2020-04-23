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
use super::texture_loader::TextureLoader;
use super::background::Background;
use super::character::Character;
use super::camera::{Camera, CameraDependentObject};
use super::map::{Map, TileType, AreaIndex};
use std::collections::HashMap;
use super::map_loader::MapLoader;
use super::colors;
use super::collider::Collider;
use super::moving_object::MovingObject;
use super::config;
use super::enemy::Enemy;

pub struct FirstLevel {
    background: Background,
    character: Character,
    key_press: Rc<RefCell<HashMap<Key, bool>>>,
    objects: HashMap<String, MovingObject>,
    objects_in_area: HashMap<AreaIndex, Vec<String>>,
    camera: Camera,
    map: Map,
    collider: Collider,
    enemy: Enemy,
}

impl FirstLevel {
    pub fn new(texture_loader: Rc<TextureLoader>, map_loader: Rc<MapLoader>) -> FirstLevel {
        let background_texture = texture_loader.load_texture("City Background.png");
        let foreground_texture = texture_loader.load_texture("City Foreground.png");

        let mut key_press = Rc::new(RefCell::new(HashMap::new()));
        (*key_press.borrow_mut()).insert(Key::Left, false);
        (*key_press.borrow_mut()).insert(Key::Right, false);
        (*key_press.borrow_mut()).insert(Key::Space, false);
        (*key_press.borrow_mut()).insert(Key::A, false);
        (*key_press.borrow_mut()).insert(Key::D, false);

        let map = Map::new(
            map_loader.load_map("level.map"),
            [0.0, 0.0],
            120,
            33,
            24.0,
            Rc::clone(&texture_loader)
        );

        let moving_object = MovingObject::new(
            [50.0, 300.0],
            [50.0, 50.0],
            [0.0, 1080.0],
            config::ACCELERATION,
            config::WALK_SPEED,
            config::JUMP_SPEED,
            "1ad31e1d-494a-41fe-bb9c-e7b8b83e59f1".to_string());

        let enemy_object = MovingObject::new(
            [300.0, 300.0],
            [50.0, 50.0],
            [0.0, 1080.0],
            config::ACCELERATION,
            config::WALK_SPEED,
            config::JUMP_SPEED,
            "5c8cd4c5-8d44-4326-bfa2-c803a30109fc".to_string());

        let box_size_x = moving_object.aabb.half_size[0] * 2.0;
        let box_size_y = moving_object.aabb.half_size[1] * 2.0;

        let enemy_box_size_x = enemy_object.aabb.half_size[0] * 2.0;
        let enemy_box_size_y = enemy_object.aabb.half_size[1] * 2.0;

        let mut objects = HashMap::<String, MovingObject>::new();
        objects.insert("1ad31e1d-494a-41fe-bb9c-e7b8b83e59f1".to_string(), moving_object);
        objects.insert("5c8cd4c5-8d44-4326-bfa2-c803a30109fc".to_string(), enemy_object);

        FirstLevel {
            background: Background::new(background_texture, foreground_texture, 2, 1000.0),
            character: Character::new(Rc::clone(&key_press), Rc::clone(&texture_loader), box_size_x, box_size_y),
            camera: Camera::new(460.0, 660.0),
            objects: objects,
            key_press: key_press,
            map: map,
            collider: Collider::new(8, 8, 120, 32),
            objects_in_area: HashMap::new(),
            enemy: Enemy::new(Rc::clone(&texture_loader), enemy_box_size_x, enemy_box_size_y),
        }
    }
}

impl GameState for FirstLevel {
    fn render(&mut self, ctx: &Context, mut gl: &mut GlGraphics, _glyphs: &mut GlyphCache) {
        self.background.render(&ctx, &mut gl);
        self.map.render(&ctx, &mut gl);
        self.character.render(&ctx, &mut gl, &mut self.objects.get_mut("1ad31e1d-494a-41fe-bb9c-e7b8b83e59f1").unwrap());
        self.enemy.render(&ctx, &mut gl, &mut self.objects.get_mut("5c8cd4c5-8d44-4326-bfa2-c803a30109fc").unwrap());
    }

    fn update(&mut self, args: &UpdateArgs) -> State<GameData> {
        for object in self.objects.values_mut() {
            self.collider.update_areas(object, &self.map, &mut self.objects_in_area);
            object.all_colliding_objects.clear();
        }
        self.collider.check_collisions(&mut self.objects_in_area, &mut self.objects);

        self.character.character_update(args.dt, &self.map, &mut self.objects.get_mut("1ad31e1d-494a-41fe-bb9c-e7b8b83e59f1").unwrap());
        self.enemy.character_update(args.dt, &self.map, &mut self.objects.get_mut("5c8cd4c5-8d44-4326-bfa2-c803a30109fc").unwrap());
        self.camera.update(&mut self.objects, &mut self.map, &mut self.character, &mut self.background, args.dt);
        return State::None;
    }

    fn key_press(&mut self, args: &Button) {
        match *args {
            Keyboard(Key::A) | Keyboard(Key::Left) => { self.character.pressed_left = true }
            Keyboard(Key::D) | Keyboard(Key::Right) => self.character.pressed_right = true,
            Keyboard(Key::S) | Keyboard(Key::Down) => self.character.pressed_drop = true,
            Keyboard(Key::Space) => self.character.pressed_jump = true,
            _ => {}
        }
    }

    fn key_release(&mut self, args: &Button) {
        match *args {
            Keyboard(Key::A) | Keyboard(Key::Left) => self.character.pressed_left = false,
            Keyboard(Key::D) | Keyboard(Key::Right) => self.character.pressed_right = false,
            Keyboard(Key::S) | Keyboard(Key::Down) => self.character.pressed_drop = false,
            Keyboard(Key::Space) => self.character.pressed_jump = false,
            _ => {}
        }
    }
}