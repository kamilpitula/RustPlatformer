use graphics::math::*;
use super::AABB::AABB;
use super::config;
use super::Map;
use std::rc::Rc;
use interpolation::Lerp;

pub struct Moving_Object{
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

    bounds: Vec2d,
    accelerate: f64,
    max_speed: f64,
    jump_speed: f64,
    one_way_platform_tsh: f64
}

impl Moving_Object {
    pub fn new(position: Vec2d, size: Vec2d, bounds: Vec2d, accelerate: f64, max_speed: f64, jump_speed: f64) -> Moving_Object {
        Moving_Object{
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
            one_way_platform_tsh: 10.0
        }
    }

    fn handle_right_side_collision(&mut self, map: &Map) {
        let (collides, wallX) = self.collides_right_side(&map);
        let mut x = self.bounds[1];
        if collides {
            x = self.bounds[1].min(wallX);
        }

        if self.position[0] + self.aabb.half_size[0] * 2.0 > x {
            self.position[0] = x - self.aabb.half_size[0];
            self.speed[0] = 0.0;
            self.pushes_right_wall = true;
            return;
        }
        
        self.pushes_right_wall = false;
    }

    fn handle_left_side_collision(&mut self, map: &Map) {
        let (collides, wallX) = self.collides_left_side(&map);
        let x = self.bounds[0].max(wallX);

        if self.position[0] < x {
            self.position[0] = x;
            self.speed[0] = 0.0;
            self.pushes_left_wall = true;
            return;
        } 

        self.pushes_left_wall = false;
    }

    fn check_ground_collision(&mut self, map: &Map) {
        let (has_ground, calculated_ground) = self.has_ground(&map);

        if self.speed[1] >= 0.0 && has_ground {
            self.position[1] = calculated_ground - self.aabb.half_size[1] * 2.0;//  - self.aabb_offset[1];
            self.speed[1] = 0.0;
            self.on_ground = true;
        } else {
            self.on_ground = false;
        }
    }

