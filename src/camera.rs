use super::character::Character;
use super::background::Background;
use super::map::Map;
use super::moving_object::Moving_Object;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

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

    pub fn update(&mut self, objects: &mut HashMap<String, Rc<RefCell<Moving_Object>>>, map: &mut Map, character: &mut Character, background: &mut Background, delta: f64) {
        let mut characterObject = objects["character"].borrow_mut();
        
        if characterObject.position[0] <= self.min {
            if background.x >= 0.0 {
                return;
            }
            characterObject.position[0] = self.min;
            let move_x = delta * characterObject.speed[0];

            if background.x - move_x >= 0.0 {
                background.x = 0.0;
                return;
            }

            background.move_object(-move_x, 0.0);
            map.move_object(-move_x, 0.0);
            
            for (k, v) in objects.iter() {
                if k == "character" {
                    continue;
                }
                v.borrow_mut().move_object(-move_x, 0.0);
            }
        }

        if characterObject.position[0] >= self.max {
            if background.x <= -(background.combined_width / 2.0) {
                return;
            }
            characterObject.position[0] = self.max;
            let move_x = delta * characterObject.speed[0];

            background.move_object(-move_x, 0.0);
            map.move_object(-move_x, 0.0);

            for (k, v) in objects.iter() {
                if k == "character" {
                    continue;
                }
                v.borrow_mut().move_object(-move_x, 0.0);
            }
        }
    }
}

pub trait camera_dependent_object {
    fn move_object(&mut self, x: f64, y: f64);
}
