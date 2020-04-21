use super::moving_object::MovingObject;
use super::config;
use std::collections::HashMap;
use piston::input::keyboard::Key;
use std::rc::Rc;
use std::cell::RefCell;
use piston_window::rectangle;
use graphics::*;
use opengl_graphics::GlGraphics;
use graphics::Context;
use super::texture_loader::TextureLoader;
use super::animation_manager::AnimationManager;
use super::colors;
use super::map::Map;

pub struct Character {
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
    pub fn new(key_map: Rc<RefCell<HashMap<Key, bool>>>, tex_loader: Rc<TextureLoader>, box_size_x: f64, box_size_y: f64) -> Character {

        let mut animation_manager = AnimationManager::new(tex_loader);

        animation_manager.add_sequence("idle".to_string(), "Character/Idle", 0.1, 1, 10, [box_size_x, box_size_y]);
        animation_manager.add_sequence("run".to_string(), "Character/Run", 0.1, 1, 8, [box_size_x, box_size_y]);
        animation_manager.add_sequence("jump".to_string(), "Character/Jump", 0.1, 1, 10, [box_size_x, box_size_y]);

        Character { 
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

    pub fn character_update(&mut self, delta: f64, map: &Map, moving_object: Rc<RefCell<MovingObject>>){
        let mut object = moving_object.borrow_mut();
        match &self.current_state {
            CharacterState::Stand => {
                self.handle_stand(delta, &mut object);
            },
            CharacterState::Walk => {
                self.handle_walk(delta, &mut object);
            },
            CharacterState::Jump => {
                self.handle_jump(delta, &mut object);
            },
            CharacterState::GrabLedge => {}
        }
        
        object.update_physics(delta, &map);
        self.animation_manager.get_animator(self.current_animator.to_string()).next(delta);
    }

    fn handle_stand(&mut self, _delta: f64, moving_object: &mut MovingObject) {
        self.current_animator = "idle".to_string();
        if !moving_object.on_ground {
            self.current_state = CharacterState::Jump;
        }

        if self.pressed_drop {
            moving_object.drop();
        }

        if self.pressed_left | self.pressed_right {
            self.current_state = CharacterState::Walk;
        }
        else if self.pressed_jump {
            moving_object.jump();
            self.current_state = CharacterState::Jump;
        }
        else {
            self.current_state = CharacterState::Stand;
        }
    }

    fn handle_walk(&mut self, _delta: f64, moving_object: &mut MovingObject) {
        self.current_animator = "run".to_string();
        if self.pressed_right {
            self.turned_back = false;
            if moving_object.pushes_right_wall {
                moving_object.stop();
            } else {
                moving_object.move_right(1.0);
            }
        }
        else if self.pressed_left {
            self.turned_back = true;
            if moving_object.pushes_left_wall {
                moving_object.stop();
            } else {
                moving_object.move_left(1.0);
            }
        } else {
            self.current_state = CharacterState::Stand;
        }

        if self.pressed_jump {
            moving_object.jump();
            self.current_state = CharacterState::Jump;
        }
    }

    fn handle_jump(&mut self, _delta: f64, moving_object: &mut MovingObject) {
        self.current_animator = "jump".to_string();
        if moving_object.on_ground {
            moving_object.stop_falling();
            self.current_state = CharacterState::Stand;
            return;
        }
        
        if self.pressed_right {
            self.turned_back = false;
            if moving_object.pushes_left_wall {
                moving_object.stop();
            } else {
                moving_object.move_right(0.7);
            }
        }
        else if self.pressed_left {
            self.turned_back = true;
            if moving_object.pushes_left_wall {
                moving_object.stop();
            } else {
                moving_object.move_left(0.7);
            }
        }

        moving_object.falling();
    }

    pub fn render(&mut self, ctx: &Context, gl: &mut GlGraphics, object: Rc<RefCell<MovingObject>>) {
        

        let mut color = colors::BLUE;	
        let moving_object = object.borrow_mut();

        let character_x = moving_object.position[0];	
        let character_y = moving_object.position[1];

        let point_trans = ctx	
                .transform	
                .trans(character_x, character_y);

        // rectangle(color, [0.0, 0.0, self.box_size_x, self.box_size_y], point_trans, gl);
        
        self.animation_manager
            .get_animator(self.current_animator.to_string())
            .render(ctx, gl, moving_object.position, self.turned_back)
    }
}

enum CharacterState {
    Stand,
    Walk,
    Jump,
    GrabLedge
}