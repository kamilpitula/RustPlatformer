use super::moving_object::Moving_Object;
use super::config;
use std::collections::HashMap;
use piston::input::keyboard::Key;
use std::rc::Rc;
use std::cell::RefCell;
use piston_window::rectangle;
use graphics::*;
use opengl_graphics::GlGraphics;
use graphics::Context;
use super::texture_loader::Texture_Loader;
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
    pub fn new(tex_loader: Rc<Texture_Loader>, box_size_x: f64, box_size_y: f64) -> Enemy {

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

    pub fn character_update(&mut self, delta: f64, map: &Map, moving_object: Rc<RefCell<Moving_Object>>){
        let mut object = moving_object.borrow_mut();
        
        self.handle_stand(delta, &mut object);
        
        object.update_physics(delta, &map);
        self.animation_manager.get_animator(self.current_animator.to_string()).next(delta);
    }

    fn handle_stand(&mut self, delta: f64, moving_object: &mut Moving_Object) {
        self.current_animator = "idle".to_string();
    }

    pub fn render(&mut self, ctx: &Context, gl: &mut GlGraphics, object: Rc<RefCell<Moving_Object>>) {
        

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