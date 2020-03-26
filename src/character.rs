use super::moving_object::Moving_Object;
use super::config;
use graphics::math::*;
use std::collections::HashMap;
use piston::input::keyboard::Key;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Character {
    pub moving_object: Moving_Object,
    current_state: CharacterState,
    pub key_pressed_map: Rc<RefCell<HashMap<Key, bool>>>,
    pub pressed_left: bool,
    pub pressed_right: bool,
    pub pressed_jump: bool,
}

impl Character {
    pub fn new(key_map: Rc<RefCell<HashMap<Key, bool>>>) -> Character {
        Character { 
            moving_object: Moving_Object::new(
                [0.0, 700.0],
                [10.0, 10.0],
                [0.0, 1080.0],
                config::ACCELERATION,
                config::WALK_SPEED,
                config::JUMP_SPEED),
            current_state: CharacterState::Stand,
            key_pressed_map: key_map,
            pressed_jump: false,
            pressed_left: false,
            pressed_right: false
        }
    }

    pub fn character_update(&mut self, delta: f64){
        match &self.current_state {
            CharacterState::Stand => {
                self.handle_stand(delta);
            },
            CharacterState::Walk => {
                self.handle_walk(delta);
            },
            CharacterState::Jump => {
                self.handle_jump(delta);
            },
            CharacterState::GrabLedge => {}
        }
        self.moving_object.update_physics(delta);
    }

    fn handle_stand(&mut self, delta: f64) {
        if !self.moving_object.on_ground {
            self.current_state = CharacterState::Jump;
        }
        if self.pressed_left | self.pressed_right {
            self.current_state = CharacterState::Walk
        }
        else if self.pressed_jump {
            self.moving_object.jump();
            self.current_state = CharacterState::Jump;
        }
        else {
            self.current_state = CharacterState::Stand;
        }
    }

    fn handle_walk(&mut self, delta: f64) {
        if self.pressed_right {
            if self.moving_object.pushes_left_wall {
                self.moving_object.stop();
            } else {
                self.moving_object.move_right(1.0);
            }
        }
        else if self.pressed_left {
            if self.moving_object.pushes_left_wall {
                self.moving_object.stop();
            } else {
                self.moving_object.move_left(1.0);
            }
        }

        if self.pressed_jump {
            self.moving_object.jump();
            self.current_state = CharacterState::Jump;
        }
        else {
            self.current_state = CharacterState::Stand;
        }
    }

    fn handle_jump(&mut self, delta: f64) {
        self.moving_object.speed[1] += config::GRAVITY * delta;
        if self.moving_object.on_ground {
            self.current_state = CharacterState::Stand;
        }
        if self.pressed_right {
            if self.moving_object.pushes_left_wall {
                self.moving_object.stop();
            } else {
                self.moving_object.move_right(0.7);
            }
        }
        else if self.pressed_left {
            if self.moving_object.pushes_left_wall {
                self.moving_object.stop();
            } else {
                self.moving_object.move_left(0.7);
            }
        }
    }
}

use super::renderable::Renderable;
use opengl_graphics::GlGraphics;
use super::colors;
use graphics::Context;

impl Renderable for Character {
    fn render(&mut self, ctx: &Context, gl: &mut GlGraphics) {
        use graphics::*;
        let mut color = colors::BLUE;

        let character_x = self.moving_object.position[0];
        let character_y = self.moving_object.position[1];

        let square = rectangle::square(0.0, 0.0, 25.0);

        let point_trans = ctx
                .transform
                .trans(character_x, character_y);
        
        rectangle(color, square, point_trans, gl);
    }
}

enum CharacterState {
    Stand,
    Walk,
    Jump,
    GrabLedge
}