use std::collections::HashMap;
use image::imageops::tile;
use sdl2::render::{Canvas, Texture, WindowCanvas};
use crate::resource_location::ResourceLocation;
use crate::tile::Tile;

pub struct Level {
    tile_big : Vec<Vec<Option<Tile>>>,
    tile_medium : Vec<Vec<Option<Tile>>>,
    tile_small : Vec<Vec<Option<Tile>>>
}

impl Level {

    pub fn render(&mut self, player_coords :  (f32, f32), texture : &HashMap<String, Texture>, canvas: &mut WindowCanvas, sf : i32) {
        Self::render_tiles(self.tile_big.clone(), player_coords, texture, canvas, sf);
        Self::render_tiles(self.tile_medium.clone(), player_coords, texture, canvas, sf);
        Self::render_tiles(self.tile_small.clone(), player_coords, texture, canvas, sf);
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

        let tile = Some(tiles.get("game:tiles\\wall.json").unwrap().clone());
        let tile2 = Some(tiles.get("game:tiles\\look_i_can_name_this_what_i_want.json").unwrap().clone());
        let tile3 = Some(tiles.get("game:tiles\\dirt.json").unwrap().clone());
        let tile4 = Some(tiles.get("game:tiles\\orange.json").unwrap().clone());

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
        }
    }
}