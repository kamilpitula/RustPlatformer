use super::moving_object::Moving_Object;
use super::config;
use std::collections::HashMap;
use piston::input::keyboard::Key;
use std::rc::Rc;
use std::cell::RefCell;
use super::texture_loader::Texture_Loader;
use super::animation_manager::AnimationManager;

pub struct Character {
    pub moving_object: Moving_Object,
    pub key_pressed_map: Rc<RefCell<HashMap<Key, bool>>>,
    pub pressed_left: bool,
    pub pressed_right: bool,
    pub pressed_jump: bool,
    current_state: CharacterState,
    current_animator: String,
    animation_manager: AnimationManager
}

impl Character {
    pub fn new(key_map: Rc<RefCell<HashMap<Key, bool>>>, tex_loader: Rc<Texture_Loader>) -> Character {

        let mut animation_manager = AnimationManager::new(tex_loader);
        animation_manager.add_sequence("idle".to_string(), "Character/Idle", 0.1, 1, 10);
        animation_manager.add_sequence("run".to_string(), "Character/Run", 0.1, 1, 8);
        animation_manager.add_sequence("jump".to_string(), "Character/Jump", 0.1, 1, 10);

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
            pressed_right: false,
            current_animator: "idle".to_string(),
            animation_manager: animation_manager
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
        
        self.select_animation();
        self.moving_object.update_physics(delta);
        self.animation_manager.get_animator(self.current_animator.to_string()).next(delta);
    }

    fn handle_stand(&mut self, delta: f64) {
        if !self.moving_object.on_ground {
            self.current_state = CharacterState::Jump;
        }
        if self.pressed_left | self.pressed_right {
            self.current_state = CharacterState::Walk;
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

    fn select_animation(&mut self) {
        if  self.moving_object.speed[1].abs() > 10.0 && !self.moving_object.on_ground {
            self.current_animator = "jump".to_string();
        } else if  self.moving_object.speed[0].abs() > 10.0 && self.moving_object.on_ground {
            self.current_animator = "run".to_string();
        } else {
            self.current_animator = "idle".to_string();
        }
    }
}

use super::renderable::Renderable;
use opengl_graphics::GlGraphics;
use graphics::Context;

impl Renderable for Character {
    fn render(&mut self, ctx: &Context, gl: &mut GlGraphics) {
        self.animation_manager.get_animator(self.current_animator.to_string()).render(ctx, gl, self.moving_object.position)
    }
}

enum CharacterState {
    Stand,
    Walk,
    Jump,
    GrabLedge
}