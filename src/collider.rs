use super::moving_object::Moving_Object;

pub struct Collider {
    gridAreaWidth: i8,
    gridAreaHeight: i8,
    objectsInArea: Vec<Vec<Moving_Object>>,
    horizontalAreaCount: i8,
    verticalAreaCount: i8,
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

        Collider{
            gridAreaHeight: gridAreaHeight,
            gridAreaWidth: gridAreaWidth,
            objectsInArea: Vec::with_capacity((horizontalAreaCount * verticalAreaCount) as usize),
            horizontalAreaCount: horizontalAreaCount,
            verticalAreaCount: verticalAreaCount
        }
    }
}