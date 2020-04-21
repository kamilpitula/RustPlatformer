use opengl_graphics::{Texture, GlGraphics};
use graphics::Context;
use graphics::math::*;

pub struct Animator {
    sprites: Vec<Texture>,
    interval: f64,
    current_sprite: usize,
    time_since_last: f64,
    scale_x: f64,
    scale_y: f64, 
    sprite_half_size_x: f64,
    sprite_size_x: f64,
    box_size: Vec2d
}

impl Animator {
    pub fn new(sprites: Vec<Texture>, interval: f64, box_size: Vec2d) -> Animator {
        use graphics::*;
        
        let sprite_size = sprites[0].get_size();

        let scale_x = box_size[0] / sprite_size.0 as f64;
        let scale_y = box_size[1] / sprite_size.1 as f64;

        let sprite_half_size_x = (sprite_size.0 / 2) as f64  * scale_x;
        let sprite_size_x = sprite_half_size_x * 2.0;

        Animator {
            sprites,
            interval,
            current_sprite: 0,
            time_since_last: 0.0,
            scale_x,
            scale_y,
            sprite_half_size_x,
            sprite_size_x,
            box_size
        }
    }

    pub fn next(&mut self, delta: f64) {
        self.time_since_last += delta;
        if self.time_since_last >= self.interval {
            self.time_since_last = 0.0;
            if self.current_sprite == self.sprites.len() - 1{
                self.current_sprite = 0;
            }
            else {
                self.current_sprite += 1;
            }
        }
    }

    pub fn render(&mut self, ctx: &Context, gl: &mut GlGraphics, position: Vec2d, mirror: bool){
        use graphics::*;

        let sprite = &self.sprites[self.current_sprite];
        
        let position_y = position[1];
        let position_x = position[0] - self.sprite_half_size_x + (self.box_size[0] / 2.0);

        let mut transform = ctx.transform
            .trans(position_x, position_y)
            .scale(self.scale_x, self.scale_y);
        
        if mirror {
            let mut flipped_transform = ctx.transform
                .trans(position_x + self.sprite_size_x, position_y)
                .scale(self.scale_x, self.scale_y);
            
            flipped_transform = flipped_transform.flip_h();
            image(sprite, flipped_transform, gl);
            return;
        }
        
        image(sprite, transform, gl);
    }
}