use super::moving_object::Moving_Object;
use super::config;
use graphics::math::*;

pub struct Character {
    moving_object: Moving_Object,
    current_state: CharacterState,
    pub key_input: KeyInput
}

impl Character {
    pub fn new() -> Character {
        Character { 
            moving_object: Moving_Object::new([0.0, 0.0], [10.0, 10.0]),
            current_state: CharacterState::Stand,
            key_input: KeyInput::None
        }
    }

    pub fn character_update(&mut self, delta: f64){
        match &self.current_state {
            CharacterState::Stand => {
                self.moving_object.speed = [0.0, 0.0];
                if !self.moving_object.on_ground {
                    self.current_state = CharacterState::Jump;
                }
                match self.key_input {
                    KeyInput::GoLeft | KeyInput::GoRight => {
                        self.current_state = CharacterState::Walk
                    },
                    KeyInput::Jump => {
                        self.moving_object.speed = [0.0, -10.0];
                        self.current_state = CharacterState::Jump;
                    }
                    KeyInput::None => {
                        self.current_state = CharacterState::Stand;
                    },
                }

            },
            CharacterState::Walk => {
                match self.key_input {
                    KeyInput::GoRight => {
                        if self.moving_object.pushes_left_wall {
                            self.moving_object.speed = [0.0, 0.0];
                        } else {
                            self.moving_object.speed = [config::WALK_SPEED, 0.0];
                        }
                    },
                    KeyInput::GoLeft => {
                        if self.moving_object.pushes_left_wall {
                            self.moving_object.speed = [0.0, 0.0];
                        } else {
                            self.moving_object.speed = [config::WALK_SPEED, 0.0];
                        }
                    },
                    KeyInput::Jump => {
                        add(self.moving_object.speed, [0.0, -10.0]);
                    },
                    KeyInput::None => {self.current_state = CharacterState::Stand;},
                }
            },
            CharacterState::Jump => {},
            CharacterState::GrabLedge => {}
        }
        self.moving_object.update_physics(delta);
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
                .trans(character_x, character_y + 700.0);
        
        rectangle(color, square, point_trans, gl);
    }
}

enum CharacterState {
    Stand,
    Walk,
    Jump,
    GrabLedge
}

pub enum KeyInput {
    None,
    GoLeft,
    GoRight,
    Jump
}