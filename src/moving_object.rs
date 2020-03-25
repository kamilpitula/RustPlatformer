use graphics::math::*;
use super::AABB::AABB;
use super::config;

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

    bounds: Vec2d
}

impl Moving_Object {
    pub fn new(position: Vec2d, size: Vec2d, bounds: Vec2d) -> Moving_Object {
        Moving_Object{
            position: position,
            old_position: [0.0, 0.0],
            acceleration: [0.0, 0.0],
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
            aabb: AABB::new(position, mul_scalar(size, 2.0)),
            aabb_offset: mul_scalar(size, 2.0),
            bounds: bounds
        }
    }

    fn check_right_wall_collision(&mut self) {
        if self.position[0] > self.bounds[1] {
            self.position[0] = self.bounds[1];
            self.pushes_right_wall = true;
        } else {
            self.pushes_right_wall = false;
        }
    }

    fn check_left_wall_collision(&mut self) {
        if self.position[0] < self.bounds[0] {
            self.position[0] = self.bounds[0];
            self.pushes_left_wall = true;
        } else {
            self.pushes_left_wall = false;
        }
    }

    fn check_ground_collision(&mut self) {
        if self.position[1] > 700.0 {
            self.position[1] = 700.0;
            self.on_ground = true;
        } else {
            self.on_ground = false;
        }
    }

    fn limit_walk_speed(&mut self, calculated_speed: Vec2d) {
        if calculated_speed[0] > config::WALK_SPEED {
            self.speed = [config::WALK_SPEED, self.speed[1]];
        } else if calculated_speed[0] < -config::WALK_SPEED {
            self.speed = [-config::WALK_SPEED, self.speed[1]];
        } else {
            self.speed = calculated_speed;
        }
    }

    pub fn update_physics(&mut self, delta: f64) {

        self.old_position = self.position;
        self.old_speed = self.speed;

        let calculated_speed = add(self.speed, mul_scalar(self.acceleration, delta));
        self.limit_walk_speed(calculated_speed);
        self.acceleration[0] = self.speed[0] * -config::FRICTION;
        
        self.was_on_ground = self.on_ground;
        self.pushed_right_wall = self.pushes_right_wall;
        self.pushed_left_wall = self.pushes_left_wall;
        self.was_at_ceiling = self.at_ceiling;

        self.position = add(self.position, mul_scalar(self.speed, delta));

        self.check_left_wall_collision();
        self.check_right_wall_collision();
        self.check_ground_collision();
        
        self.aabb.center = add(self.position, self.aabb_offset);
    }

    
}