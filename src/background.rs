use opengl_graphics::{Texture, GlGraphics};
use graphics::Context;
use super::renderable::Renderable;
use super::camera::camera_dependent_object;

pub struct Background {
    background_texture: Texture,
    x: f64,
    y: f64
}

impl Background {
    pub fn new(texture: Texture) -> Background {
        Background {
            background_texture: texture,
            x: 0.0,
            y: 0.0
        }
    }
}

impl Renderable for Background {
    fn render(&mut self, ctx: &Context, gl: &mut GlGraphics) {
        use graphics::*;
        
        let transform = ctx.transform.trans(self.x, self.y);

        image(&self.background_texture, transform, gl);
    }
}

impl camera_dependent_object for Background {
    fn move_object(&mut self, x: f64, y: f64){
        self.x += x;
        self.y += y;
    }
}