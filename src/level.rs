use std::cmp::PartialEq;
use std::collections::HashMap;
use std::u32;
use image::imageops::tile;
use log::warn;
use sdl2::render::{Canvas, Texture, WindowCanvas};
use crate::resource_location::ResourceLocation;
use crate::tile::{Tile, TileSize, TileType};
use crate::utils::get_dist;

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
        let dirt = (tiles.get("game:tiles/dirt.json").unwrap().clone());

        let mut level = Self{
            tile_big: TileGraph::create(TileSize::BIG),
            tile_medium: TileGraph::create(TileSize::MEDIUM),
            tile_small: TileGraph::create(TileSize::SMALL),
            tile_nav: TileGraph::create(TileSize::SMALL),
        };


        for x in 0..4 {

            level.tile_big.append(wall.clone(), (x, -1), vec![]);


        }

        for x in 0..8 {
            for y in 0..3 {
                level.tile_medium.append(floor.clone(), (x, y), vec![]);
            }
        }

        for x in 0..16 {
            for y in 0..6 {
                level.tile_nav.append(Tile::create_nav(), (x, y), vec![])
            }
        }
        level.tile_nav.build_connections();


        level
    }



}

pub struct TileGraph {
    nodes : HashMap<(i32, i32), Tile>,
    connections : HashMap<(i32,i32), Vec<((i32,i32), u32)>>,
    tile_size: TileSize
}

impl TileGraph {
    pub fn append(&mut self, tile : Tile,value : (i32, i32), connections : Vec<((i32, i32), u32)>) {

        if tile.clone().get_size() != self.tile_size {
            warn!("Tried appending tile \"{}\" of size \"{}\" to the wrong tile graph!", tile.clone().get_resource_location().to_string(), tile.clone().get_size().get().0)
        }
        else {
            self.nodes.insert(value, tile);
            self.connections.insert(value, connections);
        }
    }

    pub fn append_from_wolrd_space(&mut self, tile : Tile,value : (i32, i32), connections : Vec<((i32, i32), u32)>) {
        self.append(tile, (value.0 / self.tile_size.get().0 as i32, value.1 / self.tile_size.get().0 as i32), connections)
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

    pub fn build_connections(&mut self) {
        for t in self.nodes.clone() {
            let n = (t.0.0, t.0.1 + 1);
            let e = (t.0.0 + 1, t.0.1);
            let s = (t.0.0, t.0.1 - 1);
            let w = (t.0.0 - 1, t.0.1);
            let mut connections = vec![];
            if self.nodes.get(&n).is_some() {
                if self.nodes.clone().get_mut(&n).unwrap().get_type() == TileType::FLOOR {
                    connections.push((n, 1))
                }
            }
            if self.nodes.get(&e).is_some() {
                if self.nodes.clone().get_mut(&e).unwrap().get_type() == TileType::FLOOR {
                    connections.push((e, 1))
                }
            }
            if self.nodes.get(&s).is_some() {
                if self.nodes.clone().get_mut(&s).unwrap().get_type() == TileType::FLOOR {
                    connections.push((s, 1))
                }
            }
            if self.nodes.get(&w).is_some() {
                if self.nodes.clone().get_mut(&w).unwrap().get_type() == TileType::FLOOR {
                    connections.push((w, 1))
                }
            }
            self.connections.insert(t.0, connections);
        }
    }

    // an implementation of A*
    pub fn path_to(&mut self, x : i32, y : i32, tx : i32, ty : i32) -> Vec<(i32, i32)> {

        let inf = u32::MAX;

        // goal
        let mut g : HashMap<(i32,i32), u32> = HashMap::new();
        //found
        let mut f : HashMap<(i32,i32), u32> = HashMap::new();

        let startx = x / self.tile_size.get().0 as i32;
        let starty = y / self.tile_size.get().0 as i32;

        let mut graph = self.connections.clone();

        let mut prev_vert : HashMap<(i32,i32), (i32, i32)> = HashMap::new();
        let mut optimal_path = vec![];

        for vertex in graph.clone() {
            g.insert(vertex.0, inf);
            f.insert(vertex.0, inf);
        }
        *g.get_mut(&(startx, starty)).unwrap() = 0;
        *f.get_mut(&(startx, starty)).unwrap() = 0;

        while graph.len() > 0 {
            let mut shortest : Option<(i32,i32)> = None;
            for vert in graph.clone() {
                if shortest.is_none() {
                    shortest = Some(vert.0)
                }
                else if f.get(&vert.0).unwrap() < f.get(&shortest.unwrap()).unwrap(){
                    shortest = Some(vert.0)
                }
            }

            for (neighbour, cost) in graph.get(&shortest.unwrap()).unwrap() {

                if graph.get(neighbour).is_some() && cost + g.get(&shortest.unwrap()).unwrap() < *f.get(&neighbour).unwrap() {
                    *g.get_mut(&neighbour).unwrap() = cost + *g.get_mut(&shortest.unwrap()).unwrap();
                    *f.get_mut(&neighbour).unwrap() = cost + g.get(&shortest.unwrap()).unwrap();
                    prev_vert.insert(*neighbour, shortest.unwrap());
                }
            }
            graph.remove(&shortest.unwrap());

        }

        let mut vertex = (tx / self.tile_size.get().0 as i32, ty / self.tile_size.get().0 as i32);
        while vertex != (startx, starty) && prev_vert.get(&vertex).is_some() {
            optimal_path.insert(0, (vertex.0 * self.tile_size.get().0 as i32, vertex.1 * self.tile_size.get().0 as i32));
            vertex = *prev_vert.get(&vertex).unwrap()

        }
        optimal_path.insert(0, (startx * self.tile_size.get().0 as i32, starty * self.tile_size.get().0 as i32));
        optimal_path

    }

}