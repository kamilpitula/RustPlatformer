use graphics::math::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use interpolation::Lerp;

use super::AABB::AABB;
use super::config;
use super::map::{Map, AreaIndex};

pub struct MovingObject {
    pub object_id: String,
    pub old_position: Vec2d,
    pub position: Vec2d,
    pub old_speed: Vec2d,
    pub speed: Vec2d,
    pub old_accelaration: Vec2d,
    pub acceleration: Vec2d,
    pub scale: Vec2d,

    pub aabb: AABB,
    pub aabb_offset: Vec2d,

    pub pushed_right_wall: bool,
    pub pushes_right_wall: bool,

    pub pushed_left_wall: bool,
    pub pushes_left_wall: bool,

    pub was_on_ground: bool,
    pub on_ground: bool,

    pub was_at_ceiling: bool,
    pub at_ceiling: bool,

    pub on_one_way_platform: bool,
    pub areas: Vec<AreaIndex>,
    pub all_colliding_objects: HashMap<String, CollisionData>,

    bounds: Vec2d,
    accelerate: f64,
    max_speed: f64,
    jump_speed: f64,
    one_way_platform_tsh: f64
}

impl MovingObject {
    pub fn new(position: Vec2d, size: Vec2d, bounds: Vec2d, accelerate: f64, max_speed: f64, jump_speed: f64, object_id: String) -> MovingObject {
        MovingObject {
            object_id: object_id,
            position: position,
            old_position: [0.0, 0.0],
            acceleration: [0.0, config::GRAVITY],
            old_accelaration: [0.0, 0.0],
            speed: [0.0, 0.0],
            old_speed: [0.0, 0.0],
            scale: [1.0, 1.0],
            pushed_right_wall: false,
            pushes_right_wall: false,
            pushed_left_wall: false,
            pushes_left_wall: false,
            was_on_ground: true,
            on_ground: true,
            was_at_ceiling: false,
            at_ceiling: false,
            on_one_way_platform: false,
            aabb: AABB::new(position, mul_scalar(size, 0.5)),
            aabb_offset: mul_scalar(size, 0.5),
            bounds: bounds,
            accelerate: accelerate,
            max_speed: max_speed,
            jump_speed: jump_speed,
            one_way_platform_tsh: 15.0,
            areas: Vec::new(),
            all_colliding_objects: HashMap::new()
        }
    }

    fn handle_right_side_collision(&mut self, map: &Map) {
        let (collides, wall_x) = self.collides_right_side(&map);
        
        if collides && self.speed[0] > 0.0 {
            self.position[0] = wall_x - self.aabb.half_size[0] * 2.0;
            self.speed[0] = 0.0;
            self.pushes_right_wall = true;
            return;
        }

        self.pushes_right_wall = false;
    }

    fn handle_left_side_collision(&mut self, map: &Map) {
        let (collides, wall_x) = self.collides_left_side(&map);
        
        if collides && self.speed[0] < 0.0 {
            self.position[0] = wall_x;
            self.speed[0] = 0.0;
            self.pushes_left_wall = true;
            return;
        } 

        self.pushes_left_wall = false;
    }

    fn check_ground_collision(&mut self, map: &Map) {
        let height = map.tiles[0].len() as f64 * map.tile_size;
        if self.position[1] >= height {
            self.position[1] = height;
            self.speed[1] = 0.0;
            self.on_ground = true;
            return
        }
        let (has_ground, calculated_ground) = self.has_ground(&map);

        if self.speed[1] > 0.0 && has_ground {
            self.position[1] = calculated_ground - self.aabb.half_size[1] * 2.0;
            self.speed[1] = 0.0;
            self.on_ground = true;
        } else {
            self.on_ground = false;
        }
    }

    fn check_ceiling_collision(&mut self, map: &Map) {
        let (has_ceiling, calculated_ceiling) = self.has_ceiling(&map);

        if self.speed[1] < 0.0 && has_ceiling {
            self.position[1] = calculated_ceiling;//  - self.aabb_offset[1];
            self.speed[1] = 0.0;
        }
    }

    fn limit_walk_speed(&mut self, calculated_speed: Vec2d) {
        if calculated_speed[0] > self.max_speed {
            self.speed = [self.max_speed, calculated_speed[1]];
        } else if calculated_speed[0] < -self.max_speed {
            self.speed = [-self.max_speed, calculated_speed[1]];
        } else {
            self.speed = calculated_speed;
        }
    }

    pub fn stop(&mut self) {
        self.speed = [0.0, 0.0];
    }

    pub fn jump(&mut self) {
        self.speed = add(self.speed, [0.0, -self.jump_speed]);
    }

