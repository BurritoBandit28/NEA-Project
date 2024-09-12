use std::collections::HashMap;
use image::imageops::tile;
use sdl2::render::{Canvas, Texture, WindowCanvas};
use crate::resource_location::ResourceLocation;
use crate::tile::{Tile, TileSize};

pub struct Level {
    pub(crate) tile_big : Vec<Vec<Option<Tile>>>,
    pub(crate) tile_medium : Vec<Vec<Option<Tile>>>,
    pub(crate) tile_small : Vec<Vec<Option<Tile>>>,
    pub(crate) path_gird : Vec<Vec<Option<Tile>>>
}

impl Level {

    pub fn render(&mut self, player_coords :  (f32, f32), texture : &HashMap<String, Texture>, canvas: &mut WindowCanvas, sf : i32, debug : bool) {
        Self::render_tiles(self.tile_big.clone(), player_coords, texture, canvas, sf);
        Self::render_tiles(self.tile_medium.clone(), player_coords, texture, canvas, sf);
        Self::render_tiles(self.tile_small.clone(), player_coords, texture, canvas, sf);
        if debug {
            Self::render_tiles(self.path_gird.clone(), player_coords, texture, canvas, sf)
        }
    }

    pub fn get_tile(&mut self, size : TileSize, coordinates: (f32, f32)) -> Tile {
        if coordinates.0 <= 0.0 || coordinates.1 <= 0.0 {
            return Tile::create_none(size);
        }

        let x = coordinates.0 as u32 / size.get().0;
        let y = coordinates.1 as u32 / size.get().1;

        let mut tiles: Vec<Vec<Option<Tile>>> = Vec::new();
        match size {
            TileSize::BIG => {
                tiles = self.tile_big.clone();
            }
            TileSize::MEDIUM => {
                tiles = self.tile_medium.clone();
            }
            TileSize::SMALL => {
                tiles = self.tile_small.clone();
            }
        }

        let tiley = tiles.get(y as usize);

        if tiley.is_some() {
            let tile = tiley.unwrap().get(x as usize);
            if tile.is_some() {
                if tile.unwrap().is_some() {
                    return tile.unwrap().clone().unwrap().clone();
                };
            };
        }

    Tile::create_none(size)


    }

    fn render_tiles(tiles : Vec<Vec<Option<Tile>>>, player_coords :  (f32, f32), texture : &HashMap<String, Texture>, canvas: &mut WindowCanvas, sf : i32) {
        let mut x = 0;
        let mut y = 0;
        for tt in tiles.clone() {
            x = 0;
            for t in tt {
                if t.is_some() {
                    t.unwrap().render(texture, (x, y), canvas, sf, player_coords);
                }
                x += 1;
            }
            y += 1;
        }
    }

    pub fn create_test_level(tiles : &HashMap<String, Tile>) -> Self {

        let tile = Some(tiles.get("game:tiles/wall.json").unwrap().clone());
        let tile2 = Some(tiles.get("game:tiles/look_i_can_name_this_what_i_want.json").unwrap().clone());
        let tile3 = Some(tiles.get("game:tiles/dirt.json").unwrap().clone());
        let tile4 = Some(tiles.get("game:tiles/orange.json").unwrap().clone());

        Self {
            tile_big: vec![
                vec![tile.clone(), tile.clone(), tile.clone()],
                vec![None, tile2.clone(), tile2.clone(), tile2.clone()],
            ],
            tile_medium: vec![vec![None,None,None,None,None,None,None,None,tile4.clone()]],
            tile_small: vec![
                vec![tile3.clone(),tile3.clone(), None, None, tile3.clone(),tile3.clone()],
                vec![tile3.clone(),tile3.clone(), None, None, tile3.clone(),tile3.clone()],
                vec![None, None, tile3.clone(), tile3.clone(), None, None],
                vec![None, tile3.clone(), tile3.clone(), tile3.clone(), tile3.clone(), None],
                vec![None, tile3.clone(), tile3.clone(), tile3.clone(), tile3.clone(), None],
                vec![None, tile3.clone(), None, None, tile3.clone(), None],
            ],
            path_gird : vec![]
        }
    }

    pub fn create_demo_level(tiles : &HashMap<String, Tile>) -> Self {

        let wall = Some(tiles.get("game:tiles/wall.json").unwrap().clone());
        let floor = Some(tiles.get("game:tiles/floor.json").unwrap().clone());

        let mut level = Self{
            tile_big:vec![vec![]],
            tile_medium:vec![vec![],vec![],vec![]],
            tile_small:vec![],
            path_gird:vec![]
        };

        let mut x = 0;
        for x in 0..4 {
            level.tile_big.get_mut(0).unwrap().push(wall.clone());
            level.tile_medium.get_mut(2).unwrap().push(floor.clone());
            level.tile_medium.get_mut(2).unwrap().push(floor.clone());
        }

        level

    }
}