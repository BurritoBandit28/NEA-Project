use std::cmp::PartialEq;
use std::collections::HashMap;
use image::imageops::tile;
use log::warn;
use sdl2::render::{Canvas, Texture, WindowCanvas};
use crate::resource_location::ResourceLocation;
use crate::tile::{Tile, TileSize};

pub struct Level {
    pub(crate) tile_big : TileGraph,
    pub(crate) tile_medium : TileGraph,
    pub(crate) tile_small : TileGraph,
    pub(crate) tile_nav : TileGraph
}

impl Level {

    pub fn render(&mut self, player_coords :  (f32, f32), texture : &HashMap<String, Texture>, canvas: &mut WindowCanvas, sf : i32, debug : bool) {
        self.tile_big.render(player_coords, texture, canvas, sf);
        self.tile_medium.render(player_coords, texture, canvas, sf);
        self.tile_small.render(player_coords, texture, canvas, sf);
        if debug {
            self.tile_nav.render(player_coords, texture, canvas, sf)
        }
    }

    pub fn get_tile(&mut self, size : TileSize, coordinates: (f32, f32)) -> Tile {
        if coordinates.0 <= 0.0 || coordinates.1 <= 0.0 {
            return Tile::create_none(size);
        }

        match size {
            TileSize::BIG => {self.tile_big.get_tile(coordinates.0 as i32, coordinates.1 as i32)}
            TileSize::MEDIUM => {self.tile_medium.get_tile(coordinates.0 as i32, coordinates.1 as i32)}
            TileSize::SMALL => {self.tile_small.get_tile(coordinates.0 as i32, coordinates.1 as i32)}
        }


    }


    pub fn create_demo_level(tiles : &HashMap<String, Tile>) -> Self {

        let wall = (tiles.get("game:tiles/wall.json").unwrap().clone());
        let floor = (tiles.get("game:tiles/floor.json").unwrap().clone());
        let cardboard_box = (tiles.get("game:tiles/cardboard_box.json").unwrap().clone());

        let mut level = Self{
            tile_big: TileGraph::create(TileSize::BIG),
            tile_medium: TileGraph::create(TileSize::MEDIUM),
            tile_small: TileGraph::create(TileSize::SMALL),
            tile_nav: TileGraph::create(TileSize::SMALL),
        };

        let mut x = 0;
        for x in 0..4 {
            /*
            level.tile_big_old.get_mut(0).unwrap().push(wall.clone());
            level.tile_medium_old.get_mut(2).unwrap().push(floor.clone());
            level.tile_medium_old.get_mut(2).unwrap().push(floor.clone());
             */
            level.tile_big.append(wall.clone(), (x, 0), vec![]);
            level.tile_medium.append(floor.clone(), (x, 2), vec![]);

        }

        //level.tile_small_old.insert(4, vec![None, None, Some(cardboard_box.clone())]);

        level
    }



}

pub struct TileGraph {
    nodes : HashMap<(i32, i32), Tile>,
    connections : HashMap<(i32,i32), Vec<(i32,i32)>>,
    tile_size: TileSize
}

impl TileGraph {
    pub fn append(&mut self, tile : Tile,value : (i32, i32), connections : Vec<(i32, i32)>) {

        if tile.clone().get_size() != self.tile_size {
            warn!("Tried appending tile \"{}\" of size \"{}\" to the wrong tile graph!", tile.clone().get_resource_location().to_string(), tile.clone().get_size().get().0)
        }
        else {
            self.nodes.insert(value, tile);
            self.connections.insert(value, connections);
        }
    }

    pub fn get_tile(&mut self, x : i32, y : i32) -> Tile {
        let sf = self.tile_size.get().0 as i32;

        let tx = x / sf;
        let ty = y / sf;

        let result = self.nodes.get(&(tx, ty));

        if result.is_some() {
            result.unwrap().clone()
        }
        else {
            Tile::create_none(self.tile_size)
        }

    }

    pub fn render(&mut self, player_coords :  (f32, f32), texture : &HashMap<String, Texture>, canvas: &mut WindowCanvas, sf : i32) {

        let tile_scale = self.tile_size.get().0 as i32;

        for tile in self.nodes.clone() {
            tile.1.render(texture, (tile.0.0 * tile_scale, tile.0.1 * tile_scale), canvas, sf, player_coords);
        }
    }

    pub fn create(tile_size: TileSize) -> Self{
        Self {
            nodes: HashMap::new(),
            connections: HashMap::new(),
            tile_size,
        }
    }


    pub fn path_to(&mut self, x : i32, y : i32, tx : i32, ty : i32) -> Vec<Tile> {
        todo!()
    }

}