    pub fn move_left(&mut self, factor: f64) {
        self.acceleration = [-self.accelerate * factor, self.acceleration[1]];
    }

    pub fn move_right(&mut self, factor: f64) {
        self.acceleration = [self.accelerate * factor, self.acceleration[1]];
    }

    pub fn drop(&mut self) {
        if self.on_one_way_platform{
            self.position = [self.position[0], self.position[1] + self.one_way_platform_tsh];
            self.on_one_way_platform = false;
        }
    }

    pub fn falling(&mut self) {
        if self.speed[1] >= 0.0 {
            self.acceleration[1] = config::GRAVITY * 7.5;
        }
    }

    pub fn stop_falling(&mut self) {
        if self.speed[1] >= 0.0 {
            self.acceleration[1] = config::GRAVITY;
        }
    }

    pub fn move_object(&mut self, x: f64, y: f64){
        self.position[0] += x * config::MAP_TILES_PARRALAX_FACTOR;
        self.position[1] += y * config::MAP_TILES_PARRALAX_FACTOR;
    }

    pub fn update_physics(&mut self, delta: f64, map: &Map) {
        self.old_position = self.position;
        self.old_speed = self.speed;
 
        let calculated_speed = add(self.speed, mul_scalar(self.acceleration, delta));
        self.limit_walk_speed(calculated_speed);
        self.acceleration[0] = self.speed[0] * -config::FRICTION;
        
        self.was_on_ground = self.on_ground;
        self.pushed_right_wall = self.pushes_right_wall;
        self.pushed_left_wall = self.pushes_left_wall;
        self.was_at_ceiling = self.at_ceiling;

        self.aabb.center = add(self.position, self.aabb_offset);
        self.position = add(self.position, mul_scalar(self.speed, delta));

        self.handle_left_side_collision(&map);
        self.handle_right_side_collision(&map);
        self.check_ground_collision(&map);
        self.check_ceiling_collision(&map);
    }

    pub fn has_ground(&mut self, map: &Map) -> (bool, f64) {
        let (new_bottom_right, new_bottom_left, _, _) = self.get_sensors(self.position);
        let (old_bottom_right, old_bottom_left, _, _) = self.get_sensors(self.old_position);

        let end_y = map.get_map_tile_y_at_point(new_bottom_left[1]);
        let beg_y = (map.get_map_tile_y_at_point(old_bottom_left[1]) + 1).min(end_y);
        let dist = (beg_y - end_y).abs().max(1);

        for tileIndexY in beg_y..end_y + 1 {
            let bottom_right = self.round_vector(old_bottom_right.lerp(&new_bottom_right, &((end_y - tileIndexY).abs() as f64 / dist as f64)));
            let bottom_left = self.round_vector([bottom_right[0] - self.aabb.half_size[0] * 2.0 - 2.0, bottom_right[1]]);
            
            let mut checked_tile = add(bottom_left, [1.0, 0.0]);

            while checked_tile[0] < bottom_right[0] {

                checked_tile[0] = checked_tile[0].min(new_bottom_right[0]);

                let tile_index_x = map.get_map_tile_x_at_point(checked_tile[0]);
                checked_tile[0] = checked_tile[0] + map.tile_size;

                let ground_y = tileIndexY as f64 * map.tile_size + map.position[1];
                if map.is_obstacle(tile_index_x, tileIndexY){
                    self.on_one_way_platform = false;
                    return (true, ground_y);
                }
                if map.is_one_way_platform(tile_index_x, tileIndexY)
                    && (checked_tile[1] - ground_y).abs() < self.one_way_platform_tsh
                {
                    self.on_one_way_platform = true;
                    return (true, ground_y);
                }
            }
        }
        (false, 0.0)
    }

    pub fn has_ceiling(&mut self, map: &Map) -> (bool, f64) {
        let (_, _, new_top_right, _new_top_left) = self.get_sensors(self.position);
        let (_, _, old_top_right, _old_top_left) = self.get_sensors(self.old_position);

        let end_y = map.get_map_tile_y_at_point(new_top_right[1]);
        let beg_y = (map.get_map_tile_y_at_point(old_top_right[1]) + 1).min(end_y);
        let dist = (beg_y - end_y).abs().max(1);

        for tileIndexY in beg_y..end_y + 1 {
            let top_right = self.round_vector(old_top_right.lerp(&new_top_right, &((end_y - tileIndexY).abs() as f64 / dist as f64)));
            let top_left = self.round_vector([top_right[0] - self.aabb.half_size[0] * 2.0 - 1.0, top_right[1]]);
            
            let mut checked_tile = add(top_left, [1.0, 0.0]);

            while checked_tile[0] < top_right[0] {

                checked_tile[0] = checked_tile[0].min(new_top_right[0]);

                let tile_index_x = map.get_map_tile_x_at_point(checked_tile[0]);
                checked_tile[0] = checked_tile[0] + map.tile_size;

                if map.is_obstacle(tile_index_x, tileIndexY){
                    let ceiling_y = tileIndexY as f64 * map.tile_size + map.tile_size + map.position[1];
                    return (true, ceiling_y);
                }
            }
        }
        (false, 0.0)
    }