    fn check_ceiling_collision(&mut self, map: &Map) {
        let (has_ceiling, calculated_ceiling) = self.has_ceiling(&map);

        if self.speed[1] <= 0.0 && has_ceiling {
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
        
        self.check_ground_collision(&map);
        self.check_ceiling_collision(&map);

        self.handle_left_side_collision(&map);
        self.handle_right_side_collision(&map);
        
        self.aabb.center = add(self.position, self.aabb_offset);
        self.position = add(self.position, mul_scalar(self.speed, delta));
    }

    pub fn has_ground(&mut self, map: &Map) -> (bool, f64) {
        let (new_bottomRight, new_bottomLeft, _, _) = self.get_sensors(self.position);
        let (old_bottomRight, old_bottomLeft, _, _) = self.get_sensors(self.old_position);

        let endY = map.get_map_tileY_at_point(new_bottomLeft[1]);
        let begY = (map.get_map_tileY_at_point(old_bottomLeft[1]) + 1).min(endY);
        let dist = (begY - endY).abs().max(1);

        for tileIndexY in begY..endY + 1 {
            let bottomRight = old_bottomRight.lerp(&new_bottomRight, &((endY - tileIndexY).abs() as f64 / dist as f64));
            let bottomLeft = [bottomRight[0] - self.aabb.half_size[0] * 2.0 - 2.0, bottomRight[1]];
            
            let mut checkedTile = bottomLeft;

            while checkedTile[0] < bottomRight[0] {

                checkedTile[0] = checkedTile[0].min(new_bottomRight[0]);

                let tileIndexX = map.get_map_tileX_at_point(checkedTile[0]);
                checkedTile[0] = checkedTile[0] + map.tileSize;

                let groundY = tileIndexY as f64 * map.tileSize + map.position[1];
                if map.is_obstacle(tileIndexX, tileIndexY){
                    self.on_one_way_platform = false;
                    return (true,  groundY);
                }
                if map.is_one_way_platform(tileIndexX, tileIndexY) 
                    && (checkedTile[1] - groundY).abs() < self.one_way_platform_tsh
                {
                    self.on_one_way_platform = true;
                    return (true, groundY);
                }
            }
        }
        (false, 0.0)
    }

    pub fn has_ceiling(&mut self, map: &Map) -> (bool, f64) {
        let (_, _, new_top_right, new_top_left) = self.get_sensors(self.position);
        let (_, _, old_top_right, old_topLeft) = self.get_sensors(self.old_position);

        let endY = map.get_map_tileY_at_point(new_top_right[1]);
        let begY = (map.get_map_tileY_at_point(old_top_right[1]) + 1).min(endY);
        let dist = (begY - endY).abs().max(1);

        for tileIndexY in begY..endY + 1 {
            let topRight = old_top_right.lerp(&new_top_right, &((endY - tileIndexY).abs() as f64 / dist as f64));
            let topLeft = [topRight[0] - self.aabb.half_size[0] * 2.0 - 2.0, topRight[1]];
            
            let mut checkedTile = topLeft;

            while checkedTile[0] < topRight[0] {

                checkedTile[0] = checkedTile[0].min(new_top_right[0]);

                let tileIndexX = map.get_map_tileX_at_point(checkedTile[0]);
                checkedTile[0] = checkedTile[0] + map.tileSize;

                if map.is_obstacle(tileIndexX, tileIndexY){
                    let ceilingY = tileIndexY as f64 * map.tileSize + map.tileSize + map.position[1];
                    return (true,  ceilingY);
                }
            }
        }
        (false, 0.0)
    }

    pub fn collides_left_side(&mut self, map: &Map) -> (bool, f64) {
        let (_, new_bottom_left, _, new_top_left) = self.get_sensors(self.position);
        let (_, old_bottom_left, _, old_topLeft) = self.get_sensors(self.old_position);

        let endX = map.get_map_tileX_at_point(new_bottom_left[0]);
        let begX = (map.get_map_tileX_at_point(old_bottom_left[0])).min(endX);
        let dist = (endX - begX).abs().max(1);

        for tileIndexX in ((endX - 1)..begX + 1).rev() {
            let bottomLeft = old_bottom_left.lerp(&new_bottom_left, &((endX - tileIndexX).abs() as f64 / dist as f64));
            let topLeft = [bottomLeft[0], bottomLeft[1] - self.aabb.half_size[1] * 2.0 - 2.0];
            
            let mut checkedTile = topLeft;

            while checkedTile[1] < bottomLeft[1] {

                let y = checkedTile[1].max(bottomLeft[1]);
                let tileIndexY = map.get_map_tileY_at_point(y) -1;
                checkedTile[1] = checkedTile[1] + map.tileSize;

                if map.is_obstacle(tileIndexX, tileIndexY){
                    let wallX = tileIndexX as f64 * map.tileSize + map.tileSize + map.position[0];
                    return (true,  wallX);
                }
            }
        }
        (false, 0.0)
    }

    pub fn collides_right_side(&mut self, map: &Map) -> (bool, f64) {
        let (new_bottom_right, _, new_top_right, _) = self.get_sensors(self.position);
        let (old_bottom_right, _, old_top_right, _) = self.get_sensors(self.old_position);

        let endX = map.get_map_tileX_at_point(new_bottom_right[0]);
        let begX = (map.get_map_tileX_at_point(old_bottom_right[0])).min(endX);
        let dist = (endX - begX).abs().max(1);

        for tileIndexX in begX..endX + 1 {
            let bottomRight = old_bottom_right.lerp(&new_bottom_right, &((endX - tileIndexX).abs() as f64 / dist as f64));
            let topRight = [bottomRight[0], bottomRight[1] - self.aabb.half_size[1] * 2.0 - 2.0];
            
            let mut checkedTile = bottomRight;

            while checkedTile[1] > topRight[1] {

                checkedTile[1] = checkedTile[1].min(topRight[1]);

                let tileIndexY = map.get_map_tileY_at_point(checkedTile[1]) + 1;
                checkedTile[1] = checkedTile[1] - map.tileSize;

                if map.is_obstacle(tileIndexX, tileIndexY){
                    let wallX = tileIndexX as f64 * map.tileSize - map.tileSize / 2.0 + map.position[1];
                    return (true,  wallX);
                }
            }
        }
        (false, 0.0)
    }

    fn get_sensors(&self, position: Vec2d) -> (Vec2d, Vec2d, Vec2d, Vec2d) {
        let center = add(position, self.aabb.half_size);
        let bottomRight = add(add(center, self.aabb.half_size), [1.0, 1.0]);
        let topRight = [center[0] + self.aabb.half_size[0] + 1.0, center[1] - self.aabb.half_size[1] - 1.0];
        let bottomLeft = [bottomRight[0] - self.aabb.half_size[0] * 2.0 - 2.0, bottomRight[1]];
        let topLeft = [topRight[0] - self.aabb.half_size[0] * 2.0 - 2.0, topRight[1]];
        (bottomRight, bottomLeft, topRight, topLeft)
    }
}