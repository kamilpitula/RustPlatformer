use opengl_graphics::{Texture, GlGraphics};
use graphics::Context;
use super::renderable::Renderable;
use super::camera::CameraDependentObject;
use super::config;

pub struct Background {
    background_texture: Texture,
    foreground_texture: Texture,
    pub x: f64,
    y: f64,
    repeat: i8,
    width: f64,
    pub left: bool,
    pub right: bool,
    pub combined_width: f64
}

impl Background {
    pub fn new(background_texture: Texture, foreground_texture: Texture, repeat: i8, width: f64) -> Background {
        Background {
            background_texture,
            foreground_texture,
            x: 0.0,
            y: 0.0,
            width,
            left: false,
            right: false,
            repeat,
            combined_width: width * repeat as f64
        }
    }
}

impl Renderable for Background {
    fn render(&mut self, ctx: &Context, gl: &mut GlGraphics) {
        use graphics::*;
        
        for i in -1..self.repeat + 1 {
            let transform = ctx.transform.trans(self.x * config::BACKGROUND_PARRALAX_FACTOR + (self.width * i as f64), self.y);
            image(&self.background_texture, transform, gl);    
        }

        for i in -1..self.repeat + 1 {
            let transform = ctx.transform.trans(self.x + (self.width * i as f64), self.y);
            image(&self.foreground_texture, transform, gl);    
        }
    }
}

impl CameraDependentObject for Background {
    fn move_object(&mut self, x: f64, y: f64){
        self.x += x;
        self.y += y;
    }
}