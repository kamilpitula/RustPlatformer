use super::map::TileType;
use std::path::PathBuf;
use std::rc::Rc;
use std::fs::File;
use std::io::prelude::Read;

pub struct MapLoader {
    assets_path: Rc<PathBuf>
}

impl MapLoader {
    pub fn new(assets_path: Rc<PathBuf>) -> MapLoader {
        MapLoader {
            assets_path: assets_path
        }
    }

    pub fn load_map(&self, path: &str) -> Vec<Vec<TileType>> {
        let map_path = self.assets_path.join(path);
        let mut file = File::open(map_path).expect("can't open file");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("can't read file");
        let mut map = Vec::new();

        for lines in content.lines() {
            let mut row = Vec::new();
            for c in lines.chars() {
                if c == 'E' {
                 row.push(TileType::Empty);   
                } else if c == 'O' {
                    row.push(TileType::OneWay);
                } else if c == 'B' {
                    row.push(TileType::Block);
                }
            }
            map.push(row);
        }
        map
    }
}