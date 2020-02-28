use graphics::math::*;
use super::AABB::AABB;

pub struct Moving_Object{
    pub old_position: Vec2d,
    pub position: Vec2d,
    pub old_speed: Vec2d,
    pub speed: Vec2d,
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
    pub at_ceiling: bool
}

impl Moving_Object {
    pub fn new(position: Vec2d, size: Vec2d) -> Moving_Object {
        Moving_Object{
            position: position,
            old_position: [0.0, 0.0],
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
            aabb_offset: mul_scalar(size, 2.0)
        }
    }

    pub fn update_physics(&mut self, delta: f64) {

        self.old_position = self.position;
        self.old_speed = self.speed;
        self.was_on_ground = self.on_ground;
        self.pushed_right_wall = self.pushes_right_wall;
        self.pushed_left_wall = self.pushes_left_wall;
        self.was_at_ceiling = self.at_ceiling;

        self.position = add(self.position, mul_scalar(self.speed, delta));

        if self.position[1] < 0.0 {
            self.position[1] = 0.0;
            self.on_ground = true;
        } else {
            self.on_ground = false;
        }
        
        self.aabb.center = add(self.position, self.aabb_offset);
    }
}