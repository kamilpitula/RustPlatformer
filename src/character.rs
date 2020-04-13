use super::moving_object::Moving_Object;
use super::config;
use std::collections::HashMap;
use piston::input::keyboard::Key;
use std::rc::Rc;
use std::cell::RefCell;
use piston_window::rectangle;
use super::texture_loader::Texture_Loader;
use super::animation_manager::AnimationManager;
use super::colors;
use super::map::Map;

pub struct Character {
    pub moving_object: Rc<RefCell<Moving_Object>>,
    pub key_pressed_map: Rc<RefCell<HashMap<Key, bool>>>,
    pub pressed_left: bool,
    pub pressed_right: bool,
    pub pressed_jump: bool,
    pub pressed_drop: bool,
    current_state: CharacterState,
    current_animator: String,
    animation_manager: AnimationManager,
    turned_back: bool,
    box_size_x: f64,
    box_size_y: f64
}

impl Character {
    pub fn new(key_map: Rc<RefCell<HashMap<Key, bool>>>, tex_loader: Rc<Texture_Loader>, object: Rc<RefCell<Moving_Object>>) -> Character {

        let mut animation_manager = AnimationManager::new(tex_loader);
        let moving_object = Moving_Object::new(
            [50.0, 300.0],
            [50.0, 50.0],
            [0.0, 1080.0],
            config::ACCELERATION,
            config::WALK_SPEED,
            config::JUMP_SPEED);

        let box_size_x = moving_object.aabb.half_size[0] * 2.0;
        let box_size_y = moving_object.aabb.half_size[1] * 2.0;

        animation_manager.add_sequence("idle".to_string(), "Character/Idle", 0.1, 1, 10, [box_size_x, box_size_y]);
        animation_manager.add_sequence("run".to_string(), "Character/Run", 0.1, 1, 8, [box_size_x, box_size_y]);
        animation_manager.add_sequence("jump".to_string(), "Character/Jump", 0.1, 1, 10, [box_size_x, box_size_y]);

        Character { 
            moving_object: object,
            current_state: CharacterState::Stand,
            key_pressed_map: key_map,
            pressed_jump: false,
            pressed_left: false,
            pressed_right: false,
            pressed_drop: false,
            current_animator: "idle".to_string(),
            animation_manager: animation_manager,
            turned_back: false,
            box_size_x: box_size_x,
            box_size_y: box_size_y
        }
    }

    pub fn character_update(&mut self, delta: f64, map: &Map){
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
        
        self.moving_object.borrow_mut().update_physics(delta, &map);
        self.animation_manager.get_animator(self.current_animator.to_string()).next(delta);
    }

    fn handle_stand(&mut self, delta: f64) {
        self.current_animator = "idle".to_string();
        if !self.moving_object.borrow_mut().on_ground {
            self.current_state = CharacterState::Jump;
        }

        if self.pressed_drop {
            self.moving_object.borrow_mut().drop();
        }

        if self.pressed_left | self.pressed_right {
            self.current_state = CharacterState::Walk;
        }
        else if self.pressed_jump {
            self.moving_object.borrow_mut().jump();
            self.current_state = CharacterState::Jump;
        }
        else {
            self.current_state = CharacterState::Stand;
        }
    }

    fn handle_walk(&mut self, delta: f64) {
        self.current_animator = "run".to_string();
        if self.pressed_right {
            self.turned_back = false;
            if self.moving_object.borrow_mut().pushes_right_wall {
                self.moving_object.borrow_mut().stop();
            } else {
                self.moving_object.borrow_mut().move_right(1.0);
            }
        }
        else if self.pressed_left {
            self.turned_back = true;
            if self.moving_object.borrow_mut().pushes_left_wall {
                self.moving_object.borrow_mut().stop();
            } else {
                self.moving_object.borrow_mut().move_left(1.0);
            }
        } else {
            self.current_state = CharacterState::Stand;
        }

        if self.pressed_jump {
            self.moving_object.borrow_mut().jump();
            self.current_state = CharacterState::Jump;
        }
    }

    fn handle_jump(&mut self, delta: f64) {
        self.current_animator = "jump".to_string();
        if self.moving_object.borrow_mut().on_ground {
            self.moving_object.borrow_mut().stop_falling();
            self.current_state = CharacterState::Stand;
            return;
        }
        
        if self.pressed_right {
            self.turned_back = false;
            if self.moving_object.borrow_mut().pushes_left_wall {
                self.moving_object.borrow_mut().stop();
            } else {
                self.moving_object.borrow_mut().move_right(0.7);
            }
        }
        else if self.pressed_left {
            self.turned_back = true;
            if self.moving_object.borrow_mut().pushes_left_wall {
                self.moving_object.borrow_mut().stop();
            } else {
                self.moving_object.borrow_mut().move_left(0.7);
            }
        }

        self.moving_object.borrow_mut().falling();
    }
}

use super::renderable::Renderable;
use opengl_graphics::GlGraphics;
use graphics::Context;

impl Renderable for Character {
    fn render(&mut self, ctx: &Context, gl: &mut GlGraphics) {
        use graphics::*;

        let mut color = colors::BLUE;	

        let character_x = self.moving_object.borrow_mut().position[0];	
        let character_y = self.moving_object.borrow_mut().position[1];

        let point_trans = ctx	
                .transform	
                .trans(character_x, character_y);

        // rectangle(color, [0.0, 0.0, self.box_size_x, self.box_size_y], point_trans, gl);
        
        self.animation_manager
            .get_animator(self.current_animator.to_string())
            .render(ctx, gl, self.moving_object.borrow_mut().position, self.turned_back)
    }
}

enum CharacterState {
    Stand,
    Walk,
    Jump,
    GrabLedge
}