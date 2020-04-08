use graphics::math::*;
use super::config;
use super::camera::camera_dependent_object;
use super::renderable::Renderable;
use super::colors;
use super::animator;
use super::texture_loader::Texture_Loader;
use opengl_graphics::Texture;
use opengl_graphics::GlGraphics;
use graphics::Context;
use std::rc::Rc;


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
    pub tileSize: f64,
    tileTexture: Texture,
    oneWayTexture: Texture,
    tile_tex_scale: f64
}

impl Map {

   pub fn new(tiles: Vec<Vec<TileType>>, position: Vec2d, width: i8, height: i8, tileSize: f64, texture_loader: Rc<Texture_Loader>) -> Map {
        use graphics::*;

        let texture = texture_loader.load_texture("Tiles/crate-std-2.png");
        let ow_texture = texture_loader.load_texture("Tiles/grid-line-1.png");

        let texture_size = texture.get_size();

        if texture_size.0 != texture_size.1 {
            panic!("Tile texture must be square");
        }

        let tile_tex_scale = tileSize / texture_size.0 as f64;

        Map {
            tiles: tiles,
            position: position,
            width: width,
            height: height,
            tileSize: tileSize,
            tileTexture: texture,
            oneWayTexture: ow_texture,
            tile_tex_scale: tile_tex_scale
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
       let result = ((x - self.position[0]) / self.tileSize) as i8;
       return result;
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
    return self.tiles[y as usize][x as usize] == TileType::Block;
   }

   pub fn is_ground(&self, x: i8, y: i8) -> bool {
    if x < 0 || x >= self.width || y < 0 || y >= self.height {
        return false;
    }
    return self.tiles[y as usize][x as usize] == TileType::OneWay
        || self.tiles[y as usize][x as usize] == TileType::Block;
   }

   pub fn is_one_way_platform(&self, x: i8, y: i8) -> bool {
    if x < 0 || x >= self.width || y < 0 || y >= self.height {
        return false;
    }
    return self.tiles[y as usize][x as usize] == TileType::OneWay;
   }

   pub fn is_empty(&self, x: i8, y: i8) -> bool {
    if x < 0 || x >= self.width || y < 0 || y >= self.height {
        return false;
    }
    return self.tiles[y as usize][x as usize] == TileType::Empty;
   }

   pub fn render_tile(&self, ctx: &Context, gl: &mut GlGraphics, tileTexture: &Texture, tileIndex: (usize, usize)) {
    use graphics::*;

    let y = self.tileSize * tileIndex.0 as f64 + self.position[1];
    let x = self.tileSize * tileIndex.1 as f64 + self.position[0];

    let point_trans = ctx.transform
        .trans(x, y)
        .scale(self.tile_tex_scale, self.tile_tex_scale);

    image(tileTexture, point_trans, gl);
   }
}

impl Renderable for Map {
    fn render(&mut self, ctx: &Context, gl: &mut GlGraphics) {

        for (i, columns) in self.tiles.iter().enumerate() {
            for (k, tile) in columns.iter().enumerate() {
                if *tile == TileType::Block {
                    self.render_tile(&ctx, gl, &self.tileTexture, (i, k));
                }

                if *tile == TileType::OneWay {
                    self.render_tile(&ctx, gl, &self.oneWayTexture, (i, k));
                }
            }
        }
    }
}

impl camera_dependent_object for Map {
    fn move_object(&mut self, x: f64, y: f64){
        self.position[0] += x * config::MAP_TILES_PARRALAX_FACTOR;
        self.position[1] += y * config::MAP_TILES_PARRALAX_FACTOR;
    }
}