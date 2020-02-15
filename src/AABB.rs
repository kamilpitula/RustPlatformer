use graphics::math::Vec2d;

pub struct AABB {
    pub center: Vec2d,
    pub half_size: Vec2d
}

impl AABB {
    pub fn new(center: Vec2d, half_size: Vec2d) -> AABB {
       AABB {
           center: center,
           half_size: half_size
       } 
    }

    pub fn overlaps(&self, other: AABB) -> bool {

        if (self.center[0] - other.center[0]).abs() > self.half_size[0] + other.half_size[0] {
            return false;
        }

        if (self.center[1] - other.center[1]).abs() > self.half_size[1] + other.half_size[1] {
            return false;
        }

        return true;
    }
}