use super::character::Character;
use super::background::Background;

pub struct Camera {
    min: f64,
    max: f64
}

impl Camera{
    pub fn new(min: f64, max: f64) -> Camera {
        Camera {
            min: min,
            max: max
        }
    }

    pub fn update(&mut self, objects: &mut Vec<Box<camera_dependent_object>>, character: &mut Character, background: &mut Background, delta: f64) {
        if character.moving_object.position[0] <= self.min {
            character.moving_object.position[0] = self.min;
            let move_x = delta * character.moving_object.speed[0];
            background.move_object(-move_x, 0.0);
        }

        if character.moving_object.position[0] >= self.max {
            character.moving_object.position[0] = self.max;
            let move_x = delta * character.moving_object.speed[0];
            background.move_object(-move_x, 0.0);
        }
    }
}

pub trait camera_dependent_object {
    fn move_object(&mut self, x: f64, y: f64);
}
