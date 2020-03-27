use opengl_graphics::{Texture, GlGraphics};
use graphics::Context;
use graphics::math::*;

pub struct Animator {
    sprites: Vec<Texture>,
    interval: f64,
    current_sprite: usize,
    time_since_last: f64
}

impl Animator {
    pub fn new(sprites: Vec<Texture>, interval: f64) -> Animator {
        Animator {
            sprites: sprites,
            interval: interval,
            current_sprite: 0,
            time_since_last: 0.0
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

        let mut transform = ctx.transform.trans(position[0] - 10.0, position[1] - 27.0).scale(0.1, 0.1);
        if mirror {
            transform = transform.flip_h();
        }
        let sprite = &self.sprites[self.current_sprite];
        let size = sprite.get_size();
        image(sprite, transform, gl);
    }
}