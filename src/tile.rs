use std::collections::HashMap;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, WindowCanvas};
use crate::render;
use crate::render::AssetData;
use crate::resource_location::ResourceLocation;
use crate::widgets::enum_widget::WidgetEnum;

// might not keep this, not sure yet
pub enum TileType {
    WALL,
    FLOOR
}
impl TileType {
    pub fn parse(ttype : String) -> Self {
        match ttype.as_str() {
            "wall" => {Self::WALL}
            _ => {Self::FLOOR}
        }
    }
}

impl Clone for TileType {
    fn clone(&self) -> Self {
        match self {
            TileType::WALL => {Self::WALL}
            TileType::FLOOR => {Self::FLOOR}
        }
    }
}

impl Copy for TileType {

}

pub enum TileSize {
    BIG,
    MEDIUM,
    SMALL
}

impl TileSize {
    pub fn parse(ts : &str) -> Self {
        match ts.to_lowercase().as_str() {
            "small" => {
                Self::SMALL
            }
            "medium" => {
                Self::MEDIUM
            }
            _ => {
                Self::BIG
            }
        }
    }

    pub fn get(&self) -> (u32, u32) {
        match self {
            TileSize::BIG => {(64, 64)}
            TileSize::MEDIUM => {(32 ,32)}
            TileSize::SMALL => {(16, 16)}
        }
    }

}

impl WidgetEnum for TileSize {
    fn get_as_string(&mut self) -> String {
        match self {
            TileSize::BIG => {String::from("big_tile")}
            TileSize::MEDIUM => {String::from("medium_tile")}
            TileSize::SMALL => {String::from("small_tile")}
        }
    }

    fn get_from_index(&mut self, index: usize) -> Self {
        match index {
            1 => {TileSize::MEDIUM},
            2=> {TileSize::BIG}
            _ => {TileSize::SMALL}
        }
    }

    fn count(&mut self) -> usize {
        3usize
    }

    fn name(&mut self) -> String {
        String::from("tile_size")
    }
}

impl Clone for TileSize {
    fn clone(&self) -> Self {
        match self {
            TileSize::BIG => {Self::BIG}
            TileSize::MEDIUM => {Self::MEDIUM}
            TileSize::SMALL => {Self::SMALL}
        }
    }
}

impl Copy for TileSize {

}


// tiles loaded on runtime by a json file that defines its texture, material and if its a wall
pub struct Tile {
    name : String,
    resource_location: ResourceLocation,
    tile_type: TileType,
    size : TileSize,
    origin : (i32, i32),
    collision : bool,
    collision_box : Option<(u32, u32)>,
    asset_data : AssetData
}



impl Tile {

    pub fn create(
        name : String,
        resource_location : ResourceLocation,
        texture : ResourceLocation,
        uv : (u32, u32),
        tile_type : TileType,
        size : TileSize,
        origin : (i32, i32),
        collision : bool,
        collision_box : Option<(u32, u32)>
    ) -> Self {

        let s = size.get();

        let ass = AssetData {
            uv : Some(Rect::new(uv.0 as i32, uv.1 as i32, s.0, s.1)),
            origin,
            resource_location : texture,
        };

        Self {
            name,
            resource_location,
            tile_type,
            size,
            origin,
            collision,
            collision_box,
            asset_data : ass
        }
    }

    fn screen(&self, coords : (i32, i32), player_coords :  (f32, f32)) -> (i32, i32) {
        let size = self.size.get();
        let scaled = (coords.0 * size.0 as i32, coords.1 * size.1 as i32);

        let x = scaled.0;
        let y = scaled.1;
        let px = (if player_coords.0 < 0.0 {player_coords.0 - 1.0} else { player_coords.0 }) as i32;
        let py = (if player_coords.1 < 0.0 {player_coords.1 - 1.0} else { player_coords.1 }) as i32;
        ((160i32 - px) + x, (90i32 - py ) + y)
    }

    pub fn render(&self, texture: &HashMap<String,Texture>, coords : (i32, i32), canvas: &mut WindowCanvas, sf : i32, player_coords :  (f32, f32)) {
        let screen = self.screen(coords, player_coords);
        render::draw_pp_texture(screen.0, screen.1, &self.asset_data, canvas, sf, texture)

    }

}

impl Clone for Tile {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            resource_location: self.resource_location.clone(),
            tile_type: self.tile_type.clone(),
            size: self.size.clone(),
            origin: self.origin.clone(),
            collision: self.collision.clone(),
            collision_box: self.collision_box.clone(),
            asset_data: self.asset_data.clone(),
        }
    }
}
