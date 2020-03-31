use graphics::math::*;
use super::config;
use super::renderable::Renderable;
use super::colors;
use opengl_graphics::GlGraphics;
use graphics::Context;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Empty,
    Block,
    OneWay
}

pub struct Map {
    pub tiles: Vec<Vec<TileType>>,
    pub position: Vec2d,
    width: i8,
    height: i8,
    pub tileSize: f64
}

impl Map {

   pub fn new(tiles: Vec<Vec<TileType>>, position: Vec2d, width: i8, height: i8, tileSize: f64) -> Map {
    Map {
        tiles: tiles,
        position: position,
        width: width,
        height: height,
        tileSize: tileSize
    }
   }
 
   pub fn get_map_tile_in_point(&self, point: Vec2d) -> (i8, i8) {
        let x = (point[0] - self.position[0] + (self.tileSize / 2.0)) / self.tileSize;
        let y = (point[1] - self.position[1] + (self.tileSize / 2.0)) / self.tileSize;

        (x as i8, y as i8)
   }

   pub fn get_map_tileY_at_point(&self, y: f64) -> i8 {
       ((y - self.position[1] + (self.tileSize / 2.0)) / self.tileSize) as i8
   }

   pub fn get_map_tileX_at_point(&self, x: f64) -> i8 {
       ((x - self.position[0] + (self.tileSize / 2.0)) / self.tileSize) as i8
   }

   pub fn get_map_tile_position(&self, tileIndexX: i8, tileIndexY: i8) -> Vec2d {
       let x = (tileIndexX as f64 * self.tileSize) + self.position[0];
       let y = (tileIndexY as f64 * self.tileSize) + self.position[1];

       [x, y]
   }

   pub fn get_tile(&self, x: i8, y :i8) -> TileType {
       if x < 0 || x >= self.width || y < 0 || y >= self.height {
           return TileType::Block;
       } 
       return self.tiles[x as usize][y as usize];
   }

   pub fn is_obstacle(&self, x: i8, y: i8) -> bool {
    if x < 0 || x >= self.width || y < 0 || y >= self.height {
        return true;
    }
    return self.tiles[x as usize][y as usize] == TileType::Block;
   }

   pub fn is_ground(&self, x: i8, y: i8) -> bool {
    if x < 0 || x >= self.width || y < 0 || y >= self.height {
        return false;
    }
    return self.tiles[x as usize][y as usize] == TileType::OneWay 
        || self.tiles[x as usize][y as usize] == TileType::Block;
   }

   pub fn is_one_way_platform(&self, x: i8, y: i8) -> bool {
    if x < 0 || x >= self.width || y < 0 || y >= self.height {
        return false;
    }
    return self.tiles[x as usize][y as usize] == TileType::OneWay;
   }

   pub fn is_empty(&self, x: i8, y: i8) -> bool {
    if x < 0 || x >= self.width || y < 0 || y >= self.height {
        return false;
    }
    return self.tiles[x as usize][y as usize] == TileType::Empty;
   }

}