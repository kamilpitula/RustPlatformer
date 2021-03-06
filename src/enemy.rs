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

pub struct Enemy {
    current_animator: String,
    animation_manager: AnimationManager,
    turned_back: bool,
    box_size_x: f64,
    box_size_y: f64
}

impl Enemy {
    pub fn new(tex_loader: Rc<TextureLoader>, box_size_x: f64, box_size_y: f64) -> Enemy {

        let mut animation_manager = AnimationManager::new(tex_loader);
        animation_manager.add_sequence("idle".to_string(), "Enemy/idle", 0.1, 1, 9, [box_size_x, box_size_y]);

        Enemy { 
            current_animator: "idle".to_string(),
            animation_manager: animation_manager,
            turned_back: false,
            box_size_x: box_size_x,
            box_size_y: box_size_y
        }
    }

    pub fn character_update(&mut self, delta: f64, map: &Map, moving_object: &mut MovingObject){

        self.handle_stand(delta, moving_object);

        moving_object.update_physics(delta, &map);
        self.animation_manager.get_animator(self.current_animator.to_string()).next(delta);
    }

    fn handle_stand(&mut self, _delta: f64, moving_object: &mut MovingObject) {
        self.current_animator = "idle".to_string();
    }

    pub fn render(&mut self, ctx: &Context, gl: &mut GlGraphics, moving_object: &mut MovingObject) {
        
        let mut color = colors::BLUE;

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