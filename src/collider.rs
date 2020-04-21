use graphics::math::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{RefCell, RefMut};

use super::moving_object::{MovingObject, CollisionData};
use super::map::{Map, AreaIndex};

pub struct Collider {
    grid_area_width: i8,
    grid_area_height: i8,
    horizontal_area_count: i8,
    vertical_area_count: i8,
    overlapping_areas: Vec<AreaIndex>
}

impl Collider {
    pub fn new(grid_area_width: i8, grid_area_height: i8, level_width: i8, level_height: i8) -> Collider {
        if level_width % grid_area_width != 0 {
            panic!("wrong level width");
        }

        if level_height % grid_area_height != 0 {
            panic!("wrong level height");
        }

        let horizontal_area_count = level_width / grid_area_width;
        let vertical_area_count = level_height / grid_area_height;

        Collider {
            grid_area_height,
            grid_area_width,
            horizontal_area_count,
            vertical_area_count,
            overlapping_areas: Vec::new()
        }
    }

    pub fn update_areas(&mut self,
                        object_ptr: Rc<RefCell<MovingObject>>,
                        map: &Map,
                        objects_in_area: &mut HashMap<AreaIndex, HashMap<String, Rc<RefCell<MovingObject>>>>
    ) {
        let mut object = object_ptr.borrow_mut();

        let (top_left, top_right, bottom_right, bottom_left) = self.get_areas(map, &mut object);
        self.fill_overlapping_areas(top_left, top_right, bottom_right, bottom_left);

        self.remove_object(objects_in_area, &mut object);
        self.add_object(&object_ptr, objects_in_area, object);

        self.overlapping_areas.clear();
    }

    pub fn check_collisions(&self, objects_in_area: &mut HashMap<AreaIndex, HashMap<String, Rc<RefCell<MovingObject>>>>) {
        for y in 0..self.vertical_area_count {
            for x in 0..self.horizontal_area_count {
                if let Some(objectsMap) = objects_in_area.get(&AreaIndex { x, y }) {
                    let objects: Vec<Rc<RefCell<MovingObject>>> = objectsMap
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
                  object_ptr: &Rc<RefCell<MovingObject>>,
                  objects_in_area: &mut HashMap<AreaIndex, HashMap<String, Rc<RefCell<MovingObject>>>>,
                  mut object: RefMut<MovingObject>
    ) {
        for i in 0..self.overlapping_areas.len() {
            if !object.areas.contains(&self.overlapping_areas[i]) {
                if !objects_in_area.contains_key(&self.overlapping_areas[i]) {
                    let mut map: HashMap<String, Rc<RefCell<MovingObject>>> = HashMap::new();
                    map.insert(object.object_id.clone(), Rc::clone(&object_ptr));
                    objects_in_area.insert(self.overlapping_areas[i].clone(), map);
                } else {
                    objects_in_area
                        .get_mut(&self.overlapping_areas[i])
                        .unwrap()
                        .insert(object.object_id.clone(), Rc::clone(&object_ptr));
                }
                object.areas.push(self.overlapping_areas[i].clone())
            }
        }
    }

    fn remove_object(&mut self, objects_in_area: &mut HashMap<AreaIndex, HashMap<String, Rc<RefCell<MovingObject>>>>, object: &mut RefMut<MovingObject>) {
        let mut existing: Vec<AreaIndex> = Vec::new();
        for area in object.areas.iter() {
            if !self.overlapping_areas.contains(&area) {
                if objects_in_area.contains_key(&area) {
                    let mut objs = objects_in_area
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

    fn get_areas(&mut self, map: &Map, mut object: &mut RefMut<MovingObject>) -> (AreaIndex, AreaIndex, AreaIndex, AreaIndex) {
        let mut top_left = map.get_map_tile_in_point(sub(object.aabb.center, object.aabb.half_size));
        let mut top_right = map.get_map_tile_in_point(
            [object.aabb.center[0] + object.aabb.half_size[0],
                object.aabb.center[1] - object.aabb.half_size[1]]);
        let mut bottom_right = map.get_map_tile_in_point(add(object.aabb.center, object.aabb.half_size));

        top_left.x /= self.grid_area_width;
        top_left.y /= self.grid_area_height;
        top_right.x /= self.grid_area_width;
        top_right.y /= self.grid_area_height;
        bottom_right.x /= self.grid_area_width;
        bottom_right.y /= self.grid_area_height;

        let mut bottom_left = AreaIndex {
            x: top_left.x,
            y: bottom_right.y
        };

        (top_left, top_right, bottom_right, bottom_left)
    }

    fn fill_overlapping_areas(&mut self, mut top_left: AreaIndex, mut top_right: AreaIndex, mut bottom_right: AreaIndex, mut bottom_left: AreaIndex) {
        if top_left.x == top_right.x && top_left.y == bottom_left.y {
            self.overlapping_areas.push(top_left);
        } else if top_left.x == top_right.x {
            self.overlapping_areas.push(top_left);
            self.overlapping_areas.push(bottom_left);
        } else if top_left.y == bottom_left.y {
            self.overlapping_areas.push(top_left);
            self.overlapping_areas.push(top_right);
        } else {
            self.overlapping_areas.push(top_left);
            self.overlapping_areas.push(bottom_left);
            self.overlapping_areas.push(top_right);
            self.overlapping_areas.push(bottom_right);
        }
    }

    fn check_collisions_in_area(&self, objects: &Vec<Rc<RefCell<MovingObject>>>) {
        for i in 0..objects.len() - 1 {
            let mut obj1 = objects[i].borrow_mut();
            for j in (i + 1)..objects.len() {
                let mut obj2 = objects[j].borrow_mut();
                let (collides, overlaps) = obj1.aabb.overlaps_signed(&obj2.aabb);
                if collides && !obj1.all_colliding_objects.contains_key(&obj2.object_id) {

                    let obj1_data = CollisionData {
                        other: Rc::clone(&objects[j]),
                        overlap: overlaps,
                        speed1: obj1.speed,
                        speed2: obj2.speed,
                        old_pos1: obj1.old_position,
                        old_pos2: obj2.old_position,
                        pos1: obj1.position,
                        pos2: obj2.position
                    };
                    obj1.all_colliding_objects.insert(obj2.object_id.clone(), obj1_data);

                    let obj2_data = CollisionData {
                        other: Rc::clone(&objects[i]),
                        overlap: overlaps,
                        speed1: obj2.speed,
                        speed2: obj1.speed,
                        old_pos1: obj2.old_position,
                        old_pos2: obj1.old_position,
                        pos1: obj2.position,
                        pos2: obj1.position
                    };
                    obj2.all_colliding_objects.insert(obj1.object_id.clone(), obj2_data);
                }
            }
        }
    }
}