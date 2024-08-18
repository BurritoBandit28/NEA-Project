use sdl2::rect::Rect;
use std::sync::{Arc, Mutex};
use crate::entity::{Entity, Mobile};
use crate::game::Game;
use crate::render::{AssetData, TextureType};

pub struct Enemy {
    coords: (f32, f32),
    pub asset_data: AssetData,
    velocity : (f32, f32),
    uuid : String,
    game : *mut Game
}

impl Entity for Enemy {
    fn get_coords(&mut self) -> (f32, f32) {
        self.coords
    }

    fn set_coords(&mut self, coords : (f32, f32)) {
        self.coords = coords;
    }

    fn get_asset_data(&self) -> AssetData {
        AssetData {
            UV: self.asset_data.UV.clone(),
            Origin: self.asset_data.Origin.clone(),
            texture_type: self.asset_data.texture_type.clone(),
        }
    }

}

impl Enemy {
    pub fn create(game: &mut Game) {
        let asset_data = AssetData {
            UV : Option::from(Rect::new(16, 0, 16, 16)),
            Origin : (8,8),
            texture_type : TextureType::in_game_sprite
        };

        let mut entity = Self{
            coords: (0.0,0.0),
            asset_data,
            velocity: (0.0, 0.0),
            uuid: "100".to_string(), // will be from hash function
            game,
        };
        let ret = Box::new(Mutex::new(entity));
        game.mobiles.push(ret);
    }
}

impl Mobile for Enemy {
    fn get_velocity(&mut self) -> (f32, f32) {
        self.velocity
    }

    fn set_velocity(&mut self, velocity: (f32, f32)) {
        self.velocity = velocity;
    }

}