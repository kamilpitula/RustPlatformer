use graphics::math::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{RefCell, RefMut};

use super::moving_object::{Moving_Object, CollisionData};
use super::map::{Map, AreaIndex};

pub struct Collider {
    gridAreaWidth: i8,
    gridAreaHeight: i8,
    horizontalAreaCount: i8,
    verticalAreaCount: i8,
    overlappingAreas: Vec<AreaIndex>
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
            gridAreaHeight,
            gridAreaWidth,
            horizontalAreaCount,
            verticalAreaCount,
            overlappingAreas: Vec::new()
        }
    }

    pub fn update_areas(&mut self,
                        object_ptr: Rc<RefCell<Moving_Object>>,
                        map: &Map,
                        objectsInArea: &mut HashMap<AreaIndex, HashMap<String, Rc<RefCell<Moving_Object>>>>
    ) {
        let mut object = object_ptr.borrow_mut();

        let (topLeft, topRight, bottomRight, bottomLeft) = self.get_areas(map, &mut object);
        self.fill_overlapping_areas(topLeft, topRight, bottomRight, bottomLeft);

        self.remove_object(objectsInArea, &mut object);
        self.add_object(&object_ptr, objectsInArea, object);

        self.overlappingAreas.clear();
    }

    pub fn check_collisions(&self, objectsInArea: &mut HashMap<AreaIndex, HashMap<String, Rc<RefCell<Moving_Object>>>>) {
        for y in 0..self.verticalAreaCount {
            for x in 0..self.horizontalAreaCount {
                if let Some(objectsMap) = objectsInArea.get(&AreaIndex { x, y }) {
                    let objects: Vec<Rc<RefCell<Moving_Object>>> = objectsMap
                        .iter()
                        .map(|(k, v)| Rc::clone(v))
                        .collect();

                    if objects.len() == 0 {
                        continue;
                    }

                    self.check_collisions_in_area(&objects);
                }
                
            }
        }
    }

    fn add_object(&mut self,
                  object_ptr: &Rc<RefCell<Moving_Object>>,
                  objectsInArea: &mut HashMap<AreaIndex, HashMap<String, Rc<RefCell<Moving_Object>>>>,
                  mut object: RefMut<Moving_Object>
    ) {
        for i in 0..self.overlappingAreas.len() {
            if !object.areas.contains(&self.overlappingAreas[i]) {
                if !objectsInArea.contains_key(&self.overlappingAreas[i]) {
                    let mut map: HashMap<String, Rc<RefCell<Moving_Object>>> = HashMap::new();
                    map.insert(object.object_id.clone(), Rc::clone(&object_ptr));
                    objectsInArea.insert(self.overlappingAreas[i].clone(), map);
                } else {
                    objectsInArea
                        .get_mut(&self.overlappingAreas[i])
                        .unwrap()
                        .insert(object.object_id.clone(), Rc::clone(&object_ptr));
                }
                object.areas.push(self.overlappingAreas[i].clone())
            }
        }
    }

    fn remove_object(&mut self, objectsInArea: &mut HashMap<AreaIndex, HashMap<String, Rc<RefCell<Moving_Object>>>>, object: &mut RefMut<Moving_Object>) {
        let mut existing: Vec<AreaIndex> = Vec::new();
        for area in object.areas.iter() {
            if !self.overlappingAreas.contains(&area) {
                if objectsInArea.contains_key(&area) {
                    let mut objs = objectsInArea
                        .get_mut(&area)
                        .unwrap();

                    objs.remove(&object.object_id);
                }
            } else {
                existing.push(area.clone());
            }
        }
        object.areas = existing;
    }

    fn get_areas(&mut self, map: &Map, mut object: &mut RefMut<Moving_Object>) -> (AreaIndex, AreaIndex, AreaIndex, AreaIndex) {
        let mut topLeft = map.get_map_tile_in_point(sub(object.aabb.center, object.aabb.half_size));
        let mut topRight = map.get_map_tile_in_point(
            [object.aabb.center[0] + object.aabb.half_size[0],
                object.aabb.center[1] - object.aabb.half_size[1]]);
        let mut bottomRight = map.get_map_tile_in_point(add(object.aabb.center, object.aabb.half_size));

        topLeft.x /= self.gridAreaWidth;
        topLeft.y /= self.gridAreaHeight;
        topRight.x /= self.gridAreaWidth;
        topRight.y /= self.gridAreaHeight;
        bottomRight.x /= self.gridAreaWidth;
        bottomRight.y /= self.gridAreaHeight;
        let mut bottomLeft = AreaIndex {
            x: topLeft.x,
            y: bottomRight.y
        };

        (topLeft, topRight, bottomRight, bottomLeft)
    }

    fn fill_overlapping_areas(&mut self, mut topLeft: AreaIndex, mut topRight: AreaIndex, mut bottomRight: AreaIndex, mut bottomLeft: AreaIndex) {
        if topLeft.x == topRight.x && topLeft.y == bottomLeft.y {
            self.overlappingAreas.push(topLeft);
        } else if topLeft.x == topRight.x {
            self.overlappingAreas.push(topLeft);
            self.overlappingAreas.push(bottomLeft);
        } else if topLeft.y == bottomLeft.y {
            self.overlappingAreas.push(topLeft);
            self.overlappingAreas.push(topRight);
        } else {
            self.overlappingAreas.push(topLeft);
            self.overlappingAreas.push(bottomLeft);
            self.overlappingAreas.push(topRight);
            self.overlappingAreas.push(bottomRight);
        }
    }

    fn check_collisions_in_area(&self, objects: &Vec<Rc<RefCell<Moving_Object>>>) {
        for i in 0..objects.len() - 1 {
            let mut obj1 = objects[i].borrow_mut();
            for j in (i + 1)..objects.len() {
                let mut obj2 = objects[j].borrow_mut();
                let (collides, overlaps) = obj1.aabb.overlaps_signed(&obj2.aabb);
                if collides && !obj1.allCollidingObjects.contains_key(&obj2.object_id) {

                    let obj1_data = CollisionData {
                        other: Rc::clone(&objects[j]),
                        overlap: overlaps,
                        speed1: obj1.speed,
                        speed2: obj2.speed,
                        oldPos1: obj1.old_position,
                        oldPos2: obj2.old_position,
                        pos1: obj1.position,
                        pos2: obj2.position
                    };
                    obj1.allCollidingObjects.insert(obj2.object_id.clone(), obj1_data);

                    let obj2_data = CollisionData {
                        other: Rc::clone(&objects[i]),
                        overlap: overlaps,
                        speed1: obj2.speed,
                        speed2: obj1.speed,
                        oldPos1: obj2.old_position,
                        oldPos2: obj1.old_position,
                        pos1: obj2.position,
                        pos2: obj1.position
                    };
                    obj2.allCollidingObjects.insert(obj1.object_id.clone(), obj2_data);
                }
            }
        }
    }
}