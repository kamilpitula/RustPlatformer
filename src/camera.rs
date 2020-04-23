use super::character::Character;
use super::background::Background;
use super::map::Map;
use super::moving_object::MovingObject;
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
            min,
            max
        }
    }

    pub fn update(&mut self, objects: &mut HashMap<String, MovingObject>, map: &mut Map, _character: &mut Character, background: &mut Background, delta: f64) {
        let character_object = &mut objects.get("1ad31e1d-494a-41fe-bb9c-e7b8b83e59f1").unwrap();
        let position_x = character_object.position[0];
        
        if position_x <= self.min {
            if background.x >= 0.0 {
                return;
            }
            let mut character = &mut objects.get_mut("1ad31e1d-494a-41fe-bb9c-e7b8b83e59f1").unwrap();

            character.position[0] = self.min;
            let move_x = delta * character.speed[0];

            if background.x - move_x >= 0.0 {
                background.x = 0.0;
                return;
            }

            background.move_object(-move_x, 0.0);
            map.move_object(-move_x, 0.0);
            
            for (k, v) in objects.iter_mut() {
                if k == "1ad31e1d-494a-41fe-bb9c-e7b8b83e59f1" {
                    continue;
                }
                v.move_object(-move_x, 0.0);
            }
        }

        if position_x >= self.max {
            if background.x <= -(background.combined_width / 2.0) {
                return;
            }
            let mut character = &mut objects.get_mut("1ad31e1d-494a-41fe-bb9c-e7b8b83e59f1").unwrap();

            character.position[0] = self.max;
            let move_x = delta * character.speed[0];

            background.move_object(-move_x, 0.0);
            map.move_object(-move_x, 0.0);

            for (k, v) in objects.iter_mut() {
                if k == "1ad31e1d-494a-41fe-bb9c-e7b8b83e59f1" {
                    continue;
                }
                v.move_object(-move_x, 0.0);
            }
        }
    }
}

pub trait CameraDependentObject {
    fn move_object(&mut self, x: f64, y: f64);
}
