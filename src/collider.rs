use graphics::math::*;

use super::moving_object::Moving_Object;
use super::map::Map;

pub struct Collider {
    gridAreaWidth: i8,
    gridAreaHeight: i8,
    objectsInArea: Vec<Vec<Moving_Object>>,
    horizontalAreaCount: i8,
    verticalAreaCount: i8,
    overlappingAreas: Vec<(i8, i8)>
}

impl Collider {
    pub fn new(gridAreaWidth: i8, gridAreaHeight: i8, levelWidth: i8, levelHeight: i8) -> Collider {
        if levelWidth % gridAreaWidth != 0 {
            panic!("wrong level width");
        }

        if levelHeight % gridAreaHeight != 0 {
            panic!("wrong level height");
        }

        let horizontalAreaCount = levelWidth / gridAreaWidth;
        let verticalAreaCount = levelHeight / gridAreaHeight;

        Collider {
            gridAreaHeight: gridAreaHeight,
            gridAreaWidth: gridAreaWidth,
            objectsInArea: Vec::with_capacity((horizontalAreaCount * verticalAreaCount) as usize),
            horizontalAreaCount: horizontalAreaCount,
            verticalAreaCount: verticalAreaCount,
            overlappingAreas: Vec::new()
        }
    }

    pub fn update_areas(&mut self, object: &mut Moving_Object, map: &Map) {

        let mut topLeft = map.get_map_tile_in_point(sub(object.aabb.center, object.aabb.half_size)); 
        let mut topRight = map.get_map_tile_in_point(
             [object.aabb.center[0] + object.aabb.half_size[0],
              object.aabb.center[1] - object.aabb.half_size[1]]);
        let mut bottomRight = map.get_map_tile_in_point(add(object.aabb.center, object.aabb.half_size));
        let mut bottomLeft = (0,0);

        topLeft.0 /= self.gridAreaWidth;
        topLeft.1 /= self.gridAreaHeight;

        topRight.0 /= self.gridAreaWidth;
        topRight.1 /= self.gridAreaHeight;

        bottomRight.0 /= self.gridAreaWidth;
        bottomRight.1 /= self.gridAreaHeight;

        bottomLeft.0 = topLeft.0;
        bottomLeft.1 = bottomRight.1;

        if topLeft.0 == topRight.0 && topLeft.1 == bottomLeft.1 {
            self.overlappingAreas.push(topLeft);
        } else if topLeft.0 == topRight.0 {
            self.overlappingAreas.push(topLeft);
            self.overlappingAreas.push(bottomLeft);
        } else if topLeft.1 == bottomLeft.1 {
            self.overlappingAreas.push(topLeft);
            self.overlappingAreas.push(topRight);
        } else {
            self.overlappingAreas.push(topLeft);
            self.overlappingAreas.push(bottomLeft);
            self.overlappingAreas.push(topRight);
            self.overlappingAreas.push(bottomRight);
        }
        //TODO: moving object
        self.overlappingAreas.clear();
    }
}