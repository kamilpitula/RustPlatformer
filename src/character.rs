use super::moving_object::Moving_Object;

pub struct Character {
    moving_object: Moving_Object,
    current_state: CharacterState
}

impl Character {
    pub fn new() -> Character {
        Character { 
            moving_object: Moving_Object::new([0.0, 0.0], [10.0, 10.0]),
            current_state: CharacterState::Stand
        }
    }

    pub fn character_update(&self){
        match &self.current_state {
            Stand => {},
            Walk => {},
            Jump => {},
            GrabLedge => {}
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