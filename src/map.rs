use graphics::math::*;
use super::config;
use super::camera::CameraDependentObject;
use super::renderable::Renderable;
use super::colors;
use super::animator;
use super::texture_loader::TextureLoader;
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

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct AreaIndex {
    pub x: i8,
    pub y: i8
}

pub struct Map {
    pub tiles: Vec<Vec<TileType>>,
    pub position: Vec2d,
    width: i8,
    height: i8,
    pub tile_size: f64,
    tile_texture: Texture,
    one_way_texture: Texture,
    tile_tex_scale: f64
}

impl Map {

   pub fn new(tiles: Vec<Vec<TileType>>, position: Vec2d, width: i8, height: i8, tile_size: f64, texture_loader: Rc<TextureLoader>) -> Map {
        use graphics::*;

        let texture = texture_loader.load_texture("Tiles/crate-std-2.png");
        let ow_texture = texture_loader.load_texture("Tiles/grid-line-1.png");

        let texture_size = texture.get_size();

        if texture_size.0 != texture_size.1 {
            panic!("Tile texture must be square");
        }

        let tile_tex_scale = tile_size / texture_size.0 as f64;

        Map {
            tiles: tiles,
            position: position,
            width: width,
            height: height,
            tile_size,
            tile_texture: texture,
            one_way_texture: ow_texture,
            tile_tex_scale: tile_tex_scale
        }
   }

   pub fn get_map_tile_in_point(&self, point: Vec2d) -> AreaIndex {
        let x = (point[0] - self.position[0]) / self.tile_size;
        let y = (point[1] - self.position[1]) / self.tile_size;

        AreaIndex{
            x: x as i8,
            y: y as i8
        }
   }

   pub fn get_map_tile_y_at_point(&self, y: f64) -> i8 {
       ((y - self.position[1] + (self.tile_size / 2.0)) / self.tile_size) as i8
   }

   pub fn get_map_tile_x_at_point(&self, x: f64) -> i8 {
       let result = ((x - self.position[0]) / self.tile_size) as i8;
       return result;
   }

   pub fn get_map_tile_position(&self, tile_index_x: i8, tile_index_y: i8) -> Vec2d {
       let x = (tile_index_x as f64 * self.tile_size) + self.position[0];
       let y = (tile_index_y as f64 * self.tile_size) + self.position[1];

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

   pub fn render_tile(&self, ctx: &Context, gl: &mut GlGraphics, tile_texture: &Texture, tile_index: (usize, usize)) {
    use graphics::*;

    let y = self.tile_size * tile_index.0 as f64 + self.position[1];
    let x = self.tile_size * tile_index.1 as f64 + self.position[0];

    let point_trans = ctx.transform
        .trans(x, y)
        .scale(self.tile_tex_scale, self.tile_tex_scale);

    image(tile_texture, point_trans, gl);
   }
}

impl Renderable for Map {
    fn render(&mut self, ctx: &Context, gl: &mut GlGraphics) {

        for (i, columns) in self.tiles.iter().enumerate() {
            for (k, tile) in columns.iter().enumerate() {
                if *tile == TileType::Block {
                    self.render_tile(&ctx, gl, &self.tile_texture, (i, k));
                }

                if *tile == TileType::OneWay {
                    self.render_tile(&ctx, gl, &self.one_way_texture, (i, k));
                }
            }
        }
    }
}

impl CameraDependentObject for Map {
    fn move_object(&mut self, x: f64, y: f64){
        self.position[0] += x * config::MAP_TILES_PARRALAX_FACTOR;
        self.position[1] += y * config::MAP_TILES_PARRALAX_FACTOR;
    }
}