    pub fn collides_left_side(&mut self, map: &Map) -> (bool, f64) {
        let (_, new_bottom_left, _, _new_top_left) = self.get_sensors(self.position);
        let (_, old_bottom_left, _, _old_top_left) = self.get_sensors(self.old_position);

        let end_x = map.get_map_tile_x_at_point(new_bottom_left[0]);
        let beg_x = (map.get_map_tile_x_at_point(old_bottom_left[0])).min(end_x);
        let dist = (end_x - beg_x).abs().max(1);

        for tileIndexX in (end_x..beg_x + 1).rev() {
            let bottom_left =
                self.round_vector(old_bottom_left.lerp(&new_bottom_left, &((end_x - tileIndexX).abs() as f64 / dist as f64)));
            let top_left = self.round_vector([bottom_left[0], bottom_left[1] - self.aabb.half_size[1] * 2.0 - 2.0]);
            
            let mut checked_tile = top_left;

            while checked_tile[1] < bottom_left[1] {

                let mut y = checked_tile[1];

                if checked_tile[1] > bottom_left[1] {
                   y = bottom_left[1]
                }

                let tile_index_y = map.get_map_tile_y_at_point(y) - 1;
                checked_tile[1] = checked_tile[1] + map.tile_size;

                if map.is_obstacle(tileIndexX, tile_index_y){
                    let wall_x = tileIndexX as f64 * map.tile_size + map.tile_size + map.position[0];
                    return (true, wall_x);
                }
            }
        }
        (false, 0.0)
    }

    pub fn collides_right_side(&mut self, map: &Map) -> (bool, f64) {
        let (new_bottom_right, _, _new_top_right, _) = self.get_sensors(self.position);
        let (old_bottom_right, _, _old_top_right, _) = self.get_sensors(self.old_position);

        let end_x = map.get_map_tile_x_at_point(new_bottom_right[0]);
        let beg_x = (map.get_map_tile_x_at_point(old_bottom_right[0])).min(end_x);
        let dist = (end_x - beg_x).abs().max(1);

        for tileIndexX in beg_x..end_x + 1 {
            let bottom_right =
                self.round_vector(old_bottom_right.lerp(&new_bottom_right, &((end_x - tileIndexX).abs() as f64 / dist as f64)));
            let top_right =
                self.round_vector([bottom_right[0], bottom_right[1] - self.aabb.half_size[1] * 2.0 - 2.0]);
            
            let mut checked_tile = top_right;

            while checked_tile[1] < bottom_right[1] {

                let mut y = checked_tile[1];

                if checked_tile[1] > bottom_right[1] {
                   y = bottom_right[1]
                }

                let tile_index_y = map.get_map_tile_y_at_point(y) - 1;
                checked_tile[1] = checked_tile[1] + map.tile_size;

                if map.is_obstacle(tileIndexX, tile_index_y){
                    let wall_x = tileIndexX as f64 * map.tile_size + map.position[0];
                    return (true, wall_x);
                }
            }
        }
        (false, 0.0)
    }

    fn get_sensors(&self, position: Vec2d) -> (Vec2d, Vec2d, Vec2d, Vec2d) {
        let center = add(position, self.aabb.half_size);
        let bottom_right = add(add(center, self.aabb.half_size), [1.0, 1.0]);
        let top_right = [center[0] + self.aabb.half_size[0] + 1.0, center[1] - self.aabb.half_size[1] - 1.0];
        let bottom_left = [bottom_right[0] - self.aabb.half_size[0] * 2.0 - 2.0, bottom_right[1]];
        let top_left = [top_right[0] - self.aabb.half_size[0] * 2.0 - 2.0, top_right[1]];
        (self.round_vector(bottom_right),
         self.round_vector(bottom_left),
         self.round_vector(top_right),
         self.round_vector(top_left))
    }

    fn round_vector(&self, vector: Vec2d) -> Vec2d {
        [vector[0].round(), vector[1].round()]
    }
}

pub struct CollisionData {
    pub other: Rc<RefCell<MovingObject>>,
    pub overlap: Vec2d,
    pub speed1: Vec2d,
    pub speed2: Vec2d,
    pub old_pos1: Vec2d,
    pub old_pos2: Vec2d,
    pub pos1: Vec2d,
    pub pos2: Vec2d
}